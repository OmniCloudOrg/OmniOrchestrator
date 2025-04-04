use std::collections::HashMap;
use std::time::Duration;
use serde::{Serialize, Deserialize};

use super::metrics::MetricThreshold;

/// Configuration for scaling operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingPolicy {
    /// The maximum number of worker nodes allowed in the cluster
    pub max_worker_count: usize,
    /// The minimum number of worker nodes allowed in the cluster
    pub min_worker_count: usize,
    /// The cooldown period between scaling actions
    pub cooldown_period: Duration,
    /// Custom metrics and their thresholds for scaling decisions
    pub metrics_thresholds: HashMap<String, MetricThreshold>,
    /// Number of workers to add during scale up
    pub scale_up_increment: usize,
    /// Number of workers to remove during scale down
    pub scale_down_increment: usize,
    /// Time to wait before considering a scale down action
    pub scale_down_delay: Duration,
    /// Maximum percentage of workers that can be scaled down at once
    pub max_scale_down_percentage: f32,
    /// Whether to enable automatic scaling
    pub autoscaling_enabled: bool,
}

impl Default for ScalingPolicy {
    fn default() -> Self {
        Self {
            max_worker_count: 10,
            min_worker_count: 1,
            cooldown_period: Duration::from_secs(300), // 5 minutes
            metrics_thresholds: HashMap::new(),
            scale_up_increment: 1,
            scale_down_increment: 1,
            scale_down_delay: Duration::from_secs(600), // 10 minutes
            max_scale_down_percentage: 0.25, // 25%
            autoscaling_enabled: true,
        }
    }
}

// Example of a configuration function
pub fn create_default_cpu_memory_scaling_policy() -> ScalingPolicy {
    let mut policy = ScalingPolicy::default();
    
    let mut thresholds = HashMap::new();
    thresholds.insert("cpu_utilization".to_string(), MetricThreshold::Float(70.0));
    thresholds.insert("memory_utilization".to_string(), MetricThreshold::Float(80.0));
    
    policy.metrics_thresholds = thresholds;
    policy
}