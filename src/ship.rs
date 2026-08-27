use crate::{
    EDString,
    market::MarketItemType,
    ship_module::{ShipModule, ShipModuleSlot, serde_ship_module},
};
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
    cargo_type: MarketItemType,
    #[serde(rename = "Type_Localised")]
    cargo_type_localised: Option<EDString>,
    count: u64,
    abandoned: bool,
    powerplay_origin: Option<EDString>,
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
    None,
    Clean,
    Wanted,
    WantedEnemy,
    Lawless,
    Enemy,
    Hunter,
    Thargoid22,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogRebootRepair {
    modules: Vec<ShipModuleSlot>,
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
            ShipModuleCoreInternal::*,
            ShipModuleExternal::*,
            ShipModuleHardpoint::*,
            ShipModuleOptionalInternal::*,
            ShipModuleSize::{self, *},
            ShipModuleUtilityMount::*,
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
    test(
        "$ext_drive_indfighter_name;",
        External(Drive, ShipModuleClass::None, "indfighter".into()),
    );
    test("$explorer_nx_cockpit_name;", Cockpit(CaspianExplorer));
    test(
        "$int_dockingcomputer_advanced_name;",
        OptionalInternal(
            DockingComputerAdvanced,
            ShipModuleSize::None,
            ShipModuleClass::None,
        ),
    );
    test(
        "$int_engine_size7_class5_gravityoptimised_mkii_name;",
        CoreInternal(EngineGravityOptimisedMkII, Size7, Class5),
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
        OptionalInternal(PrismaticShieldGenerator, Size3, Class5),
    );
    test("smallcombat01_nx_cockpit", Cockpit(SmallCombat01NX));
    test("$asp_armour_grade1;", Armour(Asp, Grade1));
    test(
        "smallcombat01_nx_armour_grade1",
        Armour(SmallCombat01NX, Grade1),
    );
    test(
        "$int_hyperdrive_size5_class5_name;",
        CoreInternal(Hyperdrive, Size5, Class5),
    );
    test(
        "$int_hyperdrive_overcharge_size5_class5;",
        CoreInternal(HyperdriveOvercharge, Size5, Class5),
    );
    test(
        "$int_powerdistributor_size5_class3_name;",
        CoreInternal(PowerDistributor, Size5, Class3),
    );
    test(
        "$int_mkiiagileboost_engine_size5_class5_name;",
        CoreInternal(EngineMkIIAgileBoost, Size5, Class5),
    );
    test(
        "$int_dronecontrol_fueltransfer_size1_class5_name;",
        OptionalInternal(DroneControlFuelTransfer, Size1, Class5),
    );
    test(
        "$int_multidronecontrol_xeno_size3_class3_name;",
        OptionalInternal(MultiDroneControlXeno, Size3, Class3),
    );
    test(
        "$int_mkii_passengercabin_size2_class1_name;",
        OptionalInternal(PassengerCabinMkII, Size2, Class1),
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
        UtilityMount(ShieldBooster, Size0, Class5),
    );
    test(
        "hpt_xenoscanner_basic_tiny",
        UtilityMount(
            XenoScannerBasic,
            ShipModuleSize::None,
            ShipModuleClass::None,
        ),
    );
    test(
        "hpt_antiunknownshutdown_tiny_v2",
        UtilityMount(
            AntiUnknownShutdownV2,
            ShipModuleSize::None,
            ShipModuleClass::None,
        ),
    );
    test(
        "int_detailedsurfacescanner_tiny_name",
        OptionalInternal(
            DetailedSurfaceScanner,
            ShipModuleSize::None,
            ShipModuleClass::None,
        ),
    );
    test(
        "int_dronecontrol_unkvesselresearch",
        OptionalInternal(
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
