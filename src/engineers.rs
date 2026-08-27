use crate::{
    EDString,
    common_types::ModuleEngineeringModifiers,
    market::MarketItemType,
    ship_module::{ShipModule, ShipModuleSlot, serde_ship_module},
};
use ed_parse_log_files_macros::{Extractable, testcase, testcase_struct};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
pub enum Engineer {
    #[strum(to_string = "Baltanos")]
    #[serde(rename = "Baltanos")]
    Baltanos,
    #[strum(to_string = "Bill Turner")]
    #[serde(rename = "Bill Turner")]
    BillTurner,
    #[strum(to_string = "Broo Tarquin")]
    #[serde(rename = "Broo Tarquin")]
    BrooTarquin,
    #[strum(to_string = "Chloe Sedesi")]
    #[serde(rename = "Chloe Sedesi")]
    ChloeSedesi,
    #[strum(to_string = "Colonel Bris Dekker")]
    #[serde(rename = "Colonel Bris Dekker")]
    ColonelBrisDekker,
    #[strum(to_string = "Didi Vatermann")]
    #[serde(rename = "Didi Vatermann")]
    DidiVatermann,
    #[strum(to_string = "Domino Green")]
    #[serde(rename = "Domino Green")]
    DominoGreen,
    #[strum(to_string = "The Dweller")]
    #[serde(rename = "The Dweller")]
    TheDweller,
    #[strum(to_string = "Elvira Martuuk")]
    #[serde(rename = "Elvira Martuuk")]
    ElviraMartuuk,
    #[strum(to_string = "Eleanor Bresa")]
    #[serde(rename = "Eleanor Bresa")]
    EleanorBresa,
    #[strum(to_string = "Etienne Dorn")]
    #[serde(rename = "Etienne Dorn")]
    EtienneDorn,
    #[strum(to_string = "Felicity Farseer")]
    #[serde(rename = "Felicity Farseer")]
    FelicityFarseer,
    #[strum(to_string = "Hera Tani")]
    #[serde(rename = "Hera Tani")]
    HeraTani,
    #[strum(to_string = "Hero Ferrari")]
    #[serde(rename = "Hero Ferrari")]
    HeroFerrari,
    #[strum(to_string = "Jude Navarro")]
    #[serde(rename = "Jude Navarro")]
    JudeNavarro,
    #[strum(to_string = "Juri Ishmaak")]
    #[serde(rename = "Juri Ishmaak")]
    JuriIshmaak,
    #[strum(to_string = "Kit Fowler")]
    #[serde(rename = "Kit Fowler")]
    KitFowler,
    #[strum(to_string = "Lei Cheung")]
    #[serde(rename = "Lei Cheung")]
    LeiCheung,
    #[strum(to_string = "Liz Ryder")]
    #[serde(rename = "Liz Ryder")]
    LizRyder,
    #[strum(to_string = "Lori Jameson")]
    #[serde(rename = "Lori Jameson")]
    LoriJameson,
    #[strum(to_string = "Marsha Hicks")]
    #[serde(rename = "Marsha Hicks")]
    MarshaHicks,
    #[strum(to_string = "Mel Brandon")]
    #[serde(rename = "Mel Brandon")]
    MelBrandon,
    #[strum(to_string = "Marco Qwent")]
    #[serde(rename = "Marco Qwent")]
    MarcoQwent,
    #[strum(to_string = "Oden Geiger")]
    #[serde(rename = "Oden Geiger")]
    OdenGeiger,
    #[strum(to_string = "Petra Olmanova")]
    #[serde(rename = "Petra Olmanova")]
    PetraOlmanova,
    #[strum(to_string = "Professor Palin")]
    #[serde(rename = "Professor Palin")]
    ProfessorPalin,
    #[strum(to_string = "Ram Tah")]
    #[serde(rename = "Ram Tah")]
    RamTah,
    #[strum(to_string = "The Sarge")]
    #[serde(rename = "The Sarge")]
    TheSarge,
    #[strum(to_string = "Rosa Dayette")]
    #[serde(rename = "Rosa Dayette")]
    RosaDayette,
    #[strum(to_string = "Selene Jean")]
    #[serde(rename = "Selene Jean")]
    SeleneJean,
    #[strum(to_string = "Terra Velasquez")]
    #[serde(rename = "Terra Velasquez")]
    TerraVelasquez,
    #[strum(to_string = "Tiana Fortune")]
    #[serde(rename = "Tiana Fortune")]
    TianaFortune,
    #[strum(to_string = "Tod 'The Blaster' McQuinn")]
    #[serde(rename = "Tod 'The Blaster' McQuinn")]
    TodTheBlasterMcQuinn,
    #[strum(to_string = "Uma Laszlo")]
    #[serde(rename = "Uma Laszlo")]
    UmaLaszlo,
    #[strum(to_string = "Wellington Beck")]
    #[serde(rename = "Wellington Beck")]
    WellingtonBeck,
    #[strum(to_string = "Yarden Bond")]
    #[serde(rename = "Yarden Bond")]
    YardenBond,
    #[strum(to_string = "Yi Shen")]
    #[serde(rename = "Yi Shen")]
    YiShen,
    #[strum(to_string = "Zacariah Nemo")]
    #[serde(rename = "Zacariah Nemo")]
    ZacariahNemo,
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
    #[serde(rename = "HeatSinkLauncher_LightWeight")]
    #[strum(to_string = "Lightweight")]
    HeatSinkLauncherLightWeight,
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
    #[serde(rename = "Sensor_Sensor_LightWeight")]
    #[strum(to_string = "Lightweight")]
    SensorSensorLightWeight,
    #[serde(rename = "Sensor_LightWeight")]
    #[strum(to_string = "Lightweight")]
    SensorLightWeight,
    #[serde(rename = "Sensor_LongRange")]
    #[strum(to_string = "Long Range")]
    SensorLongRange,
    #[serde(rename = "Sensor_Sensor_LongRange")]
    #[strum(to_string = "Long Range")]
    SensorSensorLongRange,
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
    #[serde(rename = "Weapon_Focused")]
    #[strum(to_string = "Focused")]
    WeaponFocused,
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
    #[strum[to_string = "Concordant Sequence"]]
    SpecialConcordantSequence,
    #[strum[to_string = "Dispersal Field"]]
    SpecialDispersalField,
    #[strum[to_string = "Drag Munitions"]]
    SpecialDragMunitions,
    #[strum[to_string = "Emissive Munitions"]]
    SpecialEmissiveMunitions,
    #[strum[to_string = "Thermal Spread"]]
    SpecialEngineCooled,
    #[strum[to_string = "Drive Distributors"]]
    SpecialEngineHaulage,
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
    #[strum[to_string = "Reflective Plating"]]
    SpecialHullreinforcementThermic,
    #[strum[to_string = "Angled Plating"]]
    SpecialHullreinforcementKinetic,
    #[strum[to_string = "Incendiary Rounds"]]
    SpecialIncendiaryRounds,
    #[strum(to_string = "Overload Munitions")]
    SpecialOverloadMunitions,
    #[strum(to_string = "Penetrator Munitions")]
    SpecialPenetratorMunitions,
    #[strum(to_string = "Phasing Sequence")]
    SpecialPhasingSequence,
    #[strum[to_string = "$special_plasma_slug_name;"]]
    SpecialPlasmaSlugCooled,
    #[strum[to_string = "Super Conduits"]]
    SpecialPowerdistributorFast,
    #[strum[to_string = "Cluster Capacitors"]]
    SpecialPowerdistributorCapacity,
    #[strum[to_string = "Flow Control"]]
    SpecialPowerdistributorEfficient,
    #[strum[to_string = "Stripped Down"]]
    SpecialPowerdistributorLightweight,
    #[strum[to_string = "Thermal Spread"]]
    SpecialPowerplantCooled,
    #[strum[to_string = "Monstered"]]
    SpecialPowerplantHighcharge,
    #[strum[to_string = "Stripped Down"]]
    SpecialPowerplantLightweight,
    #[strum[to_string = "Double Braced"]]
    SpecialPowerplantToughened,
    #[strum[to_string = "Regeneration Sequence"]]
    SpecialRegenerationSequence,
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
    #[strum(to_string = "Recycling Cell")]
    SpecialShieldcellGradual,
    #[strum(to_string = "Boss Cells")]
    SpecialShieldcellOversized,
    #[strum(to_string = "Hi-Cap")]
    SpecialShieldHealth,
    #[strum(to_string = "Force Block")]
    SpecialShieldKinetic,
    #[strum(to_string = "Stripped Down")]
    SpecialShieldLightweight,
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
    #[strum(to_string = "Thermal Shock")]
    #[serde(rename = "special_thermalshock")]
    SpecialThermalShock,
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
pub enum EngineerProgressState {
    Invited,
    Unlocked,
    Known,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Engineer":"Tod 'The Blaster' McQuinn", "EngineerID":300260, "Progress":"Known" })]
pub struct EngineerProgress {
    engineer: Option<Engineer>,
    #[serde(rename = "EngineerID")]
    engineer_id: Option<u64>,
    progress: EngineerProgressState,
    rank_progress: Option<u64>,
    rank: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2022-06-13T18:58:38Z", "event":"EngineerProgress", "Engineer":"Felicity Farseer", "EngineerID":300100, "Progress":"Invited" })]
#[testcase({ "timestamp":"2022-06-15T18:12:12Z", "event":"EngineerProgress", "Engineers":[ { "Engineer":"Tod 'The Blaster' McQuinn", "EngineerID":300260, "Progress":"Known" } ] })]
pub struct EDLogEngineerProgress {
    #[serde(flatten)]
    pub engineer: Option<EngineerProgress>,
    pub engineers: Option<Vec<EngineerProgress>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EngineerCraftIngredient {
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ExperimentalEffect {
    apply_experimental_effect: EngineeringExperimentalEffect,
    experimental_effect: EngineeringExperimentalEffect,
    #[serde(rename = "ExperimentalEffect_Localised")]
    experimental_effect_localised: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-07-12T17:50:29Z", "event":"EngineerCraft", "Slot":"TinyHardpoint3", "Module":"hpt_shieldbooster_size0_class5", "Ingredients":[ 
    { "Name":"conductiveceramics", "Name_Localised":"Conductive Ceramics", "Count":1 }, 
    { "Name":"refinedfocuscrystals", "Name_Localised":"Refined Focus Crystals", "Count":1 }, 
    { "Name":"imperialshielding", "Name_Localised":"Imperial Shielding", "Count":1 } ], 
    "Engineer":"Didi Vatermann", "EngineerID":300000, "BlueprintID":128673794, "BlueprintName":"ShieldBooster_Resistive", "Level":5, "Quality":0.400000, "Modifiers":[ 
        { "Label":"Integrity", "Value":42.239998, "OriginalValue":48.000000, "LessIsGood":0 }, 
        { "Label":"PowerDraw", "Value":1.500000, "OriginalValue":1.200000, "LessIsGood":1 }, 
        {"Label":"KineticResistance", "Value":15.200001, "OriginalValue":0.000000, "LessIsGood":0 }, 
        { "Label":"ThermicResistance", "Value":15.200001, "OriginalValue":0.000000, "LessIsGood":0 }, 
        { "Label":"ExplosiveResistance", "Value":15.200001, "OriginalValue":0.000000, "LessIsGood":0 } ] })]
pub struct EDLogEngineerCraft {
    pub slot: ShipModuleSlot,
    #[serde(with = "serde_ship_module")]
    pub module: ShipModule,
    pub ingredients: Vec<EngineerCraftIngredient>,
    pub engineer: Option<Engineer>,
    #[serde(rename = "EngineerID")]
    pub engineer_id: u64,
    #[serde(rename = "BlueprintID")]
    pub blueprint_id: u64,
    pub blueprint_name: EngineeringBlueprint,
    pub level: u64,
    pub quality: f64,
    #[serde(flatten)]
    pub experimental_effect: Option<ExperimentalEffect>,
    pub modifiers: Vec<ModuleEngineeringModifiers>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum EngineerContributionType {
    Bond,
    Bounty,
    Commodity,
    Credits,
    Materials,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2024-01-20T19:22:48Z", "event":"EngineerContribution", "Engineer":"Colonel Bris Dekker", 
    "EngineerID":300140, "Type":"Bond", "Quantity":126004, "TotalQuantity":1000000 })]
#[testcase({ "timestamp":"2024-01-03T19:35:39Z", "event":"EngineerContribution", "Engineer":"The Sarge", "EngineerID":300040, 
    "Type":"Materials", "Material":"shieldpatternanalysis", "Material_Localised":"Aberrant Shield Pattern Analysis", 
    "Quantity":50, "TotalQuantity":50 })]
#[testcase({ "timestamp":"2024-01-03T14:41:59Z", "event":"EngineerContribution", "Engineer":"Zacariah Nemo", "EngineerID":300050, 
    "Type":"Commodity", "Commodity":"xihecompanions", "Commodity_Localised":"Xihe Biomorphic Companions", "Quantity":5, 
    "TotalQuantity":25 })]
#[testcase({ "timestamp":"2023-07-23T21:19:45Z", "event":"EngineerContribution", "Engineer":"Mel Brandon", "EngineerID":300280, 
    "Type":"Bounty", "Quantity":100000, "TotalQuantity":100000 })]
pub struct EDLogEngineerContribution {
    pub engineer: Engineer,
    #[serde(rename = "EngineerID")]
    pub engineer_id: u64,
    #[serde(rename = "Type")]
    pub contribution_type: EngineerContributionType,
    pub commodity: Option<MarketItemType>,
    #[serde(rename = "Commodity_Localised")]
    pub commodity_localised: Option<EDString>,
    pub material: Option<EDString>,
    #[serde(rename = "Material_Localised")]
    pub material_localised: Option<EDString>,
    pub quantity: u64,
    pub total_quantity: u64,
}
