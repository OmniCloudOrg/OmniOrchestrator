use sqlx::{MySql, MySqlPool, Pool};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use log::{info, warn, error};
use crate::db_manager::error::DatabaseError;

/// Manages database connections across the application
pub struct ConnectionManager {
    /// Base URL for database connections
    base_url: String,
    
    /// Main application database pool
    main_pool: Pool<MySql>,
    
    /// Platform-specific database pools
    platform_pools: Arc<RwLock<HashMap<i64, Pool<MySql>>>>,
}

impl ConnectionManager {
    /// Creates a new connection manager
    pub async fn new(base_url: &str) -> Result<Self, DatabaseError> {
        // Connect to the MySQL server without specifying a database
        info!("Connecting to MySQL server at {}", base_url);
        let server_pool = MySqlPool::connect(base_url)
            .await
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;
            
        // Ensure the main database exists
        Self::ensure_database_exists(&server_pool, "omni").await?;
            
        // Connect to the main database
        let main_db_url = format!("{}/omni", base_url);
        info!("Connecting to main database at {}", main_db_url);
        let main_pool = MySqlPool::connect(&main_db_url)
            .await
            .map_err(|e| DatabaseError::ConnectionError(format!(
                "Failed to connect to main database: {}", e
            )))?;
        
        info!("✓ Database connection established");
            
        Ok(Self {
            base_url: base_url.to_string(),
            main_pool,
            platform_pools: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    /// Ensures a database exists, creating it if necessary
    pub async fn ensure_database_exists(pool: &Pool<MySql>, db_name: &str) -> Result<(), DatabaseError> {
        info!("Ensuring database exists: {}", db_name);
        let query = format!("CREATE DATABASE IF NOT EXISTS `{}`", db_name);
        sqlx::query(&query)
            .execute(pool)
            .await
            .map_err(|e| DatabaseError::SqlxError(e))?;
            
        info!("✓ Database {} exists or was created", db_name);
        Ok(())
    }
    
    /// Gets the main database pool
    pub fn main_pool(&self) -> &Pool<MySql> {
        &self.main_pool
    }
    
    /// Gets or creates a platform-specific database pool
    pub async fn platform_pool(&self, platform_id: i64, platform_name: &str) -> Result<Pool<MySql>, DatabaseError> {
        // Check if we already have this pool
        {
            let pools = self.platform_pools.read().await;
            if let Some(pool) = pools.get(&platform_id) {
                return Ok(pool.clone());
            }
        }
        
        // If not found, create a new pool
        let db_name = format!("omni_p_{}", platform_name);
        
        // Ensure the database exists
        let server_pool = MySqlPool::connect(&self.base_url)
            .await
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;
            
        Self::ensure_database_exists(&server_pool, &db_name).await?;
        
        // Connect to the platform database
        let platform_db_url = format!("{}/{}", self.base_url, db_name);
        info!("Creating pool for platform {}: {}", platform_name, platform_db_url);
        
        let pool = MySqlPool::connect(&platform_db_url)
            .await
            .map_err(|e| DatabaseError::ConnectionError(format!(
                "Failed to connect to platform database {}: {}", 
                db_name, e
            )))?;
            
        // Store the pool
        {
            let mut pools = self.platform_pools.write().await;
            pools.insert(platform_id, pool.clone());
        }
        
        Ok(pool)
    }
}