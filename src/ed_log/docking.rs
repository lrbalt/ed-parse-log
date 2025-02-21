use crate::ed_log::common_types::{
    Allegiance, FactionName, MaterialCategory, StationEconomy, StationService, StationType,
    TechBrokerType, TraderType,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogRefuelAll {
    cost: u64,
    amount: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogBuyAmmo {
    cost: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogRepair {
    item: Option<String>,
    items: Option<Vec<String>>,
    cost: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogRepairAll {
    cost: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LandingPads {
    small: u64,
    medium: u64,
    large: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDockingRequested {
    #[serde(rename = "MarketID")]
    market_id: u64,
    station_name: String,
    station_type: StationType,
    landing_pads: Option<LandingPads>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDockingCancelled {
    #[serde(rename = "MarketID")]
    market_id: u64,
    station_name: String,
    station_type: StationType,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDockingGranted {
    landing_pad: u64,
    #[serde(rename = "MarketID")]
    market_id: u64,
    station_name: String,
    station_type: StationType,
}

#[derive(Serialize, Deserialize)]
pub enum DockingDeniedReason {
    NoReason,
    Distance,
    NoSpace,
    TooLarge,
    RestrictedAccess,
    Offences,
    Hostile,
    JumpImminent,
}

#[derive(Serialize, Deserialize)]
pub enum StationState {
    UnderAttack,
    Damaged,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDockingDenied {
    reason: DockingDeniedReason,
    #[serde(rename = "MarketID")]
    market_id: u64,
    station_name: String,
    station_type: StationType,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDockingTimeout {
    #[serde(rename = "MarketID")]
    market_id: u64,
    station_name: String,
    station_type: StationType,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDocked {
    station_name: String,
    station_type: StationType,
    taxi: Option<bool>,
    multicrew: Option<bool>,
    station_state: Option<StationState>,
    star_system: String,
    system_address: u64,
    #[serde(rename = "MarketID")]
    market_id: u64,
    station_faction: FactionName,
    station_government: String,
    #[serde(rename = "StationGovernment_Localised")]
    station_government_localised: String,
    station_allegiance: Option<Allegiance>,
    station_services: Vec<StationService>,
    station_economy: String,
    #[serde(rename = "StationEconomy_Localised")]
    station_economy_localised: String,
    station_economies: Vec<StationEconomy>,
    #[serde(rename = "DistFromStarLS")]
    dist_from_star_ls: f64,
    cockpit_breach: Option<bool>,
    wanted: Option<bool>,
    active_fine: Option<bool>,
    landing_pads: Option<LandingPads>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogUndocked {
    station_name: String,
    station_type: StationType,
    #[serde(rename = "MarketID")]
    market_id: u64,
    taxi: Option<bool>,
    multicrew: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogOutfitting {
    #[serde(rename = "MarketID")]
    market_id: u64,
    station_name: String,
    star_system: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ExchangedMaterials {
    material: String,
    #[serde(rename = "Material_Localised")]
    material_localised: Option<String>,
    category: MaterialCategory,
    quantity: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMaterialTrade {
    #[serde(rename = "MarketID")]
    market_id: u64,
    trader_type: TraderType,
    paid: ExchangedMaterials,
    received: ExchangedMaterials,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMaterialCollected {
    category: MaterialCategory,
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    count: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPayBounties {
    amount: u64,
    all_fines: Option<bool>,
    faction: Option<String>,
    #[serde(rename = "Faction_Localised")]
    faction_localised: Option<String>,
    #[serde(rename = "ShipID")]
    ship_id: u64,
    broker_percentage: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPayFines {
    amount: u64,
    all_fines: bool,
    #[serde(rename = "ShipID")]
    ship_id: u64,
    broker_percentage: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct BrokerItemUnlocked {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct BrokerMaterial {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    count: u64,
    category: MaterialCategory,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct BrokerCommodity {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    count: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogTechnologyBroker {
    broker_type: TechBrokerType,
    #[serde(rename = "MarketID")]
    market_id: u64,
    items_unlocked: Vec<BrokerItemUnlocked>,
    commodities: Vec<BrokerCommodity>,
    materials: Vec<BrokerMaterial>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogRestockVehicle {
    #[serde(rename = "Type")]
    vehicle_type: String,
    #[serde(rename = "Type_Localised")]
    vehicle_type_localised: Option<String>,
    loadout: String,
    #[serde(rename = "ID")]
    id: Option<u64>,
    cost: u64,
    count: u64,
}
