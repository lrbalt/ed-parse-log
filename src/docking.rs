use crate::{
    EDString,
    common_types::{
        Allegiance, Credits, FactionName, MaterialCategory, StationEconomy, StationService,
        StationType, TechBrokerType, TraderType,
    },
    utils::string_or_struct,
};
use ed_parse_log_files_macros::{Extractable, testcase};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-11-06T18:58:11Z", "event":"RefuelAll", "Cost":60, "Amount":1.187222 })]
pub struct EDLogRefuelAll {
    pub cost: Credits,
    pub amount: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogBuyAmmo {
    pub cost: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2024-10-15T18:34:25Z", "event":"Repair", "Items":[ "$python_nx_cockpit_name;", "Hull", "$modularcargobaydoor_name;", "Wear" ], "Cost":811 })]
pub struct EDLogRepair {
    pub item: Option<String>,
    pub items: Option<Vec<String>>,
    pub cost: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-11-06T18:58:12Z", "event":"RepairAll", "Cost":22513 })]
pub struct EDLogRepairAll {
    pub cost: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LandingPads {
    pub small: u64,
    pub medium: u64,
    pub large: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct StationIdentification {
    #[serde(rename = "MarketID")]
    pub market_id: Option<u64>,
    pub station_name: EDString,
    #[serde(rename = "StationName_Localised")]
    pub station_name_localised: Option<EDString>,
    pub station_type: Option<StationType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDockingRequested {
    #[serde(flatten)]
    pub station_identification: StationIdentification,
    pub landing_pads: Option<LandingPads>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDockingCancelled {
    #[serde(flatten)]
    pub station_identification: StationIdentification,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDockingGranted {
    pub landing_pad: u64,
    #[serde(flatten)]
    pub station_identification: StationIdentification,
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
#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDockingDenied {
    reason: DockingDeniedReason,
    #[serde(flatten)]
    pub station_identification: StationIdentification,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDockingTimeout {
    #[serde(flatten)]
    pub station_identification: StationIdentification,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDocked {
    #[serde(flatten)]
    pub station_identification: StationIdentification,
    pub taxi: Option<bool>,
    pub multicrew: Option<bool>,
    pub faction_state: Option<EDString>,
    pub station_state: Option<StationState>,
    pub star_system: EDString,
    pub system_address: Option<u64>,
    #[serde(deserialize_with = "string_or_struct")]
    pub station_faction: FactionName,
    pub station_government: EDString,
    #[serde(rename = "StationGovernment_Localised")]
    pub station_government_localised: EDString,
    pub station_allegiance: Option<Allegiance>,
    pub station_services: Vec<StationService>,
    pub station_economy: EDString,
    #[serde(rename = "StationEconomy_Localised")]
    pub station_economy_localised: EDString,
    pub station_economies: Option<Vec<StationEconomy>>,
    #[serde(rename = "DistFromStarLS")]
    pub dist_from_star_ls: f64,
    pub cockpit_breach: Option<bool>,
    pub wanted: Option<bool>,
    pub active_fine: Option<bool>,
    pub landing_pads: Option<LandingPads>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-10-17T01:49:26Z", "event":"Undocked", "StationName":"Verrazzano's Inheritance", "StationType":"SurfaceStation" })]
pub struct EDLogUndocked {
    #[serde(flatten)]
    pub station_identification: StationIdentification,
    taxi: Option<bool>,
    multicrew: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2022-09-17T13:01:22Z", "event":"Outfitting", "MarketID":3223506432, "StationName":"Coleman Ring", "StarSystem":"BZ Ceti" })]
pub struct EDLogOutfitting {
    #[serde(flatten)]
    pub station_identification: StationIdentification,
    star_system: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ExchangedMaterials {
    material: EDString,
    #[serde(rename = "Material_Localised")]
    material_localised: Option<EDString>,
    category: MaterialCategory,
    quantity: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMaterialTrade {
    #[serde(rename = "MarketID")]
    market_id: u64,
    trader_type: TraderType,
    paid: ExchangedMaterials,
    received: ExchangedMaterials,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMaterialCollected {
    category: MaterialCategory,
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPayBounties {
    pub amount: Credits,
    pub all_fines: Option<bool>,
    pub faction: Option<EDString>,
    #[serde(rename = "Faction_Localised")]
    pub faction_localised: Option<EDString>,
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
    pub broker_percentage: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
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
pub struct BrokerItemUnlocked {
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct BrokerMaterial {
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    count: u64,
    category: MaterialCategory,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct BrokerCommodity {
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
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
pub struct EDLogRestockVehicle {
    #[serde(rename = "Type")]
    vehicle_type: EDString,
    #[serde(rename = "Type_Localised")]
    vehicle_type_localised: Option<EDString>,
    loadout: EDString,
    #[serde(rename = "ID")]
    id: Option<u64>,
    cost: Credits,
    count: u64,
}
