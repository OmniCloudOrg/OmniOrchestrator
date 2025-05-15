use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get};

use super::super::state::GLOBAL_DEPLOYMENT_STATUS;
use super::super::models::{CloudConfig, ApiResponse, HostDeploymentStatus, ServiceStatus, SshHost};

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