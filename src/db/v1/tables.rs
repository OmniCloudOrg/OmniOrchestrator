use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::types::Json;
use serde_json::Value; 

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub salt: String,
    pub email: String,
    pub active: bool,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
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