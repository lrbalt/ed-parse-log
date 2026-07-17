use crate::{
    EDString,
    common_types::{Credits, ModuleEngineeringModifiers},
    engineers::Engineer,
    ship_module::{ShipModule, ShipModuleSlot, serde_ship_module},
    ship_type::ShipType,
};
use ed_parse_log_files_macros::{Extractable, testcase, testcase_struct};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct FuelCapacity {
    pub main: f64,
    pub reserve: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
pub enum EngineeringBlueprint {
    #[serde(rename = "Armour_Advanced")]
    #[strum(to_string = "Lightweight")]
    ArmourAdvanced,
    #[serde(rename = "Armour_Explosive")]
    #[strum(to_string = "Blast Resistant")]
    ArmourExplosive,
    #[serde(rename = "Armour_HeavyDuty")]
    #[strum(to_string = "Heavy Duty")]
    ArmourHeavyDuty,
    #[serde(rename = "Armour_Kinetic")]
    #[strum(to_string = "Kinetic Resistant")]
    ArmourKinetic,
    #[serde(rename = "Armour_Thermic")]
    #[strum(to_string = "Thermal Resistant")]
    ArmourThermic,
    #[serde(rename = "CargoRack_IncreasedCapacity")]
    #[strum(to_string = "Increased Capacity")]
    CargoRackIncreasedCapacity,
    #[serde(rename = "CargoRackS6C1_Extended")]
    #[strum(to_string = "Increased Capacity")]
    CargoRackS6c1Extended,
    #[serde(rename = "Decorative_Red")]
    #[strum(to_string = "Decorative Red")]
    DecorativeRed,
    #[serde(rename = "Decorative_Yellow")]
    #[strum(to_string = "Decorative Yellow")]
    DecorativeYellow,
    #[serde(rename = "Decorative_Green")]
    #[strum(to_string = "Decorative Green")]
    DecorativeGreen,
    #[serde(rename = "Engine_Dirty")]
    #[strum(to_string = "Dirty Drive")]
    EngineDirty,
    #[serde(rename = "Engine_Tuned")]
    #[strum(to_string = "Clean Drive")]
    EngineTuned,
    #[serde(rename = "Engine_Reinforced")]
    #[strum(to_string = "Strengthened")]
    EngineReinforced,
    #[serde(rename = "FSDinterdictor_Expanded")]
    #[strum(to_string = "Expanded Capture Arc")]
    FsdinterdictorExpanded,
    #[serde(rename = "FSDinterdictor_LongRange")]
    #[strum(to_string = "Long Range")]
    FsdinterdictorLongRange,
    #[serde(rename = "FSD_LongRange")]
    #[strum(to_string = "Increased Range")]
    FSDLongRange,
    #[serde(rename = "FSD_FastBoot")]
    #[strum(to_string = "Faster Boot Sequence")]
    FSDFastBoot,
    #[serde(rename = "FSD_Shielded")]
    #[strum(to_string = "Shielded")]
    FSDShielded,
    #[serde(rename = "GuardianModule_Sturdy")]
    #[strum(to_string = "Sturdy")]
    GuardianModuleSturdy,
    #[serde(rename = "HullReinforcement_Advanced")]
    #[strum(to_string = "Lightweight")]
    HullReinforcementAdvanced,
    #[serde(rename = "HullReinforcement_Explosive")]
    #[strum(to_string = "Blast Resistant")]
    HullReinforcementExplosive,
    #[serde(rename = "HullReinforcement_HeavyDuty")]
    #[strum(to_string = "Heavy Duty")]
    HullReinforcementHeavyDuty,
    #[serde(rename = "HullReinforcement_Kinetic")]
    #[strum(to_string = "Kinetic Resistant")]
    HullReinforcementKinetic,
    #[serde(rename = "HullReinforcement_Thermic")]
    #[strum(to_string = "Thermal Resistant")]
    HullReinforcementThermic,
    #[serde(rename = "Misc_LightWeight")]
    #[strum(to_string = "Misc Lightweight")]
    MiscLightWeight,
    #[serde(rename = "Misc_Reinforced")]
    #[strum(to_string = "Misc Reinforced")]
    MiscReinforced,
    #[serde(rename = "Misc_Shielded")]
    #[strum(to_string = "Misc Shielded")]
    MiscShielded,
    #[serde(rename = "Misc_HeatSinkCapacity")]
    #[strum(to_string = "Heat Sink Capacity")]
    MiscHeatSinkCapacity,
    #[serde(rename = "Misc_ChaffCapacity")]
    #[strum(to_string = "Chaff Capacity")]
    MiscChaffCapacity,
    #[serde(rename = "Misc_PointDefenceCapacity")]
    #[strum(to_string = "Point Defence Capacity")]
    MiscPointDefenceCapacity,
    #[serde(rename = "PowerDistributor_HighCapacity")]
    #[strum(to_string = "High Charge Capacity")]
    PowerDistributorHighCapacity,
    #[serde(rename = "PowerDistributor_HighFrequency")]
    #[strum(to_string = "Charge Enhanced")]
    PowerDistributorHighFrequency,
    #[serde(rename = "PowerDistributor_PriorityEngines")]
    #[strum(to_string = "Engine Focused")]
    PowerDistributorPriorityEngines,
    #[serde(rename = "PowerDistributor_PrioritySystems")]
    #[strum(to_string = "System Focused")]
    PowerDistributorPrioritySystems,
    #[serde(rename = "PowerDistributor_PriorityWeapons")]
    #[strum(to_string = "Weapon Focused")]
    PowerDistributorPriorityWeapons,
    #[serde(rename = "PowerDistributor_Shielded")]
    #[strum(to_string = "Shielded")]
    PowerDistributorShielded,
    #[serde(rename = "PowerPlant_Armoured")]
    #[strum(to_string = "Armoured")]
    PowerPlantArmoured,
    #[serde(rename = "PowerPlant_Boosted")]
    #[strum(to_string = "Overcharged")]
    PowerPlantBoosted,
    #[serde(rename = "PowerPlant_Stealth")]
    #[strum(to_string = "Low Emissions")]
    PowerPlantStealth,
    #[serde(rename = "Sensor_Expanded")]
    #[strum(to_string = "Expanded Radius")]
    SensorExpanded,
    #[serde(rename = "Sensor_FastScan")]
    #[strum(to_string = "Fast Scan")]
    SensorFastScan,
    #[serde(rename = "Sensor_LightWeight")]
    #[strum(to_string = "Lightweight")]
    SensorLightWeight,
    #[serde(rename = "Sensor_LongRange")]
    #[strum(to_string = "Long Range")]
    SensorLongRange,
    #[serde(rename = "Sensor_WideAngle")]
    #[strum(to_string = "Wide Angle")]
    SensorWideAngle,
    #[serde(rename = "ShieldBooster_Explosive")]
    #[strum(to_string = "Blast Resistant")]
    ShieldBoosterExplosive,
    #[serde(rename = "ShieldBooster_HeavyDuty")]
    #[strum(to_string = "Heavy Duty")]
    ShieldBoosterHeavyDuty,
    #[serde(rename = "ShieldBooster_Resistive")]
    #[strum(to_string = "Resistance Augmented")]
    ShieldBoosterResistive,
    #[serde(rename = "ShieldBooster_Kinetic")]
    #[strum(to_string = "Kinetic Resistant")]
    ShieldBoosterKinetic,
    #[serde(rename = "ShieldBooster_Thermic")]
    #[strum(to_string = "Thermal Resistant")]
    ShieldBoosterThermic,
    #[serde(rename = "ShieldCellBank_Rapid")]
    #[strum(to_string = "Rapid Charge")]
    ShieldCellBankRapid,
    #[serde(rename = "ShieldCellBank_Specialised")]
    #[strum(to_string = "Specialised")]
    ShieldCellBankSpecialised,
    #[serde(rename = "ShieldGenerator_Kinetic")]
    #[strum(to_string = "Kinetic Resistant")]
    ShieldGeneratorKinetic,
    #[serde(rename = "ShieldGenerator_Optimised")]
    #[strum(to_string = "Enhanced Low Power")]
    ShieldGeneratorOptimised,
    #[serde(rename = "ShieldGenerator_Reinforced")]
    #[strum(to_string = "Reinforced")]
    ShieldGeneratorReinforced,
    #[serde(rename = "ShieldGenerator_Thermic")]
    #[strum(to_string = "Thermal Resistant")]
    ShieldGeneratorThermic,
    #[serde(rename = "Weapon_DoubleShot")]
    #[strum(to_string = "Double Shot")]
    WeaponDoubleShot,
    #[serde(rename = "Weapon_Efficient")]
    #[strum(to_string = "Efficient")]
    WeaponEfficient,
    #[serde(rename = "Weapon_Focussed")]
    #[strum(to_string = "Focused")]
    WeaponFocussed,
    #[serde(rename = "Weapon_HighCapacity")]
    #[strum(to_string = "High Capacity")]
    WeaponHighCapacity,
    #[serde(rename = "Weapon_LightWeight")]
    #[strum(to_string = "Light Weight")]
    WeaponLightWeight,
    #[serde(rename = "weapon_longrange", alias = "Weapon_LongRange")]
    #[strum(to_string = "Long Range")]
    WeaponLongrange,
    #[serde(rename = "Weapon_Overcharged")]
    #[strum(to_string = "Overcharged")]
    WeaponOvercharged,
    #[serde(rename = "Weapon_RapidFire")]
    #[strum(to_string = "Rapid Fire")]
    WeaponRapidFire,
    #[serde(rename = "Weapon_ShortRange")]
    #[strum(to_string = "Short Range")]
    WeaponShortRange,
    #[serde(rename = "Weapon_Sturdy")]
    #[strum(to_string = "Sturdy")]
    WeaponSturdy,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
#[serde(rename_all = "snake_case")]
pub enum EngineeringExperimentalEffect {
    #[strum[to_string = "Deep Plating"]]
    SpecialArmourChunky,
    #[strum[to_string = "Reflective Plating"]]
    SpecialArmourThermic,
    #[strum[to_string = "Auto Loader"]]
    SpecialAutoLoader,
    #[strum[to_string = "Corrosive Shell"]]
    SpecialCorrosiveShell,
    #[strum[to_string = "Dispersal Field"]]
    SpecialDispersalField,
    #[strum[to_string = "Drag Munitions"]]
    SpecialDragMunitions,
    #[strum[to_string = "Emissive Munitions"]]
    SpecialEmissiveMunitions,
    #[strum[to_string = "Thermal Spread"]]
    SpecialEngineCooled,
    #[strum[to_string = "Stripped Down"]]
    SpecialEngineLightweight,
    #[strum[to_string = "Drag Drives"]]
    SpecialEngineOverloaded,
    #[strum[to_string = "Double Braced"]]
    SpecialEngineToughened,
    #[strum[to_string = "Force Shell"]]
    SpecialForceShell,
    #[serde(rename = "special_fsd_fuelcapacity")]
    #[strum[to_string = "Deep Charge"]]
    SpecialFSDFuelcapacity,
    #[serde(rename = "special_fsd_heavy")]
    #[strum[to_string = "Mass Manager"]]
    SpecialFSDHeavy,
    #[strum[to_string = "High Yield Shell"]]
    SpecialHighYieldShell,
    #[strum[to_string = "Deep Plating"]]
    SpecialHullreinforcementChunky,
    #[strum[to_string = "Incendiary Rounds"]]
    SpecialIncendiaryRounds,
    #[strum(to_string = "Overload Munitions")]
    SpecialOverloadMunitions,
    #[strum(to_string = "Phasing Sequence")]
    SpecialPhasingSequence,
    #[strum[to_string = "$special_plasma_slug_name;"]]
    SpecialPlasmaSlugCooled,
    #[strum[to_string = "Super Conduits"]]
    SpecialPowerdistributorFast,
    #[strum[to_string = "Cluster Capacitors"]]
    SpecialPowerdistributorCapacity,
    #[strum[to_string = "Stripped Down"]]
    SpecialPowerdistributorLightweight,
    #[strum[to_string = "Thermal Spread"]]
    SpecialPowerplantCooled,
    #[strum[to_string = "Monstered"]]
    SpecialPowerplantHighcharge,
    #[strum[to_string = "Stripped Down"]]
    SpecialPowerplantLightweight,
    #[strum[to_string = "Scramble Spectrum"]]
    SpecialScrambleSpectrum,
    #[strum(to_string = "Screening Shell")]
    SpecialScreeningShell,
    #[strum(to_string = "Super Capacitors")]
    SpecialShieldboosterChunky,
    #[strum(to_string = "Flow Control")]
    SpecialShieldboosterEfficient,
    #[strum(to_string = "Force Block")]
    SpecialShieldboosterKinetic,
    #[strum(to_string = "Thermo Block")]
    SpecialShieldboosterThermic,
    #[strum(to_string = "Boss Cells")]
    SpecialShieldcellOversized,
    #[strum(to_string = "Hi-Cap")]
    SpecialShieldHealth,
    #[strum(to_string = "Stripped Down")]
    SpecialShieldLightweight,
    #[strum(to_string = "Penetrator Munitions")]
    SpecialPenetratorMunitions,
    #[strum(to_string = "Lo-draw")]
    SpecialShieldEfficient,
    #[strum(to_string = "Fast Charge")]
    SpecialShieldRegenerative,
    #[strum(to_string = "Thermo Block")]
    SpecialShieldThermic,
    #[strum(to_string = "Multi-weave")]
    SpecialShieldResistive,
    #[strum(to_string = "Thermal Cascade")]
    SpecialThermalCascade,
    #[strum(to_string = "Thermal Conduit")]
    SpecialThermalConduit,
    #[strum(to_string = "Thermal Vent")]
    SpecialThermalVent,
    #[strum(to_string = "Oversized")]
    SpecialWeaponDamage,
    #[strum(to_string = "Flow Control")]
    SpecialWeaponEfficient,
    #[strum(to_string = "Stripped Down")]
    SpecialWeaponLightweight,
    #[strum(to_string = "Multi-servos")]
    SpecialWeaponRateoffire,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({"Engineer":"Zacariah Nemo", "EngineerID":300050, "BlueprintID":128673459, "BlueprintName":"Weapon_Overcharged", 
    "Level":5, "Quality":1.000000, "ExperimentalEffect":"special_drag_munitions", "ExperimentalEffect_Localised":"Drag Munitions", "Modifiers":[]})]
pub struct ModuleEngineering {
    pub engineer: Option<Engineer>,
    #[serde(rename = "EngineerID")]
    pub engineer_id: u64,
    #[serde(rename = "BlueprintID")]
    pub blueprint_id: u64,
    pub blueprint_name: EngineeringBlueprint,
    pub level: u64,
    pub quality: f64,
    pub experimental_effect: Option<EngineeringExperimentalEffect>,
    #[serde(rename = "ExperimentalEffect_Localised")]
    pub experimental_effect_localised: Option<EDString>,
    pub modifiers: Vec<ModuleEngineeringModifiers>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Slot":"LargeHardpoint1", "Item":"hpt_slugshot_fixed_large_range", "On":true, "Priority":0, "AmmoInClip":3, "AmmoInHopper":180, "Health":1.000000, "Value":1536538})]
pub struct Module {
    pub slot: ShipModuleSlot,
    #[serde(with = "serde_ship_module")]
    pub item: ShipModule,
    pub on: bool,
    pub priority: u64,
    pub ammo_in_clip: Option<u64>,
    pub ammo_in_hopper: Option<u64>,
    pub health: f64,
    pub value: Option<Credits>,
    pub engineering: Option<ModuleEngineering>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LoadOutStats {
    pub hull_value: Option<Credits>,
    pub modules_value: Credits,
    pub hull_health: f64,
    pub unladen_mass: f64,
    pub cargo_capacity: u64,
    pub max_jump_range: f64,
    pub fuel_capacity: FuelCapacity,
    pub rebuy: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-10-15T04:34:40Z", "event":"Loadout", "Ship":"CobraMkIII", "ShipID":1, 
    "ShipName":"Flat Head", "ShipIdent":"UNSC-1", "Modules":[  ] })]
#[testcase({ "timestamp": "2024-01-14T18:13:22Z", "event": "Loadout", "Ship": "cobramkiii", "ShipID": 23, 
    "ShipName": "SCAVENGER", "ShipIdent": "LRB-C3", "HullValue": 349718, "ModulesValue": 6316388,
    "HullHealth": 1.000000, "UnladenMass": 222.577606, "CargoCapacity": 60, "MaxJumpRange": 39.726162,
    "FuelCapacity": {"Main": 16.000000,"Reserve": 0.490000},"Rebuy": 333307,"Modules": []})]
#[testcase({"timestamp": "2026-07-12T19:02:58Z","event": "Loadout", "Ship": "explorer_nx",
    "ShipID": 46, "ShipName": "my-caspian", "ShipIdent": "lrb-ce", "ModulesValue": 5585370,
    "HullHealth": 1.000000, "UnladenMass": 1480.151978, "CargoCapacity": 24, "MaxJumpRange": 76.526161, 
    "FuelCapacity": {"Main": 128.000000,"Reserve": 1.140000},"Rebuy": 279269,"Modules": []})]
pub struct EDLogLoadout {
    pub ship: ShipType,
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
    pub ship_name: EDString,
    pub ship_ident: EDString,
    pub hot: Option<bool>,
    #[serde(flatten)]
    pub loadout_stats: Option<LoadOutStats>,
    pub modules: Vec<Module>,
}

#[test]
fn test_loadoput() {
    let json = r#"{ "timestamp":"2026-07-12T19:02:58Z", "event":"Loadout", "Ship":"explorer_nx", "ShipID":46, 
        "ShipName":"my-caspian", "ShipIdent":"lrb-ce", "ModulesValue":5585370, "HullHealth":1.000000, 
        "UnladenMass":1480.151978, "CargoCapacity":24, "MaxJumpRange":76.526161, "FuelCapacity":{ "Main":128.000000, 
        "Reserve":1.140000 }, "Rebuy":279269, "Modules":[ ] }"#;
    let line: crate::log_line::EDLogLine = serde_json::from_str(json).expect("Should parse");

    assert!(matches!(
        line.event(),
        crate::log_line::EDLogEvent::Loadout(_)
    ));

    let loadout = line
        .event()
        .extract::<EDLogLoadout>()
        .expect("should be loadout inside");

    assert_eq!(loadout.ship_id, 46);
    assert_eq!(
        loadout
            .loadout_stats
            .as_ref()
            .expect("it to be there")
            .cargo_capacity,
        24
    );
}
