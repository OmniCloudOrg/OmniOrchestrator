use std::sync::Arc;

use serde::{Deserialize, Serialize};

/// Represents the shared state of a node in the OmniOrchestrator cluster.
///
/// This structure contains essential information about the current node's role and
/// position within the distributed cluster. It's designed to be thread-safe and
/// is typically wrapped in synchronization primitives for concurrent access.
///
/// # Fields
///
/// * `node_id` - Unique identifier for this node in the cluster
/// * `is_leader` - Whether this node is currently the cluster leader
/// * `cluster_size` - Total number of nodes currently in the cluster
/// * `leader_id` - Identifier of the current cluster leader, if known
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedState {
    /// Unique identifier for this node in the cluster
    pub node_id: Arc<str>,
    /// Whether this node is currently the cluster leader
    pub is_leader: bool,
    /// Total number of nodes currently in the cluster
    pub cluster_size: usize,
    /// Identifier of the current cluster leader, if known
    pub leader_id: Option<Arc<str>>,
}

impl SharedState {
    /// Creates a new SharedState instance for a given node.
    ///
    /// Initializes the state with default values:
    /// - Node is not a leader initially
    /// - Cluster size starts at 1 (just this node)
    /// - No leader is known initially
    ///
    /// # Arguments
    ///
    /// * `node_id` - Unique identifier for this node
    ///
    /// # Returns
    ///
    /// A new SharedState instance with default values
    pub fn new(node_id: Arc<str>) -> Self {
        Self {
            node_id,
            is_leader: false,
            cluster_size: 1,
            leader_id: None,
        }
    }
}
