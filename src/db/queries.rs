use rusqlite::{ Connection, Result };
use std::fs;
use std::path::Path;
use std::time::SystemTime;

pub fn make_user(
    email: &str, 
    name: &str, 
    password: &str, 
    active: bool,
    last_login_at: Option<SystemTime>,
    created_at: Option<SystemTime>,
    updated_at: Option<SystemTime>
) -> Result<i64> {
    let mut conn = Connection::open("cluster.db")?;
    
    let sql_path = Path::new("./sql/versions/V1/queries/user/user_create.sql");
    let sql = fs::read_to_string(sql_path)
        .map_err(|e| rusqlite::Error::InvalidParameterName(
            format!("Failed to read SQL file: {}", e)
        ))?;

    let tx = conn.transaction()?;
    
    for statement in sql.split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty() && !s.starts_with("--")) {
            
        if statement.contains('?') {
            tx.execute(
                statement,
                rusqlite::params![
                    email,
                    name,
                    password,
                    active,
                    last_login_at.map(|t| t.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64),
                    created_at.unwrap_or(SystemTime::now()).duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64,
                    updated_at.unwrap_or(SystemTime::now()).duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64
                ]
            )?;
        } else {
            tx.execute(statement, [])?;
        }
    }
    
    let new_user_id: i64 = tx.query_row(
        "SELECT last_insert_rowid()",
        [],
        |row| row.get(0)
    )?;
    
    tx.commit()?;
    
    Ok(new_user_id)
}