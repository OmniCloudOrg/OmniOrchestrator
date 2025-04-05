use thiserror::Error;

/// Errors that can occur during autoscaling operations
#[derive(Error, Debug)]
pub enum AutoscalerError {
    #[error("Invalid metric value: {0}")]
    InvalidMetricValue(String),
    
    #[error("Failed to apply scaling decision: {0}")]
    ScalingFailed(String),
    
    #[error("Metric not found: {0}")]
    MetricNotFound(String),
    
    #[error("Insufficient node capacity: {0}")]
    InsufficientCapacity(String),
    
    #[error("Agent communication failed: {0}")]
    AgentError(String),
    
    #[error("Node not found: {0}")]
    NodeNotFound(String),
    
    #[error("App instance not found: {0}")]
    InstanceNotFound(String),
}