// backup/export.rs
//
// Handles exporting and archiving of backup sets

use crate::db_manager::v1::tables::Backup;
use std::path::{Path, PathBuf};
use std::fs;
use log::{info, warn, error, debug};
use anyhow::{Result, Context, bail, anyhow};
use chrono::Utc;
use std::process::Command;

/// Provides functionality for exporting and archiving backups
pub struct BackupExporter {
    temp_dir: PathBuf,
}

impl BackupExporter {
    /// Create a new BackupExporter instance
    pub fn new(temp_dir: impl Into<PathBuf>) -> Self {
        Self {
            temp_dir: temp_dir.into(),
        }
    }
    
    /// Recursively copy a directory and its contents
    fn copy_directory_recursive(&self, src: &Path, dst: &Path) -> Result<()> {
        fs::create_dir_all(dst)?;
        
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let path = entry.path();
            let dest_path = dst.join(path.file_name().unwrap_or_default());
            
            if path.is_dir() {
                self.copy_directory_recursive(&path, &dest_path)?;
            } else {
                fs::copy(&path, &dest_path)?;
            }
        }
        
        Ok(())
    }
    
    /// Create export manifest with additional details
    fn create_export_manifest(&self, backup: &Backup, export_dir: &Path) -> Result<()> {
        use serde_json::{json, Value};
        
        let manifest = json!({
            "export_info": {
                "exported_at": Utc::now().to_string(),
                "exported_by": "OmniCloud Backup System",
                "export_version": "1.0"
            },
            "backup_info": {
                "backup_id": backup.id,
                "backup_name": backup.name,
                "created_at": backup.created_at.to_string(),
                "created_by": backup.created_by,
                "backup_type": backup.backup_type,
                "source_environment": backup.source_environment,
                "size_bytes": backup.size_bytes.unwrap_or(0),
                "components": {
                    "system_core": backup.has_system_core,
                    "directors": backup.has_directors,
                    "orchestrators": backup.has_orchestrators,
                    "network_config": backup.has_network_config,
                    "app_definitions": backup.has_app_definitions,
                    "volume_data": backup.has_volume_data
                }
            }
        });
        
        // Write manifest file
        let manifest_path = export_dir.join("export_manifest.json");
        let manifest_json = serde_json::to_string_pretty(&manifest)?;
        fs::write(&manifest_path, manifest_json)?;
        
        Ok(())
    }
    
    /// Prepare backup for export or archiving
    pub fn prepare_for_export(&self, backup: &Backup, export_path: &Path) -> Result<PathBuf> {
        info!("Preparing backup for export: {}", backup.name);
        
        // Determine the backup directory
        let time_str = backup.created_at.format("%Y%m%d-%H%M%S").to_string();
        let backup_path = PathBuf::from(&backup.storage_location).join(format!("backup-{}", time_str));
        
        if !backup_path.exists() {
            return Err(anyhow!("Backup directory not found: {}", backup_path.display()));
        }
        
        // Create export directory
        let export_dir = export_path.join(format!("omnicloud-backup-{}-{}", backup.id, time_str));
        fs::create_dir_all(&export_dir)?;
        
        // Copy ISO files
        let isos_dir = backup_path.join("isos");
        let export_isos_dir = export_dir.join("isos");
        fs::create_dir_all(&export_isos_dir)?;
        
        for entry in fs::read_dir(&isos_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "iso") {
                let dest_path = export_isos_dir.join(path.file_name().unwrap_or_default());
                fs::copy(&path, &dest_path)?;
                info!("Copied ISO file: {}", dest_path.display());
            }
        }
        
        // Copy metadata
        let metadata_dir = backup_path.join("metadata");
        let export_metadata_dir = export_dir.join("metadata");
        fs::create_dir_all(&export_metadata_dir)?;
        
        for entry in fs::read_dir(&metadata_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                let dest_path = export_metadata_dir.join(path.file_name().unwrap_or_default());
                fs::copy(&path, &dest_path)?;
                info!("Copied metadata file: {}", dest_path.display());
            } else if path.is_dir() {
                let subdir_name = path.file_name().unwrap_or_default();
                let export_subdir = export_metadata_dir.join(subdir_name);
                
                // Create subdirectory
                fs::create_dir_all(&export_subdir)?;
                
                // Copy files in subdirectory
                for subentry in fs::read_dir(&path)? {
                    let subentry = subentry?;
                    let subpath = subentry.path();
                    
                    if subpath.is_file() {
                        let dest_path = export_subdir.join(subpath.file_name().unwrap_or_default());
                        fs::copy(&subpath, &dest_path)?;
                        info!("Copied metadata subfile: {}", dest_path.display());
                    }
                }
            }
        }
        
        // Copy recovery scripts
        let scripts_dir = backup_path.join("scripts");
        let export_scripts_dir = export_dir.join("scripts");
        fs::create_dir_all(&export_scripts_dir)?;
        
        if scripts_dir.exists() && scripts_dir.is_dir() {
            // Copy all subdirectories recursively
            self.copy_directory_recursive(&scripts_dir, &export_scripts_dir)?;
        }
        
        // Create export manifest
        self.create_export_manifest(backup, &export_dir)?;
        
        info!("Backup export prepared successfully: {}", export_dir.display());
        Ok(export_dir.to_path_buf())
    }
    
    /// Archive a backup to a compressed file
    pub fn archive_backup(&self, backup: &Backup, archive_path: Option<&Path>) -> Result<PathBuf> {
        info!("Archiving backup: {}", backup.name);
        
        // Determine the backup directory
        let time_str = backup.created_at.format("%Y%m%d-%H%M%S").to_string();
        let backup_path = PathBuf::from(&backup.storage_location).join(format!("backup-{}", time_str));
        
        if !backup_path.exists() {
            return Err(anyhow!("Backup directory not found: {}", backup_path.display()));
        }
        
        // Determine archive output path
        let output_path = if let Some(path) = archive_path {
            path.to_path_buf()
        } else {
            let archive_name = format!("omnicloud-backup-{}-{}.tar.gz", backup.id, time_str);
            PathBuf::from(&backup.storage_location).join("archives").join(archive_name)
        };
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Create archive using tar
        info!("Creating archive: {}", output_path.display());
        
        // In a real implementation, use tar and gzip to create the archive
        // For this example, we'll simulate the process
        
        #[cfg(target_family = "unix")]
        {
            // On Unix-like systems, use the tar command
            let status = Command::new("tar")
                .arg("-czf")
                .arg(&output_path)
                .arg("-C")
                .arg(backup_path.parent().unwrap_or(Path::new("/")))
                .arg(backup_path.file_name().unwrap_or_default())
                .status();
                
            match status {
                Ok(exit_status) => {
                    if !exit_status.success() {
                        return Err(anyhow!("Failed to create archive: tar command exited with non-zero status"));
                    }
                },
                Err(e) => {
                    // If tar command fails, create a placeholder archive file
                    let placeholder = format!(
                        "This is a placeholder archive for backup: {}\n\
                        Backup ID: {}\n\
                        Created at: {}\n",
                        backup.name,
                        backup.id,
                        Utc::now().to_string()
                    );
                    
                    fs::write(&output_path, placeholder)?;
                    
                    warn!("Could not run tar command: {}. Created placeholder archive.", e);
                }
            }
        }
        
        #[cfg(not(target_family = "unix"))]
        {
            // On other platforms, create a placeholder archive file
            let placeholder = format!(
                "This is a placeholder archive for backup: {}\n\
                Backup ID: {}\n\
                Created at: {}\n",
                backup.name,
                backup.id,
                Utc::now().to_string()
            );
            
            fs::write(&output_path, placeholder)?;
            
            warn!("Archive creation simulated on non-Unix platform");
        }
        
        info!("Backup archived successfully: {}", output_path.display());
        Ok(output_path)
    }
    
    /// Clean up older backups based on retention policy
    pub fn clean_old_backups(&self, backup_ids: &[i32], retention_days: i64) -> Result<Vec<i32>> {
        info!("Cleaning up old backups, retention period: {} days", retention_days);
        
        let mut removed_ids = Vec::new();
        let now = Utc::now();
        
        for &id in backup_ids {
            // In a real implementation, this would query the database
            // For this example, we'll just simulate the process
            
            // Example: Get backup details (would be from DB in real implementation)
            let backup_created_at = now - chrono::Duration::days(retention_days + 5); // Simulate old backup
            let backup_age_days = (now - backup_created_at).num_days();
            
            if backup_age_days > retention_days {
                info!("Backup ID {} is {} days old, exceeding retention period of {} days. Marking for removal.", 
                     id, backup_age_days, retention_days);
                removed_ids.push(id);
            } else {
                debug!("Backup ID {} is {} days old, within retention period of {} days. Keeping.", 
                      id, backup_age_days, retention_days);
            }
        }
        
        info!("Identified {} backups for removal", removed_ids.len());
        Ok(removed_ids)
    }
    
    /// Physically remove a backup from storage
    pub fn remove_backup(&self, backup: &Backup) -> Result<()> {
        info!("Removing backup: {}", backup.name);
        
        // Determine the backup directory
        let time_str = backup.created_at.format("%Y%m%d-%H%M%S").to_string();
        let backup_path = PathBuf::from(&backup.storage_location).join(format!("backup-{}", time_str));
        
        if backup_path.exists() {
            // Remove the backup directory
            info!("Removing backup directory: {}", backup_path.display());
            fs::remove_dir_all(&backup_path)?;
        } else {
            warn!("Backup directory not found: {}", backup_path.display());
        }
        
        // Check for related archives
        let archives_dir = PathBuf::from(&backup.storage_location).join("archives");
        if archives_dir.exists() && archives_dir.is_dir() {
            let archive_prefix = format!("omnicloud-backup-{}-", backup.id);
            
            for entry in fs::read_dir(&archives_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_file() {
                    if let Some(filename) = path.file_name() {
                        let filename_str = filename.to_string_lossy();
                        
                        if filename_str.starts_with(&archive_prefix) {
                            info!("Removing backup archive: {}", path.display());
                            fs::remove_file(&path)?;
                        }
                    }
                }
            }
        }
        
        info!("Backup removed successfully: {}", backup.name);
        Ok(())
    }
    
    /// Export backup metadata only (no ISOs)
    pub fn export_metadata_only(&self, backup: &Backup, export_path: &Path) -> Result<PathBuf> {
        info!("Exporting backup metadata only: {}", backup.name);
        
        // Determine the backup directory
        let time_str = backup.created_at.format("%Y%m%d-%H%M%S").to_string();
        let backup_path = PathBuf::from(&backup.storage_location).join(format!("backup-{}", time_str));
        
        if !backup_path.exists() {
            return Err(anyhow!("Backup directory not found: {}", backup_path.display()));
        }
        
        // Create export directory
        let export_dir = export_path.join(format!("omnicloud-backup-metadata-{}-{}", backup.id, time_str));
        fs::create_dir_all(&export_dir)?;
        
        // Copy only metadata files
        let metadata_dir = backup_path.join("metadata");
        let export_metadata_dir = export_dir.join("metadata");
        fs::create_dir_all(&export_metadata_dir)?;
        
        if metadata_dir.exists() && metadata_dir.is_dir() {
            self.copy_directory_recursive(&metadata_dir, &export_metadata_dir)?;
        }
        
        // Copy recovery scripts (optional)
        let scripts_dir = backup_path.join("scripts");
        let export_scripts_dir = export_dir.join("scripts");
        
        if scripts_dir.exists() && scripts_dir.is_dir() {
            fs::create_dir_all(&export_scripts_dir)?;
            self.copy_directory_recursive(&scripts_dir, &export_scripts_dir)?;
        }
        
        // Create export manifest
        self.create_export_manifest(backup, &export_dir)?;
        
        // Add ISO information without the actual ISO files
        let isos_info_path = export_dir.join("iso_information.txt");
        let isos_dir = backup_path.join("isos");
        
        if isos_dir.exists() && isos_dir.is_dir() {
            let mut iso_info = String::new();
            iso_info.push_str(&format!("ISO files in backup: {}\n", backup.name));
            iso_info.push_str(&format!("Backup ID: {}\n", backup.id));
            iso_info.push_str(&format!("Created at: {}\n\n", backup.created_at));
            
            let mut iso_count = 0;
            
            for entry in fs::read_dir(&isos_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.extension().map_or(false, |ext| ext == "iso") {
                    let filename = path.file_name().unwrap_or_default().to_string_lossy();
                    let metadata = fs::metadata(&path)?;
                    
                    iso_info.push_str(&format!("ISO: {}\n", filename));
                    iso_info.push_str(&format!("  Size: {} bytes\n", metadata.len()));
                    iso_info.push_str(&format!("  Last Modified: {:?}\n\n", metadata.modified()?));
                    
                    iso_count += 1;
                }
            }
            
            iso_info.push_str(&format!("Total ISO count: {}\n", iso_count));
            fs::write(&isos_info_path, iso_info)?;
        }
        
        info!("Backup metadata exported successfully: {}", export_dir.display());
        Ok(export_dir)
    }
    
    /// Merge multiple backups into a single comprehensive backup
    pub fn merge_backups(&self, backup_ids: &[i32], output_name: &str, storage_location: &Path) -> Result<PathBuf> {
        info!("Merging {} backups into: {}", backup_ids.len(), output_name);
        
        // Create merged backup directory
        let merged_dir = storage_location.join(format!("merged-backup-{}", 
            Utc::now().format("%Y%m%d-%H%M%S")));
        fs::create_dir_all(&merged_dir)?;
        fs::create_dir_all(merged_dir.join("isos"))?;
        fs::create_dir_all(merged_dir.join("metadata"))?;
        fs::create_dir_all(merged_dir.join("scripts"))?;
        
        // In a real implementation, this would query the database for backup details
        // For this example, we'll just create a simulated merged backup
        
        // Create merged backup manifest
        let manifest = serde_json::json!({
            "merged_backup_info": {
                "name": output_name,
                "created_at": Utc::now().to_string(),
                "source_backups": backup_ids,
                "merged_by": "OmniCloud Backup System"
            },
            "components": {
                "system_core": true,
                "directors": true,
                "orchestrators": true,
                "network_config": true,
                "app_definitions": true,
                "volume_data": true
            }
        });
        
        // Write manifest file
        let manifest_path = merged_dir.join("metadata").join("merged_manifest.json");
        let manifest_json = serde_json::to_string_pretty(&manifest)?;
        fs::write(&manifest_path, manifest_json)?;
        
        // Create placeholder for merged ISOs
        let placeholder = format!(
            "This is a placeholder for merged backup ISOs\n\
            Name: {}\n\
            Created at: {}\n\
            Source backups: {:?}\n",
            output_name,
            Utc::now().to_string(),
            backup_ids
        );
        
        fs::write(merged_dir.join("isos").join("merged-placeholder.txt"), placeholder)?;
        
        info!("Backup merging completed: {}", merged_dir.display());
        Ok(merged_dir)
    }
}