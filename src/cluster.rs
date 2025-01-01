use async_trait::async_trait;
use colored::Colorize;
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::state::SharedState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub id: Arc<str>,
    pub address: Arc<str>,
    pub port: u16,
}
#[derive(Debug)]
pub struct ClusterManager {
    pub state: Arc<RwLock<SharedState>>,
    pub nodes: Arc<RwLock<HashMap<Arc<str>, NodeInfo>>>,
}

impl ClusterManager {
    pub fn new(state: Arc<RwLock<SharedState>>) -> Self {
        Self {
            state,
            nodes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_node(&self, node: NodeInfo) {
        let node_uid= node.address.clone();
        println!("{}{}","CALLED REGISTER NODE FUNCTION WITH PARAMS OF:".white().on_red().bold(),node_uid.green());
        if self.nodes.read().await.contains_key(&node_uid) {
            println!("WE ALREADY HAD THIS NODE");
            return;
        }
        let size = {
            let mut nodes = self.nodes.write().await;
            println!("{}{}","ADDING NODE".white().on_red().bold().underline(),node_uid);
            nodes.insert(node_uid, node);
            let size = nodes.len();
            println!("Current node map: {:?}",nodes);
            size
        };
        let mut state = self.state.write().await;
        state.cluster_size = size;
    }

    pub async fn remove_node(&self, node_uid: Arc<str>, ) {

        debug!("{}{}","CALING REMOVE NODE FUNCTION WITH PARAMS OF:".white().on_green().bold(),node_uid.green());
        {
            let nodes_read = self.nodes.read().await;
            if !nodes_read.contains_key(&node_uid) {
                log::info!("Attempted to remove a node that does not exist");
                log::info!("Current nodes: {:?}", nodes_read);
                return;
            }
        }
        let mut nodes = self.nodes.write().await;
        log::info!("Removing node: {}",node_uid.white().on_green().bold());
        nodes.remove(&node_uid);
        
        let mut state = self.state.write().await;
        state.cluster_size = nodes.len();
    }

    pub async fn get_nodes(&self) -> Vec<NodeInfo> {
        let nodes = self.nodes.read().await;
        nodes.values().cloned().collect()
    }

    pub async fn get_nodes_and_self(&self) -> Vec<NodeInfo> {
        let state = self.state.read().await;
        let nodes = self.nodes.read().await;
    
        let mut all_nodes: Vec<NodeInfo> = nodes.values().cloned().collect();
        all_nodes.push(NodeInfo {
            id: format!("{}", state.node_id).into(),
            address: state.node_id.split(':').next().unwrap_or_default().into(),
            port: state.node_id.split(':').nth(1).unwrap_or_default().parse().unwrap_or(0),
        });
    
        println!("Current node ID: {}", state.node_id);
        println!("Known nodes: {:?}", nodes);
        
        all_nodes
    }

    pub async fn is_node_alive(&self, node_uid: Arc<str>) -> bool {
        let nodes = self.nodes.read().await;
        nodes.contains_key(&node_uid)
    }
}