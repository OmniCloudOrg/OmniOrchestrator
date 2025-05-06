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

/// Create a new backup
/// This function creates a new app backup in the database
pub async fn create_backup(
    pool: &State<sqlx::Pool<MySql>>,
    backup: &Backup,
) -> Result<Backup, sqlx::Error> {
    sqlx::query(
        "INSERT INTO backups (
            name, description, created_at, created_by, backup_type, status, format_version,
            source_environment, encryption_method, encryption_key_id, size_bytes, has_system_core,
            has_directors, has_orchestrators, has_network_config, has_app_definitions, has_volume_data,
            included_apps, included_services, last_validated_at, last_restored_at, restore_target_environment,
            restore_status, storage_location, manifest_path, metadata
        ) VALUES (
            ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
        )",
    )
    .bind(backup.name.clone())
    .bind(backup.description.clone())
    .bind(backup.created_at)
    .bind(backup.created_by.clone())
    .bind(backup.backup_type.clone())
    .bind(backup.status.clone())
    .bind(backup.format_version.clone())
    .bind(backup.source_environment.clone())
    .bind(backup.encryption_method.clone())
    .bind(backup.encryption_key_id)
    .bind(backup.size_bytes)
    .bind(backup.has_system_core)
    .bind(backup.has_directors)
    .bind(backup.has_orchestrators)
    .bind(backup.has_network_config)
    .bind(backup.has_app_definitions)
    .bind(backup.has_volume_data)
    .bind(backup.included_apps.clone())
    .bind(backup.included_services.clone())
    .bind(backup.last_validated_at)
    .bind(backup.last_restored_at)
    .bind(backup.restore_target_environment.clone())
    .bind(backup.restore_status.clone())
    .bind(backup.storage_location.clone())
    .bind(backup.manifest_path.clone())
    .bind(backup.metadata.clone())
    .execute(&**pool)
    .await?;

    let last_insert_id: i64 = sqlx::query_scalar("SELECT LAST_INSERT_ID()")
        .fetch_one(&**pool)
        .await?;

    let created_backup = sqlx::query_as::<_, Backup>(
        "SELECT * FROM backups WHERE id = ?",
    )
    .bind(last_insert_id)
    .fetch_one(&**pool)
    .await?;
    Ok(created_backup)
}
