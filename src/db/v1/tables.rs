use chrono::{DateTime, Utc};
use serde::{Serialize,Deserialize};

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
    pub id:            i64,
    pub name:          String,
    pub salt:          String,
    pub email:         String,
    pub active:        bool,
    pub password:      String,
    pub created_at:    DateTime<Utc>,
    pub updated_at:    DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct App {
    pub id:                  i64,
    pub name:                String,
    pub org_id:              i64,
    pub git_repo:            Option<String>,
    pub region_id:           Option<i64>,
    pub created_at:          DateTime<Utc>,
    pub updated_at:          DateTime<Utc>,
    pub git_branch:          Option<String>,
    pub maintenance_mode:    bool,
    pub container_image_url: Option<String>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Org {
    pub id:         i64,
    pub name:       String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Region {
    pub id:         i64,
    pub name:       String,
    pub provider:   String, // enum in DB: 'kubernetes' or 'custom'
    pub status:     String,   // enum in DB: 'active', 'maintenance', 'offline'
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Role {
    pub id:          i64,
    pub name:        String,
    pub created_at:  DateTime<Utc>,
    pub description: Option<String>,
}

#[derive(Debug, sqlx::FromRow, Serialize,Deserialize)]
pub struct Permission {
    pub id:            i64,
    pub name:          String,
    pub created_at:    DateTime<Utc>,
    pub description:   Option<String>,
    pub resource_type: Option<String>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Build {
    pub id:             i64,
    pub app_id:         i64,
    pub status:         String, // enum: 'pending', 'building', 'succeeded', 'failed'
    pub started_at:     Option<DateTime<Utc>>,
    pub created_at:     DateTime<Utc>,
    pub completed_at:   Option<DateTime<Utc>>,
    pub source_version: Option<String>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Deployment {
    pub id:           i64,
    pub status:       String, // enum: 'pending', 'in_progress', 'deployed', 'failed'
    pub app_id:       i64,
    pub build_id:     i64,
    pub created_at:   DateTime<Utc>,
    pub started_at:   Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Instance {
    pub id:              i64,
    pub app_id:          i64,
    pub status:          String,
    pub node_name:       Option<String>,
    pub created_at:      DateTime<Utc>,
    pub updated_at:      DateTime<Utc>,
    pub container_id:    Option<String>,
    pub instance_type:   String,
    pub instance_status: String,
}

#[derive(Debug, sqlx::FromRow, Serialize,Deserialize)]
pub struct AuditLog {
    pub id:            i64,
    pub org_id:        Option<i64>,
    pub action:        String,
    pub user_id:       Option<i64>,
    pub created_at:    DateTime<Utc>,
    pub resource_id:   Option<String>,
    pub resource_type: String,
}