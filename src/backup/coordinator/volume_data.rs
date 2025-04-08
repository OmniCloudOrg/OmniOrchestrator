// backup/coordinator/volume_data.rs
//
// Handles volume data backup

use super::types::BackupJobStatus;
use crate::db::v1::tables::Backup;
use crate::network::client::NetworkClient;
use crate::network::discovery::EnvironmentNode;

use anyhow::{anyhow, Result};
use chrono::Utc;
use log::{debug, error, info, warn};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

/// Backup Volume Data
pub async fn backup_volume_data(
    backup: &Backup,
    node: &EnvironmentNode,
    backup_dir: &Path,
    network_client: Arc<NetworkClient>,
    backup_jobs: Arc<Mutex<Vec<BackupJobStatus>>>,
    status_tx: mpsc::Sender<BackupJobStatus>,
) -> Result<()> {
    let component_type = "volume-data";
    let node_id = node.id.clone();

    // Get included applications for volume data, if specified
    let included_apps = if let Some(apps_value) = &backup.included_apps {
        if let Ok(apps_json) = serde_json::from_str::<Value>(apps_value) {
            if let Some(apps_array) = apps_json.as_array() {
                let apps: Vec<String> = apps_array
                    .iter()
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

    // Get volume info from the storage node
    let volume_info_result = network_client.get_node_volumes(&node_id).await;

    let volumes = match volume_info_result {
        Ok(info) => {
            let volume_data: Value = serde_json::from_str(&info)?;
            if let Some(volumes_array) = volume_data["volumes"].as_array() {
                // Filter volumes by application if needed
                let volumes: Vec<Value> = if let Some(apps) = &included_apps {
                    volumes_array
                        .iter()
                        .filter(|v| {
                            if let Some(app) = v["application"].as_str() {
                                apps.contains(&app.to_string())
                            } else {
                                false
                            }
                        })
                        .cloned()
                        .collect()
                } else {
                    volumes_array.iter().cloned().collect()
                };

                volumes
            } else {
                Vec::new()
            }
        }
        Err(e) => {
            // Update job status with error
            let job = BackupJobStatus {
                node_id: node_id.clone(),
                component_type: component_type.to_string(),
                status: "failed".to_string(),
                progress: 0.0,
                iso_path: None,
                error: Some(format!("Failed to get volume information: {}", e)),
                started_at: Utc::now(),
                completed_at: Some(Utc::now()),
                size_bytes: 0,
            };

            // Send status update
            status_tx.send(job).await?;
            return Err(anyhow!("Failed to get volume information: {}", e));
        }
    };

    // Group volumes by application
    let mut volumes_by_app: HashMap<String, Vec<Value>> = HashMap::new();

    for volume in volumes {
        let app_name = volume["application"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();

        volumes_by_app
            .entry(app_name)
            .or_insert_with(Vec::new)
            .push(volume);
    }

    // Backup each application's volumes
    for (app_name, app_volumes) in volumes_by_app {
        // Register the backup job for this application's volumes
        let job_id = format!("{}-{}", node_id, app_name);
        let component_subtype = format!("{}-{}", component_type, app_name);

        let job = BackupJobStatus {
            node_id: job_id.clone(),
            component_type: component_subtype.clone(),
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

        // Create backup configuration for this app's volumes
        let volume_ids: Vec<String> = app_volumes
            .iter()
            .filter_map(|v| v["id"].as_str().map(|s| s.to_string()))
            .collect();

        let backup_config = json!({
            "backup_id": backup.id,
            "backup_name": backup.name,
            "component_type": component_type,
            "application": app_name,
            "volume_ids": volume_ids,
            "encryption_method": backup.encryption_method,
            "encryption_key_id": backup.encryption_key_id,
            "temp_dir": backup_dir.join("temp").to_string_lossy().to_string(),
        });

        // Initiate backup on the node
        let result = network_client
            .request_component_backup(
                &node_id,
                &format!("{}-{}", component_type, app_name),
                &backup_config.to_string(),
            )
            .await;

        match result {
            Ok(response) => {
                // Parse response to get ISO path
                let response_data: Value = serde_json::from_str(&response)?;
                let iso_path = response_data["iso_path"]
                    .as_str()
                    .ok_or_else(|| anyhow!("Invalid response: missing iso_path"))?;
                let size_bytes = response_data["size_bytes"]
                    .as_u64()
                    .ok_or_else(|| anyhow!("Invalid response: missing size_bytes"))?;

                // Copy the ISO to the backup directory
                let dest_path = backup_dir.join("isos").join(format!(
                    "Volume-Data-ISO-{}-{}.iso",
                    app_name.replace(" ", "-"),
                    backup.id
                ));
                let copy_result = network_client
                    .copy_file_from_node(&node_id, iso_path, &dest_path.to_string_lossy())
                    .await;

                match copy_result {
                    Ok(_) => {
                        // Update job status
                        let job = BackupJobStatus {
                            node_id: job_id.clone(),
                            component_type: component_subtype.clone(),
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
                    }
                    Err(e) => {
                        // Update job status with error
                        let job = BackupJobStatus {
                            node_id: job_id.clone(),
                            component_type: component_subtype.clone(),
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
            }
            Err(e) => {
                // Update job status with error
                let job = BackupJobStatus {
                    node_id: job_id.clone(),
                    component_type: component_subtype.clone(),
                    status: "failed".to_string(),
                    progress: 0.0,
                    iso_path: None,
                    error: Some(format!("Failed to backup volume data: {}", e)),
                    started_at: Utc::now(),
                    completed_at: Some(Utc::now()),
                    size_bytes: 0,
                };

                // Send status update
                status_tx.send(job).await?;
            }
        }
    }

    Ok(())
}
