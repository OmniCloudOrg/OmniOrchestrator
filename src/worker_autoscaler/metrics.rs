use std::collections::HashMap;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

use super::error::AutoscalerError;

/// Interface for collecting metrics from VMs and nodes
#[async_trait]
pub trait MetricsCollector: Send + Sync + std::fmt::Debug {
    /// Collect metrics from a specific VM
    async fn collect_vm_metrics(&self, vm_id: &str) -> Result<HashMap<String, f32>, AutoscalerError>;
    
    /// Collect metrics from a specific node
    async fn collect_node_metrics(&self, node_id: &str) -> Result<HashMap<String, f32>, AutoscalerError>;
    
    /// Collect aggregate metrics for all VMs and nodes
    async fn collect_aggregate_metrics(&self) -> Result<HashMap<String, f32>, AutoscalerError>;
}

/// Possible threshold types for metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricThreshold {
    Float(f32),
    Integer(i64),
    Boolean(bool),
}

/// Represents a scaling action to be taken
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalingAction {
    ScaleUp,
    ScaleDown,
    NoAction,
}