use serde::{Deserialize, Serialize};

// TODO: @tristanpoland Review if we actually need this or should drop in favor of using a central struct. Regardless we will need to move these to the modals module and eventually to LibOmni.

/// Represents an application in the system.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Application {
    /// Unique identifier for the application
    pub id: String,
    /// Name of the application
    pub name: String,
    /// Owner of the application
    pub owner: String,
    /// Number of running instances
    pub instances: i64,
    /// Memory allocation in MB
    pub memory: i64,
    /// Current status of the application
    pub status: String,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last update timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Request data for scaling an application.
#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleRequest {
    /// Number of instances to scale to
    pub instances: i32,
    /// Memory allocation in MB to scale to
    pub memory: i32,
}

/// Statistics for an application's resource usage and performance.
#[derive(Debug, Serialize, Deserialize)]
pub struct AppStats {
    /// CPU usage as a percentage
    pub cpu_usage: f64,
    /// Memory usage in bytes
    pub memory_usage: i64,
    /// Disk usage in bytes
    pub disk_usage: i64,
    /// Average number of requests per second
    pub requests_per_second: f64,
    /// Average response time in milliseconds
    pub response_time_ms: i64,
}

/// Request data for creating a new application.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAppRequest {
    /// Name of the application
    pub name: String,
    /// Memory allocation in MB
    pub memory: i64,
    /// Number of instances
    pub instances: i64,
    /// Organization ID that owns the application
    pub org_id: i64,
}

/// Request data for updating an existing application.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAppRequest {
    /// New name for the application
    pub name: String,
    /// New memory allocation in MB
    pub memory: i64,
    /// New number of instances
    pub instances: i64,
    /// Organization ID that owns the application
    pub org_id: i64,
}