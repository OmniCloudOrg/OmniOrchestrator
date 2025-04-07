// backup/coordinator/coordinator.rs
//
// Main implementation of the backup coordinator

use crate::db::v1::tables::Backup;
use crate::network::discovery::{EnvironmentNode, NodeType};
use crate::network::client::NetworkClient;
use super::types::BackupJobStatus;
use super::manifest::create_backup_manifest;
use super::{
    backup_system_core,
    backup_director,
    backup_orchestrator,
    backup_network_config,
    backup_app_definitions,
    backup_volume_data,
};

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::Utc;
use log::{info, warn, error, debug};
use tokio::sync::mpsc;
use std::time::Duration;
use anyhow::{Result, Context, bail, anyhow};
use serde_json::{json, Value};

/// Coordinates backup operations across the OmniCloud environment
pub struct BackupCoordinator {
    network_client: Arc<NetworkClient>,
}

impl BackupCoordinator {
    /// Create a new BackupCoordinator instance
    pub fn new(network_client: Arc<NetworkClient>) -> Self {
        Self {
            network_client,
        }
    }

    /// Start the backup process, coordinating across nodes
    pub async fn start_backup(&self, backup: &mut Backup) -> Result<()> {
        info!("Starting backup process for: {}", backup.name);
        
        // Update backup status
        backup.status = "in_progress".to_string();
        
        // Initialize backup environment
        let backup_dir = backup.initialize_backup_environment()
            .context("Failed to initialize backup environment")?;
        
        // Discover all nodes in the environment
        let nodes = self.network_client.discover_environment(&backup.source_environment)
            .await
            .context("Failed to discover environment nodes")?;
        
        // Create a synchronized collection to track backup jobs
        let backup_jobs = Arc::new(Mutex::new(Vec::<BackupJobStatus>::new()));
        
        // Create a channel for job status updates
        let (tx, mut rx) = mpsc::channel(100);
        
        // Backup metadata collection
        let mut backup_metadata: HashMap<String, Value> = HashMap::new();
        backup_metadata.insert("total_nodes".to_string(), json!(nodes.len()));
        backup_metadata.insert("started_at".to_string(), json!(Utc::now().to_string()));
        
        // Launch backup jobs for each node based on type
        self.launch_backup_jobs(backup, &nodes, &backup_dir, Arc::clone(&backup_jobs), tx.clone()).await?;
        
        // Overall backup size counter
        let mut total_size_bytes: u64 = 0;
        
        // Process job status updates
        let jobs_clone = Arc::clone(&backup_jobs);
        tokio::spawn(async move {
            while let Some(status) = rx.recv().await {
                // Update job status in the collection
                let mut jobs = jobs_clone.lock().unwrap();
                if let Some(job) = jobs.iter_mut().find(|j| j.node_id == status.node_id && j.component_type == status.component_type) {
                    *job = status.clone();
                } else {
                    jobs.push(status.clone());
                }
                
                // Log progress
                if status.status == "completed" {
                    info!("Backup completed for {} on node {}", status.component_type, status.node_id);
                } else if status.status == "failed" {
                    error!("Backup failed for {} on node {}: {}", 
                          status.component_type, 
                          status.node_id, 
                          status.error.unwrap_or_else(|| "Unknown error".to_string()));
                }
            }
        });
        
        // Wait for all jobs to complete or fail
        loop {
            let jobs = backup_jobs.lock().unwrap();
            let total_jobs = jobs.len();
            let completed_jobs = jobs.iter().filter(|j| j.status == "completed" || j.status == "failed").count();
            
            if total_jobs > 0 && completed_jobs == total_jobs {
                // All jobs finished
                break;
            }
            
            // Wait a bit before checking again
            drop(jobs);
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
        
        // Process backup results
        let jobs = backup_jobs.lock().unwrap();
        let failed_jobs = jobs.iter().filter(|j| j.status == "failed").collect::<Vec<_>>();
        
        if !failed_jobs.is_empty() {
            // Some jobs failed
            let error_msg = format!("{} backup jobs failed. First error: {}", 
                failed_jobs.len(),
                failed_jobs[0].error.as_ref().unwrap_or(&"Unknown error".to_string()));
            
            backup.set_failed(Some(error_msg.clone())); // We clone here singe cloning in an error context is more acceptable
            return Err(anyhow!("Backup process failed: {}", error_msg));
        }
        
        // All jobs succeeded, create the final manifest
        let mut has_system_core = false;
        let mut has_directors = false;
        let mut has_orchestrators = false;
        let mut has_network_config = false;
        let mut has_app_definitions = false;
        let mut has_volume_data = false;
        
        // Process each completed job
        for job in jobs.iter() {
            if job.status != "completed" {
                continue;
            }
            
            // Update component flags
            match job.component_type.as_str() {
                "system-core" => has_system_core = true,
                "director" => has_directors = true,
                "orchestrator" => has_orchestrators = true,
                "network-config" => has_network_config = true,
                "app-definitions" => has_app_definitions = true,
                "volume-data" => has_volume_data = true,
                _ => {}
            }
            
            // Add job size to total
            total_size_bytes += job.size_bytes;
        }
        
        // Update backup metadata with ISO information
        let iso_info: Vec<HashMap<String, String>> = jobs.iter()
            .filter(|j| j.status == "completed" && j.iso_path.is_some())
            .map(|j| {
                let mut map = HashMap::new();
                map.insert("node_id".to_string(), j.node_id.clone());
                map.insert("component_type".to_string(), j.component_type.clone());
                map.insert("iso_path".to_string(), j.iso_path.clone().unwrap_or_default());
                map.insert("size_bytes".to_string(), j.size_bytes.to_string());
                map
            })
            .collect();
        
        backup_metadata.insert("iso_files".to_string(), json!(iso_info));
        backup_metadata.insert("completed_at".to_string(), json!(Utc::now().to_string()));
        backup_metadata.insert("total_size_bytes".to_string(), json!(total_size_bytes));
        
        // Update backup properties
        backup.has_system_core = has_system_core;
        backup.has_directors = has_directors;
        backup.has_orchestrators = has_orchestrators;
        backup.has_network_config = has_network_config;
        backup.has_app_definitions = has_app_definitions;
        backup.has_volume_data = has_volume_data;
        backup.set_size(total_size_bytes);
        backup.update_metadata(json!(backup_metadata));
        
        // Create final manifest file
        create_backup_manifest(backup, &backup_dir)?;
        
        // Mark backup as successful
        backup.set_success();
        
        info!("Backup completed successfully: {}", backup.name);
        Ok(())
    }
    
    /// Launch backup jobs for all nodes in the environment
    async fn launch_backup_jobs(
        &self,
        backup: &Backup,
        nodes: &[EnvironmentNode],
        backup_dir: &Path,
        backup_jobs: Arc<Mutex<Vec<BackupJobStatus>>>,
        status_tx: mpsc::Sender<BackupJobStatus>,
    ) -> Result<()> {
        // Process nodes by type to ensure proper backup order
        
        // 1. System Core (typically on master node)
        let master_nodes: Vec<_> = nodes.iter()
            .filter(|n| n.node_type == NodeType::Master)
            .collect();
            
        if let Some(master) = master_nodes.first() {
            backup_system_core(
                backup, 
                master, 
                backup_dir, 
                Arc::clone(&self.network_client), 
                Arc::clone(&backup_jobs), 
                status_tx.clone()
            ).await?;
        }
        
        // 2. Directors
        let director_nodes: Vec<_> = nodes.iter()
            .filter(|n| n.node_type == NodeType::Director)
            .collect();
            
        for node in director_nodes {
            backup_director(
                backup, 
                node, 
                backup_dir, 
                Arc::clone(&self.network_client), 
                Arc::clone(&backup_jobs), 
                status_tx.clone()
            ).await?;
        }
        
        // 3. Orchestrators
        let orchestrator_nodes: Vec<_> = nodes.iter()
            .filter(|n| n.node_type == NodeType::Orchestrator)
            .collect();
            
        for node in orchestrator_nodes {
            backup_orchestrator(
                backup, 
                node, 
                backup_dir, 
                Arc::clone(&self.network_client), 
                Arc::clone(&backup_jobs), 
                status_tx.clone()
            ).await?;
        }
        
        // 4. Network Configuration
        let network_nodes: Vec<_> = nodes.iter()
            .filter(|n| n.node_type == NodeType::NetworkController)
            .collect();
            
        if let Some(network_node) = network_nodes.first() {
            backup_network_config(
                backup, 
                network_node, 
                backup_dir, 
                Arc::clone(&self.network_client), 
                Arc::clone(&backup_jobs), 
                status_tx.clone()
            ).await?;
        }
        
        // 5. Application Definitions
        let app_catalog_nodes: Vec<_> = nodes.iter()
            .filter(|n| n.node_type == NodeType::ApplicationCatalog)
            .collect();
            
        if let Some(app_catalog) = app_catalog_nodes.first() {
            backup_app_definitions(
                backup, 
                app_catalog, 
                backup_dir, 
                Arc::clone(&self.network_client), 
                Arc::clone(&backup_jobs), 
                status_tx.clone()
            ).await?;
        }
        
        // 6. Volume Data
        let storage_nodes: Vec<_> = nodes.iter()
            .filter(|n| n.node_type == NodeType::Storage)
            .collect();
            
        for node in storage_nodes {
            backup_volume_data(
                backup, 
                node, 
                backup_dir, 
                Arc::clone(&self.network_client), 
                Arc::clone(&backup_jobs), 
                status_tx.clone()
            ).await?;
        }
        
        Ok(())
    }
}