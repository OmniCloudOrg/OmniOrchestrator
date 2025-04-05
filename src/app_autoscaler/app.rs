use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize, Serializer, Deserializer};

/// Represents an app instance managed by the autoscaler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInstance {
    /// Unique identifier for the app instance
    pub id: String,
    /// Human-readable name for the app instance
    pub name: String,
    /// ID of the node hosting this app instance
    pub node_id: String,
    /// CPU cores allocated to this app instance
    pub cpu: u32,
    /// Memory in MB allocated to this app instance
    pub memory: u32,
    /// Storage in GB allocated to this app instance
    pub storage: u32,
    /// Current state of the app instance
    pub state: AppInstanceState,
    /// When the app instance was created (as milliseconds since UNIX epoch)
    #[serde(with = "timestamp_serde")]
    pub created_at: Instant,
    /// Last time the app instance state was updated (as milliseconds since UNIX epoch)
    #[serde(with = "timestamp_serde")]
    pub updated_at: Instant,
    /// Additional properties specific to this app instance
    pub properties: HashMap<String, String>,
}

mod timestamp_serde {
    use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(instant: &Instant, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = instant.duration_since(Instant::now()) + SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0));
        serializer.serialize_u64(duration.as_millis() as u64)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Instant, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = u64::deserialize(deserializer)?;
        let duration = Duration::from_millis(millis);
        let system_now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0));
        Ok(Instant::now() - (system_now - duration))
    }
}

impl AppInstance {
    /// Create a new app instance
    pub fn new(id: String, name: String, node_id: String, cpu: u32, memory: u32, storage: u32) -> Self {
        let now = Instant::now();
        Self {
            id,
            name,
            node_id,
            cpu,
            memory,
            storage,
            state: AppInstanceState::Creating,
            created_at: now,
            updated_at: now,
            properties: HashMap::new(),
        }
    }
}

/// Possible states for an app instance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppInstanceState {
    /// App instance is being created
    Creating,
    /// App instance is running
    Running,
    /// App instance is stopped
    Stopped,
    /// App instance is being terminated
    Terminating,
    /// App instance has been terminated
    Terminated,
    /// App instance is in error state
    Error,
}

/// Configuration for app instance creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// CPU cores for each app instance
    pub cpu: u32,
    /// Memory in MB for each app instance
    pub memory: u32,
    /// Storage in GB for each app instance
    pub storage: u32,
    /// Additional configuration options
    pub options: HashMap<String, String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            cpu: 2,
            memory: 4096, // 4 GB
            storage: 80,  // 80 GB
            options: HashMap::new(),
        }
    }
}

/// Template for creating new app instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppTemplate {
    /// Base name for app instances created from this template
    pub base_name: String,
    /// App configuration
    pub config: AppConfig,
    /// Additional tags to apply to app instances
    pub tags: HashMap<String, String>,
}

impl Default for AppTemplate {
    fn default() -> Self {
        Self {
            base_name: "worker".to_string(),
            config: AppConfig::default(),
            tags: HashMap::new(),
        }
    }
}