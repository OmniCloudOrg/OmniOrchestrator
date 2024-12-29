use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub instances: Vec<Instance>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub port: u16,
    pub address: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 8000,
            instances: vec![Instance {
                port: 8000,
                address: "example.com".to_string(),
            }],
        }
    }   
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigError {
    FileNotFound,
    FailedToWrite,
    ParseError,
}
impl ServerConfig {
    pub fn get() -> Result<Self, ConfigError> {
        let config_path = "config.json";
        let config_content = match std::fs::read_to_string(config_path) {
            Ok(content) => content,
            Err(_) => {
                Self::write_default().expect("Failed to write default config");
                return Ok(ServerConfig::default());
            },
        };
        let config: ServerConfig = match serde_json::from_str(&config_content) {
            Ok(config) => config,
            Err(_) => return Err(ConfigError::ParseError),
        };
        Ok(config)
       
    }
    pub fn write(&self) -> Result<(), ConfigError> {
        let config_path = "config.json";
        let config_content = match serde_json::to_string_pretty(&self) {
            Ok(content) => content,
            Err(p) => return Err(ConfigError::ParseError),
        };
        match std::fs::write(config_path, config_content) {
            Ok(_) => return Ok(()),
            Err(_) => return Err(ConfigError::FailedToWrite),
        }
    }
    pub fn write_default() -> Result<(), ConfigError> {
        let config = ServerConfig::default();
        config.write()
    }
}