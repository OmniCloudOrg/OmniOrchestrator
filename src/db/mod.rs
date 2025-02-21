pub mod utils;
pub mod v1;

pub use v1::tables;
use anyhow::Context;
use sqlx::mysql::MySqlPool;
use utils::split_sql_statements;
use sqlx::{pool, Acquire, MySql, Pool};

pub async fn init_schema(version: i64, pool: &sqlx::Pool<MySql>) -> Result<(), sqlx::Error> {
    println!("Initializing schema version {}", version);

    // Load base schema
    let mut statements = split_sql_statements(include_str!("../../sql/db_init.sql"));

    // Add all versions up to the requested schema version
    for v in 1..=version {
        let version_file = format!("./sql/versions/V{}/up.sql", v);
        if let Ok(sql) = std::fs::read_to_string(version_file.clone()) {
            println!("Stepping up to version {} using {}", v, version_file);
            statements.extend(split_sql_statements(&sql));
        }
    }

    // Execute each statement separately
    for statement in statements {
        if !statement.trim().is_empty() {
            println!("Executing statement: {}", statement);
            sqlx::query(&statement)
                .execute(pool)
                .await?;
        }
    }

    Ok(())
}

pub async fn sample_data(pool: &sqlx::Pool<MySql>) -> Result<(), sqlx::Error> {
    let mut conn = pool.acquire().await?;
    let mut trans = conn.begin().await?;
    let statements = split_sql_statements(include_str!("../../sql/sample_data.sql"));

    // Execute each statement separately
    for statement in statements {
        if !statement.trim().is_empty() {
            println!("Executing statement: {}", statement);
            sqlx::query(&statement)
                .execute(pool)
                .await?;
        }
    }

    Ok(())
}