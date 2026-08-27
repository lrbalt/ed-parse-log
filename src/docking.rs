use crate::{
    EDString,
    common_types::{Credits, MaterialCategory, MercCoins, StationType, TechBrokerType, TraderType},
};
use ed_parse_log_files_macros::{Extractable, testcase, testcase_struct};
use serde::{Deserialize, Serialize};
use strum::Display;

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DockingDeniedReason {
    NoSpace,
    TooLarge,
    Hostile,
    Offences,
    Distance,
    ActiveFighter,
    NoReason,
    // following found in logs, but not in manual
    DockOffline,
    #[serde(rename = "DockingUnavliable")]
    DockingUnavailable,
    JumpImminent,
    RestrictedAccess,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum StationState {
    UnderRepairs,
    Damaged,
    Abandoned,
    UnderAttack,
    // following found in logs, but not in manual
    DamagedHuman,
    Construction,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({"id": 129045436, "Name": "hpt_basicmissilerack_fixed_medium",
    "BuyPrice": 0, "BuyMercCoinsPrice": 800 })]
#[testcase_struct({"id": 128049431, "Name": "hpt_beamlaser_fixed_huge",
    "BuyPrice": 2336256 })]
pub struct ModuleOutfitting {
    #[serde(rename = "id")]
    id: u64,
    name: EDString,
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

#[derive(Serialize, Deserialize, Clone, Debug, Copy, PartialEq, Eq, Display)]
pub enum VehicleType {
    #[serde(rename = "testbuggy")]
    #[strum(to_string = "SRV Scarab")]
    Scarab,
    #[serde(rename = "combat_multicrew_srv_01")]
    #[strum(to_string = "SRV Scorpion")]
    Scorpion,
    #[serde(rename = "independent_fighter")]
    #[strum(to_string = "Taipan")]
    Taipan,
    #[serde(rename = "gdn_hybrid_fighter_v1")]
    #[strum(to_string = "Guardian Hybrid Fighter V1")]
    GuardianHybridFighterV1,
    #[serde(rename = "gdn_hybrid_fighter_v2")]
    #[strum(to_string = "Javelin")]
    Javelin,
    #[serde(rename = "gdn_hybrid_fighter_v3")]
    #[strum(to_string = "Lance")]
    Lance,
    #[serde(rename = "federation_fighter")]
    #[strum(to_string = "F63 Condor")]
    F63Condor,
    #[serde(rename = "empire_fighter")]
    #[strum(to_string = "Gu-97")]
    Gu97,
    #[serde(rename = "lander01")]
    #[strum(to_string = "Nomad")]
    Nomad,
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
    #[serde(rename = "Type")]
    pub vehicle_type: VehicleType,
    #[serde(rename = "Type_Localised")]
    pub vehicle_type_localised: Option<EDString>,
    pub loadout: EDString,
    #[serde(rename = "ID")]
    pub id: Option<u64>,
    pub cost: Credits,
    pub count: u64,
}
