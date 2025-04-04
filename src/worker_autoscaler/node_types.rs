use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use super::AutoscalerError;

/// Types of nodes that can host VMs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeType {
    /// Physical server with capacity constraints
    Physical,
    /// Cloud provider (AWS, Azure, GCP) with "infinite" capacity
    Cloud,
    /// Edge node with limited resources
    Edge,
}

/// Represents a physical or cloud node that can host VMs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// Unique identifier for the node
    pub id: String,
    /// Human-readable name for the node
    pub name: String,
    /// Type of node (physical, cloud, etc.)
    pub node_type: NodeType,
    /// Total CPU cores available on this node
    pub total_cpu: u32,
    /// Total memory in MB available on this node
    pub total_memory: u32,
    /// Total storage in GB available on this node
    pub total_storage: u32,
    /// CPU cores currently allocated to VMs
    pub allocated_cpu: u32,
    /// Memory in MB currently allocated to VMs
    pub allocated_memory: u32,
    /// Storage in GB currently allocated to VMs
    pub allocated_storage: u32,
    /// Whether the node is online and available
    pub online: bool,
    /// The director responsible for this node
    pub director_id: String,
    /// Additional properties specific to this node
    pub properties: HashMap<String, String>,
}

impl Node {
    /// Create a new node with the specified capacity
    pub fn new(id: String, name: String, node_type: NodeType, director_id: String, 
               total_cpu: u32, total_memory: u32, total_storage: u32) -> Self {
        Self {
            id,
            name,
            node_type,
            total_cpu,
            total_memory,
            total_storage,
            allocated_cpu: 0,
            allocated_memory: 0,
            allocated_storage: 0,
            online: true,
            director_id,
            properties: HashMap::new(),
        }
    }
    
    /// Create a new cloud node with "infinite" capacity
    pub fn new_cloud(id: String, name: String, director_id: String) -> Self {
        Self {
            id,
            name,
            node_type: NodeType::Cloud,
            // High values to represent "infinite" capacity
            total_cpu: u32::MAX / 2,
            total_memory: u32::MAX / 2,
            total_storage: u32::MAX / 2,
            allocated_cpu: 0,
            allocated_memory: 0,
            allocated_storage: 0,
            online: true,
            director_id,
            properties: HashMap::new(),
        }
    }
    
    /// Check if the node has enough capacity for the specified resources
    pub fn has_capacity(&self, cpu: u32, memory: u32, storage: u32) -> bool {
        // Cloud nodes always have capacity
        if self.node_type == NodeType::Cloud {
            return true;
        }
        
        self.allocated_cpu + cpu <= self.total_cpu &&
        self.allocated_memory + memory <= self.total_memory &&
        self.allocated_storage + storage <= self.total_storage
    }
    
    /// Reserve capacity on this node
    pub fn reserve_capacity(&mut self, cpu: u32, memory: u32, storage: u32) -> Result<(), AutoscalerError> {
        if !self.has_capacity(cpu, memory, storage) {
            return Err(AutoscalerError::InsufficientCapacity(format!(
                "Node {} does not have enough capacity for CPU:{}, Memory:{}MB, Storage:{}GB",
                self.id, cpu, memory, storage
            )));
        }
        
        self.allocated_cpu += cpu;
        self.allocated_memory += memory;
        self.allocated_storage += storage;
        
        Ok(())
    }
    
    /// Release capacity on this node
    pub fn release_capacity(&mut self, cpu: u32, memory: u32, storage: u32) {
        self.allocated_cpu = self.allocated_cpu.saturating_sub(cpu);
        self.allocated_memory = self.allocated_memory.saturating_sub(memory);
        self.allocated_storage = self.allocated_storage.saturating_sub(storage);
    }
    
    /// Get the percentage of CPU capacity in use
    pub fn cpu_utilization(&self) -> f32 {
        if self.total_cpu == 0 {
            return 0.0;
        }
        (self.allocated_cpu as f32 / self.total_cpu as f32) * 100.0
    }
    
    /// Get the percentage of memory capacity in use
    pub fn memory_utilization(&self) -> f32 {
        if self.total_memory == 0 {
            return 0.0;
        }
        (self.allocated_memory as f32 / self.total_memory as f32) * 100.0
    }
    
    /// Get the percentage of storage capacity in use
    pub fn storage_utilization(&self) -> f32 {
        if self.total_storage == 0 {
            return 0.0;
        }
        (self.allocated_storage as f32 / self.total_storage as f32) * 100.0
    }
}