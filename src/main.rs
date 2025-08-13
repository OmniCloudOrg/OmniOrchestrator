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
mod cors;
mod state;
mod server;
mod leader;
mod config;
mod cluster;
mod network;
mod schemas;
mod logging;
mod endpoints;
mod db_manager;
mod api_models;
mod initialization;

// +-------------+
// | IMPORTS     |
// +-------------+
// Third-party dependencies
use std::sync::Arc;
use anyhow::Result;
use colored::Colorize;
use tokio::sync::RwLock;
use lazy_static::lazy_static;

// Internal imports
// use crate::server::build_rocket; // removed, now used in initialization::launch_server

// Convenience re-exports
pub use crate::state::SharedState;
pub use crate::config::SERVER_CONFIG;
pub use crate::leader::LeaderElection;
pub use crate::cluster::ClusterManager;
pub use crate::db_manager::DatabaseManager;

// We ignore this import as it always says
// unused even when that is not the case
#[allow(unused_imports)]
#[macro_use]
extern crate rocket;

pub static PROJECT_ROOT: &str = env!("CARGO_MANIFEST_DIR");

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
// | MAIN        |
// +-------------+
/// Main entry point for the OmniOrchestrator server
#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup initial convenience variables
    let port = SERVER_CONFIG.port;

    // ====================== INITIALIZATION ======================
    logging::print_banner("OMNI ORCHESTRATOR SERVER STARTING", |s| s.bright_cyan());
    println!("{}", format!("⇒ Starting server on port {}", port).green());

    // ====================== Setup logging ======================
    logging::print_banner("LOGGING SETUP", |s| s.bright_yellow());
    initialization::setup_logging().await;

    // ====================== Setup database ======================
    logging::print_banner("DATABASE SETUP", |s| s.bright_yellow());
    let db_manager = initialization::setup_database().await?;
    let pool = db_manager.get_main_pool();

    // ====================== Setup ClickHouse ======================
    logging::print_banner("CLICKHOUSE SETUP", |s| s.bright_yellow());
    let clickhouse_client = initialization::setup_clickhouse().await?;

    // ====================== Setup Schema ======================
    logging::print_banner("SCHEMA SETUP", |s| s.bright_yellow());
    initialization::setup_schema(&clickhouse_client, &pool).await?;

    // ====================== CLUSTER SETUP ======================
    logging::print_banner("CLUSTER MANAGEMENT", |s| s.bright_magenta());
    let (shared_state, node_id) = initialization::setup_cluster_management();

    // Clone shared_state for later use
    let shared_state_for_leader = shared_state.clone();
    let shared_state_for_server = shared_state.clone();

    // ====================== Start Peer Discovery ======================

    logging::print_banner("START PEER DISCOVERY", |s| s.bright_magenta());
    initialization::start_peer_discovery(port);

    // ====================== AUTOSCALER SETUP ======================
    logging::print_banner("AUTOSCALER SETUP", |s| s.bright_yellow());

    // TODO: Implement autoscaler setup via Lighthouse integration
    // This will require establishing communication with the Lighthouse service
    // for automatic scaling decisions based on cluster metrics and load patterns.
    // Reference: https://github.com/OmniCloudOrg/Lighthouse

    // ====================== LEADER ELECTION ======================
    logging::print_banner("LEADER ELECTION", |s| s.bright_green());

    initialization::start_leader_election(shared_state_for_leader, node_id);

    // ====================== SERVER STARTUP ======================
    logging::print_banner("SERVER STARTUP", |s| s.bright_cyan());

    initialization::launch_server(
        port,
        db_manager.clone(),
        pool.clone(),
        CLUSTER_MANAGER.clone(),
        clickhouse_client,
        shared_state_for_server,
    ).await?;

    Ok(())
}