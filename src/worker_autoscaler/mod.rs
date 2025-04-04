pub mod error;
pub mod node_types;
pub mod vm;
pub mod director;
pub mod metrics;
pub mod policy;
pub mod worker_autoscaler;

// Re-export commonly used types
pub use error::AutoscalerError;
pub use node_types::{Node, NodeType};
pub use vm::{VM, VMState, VMConfig, VMTemplate};
pub use director::{Director, CloudDirector};
pub use metrics::{MetricsCollector, MetricThreshold, ScalingAction};
pub use policy::ScalingPolicy;
pub use worker_autoscaler::WorkerAutoscaler;
pub use policy::create_default_cpu_memory_scaling_policy;