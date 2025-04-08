// backup/validation.rs
//
// Handles validation of backup sets and ISO files

use crate::db::v1::tables::Backup;
use crate::backup::iso::IsoManager;
use chrono::Utc;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::fs;
use log::{info, warn, error, debug};
use anyhow::{Result, Context, bail, anyhow};

/// Provides validation capabilities for backups
pub struct BackupValidator {
    iso_manager: IsoManager,
}

impl BackupValidator {
    /// Create a new BackupValidator instance
    pub fn new(temp_dir: impl Into<PathBuf>) -> Self {
        Self {
            iso_manager: IsoManager::new(temp_dir),
        }
    }
    
    /// Validate an existing backup
    pub fn validate_backup(&self, backup: &mut Backup, backup_dir: Option<String>) -> Result<bool> {
        info!("Validating backup: {}", backup.name);
        
        // Determine the backup directory
        let backup_path = if let Some(dir) = backup_dir {
            PathBuf::from(dir)
        } else {
            let time_str = backup.created_at.format("%Y%m%d-%H%M%S").to_string();
            PathBuf::from(&backup.storage_location).join(format!("backup-{}", time_str))
        };
        
        if !backup_path.exists() {
            return Err(anyhow!("Backup directory not found: {}", backup_path.display()));
        }
        
        // Check manifest file
        let manifest_path = backup_path.join("metadata").join("manifest.json");
        if !manifest_path.exists() {
            return Err(anyhow!("Manifest file not found: {}", manifest_path.display()));
        }
        
        // Read manifest
        let manifest_content = fs::read_to_string(&manifest_path)?;
        let manifest: Value = serde_json::from_str(&manifest_content)?;
        
        // Verify backup metadata matches
        if manifest["backup_id"] != backup.id {
            warn!("Backup ID mismatch in manifest: {} vs {}", manifest["backup_id"], backup.id);
        }
        
        // Check ISO files
        let isos_dir = backup_path.join("isos");
        let isos_exist = isos_dir.exists() && fs::read_dir(&isos_dir)?.next().is_some();
        
        if !isos_exist {
            return Err(anyhow!("No ISO files found in backup"));
        }
        
        // Perform integrity check on ISOs
        let mut validation_success = true;
        let mut validation_errors = Vec::new();
        
        for entry in fs::read_dir(&isos_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "iso") {
                info!("Validating ISO: {}", path.display());
                
                // Validate ISO structure
                if !self.iso_manager.validate_iso_structure(&path)? {
                    validation_success = false;
                    validation_errors.push(format!("Invalid ISO structure: {}", path.display()));
                    continue;
                }
                
                // Check ISO file size
                let metadata = fs::metadata(&path)?;
                if metadata.len() == 0 {
                    validation_success = false;
                    validation_errors.push(format!("Empty ISO file: {}", path.display()));
                }
            }
        }
        
        // Check recovery scripts
        let scripts_dir = backup_path.join("scripts");
        if !scripts_dir.exists() {
            validation_success = false;
            validation_errors.push("Recovery scripts directory not found".to_string());
        } else {
            // Check for required script files
            let recovery_script = scripts_dir.join("recovery").join("main.sh");
            if !recovery_script.exists() {
                validation_success = false;
                validation_errors.push("Main recovery script not found".to_string());
            }
            
            let validation_script = scripts_dir.join("validation").join("validate.sh");
            if !validation_script.exists() {
                validation_success = false;
                validation_errors.push("Validation script not found".to_string());
            }
        }
        
        // Record validation
        backup.mark_validated();
        
        if !validation_success {
            let error_msg = validation_errors.join("; ");
            warn!("Backup validation failed: {}", error_msg);
            backup.update_metadata(json!({
                "validation": {
                    "status": "failed",
                    "errors": validation_errors,
                    "timestamp": Utc::now().to_string()
                }
            }));
            return Ok(false);
        }
        
        // Update metadata with validation result
        backup.update_metadata(json!({
            "validation": {
                "status": "success",
                "timestamp": Utc::now().to_string()
            }
        }));
        
        info!("Backup validation successful: {}", backup.name);
        Ok(true)
    }
    
    /// Perform deep validation of all ISO files
    pub fn deep_validate_isos(&self, backup: &mut Backup, backup_dir: Option<String>) -> Result<bool> {
        info!("Performing deep validation of ISOs for backup: {}", backup.name);
        
        // Determine the backup directory
        let backup_path = if let Some(dir) = backup_dir {
            PathBuf::from(dir)
        } else {
            let time_str = backup.created_at.format("%Y%m%d-%H%M%S").to_string();
            PathBuf::from(&backup.storage_location).join(format!("backup-{}", time_str))
        };
        
        if !backup_path.exists() {
            return Err(anyhow!("Backup directory not found: {}", backup_path.display()));
        }
        
        // Check ISO files
        let isos_dir = backup_path.join("isos");
        if !isos_dir.exists() {
            return Err(anyhow!("ISOs directory not found: {}", isos_dir.display()));
        }
        
        // Create temporary extraction directory
        let extract_dir = backup_path.join("temp").join("validation_extract");
        fs::create_dir_all(&extract_dir)?;
        
        // Track validation results
        let mut validation_success = true;
        let mut validation_errors = Vec::new();
        let mut validated_components = Vec::new();
        
        // Process each ISO file
        for entry in fs::read_dir(&isos_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "iso") {
                let iso_filename = path.file_name().unwrap_or_default().to_string_lossy();
                info!("Deep validating ISO: {}", iso_filename);
                
                // Create component-specific extraction directory
                let component_extract_dir = extract_dir.join(iso_filename.to_string());
                fs::create_dir_all(&component_extract_dir)?;
                
                // Extract ISO files for validation
                match self.iso_manager.extract_iso_to_directory(&path, &component_extract_dir) {
                    Ok(_) => {
                        // Verify required files in the extracted ISO
                        let manifest_file = component_extract_dir.join("metadata").join("manifest.json");
                        let data_dir = component_extract_dir.join("data");
                        
                        if !manifest_file.exists() {
                            validation_success = false;
                            validation_errors.push(format!("Missing manifest file in ISO: {}", iso_filename));
                            continue;
                        }
                        
                        if !data_dir.exists() || !data_dir.is_dir() {
                            validation_success = false;
                            validation_errors.push(format!("Missing data directory in ISO: {}", iso_filename));
                            continue;
                        }
                        
                        // Read component manifest
                        match fs::read_to_string(&manifest_file) {
                            Ok(manifest_content) => {
                                match serde_json::from_str::<Value>(&manifest_content) {
                                    Ok(manifest) => {
                                        // Add component to validated list
                                        if let Some(component_type) = manifest["component_type"].as_str() {
                                            validated_components.push(component_type.to_string());
                                        }
                                    },
                                    Err(e) => {
                                        validation_success = false;
                                        validation_errors.push(format!("Invalid manifest JSON in ISO {}: {}", iso_filename, e));
                                    }
                                }
                            },
                            Err(e) => {
                                validation_success = false;
                                validation_errors.push(format!("Failed to read manifest in ISO {}: {}", iso_filename, e));
                            }
                        }
                    },
                    Err(e) => {
                        validation_success = false;
                        validation_errors.push(format!("Failed to extract ISO {}: {}", iso_filename, e));
                    }
                }
            }
        }
        
        // Clean up temporary extraction directory
        let _ = fs::remove_dir_all(&extract_dir);
        
        // Record validation
        backup.mark_validated();
        
        // Update metadata with detailed validation results
        let validation_metadata = json!({
            "deep_validation": {
                "status": if validation_success { "success" } else { "failed" },
                "errors": validation_errors,
                "validated_components": validated_components,
                "timestamp": Utc::now().to_string()
            }
        });
        
        backup.update_metadata(validation_metadata);
        
        if !validation_success {
            let error_msg = validation_errors.join("; ");
            warn!("Deep ISO validation failed: {}", error_msg);
            return Ok(false);
        }
        
        info!("Deep ISO validation successful for backup: {}", backup.name);
        Ok(true)
    }
    
    /// Verify that all required components are present in the backup
    pub fn verify_backup_completeness(&self, backup: &Backup) -> Result<bool> {
        info!("Verifying backup completeness: {}", backup.name);
        
        let mut missing_components = Vec::new();
        
        // Check for required components based on backup type
        if backup.backup_type == "full" {
            // Full backups should have all system components
            if !backup.has_system_core {
                missing_components.push("System Core");
            }
            
            if !backup.has_directors {
                missing_components.push("Directors");
            }
            
            if !backup.has_orchestrators {
                missing_components.push("Orchestrators");
            }
            
            if !backup.has_network_config {
                missing_components.push("Network Configuration");
            }
            
            if !backup.has_app_definitions {
                missing_components.push("Application Definitions");
            }
            
            if !backup.has_volume_data {
                missing_components.push("Volume Data");
            }
        } else if backup.backup_type == "system" {
            // System backups should have system components but not necessarily app data
            if !backup.has_system_core {
                missing_components.push("System Core");
            }
            
            if !backup.has_directors {
                missing_components.push("Directors");
            }
            
            if !backup.has_orchestrators {
                missing_components.push("Orchestrators");
            }
            
            if !backup.has_network_config {
                missing_components.push("Network Configuration");
            }
        } else if backup.backup_type == "app" {
            // App backups should have app definitions and volume data
            if !backup.has_app_definitions {
                missing_components.push("Application Definitions");
            }
            
            if !backup.has_volume_data {
                missing_components.push("Volume Data");
            }
        }
        
        if !missing_components.is_empty() {
            warn!("Backup is incomplete. Missing components: {}", missing_components.join(", "));
            return Ok(false);
        }
        
        info!("Backup is complete with all required components");
        Ok(true)
    }
}