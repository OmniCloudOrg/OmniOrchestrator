use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Configuration for the OmniOrchestrator server application.
///
/// This structure defines all the configurable parameters for the server,
/// including network settings and behavior options. It supports serialization 
/// to and deserialization from JSON for persistent configuration.
///
/// The configuration can be loaded from a file or generated with default
/// values if no configuration file exists.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// The port number on which the server will listen
    pub port: u16,
    
    /// The IP address to which the server will bind
    pub address: String,
    
    /// Whether to apply syntax highlighting to SQL logs
    pub highlight_sql: bool,
    
    /// List of other server instances in the cluster
    pub instances: Vec<Instance>,
}

/// Represents an instance of the server in the cluster.
///
/// This structure contains the network location information for a server
/// instance that is part of the OmniOrchestrator cluster. It's used for
/// peer discovery and communication between cluster nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    /// The port number on which the instance is listening
    pub port: u16,
    
    /// The hostname or IP address of the instance
    pub address: String,
}

/// Default implementation for ServerConfig.
///
/// Provides reasonable default values for a server configuration to be
/// used when no custom configuration is provided or when initializing
/// a new configuration file.
impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 8000,
            address: "127.0.0.1".to_string(),
            highlight_sql: true,
            instances: vec![Instance {
                port: 8000,
                address: "example.com".to_string(),
            }],
        }
    }
}

/// Possible errors that can occur during configuration operations.
///
/// This enum represents the various error conditions that might arise
/// when reading from or writing to the configuration file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigError {
    /// Indicates that the configuration file could not be found
    FileNotFound,
    
    /// Indicates that writing to the configuration file failed
    FailedToWrite,
    
    /// Indicates that parsing the configuration file content failed
    ParseError,
}

/// Global static reference to the server configuration.
///
/// This lazy_static provides thread-safe access to the server configuration
/// throughout the application. It is initialized when first accessed,
/// reading from the configuration file or creating default settings if
/// no configuration exists.
///
/// # Panics
///
/// Panics if the configuration cannot be read or written, which would
/// prevent the server from starting properly.
lazy_static! {
    pub static ref SERVER_CONFIG: Arc<ServerConfig> =
        Arc::new(ServerConfig::read().expect("Failed to initalize server config"));
}

impl ServerConfig {
    /// Reads the server configuration from the config file.
    ///
    /// Attempts to load the configuration from "config.json" in the current
    /// directory. If the file doesn't exist or can't be read, it creates a new
    /// configuration file with default values and returns those defaults.
    ///
    /// # Returns
    ///
    /// * `Ok(ServerConfig)` - Successfully loaded or created configuration
    /// * `Err(ConfigError)` - Failed to parse existing configuration
    ///
    /// # Error Handling
    ///
    /// - If the file doesn't exist, creates a default configuration
    /// - If the file exists but can't be parsed, returns a ParseError
    pub fn read() -> Result<Self, ConfigError> {
        let config_path = "config.json";
        let config_content = match std::fs::read_to_string(config_path) {
            Ok(content) => content,
            Err(_) => {
                // If file doesn't exist, create a default configuration
                Self::write_default().expect("Failed to write default config");
                return Ok(ServerConfig::default());
            }
        };
        
        // Parse the configuration from JSON
        let config: ServerConfig = match serde_json::from_str(&config_content) {
            Ok(config) => config,
            Err(_) => return Err(ConfigError::ParseError),
        };
        
        Ok(config)
    }
    
    /// Writes the current configuration to the config file.
    ///
    /// Serializes the configuration to JSON and writes it to "config.json"
    /// in the current directory. This allows configuration changes to persist
    /// across server restarts.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Successfully wrote configuration to file
    /// * `Err(ConfigError)` - Failed to serialize or write configuration
    ///
    /// # Error Handling
    ///
    /// - Returns ParseError if serialization to JSON fails
    /// - Returns FailedToWrite if writing to the file fails
    pub fn write(&self) -> Result<(), ConfigError> {
        let config_path = "config.json";
        
        // Serialize the configuration to pretty-printed JSON
        let config_content = match serde_json::to_string_pretty(&self) {
            Ok(content) => content,
            Err(_) => return Err(ConfigError::ParseError),
        };
        
        // Write the JSON to the configuration file
        match std::fs::write(config_path, config_content) {
            Ok(_) => Ok(()),
            Err(_) => Err(ConfigError::FailedToWrite),
        }
    }
    
    /// Creates and writes a default configuration to the config file.
    ///
    /// This is a convenience method that creates a ServerConfig with default
    /// values and writes it to the configuration file. It's typically used
    /// when no configuration file exists yet.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Successfully wrote default configuration to file
    /// * `Err(ConfigError)` - Failed to write default configuration
    pub fn write_default() -> Result<(), ConfigError> {
        let config = ServerConfig::default();
        config.write()
    }
}