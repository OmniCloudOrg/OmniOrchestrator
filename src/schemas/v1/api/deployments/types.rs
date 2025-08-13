use serde::{Deserialize, Serialize};

/// Request body for creating a deployment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDeploymentRequest {
    pub app_id: i64,
    pub build_id: i64,
    pub version: String,
    pub deployment_strategy: String,
    pub previous_deployment_id: Option<i64>,
    pub canary_percentage: Option<i64>,
    pub environment_variables: Option<serde_json::Value>,
    pub annotations: Option<serde_json::Value>,
    pub labels: Option<serde_json::Value>,
}

/// Request body for updating a deployment's status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDeploymentStatusRequest {
    pub status: String,
    pub error_message: Option<String>,
}