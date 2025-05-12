use sqlx::Row;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Platform {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub table_name: String,
    pub subdomain: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}