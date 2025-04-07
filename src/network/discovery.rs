// network/discovery.rs
//
// Discovers OmniCloud environment nodes

use std::fmt;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Types of nodes in an OmniCloud environment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    /// Master control node that manages system-wide operations
    Master,
    
    /// Director node that manages virtualization and resources
    Director,
    
    /// Orchestrator node that handles application scheduling
    Orchestrator,
    
    /// Network controller node that manages connectivity
    NetworkController,
    
    /// Application catalog node that stores application definitions
    ApplicationCatalog,
    
    /// Storage node that provides persistent storage for volumes
    Storage,
    
    /// Compute node that runs application workloads
    Compute,
    
    /// Edge node that handles edge computing workloads
    Edge,
    
    /// Gateway node that handles external connectivity
    Gateway,
    
    /// Unknown node type
    Unknown,
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeType::Master => write!(f, "Master"),
            NodeType::Director => write!(f, "Director"),
            NodeType::Orchestrator => write!(f, "Orchestrator"),
            NodeType::NetworkController => write!(f, "NetworkController"),
            NodeType::ApplicationCatalog => write!(f, "ApplicationCatalog"),
            NodeType::Storage => write!(f, "Storage"),
            NodeType::Compute => write!(f, "Compute"),
            NodeType::Edge => write!(f, "Edge"),
            NodeType::Gateway => write!(f, "Gateway"),
            NodeType::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Represents a node in an OmniCloud environment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentNode {
    /// Unique identifier for the node
    pub id: String,
    
    /// Human-readable name of the node
    pub name: String,
    
    /// Type of node
    pub node_type: NodeType,
    
    /// IP address of the node
    pub ip_address: String,
    
    /// Hostname of the node
    pub hostname: String,
    
    /// Status of the node (e.g., "online", "offline", "maintenance")
    pub status: String,
    
    /// Additional metadata about the node
    pub metadata: Option<serde_json::Value>,
}

impl EnvironmentNode {
    /// Create a new EnvironmentNode with default values
    pub fn new(node_type: NodeType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: format!("{}-{}", node_type, Uuid::new_v4().to_string().split('-').next().unwrap_or("node")),
            node_type,
            ip_address: "127.0.0.1".to_string(),
            hostname: "localhost".to_string(),
            status: "online".to_string(),
            metadata: None,
        }
    }
    
    /// Create a new EnvironmentNode with specified values
    pub fn with_details(
        name: impl Into<String>,
        node_type: NodeType,
        ip_address: impl Into<String>,
        hostname: impl Into<String>,
        status: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            node_type,
            ip_address: ip_address.into(),
            hostname: hostname.into(),
            status: status.into(),
            metadata: None,
        }
    }
    
    /// Check if the node is online
    pub fn is_online(&self) -> bool {
        self.status == "online"
    }
    
    /// Convert the node into a JSON value
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
}