use crate::db_manager;
use crate::db_manager::connection::ConnectionManager;
use crate::db_manager::error::DatabaseError;
use crate::db_manager::migration::MigrationManager;
use log::{error, info, warn};
use sqlx::{MySql, Pool};
use std::sync::Arc;

// Import the types we need
use libomni::types::db::v1 as types;
use types::platform::{self, Platform};

/// Central manager for all database operations
pub struct DatabaseManager {
    /// Connection manager for database pools
    connection_manager: ConnectionManager,
}

impl DatabaseManager {
    /// Creates a new database manager
    pub async fn new(connection_url: &str) -> Result<Self, DatabaseError> {
        // Create connection manager
        let connection_manager = ConnectionManager::new(connection_url).await?;

        // Create the manager
        let manager = Self { connection_manager };

        // Initialize the main database schema
        manager.initialize_main_schema().await?;

        Ok(manager)
    }

    /// Initializes the main database schema
    pub async fn initialize_main_schema(&self) -> Result<(), DatabaseError> {
        MigrationManager::initialize_main_schema(self).await
    }

    /// Gets the main database pool
    pub fn get_main_pool(&self) -> &Pool<MySql> {
        self.connection_manager.main_pool()
    }

    /// Gets or initializes a platform database
    pub async fn get_platform_pool(
        &self,
        platform_name: &String,
        platform_id: i64,
    ) -> Result<Pool<MySql>, DatabaseError> {
        // Get or create the pool
        let pool = self
            .connection_manager
            .platform_pool(platform_id, &platform_name)
            .await?;

        // TODO: Platform schema initialization needs to be relocated
        // Currently commented out pending architectural decisions about where
        // platform-specific schema initialization should be handled.
        // Consider moving to a dedicated platform management service.
        // MigrationManager::initialize_platform_schema(&pool, platform).await?;

        Ok(pool)
    }

    /// Gets all available platforms
    pub async fn get_all_platforms(&self) -> Result<Vec<Platform>, DatabaseError> {
        let pool = self.connection_manager.main_pool();

        crate::schemas::v1::db::queries::platforms::get_all_platforms(pool)
            .await
            .map_err(|e| DatabaseError::Other(format!("Failed to retrieve platforms: {}", e)))
    }

    /// Creates a new platform in the main database and initializes its schema
    pub async fn create_platform(
        &self,
        db_manager: &db_manager::DatabaseManager,
        platform: Platform
    ) -> Result<i64, DatabaseError> {
        // First, create the platform entry in the main database
        let platform = self.create_platform_entry(&platform).await?;

        let platform_id = platform.id.ok_or_else(|| {
            DatabaseError::Other("Platform ID is missing after creation".to_string())
        })?;

        self.initialize_platform_database(&db_manager, &platform.name, platform_id)
            .await?;

        info!(
            "Platform created successfully: {} (ID: {})",
            platform.name, platform_id
        );

        Ok(platform_id)
    }

    /// Creates a platform entry in the main database
    async fn create_platform_entry(&self, platform: &Platform) -> Result<Platform, DatabaseError> {
        let pool = self.connection_manager.main_pool();

        crate::schemas::v1::db::queries::platforms::create_platform(
            pool,
            platform.name.as_str(),
            Some(platform.description.as_str()),
        )
        .await
        .map_err(|e| DatabaseError::Other(format!("Failed to create platform: {}", e)))
    }

    /// Initializes a platform database schema
    async fn initialize_platform_database(
        &self,
        db_manager: &db_manager::DatabaseManager,
        platform_name: &String,
        platform_id: i64,
    ) -> Result<(), DatabaseError> {
        // Get the platform pool (this will create the database if it doesn't exist)
        let pool = self
            .connection_manager
            .platform_pool(platform_id, platform_name)
            .await?;

        // TODO: @Caznix @tristanpoland We need to find a new home for this
        // Initialize the schema
        MigrationManager::initialize_platform_schema(db_manager, platform_name.clone(), platform_id).await?;

        info!(
            "Platform database initialized for platform: {} (ID: {})",
            platform_name, platform_id
        );

        Ok(())
    }

    /// Deletes a platform and its associated database
    pub async fn delete_platform(&self, platform_id: i64) -> Result<(), DatabaseError> {
        // Get platform details before deletion for logging
        let platform = self.get_platform_by_id(platform_id).await?;

        // TODO: @Caznix @tristanpoland We need to implement this
        // Delete the platform's dedicated database
        // self.delete_platform_database(&platform.name, platform_id)
        //     .await?;

        // Delete the platform entry from the main database
        self.delete_platform_entry(platform_id).await?;

        info!(
            "Platform deleted successfully: {} (ID: {})",
            platform.name, platform_id
        );

        Ok(())
    }

    /// Gets a platform by ID
    async fn get_platform_by_id(&self, platform_id: i64) -> Result<Platform, DatabaseError> {
        let pool = self.connection_manager.main_pool();

        crate::schemas::v1::db::queries::platforms::get_platform_by_id(pool, platform_id)
            .await
            .map_err(|e| {
                DatabaseError::Other(format!(
                    "Failed to retrieve platform with ID {}: {}",
                    platform_id, e
                ))
            })
    }

    /// Deletes a platform entry from the main database
    async fn delete_platform_entry(&self, platform_id: i64) -> Result<(), DatabaseError> {
        let pool = self.connection_manager.main_pool();

        crate::schemas::v1::db::queries::platforms::delete_platform(pool, platform_id)
            .await
            .map_err(|e| {
                DatabaseError::Other(format!(
                    "Failed to delete platform with ID {}: {}",
                    platform_id, e
                ))
            })
    }

    //TODO: @Caznix @tristanpoland We need to implement this
    // Deletes a platform's dedicated database
    // async fn delete_platform_database(
    //     &self,
    //     platform_name: &String,
    //     platform_id: i64,
    // ) -> Result<(), DatabaseError> {
    //     // We need to access the connection manager to delete the database
    //     self.connection_manager
    //         .delete_platform_database(platform_id, platform_name)
    //         .await
    //         .map_err(|e| {
    //             error!(
    //                 "Failed to delete platform database for {} (ID: {}): {}",
    //                 platform_name, platform_id, e
    //             );
    //             DatabaseError::Other(format!("Failed to delete platform database: {}", e))
    //         })
    // }
}
