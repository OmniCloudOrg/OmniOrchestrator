use rocket::{self, get, routes};
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;
use reqwest::Client;
use std::time::Duration;
mod state;
mod leader;
mod cluster;
mod config;

use crate::cluster::{ClusterManager, NodeInfo};
use crate::leader::LeaderElection;
use crate::state::SharedState;
use crate::config::ServerConfig;

impl ClusterManager {
    pub async fn discover_peers(&self, config: &ServerConfig, my_port: u16) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();
        
        for instance in &config.instances {
            // Skip if this is our own instance
            if instance.port == my_port {
                continue;
            }

            let node_address = format!("http://{}:{}", instance.address, instance.port);
            
            // Try to connect to each peer
            match self.connect_to_peer(&client, &node_address).await {
                Ok(_) => log::info!("Successfully connected to peer: {}", node_address),
                Err(e) => log::warn!("Failed to connect to peer {}: {}", node_address, e),
            }
        }

        Ok(())
    }

    async fn connect_to_peer(&self, client: &Client, node_address: &str) -> Result<(), Box<dyn std::error::Error>> {
        let health_url = format!("{}/health", node_address);
        
        // First check if the node is healthy
        let response = client
            .get(&health_url)
            .timeout(Duration::from_secs(5))
            .send()
            .await?;

        if response.status().is_success() {
            // If healthy, add to cluster
            // Extract port from node_address
            let port = node_address.split(':').last()
                .unwrap_or("80")
                .parse::<u16>()
                .unwrap_or(80);

            let node_info = NodeInfo {
                id: Uuid::new_v4(), // We'll get the actual ID from the response in a more complete implementation
                address: node_address.to_string(),
                port,
            };

            self.register_node(node_info).await;
            Ok(())
        } else {
            Err("Node health check failed".into())
        }
    }
}

// Make sure you have these structs in your state.rs file:
#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    status: String,
    message: String,
}

#[get("/health")]
async fn health_check() -> rocket::serde::json::Json<ApiResponse> {
    rocket::serde::json::Json(ApiResponse {
        status: "ok".to_string(),
        message: "Service is healthy".to_string(),
    })
}

#[get("/cluster/status")]
async fn cluster_status(
    state: &rocket::State<Arc<RwLock<SharedState>>>,
    cluster: &rocket::State<Arc<ClusterManager>>,
) -> rocket::serde::json::Json<ApiResponse> {
    let state = state.read().await;
    let nodes = cluster.get_nodes().await;
    
    let node_status = futures::future::join_all(nodes.iter()
        .map(|node| async {
            format!("Node {} ({}:{}): {}", 
                node.id, 
                node.address, 
                node.port,
                if cluster.is_node_alive(&node.id).await
                 { "alive" } else { "dead" }
            )
        }))
        .await
        .join("\n");

    rocket::serde::json::Json(ApiResponse {
        status: "ok".to_string(),
        message: format!(
            "Node role: {}\nCluster nodes:\n{}",
            if state.is_leader { "Leader" } else { "Follower" },
            node_status
        ),
    })
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_LOG", "trace");
    env_logger::init();

    let config = ServerConfig::get().expect("Failed to read config file");
    let port = config.port;
    
    let node_id = Uuid::new_v4();
    let shared_state = Arc::new(RwLock::new(SharedState::new(node_id)));
    let cluster_manager = Arc::new(ClusterManager::new(shared_state.clone()));
    
    // Discover peers before starting the leader election
    let cluster_manager_clone = cluster_manager.clone();
    let config_clone = config.clone();
    tokio::spawn(async move {
        loop {
            if let Err(e) = cluster_manager_clone.discover_peers(&config_clone, port).await {
                log::error!("Failed to discover peers: {}", e);
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });

    let leader_election = LeaderElection::new(
        node_id,
        shared_state.clone(),
        cluster_manager.clone(),
    );

    // Start leader election process
    tokio::spawn(async move {
        leader_election.start().await;
    });

    let _rocket = rocket::build()
        .configure(rocket::Config { port, ..Default::default()})
        .manage(shared_state)
        .manage(cluster_manager)
        .mount("/", routes![health_check, cluster_status])
        .launch()
        .await?;

    Ok(())
}