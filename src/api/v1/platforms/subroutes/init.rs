use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use std::sync::RwLock;
use rocket::{post, get};
use serde_json::Value;
use std::sync::Arc;
use std::collections::HashMap;
use std::time::Duration;
use lazy_static::lazy_static;

use super::utils::{update_host_status, update_host_services};
use super::super::state::GLOBAL_CONFIGS;
use super::super::state::GLOBAL_DEPLOYMENT_STATUS;
use super::super::models::{CloudConfig, ApiResponse, HostDeploymentStatus, ServiceStatus, SshHost};

// Main API endpoint to initialize the platform
#[post("/platforms/init", data = "<config>")]
pub fn init_platform(config: Json<CloudConfig>) -> Json<ApiResponse> {
    let cloud_name = config.cloud_name.clone();
    let config_data = config.into_inner();
    
    // Store the configuration in global state
    {
        let mut configs = GLOBAL_CONFIGS.write().unwrap();
        configs.insert(cloud_name.clone(), config_data.clone());
    }
    
    // Initialize deployment status for each host
    let mut host_statuses = Vec::new();
    for host in &config_data.ssh_hosts {
        host_statuses.push(HostDeploymentStatus {
            host: host.name.clone(),
            status: "pending".to_string(),
            services: Vec::new(),
            current_step: "Waiting to start".to_string(),
            progress: 0,
            error: None,
            completed: false,
        });
    }
    
    {
        let mut deployment_status = GLOBAL_DEPLOYMENT_STATUS.write().unwrap();
        deployment_status.insert(cloud_name.clone(), host_statuses);
    }
    
    // Spawn async task
    let cloud_name_clone = cloud_name.clone();
    let config_data_clone = config_data.clone();
    
    tokio::spawn(async move {
        deploy_platform(cloud_name_clone, config_data_clone).await;
    });
    
    Json(ApiResponse {
        status: "success".to_string(),
        message: "Platform initialization started. Check status endpoint for progress.".to_string(),
        data: None,
    })
}

/// This is the function that initializes the platform itself. This should
/// ONLY EVER BE CALLED on initial OmniOrchestrator deployment. It will
/// bootstrap the bastion hosts, then the worker hosts, and finally configure
/// networking before transferring control of the cluster to the real Orchestrator.
async fn deploy_platform(cloud_name: String, config: CloudConfig) {
    // Get the data we need from the config
    let bastion_hosts: Vec<_> = config.ssh_hosts.iter()
        .filter(|h| h.is_bastion)
        .cloned()
        .collect();
        
    let worker_hosts: Vec<_> = config.ssh_hosts.iter()
        .filter(|h| !h.is_bastion)
        .cloned()
        .collect();
    
    // Store configuration info for later
    let enable_monitoring = config.enable_monitoring;
    let enable_backups = config.enable_backups;
    let backup_retention_days = config.backup_retention_days;
    
    // First bootstrap bastion hosts
    for host in bastion_hosts {
        simulate_host_bootstrap(cloud_name.clone(), host).await;
    }
    
    // Then bootstrap worker hosts
    for host in worker_hosts {
        simulate_host_bootstrap(cloud_name.clone(), host).await;
    }
    
    // Configure networking
    simulate_network_configuration(cloud_name.clone()).await;
    
    // Setup monitoring if enabled
    if enable_monitoring {
        simulate_monitoring_setup(cloud_name.clone()).await;
    }
    
    // Setup backups if enabled
    if enable_backups {
        simulate_backup_setup(cloud_name.clone(), backup_retention_days).await;
    }
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

async fn simulate_network_configuration(cloud_name: String) {
    // Get the hosts we need first
    let hosts = {
        let configs = GLOBAL_CONFIGS.read().unwrap();
        if let Some(config) = configs.get(&cloud_name) {
            config.ssh_hosts.clone()
        } else {
            return;
        }
    };
    
    // Now we can use .await safely with the cloned data
    for host in &hosts {
        update_host_status(&cloud_name, &host.name, "in_progress", "Establishing secure tunnels", 0, None, false);
    }
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    for host in &hosts {
        update_host_status(&cloud_name, &host.name, "in_progress", "Configuring service discovery", 25, None, false);
    }
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    for host in &hosts {
        update_host_status(&cloud_name, &host.name, "in_progress", "Setting up load balancing", 50, None, false);
    }
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    for host in &hosts {
        update_host_status(&cloud_name, &host.name, "in_progress", "Finalizing network configuration", 75, None, false);
    }
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    for host in &hosts {
        update_host_status(&cloud_name, &host.name, "completed", "Network configuration complete", 100, None, true);
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


async fn simulate_backup_setup(cloud_name: String, retention_days: u32) {
    // Get bastion hosts first
    let bastion_hosts = {
        let configs = GLOBAL_CONFIGS.read().unwrap();
        if let Some(config) = configs.get(&cloud_name) {
            config.ssh_hosts.iter()
                .filter(|h| h.is_bastion)
                .cloned()
                .collect::<Vec<_>>()
        } else {
            return;
        }
    };
    
    // Only add backup service to bastion hosts
    for host in &bastion_hosts {
        update_host_status(&cloud_name, &host.name, "in_progress", "Setting up backup system", 0, None, false);
        
        // Add backup service to the bastion host
        {
            let mut deployment_status = GLOBAL_DEPLOYMENT_STATUS.write().unwrap();
            if let Some(host_statuses) = deployment_status.get_mut(&cloud_name) {
                if let Some(host_status) = host_statuses.iter_mut().find(|h| h.host == host.name) {
                    host_status.services.push(ServiceStatus {
                        name: "backup-manager".to_string(),
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
    
    for host in &bastion_hosts {
        update_host_status(&cloud_name, &host.name, "in_progress", "Configuring backup schedules", 33, None, false);
    }
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    for host in &bastion_hosts {
        update_host_status(&cloud_name, &host.name, "in_progress", &format!("Setting {} day retention policy", retention_days), 66, None, false);
    }
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    for host in &bastion_hosts {
        update_host_status(&cloud_name, &host.name, "completed", "Backup services configured", 100, None, true);
        
        // Update backup service status
        {
            let mut deployment_status = GLOBAL_DEPLOYMENT_STATUS.write().unwrap();
            if let Some(host_statuses) = deployment_status.get_mut(&cloud_name) {
                if let Some(host_status) = host_statuses.iter_mut().find(|h| h.host == host.name) {
                    if let Some(service) = host_status.services.iter_mut().find(|s| s.name == "backup-manager") {
                        service.status = "Running".to_string();
                        service.uptime = Some("0m".to_string());
                        service.cpu = Some("6%".to_string());
                        service.memory = Some("256MB".to_string());
                    }
                }
            }
        }
    }
}

