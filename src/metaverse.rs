// src/metaverse.rs
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

// ==================== CORE METAVERSE STRUCTURES ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaverseWorld {
    pub world_id: String,
    pub name: String,
    pub description: String,
    pub owner: String,
    pub created_at: u64,
    pub total_land_parcels: u32,
    pub available_land_parcels: u32,
    pub land_price: u64, // Price in VEXA tokens
    pub world_type: WorldType,
    pub features: Vec<WorldFeature>,
    pub active_events: Vec<VirtualEvent>,
    pub visitor_count: u32,
    pub rating: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldType {
    Gaming,
    Social,
    Educational,
    Business,
    Entertainment,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldFeature {
    VrSupport,
    ArIntegration,
    VoiceChat,
    TextChat,
    CommerceEnabled,
    NftGalleries,
    EventSpaces,
    GamingAreas,
    EducationalZones,
    BusinessCenters,
}

// ==================== VIRTUAL LAND SYSTEM ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualLand {
    pub land_id: String,
    pub world_id: String,
    pub coordinates: LandCoordinates,
    pub owner: String,
    pub purchase_price: u64,
    pub purchase_date: u64,
    pub land_type: LandType,
    pub buildings: Vec<VirtualBuilding>,
    pub decorations: Vec<Decoration>,
    pub visitors: HashSet<String>, // User addresses who visited
    pub revenue_generated: u64, // VEXA tokens earned
    pub last_updated: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandCoordinates {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LandType {
    Residential,
    Commercial,
    Entertainment,
    Educational,
    Gaming,
    Advertising,
    EventSpace,
    GallerySpace,
}

// ==================== VIRTUAL BUILDINGS ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualBuilding {
    pub building_id: String,
    pub land_id: String,
    pub building_type: BuildingType,
    pub levels: u8,
    pub capacity: u32, // Max users allowed
    pub current_occupancy: u32,
    pub rental_price: u64, // per hour in VEXA
    pub owner: String,
    pub features: Vec<BuildingFeature>,
    pub revenue: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuildingType {
    House,
    Mall,
    Theater,
    Museum,
    Gallery,
    Office,
    School,
    Casino,
    Restaurant,
    ConcertHall,
    Stadium,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuildingFeature {
    VrReady,
    StreamingEnabled,
    CommerceEnabled,
    MultiUser,
    Interactive,
    Customizable,
}

// ==================== AVATAR SYSTEM ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Avatar {
    pub avatar_id: String,
    pub owner: String,
    pub name: String,
    pub appearance: AvatarAppearance,
    pub inventory: Vec<AvatarItem>,
    pub equipped_items: Vec<String>, // item_ids
    pub experience: u32,
    pub level: u8,
    pub social_score: f32,
    pub last_active: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarAppearance {
    pub skin_color: String,
    pub hair_style: String,
    pub hair_color: String,
    pub eye_color: String,
    pub height: f32,
    pub body_type: String,
    pub clothing_style: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarItem {
    pub item_id: String,
    pub name: String,
    pub item_type: ItemType,
    pub rarity: Rarity,
    pub attributes: HashMap<String, String>,
    pub equipped: bool,
    pub purchase_price: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemType {
    Clothing,
    Weapon,
    Tool,
    Accessory,
    Vehicle,
    Pet,
    Special,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

// ==================== VIRTUAL EVENTS ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualEvent {
    pub event_id: String,
    pub world_id: String,
    pub land_id: String,
    pub host: String,
    pub title: String,
    pub description: String,
    pub event_type: EventType,
    pub start_time: u64,
    pub end_time: u64,
    pub max_attendees: u32,
    pub current_attendees: u32,
    pub ticket_price: u64, // 0 for free events
    pub total_revenue: u64,
    pub status: EventStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Concert,
    Conference,
    Exhibition,
    Party,
    Wedding,
    Meeting,
    Class,
    Tournament,
    Auction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventStatus {
    Scheduled,
    Live,
    Completed,
    Cancelled,
}

// ==================== ECONOMY SYSTEM ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaverseEconomy {
    pub total_land_sales: u64,
    pub total_event_revenue: u64,
    pub total_item_sales: u64,
    pub total_rental_income: u64,
    pub circulating_vexa_in_metaverse: u64,
    pub active_businesses: u32,
    pub daily_transactions: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandTransaction {
    pub transaction_id: String,
    pub land_id: String,
    pub from_owner: String,
    pub to_owner: String,
    pub price: u64,
    pub timestamp: u64,
    pub transaction_type: TransactionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Purchase,
    Sale,
    Rent,
    Lease,
    Auction,
}

// ==================== DECORATIONS ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decoration {
    pub decoration_id: String,
    pub land_id: String,
    pub name: String,
    pub decoration_type: DecorationType,
    pub position: Position3D,
    pub rotation: Position3D,
    pub scale: Position3D,
    pub nft_asset_id: Option<String>, // Linked NFT
    pub interactive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecorationType {
    Tree,
    Fountain,
    Statue,
    Building,
    Furniture,
    Signage,
    ArtPiece,
    InteractiveObject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// ==================== METAVERSE MANAGER ====================

#[derive(Debug, Clone)]
pub struct MetaverseManager {
    pub worlds: HashMap<String, MetaverseWorld>,
    pub virtual_lands: HashMap<String, VirtualLand>,
    pub avatars: HashMap<String, Avatar>,
    pub events: HashMap<String, VirtualEvent>,
    pub economy: MetaverseEconomy,
    pub land_transactions: Vec<LandTransaction>,
}

impl MetaverseManager {
    pub fn new() -> Self {
        Self {
            worlds: HashMap::new(),
            virtual_lands: HashMap::new(),
            avatars: HashMap::new(),
            events: HashMap::new(),
            economy: MetaverseEconomy {
                total_land_sales: 0,
                total_event_revenue: 0,
                total_item_sales: 0,
                total_rental_income: 0,
                circulating_vexa_in_metaverse: 0,
                active_businesses: 0,
                daily_transactions: 0,
            },
            land_transactions: Vec::new(),
        }
    }

    // ==================== WORLD MANAGEMENT ====================

    pub fn create_world(&mut self, name: String, description: String, owner: String, world_type: WorldType, total_land: u32, land_price: u64) -> Result<String, String> {
        let world_id = format!("world_{}", Self::generate_id());
        
        let world = MetaverseWorld {
            world_id: world_id.clone(),
            name,
            description,
            owner,
            created_at: Self::current_timestamp(),
            total_land_parcels: total_land,
            available_land_parcels: total_land,
            land_price,
            world_type,
            features: Vec::new(),
            active_events: Vec::new(),
            visitor_count: 0,
            rating: 0.0,
        };

        self.worlds.insert(world_id.clone(), world);
        Ok(world_id)
    }

    pub fn add_world_feature(&mut self, world_id: &str, feature: WorldFeature) -> Result<(), String> {
        if let Some(world) = self.worlds.get_mut(world_id) {
            if !world.features.contains(&feature) {
                world.features.push(feature);
                Ok(())
            } else {
                Err("Feature already exists".to_string())
            }
        } else {
            Err("World not found".to_string())
        }
    }

    // ==================== LAND MANAGEMENT ====================

    pub fn purchase_land(&mut self, world_id: &str, coordinates: LandCoordinates, buyer: String, land_type: LandType) -> Result<String, String> {
        // Check if world exists and has available land
        let world = self.worlds.get_mut(world_id).ok_or("World not found")?;
        
        if world.available_land_parcels == 0 {
            return Err("No land available in this world".to_string());
        }

        // Check if coordinates are available
        let land_id = format!("land_{}_{}_{}_{}", coordinates.x, coordinates.y, coordinates.z, world_id);
        if self.virtual_lands.contains_key(&land_id) {
            return Err("Land already occupied".to_string());
        }

        // Create land parcel
        let land = VirtualLand {
            land_id: land_id.clone(),
            world_id: world_id.to_string(),
            coordinates,
            owner: buyer.clone(),
            purchase_price: world.land_price,
            purchase_date: Self::current_timestamp(),
            land_type,
            buildings: Vec::new(),
            decorations: Vec::new(),
            visitors: HashSet::new(),
            revenue_generated: 0,
            last_updated: Self::current_timestamp(),
        };

        // Update world and economy
        world.available_land_parcels -= 1;
        self.economy.total_land_sales += world.land_price;
        self.economy.circulating_vexa_in_metaverse += world.land_price;
        self.economy.daily_transactions += 1;

        self.virtual_lands.insert(land_id.clone(), land);

        // Record transaction
        let transaction = LandTransaction {
            transaction_id: format!("tx_{}", Self::generate_id()),
            land_id: land_id.clone(),
            from_owner: world.owner.clone(), // World owner sells the land
            to_owner: buyer,
            price: world.land_price,
            timestamp: Self::current_timestamp(),
            transaction_type: TransactionType::Purchase,
        };
        self.land_transactions.push(transaction);

        Ok(land_id)
    }

    pub fn build_on_land(&mut self, land_id: &str, building_type: BuildingType, owner: String) -> Result<String, String> {
        let land = self.virtual_lands.get_mut(land_id).ok_or("Land not found")?;
        
        let building_id = format!("building_{}", Self::generate_id());
        let building = VirtualBuilding {
            building_id: building_id.clone(),
            land_id: land_id.to_string(),
            building_type,
            levels: 1,
            capacity: match building_type {
                BuildingType::House => 10,
                BuildingType::Mall => 1000,
                BuildingType::Theater => 500,
                BuildingType::Stadium => 10000,
                _ => 100,
            },
            current_occupancy: 0,
            rental_price: 0, // Can be set later
            owner,
            features: Vec::new(),
            revenue: 0,
        };

        land.buildings.push(building);
        land.last_updated = Self::current_timestamp();

        Ok(building_id)
    }

    // ==================== AVATAR MANAGEMENT ====================

    pub fn create_avatar(&mut self, owner: String, name: String, appearance: AvatarAppearance) -> Result<String, String> {
        let avatar_id = format!("avatar_{}", Self::generate_id());
        
        let avatar = Avatar {
            avatar_id: avatar_id.clone(),
            owner,
            name,
            appearance,
            inventory: Vec::new(),
            equipped_items: Vec::new(),
            experience: 0,
            level: 1,
            social_score: 0.0,
            last_active: Self::current_timestamp(),
        };

        self.avatars.insert(avatar_id.clone(), avatar);
        Ok(avatar_id)
    }

    pub fn purchase_avatar_item(&mut self, avatar_id: &str, item: AvatarItem) -> Result<(), String> {
        let avatar = self.avatars.get_mut(avatar_id).ok_or("Avatar not found")?;
        
        // Check if already owns the item
        if avatar.inventory.iter().any(|i| i.item_id == item.item_id) {
            return Err("Item already owned".to_string());
        }

        avatar.inventory.push(item);
        self.economy.total_item_sales += item.purchase_price;
        self.economy.circulating_vexa_in_metaverse += item.purchase_price;
        self.economy.daily_transactions += 1;

        Ok(())
    }

    // ==================== EVENT MANAGEMENT ====================

    pub fn create_event(&mut self, world_id: &str, land_id: &str, host: String, title: String, description: String, event_type: EventType, start_time: u64, end_time: u64, max_attendees: u32, ticket_price: u64) -> Result<String, String> {
        // Verify land ownership
        let land = self.virtual_lands.get(land_id).ok_or("Land not found")?;
        if land.owner != host {
            return Err("Only land owner can host events".to_string());
        }

        let event_id = format!("event_{}", Self::generate_id());
        let event = VirtualEvent {
            event_id: event_id.clone(),
            world_id: world_id.to_string(),
            land_id: land_id.to_string(),
            host,
            title,
            description,
            event_type,
            start_time,
            end_time,
            max_attendees,
            current_attendees: 0,
            ticket_price,
            total_revenue: 0,
            status: EventStatus::Scheduled,
        };

        self.events.insert(event_id.clone(), event);

        // Add to world's active events
        if let Some(world) = self.worlds.get_mut(world_id) {
            world.active_events.push(VirtualEvent {
                event_id: event_id.clone(),
                world_id: world_id.to_string(),
                land_id: land_id.to_string(),
                host: world.owner.clone(),
                title: title.clone(),
                description: description.clone(),
                event_type: event_type.clone(),
                start_time,
                end_time,
                max_attendees,
                current_attendees: 0,
                ticket_price,
                total_revenue: 0,
                status: EventStatus::Scheduled,
            });
        }

        Ok(event_id)
    }

    pub fn attend_event(&mut self, event_id: &str, attendee: String) -> Result<(), String> {
        let event = self.events.get_mut(event_id).ok_or("Event not found")?;
        
        if event.current_attendees >= event.max_attendees {
            return Err("Event is full".to_string());
        }

        if event.status != EventStatus::Scheduled && event.status != EventStatus::Live {
            return Err("Event not available for attendance".to_string());
        }

        event.current_attendees += 1;
        
        // Process payment if ticket price > 0
        if event.ticket_price > 0 {
            event.total_revenue += event.ticket_price;
            self.economy.total_event_revenue += event.ticket_price;
            self.economy.circulating_vexa_in_metaverse += event.ticket_price;
            
            // Add revenue to land owner (event host)
            if let Some(land) = self.virtual_lands.get_mut(&event.land_id) {
                land.revenue_generated += event.ticket_price;
            }
        }

        self.economy.daily_transactions += 1;
        Ok(())
    }

    // ==================== ECONOMY & ANALYTICS ====================

    pub fn get_metaverse_stats(&self) -> MetaverseEconomy {
        self.economy.clone()
    }

    pub fn get_user_assets(&self, user_address: &str) -> Vec<String> {
        let mut assets = Vec::new();
        
        // Find lands owned by user
        for (land_id, land) in &self.virtual_lands {
            if land.owner == user_address {
                assets.push(format!("Land: {}", land_id));
            }
        }
        
        // Find buildings owned by user
        for land in self.virtual_lands.values() {
            for building in &land.buildings {
                if building.owner == user_address {
                    assets.push(format!("Building: {} on {}", building.building_id, land.land_id));
                }
            }
        }
        
        // Find avatars owned by user
        for (avatar_id, avatar) in &self.avatars {
            if avatar.owner == user_address {
                assets.push(format!("Avatar: {}", avatar_id));
            }
        }

        assets
    }

    pub fn get_land_value_history(&self, land_id: &str) -> Vec<LandTransaction> {
        self.land_transactions
            .iter()
            .filter(|tx| tx.land_id == land_id)
            .cloned()
            .collect()
    }

    // ==================== UTILITY FUNCTIONS ====================

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

// ==================== METAVERSE NFT INTEGRATION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaverseNFT {
    pub nft_id: String,
    pub owner: String,
    pub linked_asset: LinkedAsset,
    pub position: Option<Position3D>,
    pub land_id: Option<String>,
    pub interactive: bool,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LinkedAsset {
    Artwork { image_url: String, description: String },
    Music { audio_url: String, duration: u32 },
    Video { video_url: String, duration: u32 },
    Document { content_url: String, title: String },
    Collectible { image_url: String, rarity: Rarity },
}

impl MetaverseManager {
    pub fn place_nft_in_metaverse(&mut self, nft_id: String, land_id: &str, position: Position3D, owner: String) -> Result<(), String> {
        let land = self.virtual_lands.get_mut(land_id).ok_or("Land not found")?;
        
        if land.owner != owner {
            return Err("Only land owner can place NFTs".to_string());
        }

        let decoration = Decoration {
            decoration_id: format!("nft_decoration_{}", nft_id),
            land_id: land_id.to_string(),
            name: format!("NFT Display {}", nft_id),
            decoration_type: DecorationType::ArtPiece,
            position,
            rotation: Position3D { x: 0.0, y: 0.0, z: 0.0 },
            scale: Position3D { x: 1.0, y: 1.0, z: 1.0 },
            nft_asset_id: Some(nft_id),
            interactive: true,
        };

        land.decorations.push(decoration);
        land.last_updated = Self::current_timestamp();

        Ok(())
    }
}

// ==================== METAVERSE GAMING INTEGRATION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameIntegration {
    pub game_id: String,
    pub name: String,
    pub developer: String,
    pub supported_worlds: Vec<String>,
    pub in_game_assets: Vec<GameAsset>,
    pub play_to_earn_enabled: bool,
    pub reward_pool: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameAsset {
    pub asset_id: String,
    pub name: String,
    pub asset_type: GameAssetType,
    pub rarity: Rarity,
    pub game_specific_data: HashMap<String, String>,
    pub transferable: bool,
    pub vexa_value: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameAssetType {
    Character,
    Weapon,
    Skin,
    PowerUp,
    Currency,
    Land,
    Building,
}

impl MetaverseManager {
    pub fn integrate_game(&mut self, game: GameIntegration) -> Result<(), String> {
        // Register game in supported worlds
        for world_id in &game.supported_worlds {
            if let Some(world) = self.worlds.get_mut(world_id) {
                // Add gaming feature if not already present
                if !world.features.contains(&WorldFeature::GamingAreas) {
                    world.features.push(WorldFeature::GamingAreas);
                }
            }
        }
        
        // Update economy with game integration
        self.economy.circulating_vexa_in_metaverse += game.reward_pool;
        self.economy.active_businesses += 1;

        Ok(())
    }
}

// ==================== VIRTUAL REALITY SUPPORT ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VRSupport {
    pub enabled: bool,
    pub supported_headsets: Vec<VRHeadset>,
    pub hand_tracking: bool,
    pub spatial_audio: bool,
    pub haptic_feedback: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VRHeadset {
    OculusQuest,
    HTCVive,
    ValveIndex,
    PlayStationVR,
    WindowsMixedReality,
}

impl MetaverseManager {
    pub fn enable_vr_support(&mut self, world_id: &str, vr_support: VRSupport) -> Result<(), String> {
        let world = self.worlds.get_mut(world_id).ok_or("World not found")?;
        
        if !world.features.contains(&WorldFeature::VrSupport) {
            world.features.push(WorldFeature::VrSupport);
        }

        Ok(())
    }
}