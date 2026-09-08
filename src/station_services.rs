use crate::{
    EDString,
    common_types::{
        CarrierDockingAccess, CombatRank, Credits, CrewMemberRole, DroneType, EngineerModification,
        MaterialCategory, MercCoins, ModuleEngineeringModifiers, StationIdentification,
        StationType, TechBrokerType, TraderType, VehicleType, VoucherType,
    },
    engineers::{Engineer, EngineeringBlueprint, EngineeringExperimentalEffect},
    market::MarketItemCategory,
    market_item_type::MarketItemType,
    material::AllMaterialNames,
    ship_module::{ShipModule, ShipModuleSlot},
    ship_type::ShipType,
    utils::duration_as_secs,
};
use chrono::{DateTime, Duration, Utc};
use ed_parse_log_files_macros::{Extractable, testcase, testcase_struct};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-15T19:13:36Z", "event":"BuyAmmo", "Cost":677 })]
pub struct EDLogBuyAmmo {
    pub cost: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-04-20T18:31:25Z", "event":"BuyDrones", "Type":"Drones", "Count":40, "BuyPrice":95, "TotalCost":3800 })]
pub struct EDLogBuyDrones {
    #[serde(rename = "Type")]
    pub drone_type: DroneType,
    pub count: u64,
    pub buy_price: Credits,
    pub total_cost: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CargoUpdateType {
    Collect,
    Deliver,
    WingUpdate,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-10-16T19:36:21Z", "event":"CargoDepot", "MissionID":1032909855, 
    "UpdateType":"Deliver", "CargoType":"Polymers", "Count":122, "StartMarketID":4209688067, "EndMarketID":4246173699, "ItemsCollected":122, "ItemsDelivered":122, "TotalItemsToDeliver":122, "Progress":0.000000 })]
#[testcase({ "timestamp":"2020-09-20T08:12:40Z", "event":"CargoDepot", "MissionID":632551319, "UpdateType":"WingUpdate", "StartMarketID":3223906560, "EndMarketID":3223937536, "ItemsCollected":9, 
    "ItemsDelivered":0, "TotalItemsToDeliver":9, "Progress":1.000000 })]
pub struct EDLogCargoDepot {
    #[serde(rename = "MissionID")]
    pub mission_id: u64,
    pub update_type: CargoUpdateType,
    pub cargo_type: Option<MarketItemType>,
    #[serde(rename = "CargoType_Localised")]
    pub cargo_type_localised: Option<EDString>,
    pub count: Option<u64>,
    #[serde(rename = "StartMarketID")]
    pub start_market_id: u64,
    #[serde(rename = "EndMarketID")]
    pub end_market_id: u64,
    pub items_collected: u64,
    pub items_delivered: u64,
    pub total_items_to_deliver: u64,
    // represents pending progress for goods in transit: (ItemsCollected-ItemsDelievered)/TotalItemsToDeliver
    pub progress: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CommunityGoalTierReached {
    pub tier_reached: EDString,
    pub bonus: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CommunityGoalTopTierInfo {
    pub top_rank_size: u64,
    pub player_in_top_rank: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CommunityGoalTopTier {
    pub name: EDString,
    pub bonus: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "CGID":834, "Title":"HIP 87621 Exobiology Initiative", "SystemName":"HIP 87621", "MarketName":"Exogene Sciences", "Expiry":"2025-11-27T16:00:00Z", "IsComplete":false, "CurrentTotal":3189792, "PlayerContribution":283, "NumContributors":25373, "TopTier":{ "Name":"Tier 5", "Bonus":"" }, "TopRankSize":10, "PlayerInTopRank":false, "TierReached":"Tier 3", "PlayerPercentileBand":25, "Bonus":235000000 } )]
pub struct CommunityGoal {
    #[serde(rename = "CGID")]
    pub cgid: u64,
    pub title: EDString,
    pub system_name: EDString,
    pub market_name: EDString,
    pub expiry: DateTime<Utc>,
    pub is_complete: bool,
    pub current_total: u64,
    pub player_contribution: u64,
    pub num_contributors: u64,
    pub player_percentile_band: u64,
    pub top_tier: CommunityGoalTopTier,
    #[serde(flatten)]
    pub top_tier_info: Option<CommunityGoalTopTierInfo>,
    #[serde(flatten)]
    pub tier_reached: Option<CommunityGoalTierReached>,
}

#[testcase({ "timestamp":"2024-09-20T15:50:48Z", "event":"CommunityGoal", "CurrentGoals":[ { "CGID":810, "Title":"Defend Shinrarta Dezhra Against Thargoid Invasion", "SystemName":"V886 Centauri", "MarketName":"Rescue Ship Cornwallis", "Expiry":"2024-09-26T07:00:00Z", "IsComplete":false, "CurrentTotal":864224475278, "PlayerContribution":0, "NumContributors":3199, "TopTier":{ "Name":"Tier 4", "Bonus":"" }, "TierReached":"Tier 1", "PlayerPercentileBand":100, "Bonus":10000000 } ] })]
#[testcase({ "timestamp":"2025-03-02T13:41:22Z", "event":"CommunityGoal", "CurrentGoals":[ { "CGID":813, "Title":"Brewer Corporation Trailblazer Fleet Initiative", "SystemName":"Minerva", "MarketName":"Starlace Station", "Expiry":"2025-03-06T07:00:00Z", "IsComplete":false, "CurrentTotal":65000213, "PlayerContribution":7024, "NumContributors":13124, "TopTier":{ "Name":"Tier 8", "Bonus":"" }, "TopRankSize":10, "PlayerInTopRank":false, "TierReached":"Tier 4", "PlayerPercentileBand":25, "Bonus":245000000 }, { "CGID":814, "Title":"Protect Deliveries to Minerva", "SystemName":"Minerva", "MarketName":"Starlace Station", "Expiry":"2025-03-06T07:00:00Z", "IsComplete":false, "CurrentTotal":241388732091, "PlayerContribution":66513181, "NumContributors":10704, "TopTier":{ "Name":"Tier 5", "Bonus":"" }, "TopRankSize":10, "PlayerInTopRank":false, "TierReached":"Tier 4", "PlayerPercentileBand":25, "Bonus":300000000 } ] })]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCommunityGoal {
    pub current_goals: Vec<CommunityGoal>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2024-04-08T17:50:39Z", "event":"CommunityGoalDiscard", "CGID":804, "Name":"Aid Achilles Aerospace in Researching Titan Travel Technology", "System":"Ethgreze" })]
pub struct EDLogCommunityGoalDiscard {
    #[serde(rename = "CGID")]
    pub cgid: u64,
    pub name: EDString,
    #[serde(rename = "System")]
    pub system_name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-07-30T17:14:00Z", "event":"CommunityGoalJoin", "CGID":852, "Name":"Asura Calls for Assistance to Distribute Celebratory Commodities", "System":"Asura" })]
pub struct EDLogCommunityGoalJoin {
    #[serde(rename = "CGID")]
    pub cgid: u64,
    pub name: EDString,
    #[serde(rename = "System")]
    pub system_name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-08T09:49:47Z", "event":"CommunityGoalReward", "CGID":853, "Name":"Randgnid Calls for Assistance to Distribute Celebratory Commodities", "System":"Randgnid", "Reward":13750000 })]
pub struct EDLogCommunityGoalReward {
    #[serde(rename = "CGID")]
    pub cgid: u64,
    pub name: EDString,
    #[serde(rename = "System")]
    pub system_name: EDString,
    pub reward: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCrewAssign {
    name: EDString,
    #[serde(rename = "CrewID")]
    crew_id: u64,
    role: CrewMemberRole,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2024-04-13T19:37:38Z", "event":"CrewFire", "Name":"Harleen Gilmore", "CrewID":92612996 })]
pub struct EDLogCrewFire {
    #[serde(rename = "Name")]
    name: EDString,
    #[serde(rename = "CrewID")]
    crew_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({"timestamp":"2024-01-20T16:57:41Z","event":"CrewHire","Name":"Otha Waller","CrewID":15481250,"Faction":"Federal Security Administration","Cost":55000,"CombatRank":2})]
pub struct EDLogCrewHire {
    name: EDString,
    #[serde(rename = "CrewID")]
    crew_id: u64,
    faction: EDString,
    cost: Credits,
    combat_rank: CombatRank,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum EngineerContributionType {
    Bond,
    Bounty,
    Commodity,
    Credits,
    Materials,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2024-01-20T19:22:48Z", "event":"EngineerContribution", "Engineer":"Colonel Bris Dekker", 
    "EngineerID":300140, "Type":"Bond", "Quantity":126004, "TotalQuantity":1000000 })]
#[testcase({ "timestamp":"2024-01-03T19:35:39Z", "event":"EngineerContribution", "Engineer":"The Sarge", "EngineerID":300040, 
    "Type":"Materials", "Material":"shieldpatternanalysis", "Material_Localised":"Aberrant Shield Pattern Analysis", 
    "Quantity":50, "TotalQuantity":50 })]
#[testcase({ "timestamp":"2024-01-03T14:41:59Z", "event":"EngineerContribution", "Engineer":"Zacariah Nemo", "EngineerID":300050, 
    "Type":"Commodity", "Commodity":"xihecompanions", "Commodity_Localised":"Xihe Biomorphic Companions", "Quantity":5, 
    "TotalQuantity":25 })]
#[testcase({ "timestamp":"2023-07-23T21:19:45Z", "event":"EngineerContribution", "Engineer":"Mel Brandon", "EngineerID":300280, 
    "Type":"Bounty", "Quantity":100000, "TotalQuantity":100000 })]
pub struct EDLogEngineerContribution {
    pub engineer: Engineer,
    #[serde(rename = "EngineerID")]
    pub engineer_id: u64,
    #[serde(rename = "Type")]
    pub contribution_type: EngineerContributionType,
    pub commodity: Option<MarketItemType>,
    #[serde(rename = "Commodity_Localised")]
    pub commodity_localised: Option<EDString>,
    pub material: Option<AllMaterialNames>,
    #[serde(rename = "Material_Localised")]
    pub material_localised: Option<EDString>,
    // for bond or bounty
    pub faction: Option<String>,
    // amount offered this time
    pub quantity: u64,
    // total amount now donated
    pub total_quantity: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum EngineerProgressState {
    Invited,
    Acquainted,
    Unlocked,
    Barred,
    Known,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Engineer":"Tod 'The Blaster' McQuinn", "EngineerID":300260, "Progress":"Known" })]
pub struct EngineerProgress {
    pub engineer: Option<Engineer>,
    #[serde(rename = "EngineerID")]
    pub engineer_id: Option<u64>,
    // when unlocked
    pub rank: Option<u64>,
    pub progress: EngineerProgressState,
    // when unlocked
    pub rank_progress: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2022-06-13T18:58:38Z", "event":"EngineerProgress", "Engineer":"Felicity Farseer", "EngineerID":300100, "Progress":"Invited" })]
#[testcase({ "timestamp":"2022-06-15T18:12:12Z", "event":"EngineerProgress", "Engineers":[ { "Engineer":"Tod 'The Blaster' McQuinn", "EngineerID":300260, "Progress":"Known" } ] })]
pub struct EDLogEngineerProgress {
    #[serde(flatten)]
    pub engineer: Option<EngineerProgress>,
    pub engineers: Option<Vec<EngineerProgress>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EngineerCraftIngredient {
    pub name: AllMaterialNames,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<EDString>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ExperimentalEffect {
    pub apply_experimental_effect: EngineeringExperimentalEffect,
    pub experimental_effect: EngineeringExperimentalEffect,
    #[serde(rename = "ExperimentalEffect_Localised")]
    pub experimental_effect_localised: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-07-12T17:50:29Z", "event":"EngineerCraft", "Slot":"TinyHardpoint3", "Module":"hpt_shieldbooster_size0_class5", "Ingredients":[ 
    { "Name":"conductiveceramics", "Name_Localised":"Conductive Ceramics", "Count":1 }, 
    { "Name":"refinedfocuscrystals", "Name_Localised":"Refined Focus Crystals", "Count":1 }, 
    { "Name":"imperialshielding", "Name_Localised":"Imperial Shielding", "Count":1 } ], 
    "Engineer":"Didi Vatermann", "EngineerID":300000, "BlueprintID":128673794, "BlueprintName":"ShieldBooster_Resistive", "Level":5, "Quality":0.400000, "Modifiers":[ 
        { "Label":"Integrity", "Value":42.239998, "OriginalValue":48.000000, "LessIsGood":0 }, 
        { "Label":"PowerDraw", "Value":1.500000, "OriginalValue":1.200000, "LessIsGood":1 }, 
        {"Label":"KineticResistance", "Value":15.200001, "OriginalValue":0.000000, "LessIsGood":0 }, 
        { "Label":"ThermicResistance", "Value":15.200001, "OriginalValue":0.000000, "LessIsGood":0 }, 
        { "Label":"ExplosiveResistance", "Value":15.200001, "OriginalValue":0.000000, "LessIsGood":0 } ] })]
pub struct EDLogEngineerCraft {
    pub slot: ShipModuleSlot,
    pub module: ShipModule,
    pub engineer: Option<Engineer>,
    #[serde(rename = "EngineerID")]
    pub engineer_id: u64,
    pub blueprint_name: EngineeringBlueprint,
    #[serde(rename = "BlueprintID")]
    pub blueprint_id: u64,
    pub level: u64,
    // Represents the quality or progress of the blueprint. The quality should
    // increase from 0 to 1 as the blueprint is refined through further crafting, and once it reaches a certain
    // value, the player will have the option to upgrade the blueprint to the next level of recipe
    pub quality: f64,
    // When applying an experimental effect, the ApplyExperimentalEffect property will show the name of
    // the effect applied, and the ingredient list will hold the ingredients for that effect.
    #[serde(flatten)]
    pub experimental_effect: Option<ExperimentalEffect>,
    pub ingredients: Vec<EngineerCraftIngredient>,
    pub modifiers: Vec<ModuleEngineeringModifiers>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-06-28T17:48:28Z", "event":"FetchRemoteModule", "StorageSlot":569, "StoredItem":"$hpt_heatsinklauncher_turret_tiny_name;", 
    "StoredItem_Localised":"Heatsink", "ServerId":128049519, "TransferCost":111, "TransferTime":852, "Ship":"mediumtransport01", "ShipID":53 })]
pub struct EDLogFetchRemoteModule {
    pub storage_slot: u64,
    pub stored_item: ShipModule,
    #[serde(rename = "StoredItem_Localised")]
    pub stored_item_localised: EDString,
    pub server_id: u64,
    pub transfer_cost: Credits,
    #[serde(with = "duration_as_secs")]
    pub transfer_time: Duration,
    pub ship: ShipType,
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "id":128793127, "Name":"$thargoidheart_name;", "Name_Localised":"Thargoid Heart", "Category":"$MARKET_category_salvage;", 
    "Category_Localised":"Salvage", "BuyPrice":106696, "SellPrice":105639, "MeanPrice":140275, "StockBracket":0, "DemandBracket":0, "Stock":0, 
    "Demand":0, "Consumer":false, "Producer":false, "Rare":false })]
pub struct MarketItem {
    #[serde(rename = "id")]
    id: u64,
    #[serde(rename = "Name")]
    pub market_item_name: MarketItemType,
    #[serde(rename = "Name_Localised")]
    pub market_item_name_localised: Option<EDString>,
    pub category: MarketItemCategory,
    #[serde(rename = "Category_Localised")]
    pub category_localised: Option<EDString>,
    buy_price: Credits,
    sell_price: Credits,
    mean_price: Credits,
    stock_bracket: u64,
    demand_bracket: u64,
    stock: u64,
    demand: u64,
    consumer: bool,
    producer: bool,
    rare: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMarket {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub station_name: EDString,
    pub station_type: StationType,
    pub star_system: EDString,
    pub carrier_docking_access: Option<CarrierDockingAccess>,
    pub items: Option<Vec<MarketItem>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({"Slot":"Slot06_Size3","Name":"$int_repairer_size3_class4_name;","Name_Localised":"AFM Unit","Hot":false})]
pub struct MassStoredModule {
    pub slot: ShipModuleSlot,
    pub name: ShipModule,
    #[serde(rename = "Name_Localised")]
    pub name_localised: EDString,
    pub hot: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({"timestamp":"2025-02-02T07:00:57Z","event":"MassModuleStore","MarketID":128666762,"Ship":"krait_mkii","ShipID":40,
"Items":[
    {"Slot":"Slot01_Size6","Name":"$int_guardianfsdbooster_size5_name;","Name_Localised":"Guardian FSD Booster","Hot":false},
    {"Slot":"Slot02_Size6","Name":"$int_dronecontrol_repair_size5_class2_name;","Name_Localised":"Repair","Hot":false}]})]
pub struct EDLogMassModuleStore {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub ship: ShipType,
    #[serde(rename = "ShipID")]
    pub ship_id: u32,
    pub items: Vec<MassStoredModule>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ExchangedMaterials {
    pub material: AllMaterialNames,
    #[serde(rename = "Material_Localised")]
    pub material_localised: Option<EDString>,
    pub category: MaterialCategory,
    pub quantity: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-07-23T18:54:08Z", "event":"MaterialTrade", "MarketID":3228992256, 
    "TraderType":"raw", 
    "Paid":{ "Material":"tellurium", "Category":"Raw", "Quantity":1 }, 
    "Received":{ "Material":"tungsten", "Category":"Raw", "Quantity":3 } })]
pub struct EDLogMaterialTrade {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub trader_type: TraderType,
    pub paid: ExchangedMaterials,
    pub received: ExchangedMaterials,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-02-26T17:55:23Z", "event":"MissionAbandoned", "Name":"Mission_Assassinate_name", "LocalisedName":"Assassinate Known Pirate: Gabriel Bateman", "MissionID":1046041050 })]
pub struct EDLogMissionAbandoned {
    name: EDString,
    localised_name: Option<EDString>,
    #[serde(rename = "MissionID")]
    mission_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CommodityMission {
    commodity: EDString,
    #[serde(rename = "Commodity_Localised")]
    commodify_localised: Option<EDString>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PassengerType {
    Refugee,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct PassengerMissionInformation {
    passenger_count: u64,
    #[serde(rename = "PassengerVIPs")]
    passenger_vips: bool,
    passenger_wanted: bool,
    passenger_type: PassengerType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Mission {
    #[serde(rename = "MissionID")]
    mission_id: u64,
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    passenger_mission: bool,
    // time left in seconds
    #[serde(with = "duration_as_secs")]
    expires: Duration,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct MissionTarget {
    target: EDString,
    #[serde(rename = "Target_Localised")]
    target_localised: EDString,
    target_type: EDString,
    #[serde(rename = "TargetType_Localised")]
    target_type_localised: EDString,
    target_faction: EDString,
    kill_count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Impact {
    None,
    #[serde(alias = "+")]
    Low,
    #[serde(alias = "++")]
    Med,
    #[serde(alias = "+++")]
    High,
    #[serde(alias = "++++")]
    VeryHigh,
    #[serde(alias = "+++++")]
    SuperHigh,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct MissionDestination {
    destination_system: EDString,
    new_destination_system: Option<EDString>,
    destination_station: EDString,
    new_destination_station: Option<EDString>,
    destination_settlement: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-10-17T01:47:41Z", "event":"MissionAccepted", 
    "Faction":"Independent Murung Values Party", "Name":"Mission_Delivery_Boom", 
    "LocalisedName":"Boom time delivery of 6 units of Biowaste", "Commodity":"$Biowaste_Name;", 
    "Commodity_Localised":"Biowaste", "Count":6, "TargetFaction":"Belu Silver Federal Industry", 
    "DestinationSystem":"57 Zeta Serpentis", "DestinationStation":"Musabayev Dock", 
    "Expiry":"2017-10-18T01:45:04Z", "Influence":"Low", "Reputation":"Low", 
    "Reward":157815, "MissionID":228681523 })]
#[testcase({ "timestamp":"2022-08-26T20:48:49Z", "event":"MissionAccepted", "Faction":"Sirius Corporation", "Name":"MISSION_genericPermit1", "LocalisedName":"Permit Acquisition Opportunity", "Wing":false, "Influence":"None", "Reputation":"None", "MissionID":885648399 })]
pub struct EDLogMissionAccepted {
    name: EDString,
    localised_name: EDString,
    faction: EDString,
    #[serde(rename = "MissionID")]
    mission_id: u64,
    influence: Impact,
    reputation: Impact,
    reward: Option<u64>,
    wing: Option<bool>,
    #[serde(flatten)]
    mission_target: Option<MissionTarget>,
    #[serde(flatten)]
    mission_destination: Option<MissionDestination>,
    #[serde(flatten)]
    commodity: Option<CommodityMission>,
    donation: Option<EDString>,
    donated: Option<Credits>,
    expiry: Option<EDString>,
    #[serde(flatten)]
    passenger_mission_info: Option<PassengerMissionInformation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Name":"ModularTerminals", "Name_Localised":"Modular Terminals", "Count":4 })]
pub struct CommodityReward {
    // TODO: covert into enum. Are these OnFootItems or AllMaterialNames?
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct( {"Name": "BiotechConductors", "Name_Localised": "Biotech Conductors", 
    "Category": "$MICRORESOURCE_CATEGORY_Manufactured;","Category_Localised": "Manufactured", 
    "Count": 2})]
#[testcase_struct({ "Name":"WeaponSchematic", "Name_Localised":"Weapon Schematic", 
    "Category":"$MICRORESOURCE_CATEGORY_Item;", "Category_Localised":"Item", "Count":4 } )]
#[testcase_struct({"Name": "BloodTestResults","Name_Localised": "Blood Test Results",
    "Category": "$MICRORESOURCE_CATEGORY_Data;","Category_Localised": "Data","Count": 9})]
pub struct MaterialReward {
    // TODO: look into enum for this. It seems that on foot mats and other mats are mixed here
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    // TODO: look into enum for this. Is seems that there are more categories than
    // in MaterialCategory, mostly because the mats are mixed with on foot items
    category: EDString,
    #[serde(rename = "Category_Localised")]
    category_localised: EDString,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Effect":"$MISSIONUTIL_Interaction_Summary_EP_up;", "Effect_Localised":"The economic status of $#MinorFaction; has improved in the $#System; system.", "Trend":"UpGood" })]
pub struct Effect {
    effect: EDString,
    #[serde(rename = "Effect_Localised")]
    effect_localised: EDString,
    trend: TrendValue,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "SystemAddress":4756441502482, "Trend":"UpGood", "Influence":"++" })]
pub struct Influence {
    system_address: u64,
    trend: TrendValue,
    influence: Impact,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TrendValue {
    None,
    UpGood,
    UpBad,
    DownGood,
    DownBad,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({"Faction":"Kamadhenu Citizens' Forum", "Effects":[ 
    { "Effect":"$MISSIONUTIL_Interaction_Summary_EP_up;", "Effect_Localised":"The economic status of $#MinorFaction; has improved in the $#System; system.", "Trend":"UpGood" } ], 
    "Influence":[ { "SystemAddress":4756441502482, "Trend":"UpGood", "Influence":"++" } ], 
    "ReputationTrend":"UpGood", "Reputation":"++" })]
pub struct FactionEffect {
    faction: EDString,
    effects: Vec<Effect>,
    influence: Vec<Influence>,
    reputation_trend: TrendValue,
    reputation: Impact,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-12-15T09:36:49Z", "event":"MissionCompleted", "Faction":"CdE Corporation", "Name":"Mission_Sightseeing_name", 
    "LocalisedName":"Christina Wood Seeks Sightseeing Adventure", "MissionID":12341234, "Commodity":"$ConsumerTechnology_Name;", 
    "Commodity_Localised":"Consumer Technology", "Count":1, "DestinationSystem":"Sothis", "Reward":1818345, 
    "FactionEffects":[
        {"Faction":"CdE Corporation", "Effects":[ { "Effect":"$MISSIONUTIL_Interaction_Summary_SP_up;", "Effect_Localised":"The security status of $#MinorFaction; has improved in the $#System; system.", "Trend":"UpGood" } ], 
         "Influence":[ { "SystemAddress":9463020987689, "Trend":"UpGood", "Influence":"++" } ], 
         "ReputationTrend":"UpGood", "Reputation":"++"}
    ]})]
#[testcase({ "timestamp":"2025-11-11T19:10:37Z", "event":"MissionCompleted", "Faction":"Kamadhenu Citizens' Forum", 
    "Name":"Mission_AltruismCredits_name", "LocalisedName":"Donate 1,000,000 Cr to the cause", 
    "MissionID":12341234, "Donation":"1000000", "Donated":1000000, "FactionEffects":[ { 
        "Faction":"Kamadhenu Citizens' Forum", "Effects":[ { "Effect":"$MISSIONUTIL_Interaction_Summary_EP_up;", "Effect_Localised":"The economic status of $#MinorFaction; has improved in the $#System; system.", "Trend":"UpGood" } ], 
        "Influence":[ { "SystemAddress":4756441502482, "Trend":"UpGood", "Influence":"++" } ], 
        "ReputationTrend":"UpGood", "Reputation":"++" } 
    ] })]
#[testcase({ "timestamp":"2022-09-14T17:26:59Z", "event":"MissionCompleted", "Faction":"Brazilian Armada X", 
    "Name":"Mission_Courier_Expansion_name", "MissionID":889316420, "TargetFaction":"East India Company", 
    "DestinationSystem":"Yamahun", "DestinationStation":"Hurston Platform", "Reward":30147, 
    "CommodityReward":[ { "Name":"ModularTerminals", "Name_Localised":"Modular Terminals", "Count":4 } ], 
    "FactionEffects":[ 
        { "Faction":"Brazilian Armada X", "Effects":[ { "Effect":"$MISSIONUTIL_Interaction_Summary_EP_up;", "Effect_Localised":"The economic status of $#MinorFaction; has improved in the $#System; system.", "Trend":"UpGood" } ], "Influence":[ { "SystemAddress":2007997846218, "Trend":"UpGood", "Influence":"+" } ], "ReputationTrend":"UpGood", "Reputation":"+" }, 
        { "Faction":"East India Company", "Effects":[ { "Effect":"$MISSIONUTIL_Interaction_Summary_EP_up;", "Effect_Localised":"The economic status of $#MinorFaction; has improved in the $#System; system.", "Trend":"UpGood" } ], "Influence":[ { "SystemAddress":22958210566936, "Trend":"UpGood", "Influence":"+" } ], "ReputationTrend":"UpGood", "Reputation":"+" } ]})]
pub struct EDLogMissionCompleted {
    pub name: EDString,
    pub localised_name: Option<EDString>,
    pub faction: EDString,
    #[serde(rename = "MissionID")]
    pub mission_id: u64,
    // TODO: figure out how to parse. Looks like OnFootItems and AllMaterialNames are used here
    pub commodity: Option<EDString>,
    #[serde(rename = "Commodity_Localised")]
    pub commodity_localised: Option<EDString>,
    pub count: Option<u64>,
    #[serde(flatten)]
    pub mission_target: Option<MissionTarget>,
    #[serde(flatten)]
    pub mission_destination: Option<MissionDestination>,
    pub reward: Option<Credits>,
    pub donation: Option<EDString>,
    pub donated: Option<Credits>,
    pub permits_reward: Option<Vec<EDString>>,
    pub commodity_reward: Option<Vec<CommodityReward>>,
    pub materials_reward: Option<Vec<MaterialReward>>,
    pub faction_effects: Option<Vec<FactionEffect>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-10-02T17:39:08Z", "event":"MissionFailed", "Name":"Mission_Assassinate_name", "LocalisedName":"Assassinate Known Pirate: Rich Davey", "MissionID":1030832826 })]
pub struct EDLogMissionFailed {
    name: EDString,
    localised_name: Option<EDString>,
    #[serde(rename = "MissionID")]
    mission_id: u64,
    fine: Option<Credits>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-04-13T08:45:18Z", "event":"MissionRedirected", "MissionID":1051316259, "Name":"Mission_Assassinate", "LocalisedName":"Assassinate Known Pirate: Yohan", "NewDestinationStation":"Shaw Station", "NewDestinationSystem":"Metzili", "OldDestinationStation":"", "OldDestinationSystem":"Tascheter Sector WZ-P a5-0" })]
pub struct EDLogMissionRedirected {
    #[serde(rename = "MissionID")]
    mission_id: u64,
    name: EDString,
    localised_name: Option<EDString>,
    #[serde(rename = "LocalisedName_Localised")]
    localised_name_localised: Option<EDString>,
    new_destination_station: EDString,
    new_destination_system: EDString,
    old_destination_station: EDString,
    old_destination_system: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({"StoredItem":"$int_sensors_size3_class1_name;", "StoredItem_Localised":"Sensors"})]
pub struct StoreItem {
    pub stored_item: ShipModule,
    #[serde(rename = "StoredItem_Localised")]
    pub stored_item_localised: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({"SellItem":"$int_sensors_size4_class1_name;", "SellItem_Localised":"Sensors"})]
pub struct SellItem {
    pub sell_item: ShipModule,
    #[serde(rename = "SellItem_Localised")]
    pub sell_item_localised: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2022-09-19T19:35:50Z", "event":"ModuleBuy", "Slot":"Radar", 
    "SellItem":"$int_sensors_size4_class1_name;", "SellItem_Localised":"Sensors", "SellPrice":9646, 
    "BuyItem":"$int_sensors_size4_class5_name;", "BuyItem_Localised":"Sensors", "MarketID":3223365120,
    "BuyPrice":376829, "Ship":"ferdelance", "ShipID":6 })]
#[testcase({ "timestamp":"2026-07-30T17:48:01Z", "event":"ModuleBuy", "Slot":"Radar", 
    "StoredItem":"$int_sensors_size3_class1_name;", "StoredItem_Localised":"Sensors", "BuyItem":"$int_sensors_size3_class5_name;", 
    "BuyItem_Localised":"Sensors", "MarketID":128667761, "BuyPrice":154373, "Ship":"cobramkv", "ShipID":57 })]
pub struct EDLogModuleBuy {
    pub slot: ShipModuleSlot,
    #[serde(flatten)]
    pub store_item: Option<StoreItem>,
    pub buy_item: ShipModule,
    #[serde(rename = "BuyItem_Localised")]
    pub buy_item_localised: EDString,
    #[serde(flatten)]
    pub sell_item: Option<SellItem>,
    pub sell_price: Option<Credits>,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub buy_price: Credits,
    pub buy_merc_coins_price: Option<MercCoins>,
    pub ship: ShipType,
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({"SwapOutItem":"$int_repairer_size1_class5_name;", "SwapOutItem_Localised":"AFM Unit"})]
pub struct SwapOutItem {
    pub swap_out_item: ShipModule,
    #[serde(rename = "SwapOutItem_Localised")]
    pub swap_out_item_localised: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-07-13T13:39:29Z", "event":"ModuleRetrieve", "MarketID":128674183, "Slot":"Slot04_Size5", 
    "RetrievedItem":"$int_cargorack_size5_class1_name;", "RetrievedItem_Localised":"Cargo Rack", "Ship":"lakonminer", "ShipID":50, "Hot":false, 
    "EngineerModifications":"CargoRack_IncreasedCapacity", "Level":5, "Quality":1.000000 })]
#[testcase({ "timestamp":"2025-11-23T14:16:42Z", "event":"ModuleRetrieve", "MarketID":3706278912, "Slot":"Slot05_Size1", 
    "RetrievedItem":"$int_guardianmodulereinforcement_size1_class2_name;", "RetrievedItem_Localised":"Guardian Module Reinforcement", 
    "Ship":"python_nx", "ShipID":33, "Hot":false, "SwapOutItem":"$int_repairer_size1_class5_name;", 
    "SwapOutItem_Localised":"AFM Unit" })]
pub struct EDLogModuleRetrieve {
    #[serde(rename = "MarketID")]
    pub market_id: Option<u64>,
    pub slot: ShipModuleSlot,
    pub ship: EDString,
    #[serde(rename = "ShipID")]
    pub ship_id: u32,
    pub retrieved_item: ShipModule,
    #[serde(rename = "RetrievedItem_Localised")]
    pub retrieved_item_localised: EDString,
    pub hot: Option<bool>,
    #[serde(flatten)]
    pub engineer_modification: Option<EngineerModification>,
    // if slot was not empty
    #[serde(flatten)]
    pub swap_out_item: Option<SwapOutItem>,
    pub cost: Option<Credits>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-07-30T17:48:40Z", "event":"ModuleSell", "MarketID":128667761, 
    "Slot":"Slot09_Size1", "SellItem":"$int_supercruiseassist_name;", "SellItem_Localised":"Supercruise Assist", 
    "SellPrice":8892, "Ship":"cobramkv", "ShipID":57 })]
pub struct EDLogModuleSell {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub slot: ShipModuleSlot,
    pub sell_item: ShipModule,
    #[serde(rename = "SellItem_Localised")]
    pub sell_item_localised: EDString,
    pub sell_price: Credits,
    pub ship: EDString,
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-07-30T17:46:26Z", "event":"ModuleSellRemote", "StorageSlot":501, 
    "SellItem":"$int_powerplant_size2_class1_name;", "SellItem_Localised":"Power Plant", "ServerId":128064033, 
    "SellPrice":1977, "Ship":"cobramkv", "ShipID":57 })]
pub struct EDLogModuleSellRemote {
    pub storage_slot: u64,
    pub sell_item: ShipModule,
    #[serde(rename = "SellItem_Localised")]
    pub sell_item_localised: EDString,
    pub server_id: u64,
    pub sell_price: Credits,
    pub ship: EDString,
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-10-17T02:57:13Z", "event":"ModuleStore", 
    "Slot":"Slot06_Size2", "StoredItem":"$int_repairer_size2_class3_name;", 
    "StoredItem_Localised":"AFM Unit", "Ship":"cobramkiii", "ShipID":1 })]
pub struct EDLogModuleStore {
    #[serde(rename = "MarketID")]
    pub market_id: Option<u64>,
    pub slot: ShipModuleSlot,
    pub stored_item: ShipModule,
    #[serde(rename = "StoredItem_Localised")]
    pub stored_item_localised: EDString,
    pub ship: ShipType,
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
    pub hot: Option<bool>,
    #[serde(flatten)]
    pub engineer_modification: Option<EngineerModification>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-01-07T20:04:18Z", "event":"ModuleSwap", 
    "MarketID":3706278912, "FromSlot":"MediumHardpoint1", "ToSlot":"LargeHardpoint1", 
    "FromItem":"$hpt_flakmortar_turret_medium_name;", 
    "FromItem_Localised":"Remote Flak", "ToItem":"Null", "Ship":"explorer_nx", 
    "ShipID":46 })]
pub struct EDLogModuleSwap {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub from_slot: ShipModuleSlot,
    pub to_slot: ShipModuleSlot,
    pub from_item: ShipModule,
    #[serde(rename = "FromItem_Localised")]
    pub from_item_localised: EDString,
    pub to_item: ShipModule,
    #[serde(rename = "ToItem_Localised")]
    pub to_item_localised: Option<EDString>,
    pub ship: ShipType,
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({"id": 129045436, "Name": "hpt_basicmissilerack_fixed_medium","BuyPrice": 0, "BuyMercCoinsPrice": 800 })]
#[testcase_struct({"id": 128049431, "Name": "hpt_beamlaser_fixed_huge","BuyPrice": 2336256 })]
#[testcase_struct({ "id":129040538, "Name":"hpt_mkiiplasmashockautocannon_fixed_large", "BuyPrice":4497354 })]
pub struct ModuleOutfitting {
    #[serde(rename = "id")]
    id: u64,
    name: ShipModule,
    buy_price: Credits,
    buy_merc_coins_price: Option<MercCoins>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2022-09-17T13:01:22Z", "event":"Outfitting", "MarketID":3223506432, "StationName":"Coleman Ring", "StarSystem":"BZ Ceti" })]
pub struct EDLogOutfitting {
    #[serde(flatten)]
    pub station_identification: StationIdentification,
    star_system: EDString,
    horizons: Option<bool>,
    items: Option<Vec<ModuleOutfitting>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-01-30T18:59:33Z", "event":"PayBounties", "Amount":125, "AllFines":true, "ShipID":39, "BrokerPercentage":25.000000 })]
pub struct EDLogPayBounties {
    pub amount: Credits,
    pub all_fines: Option<bool>,
    pub faction: Option<EDString>,
    #[serde(rename = "Faction_Localised")]
    pub faction_localised: Option<EDString>,
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
    pub broker_percentage: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-05-03T14:30:00Z", "event":"PayFines", "Amount":100, "AllFines":true, "ShipID":39 })]
pub struct EDLogPayFines {
    pub amount: Credits,
    pub all_fines: bool,
    pub faction: Option<EDString>,
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
    pub broker_percentage: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct VoucherFaction {
    faction: EDString,
    amount: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-06-28T21:45:34Z", "event":"RedeemVoucher", "Type":"trade", "Amount":14229 })]
#[testcase({ "timestamp":"2025-11-09T20:07:12Z", "event":"RedeemVoucher", "Type":"bounty", "Amount":1500, 
    "Factions":[ { "Faction":"", "Amount":1500 } ], "BrokerPercentage":25.000000 })]
pub struct EDLogRedeemVoucher {
    #[serde(rename = "Type")]
    pub voucher_type: VoucherType,
    // Net amount received, after any broker fee
    pub amount: Credits,
    pub faction: Option<EDString>,
    pub factions: Option<Vec<VoucherFaction>>,
    pub broker_percentage: Option<f32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-11-06T18:58:11Z", "event":"RefuelAll", "Cost":60, "Amount":1.187222 })]
pub struct EDLogRefuelAll {
    pub cost: Credits,
    pub amount: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-11-06T18:58:11Z", "event":"RefuelPartial", "Cost":60, "Amount":1.187222 })]
pub struct EDLogRefuelPartial {
    pub cost: Credits,
    pub amount: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2024-10-15T18:34:25Z", "event":"Repair", "Items":[ "$python_nx_cockpit_name;", "Hull", "$modularcargobaydoor_name;", "Wear" ], "Cost":811 })]
pub struct EDLogRepair {
    // all, wear, hull, paint or name of module. TODO: split off module
    pub item: Option<ShipModule>,
    pub items: Option<Vec<ShipModule>>,
    pub cost: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-11-06T18:58:12Z", "event":"RepairAll", "Cost":22513 })]
pub struct EDLogRepairAll {
    pub cost: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2023-07-10T17:53:22Z", "event":"RestockVehicle", "Type":"independent_fighter", 
    "Type_Localised":"Taipan", "Loadout":"three", "ID":22, "Cost":15270, "Count":1 })]
#[testcase({"timestamp":"2025-02-08T07:42:44Z","event":"RestockVehicle","Type":"gdn_hybrid_fighter_v3",
    "Loadout":"one","Cost":4120,"Count":4})]
#[testcase({"timestamp":"2025-02-02T06:50:33Z","event":"RestockVehicle","Type":"gdn_hybrid_fighter_v2",
    "Type_Localised":"Javelin","Loadout":"one","ID":42,"Cost":13400,"Count":1})]
pub struct EDLogRestockVehicle {
    #[serde(rename = "ID")]
    pub id: Option<u64>,
    #[serde(rename = "Type")]
    pub vehicle_type: VehicleType,
    #[serde(rename = "Type_Localised")]
    pub vehicle_type_localised: Option<EDString>,
    pub loadout: EDString,
    pub cost: Credits,
    pub count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-04-04T18:51:10Z", "event":"ScientificResearch", "MarketID":129038712, 
    "Name":"nm_seed", "Name_Localised":"Unica Seed", "Category":"Item", "Count":384 })]
pub struct EDLogScientificResearch {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    // TODO: make enum. Could be onfoot and materials. Make use of Category?
    pub name: MarketItemType,
    #[serde(rename = "Name_Localised")]
    pub name_localised: EDString,
    pub category: EDString,
    pub count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-03-23T13:38:26Z", "event":"SearchAndRescue", "MarketID":3227740928, 
    "Name":"usscargoblackbox", "Name_Localised":"Black Box", "Count":3, "Reward":92316 })]
pub struct EDLogSearchAndRescue {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub name: MarketItemType,
    #[serde(rename = "Name_Localised")]
    pub name_localised: EDString,
    pub count: u64,
    pub reward: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-04-21T18:27:10Z", "event":"SellDrones", "Type":"Drones", "Count":8, "SellPrice":100, "TotalSale":800 })]
pub struct EDLogSellDrones {
    #[serde(rename = "Type")]
    pub drone_type: DroneType,
    pub count: u64,
    pub sell_price: Credits,
    pub total_sale: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-06-20T21:24:02Z", "event":"SellShipOnRebuy", "ShipType":"mamba", "System":"14 Ceti", "SellShipId":3, "ShipPrice":73796265 })]
pub struct EDLogSellShipOnRebuy {
    pub ship_type: ShipType,
    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localised: Option<EDString>,
    pub system: EDString,
    pub sell_ship_id: u64,
    pub ship_price: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-07-30T18:02:48Z", "event":"SetUserShipName", "Ship":"cobramkv", "ShipID":57, "UserShipName":"MyFirstCobraMkV", "UserShipId":"AABBCC" })]
pub struct EDLogSetUserShipName {
    ship: ShipType,
    #[serde(rename = "ShipID")]
    ship_id: u64,
    user_ship_name: EDString,
    user_ship_id: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "id":0, "ShipType":"explorer_nx", "ShipType_Localised":"Caspian Explorer", "ShipPrice":5026833 })]
pub struct ShipyardPriceListItem {
    #[serde(rename = "id")]
    pub id: u64,
    pub ship_type: ShipType,
    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localised: Option<EDString>,
    pub ship_price: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-20T19:19:17Z", "event":"Shipyard", "MarketID":128667761, "StationName":"Jaques Station", 
    "StarSystem":"Colonia", "Horizons":true, "AllowCobraMkIV":false, "PriceList":[ 
        { "id":0, "ShipType":"explorer_nx", "ShipType_Localised":"Caspian Explorer", "ShipPrice":5026833 }, 
        { "id":0, "ShipType":"cobramkv", "ShipType_Localised":"Cobra Mk V", "ShipPrice":9840482 }]})]
pub struct EDLogShipyard {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub station_name: EDString,
    pub star_system: EDString,
    pub horizons: Option<bool>,
    #[serde(rename = "AllowCobraMkIV")]
    pub allow_cobra_mk_iv: Option<bool>,
    pub price_list: Option<Vec<ShipyardPriceListItem>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ShipToStore {
    pub store_old_ship: ShipType,
    #[serde(rename = "StoreShipID")]
    pub store_ship_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SoldShip {
    pub sell_old_ship: ShipType,
    pub sell_ship_id: u64,
    pub sell_price: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-01-27T20:25:43Z", "event":"ShipyardBuy", "ShipType":"lakonminer", "ShipType_Localised":"Type-11 Prospector", "ShipPrice":56240509, "StoreOldShip":"Mandalay", "StoreShipID":35, "MarketID":4273315075 })]
pub struct EDLogShipyardBuy {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub ship_type: ShipType,
    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localised: Option<EDString>,
    pub ship_price: Credits,
    #[serde(flatten)]
    pub store: Option<ShipToStore>,
    #[serde(flatten)]
    pub sold: Option<SoldShip>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-01-27T20:25:44Z", "event":"ShipyardNew", "ShipType":"lakonminer", "ShipType_Localised":"Type-11 Prospector", "NewShipID":50 })]
pub struct EDLogShipyardNew {
    pub ship_type: ShipType,
    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localised: Option<EDString>,
    #[serde(rename = "NewShipID")]
    pub new_ship_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2022-08-01T11:32:25Z", "event":"ShipyardSell", "ShipType":"adder", "SellShipID":2, "ShipPrice":116357, "MarketID":3226459904 })]
pub struct EDLogShipyardSell {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    #[serde(rename = "ShipMarketID")]
    pub ship_market_id: Option<u64>,
    pub ship_type: ShipType,
    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localised: Option<EDString>,
    #[serde(rename = "SellShipID")]
    pub sell_ship_id: u64,
    pub ship_price: Credits,
    pub system: Option<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2024-11-10T19:50:26Z", "event":"ShipyardTransfer", "ShipType":"Mandalay", "ShipID":37, "System":"LHS 20", "ShipMarketID":3223415296, "Distance":160.883530, "TransferPrice":476645, "TransferTime":1908, "MarketID":3706278912 })]
pub struct EDLogShipyardTransfer {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub ship_type: ShipType,
    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localised: Option<EDString>,
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
    pub system: Option<EDString>,
    #[serde(rename = "ShipMarketID")]
    pub ship_market_id: u64,
    pub distance: f64,
    pub transfer_price: Credits,
    pub transfer_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-05-14T11:35:27Z", "event":"ShipyardSwap", "ShipType":"panthermkii", "ShipType_Localised":"Panther Clipper Mk II", "ShipID":43, "StoreOldShip":"MediumTransport01", "StoreShipID":53, "MarketID":3706278912 })]
pub struct EDLogShipyardSwap {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub ship_type: ShipType,
    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localised: Option<EDString>,
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
    #[serde(flatten)]
    old_ship: Option<ShipToStore>,
    #[serde(flatten)]
    sold_ship: Option<SoldShip>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({"StarSystem": "Manah","MarketID": 3706278912,"TransferCost": 100,"TransferTime": 220714})]
pub struct TransferDetails {
    #[serde(rename = "MarketID")]
    market_id: u64,
    pub star_system: EDString,
    pub transfer_cost: Credits,
    pub transfer_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({"Name": "$int_shieldgenerator_size8_class5_name;","Name_Localised": "Shield Generator",
"StorageSlot": 11,"StarSystem": "Manah","MarketID": 3706278912,"TransferCost": 215051310,"TransferTime": 220714,
"BuyPrice": 162586486,"Hot": false,"EngineerModifications": "ShieldGenerator_Thermic","Level": 5,"Quality": 1.000000})]
pub struct StoredModule {
    pub name: ShipModule,
    #[serde(rename = "Name_Localised")]
    pub name_localised: EDString,
    pub storage_slot: u64,
    // available when not in_transit
    #[serde(flatten)]
    transfer_details: Option<TransferDetails>,
    pub hot: bool,
    #[serde(flatten)]
    pub engineer_modification: Option<EngineerModification>,
    pub in_transit: Option<bool>,
    pub buy_price: Credits,
    pub buy_merc_coins_price: Option<MercCoins>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({"timestamp": "2026-08-20T18:16:25Z","event": "StoredModules",
    "MarketID": 128667761,"StationName": "Jaques Station","StarSystem": "Colonia", "Items": [
        { "Name": "$int_cargorack_size7_class1_name;", "Name_Localised": "Cargo Rack",
          "StorageSlot": 2, "StarSystem": "Manah", "MarketID": 3706278912, "TransferCost": 100,
          "TransferTime": 220714, "BuyPrice": 0,"Hot": false
        }]})]
pub struct EDLogStoredModules {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub station_name: EDString,
    pub star_system: EDString,
    pub items: Vec<StoredModule>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ShipStorageLocation {
    star_system: EDString,
    #[serde(rename = "ShipMarketID")]
    ship_market_id: u64,
    transfer_price: Credits,
    transfer_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({"ShipID": 0,"ShipType": "SideWinder","Name": "","StarSystem": "Manah","ShipMarketID": 3706278912,"TransferPrice": 43326,"TransferTime": 220714,"Value": 542263,"Hot": false})]
#[testcase_struct({"ShipID": 4,"ShipType": "Vulture","Name": "MyVulture","StarSystem": "Manah","ShipMarketID": 3706278912,"TransferPrice": 32416003,"TransferTime": 220714,"Value": 22056222,"Hot": false})]
pub struct StoredShip {
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
    pub ship_type: ShipType,
    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localised: Option<EDString>,
    pub name: Option<EDString>,
    pub in_transit: Option<bool>,
    // available when not in_transit
    #[serde(flatten)]
    pub location: Option<ShipStorageLocation>,
    pub value: Credits,
    pub hot: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({"timestamp": "2026-08-20T19:19:17Z","event": "StoredShips","StationName": "Jaques Station",
    "MarketID": 128667761,"StarSystem": "Colonia",
    "ShipsHere": [{"ShipID": 57,"ShipType": "cobramkv","ShipType_Localised": "Cobra Mk V","Name": "WhatIsInAName","Value": 9840482,"Hot": false}],
    "ShipsRemote": [{"ShipID": 0,"ShipType": "SideWinder","Name": "","StarSystem": "Manah","ShipMarketID": 3706278912,"TransferPrice": 43326,"TransferTime": 220714,"Value": 542263,"Hot": false}]})]
pub struct EDLogStoredShips {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub station_name: EDString,
    pub star_system: EDString,
    pub ships_here: Vec<StoredShip>,
    pub ships_remote: Vec<StoredShip>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Name":"Hpt_HeatSinkLauncher_Turret_Tiny", "Name_Localised":"Heatsink" })]
pub struct BrokerItemUnlocked {
    // TODO: make enum for BrokerItem, since it can be a ShipModule or a ShipLaunchedFighter
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Name":"niobium", "Count":6, "Category":"Raw" })]
pub struct BrokerMaterial {
    pub name: AllMaterialNames,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<EDString>,
    pub count: u64,
    pub category: MaterialCategory,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct BrokerCommodity {
    // TODO: make enum
    pub name: EDString,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<EDString>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-12-01T15:27:40Z", "event":"TechnologyBroker", "BrokerType":"sirius", "MarketID":129009240, "ItemsUnlocked":[ { "Name":"Hpt_HeatSinkLauncher_Turret_Tiny", "Name_Localised":"Heatsink" } ], "Commodities":[  ], "Materials":[ { "Name":"mechanicalscrap", "Name_Localised":"Mechanical Scrap", "Count":8, "Category":"Manufactured" }, { "Name":"niobium", "Count":6, "Category":"Raw" }, { "Name":"vanadium", "Count":6, "Category":"Raw" }, { "Name":"mechanicalcomponents", "Name_Localised":"Mechanical Components", "Count":5, "Category":"Manufactured" } ] })]
pub struct EDLogTechnologyBroker {
    pub broker_type: TechBrokerType,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub items_unlocked: Vec<BrokerItemUnlocked>,
    pub commodities: Vec<BrokerCommodity>,
    pub materials: Vec<BrokerMaterial>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({"timestamp": "2022-11-18T16:19:48Z","event": "ClearImpound","ShipType": "asp",
	"ShipType_Localised": "Asp Explorer","ShipID": 10,"ShipMarketID": 128833431,"MarketID": 128833431
})]
pub struct EDLogClearImpound {
    pub ship_type: ShipType,
    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localised: Option<EDString>,
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
    // pub system: Option<EDString>,
    #[serde(rename = "ShipMarketID")]
    pub ship_market_id: u64,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
}
