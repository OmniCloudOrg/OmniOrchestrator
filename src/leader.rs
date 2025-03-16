use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time;

use crate::state::SharedState;
use crate::CLUSTER_MANAGER;

/// Manages leader election in the OmniOrchestrator cluster.
///
/// The LeaderElection module is responsible for determining which node in the cluster
/// should act as the leader. It implements a simple deterministic leader election
/// algorithm based on node IDs to ensure that exactly one node assumes the leader role.
///
/// Leader election is a critical component in distributed systems that ensures:
/// - Coordination responsibilities are clearly assigned
/// - A single point of truth exists for cluster-wide decisions
/// - System stability is maintained through consistent leadership
///
/// The election process runs periodically to accommodate cluster changes such as
/// nodes joining or leaving the system.
pub struct LeaderElection {
    /// Unique identifier for the current node
    node_id: Arc<str>,
    
    /// Shared state that tracks leadership status and cluster information
    state: Arc<RwLock<SharedState>>,
    
    /// Timestamp of the last heartbeat received
    /// This can be used for more sophisticated leader election algorithms
    /// that take into account node responsiveness
    #[allow(unused)]
    last_heartbeat: Arc<RwLock<std::time::Instant>>,
}

impl LeaderElection {
    /// Creates a new LeaderElection instance.
    ///
    /// Initializes the leader election module with the current node's identity
    /// and a reference to the shared state. The last_heartbeat is initialized to
    /// the current time.
    ///
    /// # Arguments
    ///
    /// * `node_id` - Unique identifier for the current node
    /// * `state` - Shared state for tracking leadership status
    ///
    /// # Returns
    ///
    /// A new LeaderElection instance ready to begin the election process
    pub fn new(node_id: Arc<str>, state: Arc<RwLock<SharedState>>) -> Self {
        Self {
            node_id,
            state,
            last_heartbeat: Arc::new(RwLock::new(std::time::Instant::now())),
        }
    }

    /// Starts the leader election process.
    ///
    /// This method begins a continuous cycle of leader elections at a fixed interval.
    /// Once started, it will periodically execute the election_cycle method to
    /// determine the current leader based on the existing cluster composition.
    ///
    /// The election happens every 5 seconds, which provides a balance between
    /// responsiveness to cluster changes and system overhead.
    ///
    /// # Note
    ///
    /// This method runs indefinitely in a loop and should typically be
    /// spawned in its own task or thread.
    pub async fn start(&self) {
        // Create a timer that ticks every 5 seconds
        let mut interval = time::interval(Duration::from_secs(5));

        // Run the election cycle on each tick
        loop {
            interval.tick().await;
            self.election_cycle().await;
        }
    }

    /// Performs a single leader election cycle.
    ///
    /// This method implements the core leader election algorithm, which follows
    /// these steps:
    /// 1. Retrieve all nodes in the cluster
    /// 2. Sort the nodes by ID for deterministic selection
    /// 3. Select the first node in the sorted list as the leader
    /// 4. Update the shared state with the election results
    ///
    /// The algorithm is intentionally simple and deterministic, ensuring that all
    /// nodes will independently arrive at the same conclusion about who the leader is,
    /// without requiring additional communication.
    ///
    /// # Special Cases
    ///
    /// - If the cluster contains only one node, that node becomes the leader.
    /// - If the cluster contains no nodes (which shouldn't happen as the current node
    ///   should always be included), the current node becomes the leader by default.
    ///
    /// # Side Effects
    ///
    /// - Updates the shared state to reflect the new leader
    /// - Logs information about the election process and results
    async fn election_cycle(&self) {
        // Get reference to cluster manager and retrieve all nodes
        let cluster_manager = CLUSTER_MANAGER.read().await;
        let nodes = cluster_manager.get_nodes_and_self().await;
        
        // Log participating nodes for debugging
        log::info!("Nodes participating in election:");
        for node in &nodes {
            log::info!("  - {}", node.id);
        }

        // Acquire write lock on shared state to update leadership information
        let mut state = self.state.write().await;

        // Sort nodes by ID for deterministic leader selection
        // This ensures all nodes will independently choose the same leader
        let mut sorted_nodes = nodes.clone();
        sorted_nodes.sort_by(|a, b| a.id.cmp(&b.id));
        log::info!("Sorted nodes: {:?}", sorted_nodes);

        // Handle the case where this is the only node (or no nodes, which shouldn't happen)
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

        // Update state with leader information
        state.is_leader = is_self_leader;
        state.leader_id = Some(leader.id.clone());

        // Log election results
        log::info!("Leader elected: {})", leader.id);
        log::info!(
            "This node ({}) is {}",
            self.node_id,
            if is_self_leader { "leader" } else { "follower" }
        );
    }
}