use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use log::{info, error};
use async_trait::async_trait;
use rand;

use super::error::AutoscalerError;
use super::node_types::{Node, NodeType};
use super::app::{AppInstance, AppInstanceState};

/// Interface for an agent that manages app instance operations on nodes
#[async_trait]
pub trait Agent: Send + Sync + std::fmt::Debug {
    /// Get the unique ID of this agent
    async fn id(&self) -> String;
    
    /// Get the nodes managed by this agent
    async fn get_nodes(&self) -> Result<Vec<Node>, AutoscalerError>;
    
    /// Get information about a specific node
    async fn get_node(&self, node_id: &str) -> Result<Node, AutoscalerError>;
    
    /// Create a new app instance on a specific node
    async fn create_instance(&self, node_id: &str, name: &str, cpu: u32, memory: u32, storage: u32) 
        -> Result<AppInstance, AutoscalerError>;
    
    /// Terminate an app instance
    async fn terminate_instance(&self, instance_id: &str) -> Result<(), AutoscalerError>;
    
    /// Get information about a specific app instance
    async fn get_instance(&self, instance_id: &str) -> Result<AppInstance, AutoscalerError>;
    
    /// Get all app instances managed by this agent
    async fn get_instances(&self) -> Result<Vec<AppInstance>, AutoscalerError>;
    
    /// Get metrics for a specific app instance
    async fn get_instance_metrics(&self, instance_id: &str) -> Result<HashMap<String, f32>, AutoscalerError>;
}

/// Implementation of a cloud agent (AWS, Azure, GCP)
#[derive(Debug)]
pub struct CloudAgent {
    /// Unique ID of this agent
    id: String,
    /// Name of the cloud provider
    provider: String,
    /// Region for this cloud provider
    region: String,
    /// Simulated nodes for this agent
    nodes: Arc<Mutex<HashMap<String, Node>>>,
    /// Simulated app instances for this agent
    instances: Arc<Mutex<HashMap<String, AppInstance>>>,
}

impl CloudAgent {
    /// Create a new cloud agent
    pub fn new(id: String, provider: String, region: String) -> Self {
        let mut nodes = HashMap::new();
        
        // Create a single "infinite" capacity cloud node
        let node_id = format!("{}-{}-node", provider, region);
        let node = Node::new_cloud(
            node_id.clone(),
            format!("{} {} Default Node", provider, region),
            id.clone(),
        );
        
        nodes.insert(node_id, node);
        
        Self {
            id,
            provider,
            region,
            nodes: Arc::new(Mutex::new(nodes)),
            instances: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl Agent for CloudAgent {
    async fn id(&self) -> String {
        self.id.clone()
    }
    
    async fn get_nodes(&self) -> Result<Vec<Node>, AutoscalerError> {
        let nodes = self.nodes.lock().unwrap();
        Ok(nodes.values().cloned().collect())
    }
    
    async fn get_node(&self, node_id: &str) -> Result<Node, AutoscalerError> {
        let nodes = self.nodes.lock().unwrap();
        nodes.get(node_id).cloned().ok_or_else(|| 
            AutoscalerError::NodeNotFound(format!("Node {} not found", node_id)))
    }
    
    async fn create_instance(&self, node_id: &str, name: &str, cpu: u32, memory: u32, storage: u32) 
        -> Result<AppInstance, AutoscalerError> {
        // Verify the node exists
        let mut nodes = self.nodes.lock().unwrap();
        let node = nodes.get_mut(node_id).ok_or_else(|| 
            AutoscalerError::NodeNotFound(format!("Node {} not found", node_id)))?;
        
        // Reserve capacity on the node
        node.reserve_capacity(cpu, memory, storage)?;
        
        // Create a new app instance
        let instance_id = format!("{}-{}", node_id, uuid::Uuid::new_v4());
        let instance = AppInstance::new(
            instance_id.clone(),
            name.to_string(),
            node_id.to_string(),
            cpu,
            memory,
            storage,
        );
        
        // Store the app instance
        let mut instances = self.instances.lock().unwrap();
        instances.insert(instance_id, instance.clone());
        
        // Simulate API call to cloud provider
        info!("Cloud Agent {} creating app instance {} on node {}", self.id, instance.id, node_id);
        
        // In a real implementation, this would make API calls to the cloud provider
        
        Ok(instance)
    }
    
    async fn terminate_instance(&self, instance_id: &str) -> Result<(), AutoscalerError> {
        // Find the app instance
        let mut instances = self.instances.lock().unwrap();
        let instance = instances.get_mut(instance_id).ok_or_else(|| 
            AutoscalerError::InstanceNotFound(format!("App instance {} not found", instance_id)))?;
        
        // Update app instance state
        instance.state = AppInstanceState::Terminating;
        instance.updated_at = Instant::now();
        
        // Simulate API call to cloud provider
        info!("Cloud Agent {} terminating app instance {}", self.id, instance_id);
        
        // Release capacity on the node
        let mut nodes = self.nodes.lock().unwrap();
        if let Some(node) = nodes.get_mut(&instance.node_id) {
            node.release_capacity(instance.cpu, instance.memory, instance.storage);
        }
        
        // In a real implementation, this would make API calls to the cloud provider
        
        // Mark app instance as terminated
        instance.state = AppInstanceState::Terminated;
        
        Ok(())
    }
    
    async fn get_instance(&self, instance_id: &str) -> Result<AppInstance, AutoscalerError> {
        let instances = self.instances.lock().unwrap();
        instances.get(instance_id).cloned().ok_or_else(|| 
            AutoscalerError::InstanceNotFound(format!("App instance {} not found", instance_id)))
    }
    
    async fn get_instances(&self) -> Result<Vec<AppInstance>, AutoscalerError> {
        let instances = self.instances.lock().unwrap();
        Ok(instances.values().cloned().collect())
    }
    
    async fn get_instance_metrics(&self, instance_id: &str) -> Result<HashMap<String, f32>, AutoscalerError> {
        // Verify the app instance exists
        let instances = self.instances.lock().unwrap();
        let _instance = instances.get(instance_id).ok_or_else(|| 
            AutoscalerError::InstanceNotFound(format!("App instance {} not found", instance_id)))?;
        
        // Simulate gathering metrics
        // In a real implementation, this would make API calls to the cloud provider
        let mut metrics = HashMap::new();
        metrics.insert("cpu_utilization".to_string(), 50.0 + (rand::random::<f32>() * 30.0 - 15.0));
        metrics.insert("memory_utilization".to_string(), 60.0 + (rand::random::<f32>() * 20.0 - 10.0));
        
        Ok(metrics)
    }
}