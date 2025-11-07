// src/layer2_scaling.rs
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};

// ==================== LAYER 2 PROTOCOLS ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Layer2Protocol {
    ZkRollup,           // Zero-Knowledge Rollups
    OptimisticRollup,   // Optimistic Rollups
    StateChannels,      // Payment Channels
    Sidechains,         // Application-specific chains
    Plasma,             // Plasma chains
    Validium,           // Validium (data off-chain)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer2Network {
    pub network_id: String,
    pub protocol: Layer2Protocol,
    pub name: String,
    pub description: String,
    pub operator: String,
    pub created_at: u64,
    pub total_value_locked: u64,    // TVL in VEXA
    pub transaction_count: u64,
    pub active_users: u32,
    pub fee_structure: FeeStructure,
    pub security_parameters: SecurityParams,
    pub status: NetworkStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeStructure {
    pub deposit_fee: u64,           // Fee to move to L2
    pub withdrawal_fee: u64,        // Fee to move to L1
    pub transaction_fee: u64,       // Per transaction fee
    pub smart_contract_fee: u64,    // Contract execution fee
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityParams {
    pub challenge_period: u64,      // For optimistic rollups (in seconds)
    pub withdrawal_delay: u64,      // Time to withdraw (in blocks)
    pub data_availability: DataAvailability,
    pub fraud_proofs: bool,         // Fraud proof mechanism
    pub escape_hatch: bool,         // Emergency withdrawal
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataAvailability {
    OnChain,        // All data on L1
    OffChain,       // Data off-chain with commitments
    Mixed,          // Some data on-chain
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkStatus {
    Active,
    Paused,
    Upgrading,
    Emergency,
}

// ==================== ZK-ROLLUP IMPLEMENTATION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkRollupBatch {
    pub batch_id: String,
    pub previous_batch_hash: String,
    pub transactions: Vec<L2Transaction>,
    pub state_root: String,
    pub exit_root: String,
    pub zk_proof: String,           // Zero-knowledge proof
    pub public_inputs: Vec<String>, // Public inputs for verification
    pub created_at: u64,
    pub block_number: u64,
    pub operator: String,
    pub status: BatchStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProof {
    pub proof_id: String,
    pub batch_id: String,
    pub proof_data: String,         // Actual zk-SNARK proof
    public_inputs: Vec<String>,
    verification_key: String,
    created_at: u64,
    verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BatchStatus {
    Pending,
    Committed,
    Verified,
    Reverted,
}

// ==================== OPTIMISTIC ROLLUP IMPLEMENTATION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimisticBatch {
    pub batch_id: String,
    pub previous_batch_hash: String,
    pub transactions: Vec<L2Transaction>,
    pub state_root: String,
    pub created_at: u64,
    pub block_number: u64,
    pub operator: String,
    pub status: OptimisticBatchStatus,
    pub challenge_period_end: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimisticBatchStatus {
    Pending,
    Committed,
    Challenged,
    Finalized,
    Reverted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudProof {
    pub proof_id: String,
    pub batch_id: String,
    pub challenger: String,
    pub invalid_state_transition: String,
    pub evidence: String,
    pub created_at: u64,
    pub resolved: bool,
    pub valid: bool,
}

// ==================== STATE CHANNELS ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateChannel {
    pub channel_id: String,
    pub participants: Vec<String>,
    pub balances: HashMap<String, u64>,  // Participant balances
    pub total_deposit: u64,
    pub state_nonce: u64,                // State version
    pub current_state: ChannelState,
    pub created_at: u64,
    pub status: ChannelStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelState {
    pub state_hash: String,
    pub balances: HashMap<String, u64>,
    pub nonce: u64,
    pub signatures: Vec<String>,         // Participant signatures
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelStatus {
    Open,
    Updating,
    Closing,
    Closed,
    Disputed,
}

// ==================== L2 TRANSACTIONS ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L2Transaction {
    pub tx_hash: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub transaction_type: L2TransactionType,
    pub nonce: u64,
    pub signature: String,
    pub created_at: u64,
    pub batch_id: Option<String>,        // Which batch includes this
    pub status: L2TransactionStatus,
    pub gas_limit: u64,
    pub gas_used: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum L2TransactionType {
    Transfer,
    ContractDeployment,
    ContractCall,
    Deposit,
    Withdrawal,
    ForceWithdrawal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum L2TransactionStatus {
    Pending,
    Included,
    Finalized,
    Failed,
}

// ==================== BRIDGE BETWEEN L1-L2 ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bridge {
    pub bridge_id: String,
    pub l2_network_id: String,
    pub total_deposits: u64,
    pub total_withdrawals: u64,
    pub security_deposit: u64,      // Operator security deposit
    pub last_heartbeat: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deposit {
    pub deposit_id: String,
    pub user: String,
    pub amount: u64,
    pub l2_network_id: String,
    pub l1_transaction_hash: String,
    pub created_at: u64,
    pub processed: bool,
    pub l2_transaction_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Withdrawal {
    pub withdrawal_id: String,
    pub user: String,
    pub amount: u64,
    pub l2_network_id: String,
    pub l2_transaction_hash: String,
    pub created_at: u64,
    pub status: WithdrawalStatus,
    pub l1_transaction_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WithdrawalStatus {
    Pending,
    Challenged,
    Finalized,
    Completed,
}

// ==================== LAYER2 MANAGER ====================

#[derive(Debug, Clone)]
pub struct Layer2Manager {
    pub networks: HashMap<String, Layer2Network>,
    pub zk_batches: HashMap<String, ZkRollupBatch>,
    pub optimistic_batches: HashMap<String, OptimisticBatch>,
    pub state_channels: HashMap<String, StateChannel>,
    pub bridges: HashMap<String, Bridge>,
    pub deposits: HashMap<String, Deposit>,
    pub withdrawals: HashMap<String, Withdrawal>,
    pub fraud_proofs: HashMap<String, FraudProof>,
    pub transaction_pool: VecDeque<L2Transaction>,
}

impl Layer2Manager {
    pub fn new() -> Self {
        Self {
            networks: HashMap::new(),
            zk_batches: HashMap::new(),
            optimistic_batches: HashMap::new(),
            state_channels: HashMap::new(),
            bridges: HashMap::new(),
            deposits: HashMap::new(),
            withdrawals: HashMap::new(),
            fraud_proofs: HashMap::new(),
            transaction_pool: VecDeque::new(),
        }
    }

    // ==================== NETWORK MANAGEMENT ====================

    pub fn create_network(&mut self, protocol: Layer2Protocol, name: String, description: String, operator: String, fee_structure: FeeStructure, security_params: SecurityParams) -> Result<String, String> {
        let network_id = format!("l2_{}_{}", protocol_name(&protocol), Self::generate_id());
        
        let network = Layer2Network {
            network_id: network_id.clone(),
            protocol,
            name,
            description,
            operator,
            created_at: Self::current_timestamp(),
            total_value_locked: 0,
            transaction_count: 0,
            active_users: 0,
            fee_structure,
            security_parameters: security_params,
            status: NetworkStatus::Active,
        };

        self.networks.insert(network_id.clone(), network);

        // Create bridge for this network
        let bridge_id = format!("bridge_{}", network_id);
        let bridge = Bridge {
            bridge_id: bridge_id.clone(),
            l2_network_id: network_id.clone(),
            total_deposits: 0,
            total_withdrawals: 0,
            security_deposit: 1000000000, // 1000 VEXA security deposit
            last_heartbeat: Self::current_timestamp(),
        };
        self.bridges.insert(bridge_id, bridge);

        Ok(network_id)
    }

    // ==================== DEPOSIT TO L2 ====================

    pub fn deposit_to_l2(&mut self, l2_network_id: &str, user: String, amount: u64, l1_tx_hash: String) -> Result<String, String> {
        let network = self.networks.get_mut(l2_network_id).ok_or("L2 network not found")?;
        
        if amount < network.fee_structure.deposit_fee {
            return Err("Amount too small for deposit fee".to_string());
        }

        let deposit_id = format!("deposit_{}", Self::generate_id());
        let deposit = Deposit {
            deposit_id: deposit_id.clone(),
            user: user.clone(),
            amount,
            l2_network_id: l2_network_id.to_string(),
            l1_transaction_hash: l1_tx_hash,
            created_at: Self::current_timestamp(),
            processed: false,
            l2_transaction_hash: None,
        };

        // Update network statistics
        network.total_value_locked += amount;
        network.active_users += 1;

        // Update bridge
        if let Some(bridge) = self.bridges.get_mut(l2_network_id) {
            bridge.total_deposits += amount;
            bridge.last_heartbeat = Self::current_timestamp();
        }

        self.deposits.insert(deposit_id.clone(), deposit);
        Ok(deposit_id)
    }

    // ==================== ZK-ROLLUP BATCH PROCESSING ====================

    pub fn create_zk_batch(&mut self, l2_network_id: &str, transactions: Vec<L2Transaction>, operator: String) -> Result<String, String> {
        let network = self.networks.get(l2_network_id).ok_or("L2 network not found")?;
        
        if network.protocol != Layer2Protocol::ZkRollup {
            return Err("Network is not a ZK-Rollup".to_string());
        }

        let batch_id = format!("zk_batch_{}", Self::generate_id());
        
        // Calculate state root and exit root (simplified)
        let state_root = Self::calculate_state_root(&transactions);
        let exit_root = Self::calculate_exit_root(&transactions);

        let batch = ZkRollupBatch {
            batch_id: batch_id.clone(),
            previous_batch_hash: self.get_last_batch_hash(l2_network_id),
            transactions,
            state_root,
            exit_root,
            zk_proof: "simulated_zk_proof".to_string(), // In real implementation, generate actual proof
            public_inputs: vec![],
            created_at: Self::current_timestamp(),
            block_number: 0, // Would be L1 block number
            operator,
            status: BatchStatus::Pending,
        };

        self.zk_batches.insert(batch_id.clone(), batch);
        Ok(batch_id)
    }

    pub fn commit_zk_batch(&mut self, batch_id: &str, zk_proof: ZkProof) -> Result<(), String> {
        let batch = self.zk_batches.get_mut(batch_id).ok_or("Batch not found")?;
        
        batch.zk_proof = zk_proof.proof_data;
        batch.public_inputs = zk_proof.public_inputs;
        batch.status = BatchStatus::Committed;

        // Update network statistics
        if let Some(network) = self.networks.get_mut(&batch.operator) {
            network.transaction_count += batch.transactions.len() as u64;
        }

        Ok(())
    }

    // ==================== OPTIMISTIC ROLLUP BATCH PROCESSING ====================

    pub fn create_optimistic_batch(&mut self, l2_network_id: &str, transactions: Vec<L2Transaction>, operator: String) -> Result<String, String> {
        let network = self.networks.get(l2_network_id).ok_or("L2 network not found")?;
        
        if network.protocol != Layer2Protocol::OptimisticRollup {
            return Err("Network is not an Optimistic Rollup".to_string());
        }

        let batch_id = format!("opt_batch_{}", Self::generate_id());
        
        let batch = OptimisticBatch {
            batch_id: batch_id.clone(),
            previous_batch_hash: self.get_last_batch_hash(l2_network_id),
            transactions,
            state_root: Self::calculate_state_root(&vec![]), // Simplified
            created_at: Self::current_timestamp(),
            block_number: 0,
            operator: operator.clone(),
            status: OptimisticBatchStatus::Committed,
            challenge_period_end: Self::current_timestamp() + network.security_parameters.challenge_period,
        };

        self.optimistic_batches.insert(batch_id.clone(), batch);

        // Update network
        if let Some(network) = self.networks.get_mut(l2_network_id) {
            network.transaction_count += transactions.len() as u64;
        }

        Ok(batch_id)
    }

    pub fn challenge_optimistic_batch(&mut self, batch_id: &str, challenger: String, evidence: String) -> Result<String, String> {
        let batch = self.optimistic_batches.get_mut(batch_id).ok_or("Batch not found")?;
        
        if batch.status != OptimisticBatchStatus::Committed {
            return Err("Batch cannot be challenged in current state".to_string());
        }

        if Self::current_timestamp() > batch.challenge_period_end {
            return Err("Challenge period expired".to_string());
        }

        batch.status = OptimisticBatchStatus::Challenged;

        let proof_id = format!("fraud_proof_{}", Self::generate_id());
        let fraud_proof = FraudProof {
            proof_id: proof_id.clone(),
            batch_id: batch_id.to_string(),
            challenger,
            invalid_state_transition: "State transition invalid".to_string(),
            evidence,
            created_at: Self::current_timestamp(),
            resolved: false,
            valid: false, // Initially assumed invalid until verified
        };

        self.fraud_proofs.insert(proof_id.clone(), fraud_proof);
        Ok(proof_id)
    }

    // ==================== STATE CHANNELS ====================

    pub fn open_state_channel(&mut self, participants: Vec<String>, deposits: HashMap<String, u64>) -> Result<String, String> {
        if participants.len() < 2 {
            return Err("At least 2 participants required".to_string());
        }

        let channel_id = format!("channel_{}", Self::generate_id());
        let total_deposit: u64 = deposits.values().sum();

        let channel = StateChannel {
            channel_id: channel_id.clone(),
            participants: participants.clone(),
            balances: deposits,
            total_deposit,
            state_nonce: 0,
            current_state: ChannelState {
                state_hash: "initial_state".to_string(),
                balances: deposits,
                nonce: 0,
                signatures: Vec::new(),
            },
            created_at: Self::current_timestamp(),
            status: ChannelStatus::Open,
        };

        self.state_channels.insert(channel_id.clone(), channel);
        Ok(channel_id)
    }

    pub fn update_channel_state(&mut self, channel_id: &str, new_balances: HashMap<String, u64>, nonce: u64, signatures: Vec<String>) -> Result<(), String> {
        let channel = self.state_channels.get_mut(channel_id).ok_or("Channel not found")?;
        
        if nonce <= channel.state_nonce {
            return Err("Nonce must be higher than current".to_string());
        }

        // Verify signatures (simplified)
        if signatures.len() != channel.participants.len() {
            return Err("All participants must sign".to_string());
        }

        channel.current_state = ChannelState {
            state_hash: Self::calculate_state_hash(&new_balances, nonce),
            balances: new_balances,
            nonce,
            signatures,
        };
        channel.state_nonce = nonce;

        Ok(())
    }

    pub fn close_channel(&mut self, channel_id: &str, final_state: ChannelState) -> Result<(), String> {
        let channel = self.state_channels.get_mut(channel_id).ok_or("Channel not found")?;
        
        // Verify final state is properly signed
        if final_state.signatures.len() != channel.participants.len() {
            return Err("All participants must sign final state".to_string());
        }

        channel.current_state = final_state;
        channel.status = ChannelStatus::Closing;

        // In real implementation, this would trigger L1 settlement
        Ok(())
    }

    // ==================== WITHDRAWAL FROM L2 ====================

    pub fn initiate_withdrawal(&mut self, l2_network_id: &str, user: String, amount: u64, l2_tx_hash: String) -> Result<String, String> {
        let network = self.networks.get(l2_network_id).ok_or("L2 network not found")?;
        
        let withdrawal_id = format!("withdrawal_{}", Self::generate_id());
        let withdrawal = Withdrawal {
            withdrawal_id: withdrawal_id.clone(),
            user: user.clone(),
            amount,
            l2_network_id: l2_network_id.to_string(),
            l2_transaction_hash: l2_tx_hash,
            created_at: Self::current_timestamp(),
            status: WithdrawalStatus::Pending,
            l1_transaction_hash: None,
        };

        // Update bridge
        if let Some(bridge) = self.bridges.get_mut(l2_network_id) {
            bridge.total_withdrawals += amount;
            bridge.last_heartbeat = Self::current_timestamp();
        }

        self.withdrawals.insert(withdrawal_id.clone(), withdrawal);
        Ok(withdrawal_id)
    }

    pub fn finalize_withdrawal(&mut self, withdrawal_id: &str, l1_tx_hash: String) -> Result<(), String> {
        let withdrawal = self.withdrawals.get_mut(withdrawal_id).ok_or("Withdrawal not found")?;
        
        withdrawal.status = WithdrawalStatus::Completed;
        withdrawal.l1_transaction_hash = Some(l1_tx_hash);

        // Update network TVL
        if let Some(network) = self.networks.get_mut(&withdrawal.l2_network_id) {
            network.total_value_locked = network.total_value_locked.saturating_sub(withdrawal.amount);
        }

        Ok(())
    }

    // ==================== PERFORMANCE METRICS ====================

    pub fn get_network_stats(&self, l2_network_id: &str) -> Option<Layer2Network> {
        self.networks.get(l2_network_id).cloned()
    }

    pub fn get_all_networks_stats(&self) -> Vec<Layer2Network> {
        self.networks.values().cloned().collect()
    }

    pub fn calculate_tps(&self, l2_network_id: &str) -> f64 {
        if let Some(network) = self.networks.get(l2_network_id) {
            let runtime = Self::current_timestamp() - network.created_at;
            if runtime > 0 {
                network.transaction_count as f64 / runtime as f64
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    pub fn calculate_cost_savings(&self, l2_network_id: &str) -> f64 {
        // Compare L2 fees vs estimated L1 fees
        if let Some(network) = self.networks.get(l2_network_id) {
            let l1_fee_estimate = 1000000; // 0.001 VEXA per tx on L1
            let l2_fee = network.fee_structure.transaction_fee;
            
            if l1_fee_estimate > l2_fee {
                ((l1_fee_estimate - l2_fee) as f64 / l1_fee_estimate as f64) * 100.0
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    // ==================== UTILITY FUNCTIONS ====================

    fn get_last_batch_hash(&self, l2_network_id: &str) -> String {
        // In real implementation, get actual last batch hash
        "last_batch_hash".to_string()
    }

    fn calculate_state_root(transactions: &[L2Transaction]) -> String {
        let mut hasher = Sha256::new();
        for tx in transactions {
            hasher.update(tx.tx_hash.as_bytes());
        }
        format!("{:x}", hasher.finalize())
    }

    fn calculate_exit_root(transactions: &[L2Transaction]) -> String {
        let mut hasher = Sha256::new();
        for tx in transactions {
            if let L2TransactionType::Withdrawal = tx.transaction_type {
                hasher.update(tx.tx_hash.as_bytes());
            }
        }
        format!("{:x}", hasher.finalize())
    }

    fn calculate_state_hash(balances: &HashMap<String, u64>, nonce: u64) -> String {
        let mut hasher = Sha256::new();
        for (address, balance) in balances {
            hasher.update(address.as_bytes());
            hasher.update(&balance.to_be_bytes());
        }
        hasher.update(&nonce.to_be_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn generate_id() -> String {
        format!("{}", rand::random::<u64>())
    }

    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

// ==================== L2 TRANSACTION PROCESSING ====================

impl Layer2Manager {
    pub fn submit_l2_transaction(&mut self, l2_network_id: &str, from: String, to: String, amount: u64, transaction_type: L2TransactionType, signature: String) -> Result<String, String> {
        let network = self.networks.get(l2_network_id).ok_or("L2 network not found")?;
        
        let tx_hash = format!("l2_tx_{}", Self::generate_id());
        let transaction = L2Transaction {
            tx_hash: tx_hash.clone(),
            from,
            to,
            amount,
            transaction_type,
            nonce: 0, // Would be from account nonce
            signature,
            created_at: Self::current_timestamp(),
            batch_id: None,
            status: L2TransactionStatus::Pending,
            gas_limit: 100000,
            gas_used: 0,
        };

        self.transaction_pool.push_back(transaction);
        Ok(tx_hash)
    }

    pub fn process_transaction_pool(&mut self, l2_network_id: &str, operator: String) -> Result<String, String> {
        let transactions: Vec<L2Transaction> = self.transaction_pool.drain(..).collect();
        
        if transactions.is_empty() {
            return Err("No transactions to process".to_string());
        }

        let network = self.networks.get(l2_network_id).ok_or("L2 network not found")?;
        
        match network.protocol {
            Layer2Protocol::ZkRollup => self.create_zk_batch(l2_network_id, transactions, operator),
            Layer2Protocol::OptimisticRollup => self.create_optimistic_batch(l2_network_id, transactions, operator),
            _ => Err("Protocol not supported for batch processing".to_string()),
        }
    }
}

// ==================== EMERGENCY & SECURITY ====================

impl Layer2Manager {
    pub fn emergency_withdraw(&mut self, l2_network_id: &str, user: String) -> Result<String, String> {
        // Force withdrawal without waiting for challenge period
        let withdrawal_id = format!("emergency_withdraw_{}", Self::generate_id());
        
        // In real implementation, this would require proof of L2 state
        let withdrawal = Withdrawal {
            withdrawal_id: withdrawal_id.clone(),
            user,
            amount: 0, // Actual amount would be determined from L2 state
            l2_network_id: l2_network_id.to_string(),
            l2_transaction_hash: "emergency".to_string(),
            created_at: Self::current_timestamp(),
            status: WithdrawalStatus::Pending,
            l1_transaction_hash: None,
        };

        self.withdrawals.insert(withdrawal_id.clone(), withdrawal);
        Ok(withdrawal_id)
    }

    pub fn pause_network(&mut self, l2_network_id: &str) -> Result<(), String> {
        let network = self.networks.get_mut(l2_network_id).ok_or("L2 network not found")?;
        network.status = NetworkStatus::Paused;
        Ok(())
    }

    pub fn resume_network(&mut self, l2_network_id: &str) -> Result<(), String> {
        let network = self.networks.get_mut(l2_network_id).ok_or("L2 network not found")?;
        network.status = NetworkStatus::Active;
        Ok(())
    }
}

// ==================== HELPER FUNCTIONS ====================

fn protocol_name(protocol: &Layer2Protocol) -> String {
    match protocol {
        Layer2Protocol::ZkRollup => "zkrollup",
        Layer2Protocol::OptimisticRollup => "optimistic",
        Layer2Protocol::StateChannels => "statechannel",
        Layer2Protocol::Sidechains => "sidechain",
        Layer2Protocol::Plasma => "plasma",
        Layer2Protocol::Validium => "validium",
    }.to_string()
}

// ==================== L2 SMART CONTRACT SUPPORT ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L2SmartContract {
    pub contract_id: String,
    pub address: String,
    pub code_hash: String,
    pub storage_root: String,
    pub creator: String,
    pub created_at: u64,
    pub l2_network_id: String,
}

impl Layer2Manager {
    pub fn deploy_l2_contract(&mut self, l2_network_id: &str, code: String, creator: String) -> Result<String, String> {
        let contract_id = format!("l2_contract_{}", Self::generate_id());
        let address = format!("0x{}", &contract_id[..40]);
        
        let contract = L2SmartContract {
            contract_id: contract_id.clone(),
            address,
            code_hash: Self::calculate_hash(&code),
            storage_root: "initial_storage".to_string(),
            creator,
            created_at: Self::current_timestamp(),
            l2_network_id: l2_network_id.to_string(),
        };

        Ok(contract_id)
    }

    fn calculate_hash(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}