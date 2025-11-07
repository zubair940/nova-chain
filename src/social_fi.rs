// src/social_fi.rs
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};

// Social Finance Platform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialFiPlatform {
    pub platform_id: String,
    pub name: String,
    pub description: String,
    pub social_features: SocialFeatures,
    pub financial_features: FinancialFeatures,
    pub token_economy: SocialTokenEconomy,
    pub user_base: UserBase,
    pub content_ecosystem: ContentEcosystem,
    pub created_at: u64,
    pub is_live: bool,
}

// Social Features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialFeatures {
    pub user_profiles: bool,
    pub social_feed: bool,
    pub messaging: bool,
    pub groups_communities: bool,
    pub content_creation: bool,
    pub social_trading: bool,
    pub reputation_system: bool,
    pub governance: bool,
}

// Financial Features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialFeatures {
    pub tipping: bool,
    pub subscriptions: bool,
    pub nft_marketplace: bool,
    pub prediction_markets: bool,
    pub social_tokens: bool,
    pub defi_integration: bool,
    pub staking_rewards: bool,
    pub revenue_sharing: bool,
}

// Social Token Economy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialTokenEconomy {
    pub platform_token: SocialToken,
    pub creator_tokens: HashMap<String, CreatorToken>,
    pub reward_pools: HashMap<String, RewardPool>,
    pub staking_system: SocialStaking,
    pub governance_system: SocialGovernance,
}

// User Base Management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBase {
    pub total_users: u32,
    pub active_users: u32,
    pub creators: u32,
    pub investors: u32,
    pub daily_engagement: u64,
    pub user_growth_rate: f64,
}

// Content Ecosystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentEcosystem {
    pub total_content: u32,
    pub daily_uploads: u32,
    pub content_types: Vec<ContentType>,
    pub moderation_system: ModerationSystem,
    pub copyright_protection: bool,
}

// Social Token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialToken {
    pub token_id: String,
    pub name: String,
    pub symbol: String,
    pub total_supply: u64,
    pub circulating_supply: u64,
    pub token_utility: TokenUtility,
    pub distribution_mechanism: DistributionMechanism,
}

// Creator Token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorToken {
    pub token_id: String,
    pub creator_id: String,
    pub name: String,
    pub symbol: String,
    pub total_supply: u64,
    pub market_cap: u64,
    pub token_price: f64,
    pub holder_count: u32,
    pub trading_volume: u64,
}

// Reward Pool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardPool {
    pub pool_id: String,
    pub pool_type: RewardPoolType,
    pub total_rewards: u64,
    pub distributed_rewards: u64,
    pub participants: u32,
    pub distribution_criteria: DistributionCriteria,
}

// Social Staking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialStaking {
    pub staking_pools: HashMap<String, StakingPool>,
    pub total_staked: u64,
    pub average_apy: f64,
    pub staking_rewards: u64,
}

// Social Governance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialGovernance {
    pub proposals: HashMap<String, GovernanceProposal>,
    pub voting_power: VotingPowerSystem,
    pub treasury: GovernanceTreasury,
    pub executed_proposals: u32,
}

// User Profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: String,
    pub wallet_address: String,
    pub username: String,
    pub bio: String,
    pub avatar_url: String,
    pub cover_url: String,
    pub social_stats: SocialStats,
    pub financial_stats: FinancialStats,
    pub content_portfolio: ContentPortfolio,
    pub reputation_score: f64,
    pub created_at: u64,
    pub last_active: u64,
}

// Social Stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialStats {
    pub followers: u32,
    pub following: u32,
    pub posts: u32,
    pub likes_received: u64,
    pub comments_received: u32,
    pub shares: u32,
    pub engagement_rate: f64,
}

// Financial Stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialStats {
    pub total_earnings: u64,
    pub token_balance: u64,
    pub nft_portfolio_value: u64,
    pub staking_rewards: u64,
    pub trading_profit: i64,
    pub creator_earnings: u64,
}

// Content Portfolio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentPortfolio {
    pub content_items: HashMap<String, ContentItem>,
    pub total_views: u64,
    pub average_rating: f64,
    pub monetized_content: u32,
    pub top_performing: Vec<String>,
}

// Content Item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentItem {
    pub content_id: String,
    pub title: String,
    pub description: String,
    pub content_type: ContentType,
    pub media_url: String,
    pub creator: String,
    pub created_at: u64,
    pub stats: ContentStats,
    pub monetization: Monetization,
    pub nft_metadata: Option<NFTMetadata>,
}

// Content Stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentStats {
    pub views: u64,
    pub likes: u32,
    pub comments: u32,
    pub shares: u32,
    pub watch_time: u64,
    pub engagement_score: f64,
}

// Monetization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monetization {
    pub is_monetized: bool,
    pub revenue_model: RevenueModel,
    pub total_earnings: u64,
    pub current_earnings: u64,
    pub subscriber_count: u32,
}

// NFT Metadata for Content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTMetadata {
    pub nft_id: String,
    pub edition_number: u32,
    pub total_editions: u32,
    pub royalty_percentage: f64,
    pub is_minted: bool,
}

// Social Post
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialPost {
    pub post_id: String,
    pub author: String,
    pub content: String,
    pub media_attachments: Vec<String>,
    pub timestamp: u64,
    pub engagement: PostEngagement,
    pub monetization: PostMonetization,
    pub nft_attached: bool,
}

// Post Engagement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostEngagement {
    pub likes: u32,
    pub comments: u32,
    pub shares: u32,
    pub tips: u64,
    pub views: u64,
    pub engagement_rate: f64,
}

// Post Monetization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostMonetization {
    pub is_monetized: bool,
    pub tip_amount: u64,
    pub nft_sales: u64,
    pub ad_revenue: u64,
    pub total_earnings: u64,
}

// Social Trading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialTrading {
    pub trading_groups: HashMap<String, TradingGroup>,
    pub copy_trading: bool,
    pub signal_providers: Vec<SignalProvider>,
    pub performance_rankings: HashMap<String, f64>,
}

// Trading Group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingGroup {
    pub group_id: String,
    pub name: String,
    pub description: String,
    pub members: u32,
    pub total_trades: u32,
    pub success_rate: f64,
    pub average_profit: f64,
}

// Signal Provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalProvider {
    pub provider_id: String,
    pub user_id: String,
    pub performance: TradingPerformance,
    pub subscription_fee: u64,
    pub subscriber_count: u32,
    pub success_rate: f64,
}

// Trading Performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingPerformance {
    pub total_trades: u32,
    pub profitable_trades: u32,
    pub total_pnl: i64,
    pub average_pnl: f64,
    pub win_rate: f64,
    pub max_drawdown: f64,
}

// Supporting Enums
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContentType {
    Video,
    Audio,
    Image,
    Article,
    LiveStream,
    Podcast,
    ShortForm,
    Story,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TokenUtility {
    Governance,
    Staking,
    Payments,
    Access,
    Rewards,
    Trading,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DistributionMechanism {
    Airdrop,
    StakingRewards,
    ContentRewards,
    LiquidityMining,
    CommunityGrants,
    TeamAllocation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RewardPoolType {
    ContentCreation,
    CommunityEngagement,
    PlatformGrowth,
    LiquidityProvision,
    GovernanceParticipation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DistributionCriteria {
    ViewsBased,
    EngagementBased,
    QualityBased,
    CommunityVoted,
    Algorithmic,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RevenueModel {
    Advertising,
    Subscriptions,
    TipsDonations,
    NFTsales,
    Sponsorships,
    Affiliate,
    Multiple,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModerationLevel {
    Automated,
    Community,
    Professional,
    Hybrid,
}

// Moderation System
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationSystem {
    pub moderation_level: ModerationLevel,
    pub automated_filters: bool,
    community_reporting: bool,
    appeal_process: bool,
    content_rating: bool,
}

// Staking Pool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakingPool {
    pub pool_id: String,
    pub token: String,
    pub total_staked: u64,
    pub apy: f64,
    pub lock_period: u64,
    pub rewards_distributed: u64,
}

// Governance Proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceProposal {
    pub proposal_id: String,
    pub title: String,
    pub description: String,
    pub proposer: String,
    pub created_at: u64,
    pub voting_end: u64,
    pub votes_for: u64,
    pub votes_against: u64,
    pub status: ProposalStatus,
    pub execution_result: Option<ExecutionResult>,
}

// Voting Power System
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingPowerSystem {
    pub token_based: bool,
    pub reputation_based: bool,
    pub quadratic_voting: bool,
    pub vote_delegation: bool,
}

// Governance Treasury
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceTreasury {
    pub total_funds: u64,
    pub allocated_funds: u64,
    pub proposal_budget: u64,
    pub community_grants: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProposalStatus {
    Draft,
    Active,
    Passed,
    Rejected,
    Executed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExecutionResult {
    Success,
    Failed,
    PartiallyExecuted,
}

// SocialFi Manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialFiManager {
    pub platform: SocialFiPlatform,
    pub user_profiles: HashMap<String, UserProfile>,
    pub social_posts: HashMap<String, SocialPost>,
    pub content_items: HashMap<String, ContentItem>,
    pub social_trading: SocialTrading,
    pub analytics: SocialAnalytics,
}

// Social Analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialAnalytics {
    pub platform_metrics: PlatformMetrics,
    pub user_analytics: UserAnalytics,
    pub content_analytics: ContentAnalytics,
    pub financial_analytics: FinancialAnalytics,
}

// Platform Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformMetrics {
    pub daily_active_users: u32,
    pub new_signups: u32,
    pub platform_engagement: f64,
    pub retention_rate: f64,
    pub viral_coefficient: f64,
}

// User Analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAnalytics {
    pub user_segments: HashMap<String, UserSegment>,
    pub behavior_patterns: Vec<BehaviorPattern>,
    pub churn_risk: f64,
    pub lifetime_value: f64,
}

// Content Analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentAnalytics {
    pub trending_content: Vec<String>,
    pub content_performance: HashMap<String, f64>,
    pub creator_rankings: HashMap<String, u32>,
    pub content_discovery: f64,
}

// Financial Analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialAnalytics {
    pub daily_volume: u64,
    pub token_transactions: u32,
    pub nft_sales: u64,
    pub staking_activity: u64,
}

// User Segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSegment {
    pub segment_name: String,
    pub user_count: u32,
    pub average_engagement: f64,
    pub monetization_rate: f64,
}

// Behavior Pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorPattern {
    pub pattern_type: String,
    pub frequency: u32,
    pub monetization_impact: f64,
}

impl SocialFiPlatform {
    pub fn new(name: String, description: String) -> Self {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let social_features = SocialFeatures {
            user_profiles: true,
            social_feed: true,
            messaging: true,
            groups_communities: true,
            content_creation: true,
            social_trading: true,
            reputation_system: true,
            governance: true,
        };

        let financial_features = FinancialFeatures {
            tipping: true,
            subscriptions: true,
            nft_marketplace: true,
            prediction_markets: true,
            social_tokens: true,
            defi_integration: true,
            staking_rewards: true,
            revenue_sharing: true,
        };

        let platform_token = SocialToken {
            token_id: "SOCIAL".to_string(),
            name: "Social Token".to_string(),
            symbol: "SOCIAL",
            total_supply: 1_000_000_000,
            circulating_supply: 100_000_000,
            token_utility: TokenUtility::All,
            distribution_mechanism: DistributionMechanism::StakingRewards,
        };

        let token_economy = SocialTokenEconomy {
            platform_token,
            creator_tokens: HashMap::new(),
            reward_pools: HashMap::new(),
            staking_system: SocialStaking {
                staking_pools: HashMap::new(),
                total_staked: 0,
                average_apy: 15.0,
                staking_rewards: 0,
            },
            governance_system: SocialGovernance {
                proposals: HashMap::new(),
                voting_power: VotingPowerSystem {
                    token_based: true,
                    reputation_based: true,
                    quadratic_voting: false,
                    vote_delegation: true,
                },
                treasury: GovernanceTreasury {
                    total_funds: 1_000_000,
                    allocated_funds: 0,
                    proposal_budget: 100_000,
                    community_grants: 50_000,
                },
                executed_proposals: 0,
            },
        };

        let user_base = UserBase {
            total_users: 0,
            active_users: 0,
            creators: 0,
            investors: 0,
            daily_engagement: 0,
            user_growth_rate: 0.0,
        };

        let content_ecosystem = ContentEcosystem {
            total_content: 0,
            daily_uploads: 0,
            content_types: vec![
                ContentType::Video,
                ContentType::Audio,
                ContentType::Image,
                ContentType::Article,
            ],
            moderation_system: ModerationSystem {
                moderation_level: ModerationLevel::Hybrid,
                automated_filters: true,
                community_reporting: true,
                appeal_process: true,
                content_rating: true,
            },
            copyright_protection: true,
        };

        Self {
            platform_id: format!("socialfi_{}_{}", name.to_lowercase().replace(" ", "_"), current_time),
            name,
            description,
            social_features,
            financial_features,
            token_economy,
            user_base,
            content_ecosystem,
            created_at: current_time,
            is_live: true,
        }
    }

    // Create user profile
    pub fn create_user_profile(
        &mut self,
        wallet_address: String,
        username: String,
        bio: String,
    ) -> UserProfile {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let user_id = format!("user_{}_{}", username.to_lowercase(), current_time);

        let user_profile = UserProfile {
            user_id: user_id.clone(),
            wallet_address,
            username,
            bio,
            avatar_url: "default_avatar.png".to_string(),
            cover_url: "default_cover.png".to_string(),
            social_stats: SocialStats {
                followers: 0,
                following: 0,
                posts: 0,
                likes_received: 0,
                comments_received: 0,
                shares: 0,
                engagement_rate: 0.0,
            },
            financial_stats: FinancialStats {
                total_earnings: 0,
                token_balance: 1000, // Initial airdrop
                nft_portfolio_value: 0,
                staking_rewards: 0,
                trading_profit: 0,
                creator_earnings: 0,
            },
            content_portfolio: ContentPortfolio {
                content_items: HashMap::new(),
                total_views: 0,
                average_rating: 0.0,
                monetized_content: 0,
                top_performing: Vec::new(),
            },
            reputation_score: 50.0, // Starting reputation
            created_at: current_time,
            last_active: current_time,
        };

        // Update platform stats
        self.user_base.total_users += 1;
        self.user_base.active_users += 1;

        user_profile
    }

    // Create social post
    pub fn create_social_post(
        &mut self,
        author: String,
        content: String,
        media_attachments: Vec<String>,
    ) -> SocialPost {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let post_id = format!("post_{}_{}", author, current_time);

        let post = SocialPost {
            post_id: post_id.clone(),
            author,
            content,
            media_attachments,
            timestamp: current_time,
            engagement: PostEngagement {
                likes: 0,
                comments: 0,
                shares: 0,
                tips: 0,
                views: 0,
                engagement_rate: 0.0,
            },
            monetization: PostMonetization {
                is_monetized: false,
                tip_amount: 0,
                nft_sales: 0,
                ad_revenue: 0,
                total_earnings: 0,
            },
            nft_attached: false,
        };

        // Update content ecosystem
        self.content_ecosystem.total_content += 1;
        self.content_ecosystem.daily_uploads += 1;

        post
    }

    // Tip a post
    pub fn tip_post(&mut self, post: &mut SocialPost, tipper: String, amount: u64) -> bool {
        post.engagement.tips += amount;
        post.monetization.tip_amount += amount;
        post.monetization.total_earnings += amount;
        post.monetization.is_monetized = true;

        // Update platform engagement
        self.user_base.daily_engagement += amount;

        println!("💰 Tip received: {} VEXA from {}", amount, tipper);
        true
    }

    // Create creator token
    pub fn create_creator_token(
        &mut self,
        creator_id: String,
        name: String,
        symbol: String,
        total_supply: u64,
    ) -> CreatorToken {
        let token_id = format!("creator_{}_{}", symbol.to_lowercase(), creator_id);

        let creator_token = CreatorToken {
            token_id: token_id.clone(),
            creator_id,
            name,
            symbol,
            total_supply,
            market_cap: 0,
            token_price: 0.0,
            holder_count: 0,
            trading_volume: 0,
        };

        self.token_economy.creator_tokens.insert(token_id, creator_token.clone());
        creator_token
    }

    // Get platform statistics
    pub fn get_platform_stats(&self) -> String {
        format!(
            "SocialFi Platform Statistics:\n\
            Platform: {}\n\
            Total Users: {}\n\
            Active Users: {}\n\
            Creators: {}\n\
            Total Content: {}\n\
            Daily Uploads: {}\n\
            Creator Tokens: {}\n\
            Total Staked: {} SOCIAL\n\
            Governance Proposals: {}",
            self.name,
            self.user_base.total_users,
            self.user_base.active_users,
            self.user_base.creators,
            self.content_ecosystem.total_content,
            self.content_ecosystem.daily_uploads,
            self.token_economy.creator_tokens.len(),
            self.token_economy.staking_system.total_staked,
            self.token_economy.governance_system.proposals.len()
        )
    }
}

impl SocialFiManager {
    pub fn new(platform_name: String, platform_description: String) -> Self {
        let platform = SocialFiPlatform::new(platform_name, platform_description);

        let social_trading = SocialTrading {
            trading_groups: HashMap::new(),
            copy_trading: true,
            signal_providers: Vec::new(),
            performance_rankings: HashMap::new(),
        };

        let analytics = SocialAnalytics {
            platform_metrics: PlatformMetrics {
                daily_active_users: 0,
                new_signups: 0,
                platform_engagement: 0.0,
                retention_rate: 0.0,
                viral_coefficient: 0.0,
            },
            user_analytics: UserAnalytics {
                user_segments: HashMap::new(),
                behavior_patterns: Vec::new(),
                churn_risk: 0.0,
                lifetime_value: 0.0,
            },
            content_analytics: ContentAnalytics {
                trending_content: Vec::new(),
                content_performance: HashMap::new(),
                creator_rankings: HashMap::new(),
                content_discovery: 0.0,
            },
            financial_analytics: FinancialAnalytics {
                daily_volume: 0,
                token_transactions: 0,
                nft_sales: 0,
                staking_activity: 0,
            },
        };

        Self {
            platform,
            user_profiles: HashMap::new(),
            social_posts: HashMap::new(),
            content_items: HashMap::new(),
            social_trading,
            analytics,
        }
    }

    // Register new user
    pub fn register_user(
        &mut self,
        wallet_address: String,
        username: String,
        bio: String,
    ) -> String {
        let user_profile = self.platform.create_user_profile(wallet_address, username, bio);
        let user_id = user_profile.user_id.clone();

        self.user_profiles.insert(user_id.clone(), user_profile);
        
        // Update analytics
        self.analytics.platform_metrics.new_signups += 1;
        self.analytics.platform_metrics.daily_active_users += 1;

        user_id
    }

    // Create and publish post
    pub fn create_post(
        &mut self,
        author_id: String,
        content: String,
        media_attachments: Vec<String>,
    ) -> String {
        let post = self.platform.create_social_post(author_id, content, media_attachments);
        let post_id = post.post_id.clone();

        self.social_posts.insert(post_id.clone(), post);

        // Update user stats
        if let Some(user) = self.user_profiles.get_mut(&author_id) {
            user.social_stats.posts += 1;
            user.last_active = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }

        post_id
    }

    // Like a post
    pub fn like_post(&mut self, post_id: String, user_id: String) -> bool {
        if let Some(post) = self.social_posts.get_mut(&post_id) {
            post.engagement.likes += 1;
            post.engagement.views += 1;

            // Update engagement rate
            let total_engagement = post.engagement.likes + post.engagement.comments + post.engagement.shares;
            post.engagement.engagement_rate = (total_engagement as f64 / post.engagement.views as f64) * 100.0;

            // Update author stats
            if let Some(author) = self.user_profiles.get_mut(&post.author) {
                author.social_stats.likes_received += 1;
            }

            true
        } else {
            false
        }
    }

    // Tip a post
    pub fn tip_post(&mut self, post_id: String, tipper_id: String, amount: u64) -> bool {
        if let Some(post) = self.social_posts.get_mut(&post_id) {
            let success = self.platform.tip_post(post, tipper_id, amount);

            if success {
                // Update financial analytics
                self.analytics.financial_analytics.daily_volume += amount;

                // Update author earnings
                if let Some(author) = self.user_profiles.get_mut(&post.author) {
                    author.financial_stats.total_earnings += amount;
                    author.financial_stats.creator_earnings += amount;
                }

                // Update tipper stats
                if let Some(tipper) = self.user_profiles.get_mut(&tipper_id) {
                    tipper.financial_stats.token_balance -= amount;
                }
            }

            success
        } else {
            false
        }
    }

    // Create trading group
    pub fn create_trading_group(
        &mut self,
        name: String,
        description: String,
        creator_id: String,
    ) -> String {
        let group_id = format!("group_{}_{}", name.to_lowercase().replace(" ", "_"), 
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());

        let group = TradingGroup {
            group_id: group_id.clone(),
            name,
            description,
            members: 1, // Creator is first member
            total_trades: 0,
            success_rate: 0.0,
            average_profit: 0.0,
        };

        self.social_trading.trading_groups.insert(group_id.clone(), group);
        group_id
    }

    // Create governance proposal
    pub fn create_governance_proposal(
        &mut self,
        title: String,
        description: String,
        proposer_id: String,
    ) -> String {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let proposal_id = format!("proposal_{}_{}", title.to_lowercase().replace(" ", "_"), current_time);

        let proposal = GovernanceProposal {
            proposal_id: proposal_id.clone(),
            title,
            description,
            proposer: proposer_id,
            created_at: current_time,
            voting_end: current_time + (7 * 24 * 60 * 60), // 7 days voting
            votes_for: 0,
            votes_against: 0,
            status: ProposalStatus::Active,
            execution_result: None,
        };

        self.platform.token_economy.governance_system.proposals.insert(proposal_id.clone(), proposal);
        proposal_id
    }

    // Vote on proposal
    pub fn vote_on_proposal(
        &mut self,
        proposal_id: String,
        voter_id: String,
        support: bool, // true = for, false = against
        voting_power: u64,
    ) -> bool {
        if let Some(proposal) = self.platform.token_economy.governance_system.proposals.get_mut(&proposal_id) {
            if proposal.status == ProposalStatus::Active {
                if support {
                    proposal.votes_for += voting_power;
                } else {
                    proposal.votes_against += voting_power;
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    // Get platform overview
    pub fn get_platform_overview(&self) -> String {
        let total_posts = self.social_posts.len();
        let total_tips: u64 = self.social_posts.values().map(|p| p.engagement.tips).sum();
        let active_groups = self.social_trading.trading_groups.len();

        format!(
            "SocialFi Platform Overview:\n\
            Platform: {}\n\
            Total Users: {}\n\
            Total Posts: {}\n\
            Total Tips: {} VEXA\n\
            Trading Groups: {}\n\
            Active Proposals: {}\n\
            Daily Engagement: {}",
            self.platform.name,
            self.platform.user_base.total_users,
            total_posts,
            total_tips,
            active_groups,
            self.platform.token_economy.governance_system.proposals.len(),
            self.platform.user_base.daily_engagement
        )
    }

    // Update analytics
    pub fn update_analytics(&mut self) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Calculate platform engagement
        let total_interactions: u64 = self.social_posts.values()
            .map(|p| p.engagement.likes as u64 + p.engagement.comments as u64 + p.engagement.shares as u64)
            .sum();

        let total_users = self.user_profiles.len() as u64;
        self.analytics.platform_metrics.platform_engagement = 
            if total_users > 0 { total_interactions as f64 / total_users as f64 } else { 0.0 };

        // Update financial analytics
        self.analytics.financial_analytics.daily_volume = 
            self.social_posts.values().map(|p| p.engagement.tips).sum();

        println!("📊 Analytics updated: {} engagement score", self.analytics.platform_metrics.platform_engagement);
    }
}

// Default implementation
impl Default for SocialFiManager {
    fn default() -> Self {
        Self::new(
            "VexaSocial".to_string(),
            "Decentralized Social Finance Platform".to_string()
        )
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_creation() {
        let platform = SocialFiPlatform::new(
            "Test Platform".to_string(),
            "Test Description".to_string()
        );

        assert!(platform.platform_id.starts_with("socialfi_test_platform_"));
        assert!(platform.social_features.user_profiles);
        assert!(platform.financial_features.tipping);
    }

    #[test]
    fn test_user_registration() {
        let mut manager = SocialFiManager::default();
        
        let user_id = manager.register_user(
            "wallet123".to_string(),
            "testuser".to_string(),
            "Test bio".to_string(),
        );

        assert!(manager.user_profiles.contains_key(&user_id));
        assert_eq!(manager.platform.user_base.total_users, 1);
    }

    #[test]
    fn test_post_creation() {
        let mut manager = SocialFiManager::default();
        
        let user_id = manager.register_user(
            "wallet123".to_string(),
            "testuser".to_string(),
            "Bio".to_string(),
        );

        let post_id = manager.create_post(
            user_id.clone(),
            "Hello World!".to_string(),
            Vec::new(),
        );

        assert!(manager.social_posts.contains_key(&post_id));
        
        if let Some(user) = manager.user_profiles.get(&user_id) {
            assert_eq!(user.social_stats.posts, 1);
        }
    }

    #[test]
    fn test_tipping() {
        let mut manager = SocialFiManager::default();
        
        let user_id = manager.register_user("wallet1".to_string(), "user1".to_string(), "Bio".to_string());
        let post_id = manager.create_post(user_id.clone(), "Test post".to_string(), Vec::new());

        let tipper_id = manager.register_user("wallet2".to_string(), "user2".to_string(), "Bio".to_string());

        let success = manager.tip_post(post_id.clone(), tipper_id, 1000);

        assert!(success);
        
        if let Some(post) = manager.social_posts.get(&post_id) {
            assert_eq!(post.engagement.tips, 1000);
            assert!(post.monetization.is_monetized);
        }
    }
}