// backup/model.rs
//
// Core data model for the OmniCloud backup system

pub use crate::db::v1::tables::Backup;
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use std::path::PathBuf;

impl Backup {
    /// Create a new Backup instance with default values
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        Backup {
            id: 0,
            status: "pending".to_string(),
            created_at: now,
            name: String::new(),
            description: Some(String::new()),
            created_by: String::new(),
            backup_type: "full".to_string(),
            format_version: "1.0".to_string(),
            source_environment: String::new(),
            encryption_method: Some("none".to_string()),
            encryption_key_id: None,
            size_bytes: Some(0),
            has_system_core: false,
            has_directors: false,
            has_orchestrators: false,
            has_network_config: false,
            has_app_definitions: false,
            has_volume_data: false,
            included_apps: None,
            included_services: None,
            last_validated_at: None,
            last_restored_at: None,
            restore_target_environment: Some(String::new()),
            restore_status: Some("none".to_string()),
            storage_location: String::new(),
            manifest_path: String::new(),
            metadata: Some(Value::Null),
        }
    }

    /// Returns a formatted string for the backup's created_at field
    pub fn formatted_created_at(&self) -> String {
        self.created_at.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    /// Checks if the backup was successful
    pub fn is_successful(&self) -> bool {
        self.status == "success"
    }

    /// Sets the backup as successful
    pub fn set_success(&mut self) {
        self.status = "success".to_string();
    }

    /// Sets the backup as failed with an optional error message
    pub fn set_failed(&mut self, error_msg: Option<String>) {
        self.status = "failed".to_string();
        if let Some(msg) = error_msg {
            self.metadata = Some(json!({
                "error": msg
            }));
        }
    }

    /// Updates the backup size
    pub fn set_size(&mut self, size: u64) {
        self.size_bytes = Some(size.try_into().unwrap_or(0));
    }

    /// Records a validation attempt
    pub fn mark_validated(&mut self) {
        self.last_validated_at = Some(Utc::now());
    }

    /// Records a restore attempt
    pub fn mark_restored(&mut self, target_env: String) {
        self.last_restored_at = Some(Utc::now());
        self.restore_target_environment = Some(target_env);
        self.restore_status = Some("completed".to_string());
    }

    /// Checks if the backup contains any system components
    pub fn has_system_components(&self) -> bool {
        self.has_system_core || 
        self.has_directors || 
        self.has_orchestrators || 
        self.has_network_config
    }

    /// Updates the backup metadata
    pub fn update_metadata(&mut self, metadata: Value) {
        self.metadata = Some(metadata);
    }

    /// Gets the age of the backup in days
    pub fn age_in_days(&self) -> i64 {
        let now = Utc::now();
        (now - self.created_at).num_days()
    }

    /// Create a new backup job, setting up parameters and initializing structures
    pub fn create_backup_job(
        name: String, 
        description: Option<String>, 
        environment: String, 
        backup_type: String, 
        created_by: String,
        apps_to_include: Option<Vec<String>>,
        services_to_include: Option<Vec<String>>,
        encryption_method: Option<String>,
        encryption_key_id: Option<String>,
        storage_location: String,
    ) -> Self {
        let mut backup = Self::new();
        
        // Generate a unique identifier for the backup name if not provided
        let name = if name.is_empty() {
            format!("backup-{}-{}", environment, Utc::now().format("%Y%m%d-%H%M%S"))
        } else {
            name
        };
        
        backup.name = name;
        backup.description = description;
        backup.source_environment = environment;
        backup.backup_type = backup_type;
        backup.created_by = created_by;
        backup.included_apps = apps_to_include.map(|apps| serde_json::to_string(&apps).unwrap_or_default());
        backup.included_services = services_to_include.map(|services| serde_json::to_string(&services).unwrap_or_default());
        backup.encryption_method = encryption_method;
        backup.encryption_key_id = encryption_key_id.and_then(|id| id.parse::<i64>().ok());
        backup.storage_location = storage_location.clone(); // TODO: Ensure we actually NEED to clone here, or can just use a reference
        backup.status = "initializing".to_string();
        
        // Set up the manifest path
        let manifest_path = format!("{}/backup-{}/manifest.json", 
            storage_location, 
            backup.created_at.format("%Y%m%d-%H%M%S"));
        backup.manifest_path = manifest_path;
        
        backup
    }

    /// Initialize the backup environment, creating necessary directories
    pub fn initialize_backup_environment(&self) -> Result<PathBuf, anyhow::Error> {
        use std::fs;
        use std::path::Path;
        use anyhow::Context;

        let backup_dir = Path::new(&self.storage_location)
            .join(format!("backup-{}", self.created_at.format("%Y%m%d-%H%M%S")));
        
        // Create the backup directory and subdirectories
        fs::create_dir_all(&backup_dir).context("Failed to create backup directory")?;
        fs::create_dir_all(backup_dir.join("isos")).context("Failed to create ISOs directory")?;
        fs::create_dir_all(backup_dir.join("metadata")).context("Failed to create metadata directory")?;
        fs::create_dir_all(backup_dir.join("temp")).context("Failed to create temp directory")?;
        
        // Return the backup directory path
        Ok(backup_dir)
    }
}