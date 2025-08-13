use anyhow::Result;
use colored::Colorize;
use std::{env, sync::Arc};
use crate::db_manager::DatabaseManager;
use crate::logging;

/// Initializes the deployment database and registers all platforms.
///
/// - Loads the database URL from environment variables or falls back to defaults.
/// - Connects to the database and wraps the manager in an `Arc` for shared access.
/// - Discovers all platforms and pre-initializes their connection pools for fast access.
///
/// # Errors
/// Returns an error if the database connection or platform pool initialization fails.
pub async fn setup_database() -> Result<Arc<DatabaseManager>> {
    // Load database URL from environment or .env file
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        dotenv::dotenv().ok();
        env::var("DEFAULT_DATABASE_URL")
            .unwrap_or_else(|_| "mysql://root:root@localhost:4001".to_string())
    });

    log::info!("{}", format!("Database URL: {}", database_url).blue());
    let db_manager = Arc::new(DatabaseManager::new(&database_url).await?);
    
    // Print a banner for platform registration
    logging::print_banner("Platform Database Registration", |s| s.bright_blue());

    // Retrieve all platforms from the database
    let platforms = db_manager.get_all_platforms().await?;
    log::info!("{}", format!("Found {} platforms", platforms.len()).blue());

    // Pre-initialize connection pools for each platform
    for platform in &platforms {
        log::info!(
            "{}",
            format!(
                "Pre-initializing connection for platform: {}",
                platform.name
            )
            .blue()
        );
        db_manager.get_platform_pool(&platform.name, platform.id.unwrap_or(0)).await?;
    }

    Ok(db_manager)
}
