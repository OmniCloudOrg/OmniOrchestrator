use std::sync::Arc;

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedState {
    pub node_id: Arc<str>,
    pub is_leader: bool,
    pub cluster_size: usize,
    pub leader_id: Option<Arc<str>>,
}

impl SharedState {
    pub fn new(node_id: Arc<str>) -> Self {
        Self {
            node_id,
            is_leader: false,
            cluster_size: 1,
            leader_id: None,
        }
    }
}
