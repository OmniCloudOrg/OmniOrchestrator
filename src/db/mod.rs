pub mod queries;
pub mod tables;

use sqlx::mysql::MySqlPool;
use sqlx::{MySql, Pool};
use anyhow::Context;

pub async fn init_conn(database_url: &str) -> anyhow::Result<Pool<MySql>> {
    MySqlPool::connect(database_url)
        .await
        .context("Failed to connect to database")
}

pub fn init_schema(version: i64) -> String {
    let mut schema: String = String::new();
    let schema_v1 = include_str!("../../sql/db_init.sql");

    schema.push_str(schema_v1);

    // Add all versions up to the requested schema version to create the correct init script
    for v in 1..=version {
        let version_file = format!("./sql/versions/V{}/up.sql", v);
        if let Ok(sql) = std::fs::read_to_string(version_file) {
            schema.push_str(&sql);
        }
    }
    schema
}