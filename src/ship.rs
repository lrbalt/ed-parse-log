use crate::{
    EDString,
    common_types::{PilotRank, Power},
    market::MarketItemType,
};
use ed_parse_log_files_macros::{Extractable, testcase, testcase_struct};
use serde::{Deserialize, Serialize};
use strum::Display;

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
    cargo_type: MarketItemType,
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
    pub subsystem: ShipModule,
    #[serde(rename = "Subsystem_Localised")]
    pub subsystem_localised: EDString,
    pub subsystem_health: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ShipType {
    #[strum(to_string = "Adder")]
    #[serde(alias = "Adder")]
    Adder,
    #[strum(to_string = "$ADDER_NAME;")]
    #[serde(alias = "adder_taxi")]
    AdderTaxi,
    #[strum(to_string = "Asp Explorer")]
    #[serde(alias = "Asp")]
    Asp,
    #[strum(to_string = "Asp Scout")]
    #[serde(rename = "asp_scout")]
    AspScout,
    #[strum(to_string = "Anaconda")]
    #[serde(alias = "Anaconda")]
    Anaconda,
    #[strum(to_string = "Beluga Liner")]
    #[serde(alias = "BelugaLiner")]
    BelugaLiner,
    #[strum(to_string = "Cobra Mk III")]
    #[serde(alias = "CobraMkIII")]
    CobraMkIII,
    #[strum(to_string = "Cobra Mk IV")]
    #[serde(alias = "CobraMkIV")]
    CobraMkIV,
    #[strum(to_string = "Cobra Mk V")]
    #[serde(alias = "CobraMkV")]
    CobraMkV,
    #[strum(to_string = "Corsair")]
    #[serde(alias = "Corsair")]
    Corsair,
    #[strum(to_string = "Imperial Cutter")]
    #[serde(alias = "Cutter")]
    Cutter,
    #[strum(to_string = "Diamondback Scout")]
    #[serde(alias = "DiamondBack")]
    Diamondback,
    #[strum(to_string = "Diamondback Explorer")]
    #[serde(alias = "DiamondBackXL")]
    DiamondbackXL,
    #[strum(to_string = "$DIAMONDBACK_NAME;")]
    #[serde(rename = "diamondback_taxi")]
    DiamondbackTaxi,
    Dolphin,
    #[strum(to_string = "Eagle")]
    #[serde(alias = "Eagle")]
    Eagle,
    #[strum(to_string = "Imperial Courier")]
    #[serde(rename = "empire_courier", alias = "Empire_Courier")]
    EmpireCourier,
    #[strum(to_string = "Imperial Eagle")]
    #[serde(rename = "empire_eagle", alias = "Empire_Eagle")]
    EmpireEagle,
    #[strum(to_string = "Gu-97")]
    #[serde(rename = "empire_fighter")]
    Empirefighter,
    #[strum(to_string = "Imperial Clipper")]
    #[serde(rename = "empire_trader", alias = "Empire_Trader")]
    EmpireTrader,
    #[strum(to_string = "Caspian Explorer")]
    #[serde(rename = "explorer_nx", alias = "Explorer_NX")]
    CaspianExplorer,
    #[strum(to_string = "Federal Corvette")]
    #[serde(rename = "federation_corvette", alias = "Federation_Corvette")]
    FederationCorvette,
    #[strum(to_string = "Federal Dropship")]
    #[serde(rename = "federation_dropship")]
    FederationDropship,
    #[strum(to_string = "Federal Assault Ship")]
    #[serde(rename = "federation_dropship_mkii")]
    FederationDropshipMkII,
    #[strum(to_string = "F63 Condor")]
    #[serde(rename = "federation_fighter")]
    FederationFighter,
    #[serde(rename = "federation_gunship")]
    #[strum(to_string = "Federal Gunship")]
    FederationGunship,
    #[serde(alias = "FerDeLance")]
    #[strum(to_string = "Fer-de-Lance")]
    FerDeLance,
    #[strum(to_string = "Trident")]
    #[serde(rename = "gdn_hybrid_fighter_v1")]
    GuardianHybridFighterV1,
    #[strum(to_string = "Javelin")]
    #[serde(rename = "gdn_hybrid_fighter_v2")]
    GuardianHybridFighterV2,
    #[strum(to_string = "Lance")]
    #[serde(rename = "gdn_hybrid_fighter_v3")]
    GuardianHybridFighterV3,
    Hauler,
    #[serde(rename = "independent_fighter")]
    #[strum(to_string = "Taipan")]
    IndependentFighter,
    #[serde(rename = "independant_trader")]
    #[strum(to_string = "Keelback")]
    IndependentTrader,
    #[strum(to_string = "Krait Phantom")]
    #[serde(rename = "krait_light", alias = "Krait_Light")]
    KraitLight,
    #[strum(to_string = "Krait Mk II")]
    #[serde(rename = "krait_mkii", alias = "Krait_MkII")]
    KraitMkII,
    #[strum(to_string = "Lynx Highliner")]
    #[serde(alias = "mediumtransport01", alias = "MediumTransport01")]
    LynxHighliner,
    #[serde(alias = "Mamba")]
    Mamba,
    #[serde(alias = "Mandalay")]
    Mandalay,
    Orca,
    #[serde(alias = "PantherMkII")]
    #[strum(to_string = "Panther Clipper Mk II")]
    PantherMkII,
    #[serde(alias = "Python")]
    Python,
    #[strum(to_string = "Python Mk II")]
    #[serde(rename = "python_nx", alias = "Python_NX")]
    PythonNX,
    #[serde(alias = "SideWinder")]
    Sidewinder,
    #[strum(to_string = "Type-6 Transporter")]
    #[serde(alias = "Type6")]
    Type6,
    #[serde(alias = "Type7")]
    #[strum(to_string = "Type-7 Transporter")]
    Type7,
    #[strum(to_string = "Type-8 Transporter")]
    #[serde(alias = "Type8")]
    Type8,
    #[strum(to_string = "Type-9 Heavy")]
    #[serde(alias = "Type9")]
    Type9,
    #[strum(to_string = "Type-10 Defender")]
    #[serde(rename = "type9_military", alias = "Type9_Military")]
    Type9Military,
    #[strum(to_string = "Alliance Chieftain")]
    #[serde(alias = "TypeX")]
    TypeX,
    #[strum(to_string = "Alliance Crusader")]
    #[serde(rename = "typex_2")]
    TypeX2,
    #[strum(to_string = "Alliance Challenger")]
    #[serde(rename = "typex_3")]
    TypeX3,
    #[strum(to_string = "Type-11 Prospector")]
    #[serde(rename = "lakonminer", alias = "LakonMiner")]
    Type11Prospector,
    #[strum(to_string = "Viper Mk III")]
    #[serde(alias = "Viper")]
    Viper,
    #[strum(to_string = "Viper Mk IV")]
    #[serde(rename = "viper_mkiv")]
    ViperMkIV,
    #[strum(to_string = "$VIPER_NAME;")]
    #[serde(rename = "viper_taxi")]
    ViperTaxi,
    #[strum(to_string = "Vulture")]
    #[serde(alias = "Vulture")]
    Vulture,
    #[strum(to_string = "$VULTURE_NAME;")]
    #[serde(rename = "vulture_taxi")]
    VultureTaxi,
    #[strum(to_string = "SRV Scorpion")]
    #[serde(rename = "Combat_Multicrew_SRV_01")]
    SRVScorpion,
    #[strum(to_string = "SRV Scarab")]
    #[serde(rename = "TestBuggy")]
    SRVScarab,
    #[strum(to_string = "Flight Suit")]
    #[serde(rename = "FlightSuit")]
    FlightSuit,
    #[strum(to_string = "Maverick Suit")]
    #[serde(rename = "UtilitySuit_Class1")]
    UtilitySuitClass1,
    #[strum(to_string = "$UtilitySuit_Class1_Name;")]
    #[serde(rename = "UtilitySuit_Class2")]
    UtilitySuitClass2,
    #[strum(to_string = "$UtilitySuit_Class1_Name;")]
    #[serde(rename = "UtilitySuit_Class3")]
    UtilitySuitClass3,
    #[strum(to_string = "$UtilitySuit_Class1_Name;")]
    #[serde(rename = "UtilitySuit_Class4")]
    UtilitySuitClass4,
    #[strum(to_string = "$UtilitySuit_Class1_Name;")]
    #[serde(rename = "UtilitySuit_Class5")]
    UtilitySuitClass5,
    #[strum(to_string = "Artemis Suit")]
    #[serde(rename = "ExplorationSuit_Class1")]
    ExplorationSuitClass1,
    #[strum(to_string = "$ExplorationSuit_Class1_Name;")]
    #[serde(rename = "ExplorationSuit_Class2")]
    ExplorationSuitClass2,
    #[strum(to_string = "$ExplorationSuit_Class1_Name;")]
    #[serde(rename = "ExplorationSuit_Class3")]
    ExplorationSuitClass3,
    #[strum(to_string = "$ExplorationSuit_Class1_Name;")]
    #[serde(rename = "ExplorationSuit_Class4")]
    ExplorationSuitClass4,
    #[strum(to_string = "$ExplorationSuit_Class1_Name;")]
    #[serde(rename = "ExplorationSuit_Class5")]
    ExplorationSuitClass5,
    #[strum(to_string = "$TacticalSuit_Class1_Name;")]
    #[serde(rename = "TacticalSuit_Class1")]
    TacticalSuitClass1,
    #[strum(to_string = "$TacticalSuit_Class1_Name;")]
    #[serde(rename = "TacticalSuit_Class2")]
    TacticalSuitClass2,
    #[strum(to_string = "$TacticalSuit_Class1_Name;")]
    #[serde(rename = "TacticalSuit_Class3")]
    TacticalSuitClass3,
    #[strum(to_string = "$TacticalSuit_Class1_Name;")]
    #[serde(rename = "TacticalSuit_Class4")]
    TacticalSuitClass4,
    #[strum(to_string = "$TacticalSuit_Class1_Name;")]
    #[serde(rename = "TacticalSuit_Class5")]
    TacticalSuitClass5,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogShipTargeted {
    pub target_locked: bool,
    pub ship: Option<ShipType>,
    #[serde(rename = "Ship_Localised")]
    pub ship_localised: Option<EDString>,
    pub scan_stage: Option<u64>,
    pub pilot_name: Option<EDString>,
    #[serde(rename = "PilotName_Localised")]
    pub pilot_name_localised: Option<EDString>,
    #[serde(rename = "SquadronID")]
    pub squadron_id: Option<EDString>,
    pub pilot_rank: Option<PilotRank>,
    pub shield_health: Option<f64>,
    pub hull_health: Option<f64>,
    pub faction: Option<EDString>,
    pub legal_status: Option<LegalStatus>,
    pub power: Option<Power>,
    pub bounty: Option<u64>,
    #[serde(flatten)]
    pub subsystem: Option<TargetedSubsystem>,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub enum ShipModuleSlot {
    CargoHatch,
    DataLinkScanner,
    FrameShiftDrive,
    LargeHardpoint1,
    LargeHardpoint2,
    LifeSupport,
    HugeHardpoint1,
    MainEngines,
    PowerDistributor,
    TinyHardpoint2,
    #[serde(rename = "Slot01_Size6")]
    Slot01Size6,
    #[serde(rename = "Slot03_Size5")]
    Slot03Size5,
    #[serde(rename = "Slot04_Size2")]
    Slot04Size2,
    #[serde(rename = "Slot04_Size5")]
    Slot04Size5,
    #[serde(rename = "Slot04_Size6")]
    Slot04Size6,
    #[serde(rename = "Slot05_Size1")]
    Slot05Size1,
    #[serde(rename = "Slot05_Size2")]
    Slot05Size2,
    #[serde(rename = "Slot06_Size1")]
    Slot06Size1,
    #[serde(rename = "Slot07_Size5")]
    Slot07Size5,
    #[serde(rename = "Slot09_Size4")]
    Slot09Size4,
    #[serde(rename = "Slot11_Size2")]
    Slot11Size2,
    #[serde(rename = "Slot13_Size2")]
    Slot13Size2,
    #[serde(rename = "Slot14_Size1")]
    Slot14Size1,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogRebootRepair {
    modules: Vec<ShipModuleSlot>,
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

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub enum ShipModule {
    #[strum(to_string = "Guardian FSD Booster")]
    #[serde(
        alias = "$int_guardianfsdbooster_size2_name;",
        alias = "int_guardianfsdbooster_size2"
    )]
    GuardianFSDBoosterSize2,
    #[strum(to_string = "Guardian FSD Booster")]
    #[serde(
        alias = "$int_guardianfsdbooster_size3_name;",
        alias = "int_guardianfsdbooster_size3"
    )]
    GuardianFSDBoosterSize3,
    #[strum(to_string = "Guardian FSD Booster")]
    #[serde(
        alias = "$int_guardianfsdbooster_size4_name;",
        alias = "int_guardianfsdbooster_size4"
    )]
    GuardianFSDBoosterSize4,
    #[strum(to_string = "FSD Interdictor")]
    #[serde(
        alias = "$int_fsdinterdictor_size1_class2_name;",
        alias = "int_fsdinterdictor_size1_class2"
    )]
    FSDInterdictorS1C2,
    #[strum(to_string = "FSD Interdictor")]
    #[serde(
        alias = "$int_fsdinterdictor_size1_class3_name;",
        alias = "int_fsdinterdictor_size1_class3"
    )]
    FSDInterdictorS1C3,
    #[strum(to_string = "FSD Interdictor")]
    #[serde(
        alias = "$int_fsdinterdictor_size1_class5_name;",
        alias = "int_fsdinterdictor_size1_class5"
    )]
    FSDInterdictorS1C5,
    #[strum(to_string = "FSD Interdictor")]
    #[serde(
        alias = "$int_fsdinterdictor_size2_class5_name;",
        alias = "int_fsdinterdictor_size2_class5"
    )]
    FSDInterdictorS2C5,
    #[strum(to_string = "FSD Interdictor")]
    #[serde(
        alias = "$int_fsdinterdictor_size3_class3_name;",
        alias = "int_fsdinterdictor_size3_class3"
    )]
    FSDInterdictorS3C3,
    #[strum(to_string = "FSD Interdictor")]
    #[serde(
        alias = "$int_fsdinterdictor_size3_class5_name;",
        alias = "int_fsdinterdictor_size3_class5"
    )]
    FSDInterdictorS3C5,
    #[strum(to_string = "FSD Interdictor")]
    #[serde(
        alias = "$int_fsdinterdictor_size4_class3_name;",
        alias = "int_fsdinterdictor_size4_class3"
    )]
    FSDInterdictorS4C3,
    #[strum(to_string = "FSD Interdictor")]
    #[serde(
        alias = "$int_fsdinterdictor_size4_class5_name;",
        alias = "int_fsdinterdictor_size4_class5"
    )]
    FSDInterdictorS4C5,
    #[strum(to_string = "FSD")]
    #[serde(
        alias = "$int_hyperdrive_size2_class1_name;",
        alias = "int_hyperdrive_size2_class1"
    )]
    FSDS2C1,
    #[strum(to_string = "FSD")]
    #[serde(
        alias = "$int_hyperdrive_size2_class2_name;",
        alias = "int_hyperdrive_size2_class2"
    )]
    FSDS2C2,
    #[strum(to_string = "FSD")]
    #[serde(
        alias = "$int_hyperdrive_size2_class3_name;",
        alias = "int_hyperdrive_size2_class3"
    )]
    FSDS2C3,
    #[strum(to_string = "FSD (SCO)")]
    #[serde(
        alias = "$int_hyperdrive_overcharge_size2_class5_name;",
        alias = "int_hyperdrive_overcharge_size2_class5"
    )]
    FSDSCO2C5,
    #[strum(to_string = "FSD")]
    #[serde(
        alias = "$int_hyperdrive_size2_class5_name;",
        alias = "int_hyperdrive_size2_class5"
    )]
    FSDS2C5,
    #[strum(to_string = "FSD")]
    #[serde(
        alias = "$int_hyperdrive_size3_class1_name;",
        alias = "int_hyperdrive_size3_class1"
    )]
    FSDS3C1,
    #[strum(to_string = "FSD")]
    #[serde(
        alias = "$int_hyperdrive_size3_class3_name;",
        alias = "int_hyperdrive_size3_class3"
    )]
    FSDS3C3,
    #[strum(to_string = "FSD (SCO)")]
    #[serde(
        alias = "$int_hyperdrive_overcharge_size3_class3_name;",
        alias = "int_hyperdrive_overcharge_size3_class3"
    )]
    FSDSCOS3C3,
    #[strum(to_string = "FSD")]
    #[serde(
        alias = "$int_hyperdrive_size3_class4_name;",
        alias = "int_hyperdrive_size3_class4"
    )]
    FSDS3C4,
    #[strum(to_string = "FSD (SCO)")]
    #[serde(
        alias = "$int_hyperdrive_overcharge_size3_class5_name;",
        alias = "int_hyperdrive_overcharge_size3_class5"
    )]
    FSDSCOS3C5,
    #[strum(to_string = "FSD")]
    #[serde(
        alias = "$int_hyperdrive_size3_class5_name;",
        alias = "int_hyperdrive_size3_class5"
    )]
    FSDS3C5,
    #[strum(to_string = "FSD")]
    #[serde(
        alias = "$int_hyperdrive_size4_class1_name;",
        alias = "int_hyperdrive_size4_class1"
    )]
    FSDS4C1,
    #[strum(to_string = "FSD")]
    #[serde(
        alias = "$int_hyperdrive_size4_class3_name;",
        alias = "int_hyperdrive_size4_class3"
    )]
    FSDS4C3,
    #[strum(to_string = "FSD (SCO)")]
    #[serde(
        alias = "$int_hyperdrive_overcharge_size4_class1_name;",
        alias = "int_hyperdrive_overcharge_size4_class1"
    )]
    FSDSCOS4C1,
    #[strum(to_string = "FSD (SCO)")]
    #[serde(
        alias = "$int_hyperdrive_overcharge_size4_class3_name;",
        alias = "int_hyperdrive_overcharge_size4_class3"
    )]
    FSDSCOS4C3,
    #[strum(to_string = "FSD (SCO)")]
    #[serde(
        alias = "$int_hyperdrive_overcharge_size4_class5_name;",
        alias = "int_hyperdrive_overcharge_size4_class5"
    )]
    FSDSCOS4C5,
    #[strum(to_string = "FSD")]
    #[serde(
        alias = "$int_hyperdrive_size4_class5_name;",
        alias = "int_hyperdrive_size4_class5"
    )]
    FSDS4C5,
    #[strum(to_string = "FSD")]
    #[serde(
        alias = "$int_hyperdrive_size5_class1_name;",
        alias = "int_hyperdrive_size5_class1"
    )]
    FSDS5C1,
    #[strum(to_string = "FSD")]
    #[serde(
        alias = "$int_hyperdrive_size5_class2_name;",
        alias = "int_hyperdrive_size5_class2"
    )]
    FSDS5C2,
    #[strum(to_string = "FSD")]
    #[serde(
        alias = "$int_hyperdrive_size5_class3_name;",
        alias = "int_hyperdrive_size5_class3"
    )]
    FSDS5C3,
    #[strum(to_string = "FSD")]
    #[serde(
        alias = "$int_hyperdrive_size5_class4_name;",
        alias = "int_hyperdrive_size5_class4"
    )]
    FSDS5C4,
    #[strum(to_string = "FSD")]
    #[serde(
        alias = "$int_hyperdrive_size5_class5_name;",
        alias = "int_hyperdrive_size5_class5"
    )]
    FSDS5C5,
    #[strum(to_string = "FSD (SCO)")]
    #[serde(
        alias = "$int_hyperdrive_overcharge_size5_class1_name;",
        alias = "int_hyperdrive_overcharge_size5_class1"
    )]
    FSDSCOS5C1,
    #[strum(to_string = "FSD (SCO)")]
    #[serde(
        alias = "$int_hyperdrive_overcharge_size5_class2_name;",
        alias = "int_hyperdrive_overcharge_size5_class2"
    )]
    FSDSCOS5C2,
    #[strum(to_string = "FSD (SCO)")]
    #[serde(
        alias = "$int_hyperdrive_overcharge_size5_class3_name;",
        alias = "int_hyperdrive_overcharge_size5_class3"
    )]
    FSDSCOS5C3,
    #[strum(to_string = "FSD (SCO)")]
    #[serde(
        alias = "$int_hyperdrive_overcharge_size5_class5_name;",
        alias = "int_hyperdrive_overcharge_size5_class5"
    )]
    FSDSCOS5C5,
    #[strum(to_string = "FSD (SCO)")]
    #[serde(
        alias = "$int_hyperdrive_overcharge_size6_class3_name;",
        alias = "int_hyperdrive_overcharge_size6_class3"
    )]
    FSDSCOS6C3,
    #[strum(to_string = "FSD (SCO)")]
    #[serde(
        alias = "$int_hyperdrive_overcharge_size6_class5_name;",
        alias = "int_hyperdrive_overcharge_size6_class5"
    )]
    FSDSCOS6C5,
    #[strum(to_string = "FSD")]
    #[serde(
        alias = "$int_hyperdrive_size6_class1_name;",
        alias = "int_hyperdrive_size6_class1"
    )]
    FSDS6C1,
    #[strum(to_string = "FSD")]
    #[serde(
        alias = "$int_hyperdrive_size6_class3_name;",
        alias = "int_hyperdrive_size6_class3"
    )]
    FSDS6C3,
    #[strum(to_string = "FSD")]
    #[serde(
        alias = "$int_hyperdrive_size6_class5_name;",
        alias = "int_hyperdrive_size6_class5"
    )]
    FSDS6C5,
    #[strum(to_string = "FSD (SCO)")]
    #[serde(
        alias = "$int_hyperdrive_overcharge_size7_class1_name;",
        alias = "int_hyperdrive_overcharge_size7_class1"
    )]
    FSDSCOS7C1,
    #[strum(to_string = "FSD")]
    #[serde(
        alias = "$int_hyperdrive_size7_class1_name;",
        alias = "int_hyperdrive_size7_class1"
    )]
    FSDS7C1,
    #[strum(to_string = "FSD")]
    #[serde(
        alias = "$int_hyperdrive_size7_class3_name;",
        alias = "int_hyperdrive_size7_class3"
    )]
    FSDS7C3,
    #[strum(to_string = "FSD")]
    #[serde(
        alias = "$int_hyperdrive_size7_class5_name;",
        alias = "int_hyperdrive_size7_class5"
    )]
    FSDS7C5,
    #[strum(to_string = "FSD (SCO)")]
    #[serde(
        alias = "$int_hyperdrive_overcharge_size7_class5_name;",
        alias = "int_hyperdrive_overcharge_size7_class5"
    )]
    FSDSCOS7C5,
    #[strum(to_string = "FSD (SCO)")]
    #[serde(
        alias = "$int_hyperdrive_overcharge_size8_class5_overchargebooster_mkii_name;",
        alias = "int_hyperdrive_overcharge_size8_class5_overchargebooster_mkii"
    )]
    FSDSCOS8C5OverchargeBoosterMkII,
    #[strum(to_string = "Surface Scanner")]
    #[serde(
        alias = "$int_detailedsurfacescanner_tiny_name;",
        alias = "int_detailedsurfacescanner_tiny"
    )]
    SurfaceScanner,
    #[strum(to_string = "Manifest Scanner")]
    #[serde(
        alias = "$hpt_cargoscanner_size0_class1_name;",
        alias = "hpt_cargoscanner_size0_class1"
    )]
    CargoScannerS0C1,
    #[strum(to_string = "Manifest Scanner")]
    #[serde(
        alias = "$hpt_cargoscanner_size0_class2_name;",
        alias = "hpt_cargoscanner_size0_class2"
    )]
    CargoScannerS0C2,
    #[strum(to_string = "Manifest Scanner")]
    #[serde(
        alias = "$hpt_cargoscanner_size0_class3_name;",
        alias = "hpt_cargoscanner_size0_class3"
    )]
    CargoScannerS0C3,
    #[strum(to_string = "Manifest Scanner")]
    #[serde(
        alias = "$hpt_cargoscanner_size0_class4_name;",
        alias = "hpt_cargoscanner_size0_class4"
    )]
    CargoScannerS0C4,
    #[strum(to_string = "Manifest Scanner")]
    #[serde(
        alias = "$hpt_cargoscanner_size0_class5_name;",
        alias = "hpt_cargoscanner_size0_class5"
    )]
    CargoScannerS0C5,
    #[strum(to_string = "Scanner")]
    #[serde(
        alias = "$hpt_cloudscanner_size0_class1_name;",
        alias = "hpt_cloudscanner_size0_class1"
    )]
    CloudScannerS0C1,
    #[strum(to_string = "Scanner")]
    #[serde(
        alias = "$hpt_cloudscanner_size0_class2_name;",
        alias = "hpt_cloudscanner_size0_class2"
    )]
    CloudScannerS0C2,
    #[strum(to_string = "Scanner")]
    #[serde(
        alias = "$hpt_cloudscanner_size0_class3_name;",
        alias = "hpt_cloudscanner_size0_class3"
    )]
    CloudScannerS0C3,
    #[strum(to_string = "Scanner")]
    #[serde(
        alias = "$hpt_cloudscanner_size0_class4_name;",
        alias = "hpt_cloudscanner_size0_class4"
    )]
    CloudScannerS0C4,
    #[strum(to_string = "Scanner")]
    #[serde(
        alias = "$hpt_cloudscanner_size0_class5_name;",
        alias = "hpt_cloudscanner_size0_class5"
    )]
    CloudScannerS0C5,
    #[strum(to_string = "Scanner")]
    #[serde(
        alias = "$hpt_crimecanner_size0_class2_name;",
        alias = "hpt_crimecanner_size0_class2"
    )]
    CrimeScannerS0C2,
    #[strum(to_string = "Scanner")]
    #[serde(
        alias = "$hpt_crimecanner_size0_class3_name;",
        alias = "hpt_crimecanner_size0_class3"
    )]
    CrimeScannerS0C3,
    #[strum(to_string = "Scanner")]
    #[serde(
        alias = "$hpt_crimecanner_size0_class4_name;",
        alias = "hpt_crimecanner_size0_class4"
    )]
    CrimeScannerS0C4,
    #[strum(to_string = "Scanner")]
    #[serde(
        alias = "$hpt_crimecanner_size0_class5_name;",
        alias = "hpt_crimecanner_size0_class5"
    )]
    CrimeScannerS0C5,
    #[strum(to_string = "Scanner")]
    #[serde(
        alias = "$hpt_mrascanner_size0_class4_name;",
        alias = "hpt_mrascanner_size0_class4"
    )]
    MRAScannerS0C4,
    #[strum(to_string = "Scanner")]
    #[serde(
        alias = "$hpt_mrascanner_size0_class5_name;",
        alias = "hpt_mrascanner_size0_class5"
    )]
    MRAScannerS0C5,
    #[strum(to_string = "Guardian FSD Booster")]
    #[serde(
        alias = "$int_guardianfsdbooster_size5_name;",
        alias = "int_guardianfsdbooster_size5"
    )]
    GuardianFSDBooster,
    #[strum(to_string = "Drive")]
    #[serde(alias = "$ext_drive_class1_a_name;", alias = "ext_drive_class1_a")]
    DriveClass1A,
    #[strum(to_string = "Drive")]
    #[serde(alias = "$ext_drive_class1_c_name;", alias = "ext_drive_class1_c")]
    DriveClass1C,
    #[strum(to_string = "Drive")]
    #[serde(alias = "$ext_drive_class2_c_name;", alias = "ext_drive_class2_c")]
    DriveClass2C,
    #[strum(to_string = "Drive")]
    #[serde(alias = "$ext_drive_class2_a_name;", alias = "ext_drive_class2_a")]
    DriveClass2A,
    #[strum(to_string = "Drive")]
    #[serde(alias = "$ext_drive_class2_b_name;", alias = "ext_drive_class2_b")]
    DriveClass2B,
    #[strum(to_string = "Drive")]
    #[serde(alias = "$ext_drive_class3_cob_name;", alias = "ext_drive_class3_cob")]
    DriveClass3Cob,
    #[strum(to_string = "Drive")]
    #[serde(alias = "$ext_drive_class3_c_name;", alias = "ext_drive_class3_c")]
    DriveClass3C,
    #[strum(to_string = "Drive")]
    #[serde(alias = "$ext_drive_class3_d_name;", alias = "ext_drive_class3_d")]
    DriveClass3D,
    #[strum(to_string = "Drive")]
    #[serde(
        alias = "$ext_drive_class3_d_xl_name;",
        alias = "ext_drive_class3_d_xl"
    )]
    DriveClass3DXL,
    #[strum(to_string = "Drive")]
    #[serde(alias = "$ext_drive_class4_a_name;", alias = "ext_drive_class4_a")]
    DriveClass4A,
    #[strum(to_string = "Drive")]
    #[serde(alias = "$ext_drive_class4_b_name;", alias = "ext_drive_class4_b")]
    DriveClass4B,
    #[strum(to_string = "Drive")]
    #[serde(alias = "$ext_drive_class4_c_name;", alias = "ext_drive_class4_c")]
    DriveClass4C,
    #[strum(to_string = "Drive")]
    #[serde(alias = "$ext_drive_class4_d_name;", alias = "ext_drive_class4_d")]
    DriveClass4D,
    #[strum(to_string = "Drive")]
    #[serde(alias = "$ext_drive_class4_e_name;", alias = "ext_drive_class4_e")]
    DriveClass4E,
    #[strum(to_string = "Drive")]
    #[serde(alias = "$ext_drive_class5_a_name;", alias = "ext_drive_class5_a")]
    DriveClass5A,
    #[strum(to_string = "Drive")]
    #[serde(
        alias = "$ext_drive_class5_a_nx_name;",
        alias = "ext_drive_class5_a_nx"
    )]
    DriveClass5ANX,
    #[strum(to_string = "Drive")]
    #[serde(
        alias = "$ext_drive_class5_a_empire_name;",
        alias = "ext_drive_class5_a_empire"
    )]
    DriveClass5AEmpire,
    #[strum(to_string = "Drive")]
    #[serde(alias = "$ext_drive_class5_b_name;", alias = "ext_drive_class5_b")]
    DriveClass5B,
    #[strum(to_string = "Drive")]
    #[serde(alias = "$ext_drive_class5_d_name;", alias = "ext_drive_class5_d")]
    DriveClass5D,
    #[strum(to_string = "Drive")]
    #[serde(alias = "$ext_drive_class5_e_name;", alias = "ext_drive_class5_e")]
    DriveClass5E,
    #[strum(to_string = "Drive")]
    #[serde(alias = "$ext_drive_class7_a_name;", alias = "ext_drive_class7_a")]
    DriveClass7A,
    #[strum(to_string = "Drive")]
    #[serde(alias = "$ext_drive_class7_b_name;", alias = "ext_drive_class7_b")]
    DriveClass7B,
    #[strum(to_string = "Drive")]
    #[serde(alias = "$ext_drive_class7_c_name;", alias = "ext_drive_class7_c")]
    DriveClass7C,
    #[strum(to_string = "Drive")]
    #[serde(alias = "$ext_drive_class7_d_name;", alias = "ext_drive_class7_d")]
    DriveClass7D,
    #[strum(to_string = "Drive")]
    #[serde(alias = "$ext_drive_krait_name;", alias = "ext_drive_krait")]
    DriveKrait,
    #[strum(to_string = "Drive")]
    #[serde(
        alias = "$ext_drive_krait_light_name;",
        alias = "ext_drive_krait_light"
    )]
    DriveKraitLight,
    #[strum(to_string = "Drive")]
    #[serde(
        alias = "$ext_drive_krait_light_empty_name;",
        alias = "ext_drive_krait_light_empty"
    )]
    DriveKraitLightEmpty,
    #[strum(to_string = "Drive")]
    #[serde(alias = "$ext_drive_indfighter_name;", alias = "ext_drive_indfighter")]
    DriveIndependentFighter,
    #[strum(to_string = "Drive")]
    #[serde(alias = "$ext_drive_mamba_name;", alias = "ext_drive_mamba")]
    DriveMamba,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size2_class5_name;",
        alias = "int_engine_size2_class5"
    )]
    ThrustersS2C5,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size2_class1_name;",
        alias = "int_engine_size2_class1"
    )]
    ThrustersS2C1,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size3_class1_name;",
        alias = "int_engine_size3_class1"
    )]
    ThrustersS3C1,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size3_class4_name;",
        alias = "int_engine_size3_class4"
    )]
    ThrustersS3C4,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size3_class5_name;",
        alias = "int_engine_size3_class5"
    )]
    ThrustersS3C5,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size3_class5_fast_name;",
        alias = "int_engine_size3_class5_fast"
    )]
    ThrustersFastS3C5,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size4_class1_name;",
        alias = "int_engine_size4_class1"
    )]
    ThrustersS4C1,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size4_class2_name;",
        alias = "int_engine_size4_class2"
    )]
    ThrustersS4C2,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size4_class4_name;",
        alias = "int_engine_size4_class4"
    )]
    ThrustersS4C4,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size4_class5_name;",
        alias = "int_engine_size4_class5"
    )]
    ThrustersS4C5,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size5_class1_name;",
        alias = "int_engine_size5_class1"
    )]
    ThrustersS5C1,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size5_class2_name;",
        alias = "int_engine_size5_class2"
    )]
    ThrustersS5C2,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size5_class4_name;",
        alias = "int_engine_size5_class4"
    )]
    ThrustersS5C4,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size5_class5_name;",
        alias = "int_engine_size5_class5"
    )]
    ThrustersS5C5,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size6_class1_name;",
        alias = "int_engine_size6_class1"
    )]
    ThrustersS6C1,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size6_class2_name;",
        alias = "int_engine_size6_class2"
    )]
    ThrustersS6C2,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size6_class4_name;",
        alias = "int_engine_size6_class4"
    )]
    ThrustersS6C4,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size6_class5_name;",
        alias = "int_engine_size6_class5"
    )]
    ThrustersS6C5,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size7_class1_name;",
        alias = "int_engine_size7_class1"
    )]
    ThrustersS7C1,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size7_class2_name;",
        alias = "int_engine_size7_class2"
    )]
    ThrustersS7C2,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size7_class3_name;",
        alias = "int_engine_size7_class3"
    )]
    ThrustersS7C3,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size7_class4_name;",
        alias = "int_engine_size7_class4"
    )]
    ThrustersS7C4,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size7_class5_name;",
        alias = "int_engine_size7_class5"
    )]
    ThrustersS7C5,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size7_class5_gravityoptimised_mkii_name;",
        alias = "int_engine_size7_class5_gravityoptimised_mkii"
    )]
    ThrustersS7C5GravityOptimisedMkII,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size8_class1_name;",
        alias = "int_engine_size8_class1"
    )]
    ThrustersS8C1,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size8_class2_name;",
        alias = "int_engine_size8_class2"
    )]
    ThrustersS8C2,
    #[strum(to_string = "Thrusters")]
    #[serde(
        alias = "$int_engine_size8_class5_name;",
        alias = "int_engine_size8_class5"
    )]
    ThrustersS8C5,
    #[strum(to_string = "Fighter Hangar")]
    #[serde(
        alias = "$int_fighterbay_size5_class1_name;",
        alias = "int_fighterbay_size5_class1"
    )]
    FighterHangarS5C1,
    #[strum(to_string = "Fighter Hangar")]
    #[serde(
        alias = "$int_fighterbay_size6_class1_name;",
        alias = "int_fighterbay_size6_class1"
    )]
    FighterHangarS6C1,
    #[strum(to_string = "Heatsink")]
    #[serde(
        alias = "$hpt_heatsinklauncher_turret_tiny_name;",
        alias = "hpt_heatsinklauncher_turret_tiny"
    )]
    Heatsink,
    #[strum(to_string = "Supercruise Assist")]
    #[serde(
        alias = "$int_supercruiseassist_name;",
        alias = "int_supercruiseassist"
    )]
    SupercruiseAssist,
    #[strum(to_string = "Fuel Transfer")]
    #[serde(
        alias = "$int_dronecontrol_fueltransfer_size1_class5_name;",
        alias = "int_dronecontrol_fueltransfer_size1_class5"
    )]
    FuelTransferS1C5,
    #[strum(to_string = "Fuel Transfer")]
    #[serde(
        alias = "$int_dronecontrol_fueltransfer_size3_class5_name;",
        alias = "int_dronecontrol_fueltransfer_size3_class5"
    )]
    FuelTransferS3C5,
    #[strum(to_string = "Collector")]
    #[serde(
        alias = "$int_dronecontrol_collection_size1_class2_name;",
        alias = "int_dronecontrol_collection_size1_class2"
    )]
    CollectorS1C2,
    #[strum(to_string = "Collector")]
    #[serde(
        alias = "$int_dronecontrol_collection_size1_class4_name;",
        alias = "int_dronecontrol_collection_size1_class4"
    )]
    CollectorS1C4,
    #[strum(to_string = "Collector")]
    #[serde(
        alias = "$int_dronecontrol_collection_size1_class5_name;",
        alias = "int_dronecontrol_collection_size1_class5"
    )]
    CollectorS1C5,
    #[strum(to_string = "Collector")]
    #[serde(
        alias = "$int_dronecontrol_collection_size3_class2_name;",
        alias = "int_dronecontrol_collection_size3_class2"
    )]
    CollectorS3C2,
    #[strum(to_string = "Collector")]
    #[serde(
        alias = "$int_dronecontrol_collection_size3_class3_name;",
        alias = "int_dronecontrol_collection_size3_class3"
    )]
    CollectorS3C3,
    #[strum(to_string = "Collector")]
    #[serde(
        alias = "$int_dronecontrol_collection_size3_class4_name;",
        alias = "int_dronecontrol_collection_size3_class4"
    )]
    CollectorS3C4,
    #[strum(to_string = "Collector")]
    #[serde(
        alias = "$int_dronecontrol_collection_size3_class5_name;",
        alias = "int_dronecontrol_collection_size3_class5"
    )]
    CollectorS3C5,
    #[strum(to_string = "Collector")]
    #[serde(
        alias = "$int_dronecontrol_collection_size5_class2_name;",
        alias = "int_dronecontrol_collection_size5_class2"
    )]
    CollectorS5C2,
    #[strum(to_string = "Collector")]
    #[serde(
        alias = "$int_dronecontrol_collection_size5_class3_name;",
        alias = "int_dronecontrol_collection_size5_class3"
    )]
    CollectorS5C3,
    #[strum(to_string = "Collector")]
    #[serde(
        alias = "$int_dronecontrol_collection_size5_class4_name;",
        alias = "int_dronecontrol_collection_size5_class4"
    )]
    CollectorS5C4,
    #[strum(to_string = "Collector")]
    #[serde(
        alias = "$int_dronecontrol_collection_size5_class5_name;",
        alias = "int_dronecontrol_collection_size5_class5"
    )]
    CollectorS5C5,
    #[strum(to_string = "Collector")]
    #[serde(
        alias = "$int_dronecontrol_collection_size7_class3_name;",
        alias = "int_dronecontrol_collection_size7_class3"
    )]
    CollectorS7C3,
    #[strum(to_string = "Collector")]
    #[serde(
        alias = "$int_dronecontrol_collection_size7_class5_name;",
        alias = "int_dronecontrol_collection_size7_class5"
    )]
    CollectorS7C5,
    #[strum(to_string = "Repair")]
    #[serde(
        alias = "$int_dronecontrol_repair_size1_class2_name;",
        alias = "int_dronecontrol_repair_size1_class2"
    )]
    RepairS1C2,
    #[strum(to_string = "Repair")]
    #[serde(
        alias = "$int_dronecontrol_repair_size1_class5_name;",
        alias = "int_dronecontrol_repair_size1_class5"
    )]
    RepairS1C5,
    #[strum(to_string = "Repair")]
    #[serde(
        alias = "$int_dronecontrol_repair_size3_class2_name;",
        alias = "int_dronecontrol_repair_size3_class2"
    )]
    RepairS3C2,
    #[strum(to_string = "Repair")]
    #[serde(
        alias = "$int_dronecontrol_repair_size3_class5_name;",
        alias = "int_dronecontrol_repair_size3_class5"
    )]
    RepairS3C5,
    #[strum(to_string = "Repair")]
    #[serde(
        alias = "$int_dronecontrol_repair_size5_class2_name;",
        alias = "int_dronecontrol_repair_size5_class2"
    )]
    RepairS5C2,
    #[strum(to_string = "Repair")]
    #[serde(
        alias = "$int_dronecontrol_repair_size5_class5_name;",
        alias = "int_dronecontrol_repair_size5_class5"
    )]
    RepairS5C5,
    #[strum(to_string = "Repair")]
    #[serde(
        alias = "$int_dronecontrol_repair_size7_class2_name;",
        alias = "int_dronecontrol_repair_size7_class2"
    )]
    RepairS7C2,
    #[strum(to_string = "Recon")]
    #[serde(
        alias = "$int_dronecontrol_recon_size1_class1_name;",
        alias = "int_dronecontrol_recon_size1_class1"
    )]
    ReconS1C1,
    #[strum(to_string = "Prospector")]
    #[serde(
        alias = "$int_dronecontrol_prospector_size1_class4_name;",
        alias = "int_dronecontrol_prospector_size1_class4"
    )]
    ProspectorS1C4,
    #[strum(to_string = "Prospector")]
    #[serde(
        alias = "$int_dronecontrol_prospector_size1_class5_name;",
        alias = "int_dronecontrol_prospector_size1_class5"
    )]
    ProspectorS1C5,
    #[strum(to_string = "Prospector")]
    #[serde(
        alias = "$int_dronecontrol_prospector_size3_class5_name;",
        alias = "int_dronecontrol_prospector_size3_class5"
    )]
    ProspectorS3C5,
    #[strum(to_string = "Research")]
    #[serde(
        alias = "$int_dronecontrol_unkvesselresearch_name;",
        alias = "int_dronecontrol_unkvesselresearch"
    )]
    Research,
    #[strum(to_string = "Docking Computer")]
    #[serde(
        alias = "$int_dockingcomputer_standard_name;",
        alias = "int_dockingcomputer_standard"
    )]
    StandardDockingComputer,
    #[strum(to_string = "Docking Computer")]
    #[serde(
        alias = "$int_dockingcomputer_advanced_name;",
        alias = "int_dockingcomputer_advanced"
    )]
    AdvancedDockingComputer,
    #[strum(to_string = "Corrosion Proof Cargo Rack")]
    #[serde(
        alias = "$int_corrosionproofcargorack_size4_class1_name;",
        alias = "int_corrosionproofcargorack_size4_class1"
    )]
    CorrosionProofCargoRackS4C1,
    #[strum(to_string = "Corrosion Proof Cargo Rack")]
    #[serde(
        alias = "$int_corrosionproofcargorack_size6_class1_name;",
        alias = "int_corrosionproofcargorack_size6_class1"
    )]
    CorrosionProofCargoRackS6C1,
    #[strum(to_string = "Cargo Rack")]
    #[serde(
        alias = "$int_cargorack_size1_class1_name;",
        alias = "int_cargorack_size1_class1"
    )]
    CargoRackS1C1,
    #[strum(to_string = "Cargo Rack")]
    #[serde(
        alias = "$int_cargorack_size2_class1_name;",
        alias = "int_cargorack_size2_class1"
    )]
    CargoRackS2C1,
    #[strum(to_string = "Cargo Rack")]
    #[serde(
        alias = "$int_cargorack_size3_class1_name;",
        alias = "int_cargorack_size3_class1"
    )]
    CargoRackS3C1,
    #[strum(to_string = "Cargo Rack")]
    #[serde(
        alias = "$int_cargorack_size4_class1_name;",
        alias = "int_cargorack_size4_class1"
    )]
    CargoRackS4C1,
    #[strum(to_string = "Cargo Rack")]
    #[serde(
        alias = "$int_cargorack_size5_class1_name;",
        alias = "int_cargorack_size5_class1"
    )]
    CargoRackS5C1,
    #[strum(to_string = "Cargo Rack")]
    #[serde(
        alias = "$int_cargorack_size6_class1_name;",
        alias = "int_cargorack_size6_class1"
    )]
    CargoRackS6C1,
    #[strum(to_string = "Cargo Rack")]
    #[serde(
        alias = "$int_cargorack_size7_class1_name;",
        alias = "int_cargorack_size7_class1"
    )]
    CargoRackS7C1,
    #[strum(to_string = "Cargo Rack")]
    #[serde(
        alias = "$int_largecargorack_size7_class1_name;",
        alias = "int_largecargorack_size7_class1"
    )]
    LargeCargoRackS7C1,
    #[strum(to_string = "Cargo Rack")]
    #[serde(
        alias = "$int_cargorack_size8_class1_name;",
        alias = "int_cargorack_size8_class1"
    )]
    CargoRackS8C1,
    #[strum(to_string = "Cargo Rack")]
    #[serde(
        alias = "$int_largecargorack_size8_class1_name;",
        alias = "int_largecargorack_size8_class1"
    )]
    LargeCargoRackS8C1,
    #[strum(to_string = "Cargo Hatch")]
    #[serde(
        alias = "$modularcargobaydoorfdl_name;",
        alias = "modularcargobaydoorfdl"
    )]
    CargoHatchFDL,
    #[strum(to_string = "Cargo Hatch")]
    #[serde(alias = "$modularcargobaydoor_name;", alias = "modularcargobaydoor")]
    CargoHatch,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size1_class1_name;",
        alias = "int_lifesupport_size1_class1"
    )]
    LifeSupportS1C1,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size1_class2_name;",
        alias = "int_lifesupport_size1_class2"
    )]
    LifeSupportS1C2,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size1_class3_name;",
        alias = "int_lifesupport_size1_class3"
    )]
    LifeSupportS1C3,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size1_class4_name;",
        alias = "int_lifesupport_size1_class4"
    )]
    LifeSupportS1C4,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size2_class1_name;",
        alias = "int_lifesupport_size2_class1"
    )]
    LifeSupportS2C1,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size2_class3_name;",
        alias = "int_lifesupport_size2_class3"
    )]
    LifeSupportS2C3,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size3_class1_name;",
        alias = "int_lifesupport_size3_class1"
    )]
    LifeSupportS3C1,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size3_class2_name;",
        alias = "int_lifesupport_size3_class2"
    )]
    LifeSupportS3C2,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size3_class3_name;",
        alias = "int_lifesupport_size3_class3"
    )]
    LifeSupportS3C3,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size3_class5_name;",
        alias = "int_lifesupport_size3_class5"
    )]
    LifeSupportS3C5,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size4_class4_name;",
        alias = "int_lifesupport_size4_class4"
    )]
    LifeSupportS4C4,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size4_class5_name;",
        alias = "int_lifesupport_size4_class5"
    )]
    LifeSupportS4C5,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size4_class2_name;",
        alias = "int_lifesupport_size4_class2"
    )]
    LifeSupportS4C2,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size4_class1_name;",
        alias = "int_lifesupport_size4_class1"
    )]
    LifeSupportS4C1,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size4_class3_name;",
        alias = "int_lifesupport_size4_class3"
    )]
    LifeSupportS4C3,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size5_class1_name;",
        alias = "int_lifesupport_size5_class1"
    )]
    LifeSupportS5C1,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size5_class2_name;",
        alias = "int_lifesupport_size5_class2"
    )]
    LifeSupportS5C2,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size5_class3_name;",
        alias = "int_lifesupport_size5_class3"
    )]
    LifeSupportS5C3,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size5_class5_name;",
        alias = "int_lifesupport_size5_class5"
    )]
    LifeSupportS5C5,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size6_class1_name;",
        alias = "int_lifesupport_size6_class1"
    )]
    LifeSupportS6C1,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size6_class2_name;",
        alias = "int_lifesupport_size6_class2"
    )]
    LifeSupportS6C2,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size6_class3_name;",
        alias = "int_lifesupport_size6_class3"
    )]
    LifeSupportS6C3,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size7_class1_name;",
        alias = "int_lifesupport_size7_class1"
    )]
    LifeSupportS7C1,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size7_class2_name;",
        alias = "int_lifesupport_size7_class2"
    )]
    LifeSupportS7C2,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size7_class3_name;",
        alias = "int_lifesupport_size7_class3"
    )]
    LifeSupportS7C3,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size7_class5_name;",
        alias = "int_lifesupport_size7_class5"
    )]
    LifeSupportS7C5,
    #[strum(to_string = "Life Support")]
    #[serde(
        alias = "$int_lifesupport_size8_class2_name;",
        alias = "int_lifesupport_size8_class2"
    )]
    LifeSupportS8C2,
    #[strum(to_string = "Fuel Scoop")]
    #[serde(
        alias = "$int_fuelscoop_size1_class1_name;",
        alias = "int_fuelscoop_size1_class1"
    )]
    FuelScoopS1C1,
    #[strum(to_string = "Fuel Scoop")]
    #[serde(
        alias = "$int_fuelscoop_size1_class3_name;",
        alias = "int_fuelscoop_size1_class3"
    )]
    FuelScoopS1C3,
    #[strum(to_string = "Fuel Scoop")]
    #[serde(
        alias = "$int_fuelscoop_size1_class4_name;",
        alias = "int_fuelscoop_size1_class4"
    )]
    FuelScoopS1C4,
    #[strum(to_string = "Fuel Scoop")]
    #[serde(
        alias = "$int_fuelscoop_size2_class5_name;",
        alias = "int_fuelscoop_size2_class5"
    )]
    FuelScoopS2C5,
    #[strum(to_string = "Fuel Scoop")]
    #[serde(
        alias = "$int_fuelscoop_size1_class5_name;",
        alias = "int_fuelscoop_size1_class5"
    )]
    FuelScoopS1C5,
    #[strum(to_string = "Fuel Scoop")]
    #[serde(
        alias = "$int_fuelscoop_size3_class1_name;",
        alias = "int_fuelscoop_size3_class1"
    )]
    FuelScoopS3C1,
    #[strum(to_string = "Fuel Scoop")]
    #[serde(
        alias = "$int_fuelscoop_size3_class2_name;",
        alias = "int_fuelscoop_size3_class2"
    )]
    FuelScoopS3C2,
    #[strum(to_string = "Fuel Scoop")]
    #[serde(
        alias = "$int_fuelscoop_size3_class5_name;",
        alias = "int_fuelscoop_size3_class5"
    )]
    FuelScoopS3C5,
    #[strum(to_string = "Fuel Scoop")]
    #[serde(
        alias = "$int_fuelscoop_size4_class2_name;",
        alias = "int_fuelscoop_size4_class2"
    )]
    FuelScoopS4C2,
    #[strum(to_string = "Fuel Scoop")]
    #[serde(
        alias = "$int_fuelscoop_size4_class5_name;",
        alias = "int_fuelscoop_size4_class5"
    )]
    FuelScoopS4C5,
    #[strum(to_string = "Fuel Scoop")]
    #[serde(
        alias = "$int_fuelscoop_size5_class5_name;",
        alias = "int_fuelscoop_size5_class5"
    )]
    FuelScoopS5C5,
    #[strum(to_string = "Fuel Scoop")]
    #[serde(
        alias = "$int_fuelscoop_size6_class4_name;",
        alias = "int_fuelscoop_size6_class4"
    )]
    FuelScoopS6C4,
    #[strum(to_string = "Fuel Scoop")]
    #[serde(
        alias = "$int_fuelscoop_size6_class5_name;",
        alias = "int_fuelscoop_size6_class5"
    )]
    FuelScoopS6C5,
    #[strum(to_string = "Fuel Scoop")]
    #[serde(
        alias = "$int_fuelscoop_size7_class5_name;",
        alias = "int_fuelscoop_size7_class5"
    )]
    FuelScoopS7C5,
    #[strum(to_string = "Guardian Hull Reinforcement Package")]
    #[serde(
        alias = "$int_guardianhullreinforcement_size1_class2_name;",
        alias = "int_guardianhullreinforcement_size1_class2"
    )]
    GuardianHullReinforcementS1C2,
    #[strum(to_string = "Guardian Hull Reinforcement Package")]
    #[serde(
        alias = "$int_guardianhullreinforcement_size2_class2_name;",
        alias = "int_guardianhullreinforcement_size2_class2"
    )]
    GuardianHullReinforcementS2C2,
    #[strum(to_string = "Guardian Hull Reinforcement Package")]
    #[serde(
        alias = "$int_guardianhullreinforcement_size3_class2_name;",
        alias = "int_guardianhullreinforcement_size3_class2"
    )]
    GuardianHullReinforcementS3C2,
    #[strum(to_string = "Guardian Hull Reinforcement Package")]
    #[serde(
        alias = "$int_guardianhullreinforcement_size5_class2_name;",
        alias = "int_guardianhullreinforcement_size5_class2"
    )]
    GuardianHullReinforcementS5C2,
    #[strum(to_string = "Hull Reinforcement Package")]
    #[serde(
        alias = "$int_hullreinforcement_size1_class2_name;",
        alias = "int_hullreinforcement_size1_class2"
    )]
    HullReinforcementS1C2,
    #[strum(to_string = "Hull Reinforcement Package")]
    #[serde(
        alias = "$int_hullreinforcement_size2_class2_name;",
        alias = "int_hullreinforcement_size2_class2"
    )]
    HullReinforcementS2C2,
    #[strum(to_string = "Hull Reinforcement Package")]
    #[serde(
        alias = "$int_hullreinforcement_size2_class1_name;",
        alias = "int_hullreinforcement_size2_class1"
    )]
    HullReinforcementS2C1,
    #[strum(to_string = "Hull Reinforcement Package")]
    #[serde(
        alias = "$int_hullreinforcement_size3_class2_name;",
        alias = "int_hullreinforcement_size3_class2"
    )]
    HullReinforcementS3C2,
    #[strum(to_string = "Hull Reinforcement Package")]
    #[serde(
        alias = "$int_hullreinforcement_size4_class2_name;",
        alias = "int_hullreinforcement_size4_class2"
    )]
    HullReinforcementS4C2,
    #[strum(to_string = "Hull Reinforcement Package")]
    #[serde(
        alias = "$int_hullreinforcement_size5_class1_name;",
        alias = "int_hullreinforcement_size5_class1"
    )]
    HullReinforcementS5C1,
    #[strum(to_string = "Hull Reinforcement Package")]
    #[serde(
        alias = "$int_hullreinforcement_size5_class2_name;",
        alias = "int_hullreinforcement_size5_class2"
    )]
    HullReinforcementS5C2,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size1_class1_name;",
        alias = "int_sensors_size1_class1"
    )]
    SensorsS1C1,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size1_class5_name;",
        alias = "int_sensors_size1_class5"
    )]
    SensorsS1C5,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size2_class1_name;",
        alias = "int_sensors_size2_class1"
    )]
    SensorsS2C1,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size2_class2_name;",
        alias = "int_sensors_size2_class2"
    )]
    SensorsS2C2,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size2_class3_name;",
        alias = "int_sensors_size2_class3"
    )]
    SensorsS2C3,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size2_class5_name;",
        alias = "int_sensors_size2_class5"
    )]
    SensorsS2C5,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size3_class1_name;",
        alias = "int_sensors_size3_class1"
    )]
    SensorsS3C1,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size3_class2_name;",
        alias = "int_sensors_size3_class2"
    )]
    SensorsS3C2,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size3_class3_name;",
        alias = "int_sensors_size3_class3"
    )]
    SensorsS3C3,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size3_class5_name;",
        alias = "int_sensors_size3_class5"
    )]
    SensorsS3C5,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size4_class2_name;",
        alias = "int_sensors_size4_class2"
    )]
    SensorsS4C2,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size4_class3_name;",
        alias = "int_sensors_size4_class3"
    )]
    SensorsS4C3,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size4_class5_name;",
        alias = "int_sensors_size4_class5"
    )]
    SensorsS4C5,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size4_class1_name;",
        alias = "int_sensors_size4_class1"
    )]
    SensorsS4C1,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size5_class1_name;",
        alias = "int_sensors_size5_class1"
    )]
    SensorsS5C1,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size5_class2_name;",
        alias = "int_sensors_size5_class2"
    )]
    SensorsS5C2,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size5_class3_name;",
        alias = "int_sensors_size5_class3"
    )]
    SensorsS5C3,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size6_class1_name;",
        alias = "int_sensors_size6_class1"
    )]
    SensorsS6C1,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size6_class2_name;",
        alias = "int_sensors_size6_class2"
    )]
    SensorsS6C2,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size6_class5_name;",
        alias = "int_sensors_size6_class5"
    )]
    SensorsS6C5,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size7_class1_name;",
        alias = "int_sensors_size7_class1"
    )]
    SensorsS7C1,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size7_class2_name;",
        alias = "int_sensors_size7_class2"
    )]
    SensorsS7C2,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size7_class5_name;",
        alias = "int_sensors_size7_class5"
    )]
    SensorsS7C5,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size8_class1_name;",
        alias = "int_sensors_size8_class1"
    )]
    SensorsS8C1,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size8_class5_name;",
        alias = "int_sensors_size8_class5"
    )]
    SensorsS8C5,
    #[strum(to_string = "Sensors")]
    #[serde(
        alias = "$int_sensors_size8_class2_name;",
        alias = "int_sensors_size8_class2"
    )]
    SensorsS8C2,
    #[strum(to_string = "Mk II Passenger Cabin")]
    #[serde(
        alias = "$int_mkii_passengercabin_size2_class1_name;",
        alias = "int_mkii_passengercabin_size2_class1"
    )]
    MkIIPassengerCabinS2C1,
    #[strum(to_string = "Passenger Cabin")]
    #[serde(
        alias = "$int_passengercabin_size2_class1_name;",
        alias = "int_passengercabin_size2_class1"
    )]
    PassengerCabinS2C1,
    #[strum(to_string = "Passenger Cabin")]
    #[serde(
        alias = "$int_passengercabin_size3_class1_name;",
        alias = "int_passengercabin_size3_class1"
    )]
    PassengerCabinS3C1,
    #[strum(to_string = "Passenger Cabin")]
    #[serde(
        alias = "$int_passengercabin_size3_class2_name;",
        alias = "int_passengercabin_size3_class2"
    )]
    PassengerCabinS3C2,
    #[strum(to_string = "Passenger Cabin")]
    #[serde(
        alias = "$int_passengercabin_size4_class1_name;",
        alias = "int_passengercabin_size4_class1"
    )]
    PassengerCabinS4C1,
    #[strum(to_string = "Passenger Cabin")]
    #[serde(
        alias = "$int_passengercabin_size4_class2_name;",
        alias = "int_passengercabin_size4_class2"
    )]
    PassengerCabinS4C2,
    #[strum(to_string = "Mk II Passenger Cabin")]
    #[serde(
        alias = "$int_mkii_passengercabin_size4_class1_name;",
        alias = "int_mkii_passengercabin_size4_class1"
    )]
    MkIIPassengerCabinS4C1,
    #[strum(to_string = "Mk II Passenger Cabin")]
    #[serde(
        alias = "$int_mkii_passengercabin_size4_class2_name;",
        alias = "int_mkii_passengercabin_size4_class2"
    )]
    MkIIPassengerCabinS4C2,
    #[strum(to_string = "Passenger Cabin")]
    #[serde(
        alias = "$int_passengercabin_size5_class1_name;",
        alias = "int_passengercabin_size5_class1"
    )]
    PassengerCabinS5C1,
    #[strum(to_string = "Mk II Passenger Cabin")]
    #[serde(
        alias = "$int_mkii_passengercabin_size5_class1_name;",
        alias = "int_mkii_passengercabin_size5_class1"
    )]
    MkIIPassengerCabinS5C1,
    #[strum(to_string = "Passenger Cabin")]
    #[serde(
        alias = "$int_passengercabin_size5_class2_name;",
        alias = "int_passengercabin_size5_class2"
    )]
    PassengerCabinS5C2,
    #[strum(to_string = "Mk II Passenger Cabin")]
    #[serde(
        alias = "$int_mkii_passengercabin_size5_class2_name;",
        alias = "int_mkii_passengercabin_size5_class2"
    )]
    MkIIPassengerCabinS5C2,
    #[strum(to_string = "Mk II Passenger Cabin")]
    #[serde(
        alias = "$int_mkii_passengercabin_size6_class1_name;",
        alias = "int_mkii_passengercabin_size6_class1"
    )]
    MkIIPassengerCabinS6C1,
    #[strum(to_string = "Mk II Passenger Cabin")]
    #[serde(
        alias = "$int_mkii_passengercabin_size6_class2_name;",
        alias = "int_mkii_passengercabin_size6_class2"
    )]
    MkIIPassengerCabinS6C2,
    #[strum(to_string = "Passenger Cabin")]
    #[serde(
        alias = "$int_passengercabin_size6_class2_name;",
        alias = "int_passengercabin_size6_class2"
    )]
    PassengerCabinS6C2,
    #[strum(to_string = "Passenger Cabin")]
    #[serde(
        alias = "$int_passengercabin_size6_class3_name;",
        alias = "int_passengercabin_size6_class3"
    )]
    PassengerCabinS6C3,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size1_class2_name;",
        alias = "int_powerdistributor_size1_class2"
    )]
    PowerDistributorS1C2,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size1_class1_name;",
        alias = "int_powerdistributor_size1_class1"
    )]
    PowerDistributorS1C1,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size1_class5_name;",
        alias = "int_powerdistributor_size1_class5"
    )]
    PowerDistributorS1C5,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size2_class1_name;",
        alias = "int_powerdistributor_size2_class1"
    )]
    PowerDistributor2C1,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size2_class2_name;",
        alias = "int_powerdistributor_size2_class2"
    )]
    PowerDistributor2C2,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size2_class3_name;",
        alias = "int_powerdistributor_size2_class3"
    )]
    PowerDistributor2C3,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size2_class5_name;",
        alias = "int_powerdistributor_size2_class5"
    )]
    PowerDistributor2C5,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size3_class1_name;",
        alias = "int_powerdistributor_size3_class1"
    )]
    PowerDistributor3C1,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size3_class2_name;",
        alias = "int_powerdistributor_size3_class2"
    )]
    PowerDistributor3C2,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size3_class3_name;",
        alias = "int_powerdistributor_size3_class3"
    )]
    PowerDistributor3C3,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size4_class1_name;",
        alias = "int_powerdistributor_size4_class1"
    )]
    PowerDistributor4C1,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size4_class2_name;",
        alias = "int_powerdistributor_size4_class2"
    )]
    PowerDistributor4C2,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size4_class3_name;",
        alias = "int_powerdistributor_size4_class3"
    )]
    PowerDistributor4C3,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size4_class5_name;",
        alias = "int_powerdistributor_size4_class5"
    )]
    PowerDistributorS4C5,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size5_class1_name;",
        alias = "int_powerdistributor_size5_class1"
    )]
    PowerDistributorS5C1,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size5_class2_name;",
        alias = "int_powerdistributor_size5_class2"
    )]
    PowerDistributorS5C2,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size5_class5_name;",
        alias = "int_powerdistributor_size5_class5"
    )]
    PowerDistributorS5C5,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size6_class3_name;",
        alias = "int_powerdistributor_size6_class3"
    )]
    PowerDistributorS6C3,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size6_class5_name;",
        alias = "int_powerdistributor_size6_class5"
    )]
    PowerDistributorS6C5,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size7_class1_name;",
        alias = "int_powerdistributor_size7_class1"
    )]
    PowerDistributorS7C1,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size7_class2_name;",
        alias = "int_powerdistributor_size7_class2"
    )]
    PowerDistributorS7C2,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size7_class3_name;",
        alias = "int_powerdistributor_size7_class3"
    )]
    PowerDistributorS7C3,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size7_class5_name;",
        alias = "int_powerdistributor_size7_class5"
    )]
    PowerDistributorS7C5,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size8_class1_name;",
        alias = "int_powerdistributor_size8_class1"
    )]
    PowerDistributorS8C1,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size8_class3_name;",
        alias = "int_powerdistributor_size8_class3"
    )]
    PowerDistributorS8C3,
    #[strum(to_string = "Planetary Approach Suite")]
    #[serde(
        alias = "$int_planetapproachsuite_name;",
        alias = "int_planetapproachsuite"
    )]
    PlanetaryApproachSuite,
    #[strum(to_string = "Advanced Planetary Approach Suite")]
    #[serde(
        alias = "$int_planetapproachsuite_advanced_name;",
        alias = "int_planetapproachsuite_advanced"
    )]
    PlanetaryApproachSuiteAdvanced,
    #[strum(to_string = "Planetary Vehicle Hangar")]
    #[serde(
        alias = "$int_buggybay_size2_class1_name;",
        alias = "int_buggybay_size2_class1"
    )]
    PlanetaryVehicleHangarS2C1,
    #[strum(to_string = "Planetary Vehicle Hangar")]
    #[serde(
        alias = "$int_buggybay_size2_class2_name;",
        alias = "int_buggybay_size2_class2"
    )]
    PlanetaryVehicleHangarS2C2,
    #[strum(to_string = "Planetary Vehicle Hangar")]
    #[serde(
        alias = "$int_buggybay_size4_class2_name;",
        alias = "int_buggybay_size4_class2"
    )]
    PlanetaryVehicleHangarS4C2,
    #[strum(to_string = "Planetary Vehicle Hangar")]
    #[serde(
        alias = "$int_buggybay_size6_class2_name;",
        alias = "int_buggybay_size6_class2"
    )]
    PlanetaryVehicleHangarS6C2,
    #[strum(to_string = "Refinery")]
    #[serde(
        alias = "$int_refinery_size1_class3_name;",
        alias = "int_refinery_size1_class3"
    )]
    RefineryS1C3,
    #[strum(to_string = "Refinery")]
    #[serde(
        alias = "$int_refinery_size2_class1_name;",
        alias = "int_refinery_size2_class1"
    )]
    RefineryS2C1,
    #[strum(to_string = "Refinery")]
    #[serde(
        alias = "$int_refinery_size2_class3_name;",
        alias = "int_refinery_size2_class3"
    )]
    RefineryS2C3,
    #[strum(to_string = "Refinery")]
    #[serde(
        alias = "$int_refinery_size2_class4_name;",
        alias = "int_refinery_size2_class4"
    )]
    RefineryS2C4,
    #[strum(to_string = "Refinery")]
    #[serde(
        alias = "$int_refinery_size2_class5_name;",
        alias = "int_refinery_size2_class5"
    )]
    RefineryS2C5,
    #[strum(to_string = "Refinery")]
    #[serde(
        alias = "$int_refinery_size3_class1_name;",
        alias = "int_refinery_size3_class1"
    )]
    RefineryS3C1,
    #[strum(to_string = "Refinery")]
    #[serde(
        alias = "$int_refinery_size3_class3_name;",
        alias = "int_refinery_size3_class3"
    )]
    RefineryS3C3,
    #[strum(to_string = "Refinery")]
    #[serde(
        alias = "$int_refinery_size3_class5_name;",
        alias = "int_refinery_size3_class5"
    )]
    RefineryS3C5,
    #[strum(to_string = "Remote Flak")]
    #[serde(
        alias = "$hpt_flakmortar_fixed_medium_name;",
        alias = "hpt_flakmortar_fixed_medium"
    )]
    RemoteFlakFixedMedium,
    #[strum(to_string = "Remote Flak")]
    #[serde(
        alias = "$hpt_flakmortar_turret_medium_name;",
        alias = "hpt_flakmortar_turret_medium"
    )]
    RemoteFlakTurretMedium,
    #[strum(to_string = "AFM Unit")]
    #[serde(
        alias = "$int_repairer_size1_class5_name;",
        alias = "int_repairer_size1_class5"
    )]
    AFMUnitS1C5,
    #[strum(to_string = "AFM Unit")]
    #[serde(
        alias = "$int_repairer_size2_class5_name;",
        alias = "int_repairer_size2_class5"
    )]
    AFMUnitS2C5,
    #[strum(to_string = "AFM Unit")]
    #[serde(
        alias = "$int_repairer_size2_class4_name;",
        alias = "int_repairer_size2_class4"
    )]
    AFMUnitS2C4,
    #[strum(to_string = "AFM Unit")]
    #[serde(
        alias = "$int_repairer_size3_class3_name;",
        alias = "int_repairer_size3_class3"
    )]
    AFMUnitS3C3,
    #[strum(to_string = "AFM Unit")]
    #[serde(
        alias = "$int_repairer_size3_class4_name;",
        alias = "int_repairer_size3_class4"
    )]
    AFMUnitS3C4,
    #[strum(to_string = "AFM Unit")]
    #[serde(
        alias = "$int_repairer_size3_class5_name;",
        alias = "int_repairer_size3_class5"
    )]
    AFMUnitS3C5,
    #[strum(to_string = "AFM Unit")]
    #[serde(
        alias = "$int_repairer_size4_class4_name;",
        alias = "int_repairer_size4_class4"
    )]
    AFMUnitS4C4,
    #[strum(to_string = "AFM Unit")]
    #[serde(
        alias = "$int_repairer_size4_class5_name;",
        alias = "int_repairer_size4_class5"
    )]
    AFMUnitS4C5,
    #[strum(to_string = "AFM Unit")]
    #[serde(
        alias = "$int_repairer_size5_class2_name;",
        alias = "int_repairer_size5_class2"
    )]
    AFMUnitS5C2,
    #[strum(to_string = "AFM Unit")]
    #[serde(
        alias = "$int_repairer_size5_class5_name;",
        alias = "int_repairer_size5_class5"
    )]
    AFMUnitS5C5,
    #[strum(to_string = "AFM Unit")]
    #[serde(
        alias = "$int_repairer_size6_class4_name;",
        alias = "int_repairer_size6_class4"
    )]
    AFMUnitS6C4,
    #[strum(to_string = "AFM Unit")]
    #[serde(
        alias = "$int_repairer_size6_class5_name;",
        alias = "int_repairer_size6_class5"
    )]
    AFMUnitS6C5,
    #[strum(to_string = "AFM Unit")]
    #[serde(
        alias = "$int_repairer_size7_class5_name;",
        alias = "int_repairer_size7_class5"
    )]
    AFMUnitS7C5,
    #[strum(to_string = "Bi-Weave Shield")]
    #[serde(
        alias = "$int_shieldgenerator_size2_class3_fast_name;",
        alias = "int_shieldgenerator_size2_class3_fast"
    )]
    BiWeaveShieldS2C3,
    #[strum(to_string = "Bi-Weave Shield")]
    #[serde(
        alias = "$int_shieldgenerator_size3_class3_fast_name;",
        alias = "int_shieldgenerator_size3_class3_fast"
    )]
    BiWeaveShieldS3C3,
    #[strum(to_string = "Bi-Weave Shield")]
    #[serde(
        alias = "$int_shieldgenerator_size4_class3_fast_name;",
        alias = "int_shieldgenerator_size4_class3_fast"
    )]
    BiWeaveShieldS4C3,
    #[strum(to_string = "Bi-Weave Shield")]
    #[serde(
        alias = "$int_shieldgenerator_size5_class3_fast_name;",
        alias = "int_shieldgenerator_size5_class3_fast"
    )]
    BiWeaveShieldS5C3,
    #[strum(to_string = "Bi-Weave Shield")]
    #[serde(
        alias = "$int_shieldgenerator_size6_class3_fast_name;",
        alias = "int_shieldgenerator_size6_class3_fast"
    )]
    BiWeaveShieldS6C3,
    #[strum(to_string = "Bi-Weave Shield")]
    #[serde(
        alias = "$int_shieldgenerator_size7_class3_fast_name;",
        alias = "int_shieldgenerator_size7_class3_fast"
    )]
    BiWeaveShieldS7C3,
    #[strum(to_string = "Bi-Weave Shield")]
    #[serde(
        alias = "$int_shieldgenerator_size8_class3_fast_name;",
        alias = "int_shieldgenerator_size8_class3_fast"
    )]
    BiWeaveShieldS8C3,
    #[strum(to_string = "Prismatic Shield")]
    #[serde(
        alias = "$int_shieldgenerator_size3_class5_strong_name;",
        alias = "int_shieldgenerator_size3_class5_strong"
    )]
    PrismaticShieldS3C5,
    #[strum(to_string = "Prismatic Shield")]
    #[serde(
        alias = "$int_shieldgenerator_size4_class5_strong_name;",
        alias = "int_shieldgenerator_size4_class5_strong"
    )]
    PrismaticShieldS4C5,
    #[strum(to_string = "Prismatic Shield")]
    #[serde(
        alias = "$int_shieldgenerator_size5_class5_strong_name;",
        alias = "int_shieldgenerator_size5_class5_strong"
    )]
    PrismaticShieldS5C5,
    #[strum(to_string = "Prismatic Shield")]
    #[serde(
        alias = "$int_shieldgenerator_size6_class5_strong_name;",
        alias = "int_shieldgenerator_size6_class5_strong"
    )]
    PrismaticShieldS6C5,
    #[strum(to_string = "Prismatic Shield")]
    #[serde(
        alias = "$int_shieldgenerator_size7_class5_strong_name;",
        alias = "int_shieldgenerator_size7_class5_strong"
    )]
    PrismaticShieldS7C5,
    #[strum(to_string = "Prismatic Shield")]
    #[serde(
        alias = "$int_shieldgenerator_size8_class5_strong_name;",
        alias = "int_shieldgenerator_size8_class5_strong"
    )]
    PrismaticShieldS8C5,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size2_class1_name;",
        alias = "int_shieldgenerator_size2_class1"
    )]
    ShieldGeneratorS2C1,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size2_class4_name;",
        alias = "int_shieldgenerator_size2_class4"
    )]
    ShieldGeneratorS2C4,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size2_class5_name;",
        alias = "int_shieldgenerator_size2_class5"
    )]
    ShieldGeneratorS2C5,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size3_class1_name;",
        alias = "int_shieldgenerator_size3_class1"
    )]
    ShieldGeneratorS3C1,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size3_class2_name;",
        alias = "int_shieldgenerator_size3_class2"
    )]
    ShieldGeneratorS3C2,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size3_class3_name;",
        alias = "int_shieldgenerator_size3_class3"
    )]
    ShieldGeneratorS3C3,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size3_class4_name;",
        alias = "int_shieldgenerator_size3_class4"
    )]
    ShieldGeneratorS3C4,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size3_class5_name;",
        alias = "int_shieldgenerator_size3_class5"
    )]
    ShieldGeneratorS3C5,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size4_class1_name;",
        alias = "int_shieldgenerator_size4_class1"
    )]
    ShieldGeneratorS4C1,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size4_class2_name;",
        alias = "int_shieldgenerator_size4_class2"
    )]
    ShieldGeneratorS4C2,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size4_class3_name;",
        alias = "int_shieldgenerator_size4_class3"
    )]
    ShieldGeneratorS4C3,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size4_class4_name;",
        alias = "int_shieldgenerator_size4_class4"
    )]
    ShieldGeneratorS4C4,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size4_class5_name;",
        alias = "int_shieldgenerator_size4_class5"
    )]
    ShieldGeneratorS4C5,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size5_class1_name;",
        alias = "int_shieldgenerator_size5_class1"
    )]
    ShieldGeneratorS5C1,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size5_class2_name;",
        alias = "int_shieldgenerator_size5_class2"
    )]
    ShieldGeneratorS5C2,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size5_class3_name;",
        alias = "int_shieldgenerator_size5_class3"
    )]
    ShieldGeneratorS5C3,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size5_class4_name;",
        alias = "int_shieldgenerator_size5_class4"
    )]
    ShieldGeneratorS5C4,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size5_class5_name;",
        alias = "int_shieldgenerator_size5_class5"
    )]
    ShieldGeneratorS5C5,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size6_class1_name;",
        alias = "int_shieldgenerator_size6_class1"
    )]
    ShieldGeneratorS6C1,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size6_class2_name;",
        alias = "int_shieldgenerator_size6_class2"
    )]
    ShieldGeneratorS6C2,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size6_class3_name;",
        alias = "int_shieldgenerator_size6_class3"
    )]
    ShieldGeneratorS6C3,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size6_class4_name;",
        alias = "int_shieldgenerator_size6_class4"
    )]
    ShieldGeneratorS6C4,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size6_class5_name;",
        alias = "int_shieldgenerator_size6_class5"
    )]
    ShieldGeneratorS6C5,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size7_class1_name;",
        alias = "int_shieldgenerator_size7_class1"
    )]
    ShieldGeneratorS7C1,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size7_class4_name;",
        alias = "int_shieldgenerator_size7_class4"
    )]
    ShieldGeneratorS7C4,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size7_class5_name;",
        alias = "int_shieldgenerator_size7_class5"
    )]
    ShieldGeneratorS7C5,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size8_class1_name;",
        alias = "int_shieldgenerator_size8_class1"
    )]
    ShieldGeneratorS8C1,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size8_class2_name;",
        alias = "int_shieldgenerator_size8_class2"
    )]
    ShieldGeneratorS8C2,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size8_class4_name;",
        alias = "int_shieldgenerator_size8_class4"
    )]
    ShieldGeneratorS8C4,
    #[strum(to_string = "Shield Generator")]
    #[serde(
        alias = "$int_shieldgenerator_size8_class5_name;",
        alias = "int_shieldgenerator_size8_class5"
    )]
    ShieldGeneratorS8C5,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$adder_cockpit_name;", alias = "adder_cockpit")]
    AdderCockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$anaconda_cockpit_name;", alias = "anaconda_cockpit")]
    AnacondaCockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$asp_cockpit_name;", alias = "asp_cockpit")]
    AspCockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$belugaliner_cockpit_name;", alias = "belugaliner_cockpit")]
    BelugaLinerCockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$explorer_nx_cockpit_name;", alias = "explorer_nx_cockpit")]
    CaspianExplorerCockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$cobramkiii_cockpit_name;", alias = "cobramkiii_cockpit")]
    CobraMkIIICockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$cobramkv_cockpit_name;", alias = "cobramkv_cockpit")]
    CobraMkVCockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$corsair_cockpit_name;", alias = "corsair_cockpit")]
    CorsairCockpit,
    #[serde(alias = "$cutter_cockpit_name;", alias = "cutter_cockpit")]
    CutterCockpit,
    #[serde(
        alias = "$diamondbackxl_cockpit_name;",
        alias = "diamondbackxl_cockpit"
    )]
    DiamondBackXLCockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$empire_eagle_cockpit_name;", alias = "empire_eagle_cockpit")]
    EmpireEagleCockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(
        alias = "$empire_courier_cockpit_name;",
        alias = "empire_courier_cockpit"
    )]
    EmpireCourierCockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(
        alias = "$empire_trader_cockpit_name;",
        alias = "empire_trader_cockpit"
    )]
    EmpireTraderCockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(
        alias = "$federation_corvette_cockpit_name;",
        alias = "federation_corvette_cockpit"
    )]
    FederationCorvetteCockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$ferdelance_cockpit_name;", alias = "ferdelance_cockpit")]
    FerdelanceCockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$krait_light_cockpit_name;", alias = "krait_light_cockpit")]
    KraitLightCockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$krait_mkii_cockpit_name;", alias = "krait_mkii_cockpit")]
    KraitMkIICockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$lakonminer_cockpit_name;", alias = "lakonminer_cockpit")]
    LakonMinerCockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(
        alias = "$mediumtransport01_cockpit_name;",
        alias = "mediumtransport01_cockpit"
    )]
    LynxHighlinerCockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$mamba_cockpit_name;", alias = "mamba_cockpit")]
    MambaCockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$mandalay_cockpit_name;", alias = "mandalay_cockpit")]
    MandalayCockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$python_nx_cockpit_name;", alias = "python_nx_cockpit")]
    PythonMkIICockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$panthermkii_cockpit_name;", alias = "panthermkii_cockpit")]
    PantherMkIICockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$python_cockpit_name;", alias = "python_cockpit")]
    PythonCockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$sidewinder_cockpit_name;", alias = "sidewinder_cockpit")]
    SidewinderCockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$type6_cockpit_name;", alias = "type6_cockpit")]
    Type6Cockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$type7_cockpit_name;", alias = "type7_cockpit")]
    Type7Cockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$type8_cockpit_name;", alias = "type8_cockpit")]
    Type8Cockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$type9_cockpit_name;", alias = "type9_cockpit")]
    Type9Cockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(
        alias = "$type9_military_cockpit_name;",
        alias = "type9_military_cockpit"
    )]
    Type9MilitaryCockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$typex_cockpit_name;", alias = "typex_cockpit")]
    TypeXCockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$vulture_cockpit_name;", alias = "vulture_cockpit")]
    VultureCockpit,
    #[strum(to_string = "K-Warrant Scanner")]
    #[serde(
        alias = "$hpt_crimescanner_size0_class2_name;",
        alias = "hpt_crimescanner_size0_class2"
    )]
    KWarrantScannerS0C2,
    #[strum(to_string = "K-Warrant Scanner")]
    #[serde(
        alias = "$hpt_crimescanner_size0_class3_name;",
        alias = "hpt_crimescanner_size0_class3"
    )]
    KWarrantScannerS0C3,
    #[strum(to_string = "K-Warrant Scanner")]
    #[serde(
        alias = "$hpt_crimescanner_size0_class4_name;",
        alias = "hpt_crimescanner_size0_class4"
    )]
    KWarrantScannerS0C4,
    #[strum(to_string = "K-Warrant Scanner")]
    #[serde(
        alias = "$hpt_crimescanner_size0_class5_name;",
        alias = "hpt_crimescanner_size0_class5"
    )]
    KWarrantScannerS0C5,
    #[strum(to_string = "Rail Gun")]
    #[serde(
        alias = "$hpt_railgun_fixed_medium_burst_name;",
        alias = "hpt_railgun_fixed_medium_burst"
    )]
    RailGunFixedMediumBurst,
    #[strum(to_string = "Rail Gun")]
    #[serde(
        alias = "$hpt_railgun_fixed_small_name;",
        alias = "hpt_railgun_fixed_small"
    )]
    RailGunFixedSmall,
    #[strum(to_string = "Rail Gun")]
    #[serde(
        alias = "$hpt_railgun_fixed_medium_name;",
        alias = "hpt_railgun_fixed_medium"
    )]
    RailGunFixedMedium,
    #[strum(to_string = "Guardian Module Reinforcement")]
    #[serde(
        alias = "$int_guardianmodulereinforcement_size1_class2_name;",
        alias = "int_guardianmodulereinforcement_size1_class2"
    )]
    GuardianModuleReinforcementS1C2,
    #[strum(to_string = "Guardian Module Reinforcement")]
    #[serde(
        alias = "$int_guardianmodulereinforcement_size2_class1_name;",
        alias = "int_guardianmodulereinforcement_size2_class1"
    )]
    GuardianModuleReinforcementS2C1,
    #[strum(to_string = "Guardian Module Reinforcement")]
    #[serde(
        alias = "$int_guardianmodulereinforcement_size2_class2_name;",
        alias = "int_guardianmodulereinforcement_size2_class2"
    )]
    GuardianModuleReinforcementS2C2,
    #[strum(to_string = "Guardian Module Reinforcement")]
    #[serde(
        alias = "$int_guardianmodulereinforcement_size3_class2_name;",
        alias = "int_guardianmodulereinforcement_size3_class2"
    )]
    GuardianModuleReinforcementS3C2,
    #[strum(to_string = "Guardian Module Reinforcement")]
    #[serde(
        alias = "$int_guardianmodulereinforcement_size4_class1_name;",
        alias = "int_guardianmodulereinforcement_size4_class1"
    )]
    GuardianModuleReinforcementS4C1,
    #[strum(to_string = "Guardian Module Reinforcement")]
    #[serde(
        alias = "$int_guardianmodulereinforcement_size4_class2_name;",
        alias = "int_guardianmodulereinforcement_size4_class2"
    )]
    GuardianModuleReinforcementS4C2,
    #[strum(to_string = "Guardian Module Reinforcement")]
    #[serde(
        alias = "$int_guardianmodulereinforcement_size5_class1_name;",
        alias = "int_guardianmodulereinforcement_size5_class1"
    )]
    GuardianModuleReinforcementS5C1,
    #[strum(to_string = "Guardian Module Reinforcement")]
    #[serde(
        alias = "$int_guardianmodulereinforcement_size5_class2_name;",
        alias = "int_guardianmodulereinforcement_size5_class2"
    )]
    GuardianModuleReinforcementS5C2,
    #[strum(to_string = "Module Reinforcement")]
    #[serde(
        alias = "$int_modulereinforcement_size3_class1_name;",
        alias = "int_modulereinforcement_size3_class1"
    )]
    ModuleReinforcementS3C1,
    #[strum(to_string = "Module Reinforcement")]
    #[serde(
        alias = "$int_modulereinforcement_size3_class2_name;",
        alias = "int_modulereinforcement_size3_class2"
    )]
    ModuleReinforcementS3C2,
    #[strum(to_string = "Module Reinforcement")]
    #[serde(
        alias = "$int_modulereinforcement_size4_class2_name;",
        alias = "int_modulereinforcement_size4_class2"
    )]
    ModuleReinforcementS4C2,
    #[strum(to_string = "Module Reinforcement")]
    #[serde(
        alias = "$int_modulereinforcement_size1_class2_name;",
        alias = "int_modulereinforcement_size1_class2"
    )]
    ModuleReinforcementS1C2,
    #[strum(to_string = "Module Reinforcement")]
    #[serde(
        alias = "$int_modulereinforcement_size2_class1_name;",
        alias = "int_modulereinforcement_size2_class1"
    )]
    ModuleReinforcementS2C1,
    #[strum(to_string = "Module Reinforcement")]
    #[serde(
        alias = "$int_modulereinforcement_size2_class2_name;",
        alias = "int_modulereinforcement_size2_class2"
    )]
    ModuleReinforcementS2C2,
    #[strum(to_string = "Module Reinforcement")]
    #[serde(
        alias = "$int_modulereinforcement_size5_class2_name;",
        alias = "int_modulereinforcement_size5_class2"
    )]
    ModuleReinforcementS5C2,
    #[strum(to_string = "Guardian Shield Reinforcement")]
    #[serde(
        alias = "$int_guardianshieldreinforcement_size2_class2_name;",
        alias = "int_guardianshieldreinforcement_size2_class2"
    )]
    GuardianShieldReinforcementS2C2,
    #[strum(to_string = "Guardian Shield Reinforcement")]
    #[serde(
        alias = "$int_guardianshieldreinforcement_size3_class2_name;",
        alias = "int_guardianshieldreinforcement_size3_class2"
    )]
    GuardianShieldReinforcementS3C2,
    #[strum(to_string = "Guardian Shield Reinforcement")]
    #[serde(
        alias = "$int_guardianshieldreinforcement_size4_class2_name;",
        alias = "int_guardianshieldreinforcement_size4_class2"
    )]
    GuardianShieldReinforcementS4C2,
    #[strum(to_string = "Guardian Shield Reinforcement")]
    #[serde(
        alias = "$int_guardianshieldreinforcement_size5_class2_name;",
        alias = "int_guardianshieldreinforcement_size5_class2"
    )]
    GuardianShieldReinforcementS5C2,
    #[strum(to_string = "Shield Booster")]
    #[serde(
        alias = "$hpt_shieldbooster_size0_class1_name;",
        alias = "hpt_shieldbooster_size0_class1"
    )]
    ShieldBoosterS0C1,
    #[strum(to_string = "Shield Booster")]
    #[serde(
        alias = "$hpt_shieldbooster_size0_class2_name;",
        alias = "hpt_shieldbooster_size0_class2"
    )]
    ShieldBoosterS0C2,
    #[strum(to_string = "Shield Booster")]
    #[serde(
        alias = "$hpt_shieldbooster_size0_class3_name;",
        alias = "hpt_shieldbooster_size0_class3"
    )]
    ShieldBoosterS0C3,
    #[strum(to_string = "Shield Booster")]
    #[serde(
        alias = "$hpt_shieldbooster_size0_class4_name;",
        alias = "hpt_shieldbooster_size0_class4"
    )]
    ShieldBoosterS0C4,
    #[strum(to_string = "Shield Booster")]
    #[serde(
        alias = "$hpt_shieldbooster_size0_class5_name;",
        alias = "hpt_shieldbooster_size0_class5"
    )]
    ShieldBoosterS0C5,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size2_class4_name;",
        alias = "int_powerdistributor_size2_class4"
    )]
    PowerDistributorS2C4,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size3_class5_name;",
        alias = "int_powerdistributor_size3_class5"
    )]
    PowerDistributorS3C5,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size5_class3_name;",
        alias = "int_powerdistributor_size5_class3"
    )]
    PowerDistributorS5C3,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size5_class4_name;",
        alias = "int_powerdistributor_size5_class4"
    )]
    PowerDistributorS5C4,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size6_class1_name;",
        alias = "int_powerdistributor_size6_class1"
    )]
    PowerDistributorS6C1,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size6_class2_name;",
        alias = "int_powerdistributor_size6_class2"
    )]
    PowerDistributorS6C2,
    #[strum(to_string = "Power Distributor")]
    #[serde(
        alias = "$int_powerdistributor_size8_class5_name;",
        alias = "int_powerdistributor_size8_class5"
    )]
    PowerDistributorS8C5,
    #[strum(to_string = "Mining Tool")]
    #[serde(
        alias = "$hpt_miningtoolv2_fixed_large_name;",
        alias = "hpt_miningtoolv2_fixed_large"
    )]
    MiningToolV2FixedLarge,
    #[strum(to_string = "Mine Launcher")]
    #[serde(
        alias = "$hpt_minelauncher_fixed_small_name;",
        alias = "hpt_minelauncher_fixed_small"
    )]
    MineLauncherFixedSmall,
    #[strum(to_string = "Mine Launcher")]
    #[serde(
        alias = "$hpt_minelauncher_fixed_medium_name;",
        alias = "hpt_minelauncher_fixed_medium"
    )]
    MineLauncherFixedMedium,
    #[strum(to_string = "Mining Laser")]
    #[serde(
        alias = "$hpt_mininglaser_fixed_small_name;",
        alias = "hpt_mininglaser_fixed_small"
    )]
    MiningLaserFixedSmall,
    #[strum(to_string = "Mining Laser")]
    #[serde(
        alias = "$hpt_mininglaser_turret_small_name;",
        alias = "hpt_mininglaser_turret_small"
    )]
    MiningLaserTurretSmall,
    #[strum(to_string = "Mining Laser")]
    #[serde(
        alias = "$hpt_mininglaser_turret_medium_name;",
        alias = "hpt_mininglaser_turret_medium"
    )]
    MiningLaserTurretMedium,
    #[strum(to_string = "Mining Laser")]
    #[serde(
        alias = "$hpt_mininglaser_fixed_medium_name;",
        alias = "hpt_mininglaser_fixed_medium"
    )]
    MiningLaserFixedMedium,
    #[strum(to_string = "Flechette Launcher")]
    #[serde(
        alias = "$hpt_flechettelauncher_fixed_medium_name;",
        alias = "hpt_flechettelauncher_fixed_medium"
    )]
    FlechetteLauncherFixedMedium,
    #[strum(to_string = "Mining Subsurface Displacement Launcher")]
    #[serde(
        alias = "$hpt_mining_subsurfdispmisle_fixed_medium_name;",
        alias = "hpt_mining_subsurfdispmisle_fixed_medium"
    )]
    MiningSubsurfaceDisplacementLauncherFixedMedium,
    #[strum(to_string = "Mining Seismic Charge Launcher")]
    #[serde(
        alias = "$hpt_mining_seismchrgwarhd_turret_medium_name;",
        alias = "hpt_mining_seismchrgwarhd_turret_medium"
    )]
    MiningSeismicChargeLauncherTurretMedium,
    #[strum(to_string = "Mining Seismic Charge Launcher")]
    #[serde(
        alias = "$hpt_mining_seismchrgwarhd_fixed_medium_name;",
        alias = "hpt_mining_seismchrgwarhd_fixed_medium"
    )]
    MiningSeismicChargeLauncherFixedMedium,
    #[strum(to_string = "Mining Abration Blaster")]
    #[serde(
        alias = "$hpt_mining_abrblstr_turret_small_name;",
        alias = "hpt_mining_abrblstr_turret_small"
    )]
    MiningAbrationBlasterTurretSmall,
    #[strum(to_string = "Mining Abration Blaster")]
    #[serde(
        alias = "$hpt_mining_abrblstr_fixed_small_name;",
        alias = "hpt_mining_abrblstr_fixed_small"
    )]
    MiningAbrationBlasterFixedSmall,
    #[strum(to_string = "Burst Laser")]
    #[serde(
        alias = "$hpt_pulselaserburst_fixed_small_scatter_name;",
        alias = "hpt_pulselaserburst_fixed_small_scatter"
    )]
    PulseLaserBurstFixedSmallScatter,
    #[strum(to_string = "Burst Laser")]
    #[serde(
        alias = "$hpt_pulselaserburst_fixed_small_name;",
        alias = "hpt_pulselaserburst_fixed_small"
    )]
    PulseLaserBurstFixedSmall,
    #[strum(to_string = "Burst Laser")]
    #[serde(
        alias = "$hpt_pulselaserburst_fixed_medium_name;",
        alias = "hpt_pulselaserburst_fixed_medium"
    )]
    PulseLaserBurstFixedMedium,
    #[strum(to_string = "Burst Laser")]
    #[serde(
        alias = "$hpt_pulselaserburst_fixed_large_name;",
        alias = "hpt_pulselaserburst_fixed_large"
    )]
    PulseLaserBurstFixedLarge,
    #[strum(to_string = "Burst Laser")]
    #[serde(
        alias = "$hpt_pulselaserburst_fixed_huge_name;",
        alias = "hpt_pulselaserburst_fixed_huge"
    )]
    PulseLaserBurstFixedHuge,
    #[strum(to_string = "Burst Laser")]
    #[serde(
        alias = "$hpt_pulselaserburst_turret_small_name;",
        alias = "hpt_pulselaserburst_turret_small"
    )]
    PulseLaserBurstTurretSmall,
    #[strum(to_string = "Burst Laser")]
    #[serde(
        alias = "$hpt_pulselaserburst_turret_medium_name;",
        alias = "hpt_pulselaserburst_turret_medium"
    )]
    PulseLaserBurstTurretMedium,
    #[strum(to_string = "Burst Laser")]
    #[serde(
        alias = "$hpt_pulselaserburst_turret_large_name;",
        alias = "hpt_pulselaserburst_turret_large"
    )]
    PulseLaserBurstTurretLarge,
    #[strum(to_string = "Burst Laser")]
    #[serde(
        alias = "$hpt_pulselaserburst_gimbal_small_name;",
        alias = "hpt_pulselaserburst_gimbal_small"
    )]
    PulseLaserBurstGimbalSmall,
    #[strum(to_string = "Burst Laser")]
    #[serde(
        alias = "$hpt_pulselaserburst_gimbal_medium_name;",
        alias = "hpt_pulselaserburst_gimbal_medium"
    )]
    PulseLaserBurstGimbalMedium,
    #[strum(to_string = "Burst Laser")]
    #[serde(
        alias = "$hpt_pulselaserburst_gimbal_large_name;",
        alias = "hpt_pulselaserburst_gimbal_large"
    )]
    PulseLaserBurstGimbalLarge,
    #[strum(to_string = "Burst Laser")]
    #[serde(
        alias = "$hpt_pulselaserburst_gimbal_huge_name;",
        alias = "hpt_pulselaserburst_gimbal_huge"
    )]
    PulseLaserBurstGimbalHuge,
    #[strum(to_string = "Pulse Laser")]
    #[serde(
        alias = "$hpt_pulselaser_gimbal_small_name;",
        alias = "hpt_pulselaser_gimbal_small"
    )]
    PulseLaserGimbalSmall,
    #[strum(to_string = "Pulse Laser")]
    #[serde(
        alias = "$hpt_pulselaser_gimbal_large_name;",
        alias = "hpt_pulselaser_gimbal_large"
    )]
    PulseLaserGimbalLarge,
    #[strum(to_string = "Pulse Laser")]
    #[serde(
        alias = "$hpt_pulselaser_turret_small_name;",
        alias = "hpt_pulselaser_turret_small"
    )]
    PulseLaserTurretSmall,
    #[strum(to_string = "Pulse Laser")]
    #[serde(
        alias = "$hpt_pulselaser_turret_medium_name;",
        alias = "hpt_pulselaser_turret_medium"
    )]
    PulseLaserTurretMedium,
    #[strum(to_string = "Pulse Laser")]
    #[serde(
        alias = "$hpt_pulselaser_turret_large_name;",
        alias = "hpt_pulselaser_turret_large"
    )]
    PulseLaserTurretLarge,
    #[strum(to_string = "Pulse Laser")]
    #[serde(
        alias = "$hpt_pulselaser_fixed_small_name;",
        alias = "hpt_pulselaser_fixed_small"
    )]
    PulseLaserFixedSmall,
    #[strum(to_string = "Pulse Laser")]
    #[serde(
        alias = "$hpt_pulselaser_fixed_medium_name;",
        alias = "hpt_pulselaser_fixed_medium"
    )]
    PulseLaserFixedMedium,
    #[strum(to_string = "Pulse Laser")]
    #[serde(
        alias = "$hpt_pulselaser_fixed_large_name;",
        alias = "hpt_pulselaser_fixed_large"
    )]
    PulseLaserFixedLarge,
    #[strum(to_string = "Pulse Laser")]
    #[serde(
        alias = "$hpt_pulselaser_fixed_huge_name;",
        alias = "hpt_pulselaser_fixed_huge"
    )]
    PulseLaserFixedHuge,
    #[strum(to_string = "Pulse Laser")]
    #[serde(
        alias = "$hpt_pulselaser_gimbal_medium_name;",
        alias = "hpt_pulselaser_gimbal_medium"
    )]
    PulseLaserMedium,
    #[strum(to_string = "Pulse Laser")]
    #[serde(
        alias = "$hpt_pulselaser_gimbal_huge_name;",
        alias = "hpt_pulselaser_gimbal_huge"
    )]
    PulseLaserGimbalHuge,
    #[strum(to_string = "Beam Laser")]
    #[serde(
        alias = "$hpt_beamlaser_gimbal_small_name;",
        alias = "hpt_beamlaser_gimbal_small"
    )]
    BeamLaserGimbalSmall,
    #[strum(to_string = "Beam Laser")]
    #[serde(
        alias = "$hpt_beamlaser_gimbal_medium_name;",
        alias = "hpt_beamlaser_gimbal_medium"
    )]
    BeamLaserGimbalMedium,
    #[strum(to_string = "Beam Laser")]
    #[serde(
        alias = "$hpt_beamlaser_gimbal_huge_name;",
        alias = "hpt_beamlaser_gimbal_huge"
    )]
    BeamLaserGimbalHuge,
    #[strum(to_string = "Beam Laser")]
    #[serde(
        alias = "$hpt_beamlaser_turret_small_name;",
        alias = "hpt_beamlaser_turret_small"
    )]
    BeamLaserTurretSmall,
    #[strum(to_string = "Beam Laser")]
    #[serde(
        alias = "$hpt_beamlaser_turret_medium_name;",
        alias = "hpt_beamlaser_turret_medium"
    )]
    BeamLaserTurretMedium,
    #[strum(to_string = "Beam Laser")]
    #[serde(
        alias = "$hpt_beamlaser_fixed_small_name;",
        alias = "hpt_beamlaser_fixed_small"
    )]
    BeamLaserFixedSmall,
    #[strum(to_string = "Beam Laser")]
    #[serde(
        alias = "$hpt_beamlaser_fixed_medium_name;",
        alias = "hpt_beamlaser_fixed_medium"
    )]
    BeamLaserFixedMedium,
    #[strum(to_string = "Beam Laser")]
    #[serde(
        alias = "$hpt_beamlaser_fixed_large_name;",
        alias = "hpt_beamlaser_fixed_large"
    )]
    BeamLaserFixedLarge,
    #[strum(to_string = "Beam Laser")]
    #[serde(
        alias = "$hpt_beamlaser_fixed_huge_name;",
        alias = "hpt_beamlaser_fixed_huge"
    )]
    BeamLaserFixedHuge,
    #[strum(to_string = "Beam Laser")]
    #[serde(
        alias = "$hpt_beamlaser_gimbal_large_name;",
        alias = "hpt_beamlaser_gimbal_large"
    )]
    BeamLaserGimbalLarge,
    #[strum(to_string = "Beam Laser")]
    #[serde(
        alias = "$hpt_beamlaser_turret_large_name;",
        alias = "hpt_beamlaser_turret_large"
    )]
    BeamLaserTurretLarge,
    #[strum(to_string = "Gauss Cannon")]
    #[serde(
        alias = "$hpt_guardian_gausscannon_fixed_medium_name;",
        alias = "hpt_guardian_gausscannon_fixed_medium"
    )]
    GaussCannonFixedMedium,
    #[strum(to_string = "Gauss Cannon")]
    #[serde(
        alias = "$hpt_guardian_gausscannon_fixed_small_name;",
        alias = "hpt_guardian_gausscannon_fixed_small"
    )]
    GaussCannonFixedSmall,
    #[strum(to_string = "Shard Cannon")]
    #[serde(
        alias = "$hpt_guardian_shardcannon_fixed_large_name;",
        alias = "hpt_guardian_shardcannon_fixed_large"
    )]
    ShardCannonFixedLarge,
    #[strum(to_string = "Shard Cannon")]
    #[serde(
        alias = "$hpt_guardian_shardcannon_fixed_medium_name;",
        alias = "hpt_guardian_shardcannon_fixed_medium"
    )]
    ShardCannonFixedMedium,
    #[strum(to_string = "Cannon")]
    #[serde(
        alias = "$hpt_cannon_fixed_small_name;",
        alias = "hpt_cannon_fixed_small"
    )]
    CannonFixedSmall,
    #[strum(to_string = "Cannon")]
    #[serde(
        alias = "$hpt_cannon_fixed_medium_name;",
        alias = "hpt_cannon_fixed_medium"
    )]
    CannonFixedMedium,
    #[strum(to_string = "Cannon")]
    #[serde(
        alias = "$hpt_cannon_fixed_large_name;",
        alias = "hpt_cannon_fixed_large"
    )]
    CannonFixedLarge,
    #[strum(to_string = "Cannon")]
    #[serde(
        alias = "$hpt_cannon_gimbal_small_name;",
        alias = "hpt_cannon_gimbal_small"
    )]
    CannonGimbalSmall,
    #[strum(to_string = "Cannon")]
    #[serde(
        alias = "$hpt_cannon_gimbal_medium_name;",
        alias = "hpt_cannon_gimbal_medium"
    )]
    CannonGimbalMedium,
    #[strum(to_string = "Cannon")]
    #[serde(
        alias = "$hpt_cannon_gimbal_large_name;",
        alias = "hpt_cannon_gimbal_large"
    )]
    CannonGimbalLarge,
    #[strum(to_string = "Cannon")]
    #[serde(
        alias = "$hpt_cannon_gimbal_huge_name;",
        alias = "hpt_cannon_gimbal_huge"
    )]
    CannonGimbalHuge,
    #[strum(to_string = "Cannon")]
    #[serde(
        alias = "$hpt_cannon_turret_medium_name;",
        alias = "hpt_cannon_turret_medium"
    )]
    CannonTurretMedium,
    #[strum(to_string = "Cannon")]
    #[serde(
        alias = "$hpt_cannon_turret_large_name;",
        alias = "hpt_cannon_turret_large"
    )]
    CannonTurretLarge,
    #[strum(to_string = "Cannon")]
    #[serde(
        alias = "$hpt_cannon_turret_huge_name;",
        alias = "hpt_cannon_turret_huge"
    )]
    CannonTurretHuge,
    #[strum(to_string = "Plasma Charger")]
    #[serde(
        alias = "$hpt_guardian_plasmalauncher_fixed_medium_name;",
        alias = "hpt_guardian_plasmalauncher_fixed_medium"
    )]
    PlasmaChargerFixedMedium,
    #[strum(to_string = "Xeno Scanner")]
    #[serde(
        alias = "$hpt_xenoscanner_basic_tiny_name;",
        alias = "hpt_xenoscanner_basic_tiny"
    )]
    EnhancedXenoScanner,
    #[strum(to_string = "Enhanced Xeno Scanner")]
    #[serde(
        alias = "$hpt_xenoscannermk2_basic_tiny_name;",
        alias = "hpt_xenoscannermk2_basic_tiny"
    )]
    XenoScanner,
    #[strum(to_string = "Caustic Sink Launcher")]
    #[serde(
        alias = "$hpt_causticsinklauncher_turret_tiny_name;",
        alias = "hpt_causticsinklauncher_turret_tiny"
    )]
    CausticSinkLauncher,
    #[strum(to_string = "Multi-Cannon")]
    #[serde(
        alias = "$hpt_multicannon_gimbal_small_name;",
        alias = "hpt_multicannon_gimbal_small"
    )]
    MultiCannonGimbalSmall,
    #[strum(to_string = "Multi-Cannon")]
    #[serde(
        alias = "$hpt_multicannon_gimbal_medium_name;",
        alias = "hpt_multicannon_gimbal_medium"
    )]
    MultiCannonGimbalMedium,
    #[strum(to_string = "Multi-Cannon")]
    #[serde(
        alias = "$hpt_multicannon_turret_small_name;",
        alias = "hpt_multicannon_turret_small"
    )]
    MultiCannonTurretSmall,
    #[strum(to_string = "Multi-Cannon")]
    #[serde(
        alias = "$hpt_multicannon_turret_medium_name;",
        alias = "hpt_multicannon_turret_medium"
    )]
    MultiCannonTurretMedium,
    #[strum(to_string = "Multi-Cannon")]
    #[serde(
        alias = "$hpt_multicannon_fixed_small_name;",
        alias = "hpt_multicannon_fixed_small"
    )]
    MultiCannonFixedSmall,
    #[strum(to_string = "Multi-Cannon")]
    #[serde(
        alias = "$hpt_multicannon_fixed_medium_name;",
        alias = "hpt_multicannon_fixed_medium"
    )]
    MultiCannonFixedMedium,
    #[strum(to_string = "Multi-Cannon")]
    #[serde(
        alias = "$hpt_multicannon_fixed_large_name;",
        alias = "hpt_multicannon_fixed_large"
    )]
    MultiCannonFixedLarge,
    #[strum(to_string = "Multi-Cannon")]
    #[serde(
        alias = "$hpt_multicannon_fixed_huge_name;",
        alias = "hpt_multicannon_fixed_huge"
    )]
    MultiCannonFixedHuge,
    #[strum(to_string = "Multi-Cannon")]
    #[serde(
        alias = "$hpt_multicannon_gimbal_large_name;",
        alias = "hpt_multicannon_gimbal_large"
    )]
    MultiCannonGimbalLarge,
    #[strum(to_string = "Multi-Cannon")]
    #[serde(
        alias = "$hpt_multicannon_gimbal_huge_name;",
        alias = "hpt_multicannon_gimbal_huge"
    )]
    MultiCannonGimbalHuge,
    #[strum(to_string = "Enhanced AX Multi-Cannon")]
    #[serde(
        alias = "$hpt_atmulticannon_gimbal_medium_name;",
        alias = "hpt_atmulticannon_gimbal_medium"
    )]
    EnhancedAXMultiCannonGimbalMedium,
    #[strum(to_string = "AX Multi-Cannon")]
    #[serde(
        alias = "$hpt_atmulticannon_turret_medium_name;",
        alias = "hpt_atmulticannon_turret_medium"
    )]
    AXMultiCannonTurretMedium,
    #[strum(to_string = "AX Multi-Cannon")]
    #[serde(
        alias = "$hpt_atmulticannon_turret_large_name;",
        alias = "hpt_atmulticannon_turret_large"
    )]
    AXMultiCannonTurretLarge,
    #[strum(to_string = "Enhanced AX Multi-Cannon")]
    #[serde(
        alias = "$hpt_atmulticannon_turret_medium_v2_name;",
        alias = "hpt_atmulticannon_turret_medium_v2"
    )]
    EnhancedAXMultiCannonTurretMedium,
    #[strum(to_string = "Enhanced AX Multi-Cannon")]
    #[serde(
        alias = "$hpt_atmulticannon_turret_large_v2_name;",
        alias = "hpt_atmulticannon_turret_large_v2"
    )]
    EnhancedAXMultiCannonTurretLarge,
    #[strum(to_string = "Enhanced AX Multi-Cannon")]
    #[serde(
        alias = "$hpt_atmulticannon_fixed_medium_v2_name;",
        alias = "hpt_atmulticannon_fixed_medium_v2"
    )]
    EnhancedAXMultiCannonFixedMedium,
    #[strum(to_string = "AX Multi-Cannon")]
    #[serde(
        alias = "$hpt_atmulticannon_fixed_large_name;",
        alias = "hpt_atmulticannon_fixed_large"
    )]
    AXMultiCannonFixedLarge,
    #[strum(to_string = "Enhanced AX Multi-Cannon")]
    #[serde(
        alias = "$hpt_atmulticannon_gimbal_large_name;",
        alias = "hpt_atmulticannon_gimbal_large"
    )]
    EnhancedAXMultiCannonGimbalLarge,
    #[strum(to_string = "TG Pulse Neutraliser")]
    #[serde(
        alias = "$hpt_antiunknownshutdown_tiny_v2_name;",
        alias = "hpt_antiunknownshutdown_tiny_v2"
    )]
    TGPulseNeutraliser,
    #[strum(to_string = "Shutdown Field Neutraliser")]
    #[serde(
        alias = "$hpt_antiunknownshutdown_tiny_name;",
        alias = "hpt_antiunknownshutdown_tiny"
    )]
    ShutdownFieldNeutraliser,
    #[strum(to_string = "Seeker Missile Rack")]
    #[serde(
        alias = "$hpt_basicmissilerack_fixed_small_name;",
        alias = "hpt_basicmissilerack_fixed_small"
    )]
    SeekerMissileRackFixedSmall,
    #[strum(to_string = "Seeker Missile Rack")]
    #[serde(
        alias = "$hpt_basicmissilerack_fixed_medium_name;",
        alias = "hpt_basicmissilerack_fixed_medium"
    )]
    SeekerMissileRackFixedMedium,
    #[strum(to_string = "Seeker Missile Rack")]
    #[serde(
        alias = "$hpt_basicmissilerack_fixed_large_name;",
        alias = "hpt_basicmissilerack_fixed_large"
    )]
    SeekerMissileRackFixedLarge,
    #[strum(to_string = "Pack-Hound")]
    #[serde(
        alias = "$hpt_drunkmissilerack_fixed_medium_name;",
        alias = "hpt_drunkmissilerack_fixed_medium"
    )]
    PackHoundFixedMedium,
    #[strum(to_string = "Missile Rack")]
    #[serde(
        alias = "$hpt_dumbfiremissilerack_fixed_small_name;",
        alias = "hpt_dumbfiremissilerack_fixed_small"
    )]
    MissileRackFixedSmall,
    #[strum(to_string = "Missile Rack")]
    #[serde(
        alias = "$hpt_dumbfiremissilerack_fixed_medium_name;",
        alias = "hpt_dumbfiremissilerack_fixed_medium"
    )]
    MissileRackFixedMedium,
    #[strum(to_string = "Missile Rack")]
    #[serde(
        alias = "$hpt_dumbfiremissilerack_fixed_large_name;",
        alias = "hpt_dumbfiremissilerack_fixed_large"
    )]
    MissileRackFixedLarge,
    #[strum(to_string = "AX Missile Rack")]
    #[serde(
        alias = "$hpt_atdumbfiremissile_fixed_medium_name;",
        alias = "hpt_atdumbfiremissile_fixed_medium"
    )]
    AXMissileRackFixeMedium,
    #[strum(to_string = "AX Missile Rack")]
    #[serde(
        alias = "$hpt_atdumbfiremissile_fixed_large_name;",
        alias = "hpt_atdumbfiremissile_fixed_large"
    )]
    AXMissileRackFixeLarge,
    #[strum(to_string = "Enhanced AX Missile Rack")]
    #[serde(
        alias = "$hpt_atdumbfiremissile_fixed_medium_v2_name;",
        alias = "hpt_atdumbfiremissile_fixed_medium_v2"
    )]
    EnhancedAXMissileRackFixedMediumV2,
    #[strum(to_string = "Enhanced AX Missile Rack")]
    #[serde(
        alias = "$hpt_atdumbfiremissile_fixed_large_v2_name;",
        alias = "hpt_atdumbfiremissile_fixed_large_v2"
    )]
    EnhancedAXMissileRackFixedLargeV2,
    #[strum(to_string = "Universal Multi-Limpet Controller")]
    #[serde(
        alias = "$int_multidronecontrol_xeno_size3_class3_name;",
        alias = "int_multidronecontrol_xeno_size3_class3"
    )]
    XenoMultiLimpetControllerS3C3,
    #[strum(to_string = "Universal Multi-Limpet Controller")]
    #[serde(
        alias = "$int_multidronecontrol_xeno_size3_class4_name;",
        alias = "int_multidronecontrol_xeno_size3_class4"
    )]
    XenoMultiLimpetControllerS3C4,
    #[strum(to_string = "Decontamination Limpet Controller")]
    #[serde(
        alias = "$int_dronecontrol_decontamination_size1_class1_name;",
        alias = "int_dronecontrol_decontamination_size1_class1"
    )]
    DecontaminationLimpetControllerS1C1,
    #[strum(to_string = "Decontamination Limpet Controller")]
    #[serde(
        alias = "$int_dronecontrol_decontamination_size5_class1_name;",
        alias = "int_dronecontrol_decontamination_size5_class1"
    )]
    DecontaminationLimpetControllerS5C1,
    #[strum(to_string = "Hatch Breaker")]
    #[serde(
        alias = "$int_dronecontrol_resourcesiphon_size1_class3_name;",
        alias = "int_dronecontrol_resourcesiphon_size1_class3"
    )]
    HatchBreakerS1C3,
    #[strum(to_string = "Hatch Breaker")]
    #[serde(
        alias = "$int_dronecontrol_resourcesiphon_size3_class3_name;",
        alias = "int_dronecontrol_resourcesiphon_size3_class3"
    )]
    HatchBreakerS3C3,
    #[strum(to_string = "Hatch Breaker")]
    #[serde(
        alias = "$int_dronecontrol_resourcesiphon_size5_class3_name;",
        alias = "int_dronecontrol_resourcesiphon_size5_class3"
    )]
    HatchBreakerS5C3,
    #[strum(to_string = "Hatch Breaker")]
    #[serde(
        alias = "$int_dronecontrol_resourcesiphon_size5_class4_name;",
        alias = "int_dronecontrol_resourcesiphon_size5_class4"
    )]
    HatchBreakerS5C4,
    #[strum(to_string = "Rescue Limpet Controller")]
    #[serde(
        alias = "$int_multidronecontrol_rescue_size3_class3_name;",
        alias = "int_multidronecontrol_rescue_size3_class3"
    )]
    RescueLimpetController,
    #[strum(to_string = "Universal Multi-Limpet Controller")]
    #[serde(
        alias = "$int_multidronecontrol_universal_size7_class5_name;",
        alias = "int_multidronecontrol_universal_size7_class5"
    )]
    UniversalMultiLimpetController,
    #[strum(to_string = "Mining Multi-Limpet Controller")]
    #[serde(
        alias = "$int_multidronecontrol_mining_size3_class3_name;",
        alias = "int_multidronecontrol_mining_size3_class3"
    )]
    UniversalMultiLimpetControllerMiningS3C3,
    #[strum(to_string = "Mining Multi-Limpet Controller")]
    #[serde(
        alias = "$int_multidronecontrol_miningv2_size3_class3_name;",
        alias = "int_multidronecontrol_miningv2_size3_class3"
    )]
    UniversalMultiLimpetControllerMiningV2S3C3,
    #[strum(to_string = "Mining Multi-Limpet Controller")]
    #[serde(
        alias = "$int_multidronecontrol_miningv2_size5_class5_name;",
        alias = "int_multidronecontrol_miningv2_size5_class5"
    )]
    UniversalMultiLimpetControllerMiningV2S5C5,
    #[strum(to_string = "Universal Multi-Limpet Controller")]
    #[serde(
        alias = "$int_multidronecontrol_operations_size3_class4_name;",
        alias = "int_multidronecontrol_operations_size3_class4"
    )]
    UniversalMultiLimpetControllerOperationsS3C4,
    #[strum(to_string = "Advanced Torpedo Pylon")]
    #[serde(
        alias = "$hpt_advancedtorppylon_fixed_small_name;",
        alias = "hpt_advancedtorppylon_fixed_small"
    )]
    AdvancedTorpedoPylonSmall,
    #[strum(to_string = "Advanced Torpedo Pylon")]
    #[serde(
        alias = "$hpt_advancedtorppylon_fixed_medium_name;",
        alias = "hpt_advancedtorppylon_fixed_medium"
    )]
    AdvancedTorpedoPylonFixedMedium,
    #[strum(to_string = "Advanced Torpedo Pylon")]
    #[serde(
        alias = "$hpt_advancedtorppylon_fixed_large_name;",
        alias = "hpt_advancedtorppylon_fixed_large"
    )]
    AdvancedTorpedoPylonLarge,
    #[strum(to_string = "Nanite Torpedo Pylon")]
    #[serde(
        alias = "$hpt_atventdisruptorpylon_fixed_medium_name;",
        alias = "hpt_atventdisruptorpylon_fixed_medium"
    )]
    NaniteTorpedoPylonMedium,
    #[strum(to_string = "Nanite Torpedo Pylon")]
    #[serde(
        alias = "$hpt_atventdisruptorpylon_fixed_large_name;",
        alias = "hpt_atventdisruptorpylon_fixed_large"
    )]
    NaniteTorpedoPylonLarge,
    #[strum(to_string = "Frag Cannon")]
    #[serde(
        alias = "$hpt_slugshot_fixed_small_name;",
        alias = "hpt_slugshot_fixed_small"
    )]
    SlugshotFixedSmall,
    #[strum(to_string = "Frag Cannon")]
    #[serde(
        alias = "$hpt_slugshot_fixed_medium_name;",
        alias = "hpt_slugshot_fixed_medium"
    )]
    SlugshotFixedMedium,
    #[strum(to_string = "The Pacifier")]
    #[serde(
        alias = "$hpt_slugshot_fixed_large_range_name;",
        alias = "hpt_slugshot_fixed_large_range"
    )]
    SlugshotFixedLargeRange,
    #[strum(to_string = "Frag Cannon")]
    #[serde(
        alias = "$hpt_slugshot_gimbal_medium_name;",
        alias = "hpt_slugshot_gimbal_medium"
    )]
    SlugshotGimbalMedium,
    #[strum(to_string = "Frag Cannon")]
    #[serde(
        alias = "$hpt_slugshot_gimbal_large_name;",
        alias = "hpt_slugshot_gimbal_large"
    )]
    SlugshotGimbalLarge,
    #[strum(to_string = "Frag Cannon")]
    #[serde(
        alias = "$hpt_slugshot_gimbal_small_name;",
        alias = "hpt_slugshot_gimbal_small"
    )]
    SlugshotGimbalSmall,
    #[strum(to_string = "Frag Cannon")]
    #[serde(
        alias = "$hpt_slugshot_fixed_large_name;",
        alias = "hpt_slugshot_fixed_large"
    )]
    SlugshotFixedLarge,
    #[strum(to_string = "Frag Cannon")]
    #[serde(
        alias = "$hpt_slugshot_turret_medium_name;",
        alias = "hpt_slugshot_turret_medium"
    )]
    SlugshotTurretMedium,
    #[strum(to_string = "Frag Cannon")]
    #[serde(
        alias = "$hpt_slugshot_turret_large_name;",
        alias = "hpt_slugshot_turret_large"
    )]
    SlugshotTurretLarge,
    #[serde(
        alias = "$int_expmodulestabiliser_size3_class3_name;",
        alias = "int_expmodulestabiliser_size3_class3"
    )]
    ExperimentalModuleStabiliserS3C3,
    #[serde(
        alias = "$int_expmodulestabiliser_size5_class3_name;",
        alias = "int_expmodulestabiliser_size5_class3"
    )]
    ExperimentalModuleStabiliserS5C3,
    #[serde(alias = "paintjob_asp_corrosive_05")]
    PaintJobAspCorrosive05,
    #[serde(alias = "paintjob_asp_metallic_gold")]
    PaintJobAspMetallicGold,
    #[serde(alias = "paintjob_asp_operator_teal")]
    PaintJobAspOperatorTeal,
    #[serde(alias = "paintjob_asp_tactical_white")]
    PaintJobAspTacticalWhite,
    #[serde(alias = "paintjob_asp_stripe1_04")]
    PaintJobAspStripe1_04,
    #[serde(alias = "paintjob_asp_halloween01_01")]
    PaintJobAspHalloween0101,
    #[serde(alias = "paintjob_anaconda_luminous_stripe_02")]
    PaintJobAnacondaLuminousStripe02,
    #[serde(alias = "paintjob_anaconda_winter1_06")]
    PaintJobAnacondaWinter106,
    #[serde(alias = "paintjob_anaconda_halloween02_05")]
    PaintJobAnacondaHalloween0205,
    #[serde(alias = "paintjob_anaconda_metallic2_gold")]
    PaintJobAnacondaMetallic2Gold,
    #[serde(alias = "paintjob_anaconda_tactical_white")]
    PaintJobAnacondaTacticalWhite,
    #[serde(alias = "paintjob_anaconda_thargoidreward3_01")]
    PaintJobAnacondaThargoidReward301,
    #[serde(alias = "paintjob_anaconda_pulse2_purple")]
    PaintJobAnacondaPulse2Purple,
    #[serde(alias = "paintjob_anaconda_vibrant_yellow")]
    PaintJobAnacondaVibrantYellow,
    #[serde(alias = "paintjob_anaconda_lrpo_azure")]
    PaintJobAnacondaLRPOAzure,
    #[serde(alias = "paintjob_belugaliner_calligraphy_05")]
    PaintJobABelugaLinerCalligraphy05,
    #[serde(alias = "paintjob_cobramkiii_25thanniversary_01")]
    PaintJobCobraMkIII25thAnniversary01,
    #[serde(alias = "paintjob_cobramkiii_halloween02_02")]
    PaintJobCobraMkIIIHalloween0202,
    #[serde(alias = "paintjob_cobramkv_metallic_chrome")]
    PaintJobCobraMkVMetallicChrome,
    #[serde(alias = "paintjob_corsair_01_18")]
    PaintjobCorsair0118,
    #[serde(alias = "paintjob_cutter_tactical_grey")]
    PaintjobCutterTacticalGrey,
    #[serde(alias = "paintjob_cutter_iridescenthighcolour_06")]
    PaintjobCutterIridescentHighColour06,
    #[serde(alias = "paintjob_cutter_vibrant_purple")]
    PaintjobCutterVibrantPurple,
    #[serde(alias = "paintjob_cutter_thargoidreward2_01")]
    PaintjobCutterThargoidReward201,
    #[serde(alias = "paintjob_cutter_metallic_gold")]
    PaintjobCutterMetallicGold,
    #[serde(alias = "paintjob_cutter_fullmetal_bronze")]
    PaintjobCutterFullMetalBronze,
    #[serde(alias = "paintjob_cutter_expressway_01")]
    PaintjobCutterExpressway01,
    #[serde(alias = "paintjob_empire_eagle_blackfriday_01")]
    PaintJobEmpireEagleBlackFriday01,
    #[serde(alias = "paintjob_empire_trader_powerplay2empire_01")]
    PaintJobEmpireTraderPowerplay2Empire01,
    #[serde(alias = "paintjob_ferdelance_winter1_01")]
    PaintJobFerdelanceWinter101,
    #[serde(alias = "paintjob_explorer_nx_02_01")]
    PaintJobExplorerNX0201,
    #[serde(alias = "paintjob_explorer_nx_metallic_chrome")]
    PaintJobExplorerNXMetalicChrome,
    #[serde(alias = "paintjob_ferdelance_razormetal_gold")]
    PaintJobFerdelanceRazormetalGold,
    #[serde(alias = "paintjob_federation_corvette_vibrant_purple")]
    PaintJobFederationCorvetteVibrantPurple,
    #[serde(alias = "paintjob_federation_corvette_lrpo_azure")]
    PaintJobFederationCorvetteLRPOAzure,
    #[serde(alias = "paintjob_krait_mkii_salvation_09")]
    PaintJobKraitMkIISalvation09,
    #[serde(alias = "paintjob_krait_mkii_squadron_red")]
    PaintJobKraitMkIISquadronRed,
    #[serde(alias = "paintjob_krait_mkii_gradient2_red")]
    PaintJobKraitMkIIGradient2Red,
    #[serde(alias = "paintjob_krait_mkii_salvage_06")]
    PaintJobKraitMkIISalvage06,
    #[serde(alias = "paintjob_krait_mkii_egypt_01")]
    PaintJobKraitMkIIEgypt01,
    #[serde(alias = "paintjob_krait_mkii_thargoidreward3_01")]
    PaintJobKraitMkIIThargoidReward301,
    #[serde(alias = "paintjob_krait_mkii_specialeffectchristmas_01")]
    PaintJobKraitMkIISpecialEffectChristmas01,
    #[serde(alias = "paintjob_krait_light_tactical_red")]
    PaintJobKraitLightTacticalRed,
    #[serde(alias = "paintjob_krait_light_outrun_04")]
    PaintJobKraitLightOutrun04,
    #[serde(alias = "paintjob_krait_light_horus1_03")]
    PaintJobKraitLightHorus103,
    #[serde(alias = "paintjob_krait_mkii_lrpo_azure")]
    PaintJobKraitMkIILRPOAzure,
    #[serde(alias = "paintjob_lakonminer_02_10")]
    PaintJobLakonMiner0210,
    #[serde(alias = "paintjob_mamba_tactical_grey")]
    PaintJobMambaTacticalGrey,
    #[serde(alias = "paintjob_mandalay_01_08")]
    PaintJobMandalay0108,
    #[serde(alias = "paintjob_mandalay_02_02")]
    PaintJobMandalay0202,
    #[serde(alias = "paintjob_mandalay_blackfriday_01")]
    PaintJobMandalayBlackFriday01,
    #[serde(alias = "paintjob_panthermkii_01_11")]
    PaintJobPantherMkII0111,
    #[serde(alias = "paintjob_panthermkii_03_01")]
    PaintJobPantherMkII0301,
    #[serde(alias = "paintjob_panthermkii_03_08")]
    PaintJobPantherMkII0308,
    #[serde(alias = "paintjob_python_fullmetal_brass")]
    PaintJobPythonFullmetalBrass,
    #[serde(alias = "paintjob_python_halloween02_05")]
    PaintJobPythonHalloween0205,
    #[serde(alias = "paintjob_python_blackfriday_01")]
    PaintJobPythonBlackFriday01,
    #[serde(alias = "paintjob_python_lrpo_azure")]
    PaintJobPythonLRPOAzure,
    #[serde(alias = "paintjob_python_nx_metallic2_gold")]
    PaintJobPythonNXMetallic2Gold,
    #[serde(alias = "paintjob_python_nx_metallic_chrome")]
    PaintJobPythonNXMetallicChrome,
    #[serde(alias = "paintjob_python_nx_ruby_01")]
    PaintJobPythonNXRuby01,
    #[serde(alias = "paintjob_sidewinder_pilotreward_01")]
    PaintJobSidewinderPilotReward01,
    #[serde(alias = "paintjob_type8_convoy_06")]
    PaintJobType8Convoy06,
    #[serde(alias = "paintjob_type9_tactical_grey")]
    PaintJobType9TacticalGrey,
    #[serde(alias = "paintjob_type9_christmas_02")]
    PaintJobType9Christmas02,
    #[serde(alias = "paintjob_typex_thargoidreward4_01")]
    PaintJobTypeXThargoidReward401,
    #[serde(alias = "paintjob_typex_operator_pink")]
    PaintJobTypeXOperatorPink,
    #[serde(alias = "paintjob_typex_winter1_02")]
    PaintJobTypeXWinter102,
    #[serde(alias = "adder_armour_grade1")]
    AdderArmourGrade1,
    #[serde(alias = "adder_armour_grade3")]
    AdderArmourGrade3,
    #[serde(alias = "anaconda_armour_grade1")]
    AnacondaArmourGrade1,
    #[serde(alias = "anaconda_armour_grade2")]
    AnacondaArmourGrade2,
    #[serde(alias = "anaconda_armour_grade3")]
    AnacondaArmourGrade3,
    #[serde(alias = "anaconda_armour_reactive")]
    AnacondaArmourReactive,
    #[serde(alias = "asp_armour_grade1")]
    AspArmourGrade1,
    #[serde(alias = "asp_armour_grade3")]
    AspArmourGrade3,
    #[serde(alias = "belugaliner_armour_grade1")]
    BelugaLinerArmourGrade1,
    #[serde(alias = "cobramkiii_armour_grade1")]
    CobraMkIIIArmourGrade1,
    #[serde(alias = "cobramkv_armour_grade1")]
    CobraMkVArmourGrade1,
    #[serde(alias = "corsair_armour_grade1")]
    CorsairArmourGrade1,
    #[serde(alias = "corsair_armour_grade3")]
    CorsairArmourGrade3,
    #[serde(alias = "cutter_armour_grade1")]
    CutterArmourGrade1,
    #[serde(alias = "cutter_armour_grade3")]
    CutterArmourGrade3,
    #[serde(alias = "cutter_armour_reactive")]
    CutterArmourReactive,
    #[serde(alias = "diamondbackxl_armour_grade1")]
    DiamondBackXLArmourGrade1,
    #[serde(alias = "diamondbackxl_armour_grade3")]
    DiamondBackXLArmourGrade3,
    #[serde(alias = "diamondbackxl_armour_reactive")]
    DiamondBackXLArmourReactive,
    #[serde(alias = "empire_eagle_armour_grade1")]
    EmperialEagleArmourGrade1,
    #[serde(alias = "empire_trader_armour_grade1")]
    EmpireTraderArmourGrade1,
    #[serde(alias = "empire_trader_armour_grade3")]
    EmpireTraderArmourGrade3,
    #[serde(alias = "empire_courier_armour_grade1")]
    EmpireCourierArmourGrade1,
    #[serde(alias = "empire_courier_armour_grade3")]
    EmpireCourierArmourGrade3,
    #[serde(alias = "explorer_nx_armour_grade1")]
    ExplorerNXArmourGrade1,
    #[serde(alias = "federation_corvette_armour_reactive")]
    FederalCorvetteArmourReactive,
    #[serde(alias = "federation_corvette_armour_grade1")]
    FederalCorvetteArmourGrade1,
    #[serde(alias = "federation_corvette_armour_grade3")]
    FederalCorvetteArmourGrade3,
    #[serde(alias = "ferdelance_armour_reactive")]
    FerdelanceArmourReactive,
    #[serde(alias = "ferdelance_armour_grade1")]
    FerdelanceArmourGrade1,
    #[serde(alias = "ferdelance_armour_grade2")]
    FerdelanceArmourGrade2,
    #[serde(alias = "ferdelance_armour_grade3")]
    FerdelanceArmourGrade3,
    #[serde(alias = "krait_light_armour_grade1")]
    KraitLightArmourGrade1,
    #[serde(alias = "krait_mkii_armour_grade1")]
    KraitMkIIArmourGrade1,
    #[serde(alias = "krait_mkii_armour_grade3")]
    KraitMkIIArmourGrade3,
    #[serde(alias = "krait_mkii_armour_reactive")]
    KraitMkIIArmourReactive,
    #[serde(alias = "lakonminer_armour_grade1")]
    LakonMinerArmourGrade1,
    #[serde(alias = "mediumtransport01_armour_grade1")]
    LynxHighlinerArmourGrade1,
    #[serde(alias = "mamba_armour_grade1")]
    MambaArmourGrade1,
    #[serde(alias = "mamba_armour_grade3")]
    MambaArmourGrade3,
    #[serde(alias = "mandalay_armour_grade1")]
    MandalayArmourGrade1,
    #[serde(alias = "panthermkii_armour_grade1")]
    PantherMkIIArmourGrade1,
    #[serde(alias = "python_armour_grade1")]
    PythonArmourGrade1,
    #[serde(alias = "python_nx_armour_grade1")]
    PythonNXArmourGrade1,
    #[serde(alias = "python_nx_armour_reactive")]
    PythonNXArmourReactive,
    #[serde(alias = "sidewinder_armour_grade3")]
    SidewinderArmourGrade3,
    #[serde(alias = "sidewinder_armour_grade1")]
    SidewinderArmourGrade1,
    #[serde(alias = "type6_armour_grade1")]
    Type6ArmourGrade1,
    #[serde(alias = "type7_armour_grade1")]
    Type7ArmourGrade1,
    #[serde(alias = "type8_armour_grade1")]
    Type8ArmourGrade1,
    #[serde(alias = "type9_armour_grade1")]
    Type9ArmourGrade1,
    #[serde(alias = "type9_military_armour_grade1")]
    Type9MilitairyArmourGrade1,
    #[serde(alias = "type9_military_armour_grade3")]
    Type9MilitairyArmourGrade3,
    #[serde(alias = "typex_armour_grade1")]
    TypeXArmourGrade1,
    #[serde(alias = "typex_armour_grade3")]
    TypeXArmourGrade3,
    #[serde(alias = "vulture_armour_grade1")]
    VultureArmourGrade1,
    #[serde(alias = "vulture_armour_grade3")]
    VultureArmourGrade3,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size2_class1_name;",
        alias = "int_powerplant_size2_class1"
    )]
    PowerPlantS2C1,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size2_class3_name;",
        alias = "int_powerplant_size2_class3"
    )]
    PowerPlantS2C3,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size2_class5_name;",
        alias = "int_powerplant_size2_class5"
    )]
    PowerPlantS2C5,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size3_class1_name;",
        alias = "int_powerplant_size3_class1"
    )]
    PowerPlantS3C1,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size3_class3_name;",
        alias = "int_powerplant_size3_class3"
    )]
    PowerPlantS3C3,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size3_class5_name;",
        alias = "int_powerplant_size3_class5"
    )]
    PowerPlantS3C5,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size3_class4_name;",
        alias = "int_powerplant_size3_class4"
    )]
    PowerPlantS3C4,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size4_class1_name;",
        alias = "int_powerplant_size4_class1"
    )]
    PowerPlantS4C1,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size4_class2_name;",
        alias = "int_powerplant_size4_class2"
    )]
    PowerPlantS4C2,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size4_class3_name;",
        alias = "int_powerplant_size4_class3"
    )]
    PowerPlantS4C3,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size4_class5_name;",
        alias = "int_powerplant_size4_class5"
    )]
    PowerPlantS4C5,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size5_class2_name;",
        alias = "int_powerplant_size5_class2"
    )]
    PowerPlantS5C2,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size5_class3_name;",
        alias = "int_powerplant_size5_class3"
    )]
    PowerPlantS5C3,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size5_class4_name;",
        alias = "int_powerplant_size5_class4"
    )]
    PowerPlantS5C4,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size5_class5_name;",
        alias = "int_powerplant_size5_class5"
    )]
    PowerPlantS5C5,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size5_class1_name;",
        alias = "int_powerplant_size5_class1"
    )]
    PowerPlantS5C1,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size6_class1_name;",
        alias = "int_powerplant_size6_class1"
    )]
    PowerPlantS6C1,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size6_class3_name;",
        alias = "int_powerplant_size6_class3"
    )]
    PowerPlantS6C3,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size6_class4_name;",
        alias = "int_powerplant_size6_class4"
    )]
    PowerPlantS6C4,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size6_class5_name;",
        alias = "int_powerplant_size6_class5"
    )]
    PowerPlantS6C5,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size7_class1_name;",
        alias = "int_powerplant_size7_class1"
    )]
    PowerPlantS7C1,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size7_class2_name;",
        alias = "int_powerplant_size7_class2"
    )]
    PowerPlantS7C2,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size7_class3_name;",
        alias = "int_powerplant_size7_class3"
    )]
    PowerPlantS7C3,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size7_class5_name;",
        alias = "int_powerplant_size7_class5"
    )]
    PowerPlantS7C5,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size8_class1_name;",
        alias = "int_powerplant_size8_class1"
    )]
    PowerPlantS8C1,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size8_class2_name;",
        alias = "int_powerplant_size8_class2"
    )]
    PowerPlantS8C2,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size8_class3_name;",
        alias = "int_powerplant_size8_class3"
    )]
    PowerPlantS8C3,
    #[strum(to_string = "Power Plant")]
    #[serde(
        alias = "$int_powerplant_size8_class5_name;",
        alias = "int_powerplant_size8_class5"
    )]
    PowerPlantS8C5,
    #[strum(to_string = "Point Defence Turret")]
    #[serde(
        alias = "$hpt_plasmapointdefence_turret_tiny_name;",
        alias = "hpt_plasmapointdefence_turret_tiny"
    )]
    HptPlasmaPointDefenceTurretTiny,
    #[strum(to_string = "Fuel Tank")]
    #[serde(
        alias = "$int_fueltank_size1_class3_name;",
        alias = "int_fueltank_size1_class3"
    )]
    FuelTankS1C3,
    #[strum(to_string = "Fuel Tank")]
    #[serde(
        alias = "$int_fueltank_size2_class3_name;",
        alias = "int_fueltank_size2_class3"
    )]
    FuelTankS2C3,
    #[strum(to_string = "Fuel Tank")]
    #[serde(
        alias = "$int_fueltank_size3_class3_name;",
        alias = "int_fueltank_size3_class3"
    )]
    FuelTankS3C3,
    #[strum(to_string = "Fuel Tank")]
    #[serde(
        alias = "$int_fueltank_size4_class3_name;",
        alias = "int_fueltank_size4_class3"
    )]
    FuelTankS4C3,
    #[strum(to_string = "Fuel Tank")]
    #[serde(
        alias = "$int_fueltank_size5_class3_name;",
        alias = "int_fueltank_size5_class3"
    )]
    FuelTankS5C3,
    #[strum(to_string = "Fuel Tank")]
    #[serde(
        alias = "$int_fueltank_size6_class3_name;",
        alias = "int_fueltank_size6_class3"
    )]
    FuelTankS6C3,
    #[strum(to_string = "Fuel Tank")]
    #[serde(
        alias = "$int_fueltank_size7_class3_name;",
        alias = "int_fueltank_size7_class3"
    )]
    FuelTankS7C3,
    #[strum(to_string = "Chaff")]
    #[serde(
        alias = "$hpt_chafflauncher_tiny_name;",
        alias = "hpt_chafflauncher_tiny"
    )]
    HptChaffLauncherTiny,
    #[strum(to_string = "Plasma Acc")]
    #[serde(
        alias = "$hpt_plasmaaccelerator_fixed_huge_name;",
        alias = "hpt_plasmaaccelerator_fixed_huge"
    )]
    HptPlasmaAcceleratorFixedHuge,
    #[strum(to_string = "Plasma Acc")]
    #[serde(
        alias = "$hpt_plasmaaccelerator_fixed_large_name;",
        alias = "hpt_plasmaaccelerator_fixed_large"
    )]
    HptPlasmaAcceleratorFixedLarge,
    #[strum(to_string = "Plasma Acc")]
    #[serde(
        alias = "$hpt_plasmaaccelerator_fixed_medium_name;",
        alias = "hpt_plasmaaccelerator_fixed_medium"
    )]
    HptPlasmaAcceleratorFixedMedium,
    #[strum(to_string = "ECM")]
    #[serde(
        alias = "$hpt_electroniccountermeasure_tiny_name;",
        alias = "hpt_electroniccountermeasure_tiny"
    )]
    HptElectronicCounterMeasureTiny,
    #[strum(to_string = "Shield Cell Bank")]
    #[serde(
        alias = "$int_shieldcellbank_size2_class1_name;",
        alias = "int_shieldcellbank_size2_class1"
    )]
    ShieldCellBankS2C1,
    #[strum(to_string = "Shield Cell Bank")]
    #[serde(
        alias = "$int_shieldcellbank_size2_class2_name;",
        alias = "int_shieldcellbank_size2_class2"
    )]
    ShieldCellBankS2C2,
    #[strum(to_string = "Shield Cell Bank")]
    #[serde(
        alias = "$int_shieldcellbank_size2_class3_name;",
        alias = "int_shieldcellbank_size2_class3"
    )]
    ShieldCellBankS2C3,
    #[strum(to_string = "Shield Cell Bank")]
    #[serde(
        alias = "$int_shieldcellbank_size2_class5_name;",
        alias = "int_shieldcellbank_size2_class5"
    )]
    ShieldCellBankS2C5,
    #[strum(to_string = "Shield Cell Bank")]
    #[serde(
        alias = "$int_shieldcellbank_size3_class1_name;",
        alias = "int_shieldcellbank_size3_class1"
    )]
    ShieldCellBankS3C1,
    #[strum(to_string = "Shield Cell Bank")]
    #[serde(
        alias = "$int_shieldcellbank_size3_class2_name;",
        alias = "int_shieldcellbank_size3_class2"
    )]
    ShieldCellBankS3C2,
    #[strum(to_string = "Shield Cell Bank")]
    #[serde(
        alias = "$int_shieldcellbank_size3_class5_name;",
        alias = "int_shieldcellbank_size3_class5"
    )]
    ShieldCellBankS3C5,
    #[strum(to_string = "Shield Cell Bank")]
    #[serde(
        alias = "$int_shieldcellbank_size4_class1_name;",
        alias = "int_shieldcellbank_size4_class1"
    )]
    ShieldCellBankS4C1,
    #[strum(to_string = "Shield Cell Bank")]
    #[serde(
        alias = "$int_shieldcellbank_size4_class3_name;",
        alias = "int_shieldcellbank_size4_class3"
    )]
    ShieldCellBankS4C3,
    #[strum(to_string = "Shield Cell Bank")]
    #[serde(
        alias = "$int_shieldcellbank_size4_class4_name;",
        alias = "int_shieldcellbank_size4_class4"
    )]
    ShieldCellBankS4C4,
    #[strum(to_string = "Shield Cell Bank")]
    #[serde(
        alias = "$int_shieldcellbank_size4_class5_name;",
        alias = "int_shieldcellbank_size4_class5"
    )]
    ShieldCellBankS4C5,
    #[strum(to_string = "Shield Cell Bank")]
    #[serde(
        alias = "$int_shieldcellbank_size5_class3_name;",
        alias = "int_shieldcellbank_size5_class3"
    )]
    ShieldCellBankS5C3,
    #[strum(to_string = "Shield Cell Bank")]
    #[serde(
        alias = "$int_shieldcellbank_size5_class4_name;",
        alias = "int_shieldcellbank_size5_class4"
    )]
    ShieldCellBankS5C4,
    #[strum(to_string = "Shield Cell Bank")]
    #[serde(
        alias = "$int_shieldcellbank_size5_class5_name;",
        alias = "int_shieldcellbank_size5_class5"
    )]
    ShieldCellBankS5C5,
    #[strum(to_string = "Shield Cell Bank")]
    #[serde(
        alias = "$int_shieldcellbank_size6_class1_name;",
        alias = "int_shieldcellbank_size6_class1"
    )]
    ShieldCellBankS6C1,
    #[strum(to_string = "Shield Cell Bank")]
    #[serde(
        alias = "$int_shieldcellbank_size6_class2_name;",
        alias = "int_shieldcellbank_size6_class2"
    )]
    ShieldCellBankS6C2,
    #[strum(to_string = "Shield Cell Bank")]
    #[serde(
        alias = "$int_shieldcellbank_size6_class3_name;",
        alias = "int_shieldcellbank_size6_class3"
    )]
    ShieldCellBankS6C3,
    #[strum(to_string = "Shield Cell Bank")]
    #[serde(
        alias = "$int_shieldcellbank_size6_class4_name;",
        alias = "int_shieldcellbank_size6_class4"
    )]
    ShieldCellBankS6C4,
    #[strum(to_string = "Shield Cell Bank")]
    #[serde(
        alias = "$int_shieldcellbank_size6_class5_name;",
        alias = "int_shieldcellbank_size6_class5"
    )]
    ShieldCellBankS6C5,
    #[strum(to_string = "Shield Cell Bank")]
    #[serde(
        alias = "$int_shieldcellbank_size7_class4_name;",
        alias = "int_shieldcellbank_size7_class4"
    )]
    ShieldCellBankS7C4,
    #[strum(to_string = "Shield Cell Bank")]
    #[serde(
        alias = "$int_shieldcellbank_size7_class5_name;",
        alias = "int_shieldcellbank_size7_class5"
    )]
    ShieldCellBankS7C5,
    #[strum(to_string = "Shield Cell Bank")]
    #[serde(
        alias = "$int_shieldcellbank_size8_class2_name;",
        alias = "int_shieldcellbank_size8_class2"
    )]
    ShieldCellBankS8C2,
    #[strum(to_string = "Shield Cell Bank")]
    #[serde(
        alias = "$int_shieldcellbank_size8_class5_name;",
        alias = "int_shieldcellbank_size8_class5"
    )]
    ShieldCellBankS8C5,
    #[serde(alias = "anaconda_shipkit1_spoiler1")]
    AnacondaShipkit1Spoiler1,
    #[serde(alias = "anaconda_shipkit1_bumper2")]
    AnacondaShipkit1Bumper2,
    #[serde(alias = "anaconda_shipkit1_tail3")]
    AnacondaShipkit1Tail3,
    #[serde(alias = "anaconda_shipkit1_wings4")]
    AnacondaShipkit1Wings4,
    #[serde(alias = "asp_shipkit2raider_spoiler1")]
    AspShipkit2RaiderSpoiler1,
    #[serde(alias = "asp_shipkit2raider_wings2")]
    AspShipkit2RaiderWings2,
    #[serde(alias = "asp_shipkit2raider_tail3")]
    AspShipkit2RaiderTail3,
    #[serde(alias = "asp_shipkit2raider_bumper2")]
    AspShipkit2RaiderBumper2,
    #[serde(alias = "belugaliner_shipkit1_spoiler4")]
    BelugaLinerShipkit1Spoiler4,
    #[serde(alias = "belugaliner_shipkit1_wings3")]
    BelugaLinerShipkit1Wings3,
    #[serde(alias = "belugaliner_shipkit1_tail3")]
    BelugaLinerShipkit1Tail3,
    #[serde(alias = "belugaliner_shipkit1_bumper4")]
    BelugaLinerShipkit1Bumper4,
    #[serde(alias = "cobramkiii_shipkit1_spoiler1")]
    CobraMkIIIShipkit1Spoiler1,
    #[serde(alias = "cobramkiii_shipkit1_wings3")]
    CobraMkIIIShipkit1Wings3,
    #[serde(alias = "cobramkiii_shipkit1_tail3")]
    CobraMkIIIShipkit1Tail3,
    #[serde(alias = "cobramkiii_shipkit1_bumper1")]
    CobraMkIIIShipkit1Bumper1,
    #[serde(alias = "cutter_shipkit1_bumper1")]
    CutterShipkit1Bumper1,
    #[serde(alias = "cutter_shipkit1_wings2")]
    CutterShipkit1Wings2,
    #[serde(alias = "cutter_shipkit1_spoiler2")]
    CutterShipkit1Spoiler2,
    #[serde(alias = "explorer_nx_shipkita_spoiler1")]
    ExplorerNXShipkitASpoiler1,
    #[serde(alias = "explorer_nx_shipkita_spoiler2")]
    ExplorerNXShipkitASpoiler2,
    #[serde(alias = "explorer_nx_shipkita_wings1")]
    ExplorerNXShipkitAWings1,
    #[serde(alias = "explorer_nx_shipkita_wings2")]
    ExplorerNXShipkitAWings2,
    #[serde(alias = "explorer_nx_shipkita_bumper1")]
    ExplorerNXShipkitABumper1,
    #[serde(alias = "krait_mkii_shipkit1_wings2")]
    KraitMkIIShipkit1Wings2,
    #[serde(alias = "krait_mkii_shipkit1_spoiler3")]
    KraitMkIIShipkit1Spoiler3,
    #[serde(alias = "krait_mkii_shipkit1_tail4")]
    KraitMkIIShipkit1Tail4,
    #[serde(alias = "krait_mkii_shipkit1_bumper4")]
    KraitMkIIShipkit1Bumper4,
    #[serde(alias = "mamba_shipkit1_spoiler1")]
    MambaShipkit1Spoiler1,
    #[serde(alias = "mamba_shipkit1_bumper1")]
    MambaShipkit1Bumper1,
    #[serde(alias = "mamba_shipkit1_wings1")]
    MambaShipkit1Wings1,
    #[serde(alias = "mamba_shipkit1_tail3")]
    MambaShipkit1Tail3,
    #[serde(alias = "mandalay_shipkita_spoiler1")]
    MandalayShipkitASpoiler1,
    #[serde(alias = "mandalay_shipkita_spoiler2")]
    MandalayShipkitASpoiler2,
    #[serde(alias = "mandalay_shipkita_wings1")]
    MandalayShipkitAWings1,
    #[serde(alias = "mandalay_shipkita_bumper1")]
    MandalayShipkitABumper1,
    #[serde(alias = "mandalay_shipkita_bumper2")]
    MandalayShipkitABumper2,
    #[serde(alias = "python_shipkit1_spoiler3")]
    PythonShipkit1Spoiler3,
    #[serde(alias = "python_shipkit1_wings3")]
    PythonShipkit1Wings3,
    #[serde(alias = "python_shipkit1_tail3")]
    PythonShipkit1Tail3,
    #[serde(alias = "python_shipkit1_bumper3")]
    PythonShipkit1Bumper3,
    #[serde(alias = "voicepack_gerhard")]
    VoicePackGerhard,
    #[serde(alias = "voicepack_celeste")]
    VoicePackCeleste,
    #[serde(alias = "voicepack_verity")]
    VoicePackVerity,
    #[serde(alias = "voicepack_victor")]
    VoicePackVictor,
    #[serde(alias = "voicepack_jefferson")]
    VoicePackJefferson,
    #[serde(alias = "decal_alien_hunter1")]
    DecalAlienHunter1,
    #[serde(alias = "decal_fuelrats")]
    DecalFuelRats,
    #[serde(alias = "decal_caspianownersclub_01")]
    DecalCaspianOwnersClub01,
    #[serde(alias = "decal_combat_dangerous")]
    DecalCombatDangerous,
    #[serde(alias = "decal_combat_deadly")]
    DecalCombatDeadly,
    #[serde(alias = "decal_combat_master")]
    DecalCombatMaster,
    #[serde(alias = "decal_combat_elite")]
    DecalCombatElite,
    #[serde(alias = "decal_explorer_ranger")]
    DecalExplorerRanger,
    #[serde(alias = "decal_explorer_elite")]
    DecalExplorerElite,
    #[serde(alias = "decal_explorer_elite05")]
    DecalExplorerElite05,
    #[serde(alias = "decal_explorer_trailblazer")]
    DecalExplorerTrailblazer,
    #[serde(alias = "decal_explorer_starblazer")]
    DecalExplorerStarblazer,
    #[serde(alias = "decal_explorer_pathfinder")]
    DecalExplorerPathfinder,
    #[serde(alias = "decal_exploration_emissred")]
    DecalExplorationEmissRed,
    #[serde(alias = "decal_exobio_cataloguer")]
    DecalExobiologyCataloguer,
    #[serde(alias = "decal_exobio_geneticist")]
    DecalExobiologyGeneticist,
    #[serde(alias = "decal_exobio_ecologist")]
    DecalExobiologyEcologist,
    #[serde(alias = "decal_exobio_elite")]
    DecalExobiologyElite,
    #[serde(alias = "decal_lynxhighlinerownersclub_01")]
    DecalLynxHighlinerOwnersClub01,
    #[serde(alias = "decal_military_warrior")]
    DecalMilitaryWarrior,
    #[serde(alias = "decal_military_deadeye")]
    DecalMilitaryDeadeye,
    #[serde(alias = "decal_military_elite")]
    DecalMilitaryElite,
    #[serde(alias = "decal_military_elite01")]
    DecalMilitaryElite01,
    #[serde(alias = "decal_military_elite02")]
    DecalMilitaryElite02,
    #[serde(alias = "decal_military_elite03")]
    DecalMilitaryElite03,
    #[serde(alias = "decal_military_elite04")]
    DecalMilitaryElite04,
    #[serde(alias = "decal_military_elite05")]
    DecalMilitaryElite05,
    #[serde(alias = "decal_pantherownersclub_01")]
    DecalPantherOwnersClub01,
    #[serde(alias = "decal_powerplay_dynamic1")]
    DecalPowerplayDynamic1,
    #[serde(alias = "decal_powerplay_aislingduval")]
    DecalPowerplayAislingDuval,
    #[serde(alias = "decal_powerplay_duval")]
    DecalPowerplayDuval,
    #[serde(alias = "decal_powerplay_torval")]
    DecalPowerplayTorval,
    #[serde(alias = "decal_pilot_fed1")]
    DecalPilotFed1,
    #[serde(alias = "decal_ruby1")]
    DecalRuby1,
    #[serde(alias = "decal_ruby1_rubysilver")]
    DecalRubyRubySilver,
    #[serde(alias = "decal_ruby1_rubygold")]
    DecalRubyRubyGold,
    #[serde(alias = "decal_ruby1_gold")]
    DecalRuby1Gold,
    #[serde(alias = "decal_ruby1_silver")]
    DecalRuby1Silver,
    #[serde(alias = "decal_titanreward1")]
    DecalTitanReward1,
    #[serde(alias = "decal_titanreward2")]
    DecalTitanReward2,
    #[serde(alias = "decal_titanreward3")]
    DecalTitanReward3,
    #[serde(alias = "decal_titanreward3_black")]
    DecalTitanReward3Black,
    #[serde(alias = "decal_titanreward3_white")]
    DecalTitanReward3White,
    #[serde(alias = "decal_titanreward4")]
    DecalTitanReward4,
    #[serde(alias = "decal_titanreward5")]
    DecalTitanReward5,
    #[serde(alias = "decal_titanreward5_black")]
    DecalTitanReward5Black,
    #[serde(alias = "decal_titanreward5_white")]
    DecalTitanReward5White,
    #[serde(alias = "decal_titanreward6")]
    DecalTitanReward6,
    #[serde(alias = "decal_titanreward6_white")]
    DecalTitanReward6White,
    #[serde(alias = "decal_titanreward7")]
    DecalTitanReward7,
    #[serde(alias = "decal_titanreward7_white")]
    DecalTitanReward7White,
    #[serde(alias = "decal_trade_tycoon")]
    DecalTradeTycoon,
    #[serde(alias = "decal_trade_broker")]
    DecalTradeBroker,
    #[serde(alias = "decal_trade_elite")]
    DecalTradeElite,
    #[serde(alias = "decal_trade_elite01")]
    DecalTradeElite01,
    #[serde(alias = "decal_trade_elite02")]
    DecalTradeElite02,
    #[serde(alias = "decal_trade_elite03")]
    DecalTradeElite03,
    #[serde(alias = "decal_trade_elite04")]
    DecalTradeElite04,
    #[serde(alias = "decal_trade_elite05")]
    DecalTradeElite05,
    #[serde(alias = "decal_trade_entrepeneur")]
    DecalTradeEntrepeneur,
    #[serde(alias = "decal_triple_elite")]
    DecalTripleElite,
    #[serde(alias = "weaponcustomisation_red")]
    WeaponCustomisationRed,
    #[serde(alias = "weaponcustomisation_yellow")]
    WeaponCustomisationYellow,
    #[serde(alias = "weaponcustomisation_purple")]
    WeaponCustomisationPurple,
    #[serde(alias = "weaponcustomisation_green")]
    WeaponCustomisationGreen,
    #[serde(alias = "weaponcustomisation_blue")]
    WeaponCustomisationBlue,
    #[serde(alias = "weaponcustomisation_pink")]
    WeaponCustomisationPink,
    #[serde(alias = "weaponcustomisation_cyan")]
    WeaponCustomisationCyan,
    #[serde(alias = "enginecustomisation_yellow")]
    EngineCustomisationYellow,
    #[serde(alias = "enginecustomisation_red")]
    EngineCustomisationRed,
    #[serde(alias = "enginecustomisation_white")]
    EngineCustomisationWhite,
    #[serde(alias = "enginecustomisation_purple")]
    EngineCustomisationPurple,
    #[serde(alias = "enginecustomisation_pink")]
    EngineCustomisationPink,
    #[serde(alias = "enginecustomisation_blue")]
    EngineCustomisationBlue,
    #[serde(alias = "enginecustomisation_cyan")]
    EngineCustomisationCyan,
    #[serde(alias = "nameplate_explorer01_black")]
    NameplateExplorer01Black,
    #[serde(alias = "nameplate_explorer01_grey")]
    NameplateExplorer01Grey,
    #[serde(alias = "nameplate_explorer01_white")]
    NameplateExplorer01White,
    #[serde(alias = "nameplate_explorer02_black")]
    NameplateExplorer02Black,
    #[serde(alias = "nameplate_explorer02_grey")]
    NameplateExplorer02Grey,
    #[serde(alias = "nameplate_explorer02_white")]
    NameplateExplorer02White,
    #[serde(alias = "nameplate_explorer03_black")]
    NameplateExplorer03Black,
    #[serde(alias = "nameplate_explorer03_white")]
    NameplateExplorer03White,
    #[serde(alias = "nameplate_trader02_black")]
    NameplateTrader02Black,
    #[serde(alias = "nameplate_passenger02_black")]
    NameplatePassenger02Black,
    #[serde(alias = "nameplate_passenger02_white")]
    NameplatePassenger02White,
    #[serde(alias = "nameplate_practical01_black")]
    NameplatePractical01Black,
    #[serde(alias = "nameplate_practical01_white")]
    NameplatePractical01White,
    #[serde(alias = "nameplate_practical02_black")]
    NameplatePractical02Black,
    #[serde(alias = "nameplate_practical02_white")]
    NameplatePractical02White,
    #[serde(alias = "nameplate_practical03_black")]
    NameplatePractical03Black,
    #[serde(alias = "nameplate_practical03_white")]
    NameplatePractical03White,
    #[serde(alias = "nameplate_wings01_black")]
    NameplateWings01Black,
    #[serde(alias = "nameplate_wings01_white")]
    NameplateWings01White,
    #[serde(alias = "nameplate_wings02_black")]
    NameplateWings02Black,
    #[serde(alias = "nameplate_wings02_grey")]
    NameplateWings02Grey,
    #[serde(alias = "nameplate_wings02_white")]
    NameplateWings02White,
    #[serde(alias = "nameplate_wings03_black")]
    NameplateWings03Black,
    #[serde(alias = "nameplate_wings03_grey")]
    NameplateWings03Grey,
    #[serde(alias = "nameplate_wings03_white")]
    NameplateWings03White,
    #[serde(alias = "nameplate_shipname_black")]
    NameplateShipnameBlack,
    #[serde(alias = "nameplate_shipname_grey")]
    NameplateShipnameGrey,
    #[serde(alias = "nameplate_shipname_white")]
    NameplateShipnameWhite,
    #[serde(alias = "nameplate_shipid_black")]
    NameplateShipIDBlack,
    #[serde(alias = "nameplate_shipid_grey")]
    NameplateShipIDGrey,
    #[serde(alias = "nameplate_shipid_white")]
    NameplateShipIDWhite,
    #[serde(alias = "nameplate_shipid_doubleline_black")]
    NameplateShipIDDoubleLineBlack,
    #[serde(alias = "nameplate_shipid_doubleline_grey")]
    NameplateShipIDDoubleLineGrey,
    #[serde(alias = "nameplate_shipid_doubleline_white")]
    NameplateShipIDDoubleLineWhite,
    #[serde(alias = "nameplate_shipid_singleline_black")]
    NameplateShipIDSingleLineBlack,
    #[serde(alias = "nameplate_shipid_singleline_grey")]
    NameplateShipIDSingleLineGrey,
    #[serde(alias = "nameplate_shipid_singleline_white")]
    NameplateShipIDSingleLineWhite,
    #[serde(alias = "bobble_oldskool_thargoid")]
    BobbleOldskoolThargoid,
    #[serde(alias = "bobble_pp_jeromearcher")]
    BobblePPJeromeArcher,
    #[serde(alias = "bobble_trophy_combat")]
    BobbleTrophyCombat,
    #[serde(alias = "bobble_trophy_exploration")]
    BobbleTrophyExploration,
    #[serde(alias = "bobble_snowman")]
    BobbleSnowman,
    #[serde(alias = "bobble_ship_thargoid")]
    BobbleShipThargoid,
    #[serde(alias = "bobble_titan")]
    BobbleTitan,
    #[serde(alias = "bobble_planet_earth")]
    BobblePlanetEarth,
    #[serde(alias = "bobble_planet_saturn")]
    BobblePlanetSaturn,
    #[serde(alias = "bobble_ap2_textexclam")]
    BobbleAP2TextExclam,
    #[serde(alias = "bobble_ap2_texta")]
    BobbleAP2TextA,
    #[serde(alias = "bobble_ap2_textb")]
    BobbleAP2TextB,
    #[serde(alias = "bobble_ap2_textc")]
    BobbleAP2TextC,
    #[serde(alias = "bobble_ap2_textd")]
    BobbleAP2TextD,
    #[serde(alias = "bobble_ap2_texte")]
    BobbleAP2TextE,
    #[serde(alias = "bobble_ap2_textf")]
    BobbleAP2TextF,
    #[serde(alias = "bobble_ap2_textg")]
    BobbleAP2TextG,
    #[serde(alias = "bobble_ap2_texth")]
    BobbleAP2TextH,
    #[serde(alias = "bobble_ap2_texti")]
    BobbleAP2TextI,
    #[serde(alias = "bobble_ap2_textj")]
    BobbleAP2TextJ,
    #[serde(alias = "bobble_ap2_textk")]
    BobbleAP2TextK,
    #[serde(alias = "bobble_ap2_textl")]
    BobbleAP2TextL,
    #[serde(alias = "bobble_ap2_textm")]
    BobbleAP2TextM,
    #[serde(alias = "bobble_ap2_textn")]
    BobbleAP2TextN,
    #[serde(alias = "bobble_ap2_texto")]
    BobbleAP2TextO,
    #[serde(alias = "bobble_ap2_textp")]
    BobbleAP2TextP,
    #[serde(alias = "bobble_ap2_textq")]
    BobbleAP2TextQ,
    #[serde(alias = "bobble_ap2_textr")]
    BobbleAP2TextR,
    #[serde(alias = "bobble_ap2_texts")]
    BobbleAP2TextS,
    #[serde(alias = "bobble_ap2_textt")]
    BobbleAP2TextT,
    #[serde(alias = "bobble_ap2_textu")]
    BobbleAP2TextU,
    #[serde(alias = "bobble_ap2_textv")]
    BobbleAP2TextV,
    #[serde(alias = "bobble_ap2_textw")]
    BobbleAP2TextW,
    #[serde(alias = "bobble_ap2_textx")]
    BobbleAP2TextX,
    #[serde(alias = "bobble_ap2_texty")]
    BobbleAP2TextY,
    #[serde(alias = "bobble_ap2_textz")]
    BobbleAP2TextZ,
    #[serde(alias = "anaconda_thargoidreward3_spoiler1")]
    AnacondaThargoidReward3Spoiler1,
    #[serde(alias = "anaconda_thargoidreward3_wings1")]
    AnacondaThargoidReward3Wings1,
    #[serde(alias = "anaconda_thargoidreward3_bumper1")]
    AnacondaThargoidReward3Bumper1,
    #[serde(alias = "krait_mkii_thargoidreward3_spoiler1")]
    KraitMkIIThargoidReward3Spoiler1,
    #[serde(alias = "krait_mkii_thargoidreward3_wings1")]
    KraitMkIIThargoidReward3Wings1,
    #[serde(alias = "krait_mkii_thargoidreward3_bumper1")]
    KraitMkIIThargoidReward3Bumper1,
    #[serde(alias = "typex_thargoidreward4_spoiler1")]
    TypeXThargoidReward4Spoiler1,
    #[serde(alias = "typex_thargoidreward4_wings1")]
    TypeXThargoidReward4Wings1,
    #[serde(alias = "typex_thargoidreward4_bumper1")]
    TypeXThargoidReward4Bumper1,
    #[serde(alias = "string_lights_warm_white")]
    StringLightsWarmWhite,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-01-23T15:42:09Z", "event":"AfmuRepairs", "Module":"$explorer_nx_cockpit_name;", 
    "FullyRepaired":true, "Health":1.000000 })]
#[testcase({ "timestamp":"2026-03-27T18:42:08Z", "event":"AfmuRepairs", "Module":"$int_dockingcomputer_advanced_name;", "Module_Localised":"Docking Computer", "FullyRepaired":true, "Health":1.000000 })]
pub struct EDLogAfmuRepairs {
    pub module: ShipModule,
    #[serde(rename = "Module_Localised")]
    pub module_localised: Option<EDString>,
    pub fully_repaired: bool,
    pub health: f32,
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
