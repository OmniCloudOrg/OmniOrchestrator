use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use std::sync::RwLock;
use rocket::{post, get};
use serde_json::Value;
use std::sync::Arc;
use std::collections::HashMap;
use std::time::Duration;
use lazy_static::lazy_static;

use super::super::state::GLOBAL_CONFIGS;
use super::utils::{update_host_status, update_host_services};
use super::super::models::{CloudConfig, ApiResponse, HostDeploymentStatus, ServiceStatus, SshHost};

// Specific endpoint for bootstrapping a host
#[post("/platforms/<cloud_name>/hosts/<host_name>/bootstrap")]
pub fn bootstrap_host(cloud_name: String, host_name: String) -> Json<ApiResponse> {
    let host_to_bootstrap = {
        let configs = GLOBAL_CONFIGS.read().unwrap();
        
        if let Some(config) = configs.get(&cloud_name) {
            config.ssh_hosts.iter().find(|h| h.name == host_name).cloned()
        } else {
            None
        }
    };

    if let Some(host) = host_to_bootstrap {
        // Spawn async task
        let cloud_name_clone = cloud_name.clone();
        let host_clone = host.clone();
        
        tokio::spawn(async move {
            simulate_host_bootstrap(cloud_name_clone, host_clone).await;
        });
        
        return Json(ApiResponse {
            status: "success".to_string(),
            message: format!("Bootstrap started for host {}", host_name),
            data: None,
        });
    }

    Json(ApiResponse {
        status: "error".to_string(),
        message: format!("Host {} not found", host_name),
        data: None,
    })
}

async fn simulate_host_bootstrap(cloud_name: String, host: SshHost) {
    let host_name = host.name.clone();
    let is_bastion = host.is_bastion;
    
    // Update host status to in-progress
    update_host_status(&cloud_name, &host_name, "in_progress", "Establishing SSH connection", 0, None, false);
    tokio::time::sleep(Duration::from_secs(2)).await;
    
    update_host_status(&cloud_name, &host_name, "in_progress", "Verifying system requirements", 20, None, false);
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    update_host_status(&cloud_name, &host_name, "in_progress", "Installing OmniOrchestrator binaries", 40, None, false);
    tokio::time::sleep(Duration::from_secs(5)).await;
    
    update_host_status(&cloud_name, &host_name, "in_progress", "Configuring system services", 60, None, false);
    tokio::time::sleep(Duration::from_secs(4)).await;
    
    update_host_status(&cloud_name, &host_name, "in_progress", "Applying security hardening", 80, None, false);
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    // Host-specific configuration
    if is_bastion {
        update_host_status(&cloud_name, &host_name, "in_progress", "Configuring bastion-specific security", 90, None, false);
        tokio::time::sleep(Duration::from_secs(2)).await;
        
        // Add services for bastion hosts
        let services = vec![
            ServiceStatus {
                name: "orchestrator-core".to_string(),
                status: "Running".to_string(),
                uptime: Some("0m".to_string()),
                cpu: Some("12%".to_string()),
                memory: Some("256MB".to_string()),
            },
            ServiceStatus {
                name: "network-agent".to_string(),
                status: "Running".to_string(),
                uptime: Some("0m".to_string()),
                cpu: Some("5%".to_string()),
                memory: Some("128MB".to_string()),
            },
            ServiceStatus {
                name: "api-gateway".to_string(),
                status: "Running".to_string(),
                uptime: Some("0m".to_string()),
                cpu: Some("18%".to_string()),
                memory: Some("512MB".to_string()),
            },
            ServiceStatus {
                name: "auth-service".to_string(),
                status: "Running".to_string(),
                uptime: Some("0m".to_string()),
                cpu: Some("10%".to_string()),
                memory: Some("384MB".to_string()),
            },
        ];
        
        update_host_services(&cloud_name, &host_name, services);
    } else {
        update_host_status(&cloud_name, &host_name, "in_progress", "Configuring worker-specific services", 90, None, false);
        tokio::time::sleep(Duration::from_secs(2)).await;
        
        // Add services for worker hosts
        let services = vec![
            ServiceStatus {
                name: "orchestrator-core".to_string(),
                status: "Running".to_string(),
                uptime: Some("0m".to_string()),
                cpu: Some("12%".to_string()),
                memory: Some("256MB".to_string()),
            },
            ServiceStatus {
                name: "network-agent".to_string(),
                status: "Running".to_string(),
                uptime: Some("0m".to_string()),
                cpu: Some("5%".to_string()),
                memory: Some("128MB".to_string()),
            },
            ServiceStatus {
                name: "container-runtime".to_string(),
                status: "Running".to_string(),
                uptime: Some("0m".to_string()),
                cpu: Some("22%".to_string()),
                memory: Some("768MB".to_string()),
            },
        ];
        
        update_host_services(&cloud_name, &host_name, services);
    }
    
    // Mark host as completed
    update_host_status(&cloud_name, &host_name, "completed", "Bootstrap completed", 100, None, true);
}

