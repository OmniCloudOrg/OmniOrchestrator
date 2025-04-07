// backup/coordinator/app_definitions.rs
//
// Handles application definitions backup

use crate::db::v1::tables::Backup;
use crate::network::discovery::EnvironmentNode;
use crate::network::client::NetworkClient;
use super::types::BackupJobStatus;

use std::path::Path;
use std::sync::{Arc, Mutex};
use log::{info, error};
use tokio::sync::mpsc;
use anyhow::{Result, anyhow};
use serde_json::{json, Value};
use chrono::Utc;

/// Backup Application Definitions
pub async fn backup_app_definitions(
    backup: &Backup,
    node: &EnvironmentNode,
    backup_dir: &Path,
    network_client: Arc<NetworkClient>,
    backup_jobs: Arc<Mutex<Vec<BackupJobStatus>>>,
    status_tx: mpsc::Sender<BackupJobStatus>,
) -> Result<()> {
    let component_type = "app-definitions";
    let node_id = node.id.clone();
    
    // Register the backup job
    let job = BackupJobStatus {
        node_id: node_id.clone(),
        component_type: component_type.to_string(),
        status: "starting".to_string(),
        progress: 0.0,
        iso_path: None,
        error: None,
        started_at: Utc::now(),
        completed_at: None,
        size_bytes: 0,
    };
    
    {
        let mut jobs = backup_jobs.lock().unwrap();
        jobs.push(job.clone());
    }
    
    // Send status update
    status_tx.send(job).await?;
    
    // Get included apps, if specified
    let included_apps = if let Some(apps_value) = &backup.included_apps {
        if let Ok(json_value) = serde_json::from_str::<Value>(apps_value) {
            if let Some(apps_array) = json_value.as_array() {
                let apps: Vec<String> = apps_array.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
                if !apps.is_empty() {
                    Some(apps)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };
    
    // Initiate backup on the node
    let backup_config = json!({
        "backup_id": backup.id,
        "backup_name": backup.name,
        "component_type": component_type,
        "included_apps": included_apps,
        "encryption_method": backup.encryption_method,
        "encryption_key_id": backup.encryption_key_id,
        "temp_dir": backup_dir.join("temp").to_string_lossy().to_string(),
    });
    
    let result = network_client.request_component_backup(
        &node_id,
        component_type,
        backup_config.to_string().as_str(),
    ).await;
    
    match result {
        Ok(response) => {
            // Parse response to get ISO path
            let response_data: Value = serde_json::from_str(&response)?;
            let iso_path = response_data["iso_path"].as_str()
                .ok_or_else(|| anyhow!("Invalid response: missing iso_path"))?;
            let size_bytes = response_data["size_bytes"].as_u64()
                .ok_or_else(|| anyhow!("Invalid response: missing size_bytes"))?;
            
            // Copy the ISO to the backup directory
            let dest_path = backup_dir.join("isos").join(format!("Application-Definition-ISO-{}.iso", backup.id));
            let copy_result = network_client.copy_file_from_node(
                &node_id,
                iso_path,
                &dest_path.to_string_lossy(),
            ).await;
            
            match copy_result {
                Ok(_) => {
                    // Update job status
                    let job = BackupJobStatus {
                        node_id: node_id.clone(),
                        component_type: component_type.to_string(),
                        status: "completed".to_string(),
                        progress: 100.0,
                        iso_path: Some(dest_path.to_string_lossy().to_string()),
                        error: None,
                        started_at: Utc::now(),
                        completed_at: Some(Utc::now()),
                        size_bytes,
                    };
                    
                    // Send status update
                    status_tx.send(job).await?;
                },
                Err(e) => {
                    // Update job status with error
                    let job = BackupJobStatus {
                        node_id: node_id.clone(),
                        component_type: component_type.to_string(),
                        status: "failed".to_string(),
                        progress: 0.0,
                        iso_path: None,
                        error: Some(format!("Failed to copy ISO file: {}", e)),
                        started_at: Utc::now(),
                        completed_at: Some(Utc::now()),
                        size_bytes: 0,
                    };
                    
                    // Send status update
                    status_tx.send(job).await?;
                }
            }
        },
        Err(e) => {
            // Update job status with error
            let job = BackupJobStatus {
                node_id: node_id.clone(),
                component_type: component_type.to_string(),
                status: "failed".to_string(),
                progress: 0.0,
                iso_path: None,
                error: Some(format!("Failed to backup application definitions: {}", e)),
                started_at: Utc::now(),
                completed_at: Some(Utc::now()),
                size_bytes: 0,
            };
            
            // Send status update
            status_tx.send(job).await?;
        }
    }
    
    Ok(())
}