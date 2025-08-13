//! Core API endpoints for the OmniOrchestrator server.
//!
//! This module provides essential HTTP endpoints for monitoring and managing
//! the OmniOrchestrator cluster. These endpoints serve as the primary interface
//! for external systems to query the health and status of the cluster.

use rocket;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::cluster::ClusterManager;
use crate::state::SharedState;
use crate::api_models::{ApiResponse, ClusterStatusMessage};

/// Health check endpoint that provides basic service availability status.
///
/// This endpoint is used by load balancers, monitoring systems, and other external
/// services to determine if the OmniOrchestrator service is running and responding
/// to requests. It returns a simple JSON response indicating service availability.
///
/// # Returns
///
/// A JSON response with status "ok" and basic cluster information.
#[get("/health")]
pub async fn health_check() -> rocket::serde::json::Json<ApiResponse> {
    log::debug!("Health check endpoint called");
    rocket::serde::json::Json(ApiResponse {
        status: "ok".to_string(),
        message: ClusterStatusMessage {
            node_roles: "unknown".to_string(),
            cluster_nodes: vec![],
        },
    })
}

/// Provides detailed cluster status information including node roles and membership.
///
/// This endpoint returns comprehensive information about the current state of the
/// OmniOrchestrator cluster, including which node is the leader, cluster membership,
/// and the role of the current node. This information is crucial for cluster
/// monitoring and debugging distributed system issues.
///
/// # Arguments
///
/// * `state` - Shared state containing node role and cluster information
/// * `cluster` - Cluster manager with information about all known nodes
///
/// # Returns
///
/// A JSON response containing:
/// - Overall cluster status
/// - Current node's role (leader/follower)
/// - List of all known cluster nodes
#[get("/cluster/status")]
pub async fn cluster_status(
    state: &rocket::State<Arc<RwLock<SharedState>>>,
    cluster: &rocket::State<Arc<RwLock<ClusterManager>>>,
) -> rocket::serde::json::Json<ApiResponse> {
    log::debug!("Cluster status endpoint called");
    let state = state.read().await;
    let nodes = cluster.read().await;

    let role = if state.is_leader {
        "leader".to_string()
    } else {
        "follower".to_string()
    };

    log::info!("{}", format!("Current node role: {}", role));

    let response = ApiResponse {
        status: "ok".to_string(),
        message: ClusterStatusMessage {
            node_roles: role,
            cluster_nodes: nodes.get_nodes().await,
        },
    };

    rocket::serde::json::Json(response)
}