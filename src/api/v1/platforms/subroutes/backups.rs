use crate::api::v1::platforms::state::{GLOBAL_CONFIGS,GLOBAL_DEPLOYMENT_STATUS};
use rocket::serde::json::Json;
use crate::api::v1::models::ApiResponse;
use super::utils::update_host_status;
use std::time::Duration;
use super::super::models::ServiceStatus;
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
