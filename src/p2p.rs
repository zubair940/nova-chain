use crate::models::Peer;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json;

pub struct P2PNetwork {
    pub peers: Arc<RwLock<HashMap<String, Peer>>>,
    pub seed_nodes: Vec<String>,
    pub node_address: String,
    pub connected_peers: Arc<RwLock<HashSet<String>>>,
}

impl P2PNetwork {
    pub fn new(node_address: String) -> Self {
        let seed_nodes = vec![
            "127.0.0.1:8080".to_string(),
            "127.0.0.1:8081".to_string(), 
            "127.0.0.1:8082".to_string(),
            "127.0.0.1:8083".to_string(),
        ];
        
        P2PNetwork {
            peers: Arc::new(RwLock::new(HashMap::new())),
            seed_nodes,
            node_address,
            connected_peers: Arc::new(RwLock::new(HashSet::new())),
        }
    }
    
    // 🔥 FEATURE 7: P2P PEERING/DISCOVERY
    pub async fn start_peer_discovery(&self) {
        println!("🔍 Starting P2P Peer Discovery...");
        
        // Seed nodes se connect karo
        for seed_node in &self.seed_nodes {
            if seed_node != &self.node_address {
                self.connect_to_seed_node(seed_node.clone()).await;
            }
        }
        
        // Periodic peer discovery start karo
        self.start_periodic_discovery().await;
    }
    
    async fn connect_to_seed_node(&self, seed_address: String) {
        println!("🌱 Connecting to seed node: {}", seed_address);
        
        let peer = Peer {
            address: seed_address.clone(),
            version: "1.0.0".to_string(),
            height: 0,
        };
        
        // Peer ko add karo
        self.peers.write().await.insert(seed_address.clone(), peer);
        self.connected_peers.write().await.insert(seed_address);
        
        println!("✅ Connected to seed node: {}", seed_address);
    }
    
    async fn start_periodic_discovery(&self) {
        let peers_clone = self.peers.clone();
        let connected_peers_clone = self.connected_peers.clone();
        let node_address = self.node_address.clone();
        
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await; // Every 60 seconds
                
                // Discover new peers from existing peers
                P2PNetwork::discover_from_peers(
                    peers_clone.clone(), 
                    connected_peers_clone.clone(),
                    &node_address
                ).await;
            }
        });
    }
    
    async fn discover_from_peers(
        peers: Arc<RwLock<HashMap<String, Peer>>>, 
        connected_peers: Arc<RwLock<HashSet<String>>>,
        node_address: &str
    ) {
        println!("🔄 Discovering new peers from network...");
        
        let current_peers = peers.read().await;
        let peer_list: Vec<String> = current_peers.keys().cloned().collect();
        drop(current_peers);
        
        for peer_addr in peer_list {
            if let Ok(new_peers) = P2PNetwork::query_peer_for_peers(&peer_addr).await {
                let mut peers_write = peers.write().await;
                let mut connected_write = connected_peers.write().await;
                
                for new_peer_addr in new_peers {
                    // Khud ko aur already connected peers ko avoid karo
                    if new_peer_addr != node_address && !connected_write.contains(&new_peer_addr) {
                        println!("🤝 Discovered new peer: {}", new_peer_addr);
                        
                        let new_peer = Peer {
                            address: new_peer_addr.clone(),
                            version: "1.0.0".to_string(),
                            height: 0,
                        };
                        
                        peers_write.insert(new_peer_addr.clone(), new_peer);
                        connected_write.insert(new_peer_addr);
                    }
                }
            }
        }
    }
    
    async fn query_peer_for_peers(peer_addr: &str) -> Result<Vec<String>, String> {
        // Simulated peer discovery - actual implementation mein network calls honge
        println!("📡 Querying peers from: {}", peer_addr);
        
        // Mock data for testing
        let mock_peers = match peer_addr {
            "127.0.0.1:8080" => vec!["127.0.0.1:8081".to_string(), "127.0.0.1:8082".to_string()],
            "127.0.0.1:8081" => vec!["127.0.0.1:8080".to_string(), "127.0.0.1:8083".to_string()],
            "127.0.0.1:8082" => vec!["127.0.0.1:8080".to_string(), "127.0.0.1:8084".to_string()],
            _ => vec![],
        };
        
        Ok(mock_peers)
    }
    
    // 🔥 NEW NODE JOIN SUPPORT
    pub async fn handle_new_node_join(&self, new_node_address: String) {
        println!("🎯 New node joining network: {}", new_node_address);
        
        // New node ko peer list send karo
        self.send_peer_list_to_node(&new_node_address).await;
        
        // New node ko help karo initial sync ke liye
        self.assist_initial_sync(&new_node_address).await;
        
        // New node ko peers mein add karo
        let new_peer = Peer {
            address: new_node_address.clone(),
            version: "1.0.0".to_string(),
            height: 0,
        };
        
        self.peers.write().await.insert(new_node_address.clone(), new_peer);
        self.connected_peers.write().await.insert(new_node_address);
    }
    
    async fn send_peer_list_to_node(&self, node_address: &str) {
        println!("📋 Sending peer list to: {}", node_address);
        
        let peers = self.peers.read().await;
        let peer_addresses: Vec<String> = peers.keys().cloned().collect();
        
        // Actual implementation mein network call hoga
        println!("📤 Sent {} peers to {}", peer_addresses.len(), node_address);
    }
    
    async fn assist_initial_sync(&self, node_address: &str) {
        println!("🔄 Assisting initial sync for: {}", node_address);
        
        // New node ko latest block height batayo
        // Aur initial block download mein help karo
        println!("✅ Initial sync assistance sent to {}", node_address);
    }
    
    // NETWORK BROADCASTING
    pub async fn broadcast_block(&self, block_json: String) {
        println!("📢 Broadcasting block to network...");
        
        let peers = self.peers.read().await;
        let peer_count = peers.len();
        
        for (address, _) in peers.iter() {
            if address != &self.node_address {
                self.send_data_to_peer(address, block_json.clone()).await;
            }
        }
        
        println!("✅ Block broadcasted to {} peers", peer_count);
    }
    
    pub async fn broadcast_transaction(&self, transaction_json: String) {
        println!("📢 Broadcasting transaction to network...");
        
        let peers = self.peers.read().await;
        
        for (address, _) in peers.iter() {
            if address != &self.node_address {
                self.send_data_to_peer(address, transaction_json.clone()).await;
            }
        }
    }
    
    async fn send_data_to_peer(&self, peer_address: &str, data: String) {
        // Simulated network send
        println!("📤 Sending data to peer: {}", peer_address);
        // Actual implementation mein WebSocket/HTTP call hoga
    }
    
    // NETWORK INFO
    pub async fn get_network_info(&self) -> NetworkInfo {
        let peers = self.peers.read().await;
        let connected = self.connected_peers.read().await;
        
        NetworkInfo {
            total_peers: peers.len(),
            connected_peers: connected.len(),
            node_address: self.node_address.clone(),
            seed_nodes: self.seed_nodes.len(),
        }
    }
    
    pub async fn get_peer_count(&self) -> usize {
        self.peers.read().await.len()
    }
}

// NETWORK INFO STRUCT
pub struct NetworkInfo {
    pub total_peers: usize,
    pub connected_peers: usize,
    pub node_address: String,
    pub seed_nodes: usize,
}

impl NetworkInfo {
    pub fn display(&self) {
        println!("🌐 Network Information:");
        println!("   Node Address: {}", self.node_address);
        println!("   Total Peers: {}", self.total_peers);
        println!("   Connected Peers: {}", self.connected_peers);
        println!("   Seed Nodes: {}", self.seed_nodes);
    }
}

// PEER MANAGEMENT
impl P2PNetwork {
    pub async fn add_peer(&self, address: String) {
        let peer = Peer {
            address: address.clone(),
            version: "1.0.0".to_string(),
            height: 0,
        };
        
        self.peers.write().await.insert(address, peer);
    }
    
    pub async fn remove_peer(&self, address: &str) {
        self.peers.write().await.remove(address);
        self.connected_peers.write().await.remove(address);
        println!("🗑️ Removed peer: {}", address);
    }
    
    pub async fn list_peers(&self) -> Vec<Peer> {
        self.peers.read().await.values().cloned().collect()
    }
}