use rocket::serde::json::Json;
use crate::api::v1::models::ApiResponse;
use super::utils::update_host_status;
use std::time::Duration;
use super::super::models::ServiceStatus;

use crate::api::v1::platforms_new::state::GLOBAL_CONFIGS;



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