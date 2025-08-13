use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Request and response structs

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAlertRequest {
    pub alert_type: String,
    pub severity: String,
    pub service: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAlertStatusRequest {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcknowledgeAlertRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub update_status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEscalationRequest {
    pub escalation_level: i64,
    pub escalated_to: serde_json::Value,
    pub escalation_method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_required_by: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkUpdateStatusRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<i64>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}