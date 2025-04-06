use rocket::{serde::json::Json, State};
use sqlx::MySql;
use super::super::tables::Backup;

/// Paginated backups list
pub async fn list_backups_paginated(
    pool: &State<sqlx::Pool<MySql>>,
    app_id: i64,
    page: i64,
    page_size: i64,
) -> Result<Vec<Backup>, sqlx::Error> {
    let backups = sqlx::query_as::<_, Backup>(
        "SELECT * FROM backups WHERE app_id = ? LIMIT ? OFFSET ?",
    )
    .bind(app_id)
    .bind(page_size)
    .bind((page - 1) * page_size)
    .fetch_all(&**pool)
    .await?;
    println!("Found {} backups", backups.len());
    println!("Returning {} backups", backups.len());

    Ok(backups)
}