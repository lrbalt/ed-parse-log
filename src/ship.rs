use crate::{EDString, common_types::PilotRank, market::MarketItemType};
use ed_parse_log_files_macros::{Extractable, testcase, testcase_struct};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Name":"algae", "Count":1 })]
pub struct Inventory {
    name: MarketItemType,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
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
    cargo_type: EDString,
    #[serde(rename = "Type_Localised")]
    cargo_type_localised: Option<EDString>,
    count: u64,
    abandoned: bool,
    powerplay_origin: Option<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-10-14T18:41:37Z", "event":"Cargo", "Inventory":[  ] })]
pub struct EDLogCargo {
    vessel: Option<EDString>,
    count: Option<u64>,
    inventory: Option<Vec<Inventory>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogShieldState {
    shields_up: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
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
    subsystem: EDString,
    #[serde(rename = "Subsystem_Localised")]
    subsystem_localised: EDString,
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

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogShipTargeted {
    target_locked: bool,
    ship: Option<ShipType>,
    #[serde(rename = "Ship_Localised")]
    ship_localised: Option<EDString>,
    scan_stage: Option<u64>,
    pilot_name: Option<EDString>,
    #[serde(rename = "PilotName_Localised")]
    pilot_name_localised: Option<EDString>,
    #[serde(rename = "SquadronID")]
    squadron_id: Option<EDString>,
    pilot_rank: Option<PilotRank>,
    shield_health: Option<f64>,
    hull_health: Option<f64>,
    faction: Option<EDString>,
    legal_status: Option<LegalStatus>,
    power: Option<EDString>,
    bounty: Option<u64>,
    #[serde(flatten)]
    subsystem: Option<TargetedSubsystem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogInterdiction {
    success: bool,
    is_player: bool,
    faction: EDString,
    power: Option<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogInterdicted {
    submitted: bool,
    interdictor: Option<EDString>,
    #[serde(rename = "Interdictor_Localised")]
    interdictor_localised: Option<EDString>,
    is_player: bool,
    combat_rank: Option<u8>,
    faction: Option<EDString>,
    is_thargoid: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogEscapeInterdiction {
    interdictor: EDString,
    #[serde(rename = "Interdictor_Localised")]
    interdictor_localised: Option<EDString>,
    is_player: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogRebootRepair {
    modules: Vec<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogUnderAttack {
    target: Option<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogHullDamage {
    health: f64,
    player_pilot: bool,
    fighter: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SynthesisMaterial {
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSynthesis {
    name: EDString,
    materials: Vec<SynthesisMaterial>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogAfmuRepairs {
    module: EDString,
    #[serde(rename = "Module_Localised")]
    module_localised: EDString,
    fully_repaired: bool,
    health: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSetUserShipName {
    ship: EDString,
    #[serde(rename = "ShipID")]
    ship_id: u64,
    user_ship_name: EDString,
    user_ship_id: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogClearImpound {
    ship_type: EDString,
    #[serde(rename = "ShipType_Localised")]
    ship_type_localised: Option<EDString>,
    #[serde(rename = "ShipID")]
    ship_id: u64,
    system: Option<EDString>,
    #[serde(rename = "ShipMarketID")]
    ship_market_id: u64,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogLaunchFighter {
    loadout: EDString,
    #[serde(rename = "ID")]
    id: u64,
    player_controlled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(deny_unknown_fields)]
pub struct EDLogDockFighter {
    #[serde(rename = "ID")]
    id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(deny_unknown_fields)]
pub struct EDLogFighterDestroyed {
    #[serde(rename = "ID")]
    id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(deny_unknown_fields)]
pub struct EDLogFighterRebuilt {
    #[serde(rename = "Loadout")]
    loadout: EDString,
    #[serde(rename = "ID")]
    id: u64,
}
