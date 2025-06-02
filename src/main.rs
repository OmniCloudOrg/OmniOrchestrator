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
mod cluster;
mod network;
mod db_manager;
mod schemas;

// +-------------+
// | IMPORTS     |
// +-------------+
// Third-party dependencies
use anyhow::anyhow;
use anyhow::Result;
use clickhouse;
use colored::Colorize;
use core::panic;
use env_logger::Builder;
use lazy_static::lazy_static;
use reqwest::Client;
use rocket::{
    Request,
    Response,
    Rocket,
    Build,
    http::Header,
    fairing::{
        Fairing,
        Info,
        Kind
    }
};
use libomni::types::db::auth::AuthConfig;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;
use sqlx::{MySql, Pool};
use std::collections::HashMap;
use std::io::Write;
use std::time::Duration;
use std::{env, sync::Arc};
use tokio::sync::RwLock;

// Internal imports
use crate::cluster::{ClusterManager, NodeInfo};
use crate::config::ServerConfig;
use crate::config::SERVER_CONFIG;
use crate::leader::LeaderElection;
use crate::state::SharedState;
use crate::db_manager::DatabaseManager; // New database manager import

use libomni::types::db::v1 as types;
use types::platform;
use types::platform::Platform;

use schemas::v1::api;

// We ignore this import as it always says
// unused even when that is not the case
#[allow(unused_imports)]
#[macro_use]
extern crate rocket;

pub static PROJECT_ROOT: &str = env!("CARGO_MANIFEST_DIR");

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

    // Get the database URL from environment or use default
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        dotenv::dotenv().ok();
        env::var("DEFAULT_DATABASE_URL")
            .unwrap_or_else(|_| "mysql://root:root@localhost:4001".to_string())
    });

    // Initialize database manager
    log::info!("{}", format!("Database URL: {}", database_url).blue());
    let db_manager = Arc::new(DatabaseManager::new(&database_url).await?);
    
    // Get main database pool for further operations
    let pool = db_manager.get_main_pool();

    // Platform database initialization
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

    // Get all platforms and initialize their database pools
    let platforms = db_manager.get_all_platforms().await?;
    log::info!("{}", format!("Found {} platforms", platforms.len()).blue());

    for platform in &platforms {
        log::info!(
            "{}",
            format!(
                "Pre-initializing connection for platform: {}",
                platform.name
            )
            .blue()
        );
        db_manager.get_platform_pool(&platform.name, platform.id.unwrap_or(0)).await?;
    }

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

    let schema_path = format!("{}/sql/v{}/clickhouse_up.sql", PROJECT_ROOT, schema_version);
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
        "╚═══════════════════════════════════════════════════════════════╝".bright_magenta()
    );

    // TODO: Implement autoscaler setup via Lighthouse: https://github.com/OmniCloudOrg/Lighthouse

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
        "╚═══════════════════════════════════════════════════════════════╝".bright_magenta()
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
        "╚═══════════════════════════════════════════════════════════════╝".bright_magenta()
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
        .manage(pool.clone())  // This is the pool for the core deployment
                               // database. This is meant to store SYSTEM METADATA and not platform-specific data.
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