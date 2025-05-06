// This state file is being written to in leu od writing to the actual database for now
// TODO: Change all areas that write to this to instead write to the database to use the database

// CRITICAL CHANGE: Define the application state as global statics
// This completely bypasses Rocket's state management system
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use super::models::{CloudConfig, HostDeploymentStatus};

lazy_static! {
    pub static ref GLOBAL_CONFIGS: Arc<RwLock<HashMap<String, CloudConfig>>> = 
        Arc::new(RwLock::new(HashMap::new()));
    pub static ref GLOBAL_DEPLOYMENT_STATUS: Arc<RwLock<HashMap<String, Vec<HostDeploymentStatus>>>> = 
        Arc::new(RwLock::new(HashMap::new()));
}
