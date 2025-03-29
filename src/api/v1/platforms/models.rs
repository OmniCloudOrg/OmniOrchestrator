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
// TODO: Move to LibOmni
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SshHost {
    pub name: String,
    pub hostname: String,
    pub username: String,
    pub port: u16,
    pub identity_file: Option<String>,
    pub is_bastion: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CloudConfig {
    pub company_name: String,
    pub admin_name: String,
    pub cloud_name: String,
    pub region: String,
    pub ssh_hosts: Vec<SshHost>,
    pub enable_monitoring: bool,
    pub enable_backups: bool,
    pub backup_retention_days: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub status: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

// Track deployment status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostDeploymentStatus {
    pub host: String,
    pub status: String,
    pub services: Vec<ServiceStatus>,
    pub current_step: String,
    pub progress: u8,
    pub error: Option<String>,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub name: String,
    pub status: String,
    pub uptime: Option<String>,
    pub cpu: Option<String>,
    pub memory: Option<String>,
}
