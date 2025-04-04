use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use log::{info, warn, error, debug};

use super::error::AutoscalerError;
use super::node_types::{Node, NodeType};
use super::vm::{VM, VMState, VMConfig, VMTemplate};
use super::director::Director;
use super::metrics::{MetricsCollector, MetricThreshold, ScalingAction};
use super::policy::ScalingPolicy;

/// Implementation of the worker autoscaler
#[derive(Debug)]
pub struct WorkerAutoscaler {
    /// The current number of worker nodes in the cluster
    pub current_worker_count: usize,
    /// The desired number of worker nodes in the cluster
    pub desired_worker_count: usize,
    /// The scaling policy to be used for autoscaling
    pub scaling_policy: ScalingPolicy,
    /// The last time a scaling action was performed
    pub last_scaling_time: Instant,
    /// The last time metrics were evaluated
    pub last_evaluation_time: Instant,
    /// Time when scale down evaluation started
    pub scale_down_evaluation_start: Option<Instant>,
    /// History of scaling actions
    scaling_history: Vec<(Instant, ScalingAction)>,
    /// Directors managing the nodes
    directors: HashMap<String, Arc<dyn Director>>,
    /// VMs running on nodes, by VM ID
    vms: HashMap<String, VM>,
    /// Node resource information
    nodes: HashMap<String, Node>,
    /// Default VM configuration
    default_vm_config: VMConfig,
    /// Default node preference for new VMs
    preferred_node_type: NodeType,
    /// VM template for creating new VMs
    vm_template: VMTemplate,
    /// Metrics collector for gathering VM and node metrics
    metrics_collector: Option<Arc<dyn MetricsCollector>>,
}

impl WorkerAutoscaler {
    /// Creates a new instance of the WorkerAutoscaler.
    ///
    /// Initializes the autoscaler with the current number of workers, desired number of workers,
    /// and the scaling policy to be used for autoscaling.
    ///
    /// # Arguments
    ///
    /// * `current_worker_count` - The current number of worker nodes in the cluster.
    /// * `desired_worker_count` - The desired number of worker nodes in the cluster.
    /// * `scaling_policy` - The scaling policy to be used for autoscaling.
    pub fn new(current_worker_count: usize, desired_worker_count: usize, scaling_policy: ScalingPolicy) -> Self {
        info!("Initializing WorkerAutoscaler with current_count={}, desired_count={}, policy={:?}", 
             current_worker_count, desired_worker_count, scaling_policy);
        
        Self {
            current_worker_count,
            desired_worker_count,
            scaling_policy,
            last_scaling_time: Instant::now(),
            last_evaluation_time: Instant::now(),
            scale_down_evaluation_start: None,
            scaling_history: Vec::with_capacity(100),
            directors: HashMap::new(),
            vms: HashMap::new(),
            nodes: HashMap::new(),
            default_vm_config: VMConfig::default(),
            preferred_node_type: NodeType::Cloud, // Default to cloud nodes
            vm_template: VMTemplate::default(),
            metrics_collector: None,
        }
    }
    
    /// Registers a new director with the autoscaler.
    ///
    /// Directors are responsible for managing VMs and nodes in different environments.
    ///
    /// # Arguments
    /// * `director` - The director implementation to add
    pub fn add_director(&mut self, director: Arc<dyn Director>) {
        let director_id = futures::executor::block_on(director.id());
        info!("Adding director {} to autoscaler", director_id);
        self.directors.insert(director_id, director);
    }
    
    /// Configures the metrics collector for gathering performance metrics.
    ///
    /// # Arguments
    /// * `collector` - The metrics collector implementation to use
    pub fn set_metrics_collector(&mut self, collector: Arc<dyn MetricsCollector>) {
        info!("Setting metrics collector for autoscaler");
        self.metrics_collector = Some(collector);
    }
    
    /// Sets the VM template to use when creating new worker nodes.
    ///
    /// # Arguments
    /// * `template` - The template containing VM configuration details
    pub fn set_vm_template(&mut self, template: VMTemplate) {
        info!("Setting VM template: {:?}", template);
        self.vm_template = template;
    }
    
    /// Configures the preferred type of node (e.g., Cloud, Edge) for new VMs.
    ///
    /// # Arguments
    /// * `node_type` - The preferred node type to use
    pub fn set_preferred_node_type(&mut self, node_type: NodeType) {
        info!("Setting preferred node type: {:?}", node_type);
        self.preferred_node_type = node_type;
    }
    
    /// Queries all registered directors to discover available nodes.
    ///
    /// Updates the internal node registry with the discovered nodes.
    ///
    /// # Returns
    /// * `Result<(), AutoscalerError>` - Success or error if discovery fails
    pub async fn discover_nodes(&mut self) -> Result<(), AutoscalerError> {
        info!("Discovering nodes from all directors");
        
        for (director_id, director) in &self.directors {
            info!("Discovering nodes from director {}", director_id);
            
            match director.get_nodes().await {
                Ok(nodes) => {
                    for node in nodes {
                        info!("Found node {} ({}) from director {}", node.id, node.name, director_id);
                        self.nodes.insert(node.id.clone(), node);
                    }
                },
                Err(err) => {
                    error!("Failed to discover nodes from director {}: {}", director_id, err);
                }
            }
        }
        
        info!("Discovered {} nodes from directors", self.nodes.len());
        Ok(())
    }
    
    /// Queries all registered directors to discover running VMs.
    ///
    /// Updates the internal VM registry and current worker count.
    ///
    /// # Returns
    /// * `Result<(), AutoscalerError>` - Success or error if discovery fails
    pub async fn discover_vms(&mut self) -> Result<(), AutoscalerError> {
        info!("Discovering VMs from all directors");
        self.vms.clear();
        
        for (director_id, director) in &self.directors {
            info!("Discovering VMs from director {}", director_id);
            
            match director.get_vms().await {
                Ok(vms) => {
                    for vm in vms {
                        if vm.state == VMState::Running {
                            info!("Found VM {} ({}) on node {}", vm.id, vm.name, vm.node_id);
                            self.vms.insert(vm.id.clone(), vm);
                        }
                    }
                },
                Err(err) => {
                    error!("Failed to discover VMs from director {}: {}", director_id, err);
                }
            }
        }
        
        // Update current worker count based on discovered VMs
        self.current_worker_count = self.vms
            .values()
            .filter(|vm| vm.state == VMState::Running)
            .count();
            
        info!("Discovered {} running VMs from directors", self.current_worker_count);
        Ok(())
    }

    /// Locates a node with sufficient resources to host a new VM.
    ///
    /// # Arguments
    /// * `cpu` - Required CPU cores
    /// * `memory` - Required memory in MB
    /// * `storage` - Required storage in GB
    ///
    /// # Returns
    /// * `Option<String>` - ID of suitable node if found
    async fn find_available_node(&self, cpu: u32, memory: u32, storage: u32) -> Option<String> {
        // First, try to find a node of the preferred type
        for (node_id, node) in &self.nodes {
            if node.node_type == self.preferred_node_type && node.online && node.has_capacity(cpu, memory, storage) {
                return Some(node_id.clone());
            }
        }
        
        // If no preferred node is available, try any node
        for (node_id, node) in &self.nodes {
            if node.online && node.has_capacity(cpu, memory, storage) {
                return Some(node_id.clone());
            }
        }
        
        None
    }
    
    /// Retrieves the director responsible for managing a specific node.
    ///
    /// # Arguments
    /// * `node_id` - ID of the node
    ///
    /// # Returns
    /// * `Option<Arc<dyn Director>>` - Reference to responsible director if found
    fn get_node_director(&self, node_id: &str) -> Option<Arc<dyn Director>> {
        let node = self.nodes.get(node_id)?;
        self.directors.get(&node.director_id).cloned()
    }

    /// Increases the number of worker nodes by creating new VMs.
    ///
    /// Follows scaling policy limits and resource availability constraints.
    ///
    /// # Returns
    /// * `Result<usize, AutoscalerError>` - Number of workers added or error
    pub async fn scale_up(&mut self) -> Result<usize, AutoscalerError> {
        let old_count = self.current_worker_count;
        let increment = self.scaling_policy.scale_up_increment;
        let target_count = std::cmp::min(
            old_count + increment,
            self.scaling_policy.max_worker_count
        );
        
        let to_add = target_count - old_count;
        
        if to_add == 0 {
            info!("Already at maximum worker count ({}), not scaling up", old_count);
            return Ok(0);
        }
        
        info!("Scaling up from {} to {} workers (adding {})", 
              old_count, target_count, to_add);
        
        let mut added = 0;
        
        // Create new VMs
        for i in 0..to_add {
            let vm_name = format!("{}-{}", self.vm_template.base_name, uuid::Uuid::new_v4().to_string().split('-').next().unwrap_or(""));
            
            // Find a node with available capacity
            let node_id = match self.find_available_node(
                self.vm_template.config.cpu,
                self.vm_template.config.memory,
                self.vm_template.config.storage
            ).await {
                Some(id) => id,
                None => {
                    warn!("No nodes with available capacity, stopped scaling up after adding {} VMs", added);
                    break;
                }
            };
            
            // Get the director for this node
            let director = match self.get_node_director(&node_id) {
                Some(director) => director,
                None => {
                    error!("Director not found for node {}, cannot create VM", node_id);
                    continue;
                }
            };
            
            // Request VM creation from the director
            info!("Creating VM {} on node {} (VM {}/{})", 
                  vm_name, node_id, i + 1, to_add);
                  
            match director.create_vm(
                &node_id,
                &vm_name,
                self.vm_template.config.cpu,
                self.vm_template.config.memory,
                self.vm_template.config.storage
            ).await {
                Ok(vm) => {
                    info!("Created VM {} successfully", vm.id);
                    self.vms.insert(vm.id.clone(), vm);
                    added += 1;
                },
                Err(err) => {
                    error!("Failed to create VM on node {}: {}", node_id, err);
                }
            }
        }
        
        // Update current worker count
        self.current_worker_count = old_count + added;
        
        info!("Scaled up from {} to {} workers (added {})", 
              old_count, self.current_worker_count, added);
        
        self.last_scaling_time = Instant::now();
        self.scale_down_evaluation_start = None;
        self.scaling_history.push((Instant::now(), ScalingAction::ScaleUp));
        
        // Trim history if needed
        if self.scaling_history.len() > 100 {
            self.scaling_history.remove(0);
        }
        
        Ok(added)
    }

    /// Decreases the number of worker nodes by terminating VMs.
    ///
    /// Follows scaling policy limits and selects oldest VMs for termination.
    ///
    /// # Returns
    /// * `Result<usize, AutoscalerError>` - Number of workers removed or error
    pub async fn scale_down(&mut self) -> Result<usize, AutoscalerError> {
        let old_count = self.current_worker_count;
        
        // Calculate the maximum number of workers that can be removed based on percentage
        let max_removal_by_percentage = 
            (old_count as f32 * self.scaling_policy.max_scale_down_percentage).floor() as usize;
        
        let max_removal = std::cmp::min(
            self.scaling_policy.scale_down_increment,
            max_removal_by_percentage
        );
        
        let target_count = std::cmp::max(
            old_count.saturating_sub(max_removal),
            self.scaling_policy.min_worker_count
        );
        
        let to_remove = old_count - target_count;
        
        if to_remove == 0 {
            info!("Already at minimum worker count ({}), not scaling down", old_count);
            return Ok(0);
        }
        
        info!("Scaling down from {} to {} workers (removing {})", 
              old_count, target_count, to_remove);
              
        // Find candidates for termination (sort by creation time, oldest first)
        let mut candidates: Vec<_> = self.vms
            .values()
            .cloned()
            .filter(|vm| vm.state == VMState::Running)
            .collect();
            
        candidates.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        
        // Limit to the number we want to remove
        candidates.truncate(to_remove);
        
        let mut removed = 0;
        
        // Terminate the selected VMs
        for vm in &candidates {
            // Find the director for this VM's node
            let director = match self.get_node_director(&vm.node_id) {
                Some(director) => director,
                None => {
                    error!("Director not found for node {}, cannot terminate VM {}", 
                          vm.node_id, vm.id);
                    continue;
                }
            };
            
            // Request VM termination
            info!("Terminating VM {} on node {} (VM {}/{})", 
                 vm.id, vm.node_id, removed + 1, to_remove);
                 
            match director.terminate_vm(&vm.id).await {
                Ok(_) => {
                    info!("Terminated VM {} successfully", vm.id);
                    // Remove from our list
                    self.vms.remove(&vm.id);
                    removed += 1;
                },
                Err(err) => {
                    error!("Failed to terminate VM {}: {}", vm.id, err);
                }
            }
        }
        
        // Update current worker count
        self.current_worker_count = old_count - removed;
        
        info!("Scaled down from {} to {} workers (removed {})", 
              old_count, self.current_worker_count, removed);
        
        self.last_scaling_time = Instant::now();
        self.scale_down_evaluation_start = None;
        self.scaling_history.push((Instant::now(), ScalingAction::ScaleDown));
        
        // Trim history if needed
        if self.scaling_history.len() > 100 {
            self.scaling_history.remove(0);
        }
        
        Ok(removed)
    }

    /// Evaluates metrics to determine if scaling up is needed.
    ///
    /// # Arguments
    /// * `current_metrics` - Current system metrics
    ///
    /// # Returns
    /// * `Result<Option<String>, AutoscalerError>` - Name of triggering metric if scaling needed
    fn should_scale_up(&self, current_metrics: &HashMap<String, f32>) -> Result<Option<String>, AutoscalerError> {
        for (metric, threshold) in &self.scaling_policy.metrics_thresholds {
            match current_metrics.get(metric) {
                Some(value) => {
                    let should_scale = match threshold {
                        MetricThreshold::Float(thresh) => *value > *thresh,
                        MetricThreshold::Integer(thresh) => *value > *thresh as f32,
                        MetricThreshold::Boolean(thresh) => *thresh && *value > 0.5,
                    };
                    
                    if should_scale {
                        debug!("Scale up condition met: {} = {} exceeds threshold {:?}", 
                              metric, value, threshold);
                        return Ok(Some(metric.clone()));
                    }
                },
                None => {
                    warn!("Metric not found in current metrics: {}", metric);
                    // Continue checking other metrics instead of failing
                }
            }
        }
        
        Ok(None)
    }

    /// Evaluates metrics to determine if scaling down is needed.
    ///
    /// # Arguments
    /// * `current_metrics` - Current system metrics
    ///
    /// # Returns
    /// * `Result<Option<String>, AutoscalerError>` - Name of triggering metric if scaling needed
    fn should_scale_down(&self, current_metrics: &HashMap<String, f32>) -> Result<Option<String>, AutoscalerError> {
        for (metric, threshold) in &self.scaling_policy.metrics_thresholds {
            match current_metrics.get(metric) {
                Some(value) => {
                    let should_scale = match threshold {
                        MetricThreshold::Float(thresh) => *value < *thresh * 0.7, // Add buffer to prevent flapping
                        MetricThreshold::Integer(thresh) => *value < *thresh as f32 * 0.7,
                        MetricThreshold::Boolean(thresh) => !*thresh && *value < 0.3,
                    };
                    
                    if should_scale {
                        debug!("Scale down condition met: {} = {} below threshold {:?}", 
                              metric, value, threshold);
                        return Ok(Some(metric.clone()));
                    }
                },
                None => {
                    warn!("Metric not found in current metrics: {}", metric);
                    // Continue checking other metrics instead of failing
                }
            }
        }
        
        Ok(None)
    }

    /// Retrieves the recent scaling operations history.
    ///
    /// # Returns
    /// * `&[(Instant, ScalingAction)]` - List of timestamps and scaling actions
    pub fn get_scaling_history(&self) -> &[(Instant, ScalingAction)] {
        &self.scaling_history
    }
    

    /// Calculates current autoscaling statistics and metrics.
    ///
    /// # Returns
    /// * `HashMap<String, f32>` - Map of statistic names to values
    pub fn get_scaling_stats(&self) -> HashMap<String, f32> {
        let mut stats = HashMap::new();
        
        // Calculate time since last scaling action
        stats.insert(
            "time_since_last_scaling_secs".to_string(), 
            self.last_scaling_time.elapsed().as_secs() as f32
        );
        
        // Calculate time since last evaluation
        stats.insert(
            "time_since_last_evaluation_secs".to_string(), 
            self.last_evaluation_time.elapsed().as_secs() as f32
        );
        
        // Calculate current utilization percentage 
        stats.insert(
            "worker_utilization_percentage".to_string(),
            (self.current_worker_count as f32 / self.scaling_policy.max_worker_count as f32) * 100.0
        );
        
        // Count recent scaling actions
        let now = Instant::now();
        let one_hour_ago = now - Duration::from_secs(3600);
        
        let scale_ups_last_hour = self.scaling_history
            .iter()
            .filter(|(time, action)| *time >= one_hour_ago && *action == ScalingAction::ScaleUp)
            .count() as f32;
            
        let scale_downs_last_hour = self.scaling_history
            .iter()
            .filter(|(time, action)| *time >= one_hour_ago && *action == ScalingAction::ScaleDown)
            .count() as f32;
            
        stats.insert("scale_ups_last_hour".to_string(), scale_ups_last_hour);
        stats.insert("scale_downs_last_hour".to_string(), scale_downs_last_hour);
        
        stats
    }


    /// Evaluates current metrics and determines if scaling is needed.
    ///
    /// Considers cooldown periods and scaling limits when making decisions.
    ///
    /// # Arguments
    /// * `current_metrics` - Current system metrics
    ///
    /// # Returns
    /// * `Result<ScalingAction, AutoscalerError>` - Required scaling action or error
    pub fn check_scaling(&mut self, current_metrics: &HashMap<String, f32>) -> Result<ScalingAction, AutoscalerError> {
        // Don't scale if autoscaling is disabled
        if !self.scaling_policy.autoscaling_enabled {
            return Ok(ScalingAction::NoAction);
        }

        // Don't scale if we're in cooldown period
        if self.last_scaling_time.elapsed() < self.scaling_policy.cooldown_period {
            return Ok(ScalingAction::NoAction);
        }

        // Check if we need to scale up
        if let Some(_metric) = self.should_scale_up(current_metrics)? {
            if self.current_worker_count < self.scaling_policy.max_worker_count {
                // Reset scale down evaluation
                self.scale_down_evaluation_start = None;
                return Ok(ScalingAction::ScaleUp);
            }
        }

        // Check if we need to scale down
        if let Some(_metric) = self.should_scale_down(current_metrics)? {
            // Start scale down evaluation if not already started
            if self.scale_down_evaluation_start.is_none() {
                self.scale_down_evaluation_start = Some(Instant::now());
                return Ok(ScalingAction::NoAction);
            }

            // Check if we've waited long enough
            if self.scale_down_evaluation_start.unwrap().elapsed() >= self.scaling_policy.scale_down_delay {
                if self.current_worker_count > self.scaling_policy.min_worker_count {
                    return Ok(ScalingAction::ScaleDown);
                }
            }
        } else {
            // Reset scale down evaluation if metrics no longer indicate need to scale down
            self.scale_down_evaluation_start = None;
        }

        Ok(ScalingAction::NoAction)
    }
}

// Example of how to use the autoscaler with custom metrics
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_autoscaler_scaling_up() {
        let mut policy = ScalingPolicy::default();
        policy.max_worker_count = 5;
        policy.min_worker_count = 1;
        policy.cooldown_period = Duration::from_secs(0); // No cooldown for testing
        
        // Set up CPU threshold
        let mut thresholds = HashMap::new();
        thresholds.insert("cpu_utilization".to_string(), MetricThreshold::Float(70.0));
        policy.metrics_thresholds = thresholds;
        
        let mut autoscaler = WorkerAutoscaler::new(1, 1, policy);
        
        // Test with CPU above threshold
        let mut metrics = HashMap::new();
        metrics.insert("cpu_utilization".to_string(), 75.0);
        
        let result = autoscaler.check_scaling(&metrics).unwrap();
        assert_eq!(result, ScalingAction::ScaleUp);
    }
    
    #[test]
    fn test_autoscaler_scaling_down() {
        let mut policy = ScalingPolicy::default();
        policy.max_worker_count = 5;
        policy.min_worker_count = 1;
        policy.cooldown_period = Duration::from_secs(0); // No cooldown for testing
        policy.scale_down_delay = Duration::from_secs(0); // No delay for testing
        
        // Set up CPU threshold
        let mut thresholds = HashMap::new();
        thresholds.insert("cpu_utilization".to_string(), MetricThreshold::Float(30.0));
        policy.metrics_thresholds = thresholds;
        
        let mut autoscaler = WorkerAutoscaler::new(3, 3, policy);
        
        // Test with CPU below threshold
        let mut metrics = HashMap::new();
        metrics.insert("cpu_utilization".to_string(), 20.0);
        
        // First call starts the evaluation
        let result = autoscaler.check_scaling(&metrics).unwrap();
        assert_eq!(result, ScalingAction::NoAction);
        
        // Force scale down evaluation to start
        autoscaler.scale_down_evaluation_start = Some(Instant::now() - Duration::from_secs(1));
        
        // Second call should scale down
        let result = autoscaler.check_scaling(&metrics).unwrap();
        assert_eq!(result, ScalingAction::ScaleDown);
    }
}