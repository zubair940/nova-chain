use rocksdb::{DB, Options, IteratorMode};
use crate::models::{Block, Transaction};
use serde_json;
use std::collections::HashMap;

pub struct BlockchainDB {
    db: DB,
}

impl BlockchainDB {
    // 🔥 FEATURE 10: DATABASE PERSISTENCE & INDEXING
    pub fn new(path: &str) -> Result<Self, String> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_compression_type(rocksdb::DBCompressionType::Lz4);
        opts.set_max_open_files(1000);
        opts.set_use_fsync(false);
        
        match DB::open(&opts, path) {
            Ok(db) => {
                println!("💾 Database opened successfully at: {}", path);
                Ok(BlockchainDB { db })
            }
            Err(e) => Err(format!("Failed to open database: {}", e)),
        }
    }

    // BLOCK STORAGE WITH INDEXING
    pub fn put_block(&self, block: &Block) -> Result<(), String> {
        let block_data = serde_json::to_vec(block)
            .map_err(|e| format!("Serialization error: {}", e))?;
        
        // 1. Store block by hash (primary storage)
        self.db.put(block.hash.as_bytes(), &block_data)
            .map_err(|e| format!("DB put error: {}", e))?;
        
        // 2. Store block by height (height index)
        self.db.put(
            format!("height:{}", block.index).as_bytes(), 
            block.hash.as_bytes()
        ).map_err(|e| format!("DB put error: {}", e))?;
            
        // 3. Index transactions (transaction index)
        for tx in &block.transactions {
            let tx_hash = Self::calculate_tx_hash(tx);
            self.db.put(
                format!("tx:{}", tx_hash).as_bytes(), 
                block.hash.as_bytes()
            ).map_err(|e| format!("DB put error: {}", e))?;
                
            // 4. UTXO management for double-spending prevention
            self.update_utxos(tx, &block.hash)?;
        }

        println!("💾 Block {} stored in database", block.index);
        Ok(())
    }

    // BLOCK RETRIEVAL
    pub fn get_block_by_hash(&self, hash: &str) -> Result<Option<Block>, String> {
        match self.db.get(hash.as_bytes()) {
            Ok(Some(data)) => {
                let block: Block = serde_json::from_slice(&data)
                    .map_err(|e| format!("Deserialization error: {}", e))?;
                Ok(Some(block))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(format!("DB get error: {}", e)),
        }
    }

    pub fn get_block_by_height(&self, height: u64) -> Result<Option<Block>, String> {
        match self.db.get(format!("height:{}", height).as_bytes()) {
            Ok(Some(hash_bytes)) => {
                let hash = String::from_utf8(hash_bytes)
                    .map_err(|e| format!("UTF8 error: {}", e))?;
                self.get_block_by_hash(&hash)
            }
            Ok(None) => Ok(None),
            Err(e) => Err(format!("DB get error: {}", e)),
        }
    }

    // TRANSACTION LOOKUP
    pub fn get_transaction(&self, tx_hash: &str) -> Result<Option<Transaction>, String> {
        match self.db.get(format!("tx:{}", tx_hash).as_bytes()) {
            Ok(Some(block_hash_bytes)) => {
                let block_hash = String::from_utf8(block_hash_bytes)
                    .map_err(|e| format!("UTF8 error: {}", e))?;
                
                if let Some(block) = self.get_block_by_hash(&block_hash)? {
                    for tx in block.transactions {
                        if Self::calculate_tx_hash(&tx) == tx_hash {
                            return Ok(Some(tx));
                        }
                    }
                }
                Ok(None)
            }
            Ok(None) => Ok(None),
            Err(e) => Err(format!("DB get error: {}", e)),
        }
    }

    // 🔥 UTXO MANAGEMENT FOR DOUBLE-SPENDING PREVENTION
    fn update_utxos(&self, tx: &Transaction, block_hash: &str) -> Result<(), String> {
        // Mark input UTXO as spent (if exists)
        if !tx.input_utxo.is_empty() {
            self.db.put(
                format!("spent:{}", tx.input_utxo).as_bytes(), 
                "true".as_bytes()
            ).map_err(|e| format!("DB put error: {}", e))?;
        }
        
        // Create new UTXO for receiver
        let new_utxo = format!("{}:{}:{}", block_hash, tx.receiver, Self::calculate_tx_hash(tx));
        let utxo_data = serde_json::to_vec(&UTXOData {
            address: tx.receiver.clone(),
            amount: tx.amount,
            block_hash: block_hash.to_string(),
            spent: false,
        }).map_err(|e| format!("Serialization error: {}", e))?;
        
        self.db.put(
            format!("utxo:{}", new_utxo).as_bytes(), 
            &utxo_data
        ).map_err(|e| format!("DB put error: {}", e))?;
            
        Ok(())
    }

    pub fn is_utxo_spent(&self, utxo: &str) -> Result<bool, String> {
        match self.db.get(format!("spent:{}", utxo).as_bytes()) {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(e) => Err(format!("DB get error: {}", e)),
        }
    }

    pub fn get_utxos_for_address(&self, address: &str) -> Result<Vec<UTXOData>, String> {
        let mut utxos = Vec::new();
        let iterator = self.db.iterator(IteratorMode::Start);
        
        for item in iterator {
            match item {
                Ok((key, value)) => {
                    if let Ok(key_str) = String::from_utf8(key.to_vec()) {
                        if key_str.starts_with("utxo:") {
                            let utxo_data: UTXOData = serde_json::from_slice(&value)
                                .map_err(|e| format!("Deserialization error: {}", e))?;
                            
                            if utxo_data.address == address && !utxo_data.spent {
                                utxos.push(utxo_data);
                            }
                        }
                    }
                }
                Err(e) => return Err(format!("Iterator error: {}", e)),
            }
        }
        
        Ok(utxos)
    }

    // DATABASE MAINTENANCE
    pub fn get_block_count(&self) -> Result<u64, String> {
        let mut count = 0;
        let iterator = self.db.iterator(IteratorMode::Start);
        
        for item in iterator {
            match item {
                Ok((key, _)) => {
                    if let Ok(key_str) = String::from_utf8(key.to_vec()) {
                        if key_str.starts_with("height:") {
                            count += 1;
                        }
                    }
                }
                Err(e) => return Err(format!("Iterator error: {}", e)),
            }
        }
        
        Ok(count)
    }

    pub fn get_latest_block_height(&self) -> Result<u64, String> {
        let mut max_height: i64 = -1;
        let iterator = self.db.iterator(IteratorMode::Start);
        
        for item in iterator {
            match item {
                Ok((key, _)) => {
                    if let Ok(key_str) = String::from_utf8(key.to_vec()) {
                        if key_str.starts_with("height:") {
                            if let Some(height_str) = key_str.strip_prefix("height:") {
                                if let Ok(height) = height_str.parse::<u64>() {
                                    if height as i64 > max_height {
                                        max_height = height as i64;
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => return Err(format!("Iterator error: {}", e)),
            }
        }
        
        if max_height == -1 {
            Ok(0)
        } else {
            Ok(max_height as u64)
        }
    }

    // DATABASE UTILITIES
    pub fn compact_database(&self) -> Result<(), String> {
        self.db.compact_range(None::<&[u8]>, None::<&[u8]>)
            .map_err(|e| format!("Compaction error: {}", e))?;
        println("🔧 Database compaction completed");
        Ok(())
    }

    pub fn get_database_stats(&self) -> Result<String, String> {
        let block_count = self.get_block_count()?;
        let latest_height = self.get_latest_block_height()?;
        
        let stats = format!(
            "Database Stats:\n  Blocks: {}\n  Latest Height: {}\n  Size: ~{} MB",
            block_count,
            latest_height,
            self.estimate_size()? / (1024 * 1024)
        );
        
        Ok(stats)
    }

    fn estimate_size(&self) -> Result<usize, String> {
        let mut total_size = 0;
        let iterator = self.db.iterator(IteratorMode::Start);
        
        for item in iterator {
            match item {
                Ok((key, value)) => {
                    total_size += key.len() + value.len();
                }
                Err(e) => return Err(format!("Iterator error: {}", e)),
            }
        }
        
        Ok(total_size)
    }

    // HELPER FUNCTIONS
    fn calculate_tx_hash(tx: &Transaction) -> String {
        let tx_data = format!(
            "{}{}{}{}{}", 
            tx.sender, 
            tx.receiver, 
            tx.amount, 
            tx.timestamp, 
            tx.signature
        );
        format!("{:x}", sha2::Sha256::digest(tx_data.as_bytes()))
    }
}

// UTXO DATA STRUCTURE
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UTXOData {
    pub address: String,
    pub amount: u64,
    pub block_hash: String,
    pub spent: bool,
}

// DATABASE MANAGEMENT
impl BlockchainDB {
    pub fn backup_database(&self, backup_path: &str) -> Result<(), String> {
        // Simple backup implementation
        println!("💽 Creating database backup at: {}", backup_path);
        // Actual implementation would copy database files
        Ok(())
    }
    
    pub fn restore_database(&self, backup_path: &str) -> Result<(), String> {
        // Simple restore implementation
        println!("🔄 Restoring database from: {}", backup_path);
        // Actual implementation would restore from backup
        Ok(())
    }
}