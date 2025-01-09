// Import the necessary modules
mod logger;
mod cluster;
mod leader;
mod config;
mod state;
mod api;
mod db;


use logger::LOGGER as LOGGER_BOX;
// Import third-party dependen  cies
use serde::{ Deserialize, Serialize };
use rocket::{ self, get, routes };
use v1::apps::Application;
use crate::config::SERVER_CONFIG;
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::{ env, sync::Arc };
use tokio::sync::RwLock;
use std::time::Duration;
use colored::Colorize;
use reqwest::Client;
use anyhow::anyhow;
use anyhow::Result;

// Import local dependencies
use crate::cluster::{ ClusterManager, NodeInfo };
use env_logger::{Builder, Target};
use crate::leader::LeaderElection;
use crate::config::ServerConfig;
use crate::state::SharedState;
use std::fmt;
use std::io::Write;
use std::fs::File;

// Import Routes
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
    cluster_nodes: Vec<NodeInfo> // Replace with your actual Node type
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
            // Skip if this is our own instance
            if instance.port == my_port {
                continue;
            }

            let node_address: Arc<str> = format!("{}:{}", instance.address, instance.port).into();
            let node_uri = format!("{}", node_address);

            // Try to connect to each peer
            match self.connect_to_peer(&client, &node_uri.clone()).await {
                Ok(_) => log::info!("Successfully connected to peer: {}", node_uri),
                Err(e) => {
                    log::warn!("Failed to connect to peer: {} {}", node_uri, e);
                    // Remove dead node from cluster. TODO: We may eventually
                    // want to keep track of dead nodes and poll them less
                    // frequently, in case of recovery.
                    self.remove_node(node_uri.into()).await;
                }
            }
        }

        Ok(())
    }

    async fn connect_to_peer(&self, client: &Client, node_address: &str) -> Result<()> {
        let health_url = format!("{}/health", node_address);

        // First check if the node is healthy
        let response = client.get(&health_url).timeout(Duration::from_secs(5)).send().await?;

        if response.status().is_success() {
            // If healthy, add to cluster
            // Extract port from node_address
            let port = node_address.split(':').last().unwrap_or("80").parse::<u16>().unwrap_or(80);

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
    use rocket::yansi::Paint;
    use env_logger::fmt::Color;
    let port = SERVER_CONFIG.port;
    env::set_var("RUST_LOG", "trace");

    let file = File::create(format!("cluster-{}.log", port)).unwrap();
    use colored::Colorize;
    
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
    
    db::init_db().expect("Failed to initialize database");
    // db::init_sample_data().expect("Failed to initialize sample data");
    db::queries::build_create(27, "testingversion3").expect("Failed to build create queries");

    let node_id: Arc<str> = format!("{}:{}", SERVER_CONFIG.address.clone(), SERVER_CONFIG.port).into();
    let shared_state: Arc<RwLock<SharedState>> = Arc::new(RwLock::new(SharedState::new(node_id.clone())));

    tokio::task::spawn(async move {
        loop {
            if let Err(e) = CLUSTER_MANAGER.read().await.discover_peers(&SERVER_CONFIG, port).await {
                log::error!("Failed to discover peers: {e}");
            }
        }
    });

    let leader_election = LeaderElection::new(
        node_id.into(),
        shared_state.clone(),
    );

    // Start leader election process
    tokio::spawn(async move {
        leader_election.start().await;
    });

    let applications_state: Arc<RwLock<HashMap<String, Application>>> = Arc::new(RwLock::new(HashMap::new()));
    let _rocket = rocket
        ::build()
        .configure(rocket::Config { port, address: std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)), ..Default::default() })
        .manage(shared_state)
        .manage(CLUSTER_MANAGER.clone())
        .manage(applications_state)
        // Core routes
        .mount("/", routes![health_check, cluster_status])
        // API v1 routes
        .mount("/api/v1", api::v1::routes())
        .launch().await?;
    Ok(())
}
