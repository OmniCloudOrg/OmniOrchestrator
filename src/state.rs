use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedState {
    pub node_id: Uuid,
    pub is_leader: bool,
    pub cluster_size: usize,
    pub leader_id: Option<Uuid>,
}

impl SharedState {
    pub fn new(node_id: Uuid) -> Self {
        Self {
            node_id,
            is_leader: false,
            cluster_size: 1,
            leader_id: None,
        }
    }
}
