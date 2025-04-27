use sqlx::types::{chrono::NaiveDateTime, JsonValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use sqlx::types::Json;
use serde_json::Value; 
use sqlx::Row;

#[derive(Debug, sqlx::FromRow, Serialize, Clone, Deserialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub email_verified: i8,
    pub password: String,
    pub salt: String,
    pub login_attempts: i64,
    pub active: bool,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for User {
    type Error = ();

    async fn from_request(_request: &'r rocket::Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        // Placeholder implementation
        rocket::request::Outcome::Success(User {
            id: 0,
            email: String::new(),
            email_verified: 0,
            password: String::new(),
            salt: String::new(),
            login_attempts: 0,
            active: false,
            status: String::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login_at: None,
        })
    }
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct App {
    pub id: i64,
    pub name: String,
    pub org_id: i64,
    pub git_repo: Option<String>,
    pub region_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub git_branch: Option<String>,
    pub maintenance_mode: bool,
    pub container_image_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AppWithInstanceCount {
    #[serde(flatten)]
    app_data: App,
    instance_count: i64,
}


// Define the struct with flattening
#[derive(Debug, Serialize)]
pub struct AppWithInstances {
    #[serde(flatten)]
    pub app: App,
    pub instances: Vec<Instance>,
}


#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Org {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Region {
    pub id: i64,
    pub name: String,
    pub provider: i64, // enum in DB: 'kubernetes' or 'custom'
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Provider {
    pub id: i64,
    pub name: String,
    pub display_name: String,
    pub provider_type: String, // enum in DB: 'kubernetes' or 'custom'
    pub status: String, // enum in DB: 'active', 'inactive', 'maintenance'
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct ProviderAuditLog {
    pub id: i64,
    pub provider_id: i64,
    pub action: String,
    pub details: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// List provider-regions.
///
/// This function fetches all regions from the database, paired with their providers and their binding table data.
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct ProviderRegion {
    #[sqlx(flatten)]
    region: Region,
    provider_name: String,
    binding_status: String,
}

impl<'r> sqlx::FromRow<'r, sqlx::mysql::MySqlRow> for AppWithInstanceCount {
    fn from_row(row: &'r sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(AppWithInstanceCount {
            app_data: App::from_row(row)?,
            instance_count: row.try_get::<i64, _>("instance_count")?,
        })
    }
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Permission {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
    pub resource_type: Option<String>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Build {
    pub id: i64,
    pub app_id: i64,
    pub source_version: Option<String>,
    pub commit_sha: Option<String>,
    pub commit_message: Option<String>,
    pub author: Option<String>,
    pub status: String, // enum: 'pending', 'building', 'succeeded', 'failed', 'canceled'
    pub build_pack_used: Option<String>,
    pub build_pack_url: Option<String>,
    pub build_pack_version: Option<String>,
    pub build_image: Option<String>,
    pub build_arguments: Option<serde_json::Value>,
    pub build_environment: Option<serde_json::Value>,
    pub build_cache_key: Option<String>,
    pub log_url: Option<String>,
    pub artifact_url: Option<String>,
    pub artifact_checksum: Option<String>,
    pub artifact_size: Option<i64>,
    pub error_message: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub build_duration: Option<i32>, // in seconds
    pub created_at: DateTime<Utc>,
}


#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Metric {
    pub id: i64,
    pub app_id: Option<i64>,
    pub metric_name: String,
    pub metric_value: f64,
    pub labels: Option<JsonValue>,
    pub timestamp: Option<NaiveDateTime>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Deployment {
    pub id: i64,
    pub status: String, // enum: 'pending', 'in_progress', 'deployed', 'failed'
    pub app_id: i64,
    pub build_id: i64,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Instance {
    pub id: i64,
    pub app_id: i64,
    pub instance_type: String,
    pub guid: String,
    pub status: String, // enum: 'running', 'starting', 'stopping', 'stopped', 'crashed', 'terminated', 'unknown'
    pub region_id: i64,
    pub container_id: Option<String>,
    pub container_ip: Option<String>,
    pub allocation_id: Option<i64>,
    pub node_id: Option<i64>,
    pub instance_index: i32,
    pub last_health_check: Option<DateTime<Utc>>,
    pub health_status: String, // enum: 'healthy', 'unhealthy', 'unknown'
    pub cpu_usage: Option<f64>,
    pub memory_usage: Option<f64>,
    pub disk_usage: Option<f64>,
    pub uptime: Option<i32>,
    pub restart_count: Option<i32>,
    pub last_restart_reason: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub stop_time: Option<DateTime<Utc>>,
    pub exit_code: Option<i32>,
    pub exit_reason: Option<String>,
    pub scheduler_metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct StorageClass {
    pub id: i64,
    pub name: String,
    pub provisioner: String,
    pub reclaim_policy: String, // enum: 'Delete', 'Retain' TODO: @tristanpoland add recycle
    pub volume_binding_mode: String, // enum: 'Immediate', 'WaitForFirstConsumer'
    pub allow_volume_expansion: bool,
    pub storage_type: String, // 'local-disk', 'local-resilient', 'distributed', 'geo-replicated'
    pub default_filesystem: String, // enum: 'ext4', 'xfs', 'btrfs', 'zfs'
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct StorageVolume {
    pub id: i64,
    pub app_id: i64,
    pub name: String,
    pub size_gb: i64,
    pub storage_class: String,
    pub access_mode: String, // enum: 'ReadWriteOnce', 'ReadOnlyMany', 'ReadWriteMany'
    pub status: String, // enum: 'Provisioned', 'Bound', 'Mounted', 'Released', 'Deleting', 'Deleted'
    pub node_id: i64,
    pub encryption_enabled: bool,
    pub persistence_level: String, // enum: 'Basic', 'Enhanced', 'High', 'Maximum'
    pub write_concern: String, // enum: 'WriteAcknowledged', 'WriteDurable', 'WriteReplicated', 'WriteDistributed'
    pub reclaim_policy: String, // enum: 'Delete', 'Retain'
    pub filesystem_type: Option<String>,
    pub storage_class_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub snapshot_id: Option<i64>,
    pub mount_path: Option<String>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct StorageSnapshot {
    pub id: i64,
    pub volume_id: i64,
    pub name: String,
    pub size_gb: i64,
    pub created_at: DateTime<Utc>,
    pub status: String, // enum: 'Creating', 'Available', 'Deleting', 'Deleted'
    pub description: Option<String>,
    pub retention_date: Option<DateTime<Utc>>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct StorageMigration {
    pub id: i64,
    pub source_volume_id: i64,
    pub destination_volume_id: i64,
    pub migration_type: String, // enum: 'StorageClass', 'Node', 'Zone', 'Environment'
    pub status: String, // enum: 'Pending', 'Copying', 'Syncing', 'ReadyForCutover', 'Completed', 'Failed'
    pub progress_percent: i32,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub is_online: bool,
    pub error_message: Option<String>,
    pub created_by: String,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct StorageQosPolicy {
    pub id: i64,
    pub name: String,
    pub max_iops: Option<i32>,
    pub max_throughput_mbps: Option<i32>,
    pub burst_iops: Option<i32>,
    pub burst_duration_seconds: Option<i32>,
    pub latency_target_ms: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct VolumeQosPolicyAssignment {
    pub volume_id: i64,
    pub policy_id: i64,
    pub assigned_at: DateTime<Utc>
}

// User Notifications
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct UserNotification {
    pub id: i64,
    pub user_id: i64,
    pub org_id: Option<i64>,
    pub app_id: Option<i64>,
    pub notification_type: String,
    pub message: String,
    pub read_status: bool,
    pub importance: String,
    pub action_url: Option<String>,
    pub action_label: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

// Role Notifications
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct RoleNotification {
    pub id: i64,
    pub role_id: i64,
    pub org_id: Option<i64>,
    pub app_id: Option<i64>,
    pub notification_type: String,
    pub message: String,
    pub importance: String,
    pub action_url: Option<String>,
    pub action_label: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

// Notification Acknowledgments
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct NotificationAcknowledgment {
    pub id: i64,
    pub user_id: i64,
    pub notification_id: Option<i64>,
    pub role_notification_id: Option<i64>,
    pub acknowledged_at: DateTime<Utc>
}

// System Alerts
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Alert {
    pub id: i64,
    pub alert_type: String,
    pub severity: String,
    pub service: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub status: String,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<i64>,
    pub metadata: Option<Json<HashMap<String, serde_json::Value>>>,
    pub org_id: Option<i64>,
    pub app_id: Option<i64>,
    pub instance_id: Option<i64>,
    pub region_id: Option<i64>,
    pub node_id: Option<i64>,
}

// Alert Acknowledgments
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct AlertAcknowledgment {
    pub id: i64,
    pub alert_id: i64,
    pub user_id: i64,
    pub acknowledged_at: DateTime<Utc>,
    pub notes: Option<String>,
}

// Alert Escalations
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct AlertEscalation {
    pub id: i64,
    pub alert_id: i64,
    pub escalation_level: i64,
    pub escalated_at: DateTime<Utc>,
    pub escalated_to: Json<serde_json::Value>,
    pub escalation_method: String,
    pub response_required_by: Option<DateTime<Utc>>,
}

/// Represents an alert with all its related data (acknowledgments, escalations, and history).
/// This comprehensive view is useful for detailed alert pages.
#[derive(Debug, Serialize, Deserialize)]
pub struct AlertWithRelatedData {
    /// The core alert data
    pub alert: Alert,
    /// List of all acknowledgments for this alert
    pub acknowledgments: Vec<AlertAcknowledgment>,
    /// List of all escalations for this alert
    pub escalations: Vec<AlertEscalation>,
    /// History of all actions taken on this alert
    pub history: Vec<AlertHistory>
}

/// Represents an alert with its acknowledgment information.
/// This is useful for displaying alerts with their acknowledgment status.
#[derive(Debug, Serialize, Deserialize)]
pub struct AlertWithAcknowledgments {
    /// The core alert data
    pub alert: Alert,
    /// List of acknowledgments for this alert
    pub acknowledgments: Vec<AlertAcknowledgment>,
    /// Whether the alert has been acknowledged
    pub is_acknowledged: bool,
    /// Total number of acknowledgments
    pub acknowledgment_count: i64,
    /// Timestamp of the most recent acknowledgment, if any
    pub latest_acknowledgment: Option<chrono::DateTime<chrono::Utc>>,
}

// Alert History
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct AlertHistory {
    pub id: i64,
    pub alert_id: i64,
    pub action: String,
    pub performed_by: Option<i64>,
    pub performed_at: DateTime<Utc>,
    pub previous_state: Option<Json<serde_json::Value>>,
    pub new_state: Option<Json<serde_json::Value>>,
    pub notes: Option<String>,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: i64,
    pub org_id: Option<i64>,
    pub action: String,
    pub user_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub resource_id: Option<String>,
    pub resource_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ENUM", rename_all = "snake_case")]
pub enum WorkerStatus {
    Active,
    Provisioning,
    Maintenance,
    PoweredOff,
    Unreachable,
    Degraded,
    Decommissioning,
}

// Default implementation for WorkerStatus
impl Default for WorkerStatus {
    fn default() -> Self {
        WorkerStatus::Active
    }
}

// Function to provide default status for serde
fn default_status() -> WorkerStatus {
    WorkerStatus::Active
}

// Function to provide default SSH port
fn default_ssh_port() -> i32 {
    22
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Worker {
    pub id: Option<i64>,
    pub region_id: i64,
    pub name: String,
    pub provider_id: Option<String>,
    pub instance_type: Option<String>,
    pub status: String,
    pub cpu_total: f64,
    pub cpu_available: f64,
    #[serde(default)]
    pub cpu_reserved: f64,
    pub memory_total: f64,     // in MB
    pub memory_available: f64, // in MB
    #[serde(default)]
    pub memory_reserved: f64,  // in MB
    pub disk_total: f64,       // in MB
    pub disk_available: f64,   // in MB
    #[serde(default)]
    pub disk_reserved: f64,    // in MB
    pub network_in_capacity: Option<f64>,  // in Mbps
    pub network_out_capacity: Option<f64>, // in Mbps
    pub docker_version: Option<String>,
    pub ssh_address: Option<String>,
    #[serde(default = "default_ssh_port")]
    pub ssh_port: i32,
    pub ssh_user: Option<String>,
    pub ssh_key: Option<String>,
    pub labels: Option<Json<serde_json::Value>>,
    pub taints: Option<Json<serde_json::Value>>,
    pub annotations: Option<Json<serde_json::Value>>,
    pub last_heartbeat: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Notification {
    pub id: i64,
    pub user_id: Option<i64>,
    pub org_id: Option<i64>,
    pub app_id: Option<i64>,
    pub notification_type: String, // enum: 'info', 'warning', 'error', 'success'
    pub message: String,
    pub read_status: bool,
    pub created_at: DateTime<Utc>,
}

/// Represents a comprehensive view of a user's notifications with unread counts.
/// This is useful for providing notification center overviews.
#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationWithCount {
    /// Direct notifications for the user
    pub user_notifications: Vec<UserNotification>,
    /// Role-based notifications applicable to the user
    pub role_notifications: Vec<RoleNotification>,
    /// User's acknowledgments of role notifications
    pub acknowledgments: Vec<NotificationAcknowledgment>,
    /// Count of unread direct user notifications
    pub unread_user_count: i64,
    /// Count of unacknowledged role notifications
    pub unacknowledged_role_count: i64,
    /// Total count of unread notifications (user + role)
    pub total_unread_count: i64
}

/// Represents a user's notifications including those from their roles.
/// This combines personal notifications with role-based ones.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserNotificationWithRoleNotifications {
    /// Direct notifications for the user
    pub user_notifications: Vec<UserNotification>,
    /// Role-based notifications applicable to the user
    pub role_notifications: Vec<RoleNotification>,
    /// User's acknowledgments of role notifications
    pub acknowledgments: Vec<NotificationAcknowledgment>
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Backup {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub backup_type: String, // enum: 'PLATFORM', 'APPLICATION', 'PARTIAL'
    pub status: String, // enum: 'CREATING', 'AVAILABLE', 'RESTORING', 'FAILED', 'DELETED'
    pub format_version: String,
    pub source_environment: String,
    pub encryption_method: Option<String>,
    pub encryption_key_id: Option<i64>,
    pub size_bytes: Option<i64>,
    pub has_system_core: bool,
    pub has_directors: bool,
    pub has_orchestrators: bool,
    pub has_network_config: bool,
    pub has_app_definitions: bool,
    pub has_volume_data: bool,
    pub included_apps: Option<String>,
    pub included_services: Option<String>,
    pub last_validated_at: Option<DateTime<Utc>>,
    pub last_restored_at: Option<DateTime<Utc>>,
    pub restore_target_environment: Option<String>,
    pub restore_status: Option<String>,
    pub storage_location: String,
    pub manifest_path: String,
    pub metadata: Option<Value>,
}