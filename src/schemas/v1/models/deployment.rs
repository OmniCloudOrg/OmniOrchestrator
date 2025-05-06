use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use serde_json::Value;

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