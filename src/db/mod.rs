pub mod data_types;
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