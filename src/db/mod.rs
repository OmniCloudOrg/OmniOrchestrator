pub mod queries;
pub mod tables;

use sqlx::mysql::MySqlPool;
use sqlx::{pool, Acquire, MySql, Pool};
use anyhow::Context;

pub async fn init_conn(database_url: &str) -> anyhow::Result<Pool<MySql>> {
    MySqlPool::connect(database_url)
        .await
        .context("Failed to connect to database")
}

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

/// Split SQL into individual statements while handling edge cases
fn split_sql_statements(sql: &str) -> Vec<String> {
    let mut statements = Vec::new();
    let mut current_statement = String::new();
    let mut in_string = false;
    let mut in_comment = false;
    let mut delimiter = ';';

    for line in sql.lines() {
        let trimmed = line.trim();
        
        // Skip empty lines
        if trimmed.is_empty() {
            continue;
        }

        // Handle DELIMITER changes (common in MySQL scripts)
        if trimmed.to_uppercase().starts_with("DELIMITER") {
            if let Some(new_delimiter) = trimmed.chars().nth(9) {
                delimiter = new_delimiter;
                continue;
            }
        }

        // Handle comments
        if trimmed.starts_with("--") || trimmed.starts_with("#") {
            continue;
        }

        if trimmed.starts_with("/*") {
            in_comment = true;
            continue;
        }

        if trimmed.ends_with("*/") {
            in_comment = false;
            continue;
        }

        if in_comment {
            continue;
        }

        // Add the line to current statement
        current_statement.push_str(line);
        current_statement.push('\n');

        // Check for statement termination
        let mut chars: Vec<char> = line.chars().collect();
        while let Some(c) = chars.pop() {
            if c == '"' || c == '\'' {
                in_string = !in_string;
            } else if c == delimiter && !in_string {
                // We found a statement terminator
                if !current_statement.trim().is_empty() {
                    statements.push(current_statement.trim().to_string());
                    current_statement.clear();
                }
                break;
            }
        }
    }

    // Add the last statement if it doesn't end with a delimiter
    if !current_statement.trim().is_empty() {
        statements.push(current_statement.trim().to_string());
    }

    statements
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