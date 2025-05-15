pub mod error;
pub mod connection;
pub mod migration;
pub mod manager;

// Re-export commonly used types for convenience
pub use error::DatabaseError;
pub use manager::DatabaseManager;