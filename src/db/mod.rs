use std::fs;
use mysql::prelude::*;
use mysql::*;
use once_cell::sync::Lazy;

pub mod queries;
pub mod tables;

static DB_POOL: Lazy<Pool> = Lazy::new(|| {
    let url = "mysql://root:@localhost:4000";
    Pool::new(url).expect("Failed to create database pool")
});

/// Get a connection from the pool
pub fn get_conn() -> Result<PooledConn> {
    DB_POOL.get_conn()
}

/// Initialize the database with the v1 schema
pub fn init_db() -> Result<()> {
    let mut conn = get_conn()?;

    // Load SQL file as text from disk
    let sql: &str = include_str!("../../sql/db_init.sql");

    // Execute each statement in the SQL file individually
    for statement in sql.split(';').filter(|s| !s.trim().is_empty()) {
        conn.query_drop(statement)?;
    }
    Ok(())
}

/// Initialize the database with sample data to test against
pub fn init_sample_data() -> Result<()> {
    let mut conn = get_conn()?;

    // Load SQL file as text from disk
    let sql = fs::read_to_string("./sql/sample_data.sql").expect("Failed to read SQL file");

    // Execute each statement in the SQL file individually
    for statement in sql.split(';').filter(|s| !s.trim().is_empty()) {
        conn.query_drop(statement)?;
    }
    Ok(())
}