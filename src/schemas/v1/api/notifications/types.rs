use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserNotificationRequest {
    pub message: String,
    pub notification_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub importance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoleNotificationRequest {
    pub role_id: i64,
    pub message: String,
    pub notification_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub importance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcknowledgeNotificationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_notification_id: Option<i64>,
}