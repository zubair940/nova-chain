// main.rs

// Add these new imports for serialization/deserialization if not already there
// (You should have added them in Cargo.toml already for previous errors)
use serde::{Serialize, Deserialize}; 
use serde_json; // To serialize/deserialize the blockchain to/from file

use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use std::fs; // For file system operations
use std::path::Path; // For path manipulation


// --- NEW: Transaction Struct ---
#[derive(Debug, Clone, Serialize, Deserialize)] // Added Serialize/Deserialize
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64, // Amount of tokens
    pub timestamp: u64,
    pub signature: String, // Placeholder for a real signature
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: u64, signature: String) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Transaction {
            sender,
            receiver,
            amount,
            timestamp,
            signature,
        }
    }

    // A simple method to get a hash of the transaction for verification
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.sender.as_bytes());
        hasher.update(self.receiver.as_bytes());
        hasher.update(self.amount.to_string().as_bytes());
        hasher.update(self.timestamp.to_string().as_bytes());
        hasher.update(self.signature.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}


// --- Modified Block Struct ---
#[derive(Debug, Clone, Serialize, Deserialize)] // Added Serialize/Deserialize
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>, // Changed 'data' to 'transactions'
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
            transactions, // Use transactions here
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
        
        // --- IMPORTANT: Hash all transactions in the block ---
        let transactions_string: String = self.transactions
                                            .iter()
                                            .map(|t| t.calculate_hash())
                                            .collect();
        hasher.update(transactions_string);
        // --- END IMPORTANT ---

        hasher.update(self.nonce.to_string());
        format!("{:x}", hasher.finalize())
    }
}


// --- Modified Blockchain Struct ---
#[derive(Debug, Serialize, Deserialize)] // Added Serialize/Deserialize
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    #[serde(skip)] // Don't serialize pending_transactions, they are transient
    pub pending_transactions: Vec<Transaction>, // New: To hold transactions waiting to be mined
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty: 4,
            pending_transactions: Vec::new(), // Initialize
        };
        blockchain.chain.push(blockchain.create_genesis_block());
        blockchain
    }

    fn create_genesis_block(&self) -> Block {
        Block::new(0, "0".to_string(), vec![]) // Genesis block usually has no transactions
    }

    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().expect("Blockchain should have at least a genesis block")
    }

    pub fn mine_block(&mut self) -> Block { // Modified: Now takes transactions from pending_transactions
        let latest_block = self.get_latest_block();
        let new_block_index = latest_block.index + 1;
        let previous_hash = latest_block.hash.clone();

        // Take all pending transactions and clear the pool
        let transactions_to_mine = self.pending_transactions.drain(..).collect();

        let mut new_block = Block::new(new_block_index, previous_hash, transactions_to_mine);
        
        let target_prefix = "0".repeat(self.difficulty);
        while !new_block.hash.starts_with(&target_prefix) {
            new_block.nonce += 1;
            new_block.hash = new_block.calculate_hash();
        }
        new_block
    }

    // New: Add a transaction to the pending pool
    pub fn add_transaction(&mut self, transaction: Transaction) {
        // Here you would typically add validation logic (e.g., check sender balance, signature)
        println!("  Added pending transaction: {} from {} to {}", transaction.amount, transaction.sender, transaction.receiver);
        self.pending_transactions.push(transaction);
    }

    // New: Function to add a mined block to the chain
    pub fn add_mined_block(&mut self, block: Block) {
        // Basic validation before adding to chain (more robust validation needed in real app)
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

        self.chain.push(block);
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i-1];

            if current_block.hash != current_block.calculate_hash() {
                println!("Invalid Hash at block {}: Expected {}, Got {}", 
                         current_block.index, current_block.calculate_hash(), current_block.hash);
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                println!("Invalid Previous Hash at block {}: Expected {}, Got {}", 
                         current_block.index, previous_block.hash, current_block.previous_hash);
                return false;
            }

            let target_prefix = "0".repeat(self.difficulty);
            if !current_block.hash.starts_with(&target_prefix) {
                println!("Invalid Proof-of-Work at block {}: Hash {} does not start with {}", 
                         current_block.index, current_block.hash, target_prefix);
                return false;
            }
        }
        true
    }
}


// --- New: Persistence functions ---
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
    let data = serde_json::to_string_pretty(blockchain).expect("Failed to serialize blockchain");
    fs::write(BLOCKCHAIN_FILE, data).expect("Failed to write blockchain file");
    println!("Blockchain saved to {}.", BLOCKCHAIN_FILE);
}


fn main() {
    let mut my_blockchain = load_blockchain_from_file(); 

    println!("Blockchain loaded. Current latest block index: {}", my_blockchain.get_latest_block().index);
    println!("Is blockchain valid: {}", my_blockchain.is_chain_valid());

    // --- Test 1: Simulate adding transactions and mining a block ---
    println!("\n--- Test 1: Simulating transactions and mining ---");
    
    // Add some pending transactions
    my_blockchain.add_transaction(Transaction::new(
        "AddressA".to_string(), "AddressB".to_string(), 10, "sigA1".to_string()
    ));
    my_blockchain.add_transaction(Transaction::new(
        "AddressC".to_string(), "AddressD".to_string(), 5, "sigC1".to_string()
    ));

    println!("Mining a new block with {} pending transactions...", my_blockchain.pending_transactions.len());
    let mined_block = my_blockchain.mine_block();
    my_blockchain.add_mined_block(mined_block.clone());
    println!("  Mined Block {}: Index {}, Hash {}, Transactions: {}", 
             mined_block.index, mined_block.index, mined_block.hash, mined_block.transactions.len());

    // Verify chain integrity
    if my_blockchain.is_chain_valid() {
        println!("  Blockchain is VALID after mining new block.");
    } else {
        println!("  Blockchain is INVALID. Investigation needed!");
        return;
    }

    // --- Test 2: Add more transactions and mine another block ---
    println!("\n--- Test 2: Adding more transactions and mining again ---");
    my_blockchain.add_transaction(Transaction::new(
        "AddressB".to_string(), "AddressE".to_string(), 3, "sigB1".to_string()
    ));
    my_blockchain.add_transaction(Transaction::new(
        "AddressF".to_string(), "AddressA".to_string(), 20, "sigF1".to_string()
    ));
    
    println!("Mining another block with {} pending transactions...", my_blockchain.pending_transactions.len());
    let mined_block_2 = my_blockchain.mine_block();
    my_blockchain.add_mined_block(mined_block_2.clone());
    println!("  Mined Block {}: Index {}, Hash {}, Transactions: {}", 
             mined_block_2.index, mined_block_2.index, mined_block_2.hash, mined_block_2.transactions.len());

    // Verify chain integrity
    if my_blockchain.is_chain_valid() {
        println!("  Blockchain is VALID after second mining.");
    } else {
        println!("  Blockchain is INVALID. Investigation needed!");
        return;
    }


    // --- Test 3: Large scale block generation (similar to your 15M -> 17M goal) ---
    // Note: If difficulty is high, this will take a very long time!
    println!("\n--- Test 3: Simulating large scale block generation (100 blocks for now) ---");
    let start_gen_index = my_blockchain.get_latest_block().index + 1;
    let end_gen_index = start_gen_index + 100; // Generate 100 more blocks for extensive testing

    for i in start_gen_index..=end_gen_index {
        // Simulate adding some transactions for each block
        my_blockchain.add_transaction(Transaction::new(
            format!("Sender{}", i), format!("Receiver{}", i), i % 100 + 1, format!("sig{}", i)
        ));
        my_blockchain.add_transaction(Transaction::new(
            format!("SenderX{}", i), format!("ReceiverY{}", i), (i % 50) * 2, format!("sigX{}", i)
        ));

        let mined_block = my_blockchain.mine_block();
        my_blockchain.add_mined_block(mined_block);

        if i % 10 == 0 { // Print progress
            println!("  Generated Block {}...", i);
        }
    }
    println!("  Finished generating {} blocks. Latest block index: {}", end_gen_index - start_gen_index + 1, my_blockchain.get_latest_block().index);

    // Final integrity check after large generation
    println!("\n--- Final Check: Verifying full chain integrity ---");
    if my_blockchain.is_chain_valid() {
        println!("  Blockchain is VALID after large scale generation.");
    } else {
        println!("  Blockchain is INVALID after large scale generation. Investigation needed!");
    }


    // --- Final Summary ---
    println!("\n--- Test Summary ---");
    println!("Total blocks in chain: {}", my_blockchain.chain.len());
    println!("Latest block index: {}", my_blockchain.get_latest_block().index);
    println!("Is blockchain valid: {}", my_blockchain.is_chain_valid());

    // Save the final state of the blockchain
    save_blockchain_to_file(&my_blockchain);
}
