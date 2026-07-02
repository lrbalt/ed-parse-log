use crate::{
    EDString,
    common_types::{PilotRank, Power},
    market::MarketItemType,
    ship_module::{ShipModule, ShipModuleSlot, serde_ship_module},
    ship_type::ShipType,
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
#[testcase_struct({ "Subsystem":"$int_powerdistributor_size5_class3_name;", 
    "Subsystem_Localised":"Power Distributor", "SubsystemHealth":98.958328 })]
pub struct TargetedSubsystem {
    #[serde(with = "serde_ship_module")]
    pub subsystem: ShipModule,
    #[serde(rename = "Subsystem_Localised")]
    pub subsystem_localised: EDString,
    pub subsystem_health: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2022-09-21T20:29:46Z", "event":"ShipTargeted", "TargetLocked":true, 
    "Ship":"vulture", "ScanStage":3, "PilotName":"$ShipName_Military_Independent;", 
    "PilotName_Localised":"System Defence Force", "PilotRank":"Elite", "ShieldHealth":0.000000, 
    "HullHealth":95.128372, "Faction":"Foxworks Celestial", "LegalStatus":"Lawless", "Bounty":0, 
    "Subsystem":"$int_powerdistributor_size5_class3_name;", 
    "Subsystem_Localised":"Power Distributor", "SubsystemHealth":98.958328 })]
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
pub enum ShipModule2 {
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
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-01-23T15:42:09Z", "event":"AfmuRepairs", "Module":"$explorer_nx_cockpit_name;", 
    "FullyRepaired":true, "Health":1.000000 })]
#[testcase({ "timestamp":"2026-03-27T18:42:08Z", "event":"AfmuRepairs", "Module":"$int_dockingcomputer_advanced_name;", "Module_Localised":"Docking Computer", "FullyRepaired":true, "Health":1.000000 })]
pub struct EDLogAfmuRepairs {
    #[serde(with = "serde_ship_module")]
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

#[test]
fn test_serde_ship_module() {
    use crate::{
        ship_module::{
            HardpointConnection::*,
            HardpointSize::*,
            ShipArmourGrade::*,
            ShipModule::*,
            ShipModuleClass::{self, *},
            ShipModuleExternal::*,
            ShipModuleHardpoint::*,
            ShipModuleInternal::*,
            ShipModuleSize::{self, *},
        },
        ship_type::ShipType::*,
    };

    fn test(name: &str, module: ShipModule) {
        #[derive(Deserialize, Debug)]
        struct Module {
            #[serde(with = "serde_ship_module")]
            module: ShipModule,
        }
        let s = format!("{{\"module\": \"{name}\"}}");
        let result: Result<Module, _> = serde_json::from_str(s.as_str());
        println!("{:?}", result);
        assert_eq!(module, result.unwrap().module);
    }

    test(
        "hpt_slugshot_fixed_large_range",
        Hardpoint(SlugshotRange, Fixed, Large),
    );
    test(
        "$ext_drive_class3_cob_name;",
        External(Drive, Class3, "cob".into()),
    );
    test(
        "$ext_drive_krait_light_name;",
        External(Drive, ShipModuleClass::None, "krait_light".into()),
    );
    test("$explorer_nx_cockpit_name;", Cockpit(CaspianExplorer));
    test(
        "$int_dockingcomputer_advanced_name;",
        Internal(
            DockingComputerAdvanced,
            ShipModuleSize::None,
            ShipModuleClass::None,
        ),
    );
    test(
        "$int_engine_size7_class5_gravityoptimised_mkii_name;",
        Internal(EngineGravityOptimisedMkII, Size7, Class5),
    );
    test(
        "paintjob_asp_halloween01_01",
        Paintjob(Asp, "halloween01_01".into()),
    );
    test(
        "mandalay_shipkita_bumper2",
        ShipKit(Mandalay, "shipkita_bumper2".into()),
    );
    test(
        "krait_mkii_thargoidreward3_bumper1",
        ShipKit(KraitMkII, "thargoidreward3_bumper1".into()),
    );
    test(
        "paintjob_smallcombat01_nx_03_01",
        Paintjob(SmallCombat01NX, "03_01".into()),
    );
    test(
        "$int_shieldgenerator_size3_class5_strong_name;",
        Internal(PrismaticShieldGenerator, Size3, Class5),
    );
    test("smallcombat01_nx_cockpit", Cockpit(SmallCombat01NX));
    test("$asp_armour_grade1;", Armour(Asp, Grade1));
    test(
        "smallcombat01_nx_armour_grade1",
        Armour(SmallCombat01NX, Grade1),
    );
    test(
        "$int_hyperdrive_size5_class5_name;",
        Internal(Hyperdrive, Size5, Class5),
    );
    test(
        "$int_hyperdrive_overcharge_size5_class5;",
        Internal(HyperdriveOvercharge, Size5, Class5),
    );
    test(
        "$int_powerdistributor_size5_class3_name;",
        Internal(PowerDistributor, Size5, Class3),
    );
    test(
        "$int_mkiiagileboost_engine_size5_class5_name;",
        Internal(EngineMkIIAgileBoost, Size5, Class5),
    );
    test(
        "$int_dronecontrol_fueltransfer_size1_class5_name;",
        Internal(DroneControlFuelTransfer, Size1, Class5),
    );
    test(
        "$int_multidronecontrol_xeno_size3_class3_name;",
        Internal(MultiDroneControlXeno, Size3, Class3),
    );
    test(
        "$int_mkii_passengercabin_size2_class1_name;",
        Internal(PassengerCabinMkII, Size2, Class1),
    );
    test(
        "$hpt_mining_subsurfdispmisle_fixed_medium_name;",
        Hardpoint(MiningSubsurfaceDisplacementMissile, Fixed, Medium),
    );
    test(
        "$hpt_guardian_plasmalauncher_fixed_medium_name;",
        Hardpoint(GuardianPlasmaLauncher, Fixed, Medium),
    );
    test("voicepack_gerhard", VoicePack("gerhard".into()));
    test(
        "decal_caspianownersclub_01",
        Decal("caspianownersclub_01".into()),
    );
    test(
        "hpt_shieldbooster_size0_class5",
        OptionalInternalSized(ShieldBooster, Size0, Class5),
    );
    test(
        "hpt_xenoscanner_basic_tiny",
        OptionalInternalHptSized(XenoScannerBasic, Tiny),
    );
    test(
        "hpt_antiunknownshutdown_tiny_v2",
        OptionalInternalHptSized(AntiUnknownShutdownV2, Tiny),
    );
    test(
        "int_detailedsurfacescanner_tiny_name",
        OptionalInternalHptSized(DetailedSurfaceScanner, Tiny),
    );
    test(
        "int_dronecontrol_unkvesselresearch",
        Internal(
            DroneControlResearch,
            ShipModuleSize::None,
            ShipModuleClass::None,
        ),
    );
    test("$modularcargobaydoor_name;", ModularCargoBayDoor);
    test(
        "$hpt_pulselaserburst_fixed_small_scatter_name;",
        Hardpoint(Cytoscrambler, Fixed, Small),
    );
    test(
        "$hpt_railgun_fixed_medium_burst_name;",
        Hardpoint(TheHammer, Fixed, Medium),
    );
    test(
        "$hpt_flakmortar_turret_medium_name;",
        Hardpoint(FlakMortar, Turret, Medium),
    );
}
