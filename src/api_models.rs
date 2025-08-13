//! API models for the OmniOrchestrator
//! 
//! These are the general API models used by the OmniOrchestrator
//! service outside the primary platform routes located in
//! /scr/schema/VERSION/api

use serde::{Deserialize, Serialize};
use crate::cluster::NodeInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterStatusMessage {
    pub node_roles: String,
    pub cluster_nodes: Vec<NodeInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub status: String,
    pub message: ClusterStatusMessage,
}