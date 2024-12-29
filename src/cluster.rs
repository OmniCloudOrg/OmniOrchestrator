use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::state::SharedState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub id: Uuid,
    pub address: String,
    pub port: u16,
}

pub struct ClusterManager {
    state: Arc<RwLock<SharedState>>,
    nodes: Arc<RwLock<HashMap<Uuid, NodeInfo>>>,
}

impl ClusterManager {
    pub fn new(state: Arc<RwLock<SharedState>>) -> Self {
        Self {
            state,
            nodes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_node(&self, node: NodeInfo) {
        let mut nodes = self.nodes.write().await;
        nodes.insert(node.id, node);
        
        let mut state = self.state.write().await;
        state.cluster_size = nodes.len();
    }

    pub async fn remove_node(&self, node_id: Uuid) {
        let mut nodes = self.nodes.write().await;
        nodes.remove(&node_id);
        
        let mut state = self.state.write().await;
        state.cluster_size = nodes.len();
    }

    pub async fn get_nodes(&self) -> Vec<NodeInfo> {
        let nodes = self.nodes.read().await;
        nodes.values().cloned().collect()
    }
    pub async fn is_node_alive(&self, node_id: &Uuid) -> bool {
        let nodes = self.nodes.read().await;
        nodes.contains_key(node_id)
    }
}