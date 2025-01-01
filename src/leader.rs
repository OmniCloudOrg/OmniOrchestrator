use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time;

use crate::cluster::NodeInfo;
use crate::state::SharedState;
use crate::CLUSTER_MANAGER;

pub struct LeaderElection {
    node_id: Arc<str>,
    state: Arc<RwLock<SharedState>>,
    last_heartbeat: Arc<RwLock<std::time::Instant>>,
}

impl LeaderElection {
    pub fn new(node_id: Arc<str>, state: Arc<RwLock<SharedState>>) -> Self {
        Self {
            node_id,
            state,
            last_heartbeat: Arc::new(RwLock::new(std::time::Instant::now())),
        }
    }

    pub async fn start(&self) {
        let mut interval = time::interval(Duration::from_secs(5));

        loop {
            interval.tick().await;
            self.election_cycle().await;
        }
    }

    async fn election_cycle(&self) {
        let cluster_manager = CLUSTER_MANAGER.read().await;
        let nodes = cluster_manager.get_nodes_and_self().await;
        log::info!("Nodes participating in election:");
        for node in &nodes {
            log::info!("  - {}", node.id);
        }
        
        let mut state = self.state.write().await;
    
        // Sort nodes by full address string for consistent leader selection
        let mut sorted_nodes = nodes.clone();
        sorted_nodes.sort_by(|a, b| a.id.cmp(&b.id));
        log::info!("Sorted nodes: {:?}", sorted_nodes);
    
        if sorted_nodes.is_empty() {
            state.is_leader = true;
            state.leader_id = Some(self.node_id.clone());
            log::info!("Single node {} becoming leader", self.node_id);
            return;
        }
    
        // First node in sorted list becomes leader
        let leader = &sorted_nodes[0];
        let is_self_leader = leader.id == self.node_id;
        log::info!("Leader logic: {} == {}", leader.id, self.node_id);
    
        state.is_leader = is_self_leader;
        state.leader_id = Some(leader.id.clone());
    
        log::info!("Leader elected: {})", 
            leader.id);
        log::info!("This node ({}) is {}", 
            self.node_id,
            if is_self_leader { "leader" } else { "follower" });
    }
}