use colored::Colorize;
use crate::LeaderElection;
use std::sync::Arc;
use crate::RwLock;
use crate::SharedState;

pub fn start_leader_election(shared_state: Arc<RwLock<SharedState>>, node_id: Arc<str>) {
    // Initialize and start leader election
    log::info!("{}", "Initializing leader election process".green());
    let _leader_election = LeaderElection::new(node_id, shared_state.clone());
    log::info!("{}", "✓ Leader election initialized".green());
}
