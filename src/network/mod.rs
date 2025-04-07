// network/mod.rs
//
// Main module file for network-related functionality

pub mod discovery;
pub mod client;

// Re-export commonly used types for convenience
pub use discovery::{EnvironmentNode, NodeType};
pub use client::NetworkClient;