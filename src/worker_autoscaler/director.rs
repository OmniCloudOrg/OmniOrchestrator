use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use log::{info, error};
use async_trait::async_trait;

use super::error::AutoscalerError;
use super::node_types::{Node, NodeType};
use super::vm::{VM, VMState};

/// Interface for a director that manages VM operations on nodes
#[async_trait]
pub trait Director: Send + Sync + std::fmt::Debug {
    /// Get the unique ID of this director
    async fn id(&self) -> String;
    
    /// Get the nodes managed by this director
    async fn get_nodes(&self) -> Result<Vec<Node>, AutoscalerError>;
    
    /// Get information about a specific node
    async fn get_node(&self, node_id: &str) -> Result<Node, AutoscalerError>;
    
    /// Create a new VM on a specific node
    async fn create_vm(&self, node_id: &str, name: &str, cpu: u32, memory: u32, storage: u32) 
        -> Result<VM, AutoscalerError>;
    
    /// Terminate a VM
    async fn terminate_vm(&self, vm_id: &str) -> Result<(), AutoscalerError>;
    
    /// Get information about a specific VM
    async fn get_vm(&self, vm_id: &str) -> Result<VM, AutoscalerError>;
    
    /// Get all VMs managed by this director
    async fn get_vms(&self) -> Result<Vec<VM>, AutoscalerError>;
    
    /// Get metrics for a specific VM
    async fn get_vm_metrics(&self, vm_id: &str) -> Result<HashMap<String, f32>, AutoscalerError>;
}

/// Implementation of a cloud director (AWS, Azure, GCP)
#[derive(Debug)]
pub struct CloudDirector {
    /// Unique ID of this director
    id: String,
    /// Name of the cloud provider
    provider: String,
    /// Region for this cloud provider
    region: String,
    /// Simulated nodes for this director
    nodes: Arc<Mutex<HashMap<String, Node>>>,
    /// Simulated VMs for this director
    vms: Arc<Mutex<HashMap<String, VM>>>,
}

impl CloudDirector {
    /// Create a new cloud director
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
            vms: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl Director for CloudDirector {
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
    
    async fn create_vm(&self, node_id: &str, name: &str, cpu: u32, memory: u32, storage: u32) 
        -> Result<VM, AutoscalerError> {
        // Verify the node exists
        let mut nodes = self.nodes.lock().unwrap();
        let node = nodes.get_mut(node_id).ok_or_else(|| 
            AutoscalerError::NodeNotFound(format!("Node {} not found", node_id)))?;
        
        // Reserve capacity on the node
        node.reserve_capacity(cpu, memory, storage)?;
        
        // Create a new VM
        let vm_id = format!("{}-{}", node_id, uuid::Uuid::new_v4());
        let vm = VM::new(
            vm_id.clone(),
            name.to_string(),
            node_id.to_string(),
            cpu,
            memory,
            storage,
        );
        
        // Store the VM
        let mut vms = self.vms.lock().unwrap();
        vms.insert(vm_id, vm.clone());
        
        // Simulate API call to cloud provider
        info!("Cloud Director {} creating VM {} on node {}", self.id, vm.id, node_id);
        
        // In a real implementation, this would make API calls to the cloud provider
        
        Ok(vm)
    }
    
    async fn terminate_vm(&self, vm_id: &str) -> Result<(), AutoscalerError> {
        // Find the VM
        let mut vms = self.vms.lock().unwrap();
        let vm = vms.get_mut(vm_id).ok_or_else(|| 
            AutoscalerError::VMNotFound(format!("VM {} not found", vm_id)))?;
        
        // Update VM state
        vm.state = VMState::Terminating;
        vm.updated_at = Instant::now();
        
        // Simulate API call to cloud provider
        info!("Cloud Director {} terminating VM {}", self.id, vm_id);
        
        // Release capacity on the node
        let mut nodes = self.nodes.lock().unwrap();
        if let Some(node) = nodes.get_mut(&vm.node_id) {
            node.release_capacity(vm.cpu, vm.memory, vm.storage);
        }
        
        // In a real implementation, this would make API calls to the cloud provider
        
        // Mark VM as terminated
        vm.state = VMState::Terminated;
        
        Ok(())
    }
    
    async fn get_vm(&self, vm_id: &str) -> Result<VM, AutoscalerError> {
        let vms = self.vms.lock().unwrap();
        vms.get(vm_id).cloned().ok_or_else(|| 
            AutoscalerError::VMNotFound(format!("VM {} not found", vm_id)))
    }
    
    async fn get_vms(&self) -> Result<Vec<VM>, AutoscalerError> {
        let vms = self.vms.lock().unwrap();
        Ok(vms.values().cloned().collect())
    }
    
    async fn get_vm_metrics(&self, vm_id: &str) -> Result<HashMap<String, f32>, AutoscalerError> {
        // Verify the VM exists
        let vms = self.vms.lock().unwrap();
        let _vm = vms.get(vm_id).ok_or_else(|| 
            AutoscalerError::VMNotFound(format!("VM {} not found", vm_id)))?;
        
        // Simulate gathering metrics
        // In a real implementation, this would make API calls to the cloud provider
        let mut metrics = HashMap::new();
        metrics.insert("cpu_utilization".to_string(), 50.0 + (rand::random::<f32>() * 30.0 - 15.0));
        metrics.insert("memory_utilization".to_string(), 60.0 + (rand::random::<f32>() * 20.0 - 10.0));
        
        Ok(metrics)
    }
}