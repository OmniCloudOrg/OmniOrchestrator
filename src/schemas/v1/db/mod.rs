pub mod utils;
pub mod queries;

use rocket::{http::Status, serde::json::{self, Json}};
use rocket::serde::json::json;
use sqlx::{Acquire, MySql};
use utils::split_sql_statements;
use crate::{models::platform::Platform, PROJECT_ROOT};

pub async fn init_deployment_schema(version: i64, pool: &sqlx::Pool<MySql>) -> Result<(), sqlx::Error> {
    println!("Initializing schema version {}", version);

    // Load base schema
    let base_schema_path = format!("{}/sql/v{}/omni_up.sql", PROJECT_ROOT, version);
    let base_schema_sql = std::fs::read_to_string(&base_schema_path)
        .map_err(|e| {
            println!("Failed to read base schema file '{}': {}", base_schema_path, e);
            sqlx::Error::Io(e)
        })?;
    let mut statements = split_sql_statements(&base_schema_sql);

    // Add all versions up to the requested schema version
    for v in 1..=version {
        let version_file = format!("{}/sql/versions/V{}/omni_up.sql", PROJECT_ROOT, v);
        if let Ok(sql) = std::fs::read_to_string(version_file.clone()) {
            println!("Stepping up to version {} using {}", v, version_file);
            statements.extend(split_sql_statements(&sql));
        }
    }

    // Execute each statement separately
    for statement in statements {
        if !statement.trim().is_empty() {
            println!("Executing statement: {}", statement);
            sqlx::query(&statement).execute(&*pool).await?;
        }
    }

    Ok(())
}

pub async fn init_platform_schema(
    platform_name: &str,
    platform_id: i64,
    version: i64,
    db_manager: &crate::db_manager::DatabaseManager,
) -> Result<(), sqlx::Error> {
    println!("Initializing schema version {}", version);

    // Get platform-specific database pool
    let platform_name_string = platform_name.to_string();
    let pool = match db_manager.get_platform_pool(&platform_name_string, platform_id).await {
        Ok(pool) => pool,
        Err(e) => {
            println!("Failed to connect to platform database: {}", e);
            return Err(sqlx::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to connect to platform database",
            )));
        }
    };

    // Load base schema
    let base_schema_path = format!("{}/sql/v{}/platform_up.sql", PROJECT_ROOT, version);
    let base_schema_sql = std::fs::read_to_string(&base_schema_path)
        .map_err(|e| {
            println!("Failed to read base schema file '{}': {}", base_schema_path, e);
            sqlx::Error::Io(e)
        })?;
    let mut statements = split_sql_statements(&base_schema_sql);

    // Add all versions up to the requested schema version
    for v in 1..=version {
        let version_file = format!("{}/sql/versions/V{}/platform_up.sql", PROJECT_ROOT, v);
        if let Ok(sql) = std::fs::read_to_string(version_file.clone()) {
            println!("Stepping up to version {} using {}", v, version_file);
            statements.extend(split_sql_statements(&sql));
        }
    }

    // Execute each statement separately
    for statement in statements {
        if !statement.trim().is_empty() {
            println!("Executing statement: {}", statement);
            sqlx::query(&statement).execute(&pool).await?;
        }
    }

    Ok(())
}

pub async fn sample_deployment_data(pool: &sqlx::Pool<MySql>, version: i64) -> Result<(), sqlx::Error> {
    let mut conn = pool.acquire().await?;
    let _trans = conn.begin().await?; // Changed to _trans since it's not used
    let sample_data_path = format!("{}/sql/v{}/omni_sample_data.sql", PROJECT_ROOT, version);
    let sample_data_sql = std::fs::read_to_string(&sample_data_path)
        .map_err(|e| {
            println!("Failed to read sample data file '{}': {}", sample_data_path, e);
            sqlx::Error::Io(e)
        })?;
    let statements = split_sql_statements(&sample_data_sql);

    // Execute each statement separately
    for statement in statements {
        if !statement.trim().is_empty() {
            println!("Executing statement: {}", statement);
            sqlx::query(&statement).execute(pool).await?;
        }
    }

    Ok(())
}

pub async fn sample_platform_data(pool: &sqlx::Pool<MySql>, version: i64) -> Result<(), sqlx::Error> {
    let mut conn = pool.acquire().await?;
    let _trans = conn.begin().await?; // Changed to _trans since it's not used
    let sample_data_path = format!("{}/sql/v{}/platform_sample_data.sql", PROJECT_ROOT, version);
    let sample_data_sql = std::fs::read_to_string(&sample_data_path)
        .map_err(|e| {
            println!("Failed to read sample data file '{}': {}", sample_data_path, e);
            sqlx::Error::Io(e)
        })?;
    let statements = split_sql_statements(&sample_data_sql);

    // Execute each statement separately
    for statement in statements {
        if !statement.trim().is_empty() {
            println!("Executing statement: {}", statement);
            sqlx::query(&statement).execute(pool).await?;
        }
    }

    Ok(())
}