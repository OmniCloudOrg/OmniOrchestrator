use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time;
use uuid::Uuid;

use crate::cluster::ClusterManager;
use crate::state::SharedState;
use crate::CLUSTER_MANAGER;

pub struct LeaderElection {
    node_id: Arc<str>,
    state: Arc<RwLock<SharedState>>,
}

impl LeaderElection {
    pub fn new(
        node_id: Arc<str>,
        state: Arc<RwLock<SharedState>>,
    ) -> Self {
        Self {
            node_id,
            state,
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
        let read = CLUSTER_MANAGER.read().await;
        let nodes = read.nodes.read().await;
        let mut state = self.state.write().await;

        // Election: first node in the list becomes leader
        if let Some(potential_leader) = nodes.values().next() {
            if potential_leader.id == self.node_id.clone() {
            state.is_leader = true;
            state.leader_id = Some(self.node_id.clone());
            } else {
            state.is_leader = false;
            state.leader_id = Some(potential_leader.id.clone());
            }
        } else {
            // No other nodes, become leader
            state.is_leader = true;
            state.leader_id = Some(self.node_id.clone());
        }
    }
}