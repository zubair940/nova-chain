use crate::models::{Block, Transaction, BlockchainData, Fork, NovContractManager, ContractTransaction, ContractExecutionResult};
use crate::database::BlockchainDB;
use std::collections::{HashMap, HashSet};
use sha2::{Sha256, Digest};
use serde_json;

// 🆕 NEW: Import all feature modules
use crate::token_burning::{TokenBurning, BurnEvent};
use crate::dao_governance::{DAOGovernance, Proposal, Vote};
use crate::cross_chain_bridge::{CrossChainBridge, BridgeTransaction};
use crate::security_audit::{SecurityAudit, AuditReport};

// 🚀 NEW: Import Advanced Features
use crate::ai_optimization::{AIOptimization, AIAnalysis};
use crate::quantum_resistance::{QuantumResistance, QuantumTransaction};
use crate::multi_signature::{MultiSignatureManager, MultiSigTransaction};
use crate::defi_advanced::{DeFiAdvanced, FlashLoan, YieldFarm};
use crate::gaming_metaverse::{GamingMetaverse, NFTAsset, VirtualLand};
use crate::enterprise_features::{EnterpriseManager, EnterpriseTransaction};
use crate::mass_adoption_v2::{MassAdoptionV2, SocialTransaction, MicroTask};
use crate::cross_chain_v2::{CrossChainV2, MultiChainTransaction};

// 🆕 NEW: UTXO STRUCT ADD KAREIN
#[derive(Debug, Clone)]
pub struct UTXO {
    pub tx_hash: String,
    pub output_index: u32,
    pub address: String,
    pub amount: u64,
    pub spent: bool,
}

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub difficulty: u64,
    // 🔥 FEATURE 6: DOUBLE-SPENDING PREVENTION
    pub spent_utxos: HashSet<String>,
    // 🔥 FEATURE 10: DATABASE PERSISTENCE
    pub db: Option<BlockchainDB>,
    // 🔥 FEATURE 8: FORK RESOLUTION
    pub forks: Vec<Fork>,
    // 🔥 NEW: SMART CONTRACTS
    pub contract_manager: VexContractManager,
    
    // 🆕 NEW: TOKEN BURNING FEATURE
    pub token_burning: TokenBurning,
    
    // 🆕 NEW: DAO GOVERNANCE FEATURE  
    pub dao_governance: DAOGovernance,
    
    // 🆕 NEW: CROSS-CHAIN BRIDGE FEATURE
    pub cross_chain_bridge: CrossChainBridge,
    
    // 🆕 NEW: SECURITY AUDIT FEATURE
    pub security_audit: SecurityAudit,
    
    // 🚀 NEW: ADVANCED FEATURES
    pub ai_optimization: AIOptimization,              // 🚀 AI-powered optimization
    pub quantum_resistance: QuantumResistance,        // 🚀 Quantum-safe blockchain
    pub multi_signature: MultiSignatureManager,       // 🚀 Multi-signature support
    pub defi_advanced: DeFiAdvanced,                  // 🚀 Advanced DeFi features
    pub gaming_metaverse: GamingMetaverse,            // 🚀 Gaming & metaverse
    pub enterprise_features: EnterpriseManager,       // 🚀 Enterprise solutions
    pub mass_adoption_v2: MassAdoptionV2,             // 🚀 Enhanced mass adoption
    pub cross_chain_v2: CrossChainV2,                 // 🚀 Multi-chain integration
    
    // 🆕 FIX: UTXO SET ADD KAREIN - YEH MISSING THA!
    pub utxo_set: HashMap<String, UTXO>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            chain: Vec::new(),
            pending_transactions: Vec::new(),
            difficulty: 4,
            spent_utxos: HashSet::new(),
            db: None,
            forks: Vec::new(),
            // 🔥 NEW: Initialize contract manager
            contract_manager: VexContractManager::new(),
            
            // 🆕 NEW: Initialize all feature modules
            token_burning: TokenBurning::new(),
            dao_governance: DAOGovernance::new(),
            cross_chain_bridge: CrossChainBridge::new(),
            security_audit: SecurityAudit::new(),
            
            // 🚀 NEW: Initialize advanced features
            ai_optimization: AIOptimization::new(),
            quantum_resistance: QuantumResistance::new(),
            multi_signature: MultiSignatureManager::new(),
            defi_advanced: DeFiAdvanced::new(),
            gaming_metaverse: GamingMetaverse::new(),
            enterprise_features: EnterpriseManager::new(),
            mass_adoption_v2: MassAdoptionV2::new(),
            cross_chain_v2: CrossChainV2::new(),
            
            // 🆕 FIX: Initialize UTXO set
            utxo_set: HashMap::new(),
        }
    }

    // 🏆 NEW: UPDATED TOKENOMICS CONSTANTS
    pub const TOTAL_SUPPLY: u64 = 100_000_000; // 100M VEXA
    pub const INITIAL_CIRCULATING: u64 = 25_000_000; // 25M initially circulating

    // 🏆 NEW: TOKENOMICS ALLOCATIONS
    pub const FOUNDATION_TEAM: u64 = 20_000_000;    // 20%
    pub const STRATEGIC_PARTNERS: u64 = 15_000_000; // 15%
    pub const PUBLIC_DISTRIBUTION: u64 = 35_000_000; // 35%
    pub const LIQUIDITY_GROWTH: u64 = 30_000_000;   // 30%

    // 🏆 NEW: VESTING SCHEDULES
    pub const TEAM_VESTING_MONTHS: u64 = 48; // 4 years
    pub const TEAM_CLIFF_MONTHS: u64 = 12;   // 1 year cliff
    pub const INVESTOR_VESTING_MONTHS: u64 = 24; // 2 years
    pub const INVESTOR_CLIFF_MONTHS: u64 = 6;    // 6 months cliff

    // 🚀 NEW: Initialize all advanced features
    pub fn initialize_advanced_features(&mut self) {
        println!("🚀 Initializing Advanced VexaChain Features...");
        
        // Initialize AI Optimization
        self.ai_optimization.initialize();
        println!("  🤖 AI Optimization: READY");
        
        // Initialize Quantum Resistance
        self.quantum_resistance.initialize();
        println!("  ⚛️ Quantum Resistance: READY");
        
        // Initialize Multi-signature
        self.multi_signature.initialize();
        println!("  🔐 Multi-signature System: READY");
        
        // Initialize Advanced DeFi
        self.defi_advanced.initialize();
        println!("  💰 Advanced DeFi: READY");
        
        // Initialize Gaming & Metaverse
        self.gaming_metaverse.initialize();
        println!("  🎮 Gaming & Metaverse: READY");
        
        // Initialize Enterprise Features
        self.enterprise_features.initialize();
        println!("  🏢 Enterprise Solutions: READY");
        
        // Initialize Mass Adoption V2
        self.mass_adoption_v2.initialize();
        println!("  🌍 Mass Adoption V2: READY");
        
        // Initialize Cross-chain V2
        self.cross_chain_v2.initialize();
        println!("  🌉 Multi-chain Integration: READY");
        
        println!("🎉 All Advanced Features Initialized Successfully!");
    }

    // 🏆 NEW: DISPLAY UPDATED TOKENOMICS
    pub fn display_updated_tokenomics(&self) {
        println!("\n🏆 VEXA TOKENOMICS – $1-2 PRICE TARGET");
        println!("──────────────────────────────────────────────────────");
        println!("💰 Total Supply: {} VEXA", Self::TOTAL_SUPPLY);
        println!("💰 Initial Circulating: {} VEXA", Self::INITIAL_CIRCULATING);
        println!("🏢 Foundation & Team ({}M):", Self::FOUNDATION_TEAM / 1_000_000);
        println!("  Core Team: 8M (4yr vesting) | Foundation: 7M | Dev: 5M");
        println!("💰 Strategic Partners ({}M):", Self::STRATEGIC_PARTNERS / 1_000_000);
        println!("  Seed: 6M | Strategic: 5M | Ecosystem: 4M");
        println!("🌍 Public Distribution ({}M):", Self::PUBLIC_DISTRIBUTION / 1_000_000);
        println!("  Public Sale: 15M | Staking: 10M | Community: 6M | Airdrop: 4M");
        println!("🔒 Liquidity & Growth ({}M):", Self::LIQUIDITY_GROWTH / 1_000_000);
        println!("  DEX: 8M | CEX: 7M | Marketing: 5M | Partnerships: 4M");
        println!("  Tech Grants: 3M | Security: 3M");
        println!("📊 Vesting Schedules:");
        println!("  Team: {} months ({} month cliff)", Self::TEAM_VESTING_MONTHS, Self::TEAM_CLIFF_MONTHS);
        println!("  Investors: {} months ({} month cliff)", Self::INVESTOR_VESTING_MONTHS, Self::INVESTOR_CLIFF_MONTHS);
    }

    // 🏆 NEW: GET TOKEN DISTRIBUTION
    pub fn get_token_distribution(&self) -> HashMap<String, u64> {
        let mut distribution = HashMap::new();
        
        // 🏢 FOUNDATION & TEAM (20M)
        distribution.insert("Core_Development_Team".to_string(), 8_000_000);
        distribution.insert("Foundation_Reserve".to_string(), 7_000_000);
        distribution.insert("Development_Fund".to_string(), 5_000_000);
        
        // 💰 STRATEGIC PARTNERS (15M)
        distribution.insert("Seed_Investors".to_string(), 6_000_000);
        distribution.insert("Strategic_Partners".to_string(), 5_000_000);
        distribution.insert("Ecosystem_Fund".to_string(), 4_000_000);
        
        // 🌍 PUBLIC DISTRIBUTION (35M)
        distribution.insert("Public_Sale".to_string(), 15_000_000);
        distribution.insert("Staking_Rewards".to_string(), 10_000_000);
        distribution.insert("Community_Rewards".to_string(), 6_000_000);
        distribution.insert("Airdrop_Program".to_string(), 4_000_000);
        
        // 🔒 LIQUIDITY & GROWTH (30M)
        distribution.insert("DEX_Liquidity".to_string(), 8_000_000);
        distribution.insert("CEX_Liquidity".to_string(), 7_000_000);
        distribution.insert("Marketing_Fund".to_string(), 5_000_000);
        distribution.insert("Partnership_Fund".to_string(), 4_000_000);
        distribution.insert("Technology_Grants".to_string(), 3_000_000);
        distribution.insert("Security_Reserve".to_string(), 3_000_000);
        
        distribution
    }

    // 🏆 NEW: CALCULATE CIRCULATING SUPPLY
    pub fn calculate_circulating_supply(&self) -> u64 {
        let mut circulating = Self::INITIAL_CIRCULATING;
        
        // Add tokens from public distribution that are immediately available
        circulating += 15_000_000; // Public Sale
        circulating += 4_000_000;  // Airdrop Program
        
        circulating
    }

    // 🆕 NEW: Initialize all features
    pub fn initialize_features(&mut self) {
        println!("🚀 Initializing All VexaChain Features...");
        
        // Display updated tokenomics
        self.display_updated_tokenomics();
        
        // Initialize Token Burning
        self.token_burning.initialize();
        println!("  ✅ Token Burning System: READY");
        
        // Initialize DAO Governance
        self.dao_governance.initialize();
        println!("  ✅ DAO Governance System: READY");
        
        // Initialize Cross-chain Bridge
        self.cross_chain_bridge.initialize();
        println!("  ✅ Cross-chain Bridge: READY");
        
        // Run initial security audit
        let audit_report = self.security_audit.run_initial_audit(&self.chain);
        println!("  ✅ Security Audit: COMPLETED - {}", audit_report.overall_status);
        
        // 🚀 NEW: Initialize advanced features
        self.initialize_advanced_features();
        
        println!("🎉 All VexaChain Features Initialized Successfully!");
    }

    // 🔥 FEATURE 6: DOUBLE-SPENDING PREVENTION - FIXED!
    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<bool, String> {
        // 1. Signature verification
        if !self.verify_transaction_signature(&transaction) {
            return Ok(false);
        }
        
        // 2. Double-spending check - FIXED!
        if self.is_double_spend(&transaction) {
            println!("❌ Double-spend detected from {}", transaction.sender);
            return Ok(false);
        }
        
        // 3. Sufficient balance check
        if !self.has_sufficient_balance(&transaction) {
            println!("❌ Insufficient balance for {}", transaction.sender);
            return Ok(false);
        }
        
        // 🚀 NEW: Advanced transaction processing
        self.process_advanced_transaction(&transaction);
        
        // 🆕 NEW: Token burning transaction processing
        if transaction.transaction_type == "burn" {
            self.process_token_burning(transaction.clone());
        }
        
        // 🆕 FIX: Create UTXO for receiver IMMEDIATELY
        let tx_hash = self.calculate_transaction_hash(&transaction);
        let receiver_utxo = UTXO {
            tx_hash: tx_hash.clone(),
            output_index: 0,
            address: transaction.receiver.clone(),
            amount: transaction.amount,
            spent: false,
        };
        self.utxo_set.insert(tx_hash, receiver_utxo);
        
        self.pending_transactions.push(transaction);
        println!("✅ Transaction added to pending pool");
        Ok(true)
    }

    // 🚀 NEW: Process advanced transaction types
    fn process_advanced_transaction(&mut self, transaction: &Transaction) {
        match transaction.transaction_type.as_str() {
            "quantum" => {
                let quantum_tx = QuantumTransaction::from_transaction(transaction);
                self.quantum_resistance.process_quantum_transaction(quantum_tx);
                println!("⚛️ Quantum transaction processed");
            }
            "multisig" => {
                let multisig_tx = MultiSigTransaction::from_transaction(transaction);
                self.multi_signature.process_multisig_transaction(multisig_tx);
                println!("🔐 Multi-signature transaction processed");
            }
            "flash_loan" => {
                let flash_loan = FlashLoan::from_transaction(transaction);
                self.defi_advanced.process_flash_loan(flash_loan);
                println!("💰 Flash loan processed");
            }
            "nft" => {
                let nft_asset = NFTAsset::from_transaction(transaction);
                self.gaming_metaverse.process_nft_transaction(nft_asset);
                println!("🎨 NFT transaction processed");
            }
            "enterprise" => {
                let enterprise_tx = EnterpriseTransaction::from_transaction(transaction);
                self.enterprise_features.process_enterprise_transaction(enterprise_tx);
                println!("🏢 Enterprise transaction processed");
            }
            "social" => {
                let social_tx = SocialTransaction::from_transaction(transaction);
                self.mass_adoption_v2.process_social_transaction(social_tx);
                println!("📱 Social transaction processed");
            }
            "multichain" => {
                let multichain_tx = MultiChainTransaction::from_transaction(transaction);
                self.cross_chain_v2.process_multichain_transaction(multichain_tx);
                println!("🌉 Multi-chain transaction processed");
            }
            _ => {} // Regular transaction
        }
    }

    // 🆕 NEW: Process token burning transactions
    fn process_token_burning(&mut self, transaction: Transaction) {
        let burn_event = BurnEvent {
            burner: transaction.sender.clone(),
            amount: transaction.amount,
            block_height: self.chain.len() as u64,
            timestamp: transaction.timestamp,
            tx_hash: self.calculate_transaction_hash(&transaction),
        };
        
        self.token_burning.record_burn(burn_event);
        println!("🔥 Token burning processed: {} VEXA burned", transaction.amount);
    }

    // 🆕 NEW: Calculate transaction hash
    fn calculate_transaction_hash(&self, transaction: &Transaction) -> String {
        let data = format!("{}{}{}{}", 
            transaction.sender, 
            transaction.receiver, 
            transaction.amount, 
            transaction.timestamp
        );
        format!("{:x}", Sha256::digest(data.as_bytes()))
    }

    // 🔥 FIXED DOUBLE SPEND DETECTION!
    fn is_double_spend(&self, transaction: &Transaction) -> bool {
        // Special cases - allow these transaction types without double spend check
        if transaction.transaction_type == "burn" || 
           transaction.transaction_type == "stake" ||
           transaction.transaction_type == "contract" ||
           transaction.transaction_type == "reward" ||
           transaction.transaction_type == "quantum" ||
           transaction.transaction_type == "multisig" ||
           transaction.transaction_type == "flash_loan" ||
           transaction.transaction_type == "nft" ||
           transaction.transaction_type == "enterprise" ||
           transaction.transaction_type == "social" ||
           transaction.transaction_type == "multichain" ||
           transaction.sender == "Core_Development_Team" ||
           transaction.sender == "Foundation_Reserve" ||
           transaction.sender == "Development_Fund" ||
           transaction.sender == "Seed_Investors" ||
           transaction.sender == "Strategic_Partners" ||
           transaction.sender == "Ecosystem_Fund" ||
           transaction.sender == "Public_Sale" ||
           transaction.sender == "Staking_Rewards" ||
           transaction.sender == "Community_Rewards" ||
           transaction.sender == "Airdrop_Program" ||
           transaction.sender == "DEX_Liquidity" ||
           transaction.sender == "CEX_Liquidity" ||
           transaction.sender == "Marketing_Fund" ||
           transaction.sender == "Partnership_Fund" ||
           transaction.sender == "Technology_Grants" ||
           transaction.sender == "Security_Reserve" {
            return false;
        }
        
        // Skip check for genesis transactions and empty UTXOs
        if transaction.input_utxo == "genesis" || 
           transaction.input_utxo.is_empty() ||
           transaction.input_utxo == "burn_utxo" ||
           transaction.input_utxo == "stake_utxo" {
            return false;
        }
        
        // Check if input UTXO is already spent
        if self.spent_utxos.contains(&transaction.input_utxo) {
            println!("❌ UTXO already spent: {}", transaction.input_utxo);
            return true;
        }
        
        // Check pending transactions for same sender and UTXO
        for pending_tx in &self.pending_transactions {
            if pending_tx.sender == transaction.sender && 
               pending_tx.input_utxo == transaction.input_utxo &&
               pending_tx.transaction_type != "burn" &&
               pending_tx.transaction_type != "stake" {
                println!("❌ Same UTXO in pending: {}", transaction.input_utxo);
                return true;
            }
        }
        
        false
    }

    fn has_sufficient_balance(&self, transaction: &Transaction) -> bool {
        let balance = self.get_balance(&transaction.sender);
        balance >= transaction.amount
    }

    // 🆕 FIX: COMPLETELY REWRITTEN get_balance FUNCTION
    pub fn get_balance(&self, address: &str) -> u64 {
        let mut balance = 0u64;
        
        // 🆕 FIX: Use UTXO set to calculate balance
        for utxo in self.utxo_set.values() {
            if utxo.address == address && !utxo.spent {
                balance += utxo.amount;
            }
        }
        
        // Subtract pending outgoing transactions
        for tx in &self.pending_transactions {
            if tx.sender == address {
                balance = balance.saturating_sub(tx.amount);
            }
        }
        
        println!("🔍 Balance check for {}: {} VEXA", address, balance);
        balance
    }

    // 🔥 FEATURE 8: FORK RESOLUTION
    pub fn resolve_forks(&mut self) -> bool {
        if self.forks.is_empty() {
            return false;
        }

        // Find the fork with highest cumulative difficulty
        let best_fork = self.forks.iter()
            .max_by_key(|fork| fork.total_difficulty)
            .unwrap();

        if best_fork.total_difficulty > self.calculate_chain_difficulty(&self.chain) {
            println!("🔀 Fork resolved: Switching to chain with higher difficulty ({})", 
                     best_fork.total_difficulty);
            self.chain = best_fork.blocks.clone();
            self.forks.clear();
            return true;
        }

        false
    }

    pub fn add_competing_block(&mut self, block: Block) {
        let mut fork_chain = self.chain.clone();
        fork_chain.push(block);
        
        let fork = Fork {
            blocks: fork_chain,
            total_difficulty: self.calculate_chain_difficulty(&fork_chain),
            length: self.chain.len() + 1,
        };
        
        self.forks.push(fork);
        println!("⚠️ Competing block received, fork created");
    }

    fn calculate_chain_difficulty(&self, chain: &[Block]) -> u64 {
        chain.iter().map(|block| block.difficulty).sum()
    }

    // BLOCK MINING FUNCTIONS
    pub fn mine_block(&mut self, miner_address: String) -> Option<Block> {
        if self.pending_transactions.is_empty() {
            return None;
        }

        let previous_hash = if self.chain.is_empty() {
            "0".to_string()
        } else {
            self.chain.last().unwrap().hash.clone()
        };

        let mut block = Block {
            index: self.chain.len() as u64,
            timestamp: chrono::Utc::now().timestamp() as u64,
            transactions: self.pending_transactions.clone(),
            previous_hash,
            hash: String::new(),
            nonce: 0,
            difficulty: self.difficulty,
        };

        // Proof of Work
        while !self.is_valid_hash(&block.hash) {
            block.nonce += 1;
            block.hash = self.calculate_hash(&block);
        }

        // 🆕 FIX: Update UTXO set properly when mining block
        for tx in &block.transactions {
            let tx_hash = self.calculate_transaction_hash(tx);
            
            // Mark sender's UTXO as spent
            if tx.sender != "MINING_REWARD" && tx.sender != "GENESIS" && !tx.input_utxo.is_empty() {
                self.spent_utxos.insert(tx.input_utxo.clone());
                
                // Also mark in UTXO set
                if let Some(utxo) = self.utxo_set.get_mut(&tx.input_utxo) {
                    utxo.spent = true;
                }
            }
            
            // Create UTXO for receiver if not already exists
            if !self.utxo_set.contains_key(&tx_hash) {
                let receiver_utxo = UTXO {
                    tx_hash: tx_hash.clone(),
                    output_index: 0,
                    address: tx.receiver.clone(),
                    amount: tx.amount,
                    spent: false,
                };
                self.utxo_set.insert(tx_hash, receiver_utxo);
            }
        }

        self.chain.push(block.clone());
        self.pending_transactions.clear();

        // 🚀 NEW: Process advanced features after mining
        self.process_advanced_post_mining_features(&block);

        // 🆕 NEW: Process features after mining
        self.process_post_mining_features(&block);

        println!("⛏️ Mined Block {}: {}", block.index, block.hash);
        
        // 🆕 FIX: Debug balances after mining
        self.debug_balances();
        
        Some(block)
    }

    // 🆕 NEW: Debug function to check balances
    pub fn debug_balances(&self) {
        println!("🔍 DEBUG BALANCES AFTER MINING:");
        let test_wallets = vec!["3FigQmY4dsaAYT1QGCt7tvCKKKuL", "KHVNtQv7Lyu1d2Zf7U77WJ1hQoN"];
        for wallet in test_wallets {
            let balance = self.get_balance(wallet);
            println!("  {}: {} VEXA", wallet, balance);
        }
        println!("  UTXO Set Size: {}", self.utxo_set.len());
        println!("  Spent UTXOs: {}", self.spent_utxos.len());
    }

    // 🚀 NEW: Process advanced features after mining
    fn process_advanced_post_mining_features(&mut self, block: &Block) {
        // AI Optimization Analysis
        let ai_analysis = self.ai_optimization.analyze_block(block);
        if ai_analysis.recommend_adjustment {
            self.adjust_blockchain_parameters(&ai_analysis);
        }
        
        // Quantum Resistance Update
        self.quantum_resistance.update_quantum_state(block.index);
        
        // DeFi Yield Distribution
        self.defi_advanced.distribute_yield_rewards(block.index);
        
        // Gaming & Metaverse Updates
        self.gaming_metaverse.update_game_states(block.index);
        
        // Mass Adoption Rewards
        self.mass_adoption_v2.distribute_daily_rewards(block.index);
    }

    // 🚀 NEW: Adjust blockchain parameters based on AI analysis
    fn adjust_blockchain_parameters(&mut self, analysis: &AIAnalysis) {
        if analysis.recommended_difficulty != self.difficulty {
            println!("🤖 AI adjusting difficulty: {} -> {}", self.difficulty, analysis.recommended_difficulty);
            self.difficulty = analysis.recommended_difficulty;
        }
        
        // Adjust other parameters based on AI recommendations
        if analysis.network_health < 0.7 {
            println!("🤖 AI detected network health issues, adjusting parameters...");
            // Implement network health improvements
        }
    }

    // 🆕 NEW: Process all features after mining a block
    fn process_post_mining_features(&mut self, block: &Block) {
        // Process token burning for the block
        let burned_in_block: u64 = block.transactions.iter()
            .filter(|tx| tx.transaction_type == "burn")
            .map(|tx| tx.amount)
            .sum();
            
        if burned_in_block > 0 {
            self.token_burning.update_total_burned(burned_in_block);
            println!("🔥 Block {} burned {} VEXA tokens", block.index, burned_in_block);
        }
        
        // Process DAO proposals
        self.dao_governance.process_new_block(block.index);
        
        // Run security audit
        self.security_audit.analyze_block(block);
    }

    fn calculate_hash(&self, block: &Block) -> String {
        let data = format!("{}{:?}{}{}{}", 
            block.index, 
            block.transactions, 
            block.timestamp, 
            block.previous_hash, 
            block.nonce
        );
        format!("{:x}", Sha256::digest(data.as_bytes()))
    }

    fn is_valid_hash(&self, hash: &str) -> bool {
        hash.starts_with(&"0".repeat(self.difficulty as usize))
    }

    fn verify_transaction_signature(&self, transaction: &Transaction) -> bool {
        // Simple signature verification for now
        !transaction.signature.is_empty()
    }

    // 🔥 FEATURE 10: DATABASE INTEGRATION
    pub fn save_to_database(&self) -> Result<(), String> {
        if let Some(db) = &self.db {
            for block in &self.chain {
                db.put_block(block)?;
            }
            println!("💾 Blockchain saved to database");
        }
        Ok(())
    }

    pub fn load_from_database(&mut self) -> Result<(), String> {
        if let Some(db) = &self.db {
            let mut height = 0;
            while let Some(block) = db.get_block_by_height(height)? {
                self.chain.push(block);
                height += 1;
            }
            println!("📦 Loaded {} blocks from database", self.chain.len());
        }
        Ok(())
    }

    pub fn initialize_database(&mut self, path: &str) -> Result<(), String> {
        self.db = Some(BlockchainDB::new(path)?);
        Ok(())
    }

    // 🔥 NEW: SMART CONTRACT FUNCTIONS
    pub fn deploy_contract(&mut self, code: String, owner: String) -> String {
        let current_block = self.chain.len() as u64;
        let contract_address = self.contract_manager.deploy_contract(code, owner, current_block);
        println!("🚀 VEX Smart Contract deployed: {}", contract_address);
        contract_address
    }

    pub fn execute_contract(&mut self, contract_tx: ContractTransaction) -> ContractExecutionResult {
        println!("🤖 Executing VEX Contract: {}", contract_tx.contract_address);
        
        let result = self.contract_manager.execute_contract(
            &contract_tx.contract_address,
            &contract_tx.caller,
            &contract_tx.function,
            contract_tx.args,
        );

        if result.success {
            println!("✅ Contract execution successful: {}", result.output);
        } else {
            println!("❌ Contract execution failed: {}", result.output);
        }

        result
    }

    pub fn fund_contract(&mut self, contract_address: &str, amount: u64) -> bool {
        let success = self.contract_manager.fund_contract(contract_address, amount);
        if success {
            println!("💰 Contract {} funded with {} VEX", contract_address, amount);
        } else {
            println!("❌ Failed to fund contract: {}", contract_address);
        }
        success
    }

    pub fn get_contract_balance(&self, contract_address: &str) -> u64 {
        self.contract_manager.get_contract_balance(contract_address)
    }

    // 🆕 NEW: TOKEN BURNING FUNCTIONS
    pub fn burn_tokens(&mut self, burner_address: String, amount: u64) -> bool {
        let balance = self.get_balance(&burner_address);
        if amount > balance {
            println!("❌ Burn failed: Insufficient balance");
            return false;
        }

        // Create burn transaction
        let burn_transaction = Transaction {
            sender: burner_address.clone(),
            receiver: "BURN_ADDRESS".to_string(),
            amount,
            fee: 0,
            timestamp: chrono::Utc::now().timestamp() as u64,
            signature: "burn_signature".to_string(),
            public_key: "burn_public_key".to_string(),
            input_utxo: "burn_utxo".to_string(),
            nonce: 0,
            transaction_type: "burn".to_string(),
        };

        // Add to pending transactions
        if let Ok(true) = self.add_transaction(burn_transaction) {
            println!("🔥 Burn transaction created: {} VEXA", amount);
            return true;
        }

        false
    }

    pub fn get_total_burned(&self) -> u64 {
        self.token_burning.total_burned
    }

    pub fn get_burn_history(&self) -> Vec<BurnEvent> {
        self.token_burning.get_burn_history()
    }

    // 🆕 NEW: DAO GOVERNANCE FUNCTIONS
    pub fn create_dao_proposal(&mut self, creator: String, title: String, description: String, voting_duration: u64) -> u64 {
        let proposal_id = self.dao_governance.create_proposal(
            creator,
            title,
            description,
            voting_duration,
        );
        println!("🗳️ DAO Proposal #{} created: {}", proposal_id, title);
        proposal_id
    }

    pub fn vote_on_proposal(&mut self, voter: String, proposal_id: u64, vote: bool) -> bool {
        let success = self.dao_governance.vote(voter, proposal_id, vote);
        if success {
            println!("✅ Vote cast on proposal #{}", proposal_id);
        } else {
            println!("❌ Failed to vote on proposal #{}", proposal_id);
        }
        success
    }

    pub fn execute_proposal(&mut self, proposal_id: u64) -> bool {
        let success = self.dao_governance.execute_proposal(proposal_id);
        if success {
            println!("✅ Proposal #{} executed successfully", proposal_id);
        }
        success
    }

    pub fn get_dao_proposals(&self) -> Vec<Proposal> {
        self.dao_governance.get_proposals()
    }

    // 🆕 NEW: CROSS-CHAIN BRIDGE FUNCTIONS
    pub fn bridge_tokens(&mut self, from_chain: String, to_chain: String, sender: String, amount: u64) -> String {
        let bridge_tx = BridgeTransaction {
            from_chain,
            to_chain,
            sender,
            amount,
            timestamp: chrono::Utc::now().timestamp() as u64,
            status: "pending".to_string(),
        };
        
        let tx_hash = self.cross_chain_bridge.initiate_transfer(bridge_tx);
        println!("🌉 Cross-chain transfer initiated: {}", tx_hash);
        tx_hash
    }

    pub fn get_bridge_status(&self, tx_hash: &str) -> String {
        self.cross_chain_bridge.get_transfer_status(tx_hash)
    }

    pub fn get_pending_bridge_transactions(&self) -> Vec<BridgeTransaction> {
        self.cross_chain_bridge.get_pending_transactions()
    }

    // 🆕 NEW: SECURITY AUDIT FUNCTIONS
    pub fn run_security_audit(&self) -> AuditReport {
        println!("🔒 Running comprehensive security audit...");
        let report = self.security_audit.run_comprehensive_audit(&self.chain, &self.pending_transactions);
        println!("✅ Security audit completed: {}", report.overall_status);
        report
    }

    pub fn get_security_score(&self) -> u8 {
        self.security_audit.get_security_score()
    }

    // 🚀 NEW: ADVANCED FEATURES IMPLEMENTATION

    // AI Optimization Functions
    pub fn run_ai_optimization(&mut self) -> AIAnalysis {
        println!("🤖 Running AI optimization analysis...");
        let analysis = self.ai_optimization.analyze_blockchain(&self.chain, &self.pending_transactions);
        println!("✅ AI analysis completed: {}", analysis.summary);
        analysis
    }

    pub fn get_ai_recommendations(&self) -> Vec<String> {
        self.ai_optimization.get_recommendations()
    }

    // Quantum Resistance Functions
    pub fn enable_quantum_resistance(&mut self) -> bool {
        println!("⚛️ Enabling quantum resistance...");
        let success = self.quantum_resistance.enable_quantum_mode();
        if success {
            println!("✅ Quantum resistance enabled");
        }
        success
    }

    pub fn create_quantum_transaction(&self, sender: String, receiver: String, amount: u64) -> QuantumTransaction {
        self.quantum_resistance.create_quantum_transaction(sender, receiver, amount)
    }

    // Multi-signature Functions
    pub fn create_multisig_wallet(&mut self, owners: Vec<String>, required_signatures: u8) -> String {
        let address = self.multi_signature.create_multisig_wallet(owners, required_signatures);
        println!("🔐 Multi-signature wallet created: {}", address);
        address
    }

    pub fn submit_multisig_transaction(&mut self, multisig_address: String, transaction: MultiSigTransaction) -> String {
        let tx_hash = self.multi_signature.submit_transaction(multisig_address, transaction);
        println!("🔐 Multi-signature transaction submitted: {}", tx_hash);
        tx_hash
    }

    // Advanced DeFi Functions
    pub fn create_flash_loan(&mut self, borrower: String, amount: u64) -> Result<String, String> {
        let loan_id = self.defi_advanced.create_flash_loan(borrower, amount)?;
        println!("💰 Flash loan created: {}", loan_id);
        Ok(loan_id)
    }

    pub fn create_yield_farm(&mut self, creator: String, token_pair: (String, String), apr: f64) -> String {
        let farm_id = self.defi_advanced.create_yield_farm(creator, token_pair, apr);
        println!("🌾 Yield farm created: {}", farm_id);
        farm_id
    }

    // Gaming & Metaverse Functions
    pub fn mint_nft(&mut self, creator: String, metadata: String) -> String {
        let nft_id = self.gaming_metaverse.mint_nft(creator, metadata);
        println!("🎨 NFT minted: {}", nft_id);
        nft_id
    }

    pub fn purchase_virtual_land(&mut self, buyer: String, coordinates: (u64, u64)) -> bool {
        let success = self.gaming_metaverse.purchase_virtual_land(buyer, coordinates);
        if success {
            println!("🏞️ Virtual land purchased at ({}, {})", coordinates.0, coordinates.1);
        }
        success
    }

    // Enterprise Functions
    pub fn register_enterprise(&mut self, company_name: String, tax_id: String) -> String {
        let enterprise_id = self.enterprise_features.register_enterprise(company_name, tax_id);
        println!("🏢 Enterprise registered: {}", enterprise_id);
        enterprise_id
    }

    pub fn create_enterprise_transaction(&self, enterprise_id: String, to: String, amount: u64, purpose: String) -> EnterpriseTransaction {
        self.enterprise_features.create_transaction(enterprise_id, to, amount, purpose)
    }

    // Mass Adoption V2 Functions
    pub fn create_social_transaction(&mut self, from_phone: String, to_phone: String, amount: u64) -> String {
        let tx_hash = self.mass_adoption_v2.create_social_transaction(from_phone, to_phone, amount);
        println!("📱 Social transaction created: {}", tx_hash);
        tx_hash
    }

    pub fn complete_micro_task(&mut self, user: String, task_id: String) -> u64 {
        let reward = self.mass_adoption_v2.complete_micro_task(user, task_id);
        println!("✅ Micro-task completed: {} VEXA reward", reward);
        reward
    }

    // Cross-chain V2 Functions
    pub fn bridge_to_multiple_chains(&mut self, sender: String, amounts: HashMap<String, u64>) -> String {
        let tx_hash = self.cross_chain_v2.bridge_to_multiple_chains(sender, amounts);
        println!("🌉 Multi-chain bridge transaction: {}", tx_hash);
        tx_hash
    }

    pub fn get_cross_chain_balances(&self, address: &str) -> HashMap<String, u64> {
        self.cross_chain_v2.get_balances(address)
    }

    // 🔥 NEW: ENHANCED BLOCKCHAIN INFO
    pub fn get_enhanced_info(&self) -> String {
        format!(
            "Blockchain Info:\n\
            Blocks: {}\n\
            Pending TXs: {}\n\
            Forks: {}\n\
            Contracts: {}\n\
            Difficulty: {}\n\
            🔥 Burned: {} VEXA\n\
            🗳️ DAO Proposals: {}\n\
            🌉 Bridge TXs: {}\n\
            🔒 Security Score: {}/100\n\
            🤖 AI Optimizations: {}\n\
            ⚛️ Quantum Mode: {}\n\
            🔐 Multi-sig Wallets: {}\n\
            💰 DeFi TVL: {} VEXA\n\
            🎮 NFTs Minted: {}\n\
            🏢 Enterprises: {}\n\
            📱 Social Users: {}\n\
            🌉 Multi-chain TXs: {}",
            self.chain.len(),
            self.pending_transactions.len(),
            self.forks.len(),
            self.contract_manager.contracts.len(),
            self.difficulty,
            self.get_total_burned(),
            self.dao_governance.get_active_proposals_count(),
            self.cross_chain_bridge.get_pending_transactions_count(),
            self.get_security_score(),
            self.ai_optimization.get_optimization_count(),
            if self.quantum_resistance.is_enabled() { "ENABLED" } else { "DISABLED" },
            self.multi_signature.get_wallet_count(),
            self.defi_advanced.get_total_value_locked(),
            self.gaming_metaverse.get_nft_count(),
            self.enterprise_features.get_enterprise_count(),
            self.mass_adoption_v2.get_user_count(),
            self.cross_chain_v2.get_transaction_count()
        )
    }

    // 🏆 NEW: UPDATED COMPREHENSIVE BLOCKCHAIN STATUS
    pub fn get_comprehensive_status(&self) -> String {
        let circulating_supply = self.calculate_circulating_supply();
        let total_burned = self.get_total_burned();
        
        format!(
            "VexaChain Comprehensive Status:\n\
            📦 Total Blocks: {}\n\
            💰 Total Supply: {} VEXA\n\
            💸 Circulating Supply: {} VEXA\n\
            🔥 Total Burned: {} VEXA\n\
            🗳️ Active Proposals: {}\n\
            🌉 Pending Bridge TXs: {}\n\
            🤖 Deployed Contracts: {}\n\
            🔒 Security Level: {}/100\n\
            ⚡ Network Difficulty: {}\n\
            🤖 AI Optimizations: {}\n\
            ⚛️ Quantum Ready: {}\n\
            💰 DeFi Ecosystem: {} VEXA TVL\n\
            🎮 Metaverse: {} NFTs, {} Virtual Lands\n\
            🏢 Enterprise Solutions: {} Companies\n\
            🌍 Mass Adoption: {} Social Users\n\
            🌐 Multi-chain: {} Connected Chains\n\
            🏆 Tokenomics: Gate.io Ready ✅",
            self.chain.len(),
            Self::TOTAL_SUPPLY,
            circulating_supply,
            total_burned,
            self.dao_governance.get_active_proposals_count(),
            self.cross_chain_bridge.get_pending_transactions_count(),
            self.contract_manager.contracts.len(),
            self.get_security_score(),
            self.difficulty,
            self.ai_optimization.get_optimization_count(),
            if self.quantum_resistance.is_enabled() { "YES" } else { "NO" },
            self.defi_advanced.get_total_value_locked(),
            self.gaming_metaverse.get_nft_count(),
            self.gaming_metaverse.get_virtual_land_count(),
            self.enterprise_features.get_enterprise_count(),
            self.mass_adoption_v2.get_user_count(),
            self.cross_chain_v2.get_connected_chain_count()
        )
    }

    // 🏆 NEW: GET TOKENOMICS SUMMARY
    pub fn get_tokenomics_summary(&self) -> String {
        format!(
            "🏆 VEXA TOKENOMICS SUMMARY:\n\
            Total Supply: {} VEXA\n\
            Initial Circulating: {} VEXA\n\
            \n\
            🏢 Foundation & Team: {} VEXA (20%)\n\
            💰 Strategic Partners: {} VEXA (15%)\n\
            🌍 Public Distribution: {} VEXA (35%)\n\
            🔒 Liquidity & Growth: {} VEXA (30%)\n\
            \n\
            🔥 Total Burned: {} VEXA\n\
            💰 Current Circulating: {} VEXA\n\
            \n\
            🎯 Price Target: $1-2\n\
            📈 Market Cap Target: $100M-200M",
            Self::TOTAL_SUPPLY,
            Self::INITIAL_CIRCULATING,
            Self::FOUNDATION_TEAM,
            Self::STRATEGIC_PARTNERS,
            Self::PUBLIC_DISTRIBUTION,
            Self::LIQUIDITY_GROWTH,
            self.get_total_burned(),
            self.calculate_circulating_supply()
        )
    }
    
    // 🆕 FIX: UTXO System Reset Function
    pub fn reset_utxo_system(&mut self) {
        println!("🔄 COMPLETELY RESETTING UTXO SYSTEM...");
        
        // Clear everything
        self.utxo_set.clear();
        self.spent_utxos.clear();
        
        // Rebuild UTXO set from all transactions
        for (block_index, block) in self.chain.iter().enumerate() {
            println!("  Processing block {} with {} transactions", block_index, block.transactions.len());
            
            for (tx_index, tx) in block.transactions.iter().enumerate() {
                let tx_hash = self.calculate_transaction_hash(tx);
                
                // Create UTXO for receiver (transaction output)
                let receiver_utxo = UTXO {
                    tx_hash: tx_hash.clone(),
                    output_index: 0,
                    address: tx.receiver.clone(),
                    amount: tx.amount,
                    spent: false,
                };
                self.utxo_set.insert(tx_hash.clone(), receiver_utxo);
                
                // Mark sender's previous UTXO as spent (if not reward/genesis)
                if tx.sender != "MINING_REWARD" && tx.sender != "GENESIS" && !tx.input_utxo.is_empty() {
                    self.spent_utxos.insert(tx.input_utxo.clone());
                    println!("  Marked UTXO as spent: {} from {}", tx.input_utxo, tx.sender);
                }
            }
        }
        
        println!("✅ UTXO SYSTEM COMPLETELY RESET - {} UTXOs created", self.utxo_set.len());
    }
}

// BLOCK VALIDATION
impl Blockchain {
    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.previous_hash != previous.hash {
                return false;
            }

            if current.hash != self.calculate_hash(current) {
                return false;
            }

            if !self.is_valid_hash(&current.hash) {
                return false;
            }
        }
        true
    }
}