use crate::{common_types::PilotRank, market::MarketItemType};
use ed_parse_log_files_macros::{testcase, testcase_struct};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Name":"algae", "Count":1 })]
pub struct Inventory {
    name: MarketItemType,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    count: u64,
    stolen: Option<u64>,
    #[serde(rename = "MissionID")]
    mission_id: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-06-07T23:31:33Z", "event":"EjectCargo", "Type":"alliancetradeagreements", 
             "Type_Localised":"Alliance Trade Agreements", "Count":2, "Abandoned":false, "PowerplayOrigin":"" })]
pub struct EDLogEjectCargo {
    #[serde(rename = "Type")]
    cargo_type: String,
    #[serde(rename = "Type_Localised")]
    cargo_type_localised: Option<String>,
    count: u64,
    abandoned: bool,
    powerplay_origin: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-10-14T18:41:37Z", "event":"Cargo", "Inventory":[  ] })]
pub struct EDLogCargo {
    vessel: Option<String>,
    count: Option<u64>,
    inventory: Option<Vec<Inventory>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogShieldState {
    shields_up: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogReservoirReplenished {
    fuel_main: f64,
    fuel_reservoir: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LegalStatus {
    Unknown,
    Clean,
    Wanted,
    WantedEnemy,
    Lawless,
    Enemy,
    Hunter,
    Thargoid22,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct TargetedSubsystem {
    subsystem: String,
    #[serde(rename = "Subsystem_Localised")]
    subsystem_localised: String,
    subsystem_health: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ShipType {
    #[serde(alias = "Adder")]
    Adder,
    #[serde(alias = "adder_taxi")]
    AdderTaxi,
    #[serde(alias = "Asp")]
    Asp,
    #[serde(rename = "asp_scout")]
    AspScout,
    #[serde(alias = "Anaconda")]
    Anaconda,
    #[serde(alias = "BelugaLiner")]
    BelugaLiner,
    #[serde(alias = "CobraMkIII")]
    CobraMkIII,
    CobraMkIV,
    #[serde(alias = "CobraMkV")]
    CobraMkV,
    #[serde(alias = "Corsair")]
    Corsair,
    #[serde(alias = "Cutter")]
    Cutter,
    DiamondBack,
    #[serde(alias = "DiamondBackXL")]
    DiamondBackXL,
    #[serde(rename = "diamondback_taxi")]
    DiamondBackTaxi,
    Dolphin,
    #[serde(alias = "Eagle")]
    Eagle,
    #[serde(rename = "empire_courier", alias = "Empire_Courier")]
    EmpireCourier,
    #[serde(rename = "empire_eagle", alias = "Empire_Eagle")]
    EmpireEagle,
    #[serde(rename = "empire_fighter")]
    Empirefighter,
    #[serde(rename = "empire_trader", alias = "Empire_Trader")]
    EmpireTrader,
    #[serde(rename = "federation_corvette", alias = "Federation_Corvette")]
    FederationCorvette,
    #[serde(rename = "federation_dropship")]
    FederationDropship,
    #[serde(rename = "federation_dropship_mkii")]
    FederationDropshipMkII,
    #[serde(rename = "federation_fighter")]
    FederationFighter,
    #[serde(rename = "federation_gunship")]
    FederationGunship,
    #[serde(alias = "FerDeLance")]
    FerDeLance,
    #[serde(rename = "gdn_hybrid_fighter_v1")]
    GuardianHybridFighterV1,
    #[serde(rename = "gdn_hybrid_fighter_v2")]
    GuardianHybridFighterV2,
    #[serde(rename = "gdn_hybrid_fighter_v3")]
    GuardianHybridFighterV3,
    Hauler,
    #[serde(rename = "independent_fighter")]
    IndependantFighter,
    #[serde(rename = "independant_trader")]
    IndependentTrader,
    #[serde(rename = "krait_light", alias = "Krait_Light")]
    KraitLight,
    #[serde(rename = "krait_mkii", alias = "Krait_MkII")]
    KraitMkII,
    #[serde(alias = "Mamba")]
    Mamba,
    #[serde(alias = "Mandalay")]
    Mandalay,
    Orca,
    #[serde(alias = "PantherMkII")]
    PantherMkII,
    #[serde(alias = "Python")]
    Python,
    #[serde(rename = "python_nx", alias = "Python_NX")]
    PythonNX,
    #[serde(alias = "SideWinder")]
    Sidewinder,
    #[serde(alias = "Type6")]
    Type6,
    #[serde(alias = "Type7")]
    Type7,
    #[serde(alias = "Type8")]
    Type8,
    #[serde(alias = "Type9")]
    Type9,
    #[serde(rename = "type9_military", alias = "Type9_Military")]
    Type9Military,
    #[serde(alias = "TypeX")]
    TypeX,
    #[serde(rename = "typex_2")]
    TypeX2,
    #[serde(rename = "typex_3")]
    TypeX3,
    #[serde(alias = "Viper")]
    Viper,
    #[serde(rename = "viper_mkiv")]
    ViperMkIV,
    #[serde(rename = "viper_taxi")]
    ViperTaxi,
    #[serde(alias = "Vulture")]
    Vulture,
    #[serde(rename = "vulture_taxi")]
    VultureTaxi,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogShipTargeted {
    target_locked: bool,
    ship: Option<ShipType>,
    #[serde(rename = "Ship_Localised")]
    ship_localised: Option<String>,
    scan_stage: Option<u64>,
    pilot_name: Option<String>,
    #[serde(rename = "PilotName_Localised")]
    pilot_name_localised: Option<String>,
    #[serde(rename = "SquadronID")]
    squadron_id: Option<String>,
    pilot_rank: Option<PilotRank>,
    shield_health: Option<f64>,
    hull_health: Option<f64>,
    faction: Option<String>,
    legal_status: Option<LegalStatus>,
    power: Option<String>,
    bounty: Option<u64>,
    #[serde(flatten)]
    subsystem: Option<TargetedSubsystem>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogInterdiction {
    success: bool,
    is_player: bool,
    faction: String,
    power: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogInterdicted {
    submitted: bool,
    interdictor: Option<String>,
    #[serde(rename = "Interdictor_Localised")]
    interdictor_localised: Option<String>,
    is_player: bool,
    combat_rank: Option<u8>,
    faction: Option<String>,
    is_thargoid: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogEscapeInterdiction {
    interdictor: String,
    #[serde(rename = "Interdictor_Localised")]
    interdictor_localised: Option<String>,
    is_player: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogRebootRepair {
    modules: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogUnderAttack {
    target: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogHullDamage {
    health: f64,
    player_pilot: bool,
    fighter: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SynthesisMaterial {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSynthesis {
    name: String,
    materials: Vec<SynthesisMaterial>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogAfmuRepairs {
    module: String,
    #[serde(rename = "Module_Localised")]
    module_localised: String,
    fully_repaired: bool,
    health: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSetUserShipName {
    ship: String,
    #[serde(rename = "ShipID")]
    ship_id: u64,
    user_ship_name: String,
    user_ship_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogClearImpound {
    ship_type: String,
    #[serde(rename = "ShipType_Localised")]
    ship_type_localised: Option<String>,
    #[serde(rename = "ShipID")]
    ship_id: u64,
    system: Option<String>,
    #[serde(rename = "ShipMarketID")]
    ship_market_id: u64,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogLaunchFighter {
    loadout: String,
    #[serde(rename = "ID")]
    id: u64,
    player_controlled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EDLogDockFighter {
    #[serde(rename = "ID")]
    id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EDLogFighterDestroyed {
    #[serde(rename = "ID")]
    id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EDLogFighterRebuilt {
    #[serde(rename = "Loadout")]
    loadout: String,
    #[serde(rename = "ID")]
    id: u64,
}
