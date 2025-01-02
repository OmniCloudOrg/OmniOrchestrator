use rusqlite::{ Connection, Result };
pub mod queries;

/// Initialize the database with the v1 schema
pub fn init_db() -> Result<()> {
    // Check if database file exists
    if !std::path::Path::new("cluster.db").exists() {
        let conn = Connection::open("cluster.db")?;

        // Load SQL file as text from disk
        let sql = std::fs::read_to_string("./sql/db_init.sql").expect("Failed to read SQL file");

        // Execute each statement in the SQL file individually
        for statement in sql.split(';').filter(|s| !s.trim().is_empty()) {
            conn.execute(statement, [])?;
        }
    }
    Ok(())
}

/// Initialize the database with sample data to test against
pub fn init_sample_data() -> Result<()> {
    let conn = Connection::open("cluster.db")?;

    // Load SQL file as text from disk
    let sql = std::fs::read_to_string("./sql/sample_data.sql").expect("Failed to read SQL file");

    // Execute each statement in the SQL file individually
    for statement in sql.split(';').filter(|s| !s.trim().is_empty()) {
        conn.execute(statement, [])?;
    }
    Ok(())
}