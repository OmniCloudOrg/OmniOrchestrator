# Worker Autoscaler

A comprehensive system for automatically scaling worker nodes based on resource utilization and custom metrics.

## Overview

The Worker Autoscaler is a Rust-based autoscaling system designed to dynamically adjust the number of worker nodes running across a cluster. It monitors resource utilization and custom metrics to make intelligent scaling decisions, ensuring optimal resource usage and application performance.

## Features

- **Automatic scaling**: Scale worker nodes up or down based on CPU, memory usage, and custom metrics
- **Multi-cloud support**: Works with any cloud provider through the Director abstraction
- **Customizable policies**: Define thresholds, cooldown periods, and scaling increments
- **Resource-aware scaling**: Considers available node capacity when making scaling decisions
- **Scale-down protection**: Gradual scale down with delay periods to prevent thrashing
- **Detailed metrics**: Track scaling history and performance metrics

## Architecture

The Worker Autoscaler consists of the following key components:

- **WorkerAutoscaler**: Core component that manages scaling decisions
- **Director**: Interface for deploying and managing VMs on nodes
- **Node**: Representation of physical or cloud servers with capacity constraints
- **VM**: Representation of virtual machines with resource requirements
- **ScalingPolicy**: Configuration for scaling behavior, thresholds and limits
- **MetricsCollector**: Interface for collecting performance metrics from VMs and nodes

## Usage Example

```rust
use autoscaler::{WorkerAutoscaler, ScalingPolicy, CloudDirector};
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Create a scaling policy
    let mut policy = ScalingPolicy::default();
    policy.max_worker_count = 10;
    policy.min_worker_count = 2;
    policy.cooldown_period = Duration::from_secs(300);
    
    // Initialize the autoscaler
    let mut autoscaler = WorkerAutoscaler::new(2, 2, policy);
    
    // Add a cloud director
    let director = Arc::new(CloudDirector::new(
        "aws-director".to_string(),
        "aws".to_string(),
        "us-west-2".to_string()
    ));
    autoscaler.add_director(director);
    
    // Discover existing nodes and VMs
    autoscaler.discover_nodes().await.unwrap();
    autoscaler.discover_vms().await.unwrap();
    
    // Configure VM template
    let template = VMTemplate::default();
    autoscaler.set_vm_template(template);
    
    // Main monitoring loop
    loop {
        // Collect current metrics
        let metrics = collect_metrics().await;
        
        // Check if scaling is needed
        match autoscaler.check_scaling(&metrics).unwrap() {
            ScalingAction::ScaleUp => {
                println!("Scaling up...");
                autoscaler.scale_up().await.unwrap();
            },
            ScalingAction::ScaleDown => {
                println!("Scaling down...");
                autoscaler.scale_down().await.unwrap();
            },
            ScalingAction::NoAction => {
                println!("No scaling action needed");
            }
        }
        
        // Wait before next evaluation
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

async fn collect_metrics() -> HashMap<String, f32> {
    // In a real implementation, collect metrics from your monitoring system
    let mut metrics = HashMap::new();
    metrics.insert("cpu_utilization".to_string(), 65.0);
    metrics.insert("memory_utilization".to_string(), 70.0);
    metrics
}
```

## Configuration Options

The autoscaler behavior can be customized through the ScalingPolicy:

- `max_worker_count`: Maximum number of worker nodes allowed
- `min_worker_count`: Minimum number of worker nodes required
- `cooldown_period`: Time to wait between scaling actions
- `scale_up_increment`: Number of nodes to add during scale up
- `scale_down_increment`: Number of nodes to remove during scale down
- `scale_down_delay`: Time to wait before scaling down to prevent thrashing
- `max_scale_down_percentage`: Maximum percentage of nodes that can be removed at once
- `metrics_thresholds`: Custom thresholds for scaling decisions
