// backup/coordinator/types.rs
//
// Type definitions for the backup coordinator

use chrono::{DateTime, Utc};

/// Tracks the status of a backup job for a specific component
#[derive(Debug, Clone)]
pub struct BackupJobStatus {
    /// Unique identifier for the node being backed up
    pub node_id: String,
    
    /// Type of component being backed up (e.g., "system-core", "director")
    pub component_type: String,
    
    /// Current status of the backup job ("starting", "running", "completed", "failed")
    pub status: String,
    
    /// Progress percentage (0.0 to 100.0)
    pub progress: f32,
    
    /// Path to the ISO file if successful
    pub iso_path: Option<String>,
    
    /// Error message if failed
    pub error: Option<String>,
    
    /// When the backup job started
    pub started_at: DateTime<Utc>,
    
    /// When the backup job completed (or failed)
    pub completed_at: Option<DateTime<Utc>>,
    
    /// Size of the backup in bytes
    pub size_bytes: u64,
}