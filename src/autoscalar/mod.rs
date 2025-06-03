use lighthouse::{
    policies, utils, CallbackContext, LighthouseConfig, LighthouseResult, MetricsProvider,
    ResourceMetrics, ScaleAction, ScalingExecutor, ScaleDirection, LighthouseEngine, 
    LighthouseCallbacks, LighthouseHandle, ResourceConfig, ScalingPolicy, ScalingThreshold
};
use std::sync::Arc;
use std::collections::HashMap;
use async_trait::async_trait;
use colored::Colorize;

pub mod types;

// Clean trait implementations (outside of functions!)
#[async_trait]
impl MetricsProvider for types::OmniMetricsProvider {
    async fn get_metrics(
        &self,
        resource_id: &str,
        _context: &CallbackContext,
    ) -> LighthouseResult<Option<ResourceMetrics>> {
        // Your metrics collection logic
        let metrics = vec![
            ("cpu_usage", 75.0),
            ("memory_usage", 65.0),
            ("disk_io", 120.0),
            ("network_io", 200.0),
        ];

        Ok(Some(utils::multi_metrics(
            resource_id,
            "omni_application",
            metrics,
        )))
    }
}

#[async_trait]
impl ScalingExecutor for types::OmniScalar {
    async fn execute_scale_action(
        &self,
        action: &ScaleAction,
        _context: &CallbackContext,
    ) -> LighthouseResult<bool> {
        match action.direction {
            ScaleDirection::Up => {
                self.scale_deployment(&action.resource_id, action.scale_factor).await
                    .map_err(|e| lighthouse::LighthouseError::unexpected(e.to_string()))?;
                println!("✅ Scaled up {}: {}", action.resource_id, action.reason);
            }
            ScaleDirection::Down => {
                self.scale_deployment(&action.resource_id, action.scale_factor).await
                    .map_err(|e| lighthouse::LighthouseError::unexpected(e.to_string()))?;
                println!("📉 Scaled down {}: {}", action.resource_id, action.reason);
            }
            ScaleDirection::Maintain => {
                println!("➡️ Maintaining {}", action.resource_id);
            }
        }
        Ok(true)
    }

    async fn is_safe_to_scale(
        &self,
        _action: &ScaleAction,
        _context: &CallbackContext,
    ) -> LighthouseResult<bool> {
        let has_capacity = self.check_cluster_capacity().await
            .map_err(|e| lighthouse::LighthouseError::unexpected(e.to_string()))?;
        
        Ok(has_capacity && !self.is_maintenance_window())
    }

    async fn get_current_capacity(
        &self,
        _resource_id: &str,
        _context: &CallbackContext,
    ) -> LighthouseResult<Option<u32>> {
        // Return current instance count or None if unknown
        Ok(Some(3)) // Example: currently have 3 instances
    }
}

// The main initialization function that returns everything you need
pub fn init() -> (LighthouseEngine, LighthouseHandle) {
    log::info!("{}", "Initializing Lighthouse autoscaler...".yellow());

    // 1. Create configuration with actual values
    let config = LighthouseConfig::builder()
        .evaluation_interval(30) // Check every 30 seconds
        .enable_logging(true)
        .add_resource_config("omni_application", ResourceConfig {
            resource_type: "omni_application".to_string(),
            policies: vec![
                // CPU-based scaling policy
                ScalingPolicy {
                    name: "cpu-scaling".to_string(),
                    thresholds: vec![ScalingThreshold {
                        metric_name: "cpu_usage".to_string(),
                        scale_up_threshold: 80.0,   // Scale up when CPU > 80%
                        scale_down_threshold: 30.0, // Scale down when CPU < 30%
                        scale_factor: 1.5,          // Scale by 50%
                        cooldown_seconds: 300,      // Wait 5 minutes between actions
                    }],
                    min_capacity: Some(1),
                    max_capacity: Some(10),
                    enabled: true,
                },
                // Memory-based scaling policy
                ScalingPolicy {
                    name: "memory-scaling".to_string(),
                    thresholds: vec![ScalingThreshold {
                        metric_name: "memory_usage".to_string(),
                        scale_up_threshold: 85.0,
                        scale_down_threshold: 40.0,
                        scale_factor: 1.3,
                        cooldown_seconds: 600, // 10 minute cooldown for memory
                    }],
                    min_capacity: Some(1),
                    max_capacity: Some(10),
                    enabled: true,
                },
            ],
            default_policy: Some("cpu-scaling".to_string()),
            settings: HashMap::new(),
        })
        .global_setting("environment", "production")
        .global_setting("cluster_name", "omni-cluster")
        .build();

    // 2. Create callback implementations
    let metrics_provider = Arc::new(types::OmniMetricsProvider::new());
    let scaling_executor = Arc::new(types::OmniScalar::new());

    let callbacks = LighthouseCallbacks::new(metrics_provider, scaling_executor);

    // 3. Create engine and handle
    let engine = LighthouseEngine::new(config, callbacks);
    let handle = engine.handle();

    log::info!("{}", "✅ Lighthouse autoscaler initialized successfully!".green());

    (engine, handle)
}

// Helper function to start the engine in the background
pub async fn start_autoscaler() -> LighthouseHandle {
    let (engine, handle) = init();

    // Start the engine in a background task
    tokio::spawn(async move {
        if let Err(e) = engine.start().await {
            log::error!("Lighthouse engine failed: {}", e);
        }
    });

    // Give the engine a moment to start
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    log::info!("{}", "🚀 Lighthouse autoscaler started!".green());
    handle
}

// Usage example:
#[cfg(test)]
mod example_usage {
    use super::*;

    #[tokio::test]
    async fn example_usage() {
        // Option 1: Manual control
        let (engine, handle) = init();
        
        // Start engine in background
        tokio::spawn(async move {
            engine.start().await.unwrap();
        });

        // Send some metrics
        handle.update_metrics(utils::single_metric(
            "my-app-instance-1",
            "omni_application", 
            "cpu_usage",
            85.0 // High CPU should trigger scale-up
        )).await.unwrap();

        // Option 2: Simple start (recommended)
        let handle = start_autoscaler().await;
        
        // The engine is now running and will automatically scale based on metrics
        // You can still send metrics manually:
        handle.update_metrics(utils::multi_metrics(
            "my-app-instance-1",
            "omni_application",
            vec![
                ("cpu_usage", 85.0),
                ("memory_usage", 70.0),
            ]
        )).await.unwrap();

        // Get current status
        let status = handle.get_status().await.unwrap();
        println!("Engine status: {:?}", status);

        // Shutdown when done
        handle.shutdown().await.unwrap();
    }
}