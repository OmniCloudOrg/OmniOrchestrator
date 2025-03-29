use rocket::serde::json::Json;
use std::time::Duration;
use super::super::models::ServiceStatus;
use crate::api::v1::platforms::state::{GLOBAL_CONFIGS,GLOBAL_DEPLOYMENT_STATUS};

pub fn update_host_status(
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

pub fn update_host_services(
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