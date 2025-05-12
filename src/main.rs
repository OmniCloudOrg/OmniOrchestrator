//=============================================================================
// OmniOrchestrator - A distributed system for managing and orchestrating
//=============================================================================
// Maintained by: Tristan J. Poland, Maxine Deandrade, Caznix, Haywood Spartian
// and the OmniCloud community.
//=============================================================================
// This is the entry point for the OmniOrchestrator server application.
// It manages the entirety of the OmniCloud platform and its components:
//    - Database
//    - API
//    - Cluster Management
//    - Bootstrapping
//    - Load Balancing
//=============================================================================

// +-------------+
// | MODULES     |
// +-------------+
mod config;
mod leader;
mod state;
mod app_autoscaler;
mod cluster;
mod network;
mod worker_autoscaler;

mod schemas;

// +-------------+
// | IMPORTS     |
// +-------------+
// Third-party dependencies
use anyhow::anyhow;
use anyhow::Result;
use anyhow::Error;
use clickhouse;
use colored::Colorize;
use schemas::v1::models::platform;
use schemas::v1::models::platform::Platform;
use core::panic;
use env_logger::Builder;
use lazy_static::lazy_static;
use reqwest::Client;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::Build;
use rocket::Rocket;
use rocket::{Request, Response};
use schemas::auth::AuthConfig;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;
use sqlx::{MySql, Pool};
use std::collections::HashMap;
use std::io::Write;
use std::time::Duration;
use std::{env, sync::Arc};
use tokio::sync::RwLock;
use worker_autoscaler::create_default_cpu_memory_scaling_policy;
use worker_autoscaler::WorkerAutoscaler;
use worker_autoscaler::{CloudDirector, VMConfig, VMTemplate};

// Internal imports
use crate::cluster::{ClusterManager, NodeInfo};
use crate::config::ServerConfig;
use crate::config::SERVER_CONFIG;
use crate::leader::LeaderElection;
use crate::state::SharedState;

use schemas::v1::{api, models};

// We ignore this import as it always says
// unused even when that is not the case
#[allow(unused_imports)]
#[macro_use]
extern crate rocket;

// +---------------------+
// | DATABASE MANAGER    |
// +---------------------+
// Database Manager struct to handle database connections
struct DatabaseManager {
    base_url: String,
    main_pool: Pool<MySql>,
    platform_pools: RwLock<HashMap<i64, Pool<MySql>>>,
}

impl DatabaseManager {
    async fn new() -> Result<Self, Error> {
        let base_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            dotenv::dotenv().ok();
            env::var("DEFAULT_DATABASE_URL")
                .unwrap_or_else(|_| "mysql://root:root@localhost:4001".to_string())
        });

        println!(
            "{}",
            "╔═══════════════════════════════════════════════════════════════╗".bright_blue()
        );
        println!(
            "{}",
            "║                 Deployment Database Connection                ║".bright_blue()
        );
        println!(
            "{}",
            "╚═══════════════════════════════════════════════════════════════╝".bright_blue()
        );

        // Connect to main database
        let main_db_url = format!("{}/omni", base_url);
        log::info!("{}", format!("Main Database URL: {}", main_db_url).blue());

        // First, try to connect to MySQL server without specifying a database
        let server_pool = MySqlPool::connect(&base_url)
            .await
            .expect("Failed to connect to MySQL server");

        // Check if main database exists, create if it doesn't
        Self::ensure_database_exists(&server_pool, "omni").await?;

        // Now connect to the main database
        let main_pool = MySqlPool::connect(&main_db_url)
            .await
            .expect("Failed to connect to main database");

        log::info!("{}", "✓ Main database connection established".green());

        // Initialize the main database schema
        log::info!("{}", "Initializing main database schema...".blue());
        
        let target_version = std::env::var("OMNI_ORCH_SCHEMA_VERSION")
            .unwrap_or_else(|_| "1".to_string())
            .parse::<i64>()
            .unwrap_or(1);

        let current_version =
            schemas::v1::db::queries::metadata::get_meta_value(&main_pool, "omni_schema_version")
                .await
                .unwrap_or_else(|_| "0".to_string())
                .parse::<i64>()
                .unwrap_or(0);

        // Fixed: Call as a static method without self
        Self::update_deployment_schema(
            &main_pool,
            current_version,
            target_version,
        ).await?;

        Ok(Self {
            base_url,
            main_pool,
            platform_pools: RwLock::new(HashMap::new()),
        })
    }

    // Create database if it doesn't exist
    async fn ensure_database_exists(pool: &Pool<MySql>, db_name: &str) -> Result<(), Error> {
        let query = format!("CREATE DATABASE IF NOT EXISTS `{}`", db_name);
        sqlx::query(&query).execute(pool).await?;
        log::info!(
            "{}",
            format!("✓ Ensured database exists: {}", db_name).green()
        );
        Ok(())
    }

    // Get main database pool
    fn get_main_pool(&self) -> &Pool<MySql> {
        &self.main_pool
    }

    // Get or create platform database pool (lazy initialization)
    async fn get_platform_pool(
        &self,
        platform: &Platform,
    ) -> Result<Pool<MySql>, Error> {

        let platform_id = platform.id;
        let platform_name = platform.name.clone();

        // First check if we already have this pool
        {
            let pools = self.platform_pools.read().await;
            if let Some(pool) = pools.get(&platform_id) {
                return Ok(pool.clone());
            }
        }

        // If not found, create the pool
        let db_name = format!("omni_p_{}", platform_name);

        // Check if database exists, create if it doesn't
        let server_pool = MySqlPool::connect(&self.base_url)
            .await
            .expect("Failed to connect to MySQL server");
        Self::ensure_database_exists(&server_pool, &db_name).await?;

        // Create platform database URL and connect
        let platform_db_url = format!("{}/{}", self.base_url, db_name);
        log::info!(
            "{}",
            format!(
                "Creating pool for platform {}: {}",
                platform_name, platform_db_url
            )
            .blue()
        );

        let pool = MySqlPool::connect(&platform_db_url).await.expect(&format!(
            "Failed to connect to platform database: {}",
            db_name
        ));

        // Initialize this platform's database
        self.initialize_platform_database(&pool, &platform)
            .await?;

        // Store the pool
        {
            let mut pools = self.platform_pools.write().await;
            pools.insert(platform_id, pool.clone());
        }

        Ok(pool)
    }

    // Initialize platform database (schema, metadata, etc.)
    async fn initialize_platform_database(
        &self,
        pool: &Pool<MySql>,
        platform: &Platform,
    ) -> Result<(), Error> {
        // Initialize metadata system
        log::info!("{}", "Initializing metadata system...".blue());
        schemas::v1::db::queries::metadata::initialize_metadata_system(pool).await?;
        log::info!("{}", "✓ Metadata system initialized".green());

        // Check and update schema if needed
        let target_version = "1";
        let current_version =
            schemas::v1::db::queries::metadata::get_meta_value(pool, "omni_schema_version")
                .await
                .unwrap_or_else(|_| "0".to_string());

        if current_version != target_version {
            // Schema update logic
            self.update_platform_schema(pool, &current_version, target_version, platform)
                .await?;
        } else {
            log::info!(
                "{}",
                format!("Schema version check: OK (version {})", current_version).green()
            );
        }

        Ok(())
    }

    // Update schema for a platform database
    async fn update_platform_schema(
        &self,
        pool: &Pool<MySql>,
        current_version: &str,
        target_version: &str,
        platform: &Platform,
    ) -> Result<(), Error> {
        let mut input = String::new();
        log::warn!(
            "{}",
            format!(
                "Schema version mismatch! Current: {}, Target: {}",
                current_version, target_version
            )
            .yellow()
        );

        if env::var("OMNI_ORCH_BYPASS_CONFIRM").unwrap_or_default() == "confirm" {
            log::warn!(
                "{}",
                "Bypassing schema update confirmation due to env var".yellow()
            );
            input = "confirm".to_string();
        } else {
            println!("{}", "Type 'confirm' to update schema version:".yellow());
            std::io::stdin().read_line(&mut input)?;
        }

        if input.trim() == "confirm" {
            // Initialize database schema
            log::info!("{}", "Initializing database schema...".blue());

            let schema_version = std::env::var("OMNI_ORCH_SCHEMA_VERSION")
                .unwrap_or_else(|_| "1".to_string())
                .parse::<i64>()
                .unwrap_or(1);

            match schemas::v1::db::init_platform_schema(platform, schema_version, pool).await {
                Ok(_) => {
                    log::info!("{}", "✓ Database schema initialized".green());
                    schemas::v1::db::queries::metadata::set_meta_value(
                        pool,
                        "omni_schema_version",
                        target_version,
                    )
                    .await
                    .expect("Failed to set meta value")
                }
                Err(e) => log::error!(
                    "{}",
                    format!("Failed to initialize database schema: {:?}", e).red()
                ),
            };

            // Initialize sample data
            log::info!("{}", "Initializing sample data...".blue());
            match schemas::v1::db::sample_data(pool, schema_version).await {
                Ok(_) => log::info!("{}", "✓ Sample data initialized".green()),
                Err(e) => log::error!(
                    "{}",
                    format!("Failed to initialize sample data: {:?}", e).red()
                ),
            };
        } else {
            log::warn!("{}", "Schema update cancelled".yellow());
            return Err(anyhow::anyhow!("Schema update cancelled by user"));
        }

        Ok(())
    }

    // Update Schema for deployment database - Fixed to be a static method
    async fn update_deployment_schema(
        pool: &Pool<MySql>,
        current_version: i64,
        target_version: i64,
    ) -> Result<(), Error> {
        let mut input = String::new();
        log::warn!(
            "{}",
            format!(
                "Deployment schema version mismatch! Current: {}, Target: {}",
                current_version, target_version
            )
            .yellow()
        );

        if env::var("OMNI_ORCH_BYPASS_CONFIRM").unwrap_or_default() == "confirm" {
            log::warn!(
                "{}",
                "Bypassing deployment schema update confirmation due to env var".yellow()
            );
            input = "confirm".to_string();
        } else {
            println!("{}", "Type 'confirm' to update deployment schema version:".yellow());
            std::io::stdin().read_line(&mut input)?;
        }

        if input.trim() == "confirm" {
            // Initialize deployment database schema
            log::info!("{}", "Initializing deployment database schema...".blue());

            let schema_version = std::env::var("OMNI_ORCH_SCHEMA_VERSION")
                .unwrap_or_else(|_| "1".to_string())
                .parse::<i64>()
                .unwrap_or(1);

            match schemas::v1::db::init_deployment_schema(schema_version, pool).await {
                Ok(_) => {
                    log::info!("{}", "✓ Deployment database schema initialized".green());
                    // Convert i64 to String for set_meta_value
                    schemas::v1::db::queries::metadata::set_meta_value(
                        pool,
                        "omni_schema_version",
                        &target_version.to_string(),
                    )
                    .await
                    .expect("Failed to set deployment meta value")
                }
                Err(e) => log::error!(
                    "{}",
                    format!("Failed to initialize deployment database schema: {:?}", e).red()
                ),
            };

            // Initialize sample data for deployment database
            log::info!("{}", "Initializing deployment sample data...".blue());
            match schemas::v1::db::sample_data(pool, schema_version).await {
                Ok(_) => log::info!("{}", "✓ Deployment sample data initialized".green()),
                Err(e) => log::error!(
                    "{}",
                    format!("Failed to initialize deployment sample data: {:?}", e).red()
                ),
            };
        } else {
            log::warn!("{}", "Deployment schema update cancelled".yellow());
            return Err(anyhow::anyhow!("Deployment schema update cancelled by user"));
        }

        Ok(())
    }

    // Get all available platforms
    async fn get_all_platforms(&self) -> Result<Vec<schemas::v1::models::platform::Platform>, Error> {
        schemas::v1::db::queries::platforms::get_all_platforms(&self.main_pool)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to retrieve platform IDs: {:?}", e))
    }
}

// +---------------------+
// | CORS IMPLEMENTATION |
// +---------------------+
// CORS Fairing struct to add CORS headers to responses
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add comprehensive CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, PATCH, DELETE, OPTIONS, HEAD",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Authorization, Content-Type, Accept, Origin, X-Requested-With",
        ));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        response.set_header(Header::new("Access-Control-Max-Age", "86400")); // Cache preflight for 24 hours
    }
}

// CORS Preflight handler for OPTIONS requests
#[options("/<_..>")]
fn cors_preflight() -> &'static str {
    ""
}

// +-------------+
// | EXTENSIONS  |
// +-------------+
/// Extension trait for mounting multiple routes to a Rocket instance
trait RocketExt {
    /// Mount multiple route groups at once to simplify route registration
    ///
    /// # Arguments
    ///
    /// * `routes` - A vector of path and route pairs to mount
    fn mount_routes(self, routes: Vec<(&'static str, Vec<rocket::Route>)>) -> Self;
}

impl RocketExt for Rocket<Build> {
    fn mount_routes(self, routes: Vec<(&'static str, Vec<rocket::Route>)>) -> Self {
        let mut rocket = self;
        for (path, routes) in routes {
            log::info!("{}", format!("Mounting routes at {}", path).green());
            rocket = rocket.mount(path, routes);
        }
        rocket
    }
}

// +-------------+
// | GLOBALS     |
// +-------------+
// Global singleton instance of the cluster manager
// Manages node discovery and peer connections
lazy_static! {
    static ref CLUSTER_MANAGER: Arc<RwLock<ClusterManager>> = {
        let state = format!("{}:{}", SERVER_CONFIG.address, SERVER_CONFIG.port);

        let bind1 = &state;
        let bind = bind1.clone();
        let state = SharedState::new(bind.into());
        let shared_state = Arc::new(RwLock::new(state));
        Arc::new(RwLock::new(ClusterManager::new(shared_state)))
    };
}

// +-------------+
// | MODELS      |
// +-------------+
/// Message format for cluster status API responses
#[derive(Debug, Serialize, Deserialize)]
struct ClusterStatusMessage {
    /// Current role of the node (leader/follower)
    node_roles: String,
    /// List of nodes in the cluster
    cluster_nodes: Vec<NodeInfo>,
}

/// Standard API response format for cluster operations
#[derive(Serialize, Deserialize)]
struct ApiResponse {
    /// Status of the operation ("ok" or "error")
    status: String,
    /// Response message containing cluster information
    message: ClusterStatusMessage,
}

// +-------------+
// | METHODS     |
// +-------------+
impl ClusterManager {
    /// Discovers and connects to peer nodes in the cluster
    ///
    /// # Arguments
    ///
    /// * `config` - Server configuration containing instance information
    /// * `my_port` - Current node's port to avoid self-connection
    ///
    /// # Returns
    ///
    /// Result indicating success or failure of the discovery process
    pub async fn discover_peers(&self, config: &ServerConfig, my_port: u16) -> Result<()> {
        let client = Client::new();
        log::info!("{}", "Starting peer discovery...".cyan());

        for instance in &config.instances {
            let string = format!("{:#?}", instance);
            log::info!("{}", format!("Discovered: {}", string).blue().bold());
            if instance.port == my_port {
                log::debug!("Skipping self-connection at port {}", my_port);
                continue;
            }

            let node_address: Arc<str> = format!("{}:{}", instance.address, instance.port).into();
            let node_uri = format!("{}", node_address);

            match self.connect_to_peer(&client, &node_uri.clone()).await {
                Ok(_) => log::info!(
                    "{}",
                    format!("Successfully connected to peer: {}", node_uri).green()
                ),
                Err(e) => {
                    log::warn!(
                        "{}",
                        format!("Failed to connect to peer: {} {}", node_uri, e).yellow()
                    );
                    self.remove_node(node_uri.into()).await;
                }
            }
        }

        log::info!("{}", "Peer discovery completed".cyan());
        Ok(())
    }

    /// Attempts to establish a connection with a peer node
    ///
    /// # Arguments
    ///
    /// * `client` - HTTP client for making requests
    /// * `node_address` - Address of the peer node
    ///
    /// # Returns
    ///
    /// Result indicating success or failure of the connection attempt
    async fn connect_to_peer(&self, client: &Client, node_address: &str) -> Result<()> {
        let health_url = format!("{}/health", node_address);
        log::debug!("Checking health at: {}", health_url);

        let response = client
            .get(&health_url)
            .timeout(Duration::from_secs(5))
            .send()
            .await?;

        if response.status().is_success() {
            let port = node_address
                .split(':')
                .next_back()
                .unwrap_or("80")
                .parse::<u16>()
                .unwrap_or(80);

            let node_info = NodeInfo {
                id: node_address.into(),
                address: node_address.into(),
                port,
            };

            self.register_node(node_info).await;
            log::debug!("Node registered: {}", node_address);
            Ok(())
        } else {
            Err(anyhow!("Node health check failed"))
        }
    }
}

// +-------------+
// | ENDPOINTS   |
// +-------------+
/// Health check endpoint to verify node status
///
/// # Returns
///
/// JSON response with basic health status
#[get("/health")]
async fn health_check() -> rocket::serde::json::Json<ApiResponse> {
    log::debug!("Health check endpoint called");
    rocket::serde::json::Json(ApiResponse {
        status: "ok".to_string(),
        message: ClusterStatusMessage {
            node_roles: "unknown".to_string(),
            cluster_nodes: vec![],
        },
    })
}

/// Cluster status endpoint providing detailed information about the cluster
///
/// # Arguments
///
/// * `state` - Shared state containing leadership information
/// * `cluster` - Cluster manager containing node information
///
/// # Returns
///
/// JSON response with cluster status details
#[get("/cluster/status")]
async fn cluster_status(
    state: &rocket::State<Arc<RwLock<SharedState>>>,
    cluster: &rocket::State<Arc<RwLock<ClusterManager>>>,
) -> rocket::serde::json::Json<ApiResponse> {
    log::debug!("Cluster status endpoint called");
    let state = state.read().await;
    let nodes = cluster.read().await;

    let role = if state.is_leader {
        "leader".to_string()
    } else {
        "follower".to_string()
    };

    log::info!("{}", format!("Current node role: {}", role).cyan());

    let response = ApiResponse {
        status: "ok".to_string(),
        message: ClusterStatusMessage {
            node_roles: role,
            cluster_nodes: nodes.get_nodes().await,
        },
    };

    rocket::serde::json::Json(response)
}

// +-------------+
// | MAIN        |
// +-------------+
/// Main entry point for the OmniOrchestrator server
#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ====================== INITIALIZATION ======================
    let port = SERVER_CONFIG.port;
    println!(
        "{}",
        "╔═══════════════════════════════════════════════════════════════╗".bright_cyan()
    );
    println!(
        "{}",
        "║               OMNI ORCHESTRATOR SERVER STARTING               ║".bright_cyan()
    );
    println!(
        "{}",
        "╚═══════════════════════════════════════════════════════════════╝".bright_cyan()
    );
    println!("{}", format!("⇒ Starting server on port {}", port).green());

    // Setup logging
    Builder::new()
        .filter_level(log::LevelFilter::Info)
        .format(|buf, record| {
            // Get default style
            let _style = buf.default_level_style(record.level());
            writeln!(buf, "{}: {}", record.level(), format!("{}", record.args()))
        })
        .init();

    log::info!("{}", "Logger initialized successfully".green());

    // ====================== DATABASE SETUP ======================
    // Initialize database manager
    log::info!("{}", "Initializing database manager...".blue());
    let db_manager = Arc::new(DatabaseManager::new().await?);

    println!(
        "{}",
        "╔═══════════════════════════════════════════════════════════════╗".bright_blue()
    );
    println!(
        "{}",
        "║                 Platform Database Registration                ║".bright_blue()
    );
    println!(
        "{}",
        "╚═══════════════════════════════════════════════════════════════╝".bright_blue()
    );

    // Get all platforms
    let platforms = db_manager.get_all_platforms().await?;
    log::info!("{}", format!("Found {} platforms", platforms.len()).blue());

    // Pre-initialize connection pools for existing platforms if desired
    for platform in &platforms {
        log::info!(
            "{}",
            format!(
                "Pre-initializing connection for platform: {}",
                platform.name
            )
            .blue()
        );
        db_manager
            .get_platform_pool(&platform)
            .await?;
    }

    // Get main database pool for ClickHouse schema initialization
    let pool = db_manager.get_main_pool();

    // ======================= ClickHouse SETUP ======================
    println!(
        "{}",
        "╔═══════════════════════════════════════════════════════════════╗".blue()
    );
    println!(
        "{}",
        "║                  CLICKHOUSE CONNECTION                        ║".blue()
    );
    println!(
        "{}",
        "╚═══════════════════════════════════════════════════════════════╝".blue()
    );
    // Initialize ClickHouse connection pool
    let clickhouse_url = env::var("CLICKHOUSE_URL").unwrap_or_else(|_| {
        dotenv::dotenv().ok(); // Load environment variables from a .env file if available
        env::var("DEFAULT_CLICKHOUSE_URL").unwrap_or_else(|_| "http://localhost:8123".to_string())
    });
    log::info!("{}", format!("ClickHouse URL: {}", clickhouse_url).blue());
    log::info!("{}", "Initializing ClickHouse connection...".blue());

    // Modify your connection to include more debugging info
    let clickhouse_client = clickhouse::Client::default()
        .with_url(&clickhouse_url)
        .with_database("default")
        .with_user("default")
        .with_password("your_secure_password");

    // Add a simple ping test before attempting schema initialization
    match clickhouse_client.query("SELECT 1").execute().await {
        Ok(_) => log::info!("✓ ClickHouse connection test successful"),
        Err(e) => {
            log::error!("ClickHouse connection test failed: {:?}", e);
            panic!("Cannot connect to ClickHouse");
        }
    }

    log::info!("{}", "✓ ClickHouse connection established".green());
    log::info!("{}", "✓ ClickHouse connection pool initialized".green());

    // ====================== Schema SETUP ======================

    // Load schema based on version
    log::info!("{}", "Loading schema files...".blue());
    let schema_version =
        schemas::v1::db::queries::metadata::get_meta_value(pool, "omni_schema_version")
            .await
            .unwrap_or_else(|_| "1".to_string());

    let schema_path = format!("sql/v{}/up_clickhouse.sql", schema_version);
    log::info!(
        "{}",
        format!("Loading schema from path: {}", schema_path).blue()
    );

    // Initialize ClickHouse schema
    log::info!("{}", "Initializing ClickHouse schema...".blue());
    match api::logging::init_clickhouse_db(&clickhouse_client, &schema_path).await {
        Ok(_) => log::info!("{}", "✓ ClickHouse schema initialized".green()),
        Err(e) => {
            log::error!(
                "{}",
                format!("Failed to initialize ClickHouse schema: {:?}", e).red()
            );
            panic!("Failed to initialize ClickHouse schema");
        }
    };

    // ====================== CLUSTER SETUP ======================
    println!(
        "{}",
        "╔═══════════════════════════════════════════════════════════════╗".bright_magenta()
    );
    println!(
        "{}",
        "║                     CLUSTER MANAGEMENT                        ║".bright_magenta()
    );
    println!(
        "{}",
        "╚═══════════════════════════════════════════════════════════════╝".bright_magenta()
    );

    // Initialize node state and cluster management
    let node_id: Arc<str> =
        format!("{}:{}", SERVER_CONFIG.address.clone(), SERVER_CONFIG.port).into();
    log::info!("{}", format!("Node ID: {}", node_id).magenta());

    let shared_state: Arc<RwLock<SharedState>> =
        Arc::new(RwLock::new(SharedState::new(node_id.clone())));

    // Start peer discovery in background task
    log::info!("{}", "Starting peer discovery background task".magenta());
    tokio::task::spawn({
        let cluster_manager = CLUSTER_MANAGER.clone();
        let server_config = SERVER_CONFIG.clone();
        async move {
            loop {
                if let Err(e) = cluster_manager
                    .read()
                    .await
                    .discover_peers(&server_config, port)
                    .await
                {
                    log::error!("{}", format!("Failed to discover peers: {e}").red());
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
            }
        }
    });

    // ====================== AUTOSCALER SETUP ======================
    println!(
        "{}",
        "╔═══════════════════════════════════════════════════════════════╗".bright_yellow()
    );
    println!(
        "{}",
        "║                    AUTOSCALER SETUP                           ║".bright_yellow()
    );
    println!(
        "{}",
        "╚═══════════════════════════════════════════════════════════════╝".bright_yellow()
    );

    // Initialize worker autoscaler with default policy
    log::info!(
        "{}",
        "Creating worker autoscaler with default policy".yellow()
    );
    let policy = create_default_cpu_memory_scaling_policy();
    let mut autoscaler = WorkerAutoscaler::new(1, 1, policy);

    // Add cloud director for managing VMs
    log::info!("{}", "Adding cloud director (AWS/us-east-1)".yellow());
    let cloud_director = Arc::new(CloudDirector::new(
        "cloud-1".to_string(),
        "aws".to_string(),
        "us-east-1".to_string(),
    ));
    autoscaler.add_director(cloud_director);

    // Set up VM template for worker nodes
    log::info!("{}", "Setting up VM template for worker nodes".yellow());
    let mut vm_template = VMTemplate::default();
    vm_template.base_name = "omni-worker".to_string();
    vm_template.config = VMConfig {
        cpu: 2,
        memory: 4096, // 4GB
        storage: 80,  // 80GB
        options: HashMap::new(),
    };
    autoscaler.set_vm_template(vm_template);
    log::info!("{}", "✓ Worker autoscaler configured".green());

    // Start discovery tasks
    log::info!("{}", "Starting worker autoscaler discovery tasks".yellow());
    tokio::spawn({
        let mut autoscaler = autoscaler;
        async move {
            // Sleep for 1.5 seconds before starting discovery
            log::debug!("Sleeping for 1.5 seconds before discovery...");
            tokio::time::sleep(Duration::from_millis(1500)).await;
            loop {
                log::info!("{}", "Discovering nodes and VMs...".yellow());
                if let Err(e) = autoscaler.discover_nodes().await {
                    log::error!("{}", format!("Node discovery error: {}", e).red());
                }
                if let Err(e) = autoscaler.discover_vms().await {
                    log::error!("{}", format!("VM discovery error: {}", e).red());
                }
                let metrics: HashMap<String, f32> = HashMap::new(); // TODO: populate with actual metrics
                if let Err(e) = autoscaler.check_scaling(&metrics) {
                    log::error!("{}", format!("Worker scaling error: {}", e).red());
                }
                // Sleep for 3 seconds before next discovery
                log::debug!("Sleeping autoscaling thread for 3 seconds...");
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        }
    });

    // Initialize the app autoscaler
    log::info!(
        "{}",
        "Creating application autoscaler with default policy".yellow()
    );
    let app_policy = app_autoscaler::policy::create_default_cpu_memory_scaling_policy();
    let app_autoscaler = app_autoscaler::app_autoscaler::AppAutoscaler::new(
        1,  // min instances
        10, // max instances
        app_policy,
    );
    log::info!("{}", "✓ Application autoscaler configured".green());

    // Spawn a task to run the app autoscaler discovery and scaling loop
    log::info!(
        "{}",
        "Starting application autoscaler discovery tasks".yellow()
    );
    tokio::spawn({
        let mut app_autoscaler = app_autoscaler;
        async move {
            // Sleep for 1.5 seconds before starting discovery
            log::debug!("Sleeping for 1.5 seconds before app autoscaler discovery...");
            tokio::time::sleep(Duration::from_millis(1500)).await;
            loop {
                log::info!("{}", "Discovering app instances...".yellow());
                if let Err(e) = app_autoscaler.discover_app_instances().await {
                    log::error!("{}", format!("App instance discovery error: {}", e).red());
                }

                let metrics: HashMap<String, f32> = HashMap::new(); // TODO: populate with actual metrics
                if let Err(e) = app_autoscaler.check_scaling(&metrics) {
                    log::error!("{}", format!("App scaling error: {}", e).red());
                }

                // Sleep for 3 seconds before next discovery
                log::debug!("Sleeping app autoscaling thread for 3 seconds...");
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        }
    });

    // ====================== LEADER ELECTION ======================
    println!(
        "{}",
        "╔═══════════════════════════════════════════════════════════════╗".bright_green()
    );
    println!(
        "{}",
        "║                      LEADER ELECTION                          ║".bright_green()
    );
    println!(
        "{}",
        "╚═══════════════════════════════════════════════════════════════╝".bright_green()
    );

    // Initialize and start leader election
    log::info!("{}", "Initializing leader election process".green());
    let _leader_election = LeaderElection::new(node_id, shared_state.clone());
    log::info!("{}", "✓ Leader election initialized".green());

    // ====================== SERVER STARTUP ======================
    println!(
        "{}",
        "╔═══════════════════════════════════════════════════════════════╗".bright_cyan()
    );
    println!(
        "{}",
        "║                       SERVER STARTUP                          ║".bright_cyan()
    );
    println!(
        "{}",
        "╚═══════════════════════════════════════════════════════════════╝".bright_cyan()
    );

    // Define routes to mount
    log::info!("{}", "Defining API routes".cyan());
    let routes = vec![
        (
            "/",
            routes![
                health_check,
                api::index::routes_ui,
                cluster_status,
                cors_preflight
            ],
        ),
        ("/api/v1", api::routes()),
    ];

    let auth_config = AuthConfig {
        jwt_secret: std::env::var("JWT_SECRET")
            .expect("Environment variable JWT_SECRET must be set for secure operation."),
        token_expiry_hours: std::env::var("TOKEN_EXPIRY_HOURS")
            .unwrap_or_else(|_| "24".to_string())
            .parse()
            .expect("Invalid value for TOKEN_EXPIRY_HOURS"),
    };

    // Build Rocket instance with base configuration
    log::info!("{}", "Building Rocket instance".cyan());
    let rocket_instance = rocket::build()
        .configure(rocket::Config {
            port,
            address: std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
            ..Default::default()
        })
        // Add database manager to Rocket's state
        .manage(db_manager.clone())
        .manage(CLUSTER_MANAGER.clone())
        .manage(clickhouse_client)
        .manage(shared_state)
        .manage(auth_config)
        .attach(CORS); // Attach the CORS fairing

    // Mount routes to the Rocket instance
    log::info!("{}", "Mounting API routes".cyan());
    let rocket_with_routes = rocket_instance.mount_routes(routes);

    // Collect routes information before launch
    api::index::collect_routes(&rocket_with_routes);

    // Launch server
    log::info!("{}", "🚀 LAUNCHING SERVER...".bright_cyan().bold());
    let _rocket = rocket_with_routes.launch().await?;

    Ok(())
}