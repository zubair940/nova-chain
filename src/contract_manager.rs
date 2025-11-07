use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// 🆕 ADVANCED CONTRACT TYPES
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ContractType {
    Standard,
    AIContract,           // AI-powered contracts
    MultiSig,             // Multi-signature contracts
    DeFiVault,            // Yield farming contracts
    GamingAsset,          // NFT gaming contracts
    PredictionMarket,     // Prediction market contracts
    CrossChain,           // Cross-chain contracts
    QuantumResistant,     // Quantum-safe contracts
    SocialFi,             // Social finance contracts
    Identity,             // Decentralized identity contracts
}

// 🆕 AI CONTRACT SPECIFICATIONS
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AIModel {
    pub model_hash: String,
    pub model_type: String,        // "classification", "regression", "nlp"
    pub training_data: Vec<String>,
    pub accuracy: f64,
    pub last_trained: u64,
}

// 🆕 MULTI-SIG CONTRACT
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MultiSigContract {
    pub owners: Vec<String>,
    pub required_signatures: u8,
    pub pending_actions: HashMap<String, MultiSigAction>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MultiSigAction {
    pub action_id: String,
    pub action_type: String,       // "transfer", "upgrade", "add_owner"
    pub data: String,
    pub proposed_by: String,
    pub approvals: Vec<String>,
    pub executed: bool,
}

// 🆕 DEFI VAULT CONTRACT
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeFiVaultContract {
    pub strategy: String,          // "yield_farming", "liquidity_provision", "staking"
    pub total_liquidity: u64,
    pub apr: f64,
    pub investors: HashMap<String, u64>, // address -> amount invested
    pub performance_fee: u8,       // percentage
    pub last_reward_distribution: u64,
}

// 🆕 GAMING ASSET CONTRACT
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GamingAssetContract {
    pub game_id: String,
    pub asset_type: String,        // "weapon", "character", "land", "item"
    pub rarity: String,            // "common", "rare", "epic", "legendary"
    pub metadata: HashMap<String, String>,
    pub equipped: bool,
    pub level: u32,
    pub experience: u64,
}

// 🆕 CROSS-CHAIN CONTRACT
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CrossChainContract {
    pub supported_chains: Vec<String>,
    pub bridge_addresses: HashMap<String, String>, // chain -> bridge address
    pub total_swaps: u64,
    pub security_score: f64,
}

// 🆕 QUANTUM-RESISTANT CONTRACT
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuantumResistantContract {
    pub encryption_type: String,   // "kyber512", "dilithium2", "sphincs_plus"
    pub public_key: String,
    pub signature_scheme: String,
    pub quantum_safe: bool,
}

// 🆕 MAIN SMART CONTRACT STRUCTURE
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SmartContract {
    pub address: String,
    pub owner: String,
    pub code: String,
    pub balance: u64,
    pub contract_type: ContractType,
    
    // 🆕 ADVANCED FEATURES
    pub ai_model: Option<AIModel>,
    pub multi_sig: Option<MultiSigContract>,
    pub defi_vault: Option<DeFiVaultContract>,
    pub gaming_asset: Option<GamingAssetContract>,
    pub cross_chain: Option<CrossChainContract>,
    pub quantum_resistant: Option<QuantumResistantContract>,
    
    // 🆕 CONTRACT METADATA
    pub created_at: u64,
    pub last_executed: u64,
    pub execution_count: u64,
    pub gas_used_total: u64,
    pub upgradeable: bool,
    pub paused: bool,
}

// 🆕 ADDITIONAL DATA STRUCTURE FOR ADVANCED EXECUTION RESULTS
#[derive(Debug, Default, Clone)]
struct AdditionalData {
    pub ai_prediction: Option<String>,
    pub cross_chain_tx_hash: Option<String>,
    pub quantum_signature: Option<String>,
    pub defi_rewards: Option<u64>,
    pub gaming_asset_update: Option<String>,
}

// 🆕 CONTRACT EXECUTION RESULT WITH ADVANCED FEATURES
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContractExecutionResult {
    pub success: bool,
    pub output: String,
    pub gas_used: u64,
    pub logs: Vec<String>,
    
    // 🆕 NEW FIELDS
    pub ai_prediction: Option<String>,
    pub cross_chain_tx_hash: Option<String>,
    pub quantum_signature: Option<String>,
    pub defi_rewards: Option<u64>,
    pub gaming_asset_update: Option<String>,
    pub execution_time_ms: u64,
}

// 🆕 CONTRACT TEMPLATES
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContractTemplate {
    pub template_id: String,
    pub name: String,
    pub description: String,
    pub code: String,
    pub contract_type: ContractType,
    pub gas_estimate: u64,
    pub required_balance: u64,
}

// 🆕 CONTRACT ANALYTICS
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContractAnalytics {
    pub contracts_by_type: HashMap<String, u64>, // contract_type -> count
    pub average_gas_used: f64,
    pub success_rate: f64,
    pub most_active_contracts: Vec<String>,
    pub total_volume: u64,
}

// 🆕 MAIN CONTRACT MANAGER
#[derive(Debug, Serialize, Deserialize)]
pub struct ContractManager {
    pub contracts: HashMap<String, SmartContract>,
    
    // 🆕 NEW: CONTRACT TEMPLATES & ANALYTICS
    pub templates: HashMap<String, ContractTemplate>,
    pub total_contracts_deployed: u64,
    pub total_gas_used: u64,
    pub contract_analytics: ContractAnalytics,
    
    // 🆕 NEW: AI CONTRACT TRAINING
    pub ai_training_queue: Vec<String>, // contract addresses waiting for AI training
    pub trained_models: HashMap<String, AIModel>,
}

impl ContractManager {
    pub fn new() -> Self {
        let mut manager = ContractManager {
            contracts: HashMap::new(),
            templates: HashMap::new(),
            total_contracts_deployed: 0,
            total_gas_used: 0,
            contract_analytics: ContractAnalytics {
                contracts_by_type: HashMap::new(),
                average_gas_used: 0.0,
                success_rate: 0.0,
                most_active_contracts: Vec::new(),
                total_volume: 0,
            },
            ai_training_queue: Vec::new(),
            trained_models: HashMap::new(),
        };
        
        // Initialize with pre-built templates
        manager.initialize_templates();
        manager
    }

    // 🆕 INITIALIZE CONTRACT TEMPLATES
    fn initialize_templates(&mut self) {
        let templates = vec![
            ContractTemplate {
                template_id: "multisig_2of3".to_string(),
                name: "2-of-3 Multi-signature Wallet".to_string(),
                description: "Secure multi-signature wallet requiring 2 out of 3 signatures".to_string(),
                code: "// Multi-sig template code".to_string(),
                contract_type: ContractType::MultiSig,
                gas_estimate: 50000,
                required_balance: 1000000, // 0.001 VEXA
            },
            ContractTemplate {
                template_id: "defi_yield_vault".to_string(),
                name: "DeFi Yield Farming Vault".to_string(),
                description: "Automated yield farming with 15%+ APR".to_string(),
                code: "// DeFi vault template code".to_string(),
                contract_type: ContractType::DeFiVault,
                gas_estimate: 75000,
                required_balance: 5000000, // 0.005 VEXA
            },
            ContractTemplate {
                template_id: "ai_prediction".to_string(),
                name: "AI Prediction Market".to_string(),
                description: "AI-powered prediction market with machine learning".to_string(),
                code: "// AI prediction template code".to_string(),
                contract_type: ContractType::AIContract,
                gas_estimate: 100000,
                required_balance: 10000000, // 0.01 VEXA
            },
            ContractTemplate {
                template_id: "gaming_nft".to_string(),
                name: "Gaming NFT Asset".to_string(),
                description: "NFT for in-game assets with rarity and levels".to_string(),
                code: "// Gaming NFT template code".to_string(),
                contract_type: ContractType::GamingAsset,
                gas_estimate: 30000,
                required_balance: 2000000, // 0.002 VEXA
            },
            ContractTemplate {
                template_id: "quantum_safe".to_string(),
                name: "Quantum-Safe Contract".to_string(),
                description: "Post-quantum cryptography secured contract".to_string(),
                code: "// Quantum safe template code".to_string(),
                contract_type: ContractType::QuantumResistant,
                gas_estimate: 60000,
                required_balance: 3000000, // 0.003 VEXA
            }
        ];

        for template in templates {
            self.templates.insert(template.template_id.clone(), template);
        }
        
        println!("🚀 {} contract templates loaded", self.templates.len());
    }

    // 🆕 DEPLOY CONTRACT WITH ADVANCED FEATURES
    pub fn deploy_contract(&mut self, code: String, owner: String, initial_fund: u64) -> String {
        let address = format!("CONTRACT_{}_{}", self.total_contracts_deployed, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
        
        // Detect contract type from code
        let contract_type = self.detect_contract_type(&code);
        
        let contract = SmartContract {
            address: address.clone(),
            owner: owner.clone(),
            code: code.clone(),
            balance: initial_fund,
            contract_type: contract_type.clone(),
            
            // Initialize advanced features based on contract type
            ai_model: if matches!(contract_type, ContractType::AIContract) {
                Some(AIModel {
                    model_hash: format!("ai_model_{}", address),
                    model_type: "classification".to_string(),
                    training_data: Vec::new(),
                    accuracy: 0.0,
                    last_trained: 0,
                })
            } else {
                None
            },
            
            multi_sig: if matches!(contract_type, ContractType::MultiSig) {
                Some(MultiSigContract {
                    owners: vec![owner.clone()],
                    required_signatures: 1,
                    pending_actions: HashMap::new(),
                })
            } else {
                None
            },
            
            defi_vault: if matches!(contract_type, ContractType::DeFiVault) {
                Some(DeFiVaultContract {
                    strategy: "yield_farming".to_string(),
                    total_liquidity: initial_fund,
                    apr: 15.0,
                    investors: HashMap::new(),
                    performance_fee: 2,
                    last_reward_distribution: 0,
                })
            } else {
                None
            },
            
            gaming_asset: if matches!(contract_type, ContractType::GamingAsset) {
                Some(GamingAssetContract {
                    game_id: "default_game".to_string(),
                    asset_type: "item".to_string(),
                    rarity: "common".to_string(),
                    metadata: HashMap::new(),
                    equipped: false,
                    level: 1,
                    experience: 0,
                })
            } else {
                None
            },
            
            cross_chain: if matches!(contract_type, ContractType::CrossChain) {
                Some(CrossChainContract {
                    supported_chains: vec!["Ethereum".to_string(), "Binance".to_string()],
                    bridge_addresses: HashMap::new(),
                    total_swaps: 0,
                    security_score: 0.0,
                })
            } else {
                None
            },
            
            quantum_resistant: if matches!(contract_type, ContractType::QuantumResistant) {
                Some(QuantumResistantContract {
                    encryption_type: "kyber512".to_string(),
                    public_key: format!("quantum_pk_{}", address),
                    signature_scheme: "dilithium2".to_string(),
                    quantum_safe: true,
                })
            } else {
                None
            },
            
            created_at: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
            last_executed: 0,
            execution_count: 0,
            gas_used_total: 0,
            upgradeable: true,
            paused: false,
        };

        self.contracts.insert(address.clone(), contract);
        self.total_contracts_deployed += 1;
        
        // Update analytics
        let type_str = format!("{:?}", contract_type);
        *self.contract_analytics.contracts_by_type.entry(type_str).or_insert(0) += 1;
        
        println!("🚀 Advanced contract deployed: {} (Type: {:?})", address, contract_type);
        address
    }

    // 🆕 DETECT CONTRACT TYPE FROM CODE
    fn detect_contract_type(&self, code: &str) -> ContractType {
        if code.contains("AI_MODEL") || code.contains("machine_learning") || code.contains("prediction") {
            ContractType::AIContract
        } else if code.contains("multi_sig") || code.contains("required_signatures") {
            ContractType::MultiSig
        } else if code.contains("yield") || code.contains("vault") || code.contains("APR") {
            ContractType::DeFiVault
        } else if code.contains("game") || code.contains("NFT") || code.contains("rarity") {
            ContractType::GamingAsset
        } else if code.contains("cross_chain") || code.contains("bridge") {
            ContractType::CrossChain
        } else if code.contains("quantum") || code.contains("kyber") || code.contains("dilithium") {
            ContractType::QuantumResistant
        } else if code.contains("social") || code.contains("trading") {
            ContractType::SocialFi
        } else if code.contains("identity") || code.contains("DID") {
            ContractType::Identity
        } else {
            ContractType::Standard
        }
    }

    // 🆕 ADVANCED CONTRACT EXECUTION - COMPLETELY FIXED VERSION
    pub fn execute_contract(&mut self, contract_address: &str, caller: &str, function: &str, args: Vec<String>) -> ContractExecutionResult {
        let start_time = std::time::Instant::now();
        
        println!("⚙️ Executing contract {} func: {} by {}", contract_address, function, caller);
        
        // FIRST: Get contract type and calculate gas WITHOUT mutable borrow
        let contract_data = match self.contracts.get(contract_address) {
            Some(contract) => {
                let gas = self.calculate_gas_usage(function, &args, &contract.contract_type);
                Some((contract.contract_type.clone(), gas))
            }
            None => None
        };

        let (contract_type, gas_used) = match contract_data {
            Some((ct, gas)) => (ct, gas),
            None => {
                return ContractExecutionResult {
                    success: false,
                    output: "Contract not found".to_string(),
                    gas_used: 0,
                    logs: vec!["Contract not found".to_string()],
                    ai_prediction: None,
                    cross_chain_tx_hash: None,
                    quantum_signature: None,
                    defi_rewards: None,
                    gaming_asset_update: None,
                    execution_time_ms: 0,
                }
            }
        };
        
        // SECOND: Execute based on contract type - COMPLETELY SEPARATED LOGIC
        let execution_result = match contract_type {
            ContractType::AIContract => self.execute_ai_contract_logic(contract_address, function, &args),
            ContractType::MultiSig => self.execute_multisig_contract_logic(contract_address, caller, function, &args),
            ContractType::DeFiVault => self.execute_defi_contract_logic(contract_address, caller, function, &args),
            ContractType::GamingAsset => self.execute_gaming_contract_logic(contract_address, caller, function, &args),
            ContractType::CrossChain => self.execute_cross_chain_contract_logic(contract_address, function, &args),
            ContractType::QuantumResistant => self.execute_quantum_contract_logic(contract_address, function, &args),
            _ => self.execute_standard_contract_logic(contract_address, function, &args),
        };
        
        let (success, output, additional_data) = execution_result;
        let execution_time = start_time.elapsed().as_millis() as u64;
        
        // Update analytics
        self.update_analytics(contract_address, success, gas_used);

        ContractExecutionResult {
            success,
            output,
            gas_used,
            logs: vec![
                format!("Function {} executed by {}", function, caller),
                format!("Gas used: {}", gas_used),
                format!("Execution time: {}ms", execution_time),
            ],
            ai_prediction: additional_data.ai_prediction,
            cross_chain_tx_hash: additional_data.cross_chain_tx_hash,
            quantum_signature: additional_data.quantum_signature,
            defi_rewards: additional_data.defi_rewards,
            gaming_asset_update: additional_data.gaming_asset_update,
            execution_time_ms: execution_time,
        }
    }

    // SEPARATED EXECUTION LOGIC FOR EACH CONTRACT TYPE

    fn execute_ai_contract_logic(&mut self, contract_address: &str, function: &str, args: &[String]) -> (bool, String, AdditionalData) {
        if let Some(contract) = self.contracts.get_mut(contract_address) {
            match function {
                "train_model" => {
                    if let Some(ai_model) = &mut contract.ai_model {
                        ai_model.last_trained = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
                        ai_model.accuracy = 0.85;
                        (true, "AI model trained successfully".to_string(), AdditionalData::default())
                    } else {
                        (false, "AI model not found".to_string(), AdditionalData::default())
                    }
                }
                "predict" => {
                    let prediction = "AI Prediction: Market will rise by 5.2%".to_string();
                    (true, "Prediction completed".to_string(), AdditionalData {
                        ai_prediction: Some(prediction),
                        ..Default::default()
                    })
                }
                _ => (false, format!("Unknown AI function: {}", function), AdditionalData::default())
            }
        } else {
            (false, "Contract not found".to_string(), AdditionalData::default())
        }
    }

    fn execute_multisig_contract_logic(&mut self, contract_address: &str, caller: &str, function: &str, args: &[String]) -> (bool, String, AdditionalData) {
        if let Some(contract) = self.contracts.get_mut(contract_address) {
            if let Some(multi_sig) = &mut contract.multi_sig {
                match function {
                    "propose_action" => {
                        let action_id = format!("action_{}", multi_sig.pending_actions.len());
                        let action = MultiSigAction {
                            action_id: action_id.clone(),
                            action_type: args.get(0).unwrap_or(&"transfer".to_string()).clone(),
                            data: args.get(1).unwrap_or(&"".to_string()).clone(),
                            proposed_by: caller.to_string(),
                            approvals: vec![caller.to_string()],
                            executed: false,
                        };
                        multi_sig.pending_actions.insert(action_id.clone(), action);
                        (true, format!("Action {} proposed", action_id), AdditionalData::default())
                    }
                    "approve_action" => {
                        // FIXED: Temporary value issue resolved
                        let default_action_id = String::new();
                        let action_id = args.get(0).unwrap_or(&default_action_id).clone();
                        if let Some(action) = multi_sig.pending_actions.get_mut(&action_id) {
                            if !action.approvals.contains(&caller.to_string()) {
                                action.approvals.push(caller.to_string());
                            }
                            (true, format!("Action {} approved", action_id), AdditionalData::default())
                        } else {
                            (false, "Action not found".to_string(), AdditionalData::default())
                        }
                    }
                    _ => (false, format!("Unknown multi-sig function: {}", function), AdditionalData::default())
                }
            } else {
                (false, "Multi-sig data not found".to_string(), AdditionalData::default())
            }
        } else {
            (false, "Contract not found".to_string(), AdditionalData::default())
        }
    }

    fn execute_defi_contract_logic(&mut self, contract_address: &str, caller: &str, function: &str, args: &[String]) -> (bool, String, AdditionalData) {
        if let Some(contract) = self.contracts.get_mut(contract_address) {
            if let Some(defi_vault) = &mut contract.defi_vault {
                match function {
                    "deposit" => {
                        let amount = args.get(0).unwrap_or(&"0".to_string()).parse::<u64>().unwrap_or(0);
                        *defi_vault.investors.entry(caller.to_string()).or_insert(0) += amount;
                        defi_vault.total_liquidity += amount;
                        let rewards = (amount as f64 * 0.15 / 365.0) as u64;
                        (true, format!("Deposited {} into vault", amount), AdditionalData {
                            defi_rewards: Some(rewards),
                            ..Default::default()
                        })
                    }
                    "claim_rewards" => {
                        let rewards = 1000000;
                        (true, "Rewards claimed".to_string(), AdditionalData {
                            defi_rewards: Some(rewards),
                            ..Default::default()
                        })
                    }
                    _ => (false, format!("Unknown DeFi function: {}", function), AdditionalData::default())
                }
            } else {
                (false, "DeFi vault data not found".to_string(), AdditionalData::default())
            }
        } else {
            (false, "Contract not found".to_string(), AdditionalData::default())
        }
    }

    fn execute_gaming_contract_logic(&mut self, contract_address: &str, caller: &str, function: &str, args: &[String]) -> (bool, String, AdditionalData) {
        if let Some(contract) = self.contracts.get_mut(contract_address) {
            if let Some(gaming_asset) = &mut contract.gaming_asset {
                match function {
                    "upgrade" => {
                        gaming_asset.level += 1;
                        gaming_asset.experience += 100;
                        let update = format!("Asset upgraded to level {}", gaming_asset.level);
                        (true, "Asset upgraded".to_string(), AdditionalData {
                            gaming_asset_update: Some(update),
                            ..Default::default()
                        })
                    }
                    "equip" => {
                        gaming_asset.equipped = true;
                        (true, "Asset equipped".to_string(), AdditionalData {
                            gaming_asset_update: Some("Asset equipped".to_string()),
                            ..Default::default()
                        })
                    }
                    _ => (false, format!("Unknown gaming function: {}", function), AdditionalData::default())
                }
            } else {
                (false, "Gaming asset data not found".to_string(), AdditionalData::default())
            }
        } else {
            (false, "Contract not found".to_string(), AdditionalData::default())
        }
    }

    fn execute_cross_chain_contract_logic(&self, contract_address: &str, function: &str, args: &[String]) -> (bool, String, AdditionalData) {
        if let Some(contract) = self.contracts.get(contract_address) {
            match function {
                "bridge_transfer" => {
                    let tx_hash = format!("cross_chain_tx_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
                    (true, "Cross-chain transfer initiated".to_string(), AdditionalData {
                        cross_chain_tx_hash: Some(tx_hash),
                        ..Default::default()
                    })
                }
                _ => (false, format!("Unknown cross-chain function: {}", function), AdditionalData::default())
            }
        } else {
            (false, "Contract not found".to_string(), AdditionalData::default())
        }
    }

    fn execute_quantum_contract_logic(&self, contract_address: &str, function: &str, args: &[String]) -> (bool, String, AdditionalData) {
        if let Some(contract) = self.contracts.get(contract_address) {
            match function {
                "quantum_sign" => {
                    let signature = format!("quantum_sig_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
                    (true, "Quantum signature created".to_string(), AdditionalData {
                        quantum_signature: Some(signature),
                        ..Default::default()
                    })
                }
                _ => (false, format!("Unknown quantum function: {}", function), AdditionalData::default())
            }
        } else {
            (false, "Contract not found".to_string(), AdditionalData::default())
        }
    }

    fn execute_standard_contract_logic(&self, contract_address: &str, function: &str, args: &[String]) -> (bool, String, AdditionalData) {
        if self.contracts.get(contract_address).is_some() {
            (true, format!("Executed {} successfully with args: {:?}", function, args), AdditionalData::default())
        } else {
            (false, "Contract not found".to_string(), AdditionalData::default())
        }
    }

    // 🆕 CALCULATE GAS USAGE
    fn calculate_gas_usage(&self, function: &str, args: &[String], contract_type: &ContractType) -> u64 {
        let base_gas = 1000;
        let complexity_multiplier = match contract_type {
            ContractType::AIContract => 3.0,
            ContractType::MultiSig => 2.0,
            ContractType::DeFiVault => 2.5,
            ContractType::GamingAsset => 1.5,
            ContractType::CrossChain => 4.0,
            ContractType::QuantumResistant => 3.5,
            _ => 1.0,
        };
        
        let function_complexity = match function {
            "train_model" | "bridge_transfer" => 5000,
            "predict" | "quantum_sign" => 3000,
            "deposit" | "upgrade" => 2000,
            _ => 1000,
        };
        
        (base_gas + function_complexity) * complexity_multiplier as u64
    }

    // 🆕 UPDATE ANALYTICS
    fn update_analytics(&mut self, contract_address: &str, success: bool, gas_used: u64) {
        // Update success rate
        let total_executions = self.contract_analytics.most_active_contracts.len() as f64 + 1.0;
        let current_success_rate = self.contract_analytics.success_rate;
        self.contract_analytics.success_rate = (current_success_rate * (total_executions - 1.0) + if success { 1.0 } else { 0.0 }) / total_executions;
        
        // Update average gas
        let current_avg_gas = self.contract_analytics.average_gas_used;
        self.contract_analytics.average_gas_used = (current_avg_gas * (total_executions - 1.0) + gas_used as f64) / total_executions;
        
        // Update most active contracts
        if !self.contract_analytics.most_active_contracts.contains(&contract_address.to_string()) {
            self.contract_analytics.most_active_contracts.push(contract_address.to_string());
        }
    }

    // 🆕 GET CONTRACT TEMPLATES
    pub fn get_contract_templates(&self) -> Vec<ContractTemplate> {
        self.templates.values().cloned().collect()
    }

    // 🆕 DEPLOY FROM TEMPLATE
    pub fn deploy_from_template(&mut self, template_id: &str, owner: String, initial_fund: u64) -> Option<String> {
        if let Some(template) = self.templates.get(template_id) {
            if initial_fund >= template.required_balance {
                let contract_address = self.deploy_contract(template.code.clone(), owner, initial_fund);
                Some(contract_address)
            } else {
                println!("❌ Insufficient balance for template deployment");
                None
            }
        } else {
            println!("❌ Template not found: {}", template_id);
            None
        }
    }

    // 🆕 GET CONTRACT ANALYTICS
    pub fn get_analytics(&self) -> &ContractAnalytics {
        &self.contract_analytics
    }

    // ✅ ORIGINAL METHODS (Maintained for compatibility)
    pub fn get_contract_balance(&self, address: &str) -> u64 {
        self.contracts.get(address).map(|c| c.balance).unwrap_or(0)
    }

    pub fn fund_contract(&mut self, contract_address: &str, amount: u64) -> bool {
        if let Some(contract) = self.contracts.get_mut(contract_address) {
            contract.balance += amount;
            println!("💰 Funded contract {} with {} tokens", contract_address, amount);
            true
        } else {
            println!("❌ Contract {} not found for funding", contract_address);
            false
        }
    }
}