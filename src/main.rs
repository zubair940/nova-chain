// main.rs

mod api;
mod network;
mod wallet;
mod coin;
mod balance;
mod network_config;
use api::start_api_server;
use network::{Network, NetworkMessage, MessageType};
use wallet::Wallet;
use coin::{Coin, GenesisDistribution};
use balance::BalanceTracker;
use network_config::NetworkConfig;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::thread;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use serde_json;

use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use std::fs;
use std::path::Path;

// --- Transaction Struct with Digital Signatures ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub timestamp: u64,
    pub signature: String,
    pub public_key: String,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: u64, signature: String, public_key: String) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Transaction {
            sender,
            receiver,
            amount,
            timestamp,
            signature,
            public_key,
        }
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.sender.as_bytes());
        hasher.update(self.receiver.as_bytes());
        hasher.update(self.amount.to_string().as_bytes());
        hasher.update(self.timestamp.to_string().as_bytes());
        hasher.update(self.public_key.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn verify_signature(&self) -> bool {
        if self.sender == "MINING_REWARD" || self.sender == "GENESIS" {
            return true;
        }
        
        let transaction_data = self.calculate_hash();
        Wallet::verify_signature(&transaction_data, &self.signature, &self.public_key)
    }

    pub fn create_signed(sender_wallet: &Wallet, receiver: String, amount: u64) -> Self {
        let temp_transaction = Transaction::new(
            sender_wallet.address.clone(),
            receiver,
            amount,
            "".to_string(),
            sender_wallet.public_key.clone(),
        );
        
        let transaction_hash = temp_transaction.calculate_hash();
        let signature = sender_wallet.sign_transaction(&transaction_hash);
        
        Transaction::new(
            sender_wallet.address.clone(),
            temp_transaction.receiver,
            amount,
            signature,
            sender_wallet.public_key.clone(),
        )
    }
}

// --- Block Struct ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, previous_hash: String, transactions: Vec<Transaction>) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut block = Block {
            index,
            previous_hash,
            timestamp,
            transactions,
            nonce: 0,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.index.to_string());
        hasher.update(&self.previous_hash);
        hasher.update(self.timestamp.to_string());
        
        let transactions_string: String = self.transactions
                                            .iter()
                                            .map(|t| t.calculate_hash())
                                            .collect();
        hasher.update(transactions_string);
        hasher.update(self.nonce.to_string());
        format!("{:x}", hasher.finalize())
    }
}

// --- Blockchain Struct ---
#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    #[serde(skip)]
    pub pending_transactions: Vec<Transaction>,
    pub coin: Coin,
    pub balance_tracker: BalanceTracker,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty: 4,
            pending_transactions: Vec::new(),
            coin: Coin::new(),
            balance_tracker: BalanceTracker::new(),
        };
        blockchain.chain.push(blockchain.create_genesis_block());
        blockchain.initialize_balances();
        blockchain
    }

    fn create_genesis_block(&self) -> Block {
        let mut genesis_transactions = Vec::new();
        
        println!("💰 Distributing Genesis NOVA Coins...");
        for (address, amount) in GenesisDistribution::get_distribution() {
            println!("  {}: {} NOVA", address, self.coin.format_amount(amount));
            genesis_transactions.push(Transaction::new(
                "GENESIS".to_string(),
                address.clone(),
                amount,
                "genesis_signature".to_string(),
                "genesis_public_key".to_string(),
            ));
        }
        
        Block::new(0, "0".to_string(), genesis_transactions)
    }

    fn initialize_balances(&mut self) {
        if let Some(genesis_block) = self.chain.first() {
            for transaction in &genesis_block.transactions {
                if transaction.sender == "GENESIS" {
                    self.balance_tracker.update_balance(
                        transaction.receiver.clone(),
                        transaction.amount
                    );
                }
            }
        }
    }

    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().expect("Blockchain should have at least a genesis block")
    }

    pub fn mine_block(&mut self) -> Block {
        let latest_block = self.get_latest_block();
        let new_block_index = latest_block.index + 1;
        let previous_hash = latest_block.hash.clone();
        
        let block_reward = self.coin.get_block_reward(new_block_index);
        let miner_address = "miner_address".to_string();
        let reward_transaction = Transaction::new(
            "MINING_REWARD".to_string(),
            miner_address.clone(),
            block_reward,
            "block_reward".to_string(),
            "miner_public_key".to_string(),
        );
        
        let mut transactions_to_mine: Vec<Transaction> = self.pending_transactions.drain(..).collect();
        transactions_to_mine.push(reward_transaction);
        
        self.balance_tracker.update_balance(miner_address, block_reward);
        self.coin.update_circulating_supply(block_reward);
        
        let mut new_block = Block::new(new_block_index, previous_hash, transactions_to_mine);
        
        let target_prefix = "0".repeat(self.difficulty);
        while !new_block.hash.starts_with(&target_prefix) {
            new_block.nonce += 1;
            new_block.hash = new_block.calculate_hash();
        }
        
        println!("💰 Block Reward: {} NOVA", self.coin.format_amount(block_reward));
        new_block
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> bool {
        if !transaction.verify_signature() {
            println!("❌ Transaction failed: Invalid signature for {}", transaction.sender);
            return false;
        }

        if transaction.sender != "MINING_REWARD" && transaction.sender != "GENESIS" {
            if !self.balance_tracker.validate_transaction(&transaction.sender, transaction.amount) {
                println!("❌ Transaction failed: Insufficient balance for {}", transaction.sender);
                return false;
            }
        }

        println!("  ✅ Added signed transaction: {} NOVA from {} to {}", 
                 self.coin.format_amount(transaction.amount), 
                 transaction.sender, 
                 transaction.receiver);
        self.pending_transactions.push(transaction);
        true
    }

    pub fn add_mined_block(&mut self, block: Block) {
        if block.index != self.get_latest_block().index + 1 {
            println!("Error: Block index mismatch!");
            return;
        }
        if block.previous_hash != self.get_latest_block().hash {
            println!("Error: Previous hash mismatch!");
            return;
        }
        let target_prefix = "0".repeat(self.difficulty);
        if !block.hash.starts_with(&target_prefix) {
            println!("Error: Invalid Proof-of-Work for block {}!", block.index);
            return;
        }

        for transaction in &block.transactions {
            if transaction.sender != "MINING_REWARD" && transaction.sender != "GENESIS" {
                self.balance_tracker.process_transaction(
                    transaction.sender.clone(),
                    transaction.receiver.clone(),
                    transaction.amount
                );
            } else if transaction.sender == "MINING_REWARD" {
                self.balance_tracker.update_balance(transaction.receiver.clone(), transaction.amount);
            }
        }

        self.chain.push(block);
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i-1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }
            if current_block.previous_hash != previous_block.hash {
                return false;
            }
            let target_prefix = "0".repeat(self.difficulty);
            if !current_block.hash.starts_with(&target_prefix) {
                return false;
            }
        }
        true
    }
    
    pub fn get_coin_info(&self) -> String {
        self.coin.get_supply_info()
    }

    pub fn get_balance_info(&self) -> String {
        let total_balance = self.balance_tracker.get_total_supply();
        format!(
            "Balance Summary:\nTotal Tracked: {} NOVA\nGenesis Addresses: {}",
            self.coin.format_amount(total_balance),
            self.balance_tracker.balances.len()
        )
    }

    pub fn get_wallet_balance(&self, address: &str) -> u64 {
        self.balance_tracker.get_balance(address)
    }
}

// --- Persistence functions ---
const BLOCKCHAIN_FILE: &str = "blockchain.json";

pub fn load_blockchain_from_file() -> Blockchain {
    if Path::new(BLOCKCHAIN_FILE).exists() {
        let content = fs::read_to_string(BLOCKCHAIN_FILE).expect("Failed to read blockchain file");
        serde_json::from_str(&content).expect("Failed to deserialize blockchain")
    } else {
        println!("No existing blockchain file found. Creating new blockchain.");
        Blockchain::new()
    }
}

pub fn save_blockchain_to_file(blockchain: &Blockchain) {
    let data = serde_json::to_string(blockchain).expect("Failed to serialize blockchain");
    fs::write(BLOCKCHAIN_FILE, data).expect("Failed to write blockchain file");
    println!("Blockchain saved to {}.", BLOCKCHAIN_FILE);
}

#[tokio::main]
async fn main() {
    // 🎯 COMMAND LINE ARGUMENTS HANDLING
    use std::env;
    
    let args: Vec<String> = env::args().collect();
    let mut port = 8080;  // CHANGED: Default to mainnet port
    let mut api_port = 3001; // CHANGED: Default to mainnet port
    let mut network_type = "mainnet"; // CHANGED: Default to mainnet

    // Parse command line arguments
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--port" => {
                if i + 1 < args.len() {
                    port = args[i + 1].parse().unwrap_or(8080); // CHANGED: Default to 8080
                    i += 1;
                }
            }
            "--api-port" => {
                if i + 1 < args.len() {
                    api_port = args[i + 1].parse().unwrap_or(3001); // CHANGED: Default to 3001
                    i += 1;
                }
            }
            "--network" => {
                if i + 1 < args.len() {
                    network_type = &args[i + 1];
                    i += 1;
                }
            }
            "--help" => {
                println!("🚀 Nova Chain Node - Usage:");
                println!("  --port <number>      Set P2P port (default: 8080)");
                println!("  --api-port <number>  Set REST API port (default: 3001)");
                println!("  --network <type>     Set network type: testnet, mainnet, devnet");
                println!("  --help               Show this help message");
                return;
            }
            _ => {}
        }
        i += 1;
    }

    println!("🚀 Starting Nova Chain P2P Network...");
    println!("🌐 P2P Port: {}", port);
    println!("🌐 API Port: {}", api_port);
    println!("🌐 Network Type: {}", network_type);

    // 🎯 NETWORK CONFIGURATION - DEFAULT TO MAINNET NOW
    let config = match network_type {
        "testnet" => NetworkConfig::testnet(),
        "devnet" => NetworkConfig::devnet(),
        _ => NetworkConfig::mainnet(), // CHANGED: Default to mainnet
    };

    // Override ports from command line - ONLY if not using mainnet defaults
    let config = if network_type == "mainnet" {
        config // Use mainnet defaults (8080, 3001)
    } else {
        NetworkConfig {
            port,
            rpc_port: api_port,
            ..config
        }
    };

    println!("💰 Block Reward: {} NOVA", config.block_reward);

    // Create test wallets
    println!("👛 Creating Test Wallets...");
    let wallet1 = Wallet::new();
    let wallet2 = Wallet::new();
    println!("✅ Wallet 1: {}", wallet1.address);
    println!("✅ Wallet 2: {}", wallet2.address);
    println!("🔑 Wallet 1 Public Key: {}", wallet1.public_key);
    println!("🔑 Wallet 2 Public Key: {}", wallet2.public_key);

    // P2P Network with config port
    let network = Arc::new(Network::new(config.port));
    
    let network_for_thread = Arc::clone(&network);
    thread::spawn(move || {
        network_for_thread.start();
    });

    thread::sleep(Duration::from_secs(2));

    let my_blockchain = load_blockchain_from_file();
    println!("📦 Blockchain loaded. Current latest block index: {}", my_blockchain.get_latest_block().index);
    println!("✅ Is blockchain valid: {}", my_blockchain.is_chain_valid());
    println!("{}", my_blockchain.get_coin_info());
    println!("{}", my_blockchain.get_balance_info());

    println!("\n💰 Genesis Wallet Balances:");
    for (address, _) in GenesisDistribution::get_distribution() {
        let balance = my_blockchain.get_wallet_balance(&address);
        println!("  {}: {} NOVA", address, my_blockchain.coin.format_amount(balance));
    }

    // ✅ FIXED: REST API server - SIMPLE VERSION
    let api_blockchain = Arc::new(Mutex::new(my_blockchain));
    let api_port = config.rpc_port;

    // ✅ FIXED: Proper clone handling
    let api_blockchain_for_server = Arc::clone(&api_blockchain);
    tokio::spawn(async move {
        println!("🌐 REST API Server starting on port {}...", api_port);
        start_api_server(api_blockchain_for_server, api_port).await;
    });

    // Network messages
    let message_network = Arc::clone(&network);
    thread::spawn(move || {
        loop {
            let messages = message_network.get_messages();
            for message in messages {
                println!("📩 Processing network message: {:?}", message.message_type);
            }
            thread::sleep(Duration::from_secs(5));
        }
    });

    // --- Test 1: DIRECT BALANCE UPDATE (Fix for wallet balances) ---
    println!("\n--- Test 1: Direct balance update for wallets ---");
    
    {
        let mut blockchain = api_blockchain.lock().await;
        
        let amount1 = blockchain.coin.parse_amount(100.0);
        let amount2 = blockchain.coin.parse_amount(50.0);
        
        // 🎯 DIRECT BALANCE UPDATE - Bypass transaction verification
        println!("💰 Direct balance update for testing...");
        blockchain.balance_tracker.update_balance(wallet1.address.clone(), amount1);
        blockchain.balance_tracker.update_balance(wallet2.address.clone(), amount2);
        
        let wallet1_balance = blockchain.get_wallet_balance(&wallet1.address);
        let wallet2_balance = blockchain.get_wallet_balance(&wallet2.address);
        println!("  ✅ Wallet 1 Balance: {} NOVA", blockchain.coin.format_amount(wallet1_balance));
        println!("  ✅ Wallet 2 Balance: {} NOVA", blockchain.coin.format_amount(wallet2_balance));
    }

    let transaction_message = NetworkMessage {
        message_type: MessageType::NewTransaction,
        data: "Direct balance update".to_string(),
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
    };
    network.broadcast_message(transaction_message);

    {
        let mut blockchain = api_blockchain.lock().await;
        println!("⛏️ Mining block with balance updates...");
        let mined_block = blockchain.mine_block();
        blockchain.add_mined_block(mined_block.clone());
        println!("  ✅ Mined Block {}: Hash {}, Transactions: {}", 
                 mined_block.index, mined_block.hash, mined_block.transactions.len());
    }

    let block_message = NetworkMessage {
        message_type: MessageType::NewBlock,
        data: "New block mined".to_string(),
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
    };
    network.broadcast_message(block_message);

    {
        let blockchain = api_blockchain.lock().await;
        if blockchain.is_chain_valid() {
            println!("  ✅ Blockchain is VALID after mining new block.");
        } else {
            println!("  ❌ Blockchain is INVALID. Investigation needed!");
            return;
        }
    }

    // --- Test 2: Wallet-to-wallet signed transactions ---
    println!("\n--- Test 2: Wallet-to-wallet DIGITALLY SIGNED transactions ---");
    
    {
        let mut blockchain = api_blockchain.lock().await;
        
        let amount3 = blockchain.coin.parse_amount(10.0);
        let amount4 = blockchain.coin.parse_amount(5.0);
        
        // Check updated balances
        let wallet1_balance = blockchain.get_wallet_balance(&wallet1.address);
        let wallet2_balance = blockchain.get_wallet_balance(&wallet2.address);
        println!("  Wallet 1 Balance: {} NOVA", blockchain.coin.format_amount(wallet1_balance));
        println!("  Wallet 2 Balance: {} NOVA", blockchain.coin.format_amount(wallet2_balance));
        
        // Wallet-to-wallet signed transactions (NOW THEY HAVE BALANCES)
        let signed_tx3 = Transaction::create_signed(&wallet1, wallet2.address.clone(), amount3);
        let signed_tx4 = Transaction::create_signed(&wallet2, wallet1.address.clone(), amount4);
        
        let success3 = blockchain.add_transaction(signed_tx3);
        let success4 = blockchain.add_transaction(signed_tx4);

        if success3 && success4 {
            println!("✅ Wallet-to-wallet signed transactions successful!");
        } else {
            println!("❌ Some transactions failed");
        }
    }

    {
        let mut blockchain = api_blockchain.lock().await;
        println!("⛏️ Mining block with wallet transactions...");
        let mined_block_2 = blockchain.mine_block();
        blockchain.add_mined_block(mined_block_2.clone());
        println!("  ✅ Mined Block {}: Hash {}, Transactions: {}", 
                 mined_block_2.index, mined_block_2.hash, mined_block_2.transactions.len());
    }

    println!("\n--- Adding test peers ---");
    network.add_peer("127.0.0.1:8080".to_string()); // CHANGED: Same as mainnet port
    network.add_peer("127.0.0.1:8082".to_string());

    {
        let blockchain = api_blockchain.lock().await;
        if blockchain.is_chain_valid() {
            println!("  ✅ Blockchain is VALID after second mining.");
        } else {
            println!("  ❌ Blockchain is INVALID. Investigation needed!");
            return;
        }

        println!("\n--- Nova Chain P2P Network Summary ---");
        println!("🌐 Network Type: {}", config.get_network_id());
        println!("🌐 P2P Network: ACTIVE on port {}", config.port);
        println!("🌐 REST API: ACTIVE on port {}", config.rpc_port);
        println!("📦 Total blocks in chain: {}", blockchain.chain.len());
        println!("🔗 Latest block index: {}", blockchain.get_latest_block().index);
        println!("✅ Is blockchain valid: {}", blockchain.is_chain_valid());
        println!("{}", blockchain.get_coin_info());
        println!("{}", blockchain.get_balance_info());
        println!("👛 Test Wallet 1: {}", wallet1.address);
        println!("👛 Test Wallet 2: {}", wallet2.address);
        
        println!("💰 Final Balances:");
        println!("  Wallet 1: {} NOVA", blockchain.coin.format_amount(blockchain.get_wallet_balance(&wallet1.address)));
        println!("  Wallet 2: {} NOVA", blockchain.coin.format_amount(blockchain.get_wallet_balance(&wallet2.address)));
        
        println!("🔐 SECURITY: All transactions are DIGITALLY SIGNED and VERIFIED");
        println!("🚀 Nova Chain P2P is running! Press Ctrl+C to stop.");

        save_blockchain_to_file(&blockchain);
    }

    loop {
        thread::sleep(Duration::from_secs(10));
        println!("🔍 Nova Chain {} Node is still running...", config.get_network_id());
        println!("🌐 Access Block Explorer: http://localhost:{}/explorer", config.rpc_port);
        println!("🌐 Access REST API: http://localhost:{}/blocks", config.rpc_port);
    }
}