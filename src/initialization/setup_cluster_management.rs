use std::sync::Arc;
use colored::Colorize;
use crate::{SERVER_CONFIG, RwLock, SharedState};

pub fn setup_cluster_management() -> (Arc<RwLock<SharedState>>, Arc<str>) {
    // Initialize node state and cluster management
    let node_id: Arc<str> =
        format!("{}:{}", SERVER_CONFIG.address.clone(), SERVER_CONFIG.port).into();
    log::info!("{}", format!("Node ID: {}", node_id).magenta());

    let state = Arc::new(RwLock::new(SharedState::new(node_id.clone())));
    (state, node_id)
}
