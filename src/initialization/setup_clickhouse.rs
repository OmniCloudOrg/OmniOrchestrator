use anyhow::Result;
use colored::Colorize;
use std::env;

/// Initializes and tests the connection to the ClickHouse database.
///
/// - Loads the ClickHouse URL from environment variables or defaults.
/// - Creates a ClickHouse client and attempts a test query to verify connectivity.
/// - Panics if the connection test fails.
///
/// # Returns
/// Returns a configured `clickhouse::Client` ready for use.
pub async fn setup_clickhouse() -> Result<clickhouse::Client> {
    // Load ClickHouse URL from environment or .env file
    let clickhouse_url = env::var("CLICKHOUSE_URL").unwrap_or_else(|_| {
        dotenv::dotenv().ok();
        env::var("DEFAULT_CLICKHOUSE_URL").unwrap_or_else(|_| "http://localhost:8123".to_string())
    });
    
    log::info!("{}", format!("ClickHouse URL: {}", clickhouse_url).blue());
    log::info!("{}", "Initializing ClickHouse connection...".blue());

    // Build the ClickHouse client
    let clickhouse_client = clickhouse::Client::default()
        .with_url(&clickhouse_url)
        .with_database("default")
        .with_user("default")
        .with_password("your_secure_password");

    // Test the connection by executing a simple query
    match clickhouse_client.query("SELECT 1").execute().await {
        Ok(_) => log::info!("✓ ClickHouse connection test successful"),
        Err(e) => {
            log::error!("ClickHouse connection test failed: {:?}", e);
            panic!("Cannot connect to ClickHouse");
        }
    }

    log::info!("{}", "✓ ClickHouse connection established".green());
    log::info!("{}", "✓ ClickHouse connection pool initialized".green());

    Ok(clickhouse_client)
}
