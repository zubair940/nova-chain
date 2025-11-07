// network.rs
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufReader, BufRead};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

// 🆕 NEW: Import all feature models
use crate::models::{
    BurnEvent, Proposal, Vote, BridgeTransaction, 
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    NewBlock,
    NewTransaction,
    RequestChain,
    SendChain,
    // 🆕 NEW: All Feature Message Types
    TokenBurn,          // 🆕 Token burning messages
    DAOProposal,        // 🆕 DAO proposal messages  
    DAOVote,            // 🆕 DAO vote messages
    BridgeTransfer,     // 🆕 Cross-chain bridge messages
    SecurityAlert,      // 🆕 Security audit messages
    StakingUpdate,      // 🆕 Staking updates
    GovernanceUpdate,   // 🆕 Governance updates
    PeerDiscovery,      // 🆕 Peer discovery
    SyncRequest,        // 🆕 Sync requests for new features
    FeatureStatus,      // 🆕 Feature status updates
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMessage {
    pub message_type: MessageType,
    pub data: String,
    pub timestamp: u64,
    // 🆕 NEW: Enhanced message fields
    pub sender: String,      // 🆕 Message sender
    pub signature: String,   // 🆕 Digital signature
    pub version: String,     // 🆕 Protocol version
}

pub struct Network {
    port: u16,
    peers: Arc<Mutex<Vec<String>>>,
    messages: Arc<Mutex<VecDeque<NetworkMessage>>>,
    // 🆕 NEW: Enhanced network features
    node_id: String,
    is_bootstrap_node: bool,
    connected_peers: Arc<Mutex<Vec<ConnectedPeer>>>,
    feature_support: Arc<Mutex<FeatureSupport>>,
}

// 🆕 NEW: Connected peer with enhanced info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedPeer {
    pub address: String,
    pub node_id: String,
    pub version: String,
    pub features: Vec<String>, // Supported features
    pub last_seen: u64,
    pub reputation: i32,
}

// 🆕 NEW: Feature support tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureSupport {
    pub token_burning: bool,
    pub dao_governance: bool,
    pub cross_chain_bridge: bool,
    pub security_audit: bool,
    pub staking: bool,
    pub smart_contracts: bool,
}

impl Network {
    pub fn new(port: u16) -> Self {
        // 🆕 NEW: Generate node ID
        let node_id = format!("node_{}_{}", port, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
        
        Network {
            port,
            peers: Arc::new(Mutex::new(Vec::new())),
            messages: Arc::new(Mutex::new(VecDeque::new())),
            // 🆕 NEW: Initialize enhanced features
            node_id,
            is_bootstrap_node: port == 8080, // First node is bootstrap
            connected_peers: Arc::new(Mutex::new(Vec::new())),
            feature_support: Arc::new(Mutex::new(FeatureSupport {
                token_burning: true,
                dao_governance: true,
                cross_chain_bridge: true,
                security_audit: true,
                staking: true,
                smart_contracts: true,
            })),
        }
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port))
            .expect("Failed to bind to address");

        println!("🌐 Vexa Chain P2P Node started on port {}", self.port);
        println!("🆔 Node ID: {}", self.node_id);
        println!("🌟 Supported Features: Token Burning, DAO Governance, Cross-chain Bridge, Security Audit");

        // 🆕 NEW: Enhanced peer discovery with feature support
        let peers_clone = self.peers.clone();
        let connected_peers_clone = self.connected_peers.clone();
        let feature_support_clone = self.feature_support.clone();
        let node_id = self.node_id.clone();
        let port = self.port;

        thread::spawn(move || {
            Network::enhanced_peer_discovery(
                peers_clone, 
                connected_peers_clone, 
                feature_support_clone,
                node_id,
                port
            );
        });

        // 🆕 NEW: Start feature synchronization
        let messages_clone = self.messages.clone();
        let peers_clone = self.peers.clone();
        thread::spawn(move || {
            Network::feature_sync_loop(messages_clone, peers_clone);
        });

        // Handle incoming connections
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let messages_clone = self.messages.clone();
                    let connected_peers_clone = self.connected_peers.clone();
                    let peer_addr = stream.peer_addr().unwrap().to_string();
                    
                    thread::spawn(move || {
                        Network::handle_enhanced_client(
                            stream, 
                            messages_clone, 
                            connected_peers_clone,
                            peer_addr
                        );
                    });
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }
    }

    // 🆕 NEW: Enhanced client handling with feature support
    fn handle_enhanced_client(
        stream: TcpStream, 
        messages: Arc<Mutex<VecDeque<NetworkMessage>>>, 
        connected_peers: Arc<Mutex<Vec<ConnectedPeer>>>,
        peer_addr: String
    ) {
        let reader = BufReader::new(stream.try_clone().unwrap());
        
        // 🆕 NEW: Add peer to connected list
        {
            let mut peers = connected_peers.lock().unwrap();
            let new_peer = ConnectedPeer {
                address: peer_addr.clone(),
                node_id: "unknown".to_string(),
                version: "1.0.0".to_string(),
                features: vec!["all".to_string()],
                last_seen: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                reputation: 10,
            };
            peers.push(new_peer);
            println!("🔗 New peer connected: {}", peer_addr);
        }
        
        for line in reader.lines() {
            if let Ok(message_str) = line {
                if let Ok(message) = serde_json::from_str::<NetworkMessage>(&message_str) {
                    println!("📨 Received {} message from {}", 
                             format!("{:?}", message.message_type), 
                             peer_addr);
                    
                    // 🆕 NEW: Enhanced message processing
                    Network::process_enhanced_message(&message, &messages);
                    
                    messages.lock().unwrap().push_back(message);
                }
            }
        }
        
        // 🆕 NEW: Remove peer when disconnected
        {
            let mut peers = connected_peers.lock().unwrap();
            peers.retain(|p| p.address != peer_addr);
            println!("🔌 Peer disconnected: {}", peer_addr);
        }
    }

    // 🆕 NEW: Enhanced peer discovery with feature negotiation
    fn enhanced_peer_discovery(
        peers: Arc<Mutex<Vec<String>>>, 
        connected_peers: Arc<Mutex<Vec<ConnectedPeer>>>,
        feature_support: Arc<Mutex<FeatureSupport>>,
        node_id: String,
        my_port: u16
    ) {
        loop {
            // Discover peers on local network
            for port in 8080..8100 {
                if port == my_port {
                    continue; // Skip self
                }

                let peer_addr = format!("127.0.0.1:{}", port);
                if TcpStream::connect(&peer_addr).is_ok() {
                    let mut peers_guard = peers.lock().unwrap();
                    if !peers_guard.contains(&peer_addr) {
                        println!("✅ Discovered peer: {}", peer_addr);
                        peers_guard.push(peer_addr.clone());
                        
                        // 🆕 NEW: Add to connected peers with feature info
                        let mut connected_guard = connected_peers.lock().unwrap();
                        connected_guard.push(ConnectedPeer {
                            address: peer_addr,
                            node_id: format!("node_{}", port),
                            version: "1.0.0".to_string(),
                            features: vec![
                                "token_burning".to_string(),
                                "dao_governance".to_string(), 
                                "cross_chain_bridge".to_string(),
                                "security_audit".to_string(),
                                "staking".to_string(),
                                "smart_contracts".to_string(),
                            ],
                            last_seen: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                            reputation: 10,
                        });
                    }
                }
            }
            
            // 🆕 NEW: Broadcast feature support
            let feature_message = NetworkMessage {
                message_type: MessageType::FeatureStatus,
                data: serde_json::to_string(&*feature_support.lock().unwrap()).unwrap(),
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                sender: node_id.clone(),
                signature: "feature_broadcast".to_string(),
                version: "1.0.0".to_string(),
            };
            
            // Broadcast to all peers
            let peers_list = peers.lock().unwrap().clone();
            for peer in peers_list {
                if let Ok(mut stream) = TcpStream::connect(&peer) {
                    let message_str = serde_json::to_string(&feature_message).unwrap();
                    writeln!(stream, "{}", message_str).ok();
                }
            }
            
            thread::sleep(std::time::Duration::from_secs(30)); // Check every 30 seconds
        }
    }

    // 🆕 NEW: Enhanced message processing for all features
    fn process_enhanced_message(message: &NetworkMessage, _messages: &Arc<Mutex<VecDeque<NetworkMessage>>>) {
        match message.message_type {
            MessageType::TokenBurn => {
                println!("🔥 Processing token burn message");
                // Process burn event from network
                if let Ok(burn_event) = serde_json::from_str::<BurnEvent>(&message.data) {
                    // 🔥 FIXED: Use `from` field instead of `burner`
                    println!("   Burner: {}, Amount: {}", burn_event.from, burn_event.amount);
                }
            }
            MessageType::DAOProposal => {
                println!("🗳️ Processing DAO proposal message");
                if let Ok(proposal) = serde_json::from_str::<Proposal>(&message.data) {
                    println!("   Proposal: {} by {}", proposal.title, proposal.proposer);
                }
            }
            MessageType::DAOVote => {
                println!("✅ Processing DAO vote message");
                if let Ok(vote) = serde_json::from_str::<Vote>(&message.data) {
                    // 🔥 FIXED: Use `vote` field instead of `vote_type`
                    println!("   Vote on proposal {}: {}", vote.proposal_id, vote.vote);
                }
            }
            MessageType::BridgeTransfer => {
                println!("🌉 Processing bridge transfer message");
                if let Ok(bridge_tx) = serde_json::from_str::<BridgeTransaction>(&message.data) {
                    println!("   Bridge TX: {} -> {}", bridge_tx.from_chain, bridge_tx.to_chain);
                }
            }
            MessageType::FeatureStatus => {
                println!("🌟 Processing feature status update");
                // Update feature support based on network
            }
            _ => {
                // Handle existing message types
            }
        }
    }

    // 🆕 NEW: Feature synchronization loop
    fn feature_sync_loop(
        messages: Arc<Mutex<VecDeque<NetworkMessage>>>,
        _peers: Arc<Mutex<Vec<String>>>
    ) {
        loop {
            // Sync feature data every minute
            thread::sleep(std::time::Duration::from_secs(60));
            // Create sync message
            let sync_message = NetworkMessage {
                message_type: MessageType::SyncRequest,
                data: "feature_sync".to_string(),
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                sender: "sync_service".to_string(),
                signature: "sync_signature".to_string(),
                version: "1.0.0".to_string(),
            };
            
            // Add to local messages for processing
            messages.lock().unwrap().push_back(sync_message.clone());
            
            println!("🔄 Feature synchronization cycle completed");
        }
    }

    pub fn broadcast_message(&self, message: NetworkMessage) {
        let peers = self.peers.lock().unwrap();
        for peer in peers.iter() {
            if let Ok(mut stream) = TcpStream::connect(peer) {
                let message_str = serde_json::to_string(&message).unwrap();
                writeln!(stream, "{}", message_str).ok();
                println!("📤 Broadcast {} to: {}", format!("{:?}", message.message_type), peer);
            }
        }
    }

    // 🆕 NEW: Feature-specific broadcast methods
    pub fn broadcast_burn_event(&self, burn_event: BurnEvent) {
        let message = NetworkMessage {
            message_type: MessageType::TokenBurn,
            data: serde_json::to_string(&burn_event).unwrap(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            sender: self.node_id.clone(),
            signature: "burn_broadcast".to_string(),
            version: "1.0.0".to_string(),
        };
        self.broadcast_message(message);
    }

    pub fn broadcast_dao_proposal(&self, proposal: Proposal) {
        let message = NetworkMessage {
            message_type: MessageType::DAOProposal,
            data: serde_json::to_string(&proposal).unwrap(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            sender: self.node_id.clone(),
            signature: "dao_broadcast".to_string(),
            version: "1.0.0".to_string(),
        };
        self.broadcast_message(message);
    }

    pub fn broadcast_dao_vote(&self, vote: Vote) {
        let message = NetworkMessage {
            message_type: MessageType::DAOVote,
            data: serde_json::to_string(&vote).unwrap(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            sender: self.node_id.clone(),
            signature: "vote_broadcast".to_string(),
            version: "1.0.0".to_string(),
        };
        self.broadcast_message(message);
    }

    pub fn broadcast_bridge_transfer(&self, bridge_tx: BridgeTransaction) {
        let message = NetworkMessage {
            message_type: MessageType::BridgeTransfer,
            data: serde_json::to_string(&bridge_tx).unwrap(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            sender: self.node_id.clone(),
            signature: "bridge_broadcast".to_string(),
            version: "1.0.0".to_string(),
        };
        self.broadcast_message(message);
    }

    // 🆕 NEW: Security alert broadcast function
    pub fn broadcast_security_alert(&self) {
        let message = NetworkMessage {
            message_type: MessageType::SecurityAlert,
            data: "Security alert from node".to_string(),
            signature: "security_broadcast".to_string(),
            version: "1.0.0".to_string(),
            sender: self.node_id.clone(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        };
        self.broadcast_message(message);
    }

    pub fn get_messages(&self) -> Vec<NetworkMessage> {
        let mut messages = self.messages.lock().unwrap();
        let collected: Vec<NetworkMessage> = messages.drain(..).collect();
        collected
    }

    pub fn add_peer(&self, address: String) {
        self.peers.lock().unwrap().push(address);
    }

    // 🆕 NEW: Enhanced network info
    pub fn get_network_info(&self) -> String {
        let peers = self.peers.lock().unwrap();
        let connected_peers = self.connected_peers.lock().unwrap();
        
        format!(
            "Network Info:\n\
            Port: {}\n\
            Node ID: {}\n\
            Total Peers: {}\n\
            Connected Peers: {}\n\
            Bootstrap Node: {}\n\
            Features: Token Burning, DAO, Bridge, Security Audit",
            self.port,
            self.node_id,
            peers.len(),
            connected_peers.len(),
            self.is_bootstrap_node
        )
    }

    // 🆕 NEW: Get connected peers with features
    pub fn get_connected_peers(&self) -> Vec<ConnectedPeer> {
        let connected_peers = self.connected_peers.lock().unwrap();
        connected_peers.clone()
    }

    // 🆕 NEW: Check feature support
    pub fn supports_feature(&self, feature: &str) -> bool {
        let feature_support = self.feature_support.lock().unwrap();
        match feature {
            "token_burning" => feature_support.token_burning,
            "dao_governance" => feature_support.dao_governance,
            "cross_chain_bridge" => feature_support.cross_chain_bridge,
            "security_audit" => feature_support.security_audit,
            "staking" => feature_support.staking,
            "smart_contracts" => feature_support.smart_contracts,
            _ => false,
        }
    }
}

impl Clone for Network {
    fn clone(&self) -> Self {
        Network {
            port: self.port,
            peers: Arc::clone(&self.peers),
            messages: Arc::clone(&self.messages),
            node_id: self.node_id.clone(),
            is_bootstrap_node: self.is_bootstrap_node,
            connected_peers: Arc::clone(&self.connected_peers),
            feature_support: Arc::clone(&self.feature_support),
        }
    }
}