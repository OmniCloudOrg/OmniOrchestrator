use colored::Colorize;
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::state::SharedState;

/// Represents a node in the OmniOrchestrator cluster.
///
/// This structure contains all the necessary information to identify and
/// communicate with a specific node in the distributed system. Each node
/// is uniquely identified by its ID, and contains network location information.
///
/// # Fields
///
/// * `id` - Unique identifier for the node, typically in the format of "address:port"
/// * `port` - The port number that the node is listening on
/// * `address` - The network address of the node for communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Unique identifier for the node in the cluster
    pub id: Arc<str>,
    /// Port number the node is listening on
    pub port: u16,
    /// Network address of the node
    pub address: Arc<str>,
}

/// Central manager for cluster operations and node tracking.
///
/// The ClusterManager is responsible for maintaining the state of the entire 
/// cluster, including tracking which nodes are currently active, managing node
/// registration and removal, and providing information about the cluster's 
/// current composition.
///
/// It uses thread-safe data structures to allow concurrent access from multiple 
/// parts of the application, particularly important in a distributed system where
/// node changes can happen at any time.
///
/// # Fields
///
/// * `state` - Shared state that includes information about the current node and cluster
/// * `nodes` - Thread-safe map of all known nodes in the cluster, indexed by their address
#[derive(Debug)]
pub struct ClusterManager {
    /// Shared state containing information about the current node and overall cluster
    pub state: Arc<RwLock<SharedState>>,
    /// Thread-safe map of all nodes in the cluster, keyed by node address
    pub nodes: Arc<RwLock<HashMap<Arc<str>, NodeInfo>>>,
}

impl ClusterManager {
    /// Creates a new instance of the ClusterManager.
    ///
    /// Initializes a new cluster manager with the provided shared state and
    /// an empty nodes map. This is typically called during application startup
    /// to establish the cluster management subsystem.
    ///
    /// # Arguments
    ///
    /// * `state` - Shared state containing information about the current node
    ///
    /// # Returns
    ///
    /// A new ClusterManager instance ready to track and manage cluster nodes
    pub fn new(state: Arc<RwLock<SharedState>>) -> Self {
        Self {
            state,
            nodes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Registers a new node in the cluster.
    ///
    /// This method adds a node to the cluster's node registry if it doesn't already exist.
    /// After registration, it updates the shared state with the new cluster size.
    /// The method uses colorized output to make debugging in the console easier.
    ///
    /// # Arguments
    ///
    /// * `node` - Information about the node to register
    ///
    /// # Side Effects
    ///
    /// * Updates the internal nodes map if the node is new
    /// * Updates the cluster size in the shared state
    /// * Prints diagnostic information to console
    pub async fn register_node(&self, node: NodeInfo) {
        let node_uid = node.address.clone();
        println!(
            "{}{}",
            "CALLED REGISTER NODE FUNCTION WITH PARAMS OF:"
                .white()
                .on_red()
                .bold(),
            node_uid.green()
        );
        
        // Check if the node already exists in our registry
        if self.nodes.read().await.contains_key(&node_uid) {
            println!("WE ALREADY HAD THIS NODE");
            return;
        }
        
        // Add the node to our registry and get the new size
        let size = {
            let mut nodes = self.nodes.write().await;
            println!(
                "{}{}",
                "ADDING NODE".white().on_red().bold().underline(),
                node_uid
            );
            nodes.insert(node_uid, node);
            let size = nodes.len();
            println!("Current node map: {:?}", nodes);
            size
        };
        
        // Update the cluster size in the shared state
        let mut state = self.state.write().await;
        state.cluster_size = size;
    }

    /// Removes a node from the cluster.
    ///
    /// This method removes a node from the cluster's node registry if it exists.
    /// After removal, it updates the shared state with the new cluster size.
    /// The method includes debug logging to track node removal operations.
    ///
    /// # Arguments
    ///
    /// * `node_uid` - Unique identifier of the node to remove, typically its address
    ///
    /// # Side Effects
    ///
    /// * Updates the internal nodes map by removing the specified node
    /// * Updates the cluster size in the shared state
    /// * Logs diagnostic information about the removal operation
    pub async fn remove_node(&self, node_uid: Arc<str>) {
        debug!(
            "{}{}",
            "CALING REMOVE NODE FUNCTION WITH PARAMS OF:"
                .white()
                .on_green()
                .bold(),
            node_uid.green()
        );
        
        // First check if the node exists in our registry
        {
            let nodes_read = self.nodes.read().await;
            if !nodes_read.contains_key(&node_uid) {
                log::info!("Attempted to remove a node that does not exist");
                log::info!("Current nodes: {:?}", nodes_read);
                return;
            }
        }
        
        // Remove the node from our registry
        let mut nodes = self.nodes.write().await;
        log::info!("Removing node: {}", node_uid.white().on_green().bold());
        nodes.remove(&node_uid);

        // Update the cluster size in the shared state
        let mut state = self.state.write().await;
        state.cluster_size = nodes.len();
    }

    /// Retrieves a list of all known nodes in the cluster.
    ///
    /// This method provides a snapshot of all the nodes currently registered
    /// in the cluster manager. It's useful for operations that need to iterate
    /// over all nodes or display cluster status information.
    ///
    /// # Returns
    ///
    /// A vector containing information about all known nodes in the cluster
    pub async fn get_nodes(&self) -> Vec<NodeInfo> {
        let nodes = self.nodes.read().await;
        nodes.values().cloned().collect()
    }

    /// Retrieves a list of all known nodes plus the current node.
    ///
    /// This method provides a complete view of the cluster including the current node.
    /// It's particularly useful for operations that need a complete picture of the cluster,
    /// such as leader election or quorum calculations.
    ///
    /// # Returns
    ///
    /// A vector containing information about all nodes in the cluster, including the current node
    ///
    /// # Side Effects
    ///
    /// * Prints diagnostic information about the current node and known nodes to console
    pub async fn get_nodes_and_self(&self) -> Vec<NodeInfo> {
        let state = self.state.read().await;
        let nodes = self.nodes.read().await;

        // Collect all known nodes into a vector
        let mut all_nodes: Vec<NodeInfo> = nodes.values().cloned().collect();
        
        // Add the current node to the collection
        all_nodes.push(NodeInfo {
            id: format!("{}", state.node_id).into(),
            address: state.node_id.split(':').next().unwrap_or_default().into(),
            port: state
                .node_id
                .split(':')
                .nth(1)
                .unwrap_or_default()
                .parse()
                .unwrap_or(0),
        });

        // Print diagnostic information
        println!("Current node ID: {}", state.node_id);
        println!("Known nodes: {:?}", nodes);

        all_nodes
    }

    /// Checks if a specific node is currently active in the cluster.
    ///
    /// This method determines if a node is still considered active within the cluster
    /// by checking if it exists in the nodes registry. It's useful for operations that
    /// need to verify a node's presence before attempting to communicate with it.
    ///
    /// # Arguments
    ///
    /// * `node_uid` - Unique identifier of the node to check, typically its address
    ///
    /// # Returns
    ///
    /// `true` if the node is active in the cluster, `false` otherwise
    pub async fn is_node_alive(&self, node_uid: Arc<str>) -> bool {
        let nodes = self.nodes.read().await;
        nodes.contains_key(&node_uid)
    }
}