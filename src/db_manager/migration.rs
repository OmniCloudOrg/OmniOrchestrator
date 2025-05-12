use sqlx::{MySql, Pool};
use std::env;
use log::{info, warn, error};
use colored::Colorize;
use crate::db_manager::error::DatabaseError;
use crate::schemas::v1::models::platform::Platform;

/// Manages database schema migrations
pub struct MigrationManager;

impl MigrationManager {
    /// Initializes and migrates the main database schema
    pub async fn initialize_main_schema(pool: &Pool<MySql>) -> Result<(), DatabaseError> {
        info!("Initializing main database schema...");
        
        Self::initialize_metadata_system(pool).await?;

        let target_version = Self::get_target_schema_version()?;
        let current_version = Self::get_current_schema_version(pool).await?;
        
        if current_version == target_version {
            info!("Schema version check: OK (version {})", current_version);
            return Ok(());
        }
        
        Self::migrate_schema(pool, current_version, target_version, None).await
    }
    
    /// Initializes and migrates a platform database schema
    pub async fn initialize_platform_schema(pool: &Pool<MySql>, platform: &Platform) -> Result<(), DatabaseError> {
        info!("Initializing platform database schema for {}...", platform.name);
        
        // Initialize metadata system if needed
        Self::initialize_metadata_system(pool).await?;
        
        let target_version = Self::get_target_schema_version()?;
        let current_version = Self::get_current_schema_version(pool).await?;
        
        if current_version == target_version {
            info!("Schema version check: OK (version {})", current_version);
            return Ok(());
        }
        
        Self::migrate_schema(pool, current_version, target_version, Some(platform)).await
    }
    
    /// Gets the target schema version from environment or defaults to 1
    fn get_target_schema_version() -> Result<i64, DatabaseError> {
        let version = env::var("OMNI_ORCH_SCHEMA_VERSION")
            .unwrap_or_else(|_| "1".to_string())
            .parse::<i64>()
            .map_err(|_| DatabaseError::Other("Invalid schema version".into()))?;
            
        Ok(version)
    }
    
    /// Gets the current schema version from the database
    async fn get_current_schema_version(pool: &Pool<MySql>) -> Result<i64, DatabaseError> {
        // Check if metadata table exists
        let metadata_exists = sqlx::query("SHOW TABLES LIKE 'metadata'")
            .fetch_optional(pool)
            .await
            .map_err(|e| DatabaseError::SqlxError(e))?
            .is_some();
            
        if !metadata_exists {
            return Ok(0); // No schema version yet
        }
        
        let version = crate::schemas::v1::db::queries::metadata::get_meta_value(pool, "omni_schema_version")
            .await
            .unwrap_or_else(|_| "0".to_string())
            .parse::<i64>()
            .unwrap_or(0);
            
        Ok(version)
    }
    
    /// Initializes the metadata system if it doesn't exist
    async fn initialize_metadata_system(pool: &Pool<MySql>) -> Result<(), DatabaseError> {
        info!("Initializing metadata system...");
        
        crate::schemas::v1::db::queries::metadata::initialize_metadata_system(pool)
            .await
            .map_err(|e| DatabaseError::MigrationError(format!(
                "Failed to initialize metadata system: {}", e
            )))?;
            
        info!("✓ Metadata system initialized");
        Ok(())
    }
    
    /// Migrates a schema from one version to another
    async fn migrate_schema(
        pool: &Pool<MySql>, 
        current_version: i64, 
        target_version: i64,
        platform: Option<&Platform>
    ) -> Result<(), DatabaseError> {
        warn!(
            "{}",
            format!(
                "Schema version mismatch! Current: {}, Target: {}",
                current_version, target_version
            ).yellow()
        );
        
        // Check for migration confirmation
        let should_proceed = if env::var("OMNI_ORCH_BYPASS_CONFIRM").unwrap_or_default() == "confirm" {
            warn!("{}", "Bypassing schema update confirmation due to env var".yellow());
            true
        } else {
            // In an actual implementation, you would prompt the user here
            // For simplicity, we'll just log a message and proceed
            warn!("Type 'confirm' to update schema version:");
            // Assume confirmed for this example
            true
        };
        
        if !should_proceed {
            return Err(DatabaseError::Other("Schema update cancelled by user".into()));
        }
        
        // Perform the migration
        match platform {
            Some(platform) => {
                // Platform-specific schema
                info!("Initializing platform database schema...");
                crate::schemas::v1::db::init_platform_schema(platform, target_version, pool)
                    .await
                    .map_err(|e| DatabaseError::MigrationError(format!(
                        "Failed to migrate platform schema: {}", e
                    )))?;
                
                info!("✓ Platform database schema initialized");
                    
                // Also initialize sample data
                info!("Initializing platform sample data...");
                crate::schemas::v1::db::sample_platform_data(pool, target_version)
                    .await
                    .map_err(|e| DatabaseError::MigrationError(format!(
                        "Failed to initialize platform sample data: {}", e
                    )))?;
                
                info!("✓ Platform sample data initialized");
            },
            None => {
                // Main schema
                info!("Initializing deployment database schema...");
                crate::schemas::v1::db::init_deployment_schema(target_version, pool)
                    .await
                    .map_err(|e| DatabaseError::MigrationError(format!(
                        "Failed to migrate deployment schema: {}", e
                    )))?;
                
                info!("✓ Deployment database schema initialized");
                    
                // Also initialize sample data
                info!("Initializing deployment sample data...");
                crate::schemas::v1::db::sample_deployment_data(pool, target_version)
                    .await
                    .map_err(|e| DatabaseError::MigrationError(format!(
                        "Failed to initialize deployment sample data: {}", e
                    )))?;
                
                info!("✓ Deployment sample data initialized");
            }
        }
        
        // Update schema version
        crate::schemas::v1::db::queries::metadata::set_meta_value(
            pool,
            "omni_schema_version",
            &target_version.to_string(),
        )
        .await
        .map_err(|e| DatabaseError::MigrationError(format!(
            "Failed to update schema version: {}", e
        )))?;
        
        info!("Schema migrated from version {} to {}", current_version, target_version);
        
        Ok(())
    }
}