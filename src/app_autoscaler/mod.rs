pub mod error;
pub mod node_types;
pub mod app;
pub mod agent;
pub mod metrics;
pub mod policy;
pub mod app_autoscaler;

// Re-export commonly used types
pub use error::AutoscalerError;
pub use node_types::{Node, NodeType};
pub use app::{AppInstance, AppInstanceState, AppConfig, AppTemplate};
pub use agent::{Agent, CloudAgent};
pub use metrics::{MetricsCollector, MetricThreshold, ScalingAction};
pub use policy::ScalingPolicy;
pub use app_autoscaler::AppAutoscaler;
pub use policy::create_default_cpu_memory_scaling_policy;