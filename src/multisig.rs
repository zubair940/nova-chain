// src/multisig.rs
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};

// Multi-signature Wallet Structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSigWallet {
    pub address: String,
    pub name: String,
    pub description: String,
    pub owners: Vec<WalletOwner>,
    pub required_signatures: u64, // M-of-N
    pub total_owners: u64,
    pub balance: u64,
    pub transaction_count: u64,
    pub pending_transactions: HashMap<String, MultiSigTransaction>,
    pub executed_transactions: VecDeque<ExecutedTransaction>, // Last 100 transactions
    pub created_at: u64,
    pub daily_limit: u64,
    pub daily_spent: u64,
    pub last_reset: u64,
    pub wallet_type: WalletType,
    pub security_level: SecurityLevel,
    pub recovery_mechanism: RecoveryMechanism,
}

// Wallet Owner with permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletOwner {
    pub address: String,
    pub name: String,
    pub weight: u64, // For weighted multisig
    pub permissions: Vec<Permission>,
    pub added_at: u64,
    pub is_active: bool,
}

// Multi-signature Transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSigTransaction {
    pub tx_id: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub asset: String,
    pub message: String,
    pub created_by: String,
    pub created_at: u64,
    pub expires_at: u64,
    pub signatures: Vec<TransactionSignature>,
    pub status: TransactionStatus,
    pub transaction_type: TransactionType,
    pub gas_limit: u64,
    pub gas_price: u64,
}

// Transaction Signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionSignature {
    pub signer: String,
    pub signature: String,
    pub signed_at: u64,
    pub signature_type: SignatureType,
}

// Executed Transaction Record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutedTransaction {
    pub tx_id: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub asset: String,
    pub executed_at: u64,
    pub executed_by: Vec<String>, // Signers who approved
    pub gas_used: u64,
    pub status: ExecutionStatus,
}

// Advanced Multi-signature with Time-locks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedMultiSig {
    pub base_wallet: MultiSigWallet,
    pub time_locks: Vec<TimeLock>,
    pub spending_limits: HashMap<String, u64>, // Category -> Limit
    pub whitelist_addresses: Vec<String>,
    pub blacklist_addresses: Vec<String>,
    pub transaction_rules: Vec<TransactionRule>,
    pub audit_trail: Vec<AuditEvent>,
}

// Time-lock for delayed transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeLock {
    pub tx_id: String,
    pub unlock_time: u64,
    pub executed: bool,
}

// Transaction Rules for automated governance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRule {
    pub rule_id: String,
    pub condition: RuleCondition,
    pub action: RuleAction,
    pub enabled: bool,
    pub created_at: u64,
}

// Audit Event for compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub event_id: String,
    pub event_type: AuditEventType,
    pub description: String,
    pub actor: String,
    pub timestamp: u64,
    pub details: HashMap<String, String>,
}

// Supporting Enums
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WalletType {
    Corporate,      // Business treasury
    DAO,           // Decentralized organization
    Family,        // Family wallet
    Escrow,        // Escrow service
    Personal,      // Personal multisig
    Investment,    // Investment fund
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecurityLevel {
    Low,           // 2-of-3
    Medium,        // 3-of-5
    High,          // 4-of-7
    Enterprise,    // 5-of-9 with time-locks
    Military,      // 7-of-12 with advanced features
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Permission {
    CanPropose,    // Can propose transactions
    CanSign,       // Can sign transactions
    CanAddOwner,   // Can add new owners
    CanRemoveOwner, // Can remove owners
    CanChangeThreshold, // Can change required signatures
    CanManageRules, // Can manage transaction rules
    CanRecover,    // Can initiate recovery
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Approved,
    Rejected,
    Expired,
    Executed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionType {
    Transfer,      // Simple transfer
    ContractCall,  // Smart contract interaction
    OwnerChange,   // Add/remove owner
    ThresholdChange, // Change required signatures
    Recovery,      // Wallet recovery
    RuleUpdate,    // Update transaction rules
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SignatureType {
    ECDSA,         // Standard ECDSA
    Schnorr,       // Schnorr signature
    BLS,           // BLS aggregate signature
    QuantumResistant, // Post-quantum signature
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExecutionStatus {
    Success,
    Failed,
    Reverted,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RecoveryMechanism {
    SocialRecovery { guardians: Vec<String>, required: u64 },
    TimeLockRecovery { unlock_time: u64 },
    MultiSigRecovery { recovery_wallet: String },
    BiometricRecovery { biometric_hash: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RuleCondition {
    AmountGreaterThan(u64),
    AmountLessThan(u64),
    DestinationInWhitelist,
    DestinationInBlacklist,
    TimeOfDay { start: u64, end: u64 },
    DayOfWeek(Vec<u8>), // 0-6 for Sunday-Saturday
    TransactionVolumeExceeded(u64),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RuleAction {
    AutoApprove,
    AutoReject,
    RequireExtraSignatures(u64),
    SendNotification(String), // Email/Phone
    TimeLock(u64), // Lock for X seconds
    EscalateTo(String), // Escalate to specific owner
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditEventType {
    TransactionProposed,
    TransactionSigned,
    TransactionExecuted,
    OwnerAdded,
    OwnerRemoved,
    ThresholdChanged,
    RuleCreated,
    RuleModified,
    RecoveryInitiated,
    SecurityBreach,
}

// Multi-signature Wallet Manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSigManager {
    pub wallets: HashMap<String, MultiSigWallet>,
    pub advanced_wallets: HashMap<String, AdvancedMultiSig>,
    pub wallet_creation_count: u64,
    pub total_assets_secured: u64,
    pub transaction_history: HashMap<String, Vec<ExecutedTransaction>>,
    pub security_events: Vec<SecurityEvent>,
}

// Security Event Monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub event_id: String,
    pub wallet_address: String,
    pub event_type: SecurityEventType,
    pub severity: SeverityLevel,
    pub description: String,
    pub timestamp: u64,
    pub resolved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecurityEventType {
    MultipleFailedAttempts,
    UnusualAmount,
    BlacklistedDestination,
    RuleViolation,
    RecoveryAttempt,
    OwnerChangeAttempt,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SeverityLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl MultiSigWallet {
    pub fn new(
        name: String,
        description: String,
        owners: Vec<String>,
        required_signatures: u64,
        wallet_type: WalletType,
    ) -> Self {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let wallet_owners: Vec<WalletOwner> = owners
            .into_iter()
            .map(|address| WalletOwner {
                address: address.clone(),
                name: format!("Owner {}", address.chars().take(8).collect::<String>()),
                weight: 1, // Default equal weight
                permissions: vec![
                    Permission::CanPropose,
                    Permission::CanSign,
                ],
                added_at: current_time,
                is_active: true,
            })
            .collect();

        let security_level = match required_signatures {
            2 => SecurityLevel::Low,
            3 => SecurityLevel::Medium,
            4 => SecurityLevel::High,
            5 => SecurityLevel::Enterprise,
            _ => SecurityLevel::Military,
        };

        let recovery_mechanism = match wallet_type {
            WalletType::Corporate => RecoveryMechanism::MultiSigRecovery {
                recovery_wallet: "corporate_recovery".to_string(),
            },
            WalletType::DAO => RecoveryMechanism::SocialRecovery {
                guardians: vec![],
                required: 0,
            },
            _ => RecoveryMechanism::TimeLockRecovery {
                unlock_time: current_time + 30 * 24 * 60 * 60, // 30 days
            },
        };

        Self {
            address: format!("multisig_{}_{}", name.to_lowercase().replace(" ", "_"), current_time),
            name,
            description,
            owners: wallet_owners,
            required_signatures,
            total_owners: wallet_owners.len() as u64,
            balance: 0,
            transaction_count: 0,
            pending_transactions: HashMap::new(),
            executed_transactions: VecDeque::with_capacity(100),
            created_at: current_time,
            daily_limit: 100_000_000, // 100M VEXA default
            daily_spent: 0,
            last_reset: current_time,
            wallet_type,
            security_level,
            recovery_mechanism,
        }
    }

    // Propose new transaction
    pub fn propose_transaction(
        &mut self,
        from: String,
        to: String,
        amount: u64,
        asset: String,
        message: String,
        proposer: String,
        transaction_type: TransactionType,
    ) -> Option<String> {
        // Check if proposer is an owner with permission
        if !self.is_owner_with_permission(&proposer, Permission::CanPropose) {
            return None;
        }

        // Check daily limit
        if self.daily_spent + amount > self.daily_limit {
            return None;
        }

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let tx_id = format!("tx_{}_{}", self.address, current_time);

        let transaction = MultiSigTransaction {
            tx_id: tx_id.clone(),
            from,
            to,
            amount,
            asset,
            message,
            created_by: proposer,
            created_at: current_time,
            expires_at: current_time + 24 * 60 * 60, // 24 hours expiry
            signatures: Vec::new(),
            status: TransactionStatus::Pending,
            transaction_type,
            gas_limit: 21000,
            gas_price: 10,
        };

        self.pending_transactions.insert(tx_id.clone(), transaction);
        Some(tx_id)
    }

    // Sign a pending transaction
    pub fn sign_transaction(
        &mut self,
        tx_id: String,
        signer: String,
        signature: String,
        signature_type: SignatureType,
    ) -> bool {
        // Check if signer is an owner with permission
        if !self.is_owner_with_permission(&signer, Permission::CanSign) {
            return false;
        }

        if let Some(transaction) = self.pending_transactions.get_mut(&tx_id) {
            // Check if already signed by this signer
            if transaction.signatures.iter().any(|s| s.signer == signer) {
                return false;
            }

            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let tx_signature = TransactionSignature {
                signer: signer.clone(),
                signature,
                signed_at: current_time,
                signature_type,
            };

            transaction.signatures.push(tx_signature);

            // Check if enough signatures collected
            if transaction.signatures.len() >= self.required_signatures as usize {
                transaction.status = TransactionStatus::Approved;
            }

            true
        } else {
            false
        }
    }

    // Execute approved transaction
    pub fn execute_transaction(&mut self, tx_id: String, executor: String) -> bool {
        if let Some(transaction) = self.pending_transactions.get(&tx_id) {
            if transaction.status == TransactionStatus::Approved {
                let current_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                // Update daily spent
                self.daily_spent += transaction.amount;
                self.balance -= transaction.amount;

                // Create executed transaction record
                let executed_tx = ExecutedTransaction {
                    tx_id: tx_id.clone(),
                    from: transaction.from.clone(),
                    to: transaction.to.clone(),
                    amount: transaction.amount,
                    asset: transaction.asset.clone(),
                    executed_at: current_time,
                    executed_by: transaction.signatures.iter().map(|s| s.signer.clone()).collect(),
                    gas_used: transaction.gas_limit,
                    status: ExecutionStatus::Success,
                };

                // Add to executed transactions (keep only last 100)
                if self.executed_transactions.len() >= 100 {
                    self.executed_transactions.pop_front();
                }
                self.executed_transactions.push_back(executed_tx);

                self.transaction_count += 1;
                self.pending_transactions.remove(&tx_id);

                println!("✅ Multi-sig transaction executed: {} VEXA to {}", transaction.amount, transaction.to);
                return true;
            }
        }
        false
    }

    // Add new owner to wallet
    pub fn add_owner(&mut self, new_owner: String, proposer: String) -> bool {
        if !self.is_owner_with_permission(&proposer, Permission::CanAddOwner) {
            return false;
        }

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let owner = WalletOwner {
            address: new_owner.clone(),
            name: format!("Owner {}", new_owner.chars().take(8).collect::<String>()),
            weight: 1,
            permissions: vec![Permission::CanPropose, Permission::CanSign],
            added_at: current_time,
            is_active: true,
        };

        self.owners.push(owner);
        self.total_owners += 1;

        true
    }

    // Remove owner from wallet
    pub fn remove_owner(&mut self, owner_address: String, proposer: String) -> bool {
        if !self.is_owner_with_permission(&proposer, Permission::CanRemoveOwner) {
            return false;
        }

        // Cannot remove if it would make required_signatures impossible
        if self.owners.len() <= self.required_signatures as usize {
            return false;
        }

        self.owners.retain(|owner| owner.address != owner_address);
        self.total_owners -= 1;

        true
    }

    // Change required signatures threshold
    pub fn change_threshold(&mut self, new_threshold: u64, proposer: String) -> bool {
        if !self.is_owner_with_permission(&proposer, Permission::CanChangeThreshold) {
            return false;
        }

        // New threshold must be <= total owners and >= 1
        if new_threshold > self.total_owners || new_threshold < 1 {
            return false;
        }

        self.required_signatures = new_threshold;
        true
    }

    // Check if address is owner with specific permission
    fn is_owner_with_permission(&self, address: &str, permission: Permission) -> bool {
        self.owners
            .iter()
            .any(|owner| owner.address == address && owner.is_active && owner.permissions.contains(&permission))
    }

    // Reset daily spending (called automatically at midnight)
    pub fn reset_daily_spending(&mut self) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Reset if 24 hours have passed
        if current_time - self.last_reset >= 24 * 60 * 60 {
            self.daily_spent = 0;
            self.last_reset = current_time;
        }
    }

    // Get wallet info
    pub fn get_wallet_info(&self) -> String {
        format!(
            "Multi-sig Wallet: {}\n\
            Address: {}\n\
            Type: {:?}\n\
            Security: {:?}\n\
            Balance: {} VEXA\n\
            Owners: {}/{} signed\n\
            Required: {}-of-{}\n\
            Daily Limit: {} VEXA\n\
            Daily Spent: {} VEXA",
            self.name,
            self.address,
            self.wallet_type,
            self.security_level,
            self.balance,
            self.owners.iter().filter(|o| o.is_active).count(),
            self.total_owners,
            self.required_signatures,
            self.total_owners,
            self.daily_limit,
            self.daily_spent
        )
    }
}

impl MultiSigManager {
    pub fn new() -> Self {
        Self {
            wallets: HashMap::new(),
            advanced_wallets: HashMap::new(),
            wallet_creation_count: 0,
            total_assets_secured: 0,
            transaction_history: HashMap::new(),
            security_events: Vec::new(),
        }
    }

    // Create new multi-signature wallet
    pub fn create_wallet(
        &mut self,
        name: String,
        description: String,
        owners: Vec<String>,
        required_signatures: u64,
        wallet_type: WalletType,
    ) -> String {
        let wallet = MultiSigWallet::new(name, description, owners, required_signatures, wallet_type);
        let address = wallet.address.clone();
        
        self.wallets.insert(address.clone(), wallet);
        self.wallet_creation_count += 1;
        
        address
    }

    // Fund wallet
    pub fn fund_wallet(&mut self, wallet_address: String, amount: u64) -> bool {
        if let Some(wallet) = self.wallets.get_mut(&wallet_address) {
            wallet.balance += amount;
            self.total_assets_secured += amount;
            true
        } else {
            false
        }
    }

    // Propose transaction in wallet
    pub fn propose_transaction(
        &mut self,
        wallet_address: String,
        to: String,
        amount: u64,
        asset: String,
        message: String,
        proposer: String,
        transaction_type: TransactionType,
    ) -> Option<String> {
        if let Some(wallet) = self.wallets.get_mut(&wallet_address) {
            wallet.propose_transaction(
                wallet_address.clone(),
                to,
                amount,
                asset,
                message,
                proposer,
                transaction_type,
            )
        } else {
            None
        }
    }

    // Sign transaction
    pub fn sign_transaction(
        &mut self,
        wallet_address: String,
        tx_id: String,
        signer: String,
        signature: String,
        signature_type: SignatureType,
    ) -> bool {
        if let Some(wallet) = self.wallets.get_mut(&wallet_address) {
            wallet.sign_transaction(tx_id, signer, signature, signature_type)
        } else {
            false
        }
    }

    // Execute transaction
    pub fn execute_transaction(&mut self, wallet_address: String, tx_id: String, executor: String) -> bool {
        if let Some(wallet) = self.wallets.get_mut(&wallet_address) {
            let result = wallet.execute_transaction(tx_id.clone(), executor.clone());
            
            if result {
                // Record in transaction history
                if let Some(executed_tx) = wallet.executed_transactions.back() {
                    self.transaction_history
                        .entry(wallet_address)
                        .or_insert_with(Vec::new)
                        .push(executed_tx.clone());
                }
            }
            
            result
        } else {
            false
        }
    }

    // Get wallet balance
    pub fn get_wallet_balance(&self, wallet_address: &str) -> Option<u64> {
        self.wallets.get(wallet_address).map(|wallet| wallet.balance)
    }

    // Get pending transactions
    pub fn get_pending_transactions(&self, wallet_address: &str) -> Vec<MultiSigTransaction> {
        if let Some(wallet) = self.wallets.get(wallet_address) {
            wallet.pending_transactions.values().cloned().collect()
        } else {
            Vec::new()
        }
    }

    // Get system statistics
    pub fn get_system_stats(&self) -> String {
        let total_wallets = self.wallets.len();
        let total_pending_tx: usize = self.wallets.values().map(|w| w.pending_transactions.len()).sum();
        let total_executed_tx: usize = self.wallets.values().map(|w| w.executed_transactions.len()).sum();

        format!(
            "Multi-signature System Stats:\n\
            Total Wallets: {}\n\
            Total Assets Secured: {} VEXA\n\
            Pending Transactions: {}\n\
            Executed Transactions: {}\n\
            Security Events: {}",
            total_wallets,
            self.total_assets_secured,
            total_pending_tx,
            total_executed_tx,
            self.security_events.len()
        )
    }

    // Add security event
    pub fn add_security_event(
        &mut self,
        wallet_address: String,
        event_type: SecurityEventType,
        severity: SeverityLevel,
        description: String,
    ) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let event = SecurityEvent {
            event_id: format!("security_{}_{}", wallet_address, current_time),
            wallet_address,
            event_type,
            severity,
            description,
            timestamp: current_time,
            resolved: false,
        };

        self.security_events.push(event);
    }
}

// Default implementation
impl Default for MultiSigManager {
    fn default() -> Self {
        Self::new()
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_creation() {
        let owners = vec![
            "owner1".to_string(),
            "owner2".to_string(),
            "owner3".to_string(),
        ];
        
        let wallet = MultiSigWallet::new(
            "Test Wallet".to_string(),
            "Test Description".to_string(),
            owners,
            2,
            WalletType::Corporate,
        );

        assert_eq!(wallet.required_signatures, 2);
        assert_eq!(wallet.total_owners, 3);
        assert_eq!(wallet.security_level, SecurityLevel::Low);
    }

    #[test]
    fn test_transaction_proposal() {
        let mut wallet = MultiSigWallet::new(
            "Test Wallet".to_string(),
            "Test".to_string(),
            vec!["owner1".to_string()],
            1,
            WalletType::Personal,
        );

        let tx_id = wallet.propose_transaction(
            "wallet1".to_string(),
            "recipient".to_string(),
            1000,
            "VEXA".to_string(),
            "Test payment".to_string(),
            "owner1".to_string(),
            TransactionType::Transfer,
        );

        assert!(tx_id.is_some());
        assert_eq!(wallet.pending_transactions.len(), 1);
    }

    #[test]
    fn test_transaction_execution() {
        let mut manager = MultiSigManager::new();
        
        let wallet_address = manager.create_wallet(
            "Test".to_string(),
            "Test".to_string(),
            vec!["owner1".to_string()],
            1,
            WalletType::Personal,
        );

        manager.fund_wallet(wallet_address.clone(), 5000);

        let tx_id = manager.propose_transaction(
            wallet_address.clone(),
            "recipient".to_string(),
            1000,
            "VEXA".to_string(),
            "Test".to_string(),
            "owner1".to_string(),
            TransactionType::Transfer,
        ).unwrap();

        assert!(manager.execute_transaction(wallet_address, tx_id, "owner1".to_string()));
    }
}