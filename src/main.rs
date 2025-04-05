//-----------------------------------------------------------------------------
// OmniOrchestrator - A distributed system for managing and orchestrating
//-----------------------------------------------------------------------------
// Maintained by: Tristan J. Poland, Maxine Deandrade, Caznix, Haywood Spartian
// and the OmniCloud community.
//-----------------------------------------------------------------------------
// This is the entry point for the OmniOrchestrator server application.
// It is responsible of managing the entirity of the OmniCloud platform
// and its various components, including the database, API, and cluster
// management. It also handles bootstrapping when a new platform is created
// and provides a load balanced API for interacting with the platform's
// various components.
//-----------------------------------------------------------------------------

mod api;
mod cluster;
mod config;
mod db;
mod leader;
mod logger;
mod state;
mod worker_autoscaler;
mod app_autoscaler;

// Import Third-party crates
use anyhow::anyhow;
use anyhow::Result;
use colored::Colorize;
use env_logger::{Builder, Target};
use lazy_static::lazy_static;
use reqwest::Client;
use rocket::serde::json::Json;
use rocket::Build;
use rocket::Rocket;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;
use worker_autoscaler::create_default_cpu_memory_scaling_policy;
use worker_autoscaler::WorkerAutoscaler;
use worker_autoscaler::{VMTemplate, VMConfig, CloudDirector};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use std::{env, sync::Arc};
use tokio::sync::RwLock;



// Import other pieces of modules for use
use crate::cluster::{ClusterManager, NodeInfo};
use crate::config::ServerConfig;
use crate::config::SERVER_CONFIG;
use crate::leader::LeaderElection;
use crate::state::SharedState;
use api::*; // We import this so we can mount the various routes for different API versions

#[macro_use]
extern crate rocket;

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
            println!("Mounting routes at {}", path);
            rocket = rocket.mount(path, routes);
        }
        rocket
    }
}


/// Global singleton instance of the cluster manager
/// Manages node discovery and peer connections
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

        for instance in &config.instances {
            let string = format!("{:#?}", instance);
            println!("discovered: {}", string.blue().bold());
            if instance.port == my_port {
                continue;
            }

            let node_address: Arc<str> = format!("{}:{}", instance.address, instance.port).into();
            let node_uri = format!("{}", node_address);

            match self.connect_to_peer(&client, &node_uri.clone()).await {
                Ok(_) => log::info!("Successfully connected to peer: {}", node_uri),
                Err(e) => {
                    log::warn!("Failed to connect to peer: {} {}", node_uri, e);
                    self.remove_node(node_uri.into()).await;
                }
            }
        }

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
        let health_url = format!("http://{}/health", node_address);
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
            Ok(())
        } else {
            Err(anyhow!("Node health check failed"))
        }
    }
}


/// Health check endpoint to verify node status
///
/// # Returns
///
/// JSON response with basic health status
#[get("/health")]
async fn health_check() -> rocket::serde::json::Json<ApiResponse> {
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
    let state = state.read().await;
    let nodes = cluster.read().await;

    let response = ApiResponse {
        status: "ok".to_string(),
        message: ClusterStatusMessage {
            node_roles: if state.is_leader {
                "leader".to_string()
            } else {
                "follower".to_string()
            },
            cluster_nodes: nodes.get_nodes().await,
        },
    };

    rocket::serde::json::Json(response)
}

/// Main entry point for the OmniOrchestrator server
///
/// Initializes the server components in the following order:
/// 1. Server configuration and logging
/// 2. Database connection
/// 3. Schema verification and optional migration
/// 4. Cluster management and peer discovery
/// 5. Leader election
/// 6. API route mounting and server start
#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = SERVER_CONFIG.port;
    println!("Starting server on port {}", port);

    // Setup logging
    // let file = File::create(format!("cluster-{}.log", port))?;
    Builder::new()
        //.target(Target::Pipe(Box::new(file)))
        .filter_level(log::LevelFilter::Info)
        .format(|buf, record| {
            let style = buf.style();
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

    // Initialize database pool
    println!("Connecting to database...");
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root:root@localhost:4001/omni".to_string());

    println!("Database URL: {}", database_url);
    println!("Initializing database connection pool...");
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to MySQL database");

    // Initialize metadata system properly with mutex protection
    println!("Initializing metadata system...");
    db::v1::queries::metadata::initialize_metadata_system(&pool).await?;

    // Check database schema version and update if necessary
    let target_version = "1";
    let current_version = db::v1::queries::metadata::get_meta_value(&pool, "omni_schema_version")
        .await
        .unwrap_or_else(|_| "0".to_string());

    if current_version != target_version {
        println!("Current schema version: {}", current_version);
        println!("Target schema version: {}", target_version);
        println!("Type 'confirm' to update schema version:");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if input.trim() == "confirm" {
            // Initialize database schema
            println!("Initializing database schema...");
            match db::init_schema(1, &pool).await {
                Ok(_) => {
                    println!("Database schema initialized");
                    db::v1::queries::metadata::set_meta_value(
                        &pool,
                        "omni_schema_version",
                        target_version,
                    )
                    .await
                    .expect("Failed to set meta value")
                }
                Err(e) => println!("Failed to initialize database schema: {:?}", e),
            };

            // Initialize sample data for the schema
            println!("initializing sample data...");
            match db::sample_data(&pool).await {
                Ok(_) => println!("Sample data initialized"),
                Err(e) => println!("Failed to initialize sample data: {:?}", e),
            };
        } else {
            println!("Schema update cancelled");
            return Ok(());
        }
    }

    // Initialize node state and cluster management
    let node_id: Arc<str> =
        format!("{}:{}", SERVER_CONFIG.address.clone(), SERVER_CONFIG.port).into();
    let shared_state: Arc<RwLock<SharedState>> =
        Arc::new(RwLock::new(SharedState::new(node_id.clone())));

    // Start peer discovery in background task
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
                    log::error!("Failed to discover peers: {e}");
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
            }
        }
    });

    // Initialize worker autoscaler with default policy
    let policy = create_default_cpu_memory_scaling_policy();
    let mut autoscaler = WorkerAutoscaler::new(1, 1, policy);

    // Add cloud director for managing VMs
    let cloud_director = Arc::new(CloudDirector::new(
        "cloud-1".to_string(),
        "aws".to_string(), 
        "us-east-1".to_string()
    ));
    autoscaler.add_director(cloud_director);

    // Set up VM template for worker nodes
    let mut vm_template = VMTemplate::default();
    vm_template.base_name = "omni-worker".to_string();
    vm_template.config = VMConfig {
        cpu: 2,
        memory: 4096, // 4GB
        storage: 80,  // 80GB
        options: HashMap::new()
    };
    autoscaler.set_vm_template(vm_template);

    // Start discovery tasks
    tokio::spawn({
        let mut autoscaler = autoscaler;
        async move {
            // Sleep for 1.5 seconds before starting discovery
            println!("Sleeping for 1.5 seconds before discovery...");
            tokio::time::sleep(Duration::from_millis(1500)).await;
            loop {
                println!("Discovering nodes and VMs...");
                if let Err(e) = autoscaler.discover_nodes().await {
                    error!("Node discovery error: {}", e);
                }
                if let Err(e) = autoscaler.discover_vms().await {
                    error!("VM discovery error: {}", e); 
                }
                let metrics: HashMap<String, f32> = HashMap::new(); // TODO: populate with actual metrics
                if let Err(e) = autoscaler.check_scaling(&metrics) {
                    error!("Worker scaling error: {}", e);
                }
                // Sleep for 30 seconds before next discovery
                println!("Sleeping autoscaling thread for 3 seconds...");
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        }
    });

    // initialize the app autoscalar
    let app_policy = app_autoscaler::policy::create_default_cpu_memory_scaling_policy();
    let app_autoscaler = app_autoscaler::app_autoscaler::AppAutoscaler::new(
        1, // min instances
        10, // max instances
        app_policy,
    );

    // Spawn a task to run the app autoscaler discovery and scaling loop
    tokio::spawn({
        let mut app_autoscaler = app_autoscaler;
        async move {
            // Sleep for 1.5 seconds before starting discovery
            println!("Sleeping for 1.5 seconds before app autoscaler discovery...");
            tokio::time::sleep(Duration::from_millis(1500)).await;
            loop {
                println!("Discovering app instances...");
                if let Err(e) = app_autoscaler.discover_app_instances().await {
                    error!("App instance discovery error: {}", e);
                }

                let metrics: HashMap<String, f32> = HashMap::new(); // TODO: populate with actual metrics
                if let Err(e) = app_autoscaler.check_scaling(&metrics) {
                    error!("App scaling error: {}", e);
                }
                
                // Sleep for 30 seconds before next discovery
                println!("Sleeping app autoscaling thread for 3 seconds...");
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        }
    });

    // Initialize and start leader election
    let leader_election = LeaderElection::new(node_id, shared_state.clone());

    // Define routes to mount
    let routes = vec![
        ("/", routes![health_check, api::index::routes_ui, cluster_status]), 
        ("/api/v1", api::v1::routes())
    ];

    // Build Rocket instance with base configuration
    let rocket_instance = rocket::build()
        .configure(rocket::Config {
            port,
            address: std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
            ..Default::default()
        })
        .manage(pool) // Add database pool to Rocket's state
        .manage(shared_state)
        .manage(CLUSTER_MANAGER.clone());

    // Mount routes to the Rocket instance
    let rocket_with_routes = rocket_instance.mount_routes(routes);
    
    // Collect routes information before launch
    api::index::collect_routes(&rocket_with_routes);
    
    // Launch server
    let _rocket = rocket_with_routes.launch().await?;
    
    Ok(())
}