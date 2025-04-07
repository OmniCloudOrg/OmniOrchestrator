use rocket::{serde::json::Json, State};
use sqlx::MySql;
use super::super::tables::Backup;

/// Paginated backups list
pub async fn list_backups_paginated(
    pool: &State<sqlx::Pool<MySql>>,
    page: i64,
    page_size: i64,
) -> Result<Vec<Backup>, sqlx::Error> {
    tracing::info!(
        page = page,
        page_size = page_size,
        "Fetching paginated backups"
    );
    
    // Add simple plain text logs
    tracing::info!("About to execute SQL query");
    
    // Use match to explicitly handle success vs error
    match sqlx::query_as::<_, Backup>(
        "SELECT * FROM backups LIMIT ? OFFSET ?"
    )
    .bind(page_size)
    .bind((page - 1) * page_size)
    .fetch_all(&**pool)
    .await {
        Ok(backups) => {
            // Simple log without structured fields
            tracing::info!("Query successful, returned {} backups", backups.len());
            
            // Try to log each backup ID to see if any data is returned
            for backup in &backups {
                tracing::info!("Found backup ID: {}", backup.id);
            }
            
            tracing::info!("Returning {} backups for page {}", backups.len(), page);
            Ok(backups)
        },
        Err(err) => {
            // Log the error explicitly
            tracing::error!("Database query failed: {:?}", err);
            Err(err)
        }
    }
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

// Create a new backup
// This function creates a new app backup in the database
//

// TODO: Implement the create_backup function to insert a new backup record into the database