use crate::models::Block;
use crate::p2p::P2PNetwork;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub struct BlockSync {
    pub sync_in_progress: bool,
    pub sync_progress: f64,
    pub current_height: u64,
    pub target_height: u64,
    pub checkpoint_heights: Vec<u64>,
    pub batch_size: usize,
    pub retry_count: u32,
}

impl BlockSync {
    // 🔥 FEATURE 9: INITIAL BLOCK DOWNLOAD OPTIMIZATION
    pub fn new() -> Self {
        BlockSync {
            sync_in_progress: false,
            sync_progress: 0.0,
            current_height: 0,
            target_height: 0,
            checkpoint_heights: vec![0, 500, 1000, 5000, 10000], // Trusted checkpoints
            batch_size: 100, // Blocks per batch
            retry_count: 0,
        }
    }

    pub async fn start_initial_sync(&mut self, network: &P2PNetwork) -> Result<Vec<Block>, String> {
        if self.sync_in_progress {
            return Err("Sync already in progress".to_string());
        }

        self.sync_in_progress = true;
        println!("📥 Starting Initial Block Download (IBD)...");

        // Get best height from multiple peers
        self.target_height = self.get_consensus_height(network).await?;
        println!("🎯 Target chain height: {}", self.target_height);

        if self.target_height == 0 {
            self.sync_in_progress = false;
            return Ok(Vec::new());
        }

        let mut all_blocks = Vec::new();
        let mut start_height = 0;

        // Download in parallel batches for speed
        while start_height <= self.target_height {
            let end_height = std::cmp::min(start_height + self.batch_size as u64, self.target_height);
            
            match self.download_block_batch(start_height, end_height, network).await {
                Ok(batch) => {
                    all_blocks.extend(batch);
                    self.current_height = end_height;
                    self.update_progress();
                    
                    println!("✅ Downloaded {}/{} blocks ({:.1}%)", 
                             self.current_height, self.target_height, self.sync_progress);
                }
                Err(e) => {
                    println!("⚠️ Batch download failed: {}. Retrying...", e);
                    self.retry_count += 1;
                    
                    if self.retry_count > 3 {
                        return Err(format!("Failed after {} retries", self.retry_count));
                    }
                    
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    continue;
                }
            }
            
            start_height = end_height + 1;
            self.retry_count = 0; // Reset retry count on success
        }

        self.sync_in_progress = false;
        println!("🎉 IBD Completed! Downloaded {} blocks", all_blocks.len());
        
        Ok(all_blocks)
    }

    async fn download_block_batch(
        &self, 
        start: u64, 
        end: u64, 
        network: &P2PNetwork
    ) -> Result<Vec<Block>, String> {
        println!("⬇️ Downloading blocks {}-{}", start, end);
        
        let peers = self.get_reliable_peers(network).await;
        if peers.is_empty() {
            return Err("No reliable peers available".to_string());
        }

        // Try peers in order of reliability
        for peer in peers {
            match self.request_blocks_from_peer(&peer, start, end).await {
                Ok(blocks) => {
                    if self.validate_block_batch(&blocks, start, end) {
                        println!("✅ Successfully downloaded from {}", peer);
                        return Ok(blocks);
                    } else {
                        println!("❌ Invalid batch from {}", peer);
                    }
                }
                Err(e) => {
                    println!("⚠️ Failed to download from {}: {}", peer, e);
                }
            }
        }

        Err("All peers failed to provide valid block batch".to_string())
    }

    async fn get_reliable_peers(&self, network: &P2PNetwork) -> Vec<String> {
        // Get peers sorted by reliability (simplified)
        let network_info = network.get_network_info().await;
        println!("🌐 Available peers: {}", network_info.connected_peers);
        
        // Mock reliable peers list - actual implementation would score peers
        vec![
            "127.0.0.1:8080".to_string(),
            "127.0.0.1:8081".to_string(),
            "127.0.0.1:8082".to_string(),
        ]
    }

    async fn request_blocks_from_peer(
        &self, 
        peer: &str, 
        start: u64, 
        end: u64
    ) -> Result<Vec<Block>, String> {
        // Simulated block download - actual implementation would use network calls
        println!("📡 Requesting blocks {}-{} from {}", start, end, peer);
        
        let mut blocks = Vec::new();
        
        // Generate mock blocks for testing
        for height in start..=end {
            let block = self.create_mock_block(height);
            blocks.push(block);
            
            // Simulate network delay
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        
        Ok(blocks)
    }

    fn validate_block_batch(&self, blocks: &[Block], expected_start: u64, expected_end: u64) -> bool {
        if blocks.is_empty() {
            return false;
        }

        // Check batch boundaries
        if blocks[0].index != expected_start || blocks.last().unwrap().index != expected_end {
            println!("❌ Batch boundaries mismatch");
            return false;
        }

        // Check chain continuity
        for i in 1..blocks.len() {
            if blocks[i].previous_hash != blocks[i-1].hash {
                println!("❌ Chain continuity broken at block {}", blocks[i].index);
                return false;
            }
        }

        // Check checkpoint alignment
        for &checkpoint in &self.checkpoint_heights {
            if let Some(block) = blocks.iter().find(|b| b.index == checkpoint) {
                if !self.verify_checkpoint(block) {
                    println!("❌ Checkpoint verification failed at height {}", checkpoint);
                    return false;
                }
            }
        }

        true
    }

    fn verify_checkpoint(&self, block: &Block) -> bool {
        // Simplified checkpoint verification
        // Actual implementation would verify against known checkpoint hashes
        !block.hash.is_empty() && block.hash.starts_with("0000")
    }

    async fn get_consensus_height(&self, network: &P2PNetwork) -> Result<u64, String> {
        // Query multiple peers and take the most common height
        println!("📊 Getting consensus height from network...");
        
        let mock_heights = vec![1000, 1002, 1001, 1000, 1000, 1003]; // Simulated peer responses
        
        // Find mode (most common value)
        let mut frequency = std::collections::HashMap::new();
        for &height in &mock_heights {
            *frequency.entry(height).or_insert(0) += 1;
        }
        
        let consensus_height = frequency
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(height, _)| height)
            .unwrap_or(0);
            
        println!("🎯 Consensus height: {}", consensus_height);
        Ok(consensus_height)
    }

    fn update_progress(&mut self) {
        if self.target_height > 0 {
            self.sync_progress = (self.current_height as f64 / self.target_height as f64) * 100.0;
        } else {
            self.sync_progress = 0.0;
        }
    }

    fn create_mock_block(&self, height: u64) -> Block {
        Block {
            index: height,
            timestamp: chrono::Utc::now().timestamp() as u64,
            transactions: Vec::new(),
            previous_hash: if height == 0 { 
                "0".to_string() 
            } else { 
                format!("mock_prev_hash_{}", height - 1)
            },
            hash: format!("mock_hash_{}", height),
            nonce: 0,
            difficulty: 4,
        }
    }

    // SYNC STATUS AND CONTROL
    pub fn get_sync_status(&self) -> SyncStatus {
        SyncStatus {
            in_progress: self.sync_in_progress,
            progress: self.sync_progress,
            current_height: self.current_height,
            target_height: self.target_height,
            retry_count: self.retry_count,
        }
    }

    pub fn pause_sync(&mut self) {
        if self.sync_in_progress {
            self.sync_in_progress = false;
            println("⏸️ Sync paused");
        }
    }

    pub fn resume_sync(&mut self) {
        if !self.sync_in_progress && self.current_height < self.target_height {
            self.sync_in_progress = true;
            println("▶️ Sync resumed");
        }
    }

    pub fn cancel_sync(&mut self) {
        self.sync_in_progress = false;
        self.sync_progress = 0.0;
        self.current_height = 0;
        self.target_height = 0;
        println("⏹️ Sync cancelled");
    }

    // FAST SYNC OPTIMIZATION
    pub async fn fast_sync(&mut self, network: &P2PNetwork) -> Result<Vec<Block>, String> {
        println!("⚡ Starting Fast Sync...");
        
        // Download only block headers first
        let headers = self.download_headers(network).await?;
        println!("📄 Downloaded {} block headers", headers.len());
        
        // Then download full blocks in parallel
        let blocks = self.download_blocks_parallel(headers, network).await?;
        
        println!("🎉 Fast Sync completed! {} blocks", blocks.len());
        Ok(blocks)
    }

    async fn download_headers(&self, network: &P2PNetwork) -> Result<Vec<Block>, String> {
        // Simulated header download
        let mut headers = Vec::new();
        for i in 0..100 {
            headers.push(self.create_mock_block(i));
        }
        Ok(headers)
    }

    async fn download_blocks_parallel(
        &self, 
        headers: Vec<Block>, 
        network: &P2PNetwork
    ) -> Result<Vec<Block>, String> {
        // Simulated parallel block download
        let mut blocks = Vec::new();
        for header in headers {
            blocks.push(self.create_mock_block(header.index));
        }
        Ok(blocks)
    }
}

// SYNC STATUS STRUCT
pub struct SyncStatus {
    pub in_progress: bool,
    pub progress: f64,
    pub current_height: u64,
    pub target_height: u64,
    pub retry_count: u32,
}

impl SyncStatus {
    pub fn display(&self) {
        println!("🔄 Sync Status:");
        println!("   Progress: {:.1}%", self.progress);
        println!("   Blocks: {}/{}", self.current_height, self.target_height);
        println!("   Status: {}", if self.in_progress { "SYNCING" } else { "IDLE" });
        println!("   Retries: {}", self.retry_count);
    }
}

// SYNC UTILITIES
impl BlockSync {
    pub fn estimate_time_remaining(&self) -> Duration {
        if self.sync_progress >= 100.0 || !self.sync_in_progress {
            return Duration::from_secs(0);
        }
        
        let remaining_blocks = self.target_height - self.current_height;
        let estimated_seconds = (remaining_blocks as f64 * 0.1).max(1.0); // 0.1 seconds per block
        
        Duration::from_secs(estimated_seconds as u64)
    }
    
    pub fn get_speed(&self) -> f64 {
        // Blocks per second
        if self.current_height > 0 {
            self.current_height as f64 / 10.0 // Simplified
        } else {
            0.0
        }
    }
}