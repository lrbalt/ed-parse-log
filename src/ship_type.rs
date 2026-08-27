use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Serialize, Deserialize, Clone, Debug, Display, Copy, PartialEq)]
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
    #[serde(alias = "lander01", alias = "Lander01")]
    Nomad,
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
    #[serde(alias = "Skimmer")]
    Skimmer,
    #[serde(alias = "skimmerdrone")]
    #[strum(to_string = "Sentry Skimmer")]
    SkimmerDrone,
    #[serde(alias = "bombskimmerdrone")]
    #[strum(to_string = "Stinger")]
    BombSkimmerDrone,
    #[serde(alias = "missileskimmer")]
    #[strum(to_string = "Warden")]
    MissileSkimmer,
    #[strum(to_string = "Kestrel Mk II")]
    #[serde(alias = "smallcombat01_nx", alias = "SmallCombat01_NX")]
    SmallCombat01NX,
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
    #[serde(alias = "combat_multicrew_srv_01")]
    SRVScorpion,
    #[strum(to_string = "SRV Scarab")]
    #[serde(rename = "TestBuggy")]
    #[serde(alias = "testbuggy")]
    SRVScarab,

    #[serde(alias = "unknownsaucer")]
    UnknownSaucer,
    #[serde(alias = "unknownsaucer_e")]
    UnknownSaucerE,
    #[serde(alias = "unknownsaucer_f")]
    UnknownSaucerF,
    #[serde(alias = "unknownsaucer_h")]
    UnknownSaucerH,
    #[serde(alias = "scout_nq")]
    ThargoidScoutNQ,
    #[serde(alias = "scout_hq")]
    ThargoidScoutHQ,
    #[serde(alias = "scout_q")]
    ThargoidScoutQ,
    #[serde(alias = "scout")]
    ThargoidScout,
    #[serde(alias = "thargonswarm")]
    ThargonSwarm,
    #[serde(alias = "glaive")]
    Glaive,

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

    #[serde(rename = "assaultsuitai_class1")]
    #[strum(to_string = "Commando")]
    AIAssaultSuitClass1,
    #[serde(rename = "assaultsuitai_class2")]
    #[strum(to_string = "Commando")]
    AIAssaultSuitClass2,
    #[serde(rename = "assaultsuitai_class3")]
    #[strum(to_string = "Commando")]
    AIAssaultSuitClass3,
    #[serde(rename = "assaultsuitai_class4")]
    #[strum(to_string = "Commando")]
    AIAssaultSuitClass4,
    #[serde(rename = "assaultsuitai_class5")]
    #[strum(to_string = "Commando")]
    AIAssaultSuitClass5,
    #[serde(rename = "rangedsuitai_class1")]
    #[strum(to_string = "Sharpshooter")]
    AIRangedSuitClass1,
    #[serde(rename = "rangedsuitai_class2")]
    #[strum(to_string = "Sharpshooter")]
    AIRangedSuitClass2,
    #[serde(rename = "rangedsuitai_class3")]
    #[strum(to_string = "Sharpshooter")]
    AIRangedSuitClass3,
    #[serde(rename = "rangedsuitai_class4")]
    #[strum(to_string = "Sharpshooter")]
    AIRangedSuitClass4,
    #[serde(rename = "rangedsuitai_class5")]
    #[strum(to_string = "Sharpshooter")]
    AIRangedSuitClass5,
    #[serde(rename = "lightassaultsuitai_class1")]
    #[strum(to_string = "Scout")]
    AILightAssaultSuitClass1,
    #[serde(rename = "lightassaultsuitai_class2")]
    #[strum(to_string = "Scout")]
    AILightAssaultSuitClass2,
    #[serde(rename = "lightassaultsuitai_class3")]
    #[strum(to_string = "Scout")]
    AILightAssaultSuitClass3,
    #[serde(rename = "lightassaultsuitai_class4")]
    #[strum(to_string = "Scout")]
    AILightAssaultSuitClass4,
    #[serde(rename = "lightassaultsuitai_class5")]
    #[strum(to_string = "Scout")]
    AILightAssaultSuitClass5,
    #[serde(rename = "closesuitai_class1")]
    #[strum(to_string = "Striker")]
    AICloseSuitClass1,
    #[serde(rename = "closesuitai_class2")]
    #[strum(to_string = "Striker")]
    AICloseSuitClass2,
    #[serde(rename = "closesuitai_class3")]
    #[strum(to_string = "Striker")]
    AICloseSuitClass3,
    #[serde(rename = "closesuitai_class4")]
    #[strum(to_string = "Striker")]
    AICloseSuitClass4,
    #[serde(rename = "closesuitai_class5")]
    #[strum(to_string = "Striker")]
    AICloseSuitClass5,
    #[serde(rename = "heavysuitai_class1")]
    #[strum(to_string = "Enforcer")]
    AIHeavySuitClass1,
    #[serde(rename = "heavysuitai_class2")]
    #[strum(to_string = "Enforcer")]
    AIHeavySuitClass2,
    #[serde(rename = "heavysuitai_class3")]
    #[strum(to_string = "Enforcer")]
    AIHeavySuitClass3,
    #[serde(rename = "heavysuitai_class4")]
    #[strum(to_string = "Enforcer")]
    AIHeavySuitClass4,
    #[serde(rename = "heavysuitai_class5")]
    #[strum(to_string = "Enforcer")]
    AIHeavySuitClass5,
    #[serde(rename = "citizensuitai_admin")]
    #[strum(to_string = "Administrator")]
    AICitizenSuitAdmin,
    #[serde(rename = "citizensuitai_scientific")]
    #[strum(to_string = "Researcher")]
    AICitizenSuitScientific,
    #[serde(rename = "citizensuitai_industrial")]
    #[strum(to_string = "Technician")]
    AICitizenSuitIndustrial,
}
