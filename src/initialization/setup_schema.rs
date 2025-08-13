use anyhow::Result;
use colored::Colorize;
use crate::schemas;
use crate::logging;
use crate::PROJECT_ROOT;

/// Loads and initializes the ClickHouse schema from SQL files.
///
/// - Retrieves the current schema version from the MySQL metadata table.
/// - Constructs the path to the schema file based on the version.
/// - Initializes the ClickHouse schema by executing the SQL file.
/// - Panics if schema initialization fails.
///
/// # Arguments
/// * `clickhouse_client` - Reference to the ClickHouse client.
/// * `pool` - Reference to the MySQL connection pool.
///
/// # Errors
/// Returns an error if schema loading or initialization fails.
pub async fn setup_schema(
    clickhouse_client: &clickhouse::Client,
    pool: &sqlx::Pool<sqlx::MySql>
) -> Result<()> {
    log::info!("{}", "Loading schema files...".blue());
    // Get the current schema version from metadata
    let schema_version =
        schemas::v1::db::queries::metadata::get_meta_value(pool, "omni_schema_version")
            .await
            .unwrap_or_else(|_| "1".to_string());

    // Build the path to the schema file
    let schema_path = format!("{}/sql/v{}/clickhouse_up.sql", PROJECT_ROOT, schema_version);
    log::info!(
        "{}",
        format!("Loading schema from path: {}", schema_path).blue()
    );

    log::info!("{}", "Initializing ClickHouse schema...".blue());
    // Initialize the ClickHouse schema
    match schemas::v1::api::logging::init_clickhouse_db(clickhouse_client, &schema_path).await {
        Ok(_) => log::info!("{}", "✓ ClickHouse schema initialized".green()),
        Err(e) => {
            log::error!(
                "{}",
                format!("Failed to initialize ClickHouse schema: {:?}", e).red()
            );
            panic!("Failed to initialize ClickHouse schema");
        }
    };

    Ok(())
}
