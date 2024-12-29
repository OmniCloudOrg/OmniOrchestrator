use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time;
use uuid::Uuid;

use crate::cluster::ClusterManager;
use crate::state::SharedState;

pub struct LeaderElection {
    node_id: Uuid,
    state: Arc<RwLock<SharedState>>,
    cluster_manager: Arc<ClusterManager>,
}

impl LeaderElection {
    pub fn new(
        node_id: Uuid,
        state: Arc<RwLock<SharedState>>,
        cluster_manager: Arc<ClusterManager>,
    ) -> Self {
        Self {
            node_id,
            state,
            cluster_manager,
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
        let nodes = self.cluster_manager.get_nodes().await;
        let mut state = self.state.write().await;

        // Simple election: node with lowest ID becomes leader
        if let Some(potential_leader) = nodes
            .iter()
            .min_by_key(|node| node.id)
        {
            if potential_leader.id == self.node_id {
                state.is_leader = true;
                state.leader_id = Some(self.node_id);
            } else {
                state.is_leader = false;
                state.leader_id = Some(potential_leader.id);
            }
        } else {
            // No other nodes, become leader
            state.is_leader = true;
            state.leader_id = Some(self.node_id);
        }
    }
}