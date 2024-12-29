
use rocket::{self, get, post, routes};
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

mod cluster;
mod leader;
mod state;
pub mod config;

use cluster::{ClusterManager, NodeInfo};
use leader::LeaderElection;
use state::SharedState;

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
) -> rocket::serde::json::Json<ApiResponse> {
    let state = state.read().await;
    rocket::serde::json::Json(ApiResponse {
        status: "ok".to_string(),
        message: format!(
            "Node role: {}, Cluster size: {}",
            if state.is_leader { "Leader" } else { "Follower" },
            state.cluster_size
        ),
    })
}

#[post("/register", format = "json", data = "<node_info>")]
async fn register_node(
    state: &rocket::State<Arc<RwLock<SharedState>>>,
    cluster_manager: &rocket::State<Arc<ClusterManager>>,
    node_info: rocket::serde::json::Json<NodeInfo>,
) -> rocket::serde::json::Json<ApiResponse> {
    cluster_manager.register_node(node_info.0).await;
    
    rocket::serde::json::Json(ApiResponse {
        status: "ok".to_string(),
        message: "Node registered successfully".to_string(),
    })
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_LOG", "trace");
    env_logger::init();

    let node_id = Uuid::new_v4();
    let shared_state = Arc::new(RwLock::new(SharedState::new(node_id)));
    let cluster_manager = Arc::new(ClusterManager::new(shared_state.clone()));
    
    let leader_election = LeaderElection::new(
        node_id,
        shared_state.clone(),
        cluster_manager.clone(),
    );

    // Start leader election process
    tokio::spawn(async move {
        leader_election.start().await;
    });
    let port = config::ServerConfig::get().expect("Failed to read config file").port;

    let _rocket = rocket::build()
        .configure(rocket::Config { port, ..Default::default()})
        .manage(shared_state)
        .manage(cluster_manager)
        .mount("/", routes![health_check, cluster_status, register_node])
        .launch()
        .await?;

    Ok(())
}