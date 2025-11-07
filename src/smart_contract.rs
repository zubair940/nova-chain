// src/smart_contract.rs
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::collections::HashMap;

// 🔥 VEXA Smart Contract Virtual Machine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VexaContract {
    pub address: String,
    pub code: String,
    pub storage: HashMap<String, String>,
    pub owner: String,
    pub balance: u64,
    pub deployed_block: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractExecutionResult {
    pub success: bool,
    pub output: String,
    pub gas_used: u64,
    pub storage_changes: HashMap<String, String>,
}

// 🔥 VEXA Smart Contract Language Operations
#[derive(Debug, Clone)]
pub enum VexOpCode {
    PUSH(String),      // Push value to stack
    STORE(String),     // Store to storage
    LOAD(String),      // Load from storage  
    ADD,               // Add top two stack values
    SUB,               // Subtract
    MUL,               // Multiply
    DIV,               // Divide
    TRANSFER(String, u64), // Transfer tokens
    CONDITION,         // Conditional execution
    RETURN,            // Return from function
}

impl VexContract {
    pub fn new(code: String, owner: String, deployed_block: u64) -> Self {
        let address = Self::generate_contract_address(&code, &owner);
        
        VexContract {
            address,
            code,
            storage: HashMap::new(),
            owner,
            balance: 0,
            deployed_block,
        }
    }
    
    fn generate_contract_address(code: &str, owner: &str) -> String {
        let data = format!("{}{}", code, owner);
        let hash = Sha256::digest(data.as_bytes());
        format!("vex_{}", hex::encode(&hash[..16]))
    }
    
    // 🔥 Execute VEX Smart Contract
    pub fn execute(&mut self, _caller: &str, function: &str, _args: Vec<String>, gas_limit: u64) -> ContractExecutionResult {
        println!("🤖 Executing VEX Contract: {} function: {}", self.address, function);
        
        let mut gas_used = 0;
        let mut storage_changes = HashMap::new();
        let mut output = String::new();
        
        // Simple VEX VM execution
        let opcodes = self.parse_code(function);
        
        for opcode in opcodes {
            if gas_used >= gas_limit {
                output = "GAS_LIMIT_EXCEEDED".to_string();
                break;
            }
            
            match opcode {
                VexOpCode::PUSH(_value) => {
                    // Stack push simulation
                    gas_used += 10;
                }
                VexOpCode::STORE(key) => {
                    let value = "stored_value".to_string(); // Simplified
                    self.storage.insert(key.clone(), value.clone());
                    storage_changes.insert(key, value);
                    gas_used += 100;
                }
                VexOpCode::LOAD(key) => {
                    if let Some(value) = self.storage.get(&key) {
                        output = value.clone();
                    }
                    gas_used += 50;
                }
                VexOpCode::TRANSFER(to, amount) => {
                    // Token transfer logic
                    if self.balance >= amount {
                        self.balance -= amount;
                        output = format!("TRANSFER_SUCCESS: {} VEX to {}", amount, to);
                    } else {
                        output = "INSUFFICIENT_BALANCE".to_string();
                    }
                    gas_used += 500;
                }
                _ => {
                    gas_used += 10;
                }
            }
        }
        
        ContractExecutionResult {
            success: !output.contains("ERROR"),
            output,
            gas_used,
            storage_changes,
        }
    }
    
    fn parse_code(&self, function: &str) -> Vec<VexOpCode> {
        // Simple VEX language parser
        match function {
            "get_balance" => vec![
                VexOpCode::LOAD("balance".to_string()),
                VexOpCode::RETURN,
            ],
            "transfer" => vec![
                VexOpCode::LOAD("balance".to_string()),
                VexOpCode::TRANSFER("recipient".to_string(), 100),
                VexOpCode::RETURN,
            ],
            "store_data" => vec![
                VexOpCode::PUSH("data".to_string()),
                VexOpCode::STORE("key".to_string()),
                VexOpCode::RETURN,
            ],
            _ => vec![VexOpCode::RETURN],
        }
    }
    
    pub fn fund(&mut self, amount: u64) {
        self.balance += amount;
        println!("💰 Contract {} funded with {} VEX", self.address, amount);
    }
    
    pub fn get_storage(&self, key: &str) -> Option<&String> {
        self.storage.get(key)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VexContractManager {
    pub contracts: HashMap<String, VexContract>,
    pub next_contract_id: u64,
}

impl VexContractManager {
    pub fn new() -> Self {
        VexContractManager {
            contracts: HashMap::new(),
            next_contract_id: 1,
        }
    }
    
    pub fn deploy_contract(&mut self, code: String, owner: String, current_block: u64) -> String {
        let contract = VexContract::new(code, owner, current_block);
        let address = contract.address.clone();
        
        self.contracts.insert(address.clone(), contract);
        println!("🚀 Vex Contract deployed: {}", address);
        
        address
    }
    
    pub fn execute_contract(&mut self, contract_address: &str, caller: &str, function: &str, args: Vec<String>) -> ContractExecutionResult {
        if let Some(contract) = self.contracts.get_mut(contract_address) {
            contract.execute(caller, function, args, 100000)
        } else {
            ContractExecutionResult {
                success: false,
                output: "CONTRACT_NOT_FOUND".to_string(),
                gas_used: 0,
                storage_changes: HashMap::new(),
            }
        }
    }
    
    pub fn get_contract_balance(&self, contract_address: &str) -> u64 {
        self.contracts.get(contract_address)
            .map(|c| c.balance)
            .unwrap_or(0)
    }
    
    pub fn fund_contract(&mut self, contract_address: &str, amount: u64) -> bool {
        if let Some(contract) = self.contracts.get_mut(contract_address) {
            contract.fund(amount);
            true
        } else {
            false
        }
    }
}

// 🔥 Vex Smart Contract Examples
pub struct VexExamples;

impl VexExamples {
    pub fn simple_token_contract() -> String {
        r#"
        // Vex Simple Token Contract
        contract SimpleToken {
            storage balances;
            
            function transfer(to, amount) {
                if balances[msg.sender] >= amount {
                    balances[msg.sender] -= amount;
                    balances[to] += amount;
                    return "SUCCESS";
                }
                return "INSUFFICIENT_BALANCE";
            }
            
            function get_balance(address) {
                return balances[address];
            }
        }
        "#.to_string()
    }
    
    pub fn simple_storage_contract() -> String {
        r#"
        // Vex Simple Storage Contract
        contract SimpleStorage {
            storage data;
            
            function set(key, value) {
                data[key] = value;
                return "STORED";
            }
            
            function get(key) {
                return data[key];
            }
        }
        "#.to_string()
    }
}