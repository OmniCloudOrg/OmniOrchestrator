mod cluster;
mod config;
mod logger;
mod leader;
mod state;
mod api;
mod db;

use serde::{Deserialize, Serialize};
use env_logger::{Builder, Target};
use crate::config::SERVER_CONFIG;
use rocket::{self, get, routes};
use std::collections::HashMap;
use lazy_static::lazy_static;
use sqlx::mysql::MySqlPool;
use v1::apps::Application;
use std::{env, sync::Arc};
use rocket::yansi::Paint;
use std::time::Duration;
use tokio::sync::RwLock;
use colored::Colorize;
use reqwest::Client;
use anyhow::anyhow;
use anyhow::Result;
use std::io::Write;
use std::fs::File;

use crate::cluster::{ClusterManager, NodeInfo};
use crate::leader::LeaderElection;
use crate::config::ServerConfig;
use crate::state::SharedState;

use api::*;

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

#[derive(Debug, Serialize, Deserialize)]
struct ClusterStatusMessage {
    node_roles: String,
    cluster_nodes: Vec<NodeInfo>
}

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    status: String,
    message: ClusterStatusMessage
}

impl ClusterManager {
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

    async fn connect_to_peer(&self, client: &Client, node_address: &str) -> Result<()> {
        let health_url = format!("{}/health", node_address);
        let response = client.get(&health_url).timeout(Duration::from_secs(5)).send().await?;

        if response.status().is_success() {
            let port = node_address.split(':').next_back().unwrap_or("80").parse::<u16>().unwrap_or(80);

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

#[get("/health")]
async fn health_check() -> rocket::serde::json::Json<ApiResponse> {
    rocket::serde::json::Json(ApiResponse {
        status: "ok".to_string(),
        message: ClusterStatusMessage {
            node_roles: "unknown".to_string(),
            cluster_nodes: vec![]
        }
    })
}

#[get("/cluster/status")]
async fn cluster_status(
    state: &rocket::State<Arc<RwLock<SharedState>>>,
    cluster: &rocket::State<Arc<RwLock<ClusterManager>>>
) -> rocket::serde::json::Json<ApiResponse> {
    let state = state.read().await;
    let nodes = cluster.read().await;

    let response = ApiResponse {
        status: "ok".to_string(),
        message: ClusterStatusMessage {
            node_roles: if state.is_leader { "leader".to_string() } else { "follower".to_string() },
            cluster_nodes: nodes.get_nodes().await
        }
    };

    rocket::serde::json::Json(response)
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = SERVER_CONFIG.port;
    println!("Starting server on port {}", port);
    env::set_var("RUST_LOG", "trace");

    // Setup logging
    let file = File::create(format!("cluster-{}.log", port))?;
    Builder::new()
        .target(Target::Pipe(Box::new(file)))
        .filter_level(log::LevelFilter::Info)
        .format(|buf, record| {
            let style = buf.style();
            style.resetting();
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
                    db::v1::queries::metadata::set_meta_value(&pool, "omni_schema_version", target_version)
                        .await
                        .expect("Failed to set meta value")
                }
                Err(e) => println!("Failed to initialize database schema: {:?}", e)
            };

            // Initialize sample data for the schema
            println!("initializing sample data...");
            match db::sample_data(&pool).await {
                Ok(_) => println!("Sample data initialized"),
                Err(e) => println!("Failed to initialize sample data: {:?}", e)
            };
        } else {
            println!("Schema update cancelled");
            return Ok(());
        }
    }

    // Initialize node state and cluster management
    let node_id: Arc<str> = format!("{}:{}", SERVER_CONFIG.address.clone(), SERVER_CONFIG.port).into();
    let shared_state: Arc<RwLock<SharedState>> = Arc::new(RwLock::new(SharedState::new(node_id.clone())));

    // Start peer discovery
    tokio::task::spawn({
        let cluster_manager = CLUSTER_MANAGER.clone();
        let server_config = SERVER_CONFIG.clone();
        async move {
            loop {
                if let Err(e) = cluster_manager.read().await.discover_peers(&server_config, port).await {
                    log::error!("Failed to discover peers: {e}");
                }
                tokio::time::sleep(Duration::from_millis(200)).await;
            }
        }
    });

    // Initialize and start leader election
    let leader_election = LeaderElection::new(
        node_id,
        shared_state.clone(),
    );

    tokio::spawn(async move {
        leader_election.start().await;
    });

    // Initialize application state
    let applications_state: Arc<RwLock<HashMap<String, Application>>> = Arc::new(RwLock::new(HashMap::new()));

    // Build and launch Rocket
    let _rocket = rocket::build()
        .configure(rocket::Config {
            port,
            address: std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
            ..Default::default()
        })  
        .manage(pool)  // Add database pool to Rocket's state
        .manage(shared_state)
        .manage(CLUSTER_MANAGER.clone())
        .manage(applications_state)
        .mount("/", routes![health_check, cluster_status])
        .mount("/api/v1", api::v1::routes())
        .launch()
        .await?;
    Ok(())
}