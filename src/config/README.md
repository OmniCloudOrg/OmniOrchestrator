# OmniOrchestrator Configuration System Developer Guide

## Architecture Overview

The OmniOrchestrator configuration system provides a flexible, centralized way to manage application settings across the platform. This guide outlines the architecture, implementation patterns, and best practices for working with and extending the configuration system.

## Core Concepts

### Configuration Model

The configuration system is built around a hierarchical model of settings represented as Rust structs:

- **ServerConfig**: The root configuration object containing all server settings
- **Specialized Configs**: Nested configuration objects for different subsystems
- **Primitive Values**: Basic configuration values (strings, numbers, booleans)

### Static vs. Dynamic Configuration

The system supports two types of configuration:

1. **Static Configuration**: Loaded at startup from configuration files
2. **Dynamic Configuration**: Can be modified at runtime via API or UI

## Configuration Structure

### Main Configuration Object

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    // Network settings
    pub port: u16,
    pub address: String,
    
    // Feature flags
    pub highlight_sql: bool,
    
    // Cluster configuration
    pub instances: Vec<Instance>,
    
    // Security settings
    pub security: SecurityConfig,
    
    // Database configuration
    pub database: DatabaseConfig,
    
    // Logging configuration
    pub logging: LoggingConfig,
    
    // Additional specialized subsystem configurations
    pub workers: WorkerConfig,
    pub metrics: MetricsConfig,
    pub cache: CacheConfig,
}
```

### Subsystem Configuration Objects

Each major subsystem should have its own configuration struct:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub enable_tls: bool,
    pub cert_path: String,
    pub key_path: String,
    pub allowed_origins: Vec<String>,
    pub authentication_timeout_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub connection_string: String,
    pub max_connections: u32,
    pub connection_timeout_seconds: u32,
    pub query_timeout_seconds: u32,
}
```

## Implementation Guidelines

### Configuration Definition

When defining new configuration structures:

1. **Include Documentation**: Document each field with `///` comments
2. **Use Appropriate Types**: Use the most specific type that represents the data
3. **Provide Defaults**: Implement `Default` for all configuration structs
4. **Consider Validation**: Add validation methods for complex configurations

Example pattern for a new subsystem configuration:

```rust
/// Configuration for the monitoring subsystem.
///
/// Controls how system metrics are collected, stored, and processed
/// for observability and alerting purposes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// How frequently to collect metrics (in seconds)
    pub collection_interval: u32,
    
    /// Maximum number of data points to store in memory
    pub retention_count: usize,
    
    /// Whether to enable alerting on metric thresholds
    pub enable_alerts: bool,
    
    /// Threshold configurations for different metrics
    pub thresholds: HashMap<String, f64>,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            collection_interval: 60,
            retention_count: 1000,
            enable_alerts: true,
            thresholds: [
                ("cpu_usage".to_string(), 90.0),
                ("memory_usage".to_string(), 85.0),
                ("disk_usage".to_string(), 90.0),
            ].iter().cloned().collect(),
        }
    }
}

impl MonitoringConfig {
    /// Validates the monitoring configuration.
    ///
    /// Ensures that all configuration values are within acceptable ranges.
    pub fn validate(&self) -> Result<(), ConfigValidationError> {
        if self.collection_interval < 10 {
            return Err(ConfigValidationError::ValueTooLow {
                field: "collection_interval".to_string(),
                min_value: 10,
                actual_value: self.collection_interval as i64,
            });
        }
        
        if self.retention_count < 100 {
            return Err(ConfigValidationError::ValueTooLow {
                field: "retention_count".to_string(),
                min_value: 100,
                actual_value: self.retention_count as i64,
            });
        }
        
        Ok(())
    }
}
```

### Configuration Access Patterns

#### Global Access Pattern

For configuration that rarely changes and needs to be accessed throughout the codebase:

```rust
lazy_static! {
    pub static ref SERVER_CONFIG: Arc<ServerConfig> =
        Arc::new(ServerConfig::read().expect("Failed to initialize server config"));
}

// Usage throughout codebase
fn configure_network() {
    let port = SERVER_CONFIG.port;
    let address = &SERVER_CONFIG.address;
    // Configure network with these settings
}
```

#### Dependency Injection Pattern

For better testability and flexibility, especially with subsystem configs:

```rust
pub struct DatabaseService {
    config: DatabaseConfig,
    connection_pool: Pool<MySql>,
}

impl DatabaseService {
    pub fn new(config: DatabaseConfig) -> Result<Self, DbError> {
        let connection_pool = create_connection_pool(&config)?;
        Ok(Self { config, connection_pool })
    }
    
    pub fn get_connection_timeout(&self) -> Duration {
        Duration::from_secs(self.config.connection_timeout_seconds as u64)
    }
}

// Usage in application setup
let db_config = SERVER_CONFIG.database.clone();
let db_service = DatabaseService::new(db_config)?;
```

### Configuration File Management

#### Reading Configuration

```rust
pub fn load_configuration() -> Result<ServerConfig, ConfigError> {
    // Define search paths in order of precedence
    let search_paths = [
        // Environment-specific path
        std::env::var("OMNI_CONFIG_PATH").ok(),
        // User-specific path
        dirs::config_dir().map(|p| p.join("omni/config.json").to_string_lossy().to_string()),
        // Current directory
        Some("./config.json".to_string()),
        // System-wide path
        Some("/etc/omni/config.json".to_string()),
    ];
    
    // Try each path in order
    for path_opt in search_paths.iter().flatten() {
        if let Ok(content) = std::fs::read_to_string(path_opt) {
            if let Ok(config) = serde_json5::from_str::<ServerConfig>(&content) {
                return Ok(config);
            }
        }
    }
    
    // Fall back to defaults if no configuration file is found
    Ok(ServerConfig::default())
}
```

#### Writing Configuration

```rust
pub fn save_configuration(config: &ServerConfig) -> Result<(), ConfigError> {
    let config_path = std::env::var("OMNI_CONFIG_PATH")
        .unwrap_or_else(|_| "./config.json".to_string());
    
    // Create parent directories if they don't exist
    if let Some(parent) = std::path::Path::new(&config_path).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|_| ConfigError::FailedToWrite)?;
    }
    
    // Serialize the configuration to pretty-printed JSON
    let content = serde_json::to_string_pretty(config)
        .map_err(|_| ConfigError::ParseError)?;
    
    // Write the content to the file
    std::fs::write(&config_path, content)
        .map_err(|_| ConfigError::FailedToWrite)?;
    
    Ok(())
}
```

## Environment Variables Integration

### Overriding Configuration with Environment Variables

```rust
pub fn apply_environment_overrides(config: &mut ServerConfig) {
    // Override port if PORT environment variable is set
    if let Ok(port_str) = std::env::var("OMNI_PORT") {
        if let Ok(port) = port_str.parse::<u16>() {
            config.port = port;
        }
    }
    
    // Override address if ADDRESS environment variable is set
    if let Ok(address) = std::env::var("OMNI_ADDRESS") {
        config.address = address;
    }
    
    // Override database connection string if DB_URL environment variable is set
    if let Ok(db_url) = std::env::var("OMNI_DB_URL") {
        config.database.connection_string = db_url;
    }
    
    // Continue with other environment variable overrides...
}
```

### Environment Variable Naming Convention

When adding new configuration options that can be overridden by environment variables:

1. Prefix all environment variables with `OMNI_`
2. Use uppercase snake case for the rest of the name
3. Follow the hierarchical structure of the configuration

Examples:
- `OMNI_PORT` - Server port
- `OMNI_DB_URL` - Database connection string
- `OMNI_SECURITY_ENABLE_TLS` - TLS setting in security config

## Dynamic Configuration

### Implementing Runtime Configuration Changes

For settings that can be changed at runtime:

```rust
pub struct ConfigurationManager {
    current_config: RwLock<ServerConfig>,
    config_path: String,
    // Subscribers to be notified of config changes
    subscribers: Vec<Box<dyn Fn(&ServerConfig) + Send + Sync>>,
}

impl ConfigurationManager {
    pub fn new(initial_config: ServerConfig, config_path: String) -> Self {
        Self {
            current_config: RwLock::new(initial_config),
            config_path,
            subscribers: Vec::new(),
        }
    }
    
    pub fn get_config(&self) -> ServerConfig {
        self.current_config.read().unwrap().clone()
    }
    
    pub fn update_config<F>(&self, updater: F) -> Result<(), ConfigError>
    where
        F: FnOnce(&mut ServerConfig)
    {
        // Update the configuration
        {
            let mut config = self.current_config.write().unwrap();
            updater(&mut config);
            
            // Write the updated configuration to disk
            let content = serde_json::to_string_pretty(&*config)
                .map_err(|_| ConfigError::ParseError)?;
            
            std::fs::write(&self.config_path, content)
                .map_err(|_| ConfigError::FailedToWrite)?;
        }
        
        // Notify subscribers
        let config = self.get_config();
        for subscriber in &self.subscribers {
            subscriber(&config);
        }
        
        Ok(())
    }
    
    pub fn subscribe<F>(&mut self, callback: F)
    where
        F: Fn(&ServerConfig) + Send + Sync + 'static
    {
        self.subscribers.push(Box::new(callback));
    }
}
```

### Dynamic Configuration API Example

```rust
// API endpoint for updating server port
#[post("/api/v1/config/port", data = "<port>")]
pub async fn update_port(
    port: Json<u16>,
    config_manager: &State<ConfigurationManager>
) -> Result<Status, Status> {
    config_manager.update_config(|config| {
        config.port = port.0;
    }).map_err(|_| Status::InternalServerError)?;
    
    Ok(Status::Ok)
}
```

## Configuration Versioning

### Schema Versioning

For handling configuration schema changes between versions:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    // Schema version for compatibility checking
    pub schema_version: u32,
    
    // Configuration fields...
}

pub fn migrate_config(config: &str) -> Result<ServerConfig, ConfigError> {
    // Parse the JSON to a generic value first
    let value: serde_json::Value = serde_json::from_str(config)
        .map_err(|_| ConfigError::ParseError)?;
    
    // Extract the schema version
    let schema_version = value.get("schema_version")
        .and_then(|v| v.as_u64())
        .unwrap_or(1) as u32;
    
    // Handle different schema versions
    match schema_version {
        1 => migrate_v1_to_latest(value),
        2 => migrate_v2_to_latest(value),
        3 => serde_json::from_value(value).map_err(|_| ConfigError::ParseError),
        _ => Err(ConfigError::UnsupportedVersion),
    }
}

fn migrate_v1_to_latest(value: serde_json::Value) -> Result<ServerConfig, ConfigError> {
    // Parse as V1 config
    let v1_config: ServerConfigV1 = serde_json::from_value(value)
        .map_err(|_| ConfigError::ParseError)?;
    
    // Convert V1 to latest
    Ok(ServerConfig {
        schema_version: 3,
        port: v1_config.port,
        address: v1_config.address,
        // Map other fields with appropriate transformations
        // Fill new fields with default values
        instances: v1_config.instances,
        security: SecurityConfig::default(),
        database: DatabaseConfig {
            connection_string: v1_config.db_connection_string,
            max_connections: 10,
            connection_timeout_seconds: 30,
            query_timeout_seconds: 60,
        },
        // Other default configurations...
    })
}
```

## Testing Configuration

### Unit Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = ServerConfig::default();
        assert_eq!(config.port, 8000);
        assert_eq!(config.address, "127.0.0.1");
        assert!(config.highlight_sql);
    }
    
    #[test]
    fn test_serialization() {
        let config = ServerConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: ServerConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config.port, deserialized.port);
        assert_eq!(config.address, deserialized.address);
    }
    
    #[test]
    fn test_environment_overrides() {
        let mut config = ServerConfig::default();
        
        // Set environment variables for testing
        std::env::set_var("OMNI_PORT", "9000");
        std::env::set_var("OMNI_ADDRESS", "0.0.0.0");
        
        apply_environment_overrides(&mut config);
        
        assert_eq!(config.port, 9000);
        assert_eq!(config.address, "0.0.0.0");
        
        // Clean up
        std::env::remove_var("OMNI_PORT");
        std::env::remove_var("OMNI_ADDRESS");
    }
}
```

### Configuration for Testing

```rust
/// Creates a test configuration with predictable values.
///
/// This function is used in tests to create a configuration with
/// values appropriate for testing environments.
pub fn create_test_config() -> ServerConfig {
    ServerConfig {
        port: 0, // Use OS-assigned port
        address: "127.0.0.1".to_string(),
        highlight_sql: false, // Disable for predictable test output
        instances: vec![],
        database: DatabaseConfig {
            connection_string: "sqlite::memory:".to_string(),
            max_connections: 5,
            connection_timeout_seconds: 5,
            query_timeout_seconds: 5,
        },
        // Other test-appropriate settings...
    }
}
```

## Documentation Best Practices

### Configuration Documentation

Every configuration structure and field should be well-documented:

```rust
/// Configuration for managing log formats and destinations.
///
/// This structure controls how the application generates and stores logs,
/// including format, verbosity, and output destinations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Minimum log level to capture (trace, debug, info, warn, error)
    pub level: String,
    
    /// Whether to include timestamps in log messages
    pub include_timestamps: bool,
    
    /// Whether to output logs to a file
    pub log_to_file: bool,
    
    /// Path to the log file (only used if log_to_file is true)
    ///
    /// This can be absolute or relative to the application working directory.
    /// The application will attempt to create parent directories if they don't exist.
    pub log_file_path: String,
    
    /// Maximum size of log files before rotation (in megabytes)
    pub max_log_file_size_mb: u32,
    
    /// Number of rotated log files to keep
    pub log_file_history_count: u32,
}
```

### Configuration Comments in JSON

When generating or documenting example configuration files, include comments:

```jsonc
// Example config.json with comments
{
  // Network settings
  "port": 8000,
  "address": "127.0.0.1",
  
  // Feature flags
  "highlight_sql": true,
  
  // Cluster configuration
  "instances": [
    {
      "port": 8000,
      "address": "node1.example.com"
    },
    {
      "port": 8000,
      "address": "node2.example.com"
    }
  ],
  
  // Database settings
  "database": {
    "connection_string": "mysql://user:password@localhost/dbname",
    "max_connections": 20,
    "connection_timeout_seconds": 30,
    "query_timeout_seconds": 60
  }
}
```

## Best Practices and Guidelines

1. **Segregate Configurations**: Group related settings into separate structs
2. **Provide Meaningful Defaults**: All config options should have reasonable defaults
3. **Version Schemas**: Include schema version for future compatibility
4. **Validate Early**: Validate configurations at startup before using them
5. **Use Strong Types**: Avoid stringly-typed configurations
6. **Document Everything**: Every configuration option should be documented
7. **Support Multiple Sources**: Config files, environment variables, command line
8. **Make Configs Immutable**: Avoid changing config during runtime except through dedicated APIs
9. **Test Configurations**: Write tests for configuration loading and validation
10. **Include Examples**: Provide example configurations for common scenarios

## Advanced Topics

### Secure Configuration Handling

For sensitive configuration values:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    // Sensitive fields marked with serde attributes
    #[serde(serialize_with = "serialize_sensitive")]
    pub secret_key: String,
    
    #[serde(serialize_with = "serialize_sensitive")]
    pub database_password: String,
}

// Serialize sensitive data as "***" for logs and display
fn serialize_sensitive<S>(value: &String, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str("***")
}
```

### Remote Configuration

For fetching configuration from a central server:

```rust
pub async fn fetch_remote_config(
    endpoint: &str,
    token: &str
) -> Result<ServerConfig, ConfigError> {
    // Create HTTP client
    let client = reqwest::Client::new();
    
    // Make request to configuration endpoint
    let response = client.get(endpoint)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|_| ConfigError::RemoteConnectionFailed)?;
    
    // Parse the response body
    let config = response.json::<ServerConfig>()
        .await
        .map_err(|_| ConfigError::ParseError)?;
    
    Ok(config)
}
```