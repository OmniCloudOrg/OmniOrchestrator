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
// mod db;
// mod api;
mod state;
mod leader;
mod config;
// mod backup;
mod network;
mod cluster;
mod app_autoscaler;
mod worker_autoscaler;

mod schemas;

// use api::auth::AuthConfig;
// +-------------+
// | IMPORTS     |
// +-------------+
// Third-party dependencies
use rocket::Build;
use anyhow::anyhow;
use rocket::Rocket;
use anyhow::Result;
use schemas::auth::AuthConfig;
use std::io::Write;
use reqwest::Client;
use colored::Colorize;
use env_logger::Builder;
use tokio::sync::RwLock;
use std::time::Duration;
use std::{env, sync::Arc};
use sqlx::mysql::MySqlPool;
use lazy_static::lazy_static;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use worker_autoscaler::WorkerAutoscaler;
use worker_autoscaler::{VMTemplate, VMConfig, CloudDirector};
use worker_autoscaler::create_default_cpu_memory_scaling_policy;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

// Internal imports
use crate::state::SharedState;
use crate::config::ServerConfig;
use crate::config::SERVER_CONFIG;
use crate::leader::LeaderElection;
use crate::cluster::{ClusterManager, NodeInfo};

use schemas::v1::{
    models,
    api,
    db
};

// We ignore this import as it always says
// unused even when that is not the case
#[allow(unused_imports)]
// Import all API routes
// use api::*;

#[macro_use]
extern crate rocket;

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
            "GET, POST, PUT, PATCH, DELETE, OPTIONS, HEAD"
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers", 
            "Authorization, Content-Type, Accept, Origin, X-Requested-With"
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
                Ok(_) => log::info!("{}", format!("Successfully connected to peer: {}", node_uri).green()),
                Err(e) => {
                    log::warn!("{}", format!("Failed to connect to peer: {} {}", node_uri, e).yellow());
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
    println!("{}", "╔═══════════════════════════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║               OMNI ORCHESTRATOR SERVER STARTING               ║".bright_cyan());
    println!("{}", "╚═══════════════════════════════════════════════════════════════╝".bright_cyan());
    println!("{}", format!("⇒ Starting server on port {}", port).green());

    // Setup logging
    // let file = File::create(format!("cluster-{}.log", port))?;
    Builder::new()
        //.target(Target::Pipe(Box::new(file)))
        .filter_level(log::LevelFilter::Info)
        .format(|buf, record| {
            let _style = buf.style();
            // Get default style
            let style = buf.default_level_style(record.level());
            writeln!(
                buf,
                "{}: {}",
                record.level(),
                style.value(format!("{}", record.args()))
            )
        })
        .init();

    log::info!("{}", "Logger initialized successfully".green());

    // ====================== DATABASE SETUP ======================
    println!("{}", "╔═══════════════════════════════════════════════════════════════╗".bright_blue());
    println!("{}", "║                    DATABASE CONNECTION                        ║".bright_blue());
    println!("{}", "╚═══════════════════════════════════════════════════════════════╝".bright_blue());
    
    // Initialize database pool
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| {
            dotenv::dotenv().ok(); // Load environment variables from a .env file if available
            env::var("DEFAULT_DATABASE_URL").unwrap_or_else(|_| "mysql://root:root@localhost:4001/omni".to_string())
        });

    log::info!("{}", format!("Database URL: {}", database_url).blue());
    log::info!("{}", "Initializing database connection pool...".blue());
    
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to MySQL database");
    log::info!("{}", "✓ Database connection established".green());

    // Initialize metadata system properly with mutex protection
    //
    // The metadata system is used to store and manage metadata for the OmniCloud
    // platform. It is initialized with a connection pool to the database as it lives
    // outside of the database schema and is used by the platform to determine
    // when to update the schema and initialize sample data before we actually
    // touch any data or start the API.
    log::info!("{}", "Initializing metadata system...".blue());
    schemas::v1::db::queries::metadata::initialize_metadata_system(&pool).await?;
    log::info!("{}", "✓ Metadata system initialized".green());

    // Check database schema version and update if necessary
    //
    // This section checks the current schema version against the target version
    // If they differ, it prompts the user for confirmation before proceeding
    // with the schema update and sample data initialization
    let target_version = "1";
    let current_version = schemas::v1::db::queries::metadata::get_meta_value(&pool, "omni_schema_version")
        .await
        .unwrap_or_else(|_| "0".to_string());

    if current_version != target_version {
        let mut input = String::new();
        log::warn!("{}", format!("Schema version mismatch! Current: {}, Target: {}", current_version, target_version).yellow());
        if env::var("OMNI_ORCH_BYPASS_CONFIRM").unwrap_or_default() == "confirm" {
            log::warn!("{}", "Bypassing schema update confirmation due to env var".yellow());
            input = "confirm".to_string();
        } else {
            println!("{}", "Type 'confirm' to update schema version:".yellow());
            std::io::stdin().read_line(&mut input)?;
        }

        if input.trim() == "confirm" {
            // Initialize database schema
            log::info!("{}", "Initializing database schema...".blue());
            match schemas::v1::db::init_schema(1, &pool).await {
                Ok(_) => {
                    log::info!("{}", "✓ Database schema initialized".green());
                    schemas::v1::db::queries::metadata::set_meta_value(
                        &pool,
                        "omni_schema_version",
                        target_version,
                    )
                    .await
                    .expect("Failed to set meta value")
                }
                Err(e) => log::error!("{}", format!("Failed to initialize database schema: {:?}", e).red()),
            };

            // Initialize sample data for the schema
            log::info!("{}", "Initializing sample data...".blue());
            match schemas::v1::db::sample_data(&pool).await {
                Ok(_) => log::info!("{}", "✓ Sample data initialized".green()),
                Err(e) => log::error!("{}", format!("Failed to initialize sample data: {:?}", e).red()),
            };
        } else {
            log::warn!("{}", "Schema update cancelled".yellow());
            return Ok(());
        }
    } else {
        log::info!("{}", format!("Schema version check: OK (version {})", current_version).green());
    }

    // ====================== CLUSTER SETUP ======================
    println!("{}", "╔═══════════════════════════════════════════════════════════════╗".bright_magenta());
    println!("{}", "║                     CLUSTER MANAGEMENT                        ║".bright_magenta());
    println!("{}", "╚═══════════════════════════════════════════════════════════════╝".bright_magenta());

    // Initialize node state and cluster management
    let node_id: Arc<str> =
        format!("{}:{}", SERVER_CONFIG.address.clone(), SERVER_CONFIG.port).into();
    log::info!("{}", format!("Node ID: {}", node_id).magenta());
    
    let shared_state: Arc<RwLock<SharedState>> =
        Arc::new(RwLock::new(SharedState::new(node_id.clone())));

    // Start peer discovery in background task
    //
    // This task will run in the background and periodically check for peer nodes
    // in the cluster, performing discovery and connection operations
    // for each discovered node. It will also log any errors encountered during
    // discovery or connection operations.
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
    println!("{}", "╔═══════════════════════════════════════════════════════════════╗".bright_yellow());
    println!("{}", "║                    AUTOSCALER SETUP                           ║".bright_yellow());
    println!("{}", "╚═══════════════════════════════════════════════════════════════╝".bright_yellow());

    // Initialize worker autoscaler with default policy
    log::info!("{}", "Creating worker autoscaler with default policy".yellow());
    let policy = create_default_cpu_memory_scaling_policy();
    let mut autoscaler = WorkerAutoscaler::new(1, 1, policy);

    // Add cloud director for managing VMs
    log::info!("{}", "Adding cloud director (AWS/us-east-1)".yellow());
    let cloud_director = Arc::new(CloudDirector::new(
        "cloud-1".to_string(),
        "aws".to_string(), 
        "us-east-1".to_string()
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
        options: HashMap::new()
    };
    autoscaler.set_vm_template(vm_template);
    log::info!("{}", "✓ Worker autoscaler configured".green());

    // Start discovery tasks
    //
    // This task will run in the background and periodically check for nodes
    // and VMs in the cluster, performing discovery and scaling operations
    // for worker nodes. It will also log any errors encountered during
    // discovery or scaling operations.
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
    log::info!("{}", "Creating application autoscaler with default policy".yellow());
    let app_policy = app_autoscaler::policy::create_default_cpu_memory_scaling_policy();
    let app_autoscaler = app_autoscaler::app_autoscaler::AppAutoscaler::new(
        1, // min instances
        10, // max instances
        app_policy,
    );
    log::info!("{}", "✓ Application autoscaler configured".green());

    // Spawn a task to run the app autoscaler discovery and scaling loop
    //
    // This task will run in the background and periodically check for app instances
    // and perform scaling operations based on the configured policy
    // It will also log any errors encountered during discovery or scaling
    // operations.
    log::info!("{}", "Starting application autoscaler discovery tasks".yellow());
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
    println!("{}", "╔═══════════════════════════════════════════════════════════════╗".bright_green());
    println!("{}", "║                      LEADER ELECTION                          ║".bright_green());
    println!("{}", "╚═══════════════════════════════════════════════════════════════╝".bright_green());

    // Initialize and start leader election
    log::info!("{}", "Initializing leader election process".green());
    let _leader_election = LeaderElection::new(node_id, shared_state.clone());
    log::info!("{}", "✓ Leader election initialized".green());

    // ====================== SERVER STARTUP ======================
    println!("{}", "╔═══════════════════════════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║                       SERVER STARTUP                          ║".bright_cyan());
    println!("{}", "╚═══════════════════════════════════════════════════════════════╝".bright_cyan());

    // Define routes to mount
    log::info!("{}", "Defining API routes".cyan());
    let routes = vec![
        ("/", routes![health_check, api::index::routes_ui, cluster_status, cors_preflight]), 
        ("/api/v1", api::routes())
    ];

    let auth_config = AuthConfig {
        jwt_secret: std::env::var("JWT_SECRET").expect("Environment variable JWT_SECRET must be set for secure operation."),
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

        // Add database pool to Rocket's state (used by any route that needs to talk to the database) can be used in a route like:
        // #[get("/apps/count")]
        // pub async fn count_apps(pool: &State<sqlx::Pool<MySql>>) -> Json<i64> {
        //     let count = db::app::count_apps(pool).await.unwrap();
        //     Json(count)
        // }
        .manage(auth_config)
        .manage(pool)
        .manage(shared_state)
        .manage(CLUSTER_MANAGER.clone())
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