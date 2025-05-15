use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use std::sync::RwLock;
use rocket::{post, get};
use serde_json::Value;
use std::sync::Arc;
use std::collections::HashMap;
use std::time::Duration;
use lazy_static::lazy_static;

use super::super::state::{GLOBAL_CONFIGS, GLOBAL_DEPLOYMENT_STATUS};
use super::utils::{update_host_status, update_host_services};
use super::super::models::{CloudConfig, ApiResponse, HostDeploymentStatus, ServiceStatus, SshHost};

#[post("/platforms/<cloud_name>/monitoring/setup")]
pub fn setup_monitoring(cloud_name: String) -> Json<ApiResponse> {
    // Extract data we need
    let monitoring_info = {
        let configs = GLOBAL_CONFIGS.read().unwrap();
        if let Some(config) = configs.get(&cloud_name) {
            Some((true, config.enable_monitoring))
        } else {
            None
        }
    };
    
    match monitoring_info {
        Some((_, true)) => {
            // Spawn async task
            let cloud_name_clone = cloud_name.clone();
            
            tokio::spawn(async move {
                simulate_monitoring_setup(cloud_name_clone).await;
            });
            
            Json(ApiResponse {
                status: "success".to_string(),
                message: "Monitoring setup started".to_string(),
                data: None,
            })
        },
        Some((_, false)) => {
            // Cloud exists but monitoring is disabled
            Json(ApiResponse {
                status: "error".to_string(),
                message: "Monitoring is disabled for this cloud".to_string(),
                data: None,
            })
        },
        None => {
            // Cloud doesn't exist
            Json(ApiResponse {
                status: "error".to_string(),
                message: format!("Cloud {} not found", cloud_name),
                data: None,
            })
        }
    }
}

async fn simulate_monitoring_setup(cloud_name: String) {
    // Get hosts first and drop the lock before any await points
    let hosts = {
        let configs = GLOBAL_CONFIGS.read().unwrap();
        if let Some(config) = configs.get(&cloud_name) {
            config.ssh_hosts.clone()
        } else {
            return;
        }
    };
    
    for host in &hosts {
        update_host_status(&cloud_name, &host.name, "in_progress", "Deploying monitoring stack", 0, None, false);
        
        // Add monitoring service to each host
        {
            let mut deployment_status = GLOBAL_DEPLOYMENT_STATUS.write().unwrap();
            if let Some(host_statuses) = deployment_status.get_mut(&cloud_name) {
                if let Some(host_status) = host_statuses.iter_mut().find(|h| h.host == host.name) {
                    host_status.services.push(ServiceStatus {
                        name: "metrics-collector".to_string(),
                        status: "Starting".to_string(),
                        uptime: None,
                        cpu: None,
                        memory: None,
                    });
                }
            }
        }
    }
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    for host in &hosts {
        update_host_status(&cloud_name, &host.name, "in_progress", "Configuring metrics collection", 33, None, false);
    }
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    for host in &hosts {
        update_host_status(&cloud_name, &host.name, "in_progress", "Setting up dashboards", 66, None, false);
    }
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    for host in &hosts {
        update_host_status(&cloud_name, &host.name, "completed", "Monitoring services deployed", 100, None, true);
        
        // Update monitoring service status
        {
            let mut deployment_status = GLOBAL_DEPLOYMENT_STATUS.write().unwrap();
            if let Some(host_statuses) = deployment_status.get_mut(&cloud_name) {
                if let Some(host_status) = host_statuses.iter_mut().find(|h| h.host == host.name) {
                    if let Some(service) = host_status.services.iter_mut().find(|s| s.name == "metrics-collector") {
                        service.status = "Running".to_string();
                        service.uptime = Some("0m".to_string());
                        service.cpu = Some("8%".to_string());
                        service.memory = Some("192MB".to_string());
                    }
                }
            }
        }
    }
}