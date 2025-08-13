use crate::server::build_rocket;
// use crate::{CLUSTER_MANAGER}; // removed unused import
use crate::db_manager::DatabaseManager;
use crate::state::SharedState;
// use libomni::types::db::auth::AuthConfig; // removed unused import
use std::sync::Arc;
use tokio::sync::RwLock;
use colored::Colorize;

/// Builds and launches the Rocket server with the provided configuration and dependencies.
///
/// # Arguments
/// * `port` - The port to bind the server to.
/// * `db_manager` - Shared database manager instance.
/// * `pool` - Main database pool.
/// * `cluster_manager` - Shared cluster manager instance.
/// * `clickhouse_client` - ClickHouse client instance.
/// * `shared_state_for_server` - Shared state for the server.
///
/// # Errors
/// Returns an error if the Rocket server fails to launch.
pub async fn launch_server(
    port: u16,
    db_manager: Arc<DatabaseManager>,
    pool: sqlx::MySqlPool,
    cluster_manager: Arc<RwLock<crate::cluster::ClusterManager>>,
    clickhouse_client: clickhouse::Client,
    shared_state_for_server: Arc<RwLock<SharedState>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let auth_config = super::create_auth_config();
    let rocket_with_routes = build_rocket(
        port,
        db_manager,
        pool,
        cluster_manager,
        clickhouse_client,
        shared_state_for_server,
        auth_config,
    );
    log::info!("{}", "🚀 LAUNCHING SERVER...".bright_cyan().bold());
    rocket_with_routes.launch().await?;
    Ok(())
}
