use rocket::{serde::json::Json, State};
use sqlx::MySql;
use super::super::tables::Backup;

/// Paginated backups list
pub async fn list_backups_paginated(
    pool: &State<sqlx::Pool<MySql>>,
    page: i64,
    page_size: i64,
) -> Result<Vec<Backup>, sqlx::Error> {
    let backups = sqlx::query_as::<_, Backup>(
        "SELECT * FROM backups LIMIT ? OFFSET ?",
    )
    .bind(page_size)
    .bind((page - 1) * page_size)
    .fetch_all(&**pool)
    .await?;
    println!("Found {} backups", backups.len());
    println!("Returning {} backups", backups.len());

    Ok(backups)
}

/// Paginated backups list by app_id
pub async fn list_backups_by_app_id(
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

/// Get a backup by ID
pub async fn get_backup_by_id(
    pool: &State<sqlx::Pool<MySql>>,
    backup_id: i64,
) -> Result<Option<Backup>, sqlx::Error> {
    let backup = sqlx::query_as::<_, Backup>(
        "SELECT * FROM backups WHERE id = ?",
    )
    .bind(backup_id)
    .fetch_optional(&**pool)
    .await?;
    println!("Found backup: {:?}", backup);

    Ok(backup)
}