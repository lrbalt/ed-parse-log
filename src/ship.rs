use crate::{EDString, common_types::PilotRank, market::MarketItemType};
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
    #[serde(rename = "explorer_nx", alias = "Explorer_NX")]
    CaspianExplorer,
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
    IndependentFighter,
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
    #[serde(alias = "mediumtransport01", alias = "MediumTransport01")]
    LynxHighliner,
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
    #[serde(rename = "lakonminer", alias = "LakonMiner")]
    Type11Prospector,
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

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub enum ShipModule {
    #[strum(to_string = "FSD (SCO)")]
    #[serde(alias = "$int_hyperdrive_overcharge_size5_class5_name;")]
    FSDSCOS5C5,
    #[strum(to_string = "FSD (SCO)")]
    #[serde(alias = "$int_hyperdrive_overcharge_size8_class5_overchargebooster_mkii_name;")]
    FSDSCOS8C5OverchargeBoosterMkII,
    #[strum(to_string = "FSD (SCO)")]
    #[serde(alias = "$int_hyperdrive_overcharge_size6_class5_name;")]
    FSDSCOS6C5,
    #[strum(to_string = "Surface Scanner")]
    #[serde(alias = "$int_detailedsurfacescanner_tiny_name;")]
    SurfaceScanner,
    #[strum(to_string = "Guardian FSD Booster")]
    #[serde(alias = "$int_guardianfsdbooster_size5_name;")]
    GuardianFSDBooster,
    #[strum(to_string = "Thrusters")]
    #[serde(alias = "$int_engine_size6_class5_name;")]
    ThrustersS6C5,
    #[strum(to_string = "Thrusters")]
    #[serde(alias = "$int_engine_size5_class5_name;")]
    ThrustersS5C5,
    #[strum(to_string = "Thrusters")]
    #[serde(alias = "$int_engine_size7_class5_gravityoptimised_mkii_name;")]
    ThrustersS7C5GravityOptimisedMkII,
    #[strum(to_string = "Fighter Hangar")]
    #[serde(alias = "$int_fighterbay_size5_class1_name;")]
    FighterHangar,
    #[strum(to_string = "Heatsink")]
    #[serde(alias = "$hpt_heatsinklauncher_turret_tiny_name;")]
    Heatsink,
    #[strum(to_string = "Supercruise Assist")]
    #[serde(alias = "$int_supercruiseassist_name;")]
    SupercruiseAssist,
    #[strum(to_string = "Repair")]
    #[serde(alias = "$int_dronecontrol_repair_size3_class2_name;")]
    RepairS3C2,
    #[strum(to_string = "Repair")]
    #[serde(alias = "$int_dronecontrol_repair_size5_class2_name;")]
    RepairS5C2,
    #[strum(to_string = "Recon")]
    #[serde(alias = "$int_dronecontrol_recon_size1_class1_name;")]
    ReconS1C1,
    #[strum(to_string = "Research")]
    #[serde(alias = "$int_dronecontrol_unkvesselresearch_name;")]
    Research,
    #[strum(to_string = "Docking Computer")]
    #[serde(alias = "$int_dockingcomputer_advanced_name;")]
    DockingComputer,
    #[strum(to_string = "Cargo Hatch")]
    #[serde(alias = "$modularcargobaydoorfdl_name;")]
    CargoHatchFDL,
    #[strum(to_string = "Cargo Hatch")]
    #[serde(alias = "$modularcargobaydoor_name;")]
    CargoHatch,
    #[strum(to_string = "Life Support")]
    #[serde(alias = "$int_lifesupport_size4_class5_name;")]
    LifeSupportS4C5,
    #[strum(to_string = "Life Support")]
    #[serde(alias = "$int_lifesupport_size4_class2_name;")]
    LifeSupportS4C2,
    #[strum(to_string = "Life Support")]
    #[serde(alias = "$int_lifesupport_size5_class2_name;")]
    LifeSupportS5C2,
    #[strum(to_string = "Life Support")]
    #[serde(alias = "$int_lifesupport_size5_class5_name;")]
    LifeSupportS5C5,
    #[strum(to_string = "Fuel Scoop")]
    #[serde(alias = "$int_fuelscoop_size6_class5_name;")]
    FuelScoopS6C5,
    #[strum(to_string = "Fuel Scoop")]
    #[serde(alias = "$int_fuelscoop_size7_class5_name;")]
    FuelScoopS7C5,
    #[strum(to_string = "Sensors")]
    #[serde(alias = "$int_sensors_size5_class2_name;")]
    SensorsS5C2,
    #[strum(to_string = "Sensors")]
    #[serde(alias = "$int_sensors_size6_class2_name;")]
    SensorsS6C2,
    #[strum(to_string = "Sensors")]
    #[serde(alias = "$int_sensors_size8_class2_name;")]
    SensorsS8C2,
    #[strum(to_string = "Power Distributor")]
    #[serde(alias = "$int_powerdistributor_size6_class5_name;")]
    PowerDistributorS6C5,
    #[strum(to_string = "Power Distributor")]
    #[serde(alias = "$int_powerdistributor_size5_class5_name;")]
    PowerDistributorS5C5,
    #[strum(to_string = "Power Distributor")]
    #[serde(alias = "$int_powerdistributor_size7_class5_name;")]
    PowerDistributorS7C5,
    #[strum(to_string = "Planetary Vehicle Hangar")]
    #[serde(alias = "$int_buggybay_size4_class2_name;")]
    PlanetaryVehicleHangar,
    #[strum(to_string = "Remote Flak")]
    #[serde(alias = "$hpt_flakmortar_fixed_medium_name;")]
    RemoteFlakFixedMedium,
    #[strum(to_string = "Remote Flak")]
    #[serde(alias = "$hpt_flakmortar_turret_medium_name;")]
    RemoteFlakTurretMedium,
    #[strum(to_string = "AFM Unit")]
    #[serde(alias = "$int_repairer_size6_class5_name;")]
    AFMUnit,
    #[strum(to_string = "Bi-Weave Shield")]
    #[serde(alias = "$int_shieldgenerator_size6_class3_fast_name;")]
    BiWeaveShieldS6C3,
    #[strum(to_string = "Prismatic Shield")]
    #[serde(alias = "$int_shieldgenerator_size3_class5_strong_name;")]
    PrismaticShieldS3C5,
    #[strum(to_string = "Prismatic Shield")]
    #[serde(alias = "$int_shieldgenerator_size6_class5_strong_name;")]
    PrismaticShieldS6C5,
    #[strum(to_string = "Shield Generator")]
    #[serde(alias = "$int_shieldgenerator_size6_class2_name;")]
    ShieldGeneratorS6C2,
    #[strum(to_string = "Shield Generator")]
    #[serde(alias = "$int_shieldgenerator_size5_class2_name;")]
    ShieldGeneratorS5C2,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$typex_cockpit_name;")]
    TypeXCockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$python_nx_cockpit_name;")]
    PythonMkIICockpit,
    #[strum(to_string = "Cockpit")]
    #[serde(alias = "$explorer_nx_cockpit_name;")]
    CaspianExplorerCockpit,
    #[strum(to_string = "K-Warrant Scanner")]
    #[serde(alias = "$hpt_crimescanner_size0_class5_name;")]
    KWarrantScanner,
    #[strum(to_string = "Rail Gun")]
    #[serde(alias = "$hpt_railgun_fixed_medium_name;")]
    RailGun,
    #[strum(to_string = "Thrusters")]
    #[serde(alias = "$int_engine_size7_class5_name;")]
    ThrustersS7C5,
    #[strum(to_string = "Guardian Module Reinforcement")]
    #[serde(alias = "$int_guardianmodulereinforcement_size5_class2_name;")]
    GuardianModuleReinforcementS5C2,
    #[strum(to_string = "Module Reinforcement")]
    #[serde(alias = "$int_modulereinforcement_size3_class2_name;")]
    ModuleReinforcementS3C2,
    #[strum(to_string = "Module Reinforcement")]
    #[serde(alias = "$int_modulereinforcement_size4_class2_name;")]
    ModuleReinforcementS4C2,
    #[strum(to_string = "Module Reinforcement")]
    #[serde(alias = "$int_modulereinforcement_size1_class2_name;")]
    ModuleReinforcementS1C2,
    #[strum(to_string = "Module Reinforcement")]
    #[serde(alias = "$int_modulereinforcement_size2_class2_name;")]
    ModuleReinforcementS2C2,
    #[strum(to_string = "Module Reinforcement")]
    #[serde(alias = "$int_modulereinforcement_size5_class2_name;")]
    ModuleReinforcementS5C2,
    #[strum(to_string = "Shield Booster")]
    #[serde(alias = "$hpt_shieldbooster_size0_class1_name;")]
    ShieldBoosterS0C1,
    #[strum(to_string = "Shield Booster")]
    #[serde(alias = "$hpt_shieldbooster_size0_class5_name;")]
    ShieldBoosterS0C5,
    #[strum(to_string = "Power Distributor")]
    #[serde(alias = "$int_powerdistributor_size8_class5_name;")]
    PowerDistributorS8C5,
    #[strum(to_string = "Beam Laser")]
    #[serde(alias = "$hpt_beamlaser_gimbal_medium_name;")]
    BeamLaserMedium,
    #[strum(to_string = "Beam Laser")]
    #[serde(alias = "$hpt_beamlaser_gimbal_small_name;")]
    BeamLaserSmall,
    #[strum(to_string = "Gauss Cannon")]
    #[serde(alias = "$hpt_guardian_gausscannon_fixed_medium_name;")]
    GaussCannonFixedMedium,
    #[strum(to_string = "Gauss Cannon")]
    #[serde(alias = "$hpt_guardian_gausscannon_fixed_small_name;")]
    GaussCannonFixedSmall,
    #[strum(to_string = "Shard Cannon")]
    #[serde(alias = "$hpt_guardian_shardcannon_fixed_large_name;")]
    ShardCannonFixedLarge,
    #[strum(to_string = "Shard Cannon")]
    #[serde(alias = "$hpt_guardian_shardcannon_fixed_medium_name;")]
    ShardCannonFixedMedium,
    #[strum(to_string = "Plasma Charger")]
    #[serde(alias = "$hpt_guardian_plasmalauncher_fixed_medium_name;")]
    PlasmaChargerFixedMedium,
    #[strum(to_string = "Enhanced Xeno Scanner")]
    #[serde(alias = "$hpt_xenoscannermk2_basic_tiny_name;")]
    EnhancedXenoScanner,
    #[strum(to_string = "Caustic Sink Launcher")]
    #[serde(alias = "$hpt_causticsinklauncher_turret_tiny_name;")]
    CausticSinkLauncher,
    #[strum(to_string = "Enhanced AX Multi-Cannon")]
    #[serde(alias = "$hpt_atmulticannon_gimbal_medium_name;")]
    EnhancedAXMultiCannonGimbalMedium,
    #[strum(to_string = "Enhanced AX Multi-Cannon")]
    #[serde(alias = "$hpt_atmulticannon_gimbal_large_name;")]
    EnhancedAXMultiCannonGimbalLarge,
    #[strum(to_string = "TG Pulse Neutraliser")]
    #[serde(alias = "$hpt_antiunknownshutdown_tiny_v2_name;")]
    TGPulseNeutraliser,
    #[strum(to_string = "Shutdown Field Neutraliser")]
    #[serde(alias = "$hpt_antiunknownshutdown_tiny_name;")]
    ShutdownFieldNeutraliser,
    #[strum(to_string = "Seeker Missile Rack")]
    #[serde(alias = "$hpt_basicmissilerack_fixed_small_name;")]
    SeekerMissileRackFixedSmall,
    #[strum(to_string = "Pack-Hound")]
    #[serde(alias = "$hpt_drunkmissilerack_fixed_medium_name;")]
    PackHoundFixedMedium,
    #[strum(to_string = "Missile Rack")]
    #[serde(alias = "$hpt_dumbfiremissilerack_fixed_medium_name;")]
    MissileRackFixedMedium,
    #[strum(to_string = "AX Missile Rack")]
    #[serde(alias = "$hpt_atdumbfiremissile_fixed_medium_name;")]
    AXMissileRackFixeMedium,
    #[strum(to_string = "Enhanced AX Missile Rack")]
    #[serde(alias = "$hpt_atdumbfiremissile_fixed_medium_v2_name;")]
    EnhancedAXMissileRackFixedMediumV2,
    #[strum(to_string = "Enhanced AX Missile Rack")]
    #[serde(alias = "$hpt_atdumbfiremissile_fixed_large_v2_name;")]
    EnhancedAXMissileRackFixedLargeV2,
    #[strum(to_string = "Universal Multi-Limpet Controller")]
    #[serde(alias = "$int_multidronecontrol_universal_size7_class5_name;")]
    UniversalMultiLimpetController,
    #[strum(to_string = "Nanite Torpedo Pylon")]
    #[serde(alias = "$hpt_atventdisruptorpylon_fixed_medium_name;")]
    NaniteTorpedoPylon,
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
