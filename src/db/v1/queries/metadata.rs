use sqlx::{MySql, Pool};
use anyhow::Context;
// use super::super::tables::App;

pub async fn create_meta_table(pool: &Pool<MySql>) {
    let result = sqlx::query(
        r#"
                CREATE TABLE IF NOT EXISTS metadata (
                id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
                `key` VARCHAR(255) NOT NULL UNIQUE,
                value TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                INDEX idx_metadata_key (`key`)
            )
    "#,
    )
    .execute(pool)
    .await;

    match result {
        Ok(_) => println!("Successfully created metadata table"),
        Err(e) => eprintln!("Error creating metadata table: {:#?}", e),
    }
}

pub async fn get_meta_value(pool: &Pool<MySql>, key: &str) -> anyhow::Result<String> {
    let value = sqlx::query_scalar::<_, String>(
        "SELECT value FROM metadata WHERE `key` = ?"
    )
    .bind(key)
    .fetch_one(pool)
    .await
    .context("Failed to fetch metadata value")?;

    Ok(value)
}

pub async fn set_meta_value(pool: &Pool<MySql>, key: &str, value: &str) -> anyhow::Result<()> {
    let result = sqlx::query(
        "INSERT INTO metadata (`key`, value) VALUES (?, ?) ON DUPLICATE KEY UPDATE value = ?"
    )
    .bind(key)
    .bind(value)
    .bind(value)
    .execute(pool)
    .await
    .context("Failed to set metadata value")?;

    Ok(())
}

pub async fn meta_table_exists(pool: &Pool<MySql>) -> bool {
    let table_exists = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM information_schema.tables WHERE table_name = 'metadata'"
    )
    .fetch_one(pool)
    .await;

    match table_exists {
        Ok(count) => count > 0,
        Err(_) => false,
    }
}