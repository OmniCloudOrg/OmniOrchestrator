use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub password: String,
    pub active: Option<i32>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub resource_type: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionRole {
    pub permissions_id: i32,
    pub role_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleUser {
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Org {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrgMember {
    pub id: i32,
    pub org_id: i32,
    pub user_id: i32,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    pub id: i32,
    pub name: String,
    pub org_id: i32,
    pub git_repo: Option<String>,
    pub git_branch: String,
    pub buildpack_url: Option<String>,
    pub region_id: Option<i32>,
    pub maintenance_mode: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Region {
    pub id: i32,
    pub name: String,
    pub provider: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instance {
    pub id: i32,
    pub app_id: i32,
    pub instance_type: String,
    pub status: String,
    pub container_id: Option<String>,
    pub pod_name: Option<String>,
    pub node_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Domain {
    pub id: i32,
    pub app_id: i32,
    pub name: String,
    pub ssl_enabled: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Build {
    pub id: i32,
    pub app_id: i32,
    pub source_version: Option<String>,
    pub status: String,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Deployment {
    pub id: i32,
    pub app_id: i32,
    pub build_id: i32,
    pub status: String,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigVar {
    pub id: i32,
    pub app_id: i32,
    pub key: String,
    pub value: Option<String>,
    pub is_secret: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metric {
    pub id: i32,
    pub instance_id: i32,
    pub metric_name: String,
    pub metric_value: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceLog {
    pub id: i32,
    pub instance_id: i32,
    pub log_type: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: i32,
    pub org_id: i32,
    pub name: String,
    pub key_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: i32,
    pub user_id: Option<i32>,
    pub org_id: Option<i32>,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub created_at: DateTime<Utc>,
}