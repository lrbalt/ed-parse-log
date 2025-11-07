use crate::{
    common_types::{
        Allegiance, Credits, FactionName, MaterialCategory, StationEconomy, StationService,
        StationType, TechBrokerType, TraderType,
    },
    log_line::{EDLogEvent, Extractable},
    utils::string_or_struct,
};
use ed_parse_log_file_testcase::testcase;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-11-06T18:58:11Z", "event":"RefuelAll", "Cost":60, "Amount":1.187222 })]
pub struct EDLogRefuelAll {
    pub cost: Credits,
    pub amount: f64,
}

impl Extractable for EDLogRefuelAll {
    fn extract(event: &EDLogEvent) -> Option<&Self> {
        if let EDLogEvent::RefuelAll(loc) = event {
            return Some(loc);
        }
        None
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogBuyAmmo {
    cost: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2024-10-15T18:34:25Z", "event":"Repair", "Items":[ "$python_nx_cockpit_name;", "Hull", "$modularcargobaydoor_name;", "Wear" ], "Cost":811 })]
pub struct EDLogRepair {
    pub item: Option<String>,
    pub items: Option<Vec<String>>,
    pub cost: Credits,
}

impl Extractable for EDLogRepair {
    fn extract(event: &EDLogEvent) -> Option<&Self> {
        if let EDLogEvent::Repair(loc) = event {
            return Some(loc);
        }
        None
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-11-06T18:58:12Z", "event":"RepairAll", "Cost":22513 })]
pub struct EDLogRepairAll {
    pub cost: Credits,
}

impl Extractable for EDLogRepairAll {
    fn extract(event: &EDLogEvent) -> Option<&Self> {
        if let EDLogEvent::RepairAll(loc) = event {
            return Some(loc);
        }
        None
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LandingPads {
    small: u64,
    medium: u64,
    large: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDockingRequested {
    #[serde(rename = "MarketID")]
    market_id: Option<u64>,
    station_name: String,
    #[serde(rename = "StationName_Localised")]
    station_name_localised: Option<String>,
    station_type: Option<StationType>,
    landing_pads: Option<LandingPads>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDockingCancelled {
    #[serde(rename = "MarketID")]
    market_id: u64,
    station_name: String,
    station_type: StationType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDockingGranted {
    landing_pad: u64,
    #[serde(rename = "MarketID")]
    market_id: Option<u64>,
    station_name: String,
    #[serde(rename = "StationName_Localised")]
    station_name_localised: Option<String>,
    station_type: Option<StationType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DockingDeniedReason {
    Distance,
    DockOffline,
    Hostile,
    JumpImminent,
    NoReason,
    NoSpace,
    Offences,
    RestrictedAccess,
    TooLarge,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum StationState {
    UnderAttack,
    Damaged,
    Construction,
}

#[testcase({ "timestamp":"2025-06-30T12:38:07Z", "event":"DockingDenied", "Reason":"DockOffline", "MarketID":3906562304, "StationName":"Joshi Military Complex", "StationType":"OnFootSettlement" })]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDockingDenied {
    reason: DockingDeniedReason,
    #[serde(rename = "MarketID")]
    market_id: u64,
    station_name: String,
    #[serde(rename = "StationName_Localised")]
    station_name_localised: Option<String>,
    station_type: StationType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDockingTimeout {
    #[serde(rename = "MarketID")]
    market_id: u64,
    station_name: String,
    station_type: StationType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDocked {
    pub station_name: String,
    #[serde(rename = "StationName_Localised")]
    station_name_localised: Option<String>,
    pub station_type: StationType,
    pub taxi: Option<bool>,
    pub multicrew: Option<bool>,
    pub faction_state: Option<String>,
    pub station_state: Option<StationState>,
    pub star_system: String,
    pub system_address: Option<u64>,
    #[serde(rename = "MarketID")]
    pub market_id: Option<u64>,
    #[serde(deserialize_with = "string_or_struct")]
    pub station_faction: FactionName,
    pub station_government: String,
    #[serde(rename = "StationGovernment_Localised")]
    pub station_government_localised: String,
    pub station_allegiance: Option<Allegiance>,
    pub station_services: Vec<StationService>,
    pub station_economy: String,
    #[serde(rename = "StationEconomy_Localised")]
    pub station_economy_localised: String,
    pub station_economies: Option<Vec<StationEconomy>>,
    #[serde(rename = "DistFromStarLS")]
    pub dist_from_star_ls: f64,
    pub cockpit_breach: Option<bool>,
    pub wanted: Option<bool>,
    pub active_fine: Option<bool>,
    pub landing_pads: Option<LandingPads>,
}

impl Extractable for EDLogDocked {
    fn extract(event: &EDLogEvent) -> Option<&Self> {
        if let EDLogEvent::Docked(loc) = event {
            return Some(loc);
        }
        None
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-10-17T01:49:26Z", "event":"Undocked", "StationName":"Verrazzano's Inheritance", "StationType":"SurfaceStation" })]
pub struct EDLogUndocked {
    station_name: String,
    #[serde(rename = "StationName_Localised")]
    station_name_localised: Option<String>,
    station_type: StationType,
    #[serde(rename = "MarketID")]
    market_id: Option<u64>,
    taxi: Option<bool>,
    multicrew: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogOutfitting {
    #[serde(rename = "MarketID")]
    market_id: u64,
    station_name: String,
    star_system: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ExchangedMaterials {
    material: String,
    #[serde(rename = "Material_Localised")]
    material_localised: Option<String>,
    category: MaterialCategory,
    quantity: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMaterialTrade {
    #[serde(rename = "MarketID")]
    market_id: u64,
    trader_type: TraderType,
    paid: ExchangedMaterials,
    received: ExchangedMaterials,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMaterialCollected {
    category: MaterialCategory,
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPayFines {
    amount: u64,
    all_fines: bool,
    faction: Option<String>,
    #[serde(rename = "ShipID")]
    ship_id: u64,
    broker_percentage: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct BrokerItemUnlocked {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct BrokerMaterial {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    count: u64,
    category: MaterialCategory,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct BrokerCommodity {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogTechnologyBroker {
    pub broker_type: TechBrokerType,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub items_unlocked: Vec<BrokerItemUnlocked>,
    pub commodities: Vec<BrokerCommodity>,
    pub materials: Vec<BrokerMaterial>,
}

impl Extractable for EDLogTechnologyBroker {
    fn extract(event: &EDLogEvent) -> Option<&Self> {
        if let EDLogEvent::TechnologyBroker(loc) = event {
            return Some(loc);
        }
        None
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
