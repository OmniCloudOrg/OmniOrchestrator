use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Failed to connect to database: {0}")]
    ConnectionError(String),
    
    #[error("Migration failed: {0}")]
    MigrationError(String),
    
    #[error("Database '{0}' not found")]
    DatabaseNotFound(String),
    
    #[error("Schema version mismatch: current {current}, target {target}")]
    SchemaVersionMismatch { current: String, target: String },
    
    #[error("SQL error: {0}")]
    SqlxError(#[from] sqlx::Error),
    
    #[error("Other error: {0}")]
    Other(String),
}