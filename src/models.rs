use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub difficulty: u64,
    pub burned_tokens: u64,
    pub dao_proposals: Vec<DAOProposal>,
}

// 🔥 FIXED: Added all required traits for BinaryHeap
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub signature: String,
    pub timestamp: u64,
    pub input_utxo: String,
    pub fee: u64,
    pub nonce: u64,
    pub transaction_type: String,
}

// Required for BinaryHeap
impl PartialOrd for Transaction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Transaction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.fee.cmp(&other.fee).reverse() // Higher fee first
    }
}

// 🆕 NEW: SPONSORED TRANSACTIONS
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SponsoredTransaction {
    pub user_transaction: Transaction,
    pub sponsor_address: String,
    pub sponsor_signature: String,
    pub sponsor_nonce: u64,
}

// 🆕 NEW: DAILY REWARDS SYSTEM
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DailyRewards {
    pub user_rewards: HashMap<String, UserReward>,
    pub daily_base_reward: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserReward {
    pub last_claim_time: u64,
    pub streak_count: u32,
    pub total_claimed: u64,
}

// 🆕 NEW: REFERRAL PROGRAM
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReferralProgram {
    pub referrals: HashMap<String, ReferralData>,
    pub level_rates: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReferralData {
    pub referrer: String,
    pub referral_date: u64,
    pub earned_amount: u64,
}

// 🆕 NEW: SOCIAL WALLETS
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SocialWallet {
    pub phone_number: String,
    pub email: String,
    pub recovery_code: String,
    pub main_address: String,
}

// 🆕 NEW: MICRO-EARNING TASKS
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MicroEarning {
    pub tasks: HashMap<String, EarningTask>,
    pub completed_tasks: HashMap<String, Vec<CompletedTask>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EarningTask {
    pub task_id: String,
    pub task_type: String, // "survey", "watch_ad", "play_game"
    pub reward_amount: u64,
    pub description: String,
    pub active: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompletedTask {
    pub task_id: String,
    pub user_address: String,
    pub completion_time: u64,
    pub reward_earned: u64,
}

// 🆕 NEW: FIAT ON-RAMP
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FiatGateway {
    pub upi_integration: bool,
    pub credit_card_support: bool,
    pub bank_transfer_support: bool,
    pub supported_currencies: Vec<String>,
}

// 🆕 NEW: ONE-CLICK NFT CREATOR
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NFTCreator {
    pub user_nfts: HashMap<String, Vec<UserNFT>>,
    pub marketplace_listings: HashMap<String, NFTListing>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserNFT {
    pub nft_id: String,
    pub owner: String,
    pub metadata: String,
    pub image_url: String,
    pub created_at: u64,
    pub price: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NFTListing {
    pub nft_id: String,
    pub seller: String,
    pub price: u64,
    pub listed_at: u64,
}

// 🆕 NEW: PREDICTION MARKETS
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PredictionMarket {
    pub active_markets: HashMap<String, PredictionEvent>,
    pub user_bets: HashMap<String, Vec<UserBet>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PredictionEvent {
    pub event_id: String,
    pub title: String,
    pub description: String,
    pub options: Vec<String>,
    pub end_time: u64,
    pub resolved: bool,
    pub winning_option: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserBet {
    pub event_id: String,
    pub user_address: String,
    pub option: String,
    pub amount: u64,
    pub placed_at: u64,
    pub won: Option<bool>,
}

// 🆕 NEW: SOCIAL PAYMENT LINKS
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SocialPay {
    pub payment_links: HashMap<String, PaymentLink>,
    pub group_payments: HashMap<String, GroupPayment>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentLink {
    pub link_id: String,
    pub creator: String,
    pub amount: u64,
    pub description: String,
    pub created_at: u64,
    pub claimed: bool,
    pub claimed_by: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroupPayment {
    pub group_id: String,
    pub creator: String,
    pub total_amount: u64,
    pub participants: Vec<Participant>,
    pub settled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Participant {
    pub address: String,
    pub share: u64,
    pub paid: bool,
}

// 🆕 NEW: MULTI-LANGUAGE SUPPORT
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Localization {
    pub supported_languages: Vec<String>,
    pub user_preferences: HashMap<String, String>, // address -> language
}

// 🆕 NEW: OFFLINE TRANSACTIONS
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OfflineTransaction {
    pub qr_code_data: String,
    pub signed_transaction: Transaction,
    pub created_at: u64,
    pub broadcasted: bool,
}

// 🆕 NEW: INSTANT FINALITY ENGINE
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstantFinalityEngine {
    pub pending_instant_txs: HashMap<String, InstantTransaction>,
    pub confirmation_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstantTransaction {
    pub tx_hash: String,
    pub submitted_at: u64,
    pub confirmed: bool,
    pub confirmation_time: u64,
}

// 🆕 NEW: GUARANTEED TRANSACTIONS
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuaranteedTx {
    pub tx_hash: String,
    pub user_address: String,
    pub guaranteed: bool,
    pub insurance_fund: u64,
}

// 🆕 NEW: BIOMETRIC SECURITY
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BiometricAuth {
    pub user_biometrics: HashMap<String, BiometricData>,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BiometricData {
    pub face_id_data: Option<String>,
    pub fingerprint_data: Option<String>,
    pub voice_data: Option<String>,
}

// 🆕 NEW: VOICE COMMANDS
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VoiceCommands {
    pub voice_profiles: HashMap<String, VoiceProfile>,
    pub supported_commands: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VoiceProfile {
    pub user_address: String,
    pub voice_model: String,
    pub created_at: u64,
}

// 🆕 NEW: BUSINESS ACCOUNTS
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BusinessAccounts {
    pub business_profiles: HashMap<String, BusinessProfile>,
    pub invoices: HashMap<String, Invoice>,
}

// 🔥 FIXED: Single definition with missing variants and pub fields
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BusinessProfile {
    pub business_name: String,
    pub owner_address: String,
    pub tax_id: String,
    pub business_type: String, // ✅ ADDED 'pub'
    pub registered_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Invoice {
    pub invoice_id: String,
    pub business_address: String,
    pub customer_address: String,
    pub amount: u64,
    pub description: String,
    pub created_at: u64,
    pub paid: bool,
}

// 🆕 NEW: CROSS-BORDER PAYMENTS
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CrossBorderPay {
    pub supported_countries: Vec<String>,
    pub exchange_rates: HashMap<String, f64>,
    pub pending_transfers: HashMap<String, CrossBorderTransfer>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CrossBorderTransfer {
    pub transfer_id: String,
    pub from_country: String,
    pub to_country: String,
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub exchange_rate: f64,
    pub created_at: u64,
    pub completed: bool,
}

// 🆕 NEW: ANALYTICS DASHBOARD
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AnalyticsDashboard {
    pub user_analytics: HashMap<String, UserAnalytics>,
    pub platform_metrics: PlatformMetrics,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserAnalytics {
    pub user_address: String,
    pub total_earned: u64,
    pub total_spent: u64,
    pub tasks_completed: u32,
    pub referrals_count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlatformMetrics {
    pub total_users: u64,
    pub daily_active_users: u64,
    pub total_transactions: u64,
    pub total_volume: u64,
}

// 🆕 NEW: LEARN & EARN
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LearnAndEarn {
    pub courses: HashMap<String, Course>,
    pub user_progress: HashMap<String, UserProgress>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Course {
    pub course_id: String,
    pub title: String,
    pub description: String,
    pub lessons: Vec<Lesson>,
    pub total_reward: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Lesson {
    pub lesson_id: String,
    pub title: String,
    pub content: String,
    pub quiz: Option<Quiz>,
    pub reward: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Quiz {
    pub questions: Vec<Question>,
    pub passing_score: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Question {
    pub question: String,
    pub options: Vec<String>,
    pub correct_answer: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserProgress {
    pub user_address: String,
    pub completed_courses: Vec<String>,
    pub current_lesson: Option<String>,
    pub total_earned: u64,
}

// 🆕 NEW: AI ASSISTANT
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AIAssistant {
    pub chat_sessions: HashMap<String, ChatSession>,
    pub ai_model: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatSession {
    pub session_id: String,
    pub user_address: String,
    pub messages: Vec<ChatMessage>,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatMessage {
    pub role: String, // "user" or "assistant"
    pub content: String,
    pub timestamp: u64,
}

// 🆕 NEW: MASS ADOPTION MANAGER
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MassAdoptionManager {
    pub sponsored_transactions: Vec<SponsoredTransaction>,
    pub daily_rewards: DailyRewards,
    pub referral_program: ReferralProgram,
    pub social_wallets: HashMap<String, SocialWallet>,
    pub micro_earning: MicroEarning,
    pub fiat_gateway: FiatGateway,
    pub nft_creator: NFTCreator,
    pub prediction_market: PredictionMarket,
    pub social_pay: SocialPay,
    pub localization: Localization,
    pub offline_transactions: Vec<OfflineTransaction>,
    pub instant_finality_engine: InstantFinalityEngine,
    pub guaranteed_transactions: Vec<GuaranteedTx>,
    pub biometric_auth: BiometricAuth,
    pub voice_commands: VoiceCommands,
    pub business_accounts: BusinessAccounts,
    pub cross_border_pay: CrossBorderPay,
    pub analytics_dashboard: AnalyticsDashboard,
    pub learn_and_earn: LearnAndEarn,
    pub ai_assistant: AIAssistant,
}

// 🆕 ADVANCED FEATURES - NEWLY ADDED

// 1. MULTI-SIGNATURE WALLETS
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MultiSigWallet {
    pub address: String,
    pub owners: Vec<String>,
    pub required_signatures: u8,
    pub pending_transactions: HashMap<String, MultiSigTransaction>,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MultiSigTransaction {
    pub tx_hash: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub signatures: Vec<String>,
    pub created_at: u64,
    pub executed: bool,
}

// 2. FLASH LOANS
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlashLoan {
    pub id: String,
    pub borrower: String,
    pub amount: u64,
    pub asset: String,
    pub fee: u64,
    pub timestamp: u64,
    pub repaid: bool,
    pub collateral: u64,
}

// 3. QUANTUM-RESISTANT WALLETS
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QuantumResistantWallet {
    pub address: String,
    pub kyber512_public_key: String,
    pub dilithium2_signature: String,
    pub sphincs_plus_backup: String,
    pub created_at: u64,
}

// 4. AI SMART CONTRACTS
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AIContract {
    pub address: String,
    pub ml_model_hash: String,
    pub prediction_engine: bool,
    pub automated_governance: bool,
    pub training_data: Vec<String>,
    pub created_at: u64,
}

// 5. CROSS-CHAIN BRIDGE
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CrossChainBridge {
    pub supported_chains: Vec<SupportedChain>,
    pub liquidity_pools: HashMap<String, u64>,
    pub bridge_fees: BridgeFeeStructure,
    pub total_volume: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SupportedChain {
    pub name: String,
    pub chain_id: String,
    pub bridge_address: String,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BridgeFeeStructure {
    pub percentage: f64,
    pub min_fee: u64,
    pub max_fee: u64,
}

// 6. GAMING ASSETS
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GamingAsset {
    pub asset_id: String,
    pub owner: String,
    pub game_id: String,
    pub metadata: String,
    pub rarity: String,
    pub equipped: bool,
    pub created_at: u64,
}

// 7. METAVERSE ASSETS
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MetaverseAsset {
    pub asset_id: String,
    pub owner: String,
    pub virtual_world: String,
    pub coordinates: (f64, f64, f64),
    pub asset_type: String,
    pub value: u64,
    pub created_at: u64,
}

// 8. DEFI VAULTS
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeFiVault {
    pub vault_id: String,
    pub creator: String,
    pub total_liquidity: u64,
    pub strategy: String,
    pub apr: f64,
    pub investors: HashMap<String, u64>,
    pub created_at: u64,
}

// 9. ENTERPRISE ACCOUNTS
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EnterpriseAccount {
    pub account_id: String,
    pub company_name: String,
    pub tax_id: String,
    pub kyc_verified: bool,
    pub private_transactions: bool,
    pub compliance_tools: bool,
    pub created_at: u64,
}

// 10. SOCIALFI PLATFORM
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SocialFi {
    pub social_trading: HashMap<String, SocialTrader>,
    pub content_creators: HashMap<String, ContentCreator>,
    pub decentralized_social: HashMap<String, SocialPost>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SocialTrader {
    pub address: String,
    pub followers: Vec<String>,
    pub performance: f64,
    pub copy_trading_enabled: bool,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ContentCreator {
    pub address: String,
    pub content_hash: String,
    pub earnings: u64,
    pub subscribers: Vec<String>,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SocialPost {
    pub post_id: String,
    pub author: String,
    pub content: String,
    pub likes: u64,
    pub tips: u64,
    pub timestamp: u64,
}

// 11. DECENTRALIZED IDENTITY
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DecentralizedIdentity {
    pub did: String,
    pub owner: String,
    pub soulbound_tokens: Vec<String>,
    pub verifiable_credentials: Vec<String>,
    pub reputation_score: f64,
    pub created_at: u64,
}

// 12. CONCENTRATED LIQUIDITY
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConcentratedLiquidity {
    pub position_id: String,
    pub owner: String,
    pub token_a: String,
    pub token_b: String,
    pub lower_tick: i32,
    pub upper_tick: i32,
    pub liquidity: u64,
    pub created_at: u64,
}

// 13. OPTIONS TRADING
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OptionsContract {
    pub contract_id: String,
    pub underlying_asset: String,
    pub strike_price: u64,
    pub expiration: u64,
    pub option_type: String, // "call" or "put"
    pub writer: String,
    pub buyer: Option<String>,
    pub premium: u64,
    pub created_at: u64,
}

// 14. INSURANCE POOLS
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InsurancePool {
    pub pool_id: String,
    pub insured_asset: String,
    pub total_coverage: u64,
    pub premium_rate: f64,
    pub claims: Vec<InsuranceClaim>,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InsuranceClaim {
    pub claim_id: String,
    pub claimant: String,
    pub amount: u64,
    pub reason: String,
    pub approved: bool,
    pub timestamp: u64,
}

// 15. YIELD AGGREGATOR
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct YieldAggregator {
    pub strategy_id: String,
    pub name: String,
    pub apy: f64,
    pub total_deposits: u64,
    pub risk_level: String,
    pub created_at: u64,
}

// 16. LAYER 2 SOLUTIONS
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Layer2Solution {
    pub solution_id: String,
    pub name: String,
    pub technology: String, // "zkRollup", "Optimistic", "Sidechain"
    pub tps_capacity: u64,
    pub enabled: bool,
    pub created_at: u64,
}

// 17. DECENTRALIZED STORAGE
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DecentralizedStorage {
    pub file_id: String,
    pub owner: String,
    pub file_hash: String,
    pub file_size: u64,
    pub storage_providers: Vec<String>,
    pub created_at: u64,
}

// 18. CBDC READINESS
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CBDCIntegration {
    pub integration_id: String,
    pub central_bank: String,
    pub currency_code: String,
    pub integration_level: String,
    pub enabled: bool,
    pub created_at: u64,
}

// 19. QUANTUM KEY DISTRIBUTION
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QuantumKeyDistribution {
    pub session_id: String,
    pub sender: String,
    pub receiver: String,
    pub quantum_key: String,
    pub established_at: u64,
    pub active: bool,
}

// 20. ZERO-KNOWLEDGE PROOFS
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ZeroKnowledgeProof {
    pub proof_id: String,
    pub prover: String,
    pub statement: String,
    pub proof_data: String,
    pub verified: bool,
    pub created_at: u64,
}

// 🆕 UPDATED TOKENOMICS STRUCTURES - GATE.IO FRIENDLY
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tokenomics {
    pub total_supply: u64,
    pub allocations: HashMap<String, TokenAllocation>,
    pub vesting_schedules: HashMap<String, VestingSchedule>,
    pub circulating_supply: u64,
    pub burned_tokens: u64,
    pub inflation_rate: f64,
    pub deflation_rate: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenAllocation {
    pub category: String,
    pub amount: u64,
    pub percentage: f64,
    pub wallet_address: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VestingSchedule {
    pub wallet_address: String,
    pub total_tokens: u64,
    pub vested_tokens: u64,
    pub vesting_start: u64,
    pub vesting_duration: u64, // in seconds
    pub cliff_period: u64,     // in seconds
    pub unlocked_tokens: u64,
    pub last_claim_time: u64,
}

// 🆕 UPDATED GENESIS ALLOCATION STRUCTURE
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GenesisAllocation {
    pub wallet_address: String,
    pub initial_balance: u64,
    pub allocation_type: AllocationType,
    pub vesting_schedule: Option<VestingSchedule>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AllocationType {
    Foundation,
    Team,
    Investors,
    PublicSale,
    Ecosystem,
    Liquidity,
    StakingRewards,
    Community,
    Partnerships,
    Security,
}

// Existing structs (jo pehle se the) - UNCHANGED
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Wallet {
    pub public_key: String,
    pub private_key: String,
    pub address: String,
    pub nonce: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Peer {
    pub address: String,
    pub version: String,
    pub height: u64,
    pub last_seen: u64,
    pub reputation: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UTXO {
    pub tx_hash: String,
    pub output_index: u32,
    pub address: String,
    pub amount: u64,
    pub spent: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockchainData {
    pub chain: Vec<Block>,
    pub difficulty: u64,
    pub pending_transactions: Vec<Transaction>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Fork {
    pub blocks: Vec<Block>,
    pub total_difficulty: u64,
    pub length: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockchainConfig {
    pub block_time_target: u64,
    pub difficulty_adjustment_blocks: u64,
    pub initial_difficulty: u64,
    pub min_transaction_fee: u64,
    pub max_block_size: usize,
    pub max_mempool_size: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Mempool {
    pub transactions: VecDeque<Transaction>,
    pub seen_txids: HashSet<String>,
    pub max_size: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetworkPeer {
    pub address: String,
    pub last_seen: u64,
    pub reputation: i32,
    pub version: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ContractTransaction {
    pub contract_address: String,
    pub function: String,
    pub args: Vec<String>,
    pub caller: String,
    pub value: u64,
    pub gas_limit: u64,
    pub gas_price: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ContractExecutionResult {
    pub success: bool,
    pub output: String,
    pub gas_used: u64,
    pub storage_changes: HashMap<String, String>,
    pub logs: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VexContract {
    pub address: String,
    pub code: String,
    pub storage: HashMap<String, String>,
    pub owner: String,
    pub balance: u64,
    pub deployed_block: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VexContractManager {
    pub contracts: HashMap<String, VexContract>,
    pub next_contract_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SyncStatus {
    pub in_progress: bool,
    pub progress: f64,
    pub current_height: u64,
    pub target_height: u64,
    pub retry_count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetworkInfo {
    pub total_peers: usize,
    pub connected_peers: usize,
    pub node_address: String,
    pub seed_nodes: usize,
    pub network_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UTXOData {
    pub address: String,
    pub amount: u64,
    pub block_hash: String,
    pub spent: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StakingPool {
    pub total_staked: u64,
    pub stakers: HashMap<String, Staker>,
    pub apr: u16,
    pub min_stake_amount: u64,
    pub lock_period: u64,
    pub total_rewards_distributed: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Staker {
    pub address: String,
    pub staked_amount: u64,
    pub stake_time: u64,
    pub rewards_earned: u64,
    pub unlock_time: u64,
    pub last_reward_calculation: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StakeTransaction {
    pub staker_address: String,
    pub amount: u64,
    pub lock_period: u64,
    pub timestamp: u64,
    pub signature: String,
    pub tx_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnstakeTransaction {
    pub staker_address: String,
    pub amount: u64,
    pub timestamp: u64,
    pub signature: String,
    pub tx_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RewardDistribution {
    pub staker_address: String,
    pub amount: u64,
    pub timestamp: u64,
    pub block_height: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DAOProposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub proposer: String,
    pub voting_deadline: u64,
    pub votes_for: u64,
    pub votes_against: u64,
    pub executed: bool,
    pub target_contract: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DAOVote {
    pub voter: String,
    pub proposal_id: u64,
    pub vote: bool,
    pub voting_power: u64,
    pub timestamp: u64,
}

// ... (existing enums and other structs)

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionPool {
    pub transactions: Vec<Transaction>,
    pub seen_txids: HashSet<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Consensus {
    pub algo_name: String,
    pub validators: HashSet<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PeerType {
    FullNode,
    LightNode,
    Validator,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum StakingTier {
    Standard,
    Premium,
    VIP,
}

// 🔥 FIXED: Single SecurityLevel definition with ALL variants including Maximum
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityLevel {
    Low,      // ✅ ADDED THIS
    Basic,    
    Medium,   
    High,
    Critical, // ✅ ADDED THIS
    Maximum,  // ✅ ADDED THIS - for wallet.rs compatibility
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BurnEvent {
    pub id: String,
    pub from: String,
    pub amount: u64,
    pub timestamp: u64,
    pub token: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Proposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub proposer: String,
    pub votes_for: u64,
    pub votes_against: u64,
    pub status: ProposalStatus,
    pub start_time: u64,
    pub end_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Expired,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vote {
    pub proposal_id: String,
    pub voter: String,
    pub vote: bool, // true = for, false = against
    pub timestamp: u64,
    pub weight: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BridgeTransaction {
    pub id: String,
    pub from_chain: String,
    pub to_chain: String,
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub timestamp: u64,
    pub status: BridgeStatus,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BridgeStatus {
    Pending,
    Confirmed,
    Completed,
    Failed,
}