use anyhow::{Context, Result};
use lazy_static::lazy_static;
use sqlx::{MySql, Pool};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Global mutex for synchronizing metadata operations.
///
/// This mutex prevents race conditions when multiple threads attempt to modify
/// the metadata table simultaneously. It's particularly important for operations
/// like table creation and key deduplication which require exclusive access.
lazy_static! {
    static ref METADATA_MUTEX: Mutex<()> = Mutex::new(());
}

/// Creates the metadata table in the database if it doesn't already exist.
///
/// This function ensures that the system has a place to store key-value metadata.
/// It uses a mutex to prevent multiple threads from attempting to create the table
/// simultaneously, and includes schema optimizations like indices and uniqueness constraints.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
///
/// # Returns
///
/// * `Ok(())` - Table was successfully created or already existed
/// * `Err(anyhow::Error)` - Failed to create the table
///
/// # Concurrency
///
/// This function acquires the global metadata mutex to ensure thread safety.
/// If the table already exists, it returns immediately without performing any changes.
///
/// # Schema
///
/// The metadata table is created with the following schema:
/// - `id`: Auto-incrementing primary key
/// - `key`: Unique string identifier (indexed for fast lookups)
/// - `value`: Text field storing the metadata value
/// - `created_at`: Timestamp of when the record was created
/// - `updated_at`: Timestamp of the last update to the record
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

/// Retrieves a metadata value by its key.
///
/// This function performs a simple lookup in the metadata table to retrieve
/// the value associated with the provided key.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `key` - The unique key whose value to retrieve
///
/// # Returns
///
/// * `Ok(String)` - Successfully retrieved the value for the key
/// * `Err(anyhow::Error)` - Failed to fetch the value or key doesn't exist
///
/// # Error Handling
///
/// Returns an error if the key doesn't exist in the metadata table or if a database
/// error occurs during the query execution.
///
/// # Concurrency
///
/// This function doesn't acquire the metadata mutex since it only performs a read
/// operation, which doesn't risk data corruption.
pub async fn get_meta_value(pool: &Pool<MySql>, key: &str) -> Result<String> {
    // No need for mutex here since we're only reading
    let value = sqlx::query_scalar::<_, String>("SELECT value FROM metadata WHERE `key` = ?")
        .bind(key)
        .fetch_one(pool)
        .await
        .context(format!("Failed to fetch metadata value for key '{}'", key))?;

    Ok(value)
}

/// Sets a metadata value for a specific key.
///
/// This function stores a key-value pair in the metadata table. If the key already
/// exists, its value is updated. The operation is performed atomically within a
/// transaction and is protected by a mutex to prevent concurrent modifications.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `key` - The unique key to associate with the value
/// * `value` - The value to store
///
/// # Returns
///
/// * `Ok(())` - Successfully stored the key-value pair
/// * `Err(anyhow::Error)` - Failed to store the key-value pair
///
/// # Concurrency
///
/// This function acquires the global metadata mutex to ensure thread safety
/// when multiple threads attempt to modify the same key.
///
/// # Transaction Handling
///
/// This function uses a database transaction that:
/// 1. Deletes any existing entries with the same key
/// 2. Inserts the new key-value pair
///
/// If any part of this operation fails, the entire transaction is rolled back,
/// preserving data consistency.
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
        .context(format!(
            "Failed to delete existing metadata entry for key '{}'",
            key
        ))?;

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

/// Internal function to check if the metadata table exists.
///
/// This function checks if the metadata table exists in the current database
/// without acquiring the metadata mutex. It's intended for internal use by
/// functions that already hold the mutex.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
///
/// # Returns
///
/// * `Ok(bool)` - True if the table exists, false otherwise
/// * `Err(anyhow::Error)` - Failed to check if the table exists
///
/// # Note
///
/// This is an internal function not protected by the metadata mutex.
/// It should only be called from functions that already hold the mutex
/// or where concurrent access is not a concern.
async fn meta_table_exists_internal(pool: &Pool<MySql>) -> Result<bool> {
    let table_exists = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = DATABASE() AND table_name = 'metadata'"
    )
    .fetch_one(pool)
    .await
    .context("Failed to check if metadata table exists")?;

    Ok(table_exists > 0)
}

/// Checks if the metadata table exists in the database.
///
/// This function provides a thread-safe way to check if the metadata table
/// has been created in the database.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
///
/// # Returns
///
/// * `Ok(bool)` - True if the table exists, false otherwise
/// * `Err(anyhow::Error)` - Failed to check if the table exists
///
/// # Concurrency
///
/// This function doesn't acquire the metadata mutex since it only performs a read
/// operation, which doesn't risk data corruption.
pub async fn meta_table_exists(pool: &Pool<MySql>) -> Result<bool> {
    // No need for mutex here since we're only reading
    meta_table_exists_internal(pool).await
}

/// Internal function to clean up duplicate metadata keys.
///
/// This function identifies and resolves duplicate keys in the metadata table
/// by keeping only the most recently updated entry for each key. It's intended
/// for internal use by functions that already hold the metadata mutex.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
///
/// # Returns
///
/// * `Ok(usize)` - Number of duplicate entries that were removed
/// * `Err(anyhow::Error)` - Failed to clean up duplicate keys
///
/// # Process
///
/// For each key with multiple entries, this function:
/// 1. Identifies the most recently updated entry by `updated_at` timestamp
/// 2. Deletes all other entries with the same key
/// 3. Logs a warning if duplicates were found
///
/// # Note
///
/// This is an internal function not protected by the metadata mutex.
/// It should only be called from functions that already hold the mutex.
async fn cleanup_duplicate_keys_internal(pool: &Pool<MySql>) -> Result<usize> {
    // Find duplicate keys
    let duplicates = sqlx::query_as::<_, (String,)>(
        r#"
        SELECT `key` FROM metadata 
        GROUP BY `key` 
        HAVING COUNT(*) > 1
        "#,
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
            "SELECT id FROM metadata WHERE `key` = ? ORDER BY updated_at DESC LIMIT 1",
        )
        .bind(&key)
        .fetch_one(&mut *tx)
        .await
        .context(format!("Failed to get latest entry ID for key '{}'", key))?;

        // Delete all entries with this key except the latest one
        let deleted = sqlx::query("DELETE FROM metadata WHERE `key` = ? AND id != ?")
            .bind(&key)
            .bind(latest_id)
            .execute(&mut *tx)
            .await
            .context(format!(
                "Failed to delete duplicate entries for key '{}'",
                key
            ))?;

        cleaned_count += deleted.rows_affected() as usize;

        tx.commit().await.context("Failed to commit transaction")?;

        if cleaned_count > 0 {
            log::warn!(
                "Cleaned up {} duplicate entries for key '{}'",
                cleaned_count,
                key
            );
        }
    }

    Ok(cleaned_count)
}

/// Cleans up duplicate metadata keys in a thread-safe manner.
///
/// This function provides a public, mutex-protected interface to clean up
/// duplicate keys in the metadata table. It ensures that only one thread
/// can perform this operation at a time.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
///
/// # Returns
///
/// * `Ok(usize)` - Number of duplicate entries that were removed
/// * `Err(anyhow::Error)` - Failed to clean up duplicate keys
///
/// # Concurrency
///
/// This function acquires the global metadata mutex to ensure thread safety
/// when cleaning up duplicate keys.
///
/// # Use Cases
///
/// This function is useful for:
/// - Periodic maintenance of the metadata table
/// - Resolving inconsistencies that might have been introduced by bugs
/// - Cleaning up after schema migrations or application upgrades
pub async fn cleanup_duplicate_keys(pool: &Pool<MySql>) -> Result<usize> {
    let _lock = METADATA_MUTEX.lock().await;
    cleanup_duplicate_keys_internal(pool).await
}

/// Initializes the metadata system, ensuring the table exists and is clean.
///
/// This function provides a safe way to initialize the metadata system by:
/// 1. Creating the metadata table if it doesn't exist
/// 2. Cleaning up any duplicate keys that might exist
///
/// It's designed to be called during application startup to ensure the metadata
/// system is in a consistent state before it's used.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
///
/// # Returns
///
/// * `Ok(())` - Successfully initialized the metadata system
/// * `Err(anyhow::Error)` - Failed to initialize the metadata system
///
/// # Concurrency
///
/// This function acquires the global metadata mutex to ensure thread safety
/// during initialization.
///
/// # Warning
///
/// If duplicate keys are found and cleaned up during initialization, an error
/// is logged as this could indicate issues with the application's use of the
/// metadata API.
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

/// Cache implementation for metadata to reduce database access.
///
/// This cache maintains an in-memory copy of metadata values to minimize
/// database queries. It provides methods to get and set values, as well as
/// to refresh the entire cache from the database.
///
/// # Fields
///
/// * `pool` - Database connection pool for executing queries
/// * `cache` - In-memory hash map storing key-value pairs
///
/// # Thread Safety
///
/// This struct is not thread-safe on its own and should either be used from a
/// single thread or wrapped in a synchronization primitive like `Arc<Mutex<MetadataCache>>`.
///
/// # Performance Considerations
///
/// The cache can significantly reduce database load for frequently accessed
/// metadata values, but it can become stale if other processes modify the
/// metadata table directly. Use `refresh_cache` periodically or after expected
/// external changes.
pub struct MetadataCache {
    /// Database connection pool for executing queries
    pool: Pool<MySql>,
    /// In-memory cache of metadata key-value pairs
    cache: HashMap<String, String>,
}

impl MetadataCache {
    /// Creates a new metadata cache with an empty cache and the provided database pool.
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool for executing queries
    ///
    /// # Returns
    ///
    /// A new `MetadataCache` instance with an empty cache
    pub fn new(pool: Pool<MySql>) -> Self {
        Self {
            pool,
            cache: HashMap::new(),
        }
    }

    /// Retrieves a metadata value by its key, using the cache when possible.
    ///
    /// This method first checks the in-memory cache for the requested key.
    /// If the key is not found in the cache, it queries the database and
    /// updates the cache with the result.
    ///
    /// # Arguments
    ///
    /// * `key` - The unique key whose value to retrieve
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - Successfully retrieved the value for the key
    /// * `Err(anyhow::Error)` - Failed to fetch the value or key doesn't exist
    ///
    /// # Cache Behavior
    ///
    /// This method:
    /// - Returns cached values without querying the database when possible
    /// - Automatically populates the cache with values fetched from the database
    /// - Does not refresh existing cache entries (use `refresh_cache` for that)
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

    /// Sets a metadata value for a specific key and updates the cache.
    ///
    /// This method updates both the database and the in-memory cache with
    /// the new key-value pair. This ensures that subsequent `get` calls
    /// will return the updated value without requiring a database query.
    ///
    /// # Arguments
    ///
    /// * `key` - The unique key to associate with the value
    /// * `value` - The value to store
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Successfully stored the key-value pair
    /// * `Err(anyhow::Error)` - Failed to store the key-value pair
    ///
    /// # Error Handling
    ///
    /// If the database update fails, the cache is not updated, ensuring
    /// consistency between the cache and the database.
    pub async fn set(&mut self, key: &str, value: &str) -> Result<()> {
        // Update database
        set_meta_value(&self.pool, key, value).await?;

        // Update cache
        self.cache.insert(key.to_string(), value.to_string());

        Ok(())
    }

    /// Refreshes the entire cache from the database.
    ///
    /// This method clears the in-memory cache and reloads all metadata entries
    /// from the database. It's useful when the cache might be stale due to
    /// external changes to the metadata table.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Successfully refreshed the cache
    /// * `Err(anyhow::Error)` - Failed to refresh the cache
    ///
    /// # Use Cases
    ///
    /// This method is particularly useful in scenarios such as:
    /// - After application startup to prime the cache
    /// - After scheduled maintenance that might have modified metadata
    /// - When cache staleness is detected or suspected
    /// - Periodically in long-running applications to ensure cache freshness
    pub async fn refresh_cache(&mut self) -> Result<()> {
        // Clear cache
        self.cache.clear();

        // Get all metadata entries
        let entries = sqlx::query_as::<_, (String, String)>("SELECT `key`, value FROM metadata")
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