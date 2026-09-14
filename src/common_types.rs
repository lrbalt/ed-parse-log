use crate::{
    EDString,
    utils::{parse_number_of_days, string_or_struct},
};
use chrono::Duration;
use ed_parse_log_files_macros::testcase_struct;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign},
    str::FromStr,
};
use strum::Display;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Default)]
pub struct Merits(u64);

impl Merits {
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl Sub for Merits {
    type Output = Merits;

    fn sub(self, rhs: Self) -> Self::Output {
        Merits(self.0 - rhs.0)
    }
}

impl Display for Merits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Default)]
pub struct MercCoins(pub i64);

impl Display for MercCoins {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl MercCoins {
    pub fn to_human_readable_string(&self) -> String {
        crate::utils::to_human_readable_string(self.0)
    }

    pub fn value(&self) -> i64 {
        self.0
    }
}

impl Add for MercCoins {
    type Output = MercCoins;

    fn add(self, rhs: Self) -> Self::Output {
        MercCoins(self.0 + rhs.0)
    }
}

impl AddAssign for MercCoins {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for MercCoins {
    type Output = MercCoins;

    fn sub(self, rhs: Self) -> Self::Output {
        MercCoins(self.0 - rhs.0)
    }
}

impl SubAssign for MercCoins {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Sum for MercCoins {
    fn sum<I: Iterator<Item = MercCoins>>(iter: I) -> Self {
        iter.fold(MercCoins(0), |acc, x| acc + x)
    }
}

impl Mul<i64> for MercCoins {
    type Output = MercCoins;

    fn mul(self, rhs: i64) -> Self::Output {
        MercCoins(rhs * self.0)
    }
}

impl Mul<u64> for MercCoins {
    type Output = MercCoins;

    fn mul(self, rhs: u64) -> Self::Output {
        self * (rhs as i64)
    }
}

impl Neg for MercCoins {
    type Output = MercCoins;

    fn neg(self) -> Self::Output {
        MercCoins(-self.0)
    }
}

impl PartialOrd for MercCoins {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Default)]
pub struct Credits(pub i64);

impl Display for Credits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Add for Credits {
    type Output = Credits;

    fn add(self, rhs: Self) -> Self::Output {
        Credits(self.0 + rhs.0)
    }
}

impl AddAssign for Credits {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Credits {
    type Output = Credits;

    fn sub(self, rhs: Self) -> Self::Output {
        Credits(self.0 - rhs.0)
    }
}

impl SubAssign for Credits {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Sum for Credits {
    fn sum<I: Iterator<Item = Credits>>(iter: I) -> Self {
        iter.fold(Credits(0), |acc, x| acc + x)
    }
}

impl Mul<i64> for Credits {
    type Output = Credits;

    fn mul(self, rhs: i64) -> Self::Output {
        Credits(rhs * self.0)
    }
}

impl Mul<u64> for Credits {
    type Output = Credits;

    fn mul(self, rhs: u64) -> Self::Output {
        self * (rhs as i64)
    }
}

impl Neg for Credits {
    type Output = Credits;

    fn neg(self) -> Self::Output {
        Credits(-self.0)
    }
}

impl PartialOrd for Credits {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Credits {
    pub fn to_human_readable_string(&self) -> String {
        crate::utils::to_human_readable_string(self.0)
    }
}

pub const COMBAT_RANK: [&str; 14] = [
    "Harmless",
    "Mostly Harmless",
    "Novice",
    "Competent",
    "Expert",
    "Master",
    "Dangerous",
    "Deadly",
    "Elite",
    "Elite I",
    "Elite II",
    "Elite III",
    "Elite IV",
    "Elite V",
];

pub const TRADE_RANK: [&str; 14] = [
    "Penniless",
    "Mostly Penniless",
    "Peddler",
    "Dealer",
    "Merchant",
    "Broker",
    "Entrepreneur",
    "Tycoon",
    "Elite",
    "Elite I",
    "Elite II",
    "Elite III",
    "Elite IV",
    "Elite V",
];

pub const EXPLORE_RANK: [&str; 14] = [
    "Aimless",
    "Mostly Aimless",
    "Scout",
    "Surveyor",
    "Trailblazer",
    "Pathfinder",
    "Ranger",
    "Pioneer",
    "Elite",
    "Elite I",
    "Elite II",
    "Elite III",
    "Elite IV",
    "Elite V",
];

pub const SOLDIER_RANK: [&str; 14] = [
    "Defenceless",
    "Mostly Defenceless",
    "Rookie",
    "Soldier",
    "Gunslinger",
    "Warrior",
    "Gladiator",
    "Deadeye",
    "Elite",
    "Elite I",
    "Elite II",
    "Elite III",
    "Elite IV",
    "Elite V",
];

pub const EXOBIOLOGIST_RANK: [&str; 14] = [
    "Directionless",
    "Mostly Directionless",
    "Compiler",
    "Collector",
    "Cataloguer",
    "Taxonomist",
    "Ecologist",
    "Geneticist",
    "Elite",
    "Elite I",
    "Elite II",
    "Elite III",
    "Elite IV",
    "Elite V",
];

pub const CQC_RANK: [&str; 15] = [
    "None",
    "Helpless",
    "Mostly Helpless",
    "Amateur",
    "Semi Professional",
    "Professional",
    "Champion",
    "Hero",
    "Legend",
    "Elite",
    "Elite I",
    "Elite II",
    "Elite III",
    "Elite IV",
    "Elite V",
];

pub const EMPIRE_RANK: [&str; 15] = [
    "None", "Outsider", "Serf", "Master", "Squire", "Knight", "Lord", "Baron", "Viscount", "Count",
    "Earl", "Marquis", "Duke", "Prince", "King",
];

pub const FEDERATION_RANK: [&str; 15] = [
    "None",
    "Recruit",
    "Cadet",
    "Midshipman",
    "Petty Officer",
    "Chief Petty Officer",
    "Warrant Officer",
    "Ensign",
    "Lieutenant",
    "Leutenant Commander",
    "Post Commander",
    "Post Capatain",
    "Rear Admiral",
    "Vice Admiral",
    "Admiral",
];

// TODO: enumify
pub type CombatRank = u8;
pub type TradeRank = u8;
pub type ExploreRank = u8;
pub type SoldierRank = u8;
pub type ExobiologistRank = u8;
pub type EmpireRank = u8;
pub type FederationRank = u8;
pub type CQCRank = u8;

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
    #[serde(rename = "FuelScoop_Efficiency")]
    #[strum(to_string = "FuelScoop Efficiency")]
    FuelScoopEfficiency,
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

#[derive(Serialize, Deserialize, Clone, Debug, Copy, Display)]
pub enum GameMode {
    Group,
    Solo,
    Open,
    MainGame,
    Operation,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
// [x, y, z], in light years
pub struct StarPos([f64; 3]);

impl StarPos {
    pub fn x(&self) -> f64 {
        self.0[0]
    }
    pub fn y(&self) -> f64 {
        self.0[1]
    }
    pub fn z(&self) -> f64 {
        self.0[2]
    }
    pub fn distance_from_sol(&self) -> f64 {
        (self.x().powi(2) + self.y().powi(2) + self.z().powi(2)).sqrt()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Unknown {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CrewMemberRole {
    Active,
    Helm,
    OnShoreLeave,
    OnFoot,
    Idle,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
#[serde(rename_all = "lowercase")]
pub enum OnFootItem {
    AeroGel,
    #[strum(to_string = "Agricultural Sample")]
    AgriculturalProcessSample,
    #[strum(to_string = "Biochemical Agent")]
    BiochemicalAgent,
    #[serde(alias = "$californium_name;")]
    Californium,
    #[strum(to_string = "Cast Fossil")]
    CastFossil,
    #[strum(to_string = "Chemical Catalyst")]
    ChemicalCatalyst,
    #[strum(to_string = "Chemical Superbase")]
    ChemicalSuperbase,
    #[strum(to_string = "Chemical Sample")]
    ChemicalSample,
    #[strum(to_string = "Chemical Formulae")]
    ChemicalFormulae,
    #[strum(to_string = "Deep Mantle Sample")]
    DeepMantleSample,
    #[strum(to_string = "Genetic Repair Meds")]
    GeneticRepairMeds,
    #[strum(to_string = "Biological Sample")]
    GeneticSample,
    #[strum(to_string = "G-Meds")]
    GMeds,
    Graphene,
    Hush,
    Infinity,
    #[strum(to_string = "Inorganic Contaminant")]
    InorganicContaminant,
    Kompromat,
    Lazarus,
    #[strum(to_string = "Mutagenic Catalyst")]
    MutagenicCatalyst,
    #[strum(to_string = "Nutritional Concentrate")]
    NutritionalConcentrate,
    #[strum(to_string = "Oxygenic Bacteria")]
    OxygenicBacteria,
    #[strum(to_string = "Petrified Fossil")]
    PetrifiedFossil,
    #[strum(to_string = "pH Neutraliser")]
    PHNeutraliser,
    Push,
    RDX,
    #[strum(to_string = "Synthetic Genome")]
    SyntheticGenome,
    #[strum(to_string = "True Form Fossil")]
    TrueFormFossil,
    #[strum(to_string = "Tactical Plans")]
    TacticalPlans,
    #[strum(to_string = "Troop Deployment Records")]
    TroopDeploymentRecords,

    #[strum(to_string = "Medkit")]
    HealthPack,
    #[strum(to_string = "Energy Cell")]
    EnergyCell,
    #[serde(rename = "amm_grenade_emp")]
    #[strum(to_string = "Shield Disruptor")]
    AmmGrenadeEmp,
    #[serde(rename = "amm_grenade_frag")]
    #[strum(to_string = "Frag Grenade")]
    AmmGrenadeFrag,
    #[serde(rename = "amm_grenade_shield")]
    #[strum(to_string = "Shield Projector")]
    AmmGrenadeShield,

    Epinephrine,
    #[strum(to_string = "Microbial Inhibitor")]
    MicrobialInhibitor,
    #[strum(to_string = "Inertia Canister")]
    InertiaCanister,

    #[strum(to_string = "E-Breach")]
    Bypass,
    #[strum(to_string = "Carbon Fibre Plating")]
    CarbonfibrePlating,
    #[strum(to_string = "Circuit Switch")]
    CircuitSwitch,
    #[strum(to_string = "Circuit Board")]
    CircuitBoard,
    #[strum(to_string = "Compact Library")]
    CompactLibrary,
    #[strum(to_string = "Compression-Liquefied Gas")]
    CompressionLiquefiedGas,
    #[strum(to_string = "Degraded Power Regulator")]
    DegradedPowerRegulator,
    #[strum(to_string = "Electrical Wiring")]
    ElectricalWiring,
    #[strum(to_string = "Electrical Fuse")]
    ElectricalFuse,
    #[strum(to_string = "Electro Magnet")]
    ElectroMagnet,
    #[strum(to_string = "Encrypted Memory Chip")]
    EncryptedMemoryChip,
    #[strum(to_string = "Epoxy Adhesive")]
    EpoxyAdhesive,
    #[strum(to_string = "Health Monitor")]
    HealthMonitor,
    #[strum(to_string = "Ion Battery")]
    IonBattery,
    #[strum(to_string = "Ionised Gas")]
    IonisedGas,
    #[strum(to_string = "Power Regulator")]
    LargeCapacityPowerRegulator,
    #[strum(to_string = "Memory Chip")]
    MemoryChip,
    #[strum(to_string = "Metal Coil")]
    MetalCoil,
    #[strum(to_string = "Micro Electrode")]
    MicroElectrode,
    #[strum(to_string = "Micro Hydraulics")]
    MicroHydraulics,
    #[strum(to_string = "Micro Supercapacitor")]
    MicroSupercapacitor,
    #[strum(to_string = "Micro Thrusters")]
    MicroThrusters,
    #[strum(to_string = "Micro Transformer")]
    MicroTransformer,
    Motor,
    #[strum(to_string = "Optical Fibre")]
    OpticalFibre,
    #[strum(to_string = "Optical Lens")]
    OpticalLens,
    #[strum(to_string = "Personal Computer")]
    PersonalComputer,
    #[strum(to_string = "Personal Documents")]
    PersonalDocuments,
    #[strum(to_string = "Agricultural Sample")]
    PowerAgriculture,
    #[strum(to_string = "Power Classified Data")]
    PowerClassifiedData,
    #[strum(to_string = "Computer Parts")]
    PowerComputer,
    #[strum(to_string = "Electronics Package")]
    PowerElectronics,
    #[strum(to_string = "Personal Protective Equipment")]
    PowerEquipment,
    PowerEmployeeData,
    #[strum(to_string = "Experiment Prototype")]
    PowerExperiment,
    #[strum(to_string = "Extraction Sample")]
    PowerExtraction,
    #[strum(to_string = "Power Industrial Data")]
    PowerFinancialRecords,
    #[strum(to_string = "Industrial Component")]
    PowerIndustrial,
    #[strum(to_string = "Power Injection Malware")]
    PowerPreparationSpyware,
    #[strum(to_string = "Inventory Record")]
    PowerInventory,
    #[strum(to_string = "Medical Sample")]
    PowerMedical,
    #[strum(to_string = "Data Storage Device")]
    PowerMiscComputer,
    #[strum(to_string = "Industrial Machinery")]
    PowerMiscIndust,
    #[strum(to_string = "Energy Regulator")]
    PowerPower,
    #[strum(to_string = "Power Political Data")]
    PowerPropagandaData,
    PowerRegulator,
    #[strum(to_string = "Security Logs")]
    PowerSecurity,
    #[strum(to_string = "Power Tracker Malware")]
    PowerSpyware,
    #[strum(to_string = "Research Notes")]
    PowerResearch,
    #[strum(to_string = "Power Research Data")]
    PowerResearchData,
    #[strum(to_string = "Military Schematic")]
    PowerplayMilitary,
    #[strum(to_string = "Pyrolytic Catalyst")]
    PyrolyticCatalyst,

    Scrambler,
    #[strum(to_string = "Surveillance Equipment")]
    SurveillanceEquipment,
    #[strum(to_string = "Synthetic Pathogen")]
    SyntheticPathogen,
    Transmitter,
    #[strum(to_string = "Titanium Plating")]
    TitaniumPlating,
    #[strum(to_string = "Tungsten Carbide")]
    TungstenCarbide,
    #[strum(to_string = "Universal Translator")]
    UniversalTranslator,
    #[strum(to_string = "Viscoelastic Polymer")]
    ViscoElasticPolymer,
    #[strum(to_string = "Weapon Component")]
    WeaponComponent,

    #[strum(to_string = "Building Schematic")]
    BuildingSchematic,
    #[strum(to_string = "Ship Schematic")]
    ShipSchematic,
    #[strum(to_string = "Suit Schematic")]
    SuitSchematic,
    #[strum(to_string = "Vehicle Schematic")]
    VehicleSchematic,
    #[strum(to_string = "Weapon Schematic")]
    WeaponSchematic,

    #[strum(to_string = "Accident Logs")]
    AccidentLogs,
    #[strum(to_string = "Air Quality Reports")]
    AirQualityReports,
    #[strum(to_string = "Atmospheric Data")]
    AtmosphericData,
    #[strum(to_string = "AX Combat Logs")]
    AXcombatLogs,
    #[strum(to_string = "Audio Logs")]
    AudioLogs,
    #[strum(to_string = "Ballistics Data")]
    BallisticsData,
    #[strum(to_string = "Biological Weapon Data")]
    BiologicalWeaponData,
    #[strum(to_string = "Biomechanical Component")]
    BiomechanicalComponent,
    #[strum(to_string = "Biometric Data")]
    BiometricData,
    #[strum(to_string = "Blacklist Data")]
    BlacklistData,
    #[strum(to_string = "Blood Test Results")]
    BloodTestResults,
    #[strum(to_string = "Campaign Plans")]
    CampaignPlans,
    #[strum(to_string = "Cat Media")]
    CatMedia,
    #[strum(to_string = "Census Data")]
    CensusData,
    #[strum(to_string = "Chemical Experiment Date")]
    ChemicalExperimentData,
    #[strum(to_string = "Chemical Inventory")]
    ChemicalInventory,
    #[strum(to_string = "Chemical Patents")]
    ChemicalPatents,
    #[strum(to_string = "Chemical Process Sample")]
    ChemicalProcessSample,
    #[strum(to_string = "Chemical Weapon Data")]
    ChemicalWeaponData,
    #[strum(to_string = "Classic Entertainment")]
    ClassicEntertainment,
    #[strum(to_string = "Cocktail Recipes")]
    CocktailRecipes,
    #[strum(to_string = "Combatant Performance")]
    CombatantPerformance,
    #[strum(to_string = "Combat Training Material")]
    CombatTrainingMaterial,
    #[strum(to_string = "Conflict History")]
    ConflictHistory,
    #[strum(to_string = "Criminal Records")]
    CriminalRecords,
    #[strum(to_string = "Crop Yield Analysis")]
    CropYieldAnalysis,
    #[strum(to_string = "Culinary Recipes")]
    CulinaryRecipes,
    #[strum(to_string = "Digital Designs")]
    DigitalDesigns,
    #[strum(to_string = "Duty Rota")]
    DutyRota,
    #[strum(to_string = "Employee Directory")]
    EmployeeDirectory,
    #[strum(to_string = "Employee Expenses")]
    EmployeeExpenses,
    #[strum(to_string = "Employee Genetic Data")]
    EmployeeGeneticData,
    #[strum(to_string = "Employment History")]
    EmploymentHistory,
    #[strum(to_string = "Enhanced Interrogation Recordings")]
    EnhancedInterrogationRecordings,
    #[strum(to_string = "Exploration Journals")]
    ExplorationJournals,
    #[strum(to_string = "Extraction Yield Data")]
    ExtractionYieldData,
    #[strum(to_string = "Evacuation Protocols")]
    EvacuationProtocols,
    #[strum(to_string = "Faction Associates")]
    FactionAssociates,
    #[strum(to_string = "Faction Donator List")]
    FactionDonatorList,
    #[strum(to_string = "Faction News")]
    FactionNews,
    #[strum(to_string = "Financial Projections")]
    FinancialProjections,
    #[strum(to_string = "Fleet Registry")]
    FleetRegistry,
    #[strum(to_string = "Gene Sequencing Data")]
    GeneSequencingData,
    #[strum(to_string = "Genetic Research")]
    GeneticResearch,
    #[strum(to_string = "Geological Data")]
    GeologicalData,
    #[strum(to_string = "Hydroponic Data")]
    HydroponicData,
    #[strum(to_string = "Incident Logs")]
    IncidentLogs,
    #[strum(to_string = "Influence Projections")]
    InfluenceProjections,
    #[strum(to_string = "Insight Entertainment Suite")]
    InsightEntertainmentSuite,
    #[strum(to_string = "Insight Data Bank")]
    InsightDatabank,
    Insight,
    #[strum(to_string = "Internal Correspondence")]
    InternalCorrespondence,
    #[strum(to_string = "Interrogation Recordings")]
    InterrogationRecordings,
    #[strum(to_string = "Interview Recordings")]
    InterviewRecordings,
    #[strum(to_string = "Job Applications")]
    JobApplications,
    #[strum(to_string = "Literary Fiction")]
    LiteraryFiction,
    #[strum(to_string = "Manufacturing Instructions")]
    ManufacturingInstructions,
    #[strum(to_string = "Maintenance Logs")]
    MaintenanceLogs,
    #[strum(to_string = "Mineral Survey")]
    MineralSurvey,
    #[strum(to_string = "Mining Analytics")]
    MiningAnalytics,
    #[strum(to_string = "Meeting Minutes")]
    MeetingMinutes,
    #[strum(to_string = "Medical Records")]
    MedicalRecords,
    #[strum(to_string = "Medical Trial Records")]
    MedicalTrialRecords,
    #[strum(to_string = "Multimedia Entertainment")]
    MultimediaEntertainment,
    #[strum(to_string = "Network Access History")]
    NetworkAccessHistory,
    #[strum(to_string = "Network Security Protocols")]
    NetworkSecurityProtocols,
    #[strum(to_string = "Next Of Kin Records")]
    NextOfKinRecords,
    #[strum(to_string = "NOC Data")]
    NOCData,
    #[strum(to_string = "Operational Manual")]
    OperationalManual,
    #[strum(to_string = "Opinion Polls")]
    OpinionPolls,
    #[strum(to_string = "Patient History")]
    PatientHistory,
    #[strum(to_string = "Patrol Routes")]
    PatrolRoutes,
    #[strum(to_string = "Payroll Information")]
    PayrollInformation,
    #[strum(to_string = "Personal Logs")]
    PersonalLogs,
    #[strum(to_string = "Pharmaceutical Patents")]
    PharmaceuticalPatents,
    #[strum(to_string = "Photo Albums")]
    PhotoAlbums,
    #[strum(to_string = "Political Affiliations")]
    PoliticalAffiliations,
    #[strum(to_string = "Prisoner Logs")]
    PrisonerLogs,
    #[strum(to_string = "Production Reports")]
    ProductionReports,
    #[strum(to_string = "Production Schedule")]
    ProductionSchedule,
    #[strum(to_string = "Propaganda")]
    Propaganda,
    #[strum(to_string = "Purchase Records")]
    PurchaseRecords,
    #[strum(to_string = "Purchase Requests")]
    PurchaseRequests,
    #[strum(to_string = "Radioactivity Data")]
    RadioactivityData,
    #[strum(to_string = "Residential Directory")]
    ResidentialDirectory,
    #[strum(to_string = "Reactor Output Review")]
    ReactorOutputReview,
    #[strum(to_string = "Recycling Logs")]
    RecyclingLogs,
    #[strum(to_string = "Refinement Process Sample")]
    RefinementProcessSample,
    #[strum(to_string = "Risk Assessments")]
    RiskAssessments,
    #[strum(to_string = "Sales Records")]
    SalesRecords,
    #[strum(to_string = "Seed Geneaology")]
    SeedGeneaology,
    #[strum(to_string = "Security Expenses")]
    SecurityExpenses,
    #[strum(to_string = "Settlement Assault Plans")]
    SettlementAssaultPlans,
    #[strum(to_string = "Settlement Defence Plans")]
    SettlementDefencePlans,
    #[strum(to_string = "Shareholder Information")]
    ShareholderInformation,
    #[strum(to_string = "Slush Fund Logs")]
    SlushFundLogs,
    #[strum(to_string = "Smear Campaign Plans")]
    SmearCampaignPlans,
    #[strum(to_string = "Spectral Analysis Data")]
    SpectralAnalysisData,
    #[strum(to_string = "Stellar Activity Logs")]
    StellarActivityLogs,
    #[strum(to_string = "Surveillance Logs")]
    SurveilleanceLogs,
    #[strum(to_string = "Tax Records")]
    TaxRecords,
    #[strum(to_string = "Travel Permits")]
    TravelPermits,
    #[strum(to_string = "Topographical Surveys")]
    TopographicalSurveys,
    #[strum(to_string = "Union Membership")]
    UnionMembership,
    #[strum(to_string = "Vaccination Records")]
    VaccinationRecords,
    #[strum(to_string = "Vaccine Research")]
    VaccineResearch,
    #[strum(to_string = "VIP Security Detail")]
    VIPSecurityDetail,
    #[strum(to_string = "Virology Data")]
    VirologyData,
    #[strum(to_string = "Visitor Register")]
    VisitorRegister,
    #[strum(to_string = "Weapon Inventory")]
    WeaponInventory,
    #[strum(to_string = "Weapon Test Data")]
    WeaponTestData,
    #[strum(to_string = "Xeno Defence Protocols")]
    XenoDefenceProtocols,
    #[strum(to_string = "Operations Strike Data")]
    OperationsStrikeData,
    #[strum(to_string = "Operations Counter Attack Data")]
    OperationsCounterAttackData,

    #[serde(rename = "nm_seed")]
    #[strum(to_string = "Unica Seed")]
    NMSeed,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum TerraformState {
    #[serde(rename = "")]
    None,
    Terraformable,
    Terraforming,
    Terraformed,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, Display)]
pub enum PlanetClass {
    #[serde(rename = "Ammonia world")]
    #[strum(to_string = "Ammonia world")]
    AmmoniaWorld,
    #[serde(rename = "Earthlike body")]
    #[strum(to_string = "Earthlike body")]
    EarthlikeBody,
    #[serde(rename = "Gas giant with ammonia based life")]
    #[strum(to_string = "Gas giant with ammonia based life")]
    GasGiantWithAmmoniaBasedLife,
    #[serde(rename = "Gas giant with water based life")]
    #[strum(to_string = "Gas giant with water based life")]
    GasGiantWithWaterBasedLife,
    #[serde(rename = "High metal content body")]
    #[strum(to_string = "High metal content body")]
    HighMetalContentBody,
    #[serde(rename = "Helium rich gas giant")]
    #[strum(to_string = "Helium rich gas giant")]
    HeliumRichGasGiant,
    #[serde(rename = "Icy body")]
    #[strum(to_string = "Icy body")]
    IcyBody,
    #[serde(rename = "Metal rich body")]
    #[strum(to_string = "Metal rich body")]
    MetalRichBody,
    #[serde(rename = "Rocky ice body")]
    #[strum(to_string = "Rocky ice body")]
    RockyIceBody,
    #[serde(rename = "Rocky body")]
    #[strum(to_string = "Rocky body")]
    RockyBody,
    #[serde(rename = "Sudarsky class I gas giant")]
    #[strum(to_string = "Sudarsky class I gas giant")]
    SudarskyClassIGasGiant,
    #[serde(rename = "Sudarsky class II gas giant")]
    #[strum(to_string = "Sudarsky class II gas giant")]
    SudarskyClassIIGasGiant,
    #[serde(rename = "Sudarsky class III gas giant")]
    #[strum(to_string = "Sudarsky class III gas giant")]
    SudarskyClassIIIGasGiant,
    #[serde(rename = "Sudarsky class IV gas giant")]
    #[strum(to_string = "Sudarsky class IV gas giant")]
    SudarskyClassIVGasGiant,
    #[serde(rename = "Sudarsky class V gas giant")]
    #[strum(to_string = "Sudarsky class V gas giant")]
    SudarskyClassVGasGiant,
    #[serde(rename = "Water giant")]
    #[strum(to_string = "Water giant")]
    WaterGiant,
    #[serde(rename = "Water world")]
    #[strum(to_string = "Water world")]
    WaterWorld,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum AtmosphereType {
    None,

    Ammonia,
    AmmoniaOxygen,
    AmmoniaRich,
    Argon,
    ArgonRich,
    CarbonDioxide,
    CarbonDioxideRich,
    EarthLike,
    Hydrogen,
    Helium,
    Iron,
    MetallicVapour,
    Methane,
    MethaneRich,
    Neon,
    NeonRich,
    Nitrogen,
    Oxygen,
    Silicates,
    SilicateVapour,
    SulphurDioxide,
    Water,
    WaterRich,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display, PartialEq)]
pub enum BodySignalType {
    #[serde(rename = "$SAA_SignalType_Biological;")]
    Biological,
    #[serde(rename = "$SAA_SignalType_Geological;")]
    Geological,
    #[serde(rename = "$SAA_SignalType_Guardian;")]
    Guardian,
    #[serde(rename = "$SAA_SignalType_Human;")]
    Human,
    #[serde(rename = "$SAA_SignalType_Other;")]
    Other,
    #[serde(rename = "$SAA_SignalType_Thargoid;")]
    Thargoid,
    #[serde(rename = "$SAA_SignalType_PlanetAnomaly;")]
    #[strum(to_string = "Major Anomaly")]
    PlanetAnomaly,
    #[serde(rename = "$PlanetaryMiningLocation_Name;")]
    #[strum(to_string = "Planetary Mining Location")]
    PlanetaryMiningLocation,

    Alexandrite,
    Benitoite,
    Bromellite,
    Grandidierite,
    #[strum(to_string = "Low Temp. Diamonds")]
    LowTemperatureDiamond,
    Monazite,
    Musgravite,
    #[strum(to_string = "Void Opal")]
    Opal,
    Painite,
    Platinum,
    Rhodplumsite,
    Serendibite,
    #[serde(alias = "tritium")]
    Tritium,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ReserveLevel {
    #[serde(rename = "MajorResources")]
    Major,
    #[serde(rename = "PristineResources")]
    Pristine,
    #[serde(rename = "LowResources")]
    Low,
    #[serde(rename = "CommonResources")]
    Common,
    #[serde(rename = "DepletedResources")]
    Depleted,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RingClass {
    #[serde(rename = "eRingClass_Rocky")]
    Rocky,
    #[serde(rename = "eRingClass_Icy")]
    Icy,
    #[serde(rename = "eRingClass_MetalRich")]
    MetalRich,
    #[serde(rename = "eRingClass_Metalic")]
    Metalic,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FactionState {
    None,
    Expansion,
    Boom,
    Outbreak,
    War,
    Election,
    Bust,
    CivilWar,
    Drought,
    InfrastructureFailure,
    CivilUnrest,
    Terrorism,
    State,
    Blight,
    CivilLiberty,
    PublicHoliday,
    Retreat,
    PirateAttack,
    Famine,
    NaturalDisaster,
    Investment,
    Lockdown,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Display)]
pub enum BodyType {
    #[strum(to_string = "Bary Centre")]
    Null,
    Planet,
    Star,
    Station,
    #[strum(to_string = "Planetary Ring")]
    PlanetaryRing,
    #[strum(to_string = "Stellar Ring")]
    StellarRing,
    #[strum(to_string = "Asteroid Cluster")]
    AsteroidCluster,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum CrimeType {
    Assault,
    CollidedAtSpeedInNoFireZone,
    #[serde(rename = "collidedAtSpeedInNoFireZone_hulldamage")]
    CollidedAtSpeedInNoFireZoneHulldamage,
    DisobayPolice,
    DockingMajorBlockingAirlock,
    DockingMajorBlockingLandingPad,
    DockingMajorTresspass,
    DockingMinorBlockingAirlock,
    DockingMinorBlockingLandingPad,
    DockingMinorTresspass,
    DumpingDangerous,
    DumpingNearStation,
    FireInNoFireZone,
    FireInStation,
    IllegalCargo,
    Interdiction,
    Murder,
    Piracy,
    RecklessWeaponsDischarge,
    StationTamperingMinor,
    #[serde(rename = "onFoot_arcCutterUse")]
    OnFootArcCutterUse,
    #[serde(rename = "onFoot_carryingIllegalGoods")]
    OnFootCarryingIllegalGoods,
    #[serde(rename = "onFoot_damagingDefences")]
    OnFootDamagingDefences,
    #[serde(rename = "onFoot_dataTransfer")]
    OnFootDataTransfer,
    #[serde(rename = "onFoot_detectionOfWeapon")]
    OnFootDetectionOfWeapon,
    #[serde(rename = "onFoot_failureToSubmitToPolice")]
    OnFootFailureToSubmitToPolice,
    #[serde(rename = "onFoot_identityTheft")]
    OnFootIdentityTheft,
    #[serde(rename = "onFoot_murder")]
    OnFootMurder,
    #[serde(rename = "onFoot_profileCloningIntent")]
    OnFootCloningIntent,
    #[serde(rename = "onFoot_propertyTheft")]
    OnFootPropertyTheft,
    #[serde(rename = "onFoot_recklessEndangerment")]
    OnFootRecklessEndangerment,
    #[serde(rename = "onFoot_trespass")]
    OnFootTrespass,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CarrierDockingAccess {
    All,
    Friends,
    None,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
#[serde(rename_all = "PascalCase")]
pub enum StarClass {
    AeBe,
    A,
    #[serde(rename = "A_BlueWhiteSupergiant")]
    ABlueWhiteSupergiant,
    B,
    #[serde(rename = "B_BlueWhiteSuperGiant")]
    BBlueWhiteSuperGiant,
    C,
    CS,
    CJ,
    CN,
    CH,
    CHd,
    D,
    DA,
    DAB,
    DAO,
    DAV,
    DAZ,
    DB,
    DBV,
    DBZ,
    DC,
    DCV,
    DO,
    DOV,
    DQ,
    DX,
    F,
    #[serde(rename = "F_WhiteSupergiant")]
    FWhiteSupergiant,
    G,
    H,
    K,
    #[serde(rename = "K_OrangeGiant")]
    #[strum(to_string = "K Orange Giant")]
    KOrangeGiant,
    L,
    M,
    MS,
    #[serde(rename = "M_RedGiant")]
    #[strum(to_string = "M Red Giant")]
    MRedGiant,
    #[serde(rename = "M_RedASuperGiant")]
    #[strum(to_string = "M Red Super Giant")]
    MRedSuperGiant,
    N,
    Nebula,
    O,
    RoguePlanet,
    S,
    StellarRemnantNebula,
    #[strum(to_string = "Supermassive Blackhole")]
    SupermassiveBlackHole,
    T,
    TTS,
    W,
    WC,
    WO,
    WN,
    WNC,
    X,
    Y,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
pub enum LuminosityType {
    #[serde(rename = "0")]
    #[strum(to_string = "0")]
    Zero,
    #[serde(rename = "I")]
    #[strum(to_string = "I")]
    One,
    #[serde(rename = "Ia0")]
    #[strum(to_string = "Ia0")]
    OneAZero,
    #[serde(rename = "Ia")]
    #[strum(to_string = "Ia")]
    OneA,
    #[serde(rename = "Ib")]
    #[strum(to_string = "Ib")]
    OneB,
    #[serde(rename = "Iab")]
    #[strum(to_string = "Iab")]
    OneAB,
    #[serde(rename = "II")]
    #[strum(to_string = "II")]
    Two,
    #[serde(rename = "IIa")]
    #[strum(to_string = "IIa")]
    TwoA,
    #[serde(rename = "IIab")]
    #[strum(to_string = "IIab")]
    TwoAB,
    #[serde(rename = "IIb")]
    #[strum(to_string = "IIb")]
    TwoB,
    #[serde(rename = "III")]
    #[strum(to_string = "III")]
    Three,
    #[serde(rename = "IIIa")]
    #[strum(to_string = "IIIa")]
    ThreeA,
    #[serde(rename = "IIIab")]
    #[strum(to_string = "IIIab")]
    ThreeAB,
    #[serde(rename = "IIIb")]
    #[strum(to_string = "IIIb")]
    ThreeB,
    #[serde(rename = "IV")]
    #[strum(to_string = "IV")]
    Four,
    #[serde(rename = "IVa")]
    #[strum(to_string = "IVa")]
    FourA,
    #[serde(rename = "IVab")]
    #[strum(to_string = "IVab")]
    FourAB,
    #[serde(rename = "IVb")]
    #[strum(to_string = "IVb")]
    FourB,
    #[serde(rename = "V")]
    #[strum(to_string = "V")]
    Five,
    #[serde(rename = "Va")]
    #[strum(to_string = "Va")]
    FiveA,
    #[serde(rename = "Vab")]
    #[strum(to_string = "Vab")]
    FiveAB,
    #[serde(rename = "Vb")]
    #[strum(to_string = "Vb")]
    FiveB,
    #[serde(rename = "Vz")]
    #[strum(to_string = "Vz")]
    FiveZ,
    #[serde(rename = "VI")]
    #[strum(to_string = "VI")]
    Six,
    #[serde(rename = "VII")]
    #[strum(to_string = "V")]
    Seven,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SignalType {
    StationBernalSphere,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MaterialCategory {
    Manufactured,
    Encoded,
    Raw,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TraderType {
    Manufactured,
    Encoded,
    Raw,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ScanType {
    AutoScan,
    Detailed,
    Basic,
    Log,     // Organic
    Sample,  // Organic
    Analyse, // Organic
    NavBeacon,
    NavBeaconDetail,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SAASignalType {
    #[serde(rename = "tritium")]
    Tritium,
    Grandidierite,
    Opal,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
pub enum FSSSignalType {
    Generic,
    #[strum(to_string = "Resource Extraction")]
    ResourceExtraction,
    Combat,
    #[strum(to_string = "Navigation Beacon")]
    NavBeacon,
    Outpost,
    Installation,
    #[strum(to_string = "Coriolis Station")]
    StationCoriolis,
    #[strum(to_string = "Dodec Station")]
    StationDodec,
    #[strum(to_string = "Asteroid Station")]
    StationAsteroid,
    #[strum(to_string = "Bernal Sphere Station")]
    StationBernalSphere,
    #[strum(to_string = "O'Neil Orbis Station")]
    StationONeilOrbis,
    #[strum(to_string = "O'Neil Cylinder Station")]
    StationONeilCylinder,
    #[strum(to_string = "Fleet Carrier")]
    FleetCarrier,
    #[strum(to_string = "Squadron Carrier")]
    SquadronCarrier,
    #[strum(to_string = "Mega Ship")]
    Megaship,
    #[strum(to_string = "Mega Ship (Station)")]
    StationMegaShip,
    #[strum(to_string = "Tourist Beacon")]
    TouristBeacon,
    Titan,
    #[serde(rename = "USS")]
    Uss,
    Codex,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Display)]
pub enum StationType {
    #[serde(rename = "")]
    #[strum(to_string = "None")]
    None,
    #[strum(to_string = "Asteroid Base")]
    AsteroidBase,
    Bernal,
    Coriolis,
    #[strum(to_string = "Crater Outpost")]
    CraterOutpost,
    #[strum(to_string = "Crater Port")]
    CraterPort,
    Dodec,
    #[strum(to_string = "Dockable Planet Station")]
    DockablePlanetStation,
    #[strum(to_string = "Fleet Carrier")]
    FleetCarrier,
    #[strum(to_string = "Mega Ship")]
    MegaShip,
    Ocellus,
    #[strum(to_string = "On Foot Settlement")]
    OnFootSettlement,
    Orbis,
    Outpost,
    #[strum(to_string = "Planetary Construction Depot")]
    PlanetaryConstructionDepot,
    #[strum(to_string = "Space Construction Depot")]
    SpaceConstructionDepot,
    #[strum(to_string = "Surface Station")]
    SurfaceStation,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ShipScanType {
    Crime,
    Cargo,
    Data,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Allegiance {
    #[serde(rename = "")]
    None,
    Independent,
    PilotsFederation,
    Federation,
    Empire,
    Guardian,
    Thargoid,
    Alliance,
    FrontlineSolutions,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DroneType {
    Drones,
    Collection,
    Prospector,
    Repair,
    Decontamination,
    Recon,
    Research,
    FuelTransfer,
    Hatchbreaker,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Display)]
pub enum PowerplayState {
    Unoccupied,
    Exploited,
    Controlled,
    Fortified,
    Contested,
    #[strum(to_string = "Home System")]
    HomeSystem,
    Stronghold,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Display)]
pub enum Power {
    #[serde(rename = "A. Lavigny-Duval")]
    #[strum(to_string = "A. Lavigny-Duval")]
    ALavignyDuval,
    #[serde(rename = "Aisling Duval")]
    #[strum(to_string = "Aisling Duval")]
    AislingDuval,
    #[serde(rename = "Archon Delaine")]
    #[strum(to_string = "Archon Delaine")]
    ArchonDelaine,
    #[serde(rename = "Denton Patreus")]
    #[strum(to_string = "Denton Patreus")]
    DentonPatreus,
    #[serde(rename = "Edmund Mahon")]
    #[strum(to_string = "Edmund Mahon")]
    EdmundMahon,
    #[serde(rename = "Felicia Winters")]
    #[strum(to_string = "Felicia Winters")]
    FeliciaWinters,
    #[serde(rename = "Jerome Archer")]
    #[strum(to_string = "Jerome Archer")]
    JeromeArcher,
    #[serde(rename = "Li Yong-Rui")]
    #[strum(to_string = "Li Yong-Rui")]
    LiYongRui,
    #[serde(rename = "Nakato Kaine")]
    #[strum(to_string = "Nakato Kaine")]
    NakatoKaine,
    #[serde(rename = "Pranav Antal")]
    #[strum(to_string = "Pranav Antal")]
    PranavAntal,
    #[serde(rename = "Yuri Grom")]
    #[strum(to_string = "Yuri Grom")]
    YuriGrom,
    #[serde(rename = "Zachary Hudson")]
    #[strum(to_string = "Zachary Hudson")]
    ZacharyHudson,
    #[serde(rename = "Zemina Torval")]
    #[strum(to_string = "Zemina Torval")]
    ZeminaTorval,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VoucherType {
    #[serde(rename = "CombatBond")]
    CombatBond,
    Bounty,
    Trade,
    Settlement,
    Codex,
    Scannable,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LandingPads {
    pub small: u64,
    pub medium: u64,
    pub large: u64,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct PowerplayConflictProgress {
    pub power: Power,
    pub conflict_progress: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Powers {
    pub controlling_power: Option<Power>,
    pub powers: Vec<Power>,
    pub powerplay_state: PowerplayState,
    pub powerplay_conflict_progress: Option<Vec<PowerplayConflictProgress>>,
    pub powerplay_state_control_progress: Option<f64>,
    pub powerplay_state_reinforcement: Option<u64>,
    pub powerplay_state_undermining: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct BodyInformation {
    pub star_system: EDString,
    pub system_address: u64,
    pub body: EDString,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
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
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({"StationName":"Exogene Sciences", "StationType":"AsteroidBase", "MarketID":129038712, 
    "StationFaction":{ "Name":"October Consortium" }, "StationGovernment":"$government_Corporate;", 
    "StationGovernment_Localised":"Corporate", "StationServices":[ "dock", "autodock", "commodities", 
    "contacts", "exploration", "initiatives", "missions", "outfitting", "crewlounge", "rearm", "refuel", 
    "repair", "shipyard", "engineer", "facilitator", "flightcontroller", "stationoperations", "searchrescue", 
    "stationMenu", "livery", "socialspace", "bartender", "vistagenomics", "pioneersupplies", "apexinterstellar" ], 
    "StationEconomy":"$economy_Service;", "StationEconomy_Localised":"Service", "StationEconomies":[ 
    { "Name":"$economy_Service;", "Name_Localised":"Service", "Proportion":1.000000 } ] })]
pub struct StationInformation {
    // TODO: use StationIdentification
    pub station_name: EDString,
    #[serde(rename = "StationName_Localised")]
    pub station_name_localised: Option<EDString>,
    pub station_type: StationType,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub station_faction: FactionName,
    pub station_government: GovernmentType,
    #[serde(rename = "StationGovernment_Localised")]
    pub station_government_localised: EDString,
    pub station_allegiance: Option<Allegiance>,
    pub station_services: Vec<StationService>,
    pub station_economy: SystemEconomy,
    #[serde(rename = "StationEconomy_Localised")]
    pub station_economy_localised: EDString,
    pub station_economies: Vec<StationEconomy>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum StationService {
    #[serde(alias = "Autodock")]
    Autodock,
    #[serde(alias = "BlackMarket")]
    BlackMarket,
    #[serde(alias = "CarrierFuel")]
    CarrierFuel,
    #[serde(alias = "CarrierManagement")]
    CarrierManagement,
    #[serde(alias = "CarrierVendor")]
    CarrierVendor,
    #[serde(alias = "Commodities")]
    Commodities,
    #[serde(alias = "Contacts")]
    Contacts,
    #[serde(alias = "CrewLounge")]
    CrewLounge,
    #[serde(alias = "Dock")]
    Dock,
    #[serde(alias = "Engineer")]
    Engineer,
    #[serde(alias = "Exploration")]
    Exploration,
    #[serde(alias = "Facilitator")]
    Facilitator,
    #[serde(alias = "FlightController")]
    FlightController,
    #[serde(alias = "Initiatives")]
    Initiatives,
    #[serde(alias = "Livery")]
    Livery,
    #[serde(alias = "Missions")]
    Missions,
    #[serde(alias = "MissionsGenerated")]
    MissionsGenerated,
    #[serde(alias = "ModulePacks")]
    ModulePacks,
    #[serde(alias = "OnDockMission")]
    OnDockMission,
    #[serde(alias = "Outfitting")]
    Outfitting,
    #[serde(alias = "Powerplay")]
    Powerplay,
    #[serde(alias = "Rearm")]
    Rearm,
    #[serde(alias = "Refuel")]
    Refuel,
    #[serde(alias = "Repair")]
    Repair,
    #[serde(alias = "Research")]
    Research,
    #[serde(alias = "SearchAndRescue")]
    SearchRescue,
    #[serde(alias = "Shipyard")]
    Shipyard,
    #[serde(alias = "Shop")]
    Shop,
    #[serde(alias = "StationOperations")]
    StationOperations,
    #[serde(alias = "Tuning")]
    Tuning,
    #[serde(alias = "Workshop")]
    Workshop,
    #[serde(alias = "VoucherRedemption")]
    VoucherRedemption,
    // following found in logs, but not in manual
    ApexInterstellar,
    Bartender,
    ColonisationContribution,
    FrontlineSolutions,
    MaterialTrader,
    PioneerSupplies,
    Refinery,
    RegisteringColonisation,
    SocialSpace,
    #[serde(rename = "stationMenu")]
    StationMenu,
    #[serde(rename = "squadronBank")]
    SquadronBank,
    #[serde(rename = "techBroker")]
    TechBroker,
    VistaGenomics,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum WarType {
    Election,
    War,
    CivilWar,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SuitMod {
    #[serde(rename = "suit_backpackcapacity")]
    Backpackcapacity,
    #[serde(rename = "suit_increasedammoreserves")]
    IncreasedAmmoReserves,
    #[serde(rename = "suit_reducedtoolbatteryconsumption")]
    ReducedToolBatteryConsumption,
    #[serde(rename = "suit_increasedbatterycapacity")]
    IncreasedBatteryCapacity,
    #[serde(rename = "suit_nightvision")]
    NightVision,
    #[serde(rename = "suit_increasedshieldregen")]
    IncreasedShieldRegen,
    #[serde(rename = "suit_improvedjumpassist")]
    ImprovedJumpAssist,
    #[serde(rename = "suit_increasedsprintduration")]
    IncreasedSprintDuration,
    #[serde(rename = "suit_increasedmeleedamage")]
    IncreasedMeleeDamage,
    #[serde(rename = "suit_quieterfootsteps")]
    QuieterFootsteps,
    #[serde(rename = "suit_increasedo2capacity")]
    IncreasedO2Capacity,
    #[serde(rename = "suit_improvedradar")]
    ImprovedRadar,
    #[serde(rename = "suit_adsmovementspeed")]
    AdsMovementSpeed,
    #[serde(rename = "suit_improvedarmourrating")]
    ImprovedArmourRating,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum WeaponMod {
    #[serde(rename = "weapon_clipsize")]
    Clipsize,
    #[serde(rename = "weapon_range")]
    Range,
    #[serde(rename = "weapon_suppression_pressurised")]
    SuppressionPressurised,
    #[serde(rename = "weapon_backpackreloading")]
    BackpackReloading,
    #[serde(rename = "weapon_suppression_unpressurised")]
    SuppressionUnpressurised,
    #[serde(rename = "weapon_stability")]
    Stability,
    #[serde(rename = "weapon_scope")]
    Scope,
    #[serde(rename = "weapon_reloadspeed")]
    ReloadSpeed,
    #[serde(rename = "weapon_accuracy")]
    Accuracy,
    #[serde(rename = "weapon_handling")]
    Handling,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ConflictStatus {
    #[serde(rename = "")]
    None,
    Active,
    Pending,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ConflictFaction {
    name: EDString,
    stake: EDString,
    won_days: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Conflict {
    war_type: WarType,
    status: ConflictStatus,
    faction1: ConflictFaction,
    faction2: ConflictFaction,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ThargoidWarState {
    Unknown,
    #[serde(rename = "")]
    None,
    #[serde(rename = "Thargoid_Harvest")]
    Harvest,
    #[serde(rename = "Thargoid_Recovery")]
    Recovery,
    #[serde(rename = "Thargoid_Controlled")]
    Controlled,
    #[serde(rename = "Thargoid_Stronghold")]
    Stronghold,
    #[serde(rename = "Thargoid_Probing")]
    Probing,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct RemainingTime {
    #[serde(with = "parse_number_of_days")]
    estimated_remaining_time: Duration,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({"CurrentState": "Thargoid_Recovery","NextStateSuccess": "","NextStateFailure": "Thargoid_Recovery",
        "SuccessStateReached": false,"WarProgress": 0.000000,"RemainingPorts": 16,"EstimatedRemainingTime": "19 Days"})]
pub struct ThargoidWar {
    current_state: ThargoidWarState,
    next_state_success: ThargoidWarState,
    next_state_failure: ThargoidWarState,
    success_state_reached: bool,
    war_progress: f64,
    remaining_ports: u64,
    #[serde(flatten)]
    estimated_remaining_time: Option<RemainingTime>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct StationEconomy {
    name: SystemEconomy,
    #[serde(rename = "Name_Localised")]
    name_localised: EDString,
    proportion: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct FactionName {
    pub name: EDString,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<EDString>,
    pub faction_state: Option<FactionState>,
}

impl FromStr for FactionName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(FactionName {
            name: s.into(),
            name_localised: None,
            faction_state: None,
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct FactionRecoveringState {
    state: FactionState,
    trend: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct FactionActiveState {
    state: FactionState,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct FactionPendingState {
    state: FactionState,
    trend: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, Display)]
pub enum GovernmentType {
    #[serde(alias = "$government_Anarchy;")]
    Anarchy,
    #[serde(alias = "$government_Communism;")]
    #[strum(to_string = "Communist")]
    Communism,
    #[serde(alias = "$government_Confederacy;")]
    Confederacy,
    #[serde(alias = "$government_Carrier;")]
    Carrier,
    #[serde(alias = "$government_Cooperative;")]
    Cooperative,
    #[serde(alias = "$government_Corporate;")]
    Corporate,
    #[serde(alias = "$government_Democracy;")]
    Democracy,
    #[serde(alias = "$government_Dictatorship;")]
    Dictatorship,
    #[serde(alias = "$government_Engineer;")]
    Engineer,
    #[serde(alias = "$government_Feudal;")]
    Feudal,
    #[serde(alias = "$government_Megaconstruction;")]
    Megaconstruction,
    // Imperial,
    #[serde(alias = "$government_Patronage;")]
    Patronage,
    #[serde(alias = "$government_Prison;")]
    #[strum(to_string = "Detention Centre")]
    Prison,
    #[serde(alias = "$government_PrisonColony;")]
    #[strum(to_string = "Prison colony")]
    PrisonColony,
    #[serde(alias = "$government_Theocracy;")]
    Theocracy,
    #[serde(alias = "$government_None;")]
    None,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, Display)]
pub enum SystemEconomy {
    #[serde(alias = "$economy_Agri;")]
    Agriculture,
    #[serde(alias = "$economy_Colony;")]
    Colony,
    #[serde(alias = "$economy_Carrier;")]
    Carrier,
    #[serde(alias = "$economy_Damaged;")]
    Damaged,
    #[serde(alias = "$economy_Engineer;")]
    Engineer,
    #[serde(alias = "$economy_Extraction;")]
    Extraction,
    #[serde(alias = "$economy_HighTech;")]
    #[strum(to_string = "High Tech")]
    HighTech,
    #[serde(alias = "$economy_Industrial;")]
    Industrial,
    #[serde(alias = "$economy_Military;")]
    Military,
    #[serde(alias = "$economy_Prison;")]
    Prison,
    #[serde(alias = "$economy_Refinery;")]
    Refinery,
    #[serde(alias = "$economy_Rescue;")]
    Rescue,
    #[serde(alias = "$economy_Service;")]
    Service,
    #[serde(alias = "$economy_Terraforming;")]
    Terraforming,
    #[serde(alias = "$economy_Tourism;")]
    Tourism,
    #[serde(alias = "$economy_Undefined;")]
    #[strum(to_string = "Unknown")]
    Undefined,
    #[serde(alias = "$economy_None;")]
    None,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, Display)]
pub enum SystemSecurity {
    #[serde(alias = "$SYSTEM_SECURITY_high;")]
    #[strum(to_string = "High Security")]
    High,
    #[serde(alias = "$SYSTEM_SECURITY_medium;")]
    #[strum(to_string = "Medium Security")]
    Medium,
    #[serde(alias = "$SYSTEM_SECURITY_low;")]
    #[strum(to_string = "Low Security")]
    Low,
    #[serde(alias = "$GAlAXY_MAP_INFO_state_anarchy;")]
    Anarchy,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display, PartialEq)]
pub enum HappinessValue {
    #[serde(rename = "")]
    None,
    #[serde(rename = "$Faction_HappinessBand1;")]
    Elated,
    #[serde(rename = "$Faction_HappinessBand2;")]
    Happy,
    #[serde(rename = "$Faction_HappinessBand3;")]
    Discontented,
    #[serde(rename = "$Faction_HappinessBand4;")]
    Unhappy,
    #[serde(rename = "$Faction_HappinessBand5;")]
    Despondent,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Name":"Pilots Federation Local Branch", "FactionState":"None", 
    "Government":"Democracy", "Influence":0.000000, "Allegiance":"PilotsFederation" })]
#[testcase_struct({ "Name":"Murung Services", "FactionState":"Boom", "Government":"Corporate", 
    "Influence":0.332667, "Allegiance":"Federation" })]
pub struct Faction {
    pub name: EDString,
    pub faction_state: FactionState,
    pub government: GovernmentType,
    pub allegiance: EDString,
    pub influence: f64,
    pub happiness: Option<HappinessValue>,
    #[serde(rename = "Happiness_Localised")]
    pub happiness_localised: Option<EDString>,
    pub my_reputation: Option<f64>,
    pub pending_states: Option<Vec<FactionPendingState>>,
    pub recovering_states: Option<Vec<FactionRecoveringState>>,
    pub active_states: Option<Vec<FactionActiveState>>,
    pub squadron_faction: Option<bool>,
    pub happiest_system: Option<bool>,
    pub home_system: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EngineerModification {
    engineer_modifications: EDString,
    level: u64,
    quality: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ModifierValue {
    #[serde(rename = "Label_Localised")]
    label_localised: Option<EDString>,
    value: f64,
    original_value: f64,
    less_is_good: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ModifierDescription {
    value_str: EDString,
    #[serde(rename = "ValueStr_Localised")]
    value_str_localised: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "Label")]
#[testcase_struct({ "Label":"DamagePerSecond", "Value":367.200012, "OriginalValue":216.000000, "LessIsGood":0 })]
#[testcase_struct({ "Label":"DamageType", "ValueStr":"$Thermic;", "ValueStr_Localised":"Thermal" })]
#[testcase_struct({"Label": "$Kinetic;", "Label_Localised": "Kinetic", "Value": 0.000000, "OriginalValue": 100.000000, "LessIsGood": 0 })]
pub enum ModuleEngineeringModifiers {
    AmmoClipSize(ModifierValue),
    AmmoMaximum(ModifierValue),
    ArmourPenetration(ModifierValue),
    BootTime(ModifierValue),
    BrokenRegenRate(ModifierValue),
    BurstRateOfFire(ModifierValue),
    BurstSize(ModifierValue),
    CargoCapacity(ModifierValue),
    Damage(ModifierValue),
    DamageFalloffRange(ModifierValue),
    DamagePerSecond(ModifierValue),
    DamageType(ModifierDescription),
    DefenceModifierHealthAddition(ModifierValue),
    DefenceModifierHealthMultiplier(ModifierValue),
    DefenceModifierShieldMultiplier(ModifierValue),
    #[serde(rename = "DSS_PatchRadius")]
    DssPatchRadius(ModifierValue),
    DistributorDraw(ModifierValue),
    EnergyPerRegen(ModifierValue),
    EngineHeatRate(ModifierValue),
    EngineOptimalMass(ModifierValue),
    EngineOptPerformance(ModifierValue),
    EnginesCapacity(ModifierValue),
    EnginesRecharge(ModifierValue),
    #[serde(rename = "$Explosive;")]
    Explosive(ModifierValue),
    ExplosiveResistance(ModifierValue),
    FSDHeatRate(ModifierValue),
    FSDInterdictorRange(ModifierValue),
    FSDInterdictorFacingLimit(ModifierValue),
    FSDOptimalMass(ModifierValue),
    FuelScoopRate(ModifierValue),
    GuardianModuleResistance(ModifierDescription),
    HeatEfficiency(ModifierValue),
    Integrity(ModifierValue),
    Jitter(ModifierValue),
    #[serde(rename = "$Kinetic;")]
    Kinetic(ModifierValue),
    KineticResistance(ModifierValue),
    Mass(ModifierValue),
    MaxAngle(ModifierValue),
    MaxFuelPerJump(ModifierValue),
    MaximumRange(ModifierValue),
    PowerCapacity(ModifierValue),
    PowerDraw(ModifierValue),
    Range(ModifierValue),
    RateOfFire(ModifierValue),
    RegenRate(ModifierValue),
    ReloadTime(ModifierValue),
    ScannerRange(ModifierValue),
    ScannerTimeToScan(ModifierValue),
    SensorTargetScanAngle(ModifierValue),
    ShieldBankDuration(ModifierValue),
    ShieldBankHeat(ModifierValue),
    ShieldBankReinforcement(ModifierValue),
    ShieldBankSpinUp(ModifierValue),
    ShieldGenOptimalMass(ModifierValue),
    ShieldGenStrength(ModifierValue),
    ShotSpeed(ModifierValue),
    SystemsCapacity(ModifierValue),
    SystemsRecharge(ModifierValue),
    ThermalLoad(ModifierValue),
    #[serde(rename = "$Thermic;")]
    Thermic(ModifierValue),
    ThermicResistance(ModifierValue),
    WeaponsCapacity(ModifierValue),
    WeaponsRecharge(ModifierValue),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogName {
    name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PilotRank {
    #[serde(rename = "Mostly Harmless")]
    MostlyHarmless,
    Harmless,
    Novice,
    Competent,
    Expert,
    Dangerous,
    Deadly,
    Master,
    Elite,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TechBrokerType {
    Sirius,
    Rescue,
    Guardian,
    Human,
    Salvation,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct FuelCapacity {
    pub main: f64,
    pub reserve: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum VulcanismType {
    #[serde(rename = "")]
    None,
    #[serde(rename = "ammonia magma volcanism")]
    AmmoniaMagmaVolcanism,
    #[serde(rename = "minor ammonia magma volcanism")]
    MinorAmmoniaMagmaVolcanism,
    #[serde(rename = "major ammonia magma volcanism")]
    MajorAmmoniaMagmaVolcanism,
    #[serde(rename = "methane magma volcanism")]
    MethaneMagmaVolcanism,
    #[serde(rename = "major methane magma volcanism")]
    MajorMethaneMagmaVolcanism,
    #[serde(rename = "minor methane magma volcanism")]
    MinorMethaneMagmaVolcanism,
    #[serde(rename = "sulphur dioxide magma volcanism")]
    SulphurDioxideMagmaVolcanism,
    #[serde(rename = "minor sulphur dioxide magma volcanism")]
    MinorSulphurDioxideMagmaVolcanism,
    #[serde(rename = "major sulphur dioxide magma volcanism")]
    MajorSulphurDioxideMagmaVolcanism,
    #[serde(rename = "nitrogen magma volcanism")]
    NitrogenMagmaVolcanism,
    #[serde(rename = "minor nitrogen magma volcanism")]
    MinorNitrogenMagmaVolcanism,
    #[serde(rename = "major nitrogen magma volcanism")]
    MajorNitrogenMagmaVolcanism,
    #[serde(rename = "silicate magma volcanism")]
    SilicateMagmaVolcanism,
    #[serde(rename = "major silicate magma volcanism")]
    MajorSilicateMagmaVolcanism,
    #[serde(rename = "minor silicate magma volcanism")]
    MinorSilicateMagmaVolcanism,
    #[serde(rename = "metallic magma volcanism")]
    MetallicMagmaVolcanism,
    #[serde(rename = "major metallic magma volcanism")]
    MajorMetallicMagmaVolcanism,
    #[serde(rename = "minor metallic magma volcanism")]
    MinorMetallicMagmaVolcanism,
    #[serde(rename = "water magma volcanism")]
    WaterMagmaVolcanism,
    #[serde(rename = "major water magma volcanism")]
    MajorWaterMagmaVolcanism,
    #[serde(rename = "minor water magma volcanism")]
    MinorWaterMagmaVolcanism,

    #[serde(rename = "ammonia geysers volcanism")]
    AmmoniaGeyserVolcanism,
    #[serde(rename = "minor ammonia geysers volcanism")]
    MinorAmmoniaGeyserVolcanism,
    #[serde(rename = "major ammonia geysers volcanism")]
    MajorAmmoniaGeyserVolcanism,
    #[serde(rename = "carbon dioxide geysers volcanism")]
    CarbonDioxideGeyserVolcanism,
    #[serde(rename = "minor carbon dioxide geysers volcanism")]
    MinorCarbonDioxideGeyserVolcanism,
    #[serde(rename = "major carbon dioxide geysers volcanism")]
    MajorCarbonDioxideGeyserVolcanism,
    #[serde(rename = "helium geysers volcanism")]
    HeliumGeyserVolcanism,
    #[serde(rename = "major helium geysers volcanism")]
    MajorHeliumGeyserVolcanism,
    #[serde(rename = "minor helium geysers volcanism")]
    MinorHeliumGeyserVolcanism,
    #[serde(rename = "methane geysers volcanism")]
    MethaneGeyserVolcanism,
    #[serde(rename = "major methane geysers volcanism")]
    MajorMethaneGeyserVolcanism,
    #[serde(rename = "minor methane geysers volcanism")]
    MinorMethaneGeyserVolcanism,
    #[serde(rename = "nitrogen geysers volcanism")]
    NitrogenGeyserVolcanism,
    #[serde(rename = "major nitrogen geysers volcanism")]
    MajorNitrogenGeyserVolcanism,
    #[serde(rename = "minor nitrogen geysers volcanism")]
    MinorNitrogenGeyserVolcanism,
    #[serde(rename = "water geysers volcanism")]
    WaterGeyserVolcanism,
    #[serde(rename = "major water geysers volcanism")]
    MajorWaterGeyserVolcanism,
    #[serde(rename = "minor water geysers volcanism")]
    MinorWaterGeyserVolcanism,
    #[serde(rename = "silicate vapour geysers volcanism")]
    SilicateVapourGeyserVolcanism,
    #[serde(rename = "major silicate vapour geysers volcanism")]
    MajorSilicateVapourGeyserVolcanism,
    #[serde(rename = "minor silicate vapour geysers volcanism")]
    MinorSilicateVapourGeyserVolcanism,
    // not in manual, but found in logs
    #[serde(rename = "rocky magma volcanism")]
    RockyMagmaVolcanism,
    #[serde(rename = "major rocky magma volcanism")]
    MajorRockyMagmaVolcanism,
    #[serde(rename = "minor rocky magma volcanism")]
    MinorRockyMagmaVolcanism,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Trend {
    trend: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SystemFactionName {
    #[serde(deserialize_with = "string_or_struct")]
    pub system_faction: FactionName,
}

#[test]
fn test_faction() {
    let json = r#"{ "Name":"People's Madjandji Resistance", "FactionState":"None", "Government":"Democracy", "Influence":0.063555,
    "Allegiance":"Federation", "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", "MyReputation":0.000000 }"#;
    let line: Result<Faction, _> = serde_json::from_str(json);
    assert!(line.is_ok());

    let json = r#"{ "Name":"DaVinci Corp.", "FactionState":"Blight", "Government":"Corporate", "Influence":0.599801, "Allegiance":"Independent",
    "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", "MyReputation":61.321701, 
    "ActiveStates":[ { "State":"Blight" } ] }"#;
    let line: Result<Faction, _> = serde_json::from_str(json);
    assert!(line.is_ok());

    let json = r#"{ "Name":"Phoenix Flight Explorers Commune", "FactionState":"None", "Government":"Cooperative",
    "Influence":0.266137, "Allegiance":"Independent", "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", 
    "MyReputation":86.373100 } 
    "#;
    let line: Result<Faction, _> = serde_json::from_str(json);
    assert!(line.is_ok());
}

#[test]
fn test_power() {
    let json = r#"{ 
        "Powers":[ "Pranav Antal", "Jerome Archer" ], 
        "PowerplayState":"Unoccupied", 
        "PowerplayConflictProgress":[ 
            { "Power":"Pranav Antal", "ConflictProgress":0.005875 }, 
            { "Power":"Jerome Archer", "ConflictProgress":0.478375 } 
        ] 
    }"#;
    let _line: Powers = serde_json::from_str(json).expect("should parse");

    let json = r#"{ 
        "ControllingPower": "Jerome Archer",
        "Powers": ["Jerome Archer"],
        "PowerplayState": "Exploited"
    }"#;
    let _line: Powers = serde_json::from_str(json).expect("should parse");

    let json = r#"{ 
        "ControllingPower": "Jerome Archer",
        "Powers": ["Pranav Antal","Jerome Archer"],
        "PowerplayState": "Fortified",
        "PowerplayStateControlProgress": 0.337526,
        "PowerplayStateReinforcement": 792,
        "PowerplayStateUndermining": 0
    }"#;
    let _line: Powers = serde_json::from_str(json).expect("should parse");
}

#[test]
fn test_station_info() {
    let json = r#"{
    "StationName": "Otiman Dock",
    "StationType": "Bernal",
    "MarketID": 3227675648,
    "StationFaction": {
        "Name": "Nagii Union",
        "FactionState": "Expansion"
    },
    "StationGovernment": "$government_Communism;",
    "StationGovernment_Localised": "Communist",
    "StationServices": [
        "dock",
        "autodock",
        "blackmarket",
        "commodities",
        "contacts",
        "exploration",
        "missions",
        "outfitting",
        "crewlounge",
        "rearm",
        "refuel",
        "repair",
        "shipyard",
        "tuning",
        "engineer",
        "missionsgenerated",
        "facilitator",
        "flightcontroller",
        "stationoperations",
        "powerplay",
        "searchrescue",
        "stationMenu",
        "shop",
        "livery",
        "socialspace",
        "bartender",
        "vistagenomics",
        "pioneersupplies",
        "apexinterstellar",
        "frontlinesolutions"
    ],
    "StationEconomy": "$economy_Industrial;",
    "StationEconomy_Localised": "Industrial",
    "StationEconomies": [
        {
            "Name": "$economy_Industrial;",
            "Name_Localised": "Industrial",
            "Proportion": 1.000000
        }
    ]}"#;
    let _line: StationInformation = serde_json::from_str(json).expect("should parse");
}
