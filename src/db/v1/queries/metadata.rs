use sqlx::{MySql, Pool};
use anyhow::{Context, Result};
use tokio::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use lazy_static::lazy_static;

// Global mutex for metadata operations to prevent race conditions
lazy_static! {
    static ref METADATA_MUTEX: Mutex<()> = Mutex::new(());
}

pub async fn create_meta_table(pool: &Pool<MySql>) -> Result<()> {
    // Acquire a lock to ensure only one thread can modify the metadata table at a time
    let _lock = METADATA_MUTEX.lock().await;
    
    // First check if table exists to prevent unnecessary queries
    if meta_table_exists_internal(pool).await? {
        return Ok(());
    }
    
    // Use stronger uniqueness enforcement in schema
    let result = sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS metadata (
            id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
            `key` VARCHAR(255) NOT NULL,
            value TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
            UNIQUE KEY unique_key (`key`),
            INDEX idx_metadata_key (`key`)
        ) ENGINE=InnoDB
        "#,
    )
    .execute(pool)
    .await
    .context("Error creating metadata table")?;

    log::info!("Successfully created metadata table");
    
    // After creating the table, ensure no duplicates exist
    cleanup_duplicate_keys_internal(pool).await?;
    
    Ok(())
}

pub async fn get_meta_value(pool: &Pool<MySql>, key: &str) -> Result<String> {
    // No need for mutex here since we're only reading
    let value = sqlx::query_scalar::<_, String>(
        "SELECT value FROM metadata WHERE `key` = ?"
    )
    .bind(key)
    .fetch_one(pool)
    .await
    .context(format!("Failed to fetch metadata value for key '{}'", key))?;

    Ok(value)
}

pub async fn set_meta_value(pool: &Pool<MySql>, key: &str, value: &str) -> Result<()> {
    // Acquire a lock to ensure only one thread can modify a key at a time
    let _lock = METADATA_MUTEX.lock().await;
    
    // Use a transaction to ensure atomicity
    let mut tx = pool.begin().await.context("Failed to begin transaction")?;
    
    // First delete any existing entries with this key to ensure no duplicates
    sqlx::query("DELETE FROM metadata WHERE `key` = ?")
        .bind(key)
        .execute(&mut *tx)
        .await
        .context(format!("Failed to delete existing metadata entry for key '{}'", key))?;
    
    // Then insert the new value
    sqlx::query("INSERT INTO metadata (`key`, value) VALUES (?, ?)")
        .bind(key)
        .bind(value)
        .execute(&mut *tx)
        .await
        .context(format!("Failed to insert metadata value for key '{}'", key))?;
    
    // Commit the transaction
    tx.commit().await.context("Failed to commit transaction")?;
    
    Ok(())
}

// Non-public internal version without lock
async fn meta_table_exists_internal(pool: &Pool<MySql>) -> Result<bool> {
    let table_exists = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = DATABASE() AND table_name = 'metadata'"
    )
    .fetch_one(pool)
    .await
    .context("Failed to check if metadata table exists")?;

    Ok(table_exists > 0)
}

// Public version with mutex protection
pub async fn meta_table_exists(pool: &Pool<MySql>) -> Result<bool> {
    // No need for mutex here since we're only reading
    meta_table_exists_internal(pool).await
}

// Non-public internal version without lock
async fn cleanup_duplicate_keys_internal(pool: &Pool<MySql>) -> Result<usize> {
    // Find duplicate keys
    let duplicates = sqlx::query_as::<_, (String,)>(
        r#"
        SELECT `key` FROM metadata 
        GROUP BY `key` 
        HAVING COUNT(*) > 1
        "#
    )
    .fetch_all(pool)
    .await
    .context("Failed to fetch duplicate keys")?;
    
    let mut cleaned_count = 0;
    
    // For each duplicate key, keep only the latest entry
    for (key,) in duplicates {
        let mut tx = pool.begin().await?;
        
        // Get the ID of the most recently updated entry
        let latest_id = sqlx::query_scalar::<_, i64>(
            "SELECT id FROM metadata WHERE `key` = ? ORDER BY updated_at DESC LIMIT 1"
        )
        .bind(&key)
        .fetch_one(&mut *tx)
        .await
        .context(format!("Failed to get latest entry ID for key '{}'", key))?;
        
        // Delete all entries with this key except the latest one
        let deleted = sqlx::query(
            "DELETE FROM metadata WHERE `key` = ? AND id != ?"
        )
        .bind(&key)
        .bind(latest_id)
        .execute(&mut *tx)
        .await
        .context(format!("Failed to delete duplicate entries for key '{}'", key))?;
        
        cleaned_count += deleted.rows_affected() as usize;
        
        tx.commit().await.context("Failed to commit transaction")?;
        
        if cleaned_count > 0 {
            log::warn!("Cleaned up {} duplicate entries for key '{}'", cleaned_count, key);
        }
    }
    
    Ok(cleaned_count)
}

// Public version with mutex protection
pub async fn cleanup_duplicate_keys(pool: &Pool<MySql>) -> Result<usize> {
    let _lock = METADATA_MUTEX.lock().await;
    cleanup_duplicate_keys_internal(pool).await
}

// Safe initialization function that ensures table exists and is clean
pub async fn initialize_metadata_system(pool: &Pool<MySql>) -> Result<()> {
    let _lock = METADATA_MUTEX.lock().await;
    
    // Create table if it doesn't exist
    if !meta_table_exists_internal(pool).await? {
        // Use direct query execution to create table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS metadata (
                id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
                `key` VARCHAR(255) NOT NULL,
                value TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                UNIQUE KEY unique_key (`key`),
                INDEX idx_metadata_key (`key`)
            ) ENGINE=InnoDB
            "#,
        )
        .execute(pool)
        .await
        .context("Error creating metadata table")?;
        
        log::info!("Successfully created metadata table");
    }
    
    // Always clean up any duplicates that may exist
    let cleaned = cleanup_duplicate_keys_internal(pool).await?;
    if cleaned > 0 {
        log::error!("Cleaned up {} total duplicate metadata entries during initialization, this could indicate issues with your usage of the matadata API", cleaned);
    }
    
    Ok(())
}

// Cache implementation to reduce database access
pub struct MetadataCache {
    pool: Pool<MySql>,
    cache: HashMap<String, String>,
}

impl MetadataCache {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self {
            pool,
            cache: HashMap::new(),
        }
    }
    
    pub async fn get(&mut self, key: &str) -> Result<String> {
        // Check cache first
        if let Some(value) = self.cache.get(key) {
            return Ok(value.clone());
        }
        
        // If not in cache, get from database
        let value = get_meta_value(&self.pool, key).await?;
        
        // Store in cache
        self.cache.insert(key.to_string(), value.clone());
        
        Ok(value)
    }
    
    pub async fn set(&mut self, key: &str, value: &str) -> Result<()> {
        // Update database
        set_meta_value(&self.pool, key, value).await?;
        
        // Update cache
        self.cache.insert(key.to_string(), value.to_string());
        
        Ok(())
    }
    
    pub async fn refresh_cache(&mut self) -> Result<()> {
        // Clear cache
        self.cache.clear();
        
        // Get all metadata entries
        let entries = sqlx::query_as::<_, (String, String)>(
            "SELECT `key`, value FROM metadata"
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch metadata entries for cache refresh")?;
        
        // Populate cache
        for (key, value) in entries {
            self.cache.insert(key, value);
        }
        
        Ok(())
    }
}