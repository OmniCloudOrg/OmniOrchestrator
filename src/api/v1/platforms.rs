/// All routes related to the configuration and setup of an omni platform.
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use std::sync::RwLock;
use rocket::{post, get};
use serde_json::Value;
use std::sync::Arc;
use std::collections::HashMap;
use std::time::Duration;
use lazy_static::lazy_static;

// Types that mirror the client-side structs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SshHost {
    name: String,
    hostname: String,
    username: String,
    port: u16,
    identity_file: Option<String>,
    is_bastion: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CloudConfig {
    company_name: String,
    admin_name: String,
    cloud_name: String,
    region: String,
    ssh_hosts: Vec<SshHost>,
    enable_monitoring: bool,
    enable_backups: bool,
    backup_retention_days: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    status: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

// Track deployment status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostDeploymentStatus {
    host: String,
    status: String,
    services: Vec<ServiceStatus>,
    current_step: String,
    progress: u8,
    error: Option<String>,
    completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    name: String,
    status: String,
    uptime: Option<String>,
    cpu: Option<String>,
    memory: Option<String>,
}

// CRITICAL CHANGE: Define the application state as global statics
// This completely bypasses Rocket's state management system
lazy_static! {
    static ref GLOBAL_CONFIGS: Arc<RwLock<HashMap<String, CloudConfig>>> = 
        Arc::new(RwLock::new(HashMap::new()));
    static ref GLOBAL_DEPLOYMENT_STATUS: Arc<RwLock<HashMap<String, Vec<HostDeploymentStatus>>>> = 
        Arc::new(RwLock::new(HashMap::new()));
}

// CRITICAL CHANGE: Removed AppState struct entirely and don't use Rocket's State

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

// Check deployment status
#[get("/platforms/<cloud_name>/status")]
pub fn check_platform_status(cloud_name: String) -> Json<ApiResponse> {
    let host_status = {
        let deployment_status = GLOBAL_DEPLOYMENT_STATUS.read().unwrap();
        deployment_status.get(&cloud_name).cloned()
    };
    
    if let Some(host_status) = host_status {
        let overall_progress = host_status.iter()
            .map(|h| h.progress as u32)
            .sum::<u32>() / host_status.len() as u32;
            
        let all_completed = host_status.iter().all(|h| h.completed);
        let status = if all_completed { "completed" } else { "in_progress" };
        
        return Json(ApiResponse {
            status: status.to_string(),
            message: format!("Platform deployment is {}% complete", overall_progress),
            data: Some(serde_json::to_value(host_status).unwrap()),
        });
    }
    
    Json(ApiResponse {
        status: "error".to_string(),
        message: format!("No deployment found for cloud name: {}", cloud_name),
        data: None,
    })
}

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

// Configure networking between hosts
#[post("/platforms/<cloud_name>/network/configure")]
pub fn configure_network(cloud_name: String) -> Json<ApiResponse> {
    let has_cloud = {
        let configs = GLOBAL_CONFIGS.read().unwrap();
        configs.contains_key(&cloud_name)
    };
    
    if has_cloud {
        // Spawn async task
        let cloud_name_clone = cloud_name.clone();
        
        tokio::spawn(async move {
            simulate_network_configuration(cloud_name_clone).await;
        });
        
        return Json(ApiResponse {
            status: "success".to_string(),
            message: "Network configuration started".to_string(),
            data: None,
        });
    }
    
    Json(ApiResponse {
        status: "error".to_string(),
        message: format!("Cloud {} not found", cloud_name),
        data: None,
    })
}

// Set up monitoring services
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

// Configure backup services
#[post("/platforms/<cloud_name>/backups/setup")]
pub fn setup_backups(cloud_name: String) -> Json<ApiResponse> {
    let backup_config = {
        let configs = GLOBAL_CONFIGS.read().unwrap();
        
        if let Some(config) = configs.get(&cloud_name) {
            if config.enable_backups {
                Some((true, config.backup_retention_days))
            } else {
                Some((false, 0))
            }
        } else {
            None
        }
    };
    
    match backup_config {
        Some((true, retention_days)) => {
            // Spawn async task
            let cloud_name_clone = cloud_name.clone();
            let retention_days_clone = retention_days;
            
            tokio::spawn(async move {
                simulate_backup_setup(cloud_name_clone, retention_days_clone).await;
            });
            
            return Json(ApiResponse {
                status: "success".to_string(),
                message: format!("Backup setup started with {} day retention", retention_days),
                data: None,
            })
        },
        Some((false, _)) => {
            return Json(ApiResponse {
                status: "error".to_string(),
                message: "Backups are disabled for this cloud".to_string(),
                data: None,
            });
        },
        None => {
            return Json(ApiResponse {
                status: "error".to_string(),
                message: format!("Cloud {} not found", cloud_name),
                data: None,
            });
        }
    }
}

// Get a list of services running on a host
#[get("/platforms/<cloud_name>/hosts/<host_name>/services")]
pub fn get_host_services(cloud_name: String, host_name: String) -> Json<ApiResponse> {
    let deployment_status = GLOBAL_DEPLOYMENT_STATUS.read().unwrap();
    
    if let Some(host_statuses) = deployment_status.get(&cloud_name) {
        if let Some(host_status) = host_statuses.iter().find(|h| h.host == host_name) {
            return Json(ApiResponse {
                status: "success".to_string(),
                message: format!("Services for host {}", host_name),
                data: Some(serde_json::to_value(&host_status.services).unwrap()),
            });
        }
    }
    
    Json(ApiResponse {
        status: "error".to_string(),
        message: format!("Host {} not found in cloud {}", host_name, cloud_name),
        data: None,
    })
}

// Get logs for a specific service
#[get("/platforms/<cloud_name>/hosts/<host_name>/services/<service_name>/logs")]
pub fn get_service_logs(cloud_name: String, host_name: String, service_name: String) -> Json<ApiResponse> {
    // In a real system, this would fetch actual logs
    // For now, we'll return mock log data
    let logs = vec![
        "2025-03-27 12:05:32 [INFO] Service started successfully",
        "2025-03-27 12:05:33 [INFO] Connecting to database",
        "2025-03-27 12:05:34 [INFO] Database connection established",
        "2025-03-27 12:10:45 [INFO] Processed 125 requests in the last 5 minutes",
        "2025-03-27 12:15:45 [INFO] Processed 143 requests in the last 5 minutes",
        "2025-03-27 12:20:45 [WARN] High memory usage detected: 78%",
        "2025-03-27 12:25:45 [INFO] Memory usage returned to normal: 62%",
    ];
    
    Json(ApiResponse {
        status: "success".to_string(),
        message: format!("Logs for service {} on host {}", service_name, host_name),
        data: Some(serde_json::to_value(logs).unwrap()),
    })
}

// Restart a service
#[post("/platforms/<cloud_name>/hosts/<host_name>/services/<service_name>/restart")]
pub fn restart_service(cloud_name: String, host_name: String, service_name: String) -> Json<ApiResponse> {
    let cloud_exists = {
        let configs = GLOBAL_CONFIGS.read().unwrap();
        configs.contains_key(&cloud_name)
    };
    
    if cloud_exists {
        // Spawn async task
        let cloud_name_clone = cloud_name.clone();
        let host_name_clone = host_name.clone();
        let service_name_clone = service_name.clone();
        
        tokio::spawn(async move {
            simulate_service_restart(
                cloud_name_clone,
                host_name_clone,
                service_name_clone,
            ).await;
        });
        
        return Json(ApiResponse {
            status: "success".to_string(),
            message: format!("Restarting service {} on host {}", service_name, host_name),
            data: None,
        });
    }
    
    Json(ApiResponse {
        status: "error".to_string(),
        message: format!("Cloud {} not found", cloud_name),
        data: None,
    })
}

// Async functions for backend operations
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

async fn simulate_service_restart(
    cloud_name: String,
    host_name: String,
    service_name: String,
) {
    // Update service status to restarting
    {
        let mut deployment_status = GLOBAL_DEPLOYMENT_STATUS.write().unwrap();
        if let Some(host_statuses) = deployment_status.get_mut(&cloud_name) {
            if let Some(host_status) = host_statuses.iter_mut().find(|h| h.host == host_name) {
                if let Some(service) = host_status.services.iter_mut().find(|s| s.name == service_name) {
                    service.status = "Restarting".to_string();
                }
            }
        }
    }
    
    // Simulate restart delay
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    // Update service status to running
    {
        let mut deployment_status = GLOBAL_DEPLOYMENT_STATUS.write().unwrap();
        if let Some(host_statuses) = deployment_status.get_mut(&cloud_name) {
            if let Some(host_status) = host_statuses.iter_mut().find(|h| h.host == host_name) {
                if let Some(service) = host_status.services.iter_mut().find(|s| s.name == service_name) {
                    service.status = "Running".to_string();
                    service.uptime = Some("0m".to_string());
                }
            }
        }
    }
}

// Helper functions using global state
fn update_host_status(
    cloud_name: &str,
    host_name: &str,
    status: &str,
    current_step: &str,
    progress: u8,
    error: Option<&str>,
    completed: bool,
) {
    let mut deployment_status = GLOBAL_DEPLOYMENT_STATUS.write().unwrap();
    
    if let Some(host_statuses) = deployment_status.get_mut(cloud_name) {
        if let Some(host_status) = host_statuses.iter_mut().find(|h| h.host == host_name) {
            host_status.status = status.to_string();
            host_status.current_step = current_step.to_string();
            host_status.progress = progress;
            host_status.error = error.map(|e| e.to_string());
            host_status.completed = completed;
        }
    }
}

fn update_host_services(
    cloud_name: &str,
    host_name: &str,
    services: Vec<ServiceStatus>,
) {
    let mut deployment_status = GLOBAL_DEPLOYMENT_STATUS.write().unwrap();
    
    if let Some(host_statuses) = deployment_status.get_mut(cloud_name) {
        if let Some(host_status) = host_statuses.iter_mut().find(|h| h.host == host_name) {
            host_status.services = services;
        }
    }
}