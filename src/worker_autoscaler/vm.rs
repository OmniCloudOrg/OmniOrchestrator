use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize, Serializer, Deserializer};

/// Represents a VM managed by the autoscaler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VM {
    /// Unique identifier for the VM
    pub id: String,
    /// Human-readable name for the VM
    pub name: String,
    /// ID of the node hosting this VM
    pub node_id: String,
    /// CPU cores allocated to this VM
    pub cpu: u32,
    /// Memory in MB allocated to this VM
    pub memory: u32,
    /// Storage in GB allocated to this VM
    pub storage: u32,
    /// Current state of the VM
    pub state: VMState,
    /// When the VM was created (as milliseconds since UNIX epoch)
    #[serde(with = "timestamp_serde")]
    pub created_at: Instant,
    /// Last time the VM state was updated (as milliseconds since UNIX epoch)
    #[serde(with = "timestamp_serde")]
    pub updated_at: Instant,
    /// Additional properties specific to this VM
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

impl VM {
    /// Create a new VM
    pub fn new(id: String, name: String, node_id: String, cpu: u32, memory: u32, storage: u32) -> Self {
        let now = Instant::now();
        Self {
            id,
            name,
            node_id,
            cpu,
            memory,
            storage,
            state: VMState::Creating,
            created_at: now,
            updated_at: now,
            properties: HashMap::new(),
        }
    }
}

/// Possible states for a VM
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VMState {
    /// VM is being created
    Creating,
    /// VM is running
    Running,
    /// VM is stopped
    Stopped,
    /// VM is being terminated
    Terminating,
    /// VM has been terminated
    Terminated,
    /// VM is in error state
    Error,
}

/// Configuration for VM creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VMConfig {
    /// CPU cores for each VM
    pub cpu: u32,
    /// Memory in MB for each VM
    pub memory: u32,
    /// Storage in GB for each VM
    pub storage: u32,
    /// Additional configuration options
    pub options: HashMap<String, String>,
}

impl Default for VMConfig {
    fn default() -> Self {
        Self {
            cpu: 2,
            memory: 4096, // 4 GB
            storage: 80,  // 80 GB
            options: HashMap::new(),
        }
    }
}

/// Template for creating new VMs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VMTemplate {
    /// Base name for VMs created from this template
    pub base_name: String,
    /// VM configuration
    pub config: VMConfig,
    /// Additional tags to apply to VMs
    pub tags: HashMap<String, String>,
}

impl Default for VMTemplate {
    fn default() -> Self {
        Self {
            base_name: "worker".to_string(),
            config: VMConfig::default(),
            tags: HashMap::new(),
        }
    }
}