# App Autoscaler

A robust, customizable system for automatically scaling application instances based on resource utilization and custom metrics.

## Overview

The App Autoscaler is a Rust implementation of an autoscaling system designed to dynamically adjust the number of application instances running across a cluster of nodes. It monitors resource utilization and other custom metrics to make intelligent scaling decisions, ensuring optimal resource usage and application performance.

## Features

- **Automatic scaling**: Scale application instances up or down based on CPU, memory usage, and custom metrics
- **Multi-cloud support**: Works with any cloud provider through the Agent abstraction
- **Customizable policies**: Define thresholds, cooldown periods, and scaling increments
- **Resource-aware scaling**: Considers available node capacity when making scaling decisions
- **Scale-down protection**: Gradual scale down with delay periods to prevent thrashing
- **Detailed metrics**: Track scaling history and performance metrics

## Architecture

The App Autoscaler consists of the following key components:

- **AppAutoscaler**: Core component that manages scaling decisions
- **Agent**: Interface for deploying and managing app instances on nodes
- **Node**: Representation of physical or cloud servers with capacity constraints
- **AppInstance**: Representation of application instances with resource requirements
- **ScalingPolicy**: Configuration for scaling behavior, thresholds and limits
- **MetricsCollector**: Interface for collecting performance metrics from instances and nodes

## Usage Example

```rust
use autoscaler::{AppAutoscaler, ScalingPolicy, CloudAgent};
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
    let mut autoscaler = AppAutoscaler::new(2, 2, policy);
    
    // Add a cloud agent
    let agent = Arc::new(CloudAgent::new(
        "aws-agent".to_string(),
        "aws".to_string(),
        "us-west-2".to_string()
    ));
    autoscaler.add_agent(agent);
    
    // Discover existing nodes and instances
    autoscaler.discover_nodes().await.unwrap();
    autoscaler.discover_instances().await.unwrap();
    
    // Configure app template
    let template = AppTemplate::default();
    autoscaler.set_app_template(template);
    
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

- `max_worker_count`: Maximum number of app instances allowed
- `min_worker_count`: Minimum number of app instances required
- `cooldown_period`: Time to wait between scaling actions
- `scale_up_increment`: Number of instances to add during scale up
- `scale_down_increment`: Number of instances to remove during scale down
- `scale_down_delay`: Time to wait before scaling down to prevent thrashing
- `max_scale_down_percentage`: Maximum percentage of instances that can be removed at once
- `metrics_thresholds`: Custom thresholds for scaling decisions
