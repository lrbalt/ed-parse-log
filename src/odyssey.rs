use crate::{
    EDString,
    codex::{CodexGenus, CodexNames, CodexSpecies},
    common_types::{
        Credits, CrewMemberRole, OnFootItem, ScanType, StationType, SuitMod, WeaponMod,
    },
    log_line::{EDLogEvent, Extractable},
    ship_type::ShipType,
};
use ed_parse_log_files_macros::{Extractable, testcase};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BackpackItemType {
    Component,
    Data,
    Item,
    Consumable,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct BackpackItem {
    pub name: OnFootItem,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<EDString>,
    #[serde(rename = "OwnerID")]
    pub owner_id: u64,
    #[serde(rename = "MissionID")]
    pub mission_id: Option<u64>,
    pub count: u64,
    #[serde(rename = "Type")]
    pub item_type: Option<BackpackItemType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-09-06T17:45:48Z", "event":"Backpack", 
    "Items":[ { "Name":"largecapacitypowerregulator", "Name_Localised":"Power Regulator", "OwnerID":0, "MissionID":1147286740, "Count":1} ], 
    "Components":[ { "Name":"encryptedmemorychip", "Name_Localised":"Encrypted Memory Chip", "OwnerID":0, "Count":1} ], 
    "Consumables":[ { "Name":"healthpack", "Name_Localised":"Medkit", "OwnerID":0, "Count":1 } ], 
    "Data":[ { "Name":"operationscounterattackdata", "Name_Localised":"Installation Intelligence Report", "OwnerID":0, "Count":1 } ] })]
pub struct EDLogBackpack {
    pub items: Vec<BackpackItem>,
    pub components: Vec<BackpackItem>,
    pub consumables: Vec<BackpackItem>,
    pub data: Vec<BackpackItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2022-09-27T19:59:13Z", "event":"BackpackChange", "Added":[ { "Name":"circuitswitch", "Name_Localised":"Circuit Switch", "OwnerID":0, "Count":1, "Type":"Component" } ] })]
#[testcase({ "timestamp":"2026-08-26T18:36:23Z", "event":"BackpackChange", "Removed":[ { "Name":"operationscounterattackdata", "Name_Localised":"Installation Intelligence Report", "OwnerID":0, "Count":1, "Type":"Data" } ] })]
pub struct EDLogBackpackChange {
    pub added: Option<Vec<BackpackItem>>,
    pub removed: Option<Vec<BackpackItem>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-06-18T18:37:23Z", "event":"BookDropship", "Retreat":true, "Cost":0, "DestinationSystem":"Muncheim", "DestinationLocation":"Hubble Ring" })]
pub struct EDLogBookDropship {
    pub retreat: bool,
    pub cost: Credits,
    pub destination_system: EDString,
    pub destination_location: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2023-08-16T09:00:44Z", "event":"BookTaxi", "Cost":4532, "DestinationSystem":"Ratraii", 
"DestinationLocation":"Uniyal Extraction Prospect" })]
pub struct EDLogBookTaxi {
    pub retreat: Option<bool>,
    pub cost: Credits,
    pub destination_system: EDString,
    pub destination_location: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MicroResourceType {
    Encoded,
    Raw,
    Manufactured,
    Item,
    Component,
    Data,
    Consumable,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct MicroResource {
    // TODO: parse to enum using category
    pub name: OnFootItem,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<EDString>,
    pub category: MicroResourceType,
    pub count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2024-09-26T18:32:04Z", "event":"BuyMicroResources", "Name":"bypass", "Name_Localised":"E-Breach", 
    "Category":"Consumable", "Count":11, "Price":275000, "MarketID":3706278912 })]
pub struct EDLogBuyMicroResources {
    pub total_count: Option<u64>,
    pub price: Credits,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    #[serde(flatten)]
    pub micro_resource: Option<MicroResource>,
    pub micro_resources: Option<Vec<MicroResource>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2023-01-19T15:04:45Z", "event":"BuySuit", "Name":"ExplorationSuit_Class3", "Name_Localised":"$ExplorationSuit_Class1_Name;", "Price":11250000, "SuitID":1755463743129538, "SuitMods":[ "suit_backpackcapacity" ] })]
pub struct EDLogBuySuit {
    // TODO: make better enum for Suit, Vessel, Ship
    pub name: ShipType,
    #[serde(rename = "Name_Localised")]
    pub name_localised: EDString,
    pub price: Credits,
    #[serde(rename = "SuitID")]
    pub suit_id: u64,
    pub suit_mods: Vec<SuitMod>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
pub enum WeaponType {
    #[serde(
        alias = "Wpn_M_AssaultRifle_Kinetic_FAuto",
        alias = "wpn_m_assaultrifle_kinetic_fauto"
    )]
    #[strum(to_string = "Karma AR-50")]
    AssaultRifleKinetic,
    #[serde(
        alias = "Wpn_M_AssaultRifle_Laser_FAuto",
        alias = "wpn_m_assaultrifle_laser_fauto"
    )]
    #[strum(to_string = "TK Aphelion")]
    AssaultRifleLaser,
    #[serde(
        alias = "Wpn_M_AssaultRifle_Plasma_FAuto",
        alias = "wpn_m_assaultrifle_plasma_fauto"
    )]
    #[strum(to_string = "Manticore Oppressor")]
    AssaultRiflePlasma,
    #[serde(
        alias = "Wpn_M_Launcher_Rocket_SAuto",
        alias = "wpn_m_launcher_rocket_sauto"
    )]
    #[strum(to_string = "Karma L-6")]
    LauncherRocket,
    #[serde(
        alias = "Wpn_S_Pistol_Kinetic_SAuto",
        alias = "wpn_s_pistol_kinetic_sauto"
    )]
    #[strum(to_string = "Karma P-15")]
    PistolKinetic,
    #[serde(alias = "Wpn_S_Pistol_Laser_SAuto", alias = "wpn_s_pistol_laser_sauto")]
    #[strum(to_string = "TK Zenith")]
    PistolLaser,
    #[serde(
        alias = "Wpn_S_Pistol_Plasma_Charged",
        alias = "wpn_s_pistol_plasma_charged"
    )]
    #[strum(to_string = "Manticore Tormentor")]
    PistolPlasmaCharged,
    #[serde(
        alias = "Wpn_M_Shotgun_Plasma_DoubleBarrel",
        alias = "wpn_m_shotgun_plasma_doublebarrel"
    )]
    #[strum(to_string = "Manticore Intimidator")]
    ShotgunPlasma,
    #[serde(
        alias = "Wpn_M_Sniper_Plasma_Charged",
        alias = "wpn_m_sniper_plasma_charged"
    )]
    #[strum(to_string = "Manticore Executioner")]
    SniperPlasma,
    #[serde(
        alias = "Wpn_M_SubMachineGun_Kinetic_FAuto",
        alias = "wpn_m_submachinegun_kinetic_fauto"
    )]
    #[strum(to_string = "Karma C-44")]
    SubMachineGunKinetic,
    #[serde(
        alias = "Wpn_M_SubMachineGun_Laser_FAuto",
        alias = "wpn_m_submachinegun_laser_fauto"
    )]
    #[strum(to_string = "TK Eclipse")]
    SubMachineGunLaser,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2023-01-22T22:12:15Z", "event":"BuyWeapon", "Name":"Wpn_S_Pistol_Plasma_Charged", "Name_Localised":"Manticore Tormentor", "Class":2, "Price":750000, "SuitModuleID":1755762430656441, "WeaponMods":[ "weapon_suppression_pressurised" ] })]
pub struct EDLogBuyWeapon {
    pub name: WeaponType,
    #[serde(rename = "Name_Localised")]
    pub name_localised: EDString,
    pub class: u64,
    pub price: Credits,
    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,
    pub weapon_mods: Vec<WeaponMod>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2023-01-16T18:04:41Z", "event":"CancelDropship", "Refund":0 })]
pub struct EDLogCancelDropship {
    refund: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({"timestamp":"2024-05-07T21:37:17Z","event":"CancelTaxi","Refund":39028})]
pub struct EDLogCancelTaxi {
    pub refund: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-09-08T18:58:04Z", "event":"CollectItems", "Name":"energycell", "Name_Localised":"Energy Cell", "Type":"Consumable", "OwnerID":0, "Count":1, "Stolen":false })]
pub struct EDLogCollectItems {
    pub name: OnFootItem,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<EDString>,
    #[serde(rename = "Type")]
    pub item_type: BackpackItemType,
    #[serde(rename = "OwnerID")]
    pub owner_id: u64,
    pub count: u64,
    pub stolen: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SuitSlot {
    PrimaryWeapon1,
    PrimaryWeapon2,
    SecondaryWeapon,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SuitModule {
    pub slot_name: SuitSlot,
    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,
    pub module_name: WeaponType,
    #[serde(rename = "ModuleName_Localised")]
    pub module_name_localised: EDString,
    pub class: u64,
    pub weapon_mods: Vec<WeaponMod>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-04-11T15:01:48Z", "event":"CreateSuitLoadout", "SuitID":1862092190235536, 
    "SuitName":"utilitysuit_class5", "SuitName_Localised":"$UtilitySuit_Class1_Name;", 
    "SuitMods":[ "suit_increasedammoreserves", "suit_improvedarmourrating", "suit_increasedbatterycapacity", 
    "suit_backpackcapacity" ], 
    "LoadoutID":4293000009, "LoadoutName":"MySuitLoadout", "Modules":[ 
        { "SlotName":"PrimaryWeapon1", "SuitModuleID":1853593755304357, "ModuleName":"wpn_m_assaultrifle_laser_fauto", 
          "ModuleName_Localised":"TK Aphelion", "Class":5, "WeaponMods":[ 
            "weapon_clipsize", "weapon_backpackreloading", "weapon_range", "weapon_handling" ] }, 
        { "SlotName":"SecondaryWeapon", "SuitModuleID":1755762430656441, "ModuleName":"wpn_s_pistol_plasma_charged", 
          "ModuleName_Localised":"Manticore Tormentor", "Class":5, "WeaponMods":[ 
            "weapon_suppression_pressurised", "weapon_backpackreloading", "weapon_clipsize", "weapon_suppression_unpressurised" ] } ] })]
pub struct EDLogCreateSuitLoadout {
    #[serde(rename = "SuitID")]
    pub suit_id: u64,
    // TODO: split ShipType into SuitType and ShipType
    pub suit_name: ShipType,
    // See testcase, these localised strings cannot be used for Display trait
    #[serde(rename = "SuitName_Localised")]
    pub suit_name_localised: EDString,
    pub suit_mods: Vec<SuitMod>,
    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,
    pub loadout_name: EDString,
    pub modules: Vec<SuitModule>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CrewMember {
    name: EDString,
    role: CrewMemberRole,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct StationEmbarkOrDisembark {
    pub station_name: EDString,
    pub station_type: StationType,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
// testcase in seperate test in this file below: test_embark_or_disembark
pub struct EDLogEmbarkOrDisembark {
    // true if getting out of SRV, false if getting out of a ship
    #[serde(rename = "SRV")]
    pub srv: bool,
    // true when getting out of a taxi transposrt ship
    pub taxi: bool,
    // true when getting out of another player’s vessel
    pub multicrew: bool,
    pub crew: Option<Vec<CrewMember>>,
    #[serde(rename = "ID")]
    pub id: Option<u64>,
    pub star_system: EDString,
    pub system_address: u64,
    pub body: EDString,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
    pub on_station: bool,
    pub on_planet: bool,
    #[serde(flatten)]
    pub station: Option<StationEmbarkOrDisembark>,
}

impl Extractable for EDLogEmbarkOrDisembark {
    fn extract(event: &EDLogEvent) -> Option<&Self> {
        match event {
            EDLogEvent::Embark(info) => Some(info),
            EDLogEvent::Disembark(info) => Some(info),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2023-08-28T17:49:46Z", "event":"DropItems", "Name":"geneticsample", 
    "Name_Localised":"Biological Sample", "Type":"Item", "OwnerID":0, "MissionID":892206108, 
    "Count":1 })]
pub struct EDLogDropItems {
    pub name: OnFootItem,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<EDString>,
    #[serde(rename = "Type")]
    pub item_type: EDString,
    #[serde(rename = "OwnerID")]
    pub owner_id: u64,
    #[serde(rename = "MissionID")]
    pub mission_id: Option<u64>,
    pub count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-06-26T18:17:46Z", "event":"DropshipDeploy", "StarSystem":"Arietis Sector CC-J a10-4", "SystemAddress":75730473863928, "Body":"Arietis Sector CC-J a10-4 2", "BodyID":2, "OnStation":false, "OnPlanet":true })]
pub struct EDLogDropshipDeploy {
    star_system: EDString,
    system_address: u64,
    body: EDString,
    #[serde(rename = "BodyID")]
    body_id: u64,
    on_station: bool,
    on_planet: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct FCMaterials {
    #[serde(rename = "id")]
    pub id: u64,
    pub name: OnFootItem,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<EDString>,
    pub price: Credits,
    pub stock: u64,
    pub demand: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2022-03-24T11:37:28Z", "event":"FCMaterials", "MarketID":3700020480, "CarrierName":"ANiceCarrier",
    "CarrierID":"A1A-A1A", "Items":[
    { "id":128961556, "Name":"$californium_name;", "Name_Localised":"Californium", "Price":74000, "Stock":0, "Demand":1}
    ] })]
pub struct EDLogFCMaterials {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub carrier_name: EDString,
    #[serde(rename = "CarrierID")]
    pub carrier_id: EDString,
    pub items: Option<Vec<FCMaterials>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-07-07T18:14:21Z", "event":"LoadoutEquipModule", "LoadoutName":"MyLoadOut", 
    "SuitID":1758014072486724, "SuitName":"tacticalsuit_class5", "SuitName_Localised":"$TacticalSuit_Class1_Name;", 
    "LoadoutID":4293000008, "SlotName":"PrimaryWeapon2", "ModuleName":"wpn_m_assaultrifle_laser_fauto", 
    "ModuleName_Localised":"TK Aphelion", "Class":5, "WeaponMods":[ 
        "weapon_clipsize", "weapon_backpackreloading", "weapon_range", "weapon_handling" 
    ], "SuitModuleID":1853593755304357 })]
pub struct EDLogLoadoutEquipModule {
    #[serde(rename = "SuitID")]
    pub suit_id: u64,
    // TODO: split up shiptype for suits and ships
    pub suit_name: ShipType,
    #[serde(rename = "SuitName_Localised")]
    pub suit_name_localised: EDString,
    pub slot_name: EDString,
    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,
    pub loadout_name: EDString,
    pub module_name: WeaponType,
    #[serde(rename = "ModuleName_Localised")]
    pub module_name_localised: EDString,
    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,
    pub class: u64,
    pub weapon_mods: Vec<WeaponMod>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({"timestamp": "2026-07-07T18:14:15Z","event": "LoadoutRemoveModule",
"LoadoutName": "K en A - rifles","SuitID": 1758014072486724,"SuitName": "tacticalsuit_class5",
"SuitName_Localised": "$TacticalSuit_Class1_Name;","LoadoutID": 4293000008,
"SlotName": "PrimaryWeapon2","ModuleName": "wpn_m_assaultrifle_kinetic_fauto",
"ModuleName_Localised": "Karma AR-50","Class": 5,"SuitModuleID": 1757340617280755,
"WeaponMods": ["weapon_clipsize","weapon_backpackreloading","weapon_range","weapon_handling"]})]
pub struct EDLogLoadoutRemoveModule {
    #[serde(rename = "SuitID")]
    pub suit_id: u64,
    // TODO: split up shiptype for suits and ships
    pub suit_name: ShipType,
    #[serde(rename = "SuitName_Localised")]
    pub suit_name_localised: EDString,
    pub slot_name: EDString,
    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,
    pub loadout_name: EDString,
    pub module_name: WeaponType,
    #[serde(rename = "ModuleName_Localised")]
    pub module_name_localised: EDString,
    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,
    pub class: u64,
    pub weapon_mods: Vec<WeaponMod>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-04-07T16:50:26Z", "event":"RenameSuitLoadout", "SuitID":1757094195283325, 
    "SuitName":"utilitysuit_class3", "SuitName_Localised":"$UtilitySuit_Class1_Name;", "LoadoutID":4293000001, 
    "LoadoutName":"MyNewName" })]
pub struct EDLogRenameSuitLoadout {
    #[serde(rename = "SuitID")]
    pub suit_id: u64,
    // TODO: split up shiptype for suits and ships
    pub suit_name: ShipType,
    #[serde(rename = "SuitName_Localised")]
    pub suit_name_localised: EDString,
    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,
    pub loadout_name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-11-13T20:22:43Z", "event":"ScanOrganic", "ScanType":"Sample", 
    "Genus":"$Codex_Ent_Ingensradices_Genus_Name;", "Genus_Localised":"Radicoida", 
    "Species":"$Codex_Ent_Ingensradices_Unicus_Name;", "Species_Localised":"Radicoida Unica", 
    "Variant":"$Codex_Ent_Ingensradices_Unicus_Name;", "Variant_Localised":"Radicoida Unica", 
    "WasLogged":false, "SystemAddress":147882789259, "Body":3 })]
pub struct EDLogScanOrganic {
    pub scan_type: ScanType,
    pub genus: CodexGenus,
    #[serde(rename = "Genus_Localised")]
    pub genus_localised: EDString,
    pub species: CodexSpecies,
    #[serde(rename = "Species_Localised")]
    pub species_localised: EDString,
    // TODO: split variant from CodexNames
    pub variant: Option<CodexNames>,
    #[serde(rename = "Variant_Localised")]
    pub variant_localised: Option<EDString>,
    pub system_address: u64,
    pub body: u64,
    pub was_logged: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-01-21T20:24:20Z", "event":"SellMicroResources", "TotalCount":44, 
    "MicroResources":[ 
        { "Name":"compactlibrary", "Name_Localised":"Compact Library", "Category":"Item", "Count":1 }, 
        { "Name":"insight", "Category":"Item", "Count":1 } ], 
    "Price":479000, "MarketID":3228823296 })]
pub struct EDLogSellMicroResources {
    pub micro_resources: Vec<MicroResource>,
    pub total_count: u64,
    pub price: Credits,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
// TODO: merge with EDLogScanOrganicData
pub struct SoldBioData {
    pub genus: CodexGenus,
    #[serde(rename = "Genus_Localised")]
    pub genus_localised: EDString,
    pub species: CodexSpecies,
    #[serde(rename = "Species_Localised")]
    pub species_localised: EDString,
    pub variant: Option<CodexNames>,
    #[serde(rename = "Variant_Localised")]
    pub variant_localised: Option<EDString>,
    pub value: Credits,
    pub bonus: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-09-08T18:04:14Z", "event":"SellOrganicData", "MarketID":3706278912, 
    "BioData":[ { "Genus":"$Codex_Ent_Tussocks_Genus_Name;", "Genus_Localised":"Tussock", "Species":"$Codex_Ent_Tussocks_14_Name;", "Species_Localised":"Tussock Virgam", "Variant":"$Codex_Ent_Tussocks_14_G_Name;", "Variant_Localised":"Tussock Virgam - Lime", "Value":14313700, "Bonus":0 } ] })]
pub struct EDLogSellOrganicData {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub bio_data: Vec<SoldBioData>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2023-02-16T18:41:37Z", "event":"SellSuit", "SuitID":1756196919922232, "SuitMods":[  ], "Name":"tacticalsuit_class3", "Name_Localised":"$TacticalSuit_Class1_Name;", "Price":450000 })]
pub struct EDLogSellSuit {
    // TODO: move suits out of shiptype
    pub name: ShipType,
    #[serde(rename = "Name_Localised")]
    pub name_localised: EDString,
    pub price: Credits,
    #[serde(rename = "SuitID")]
    pub suit_id: u64,
    pub suit_mods: Vec<SuitMod>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2023-06-03T12:13:48Z", "event":"SellWeapon", "Name":"wpn_m_launcher_rocket_sauto", 
    "Name_Localised":"Karma L-6", "Class":1, "WeaponMods":[  ], "Price":105000, "SuitModuleID":1755467810887689 })]
pub struct EDLogSellWeapon {
    pub name: WeaponType,
    #[serde(rename = "Name_Localised")]
    pub name_localised: EDString,
    pub price: Credits,
    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,
    pub class: u64,
    pub weapon_mods: Vec<WeaponMod>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ShipLockerItem {
    name: OnFootItem,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    #[serde(rename = "OwnerID")]
    owner_id: u64,
    #[serde(rename = "MissionID")]
    mission_id: Option<u64>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LockerContent {
    items: Vec<ShipLockerItem>,
    components: Vec<ShipLockerItem>,
    consumables: Vec<ShipLockerItem>,
    data: Vec<ShipLockerItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-12-09T19:58:35Z", "event":"ShipLocker" })]
#[testcase({ "timestamp":"2025-12-09T20:19:43Z", "event":"ShipLocker", 
    "Items":[ { "Name":"geneticsample", "Name_Localised":"Biological Sample", "OwnerID":0, "Count":20 }, { "Name":"lazarus", "OwnerID":0, "Count":6 }], 
    "Components":[ { "Name":"graphene", "OwnerID":0, "Count":40 }, { "Name":"carbonfibreplating", "Name_Localised":"Carbon Fibre Plating", "OwnerID":0, "Count":10 }], 
    "Consumables":[ { "Name":"healthpack", "Name_Localised":"Medkit", "OwnerID":0, "Count":100 } ], 
    "Data":[ { "Name":"biometricdata", "Name_Localised":"Biometric Data", "OwnerID":0, "Count":8 }] })]
pub struct EDLogShipLocker {
    #[serde(flatten)]
    content: Option<LockerContent>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSuitLoadout {
    #[serde(rename = "SuitID")]
    pub suit_id: u64,
    // TODO: split suit out of shiptype
    pub suit_name: ShipType,
    #[serde(rename = "SuitName_Localised")]
    pub suit_name_localised: EDString,
    pub suit_mods: Vec<SuitMod>,
    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,
    pub loadout_name: EDString,
    pub modules: Vec<SuitModule>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDeleteSuitLoadout {
    #[serde(rename = "SuitID")]
    suit_id: u64,
    suit_name: EDString,
    #[serde(rename = "SuitName_Localised")]
    suit_name_localised: EDString,
    #[serde(rename = "LoadoutID")]
    loadout_id: u64,
    loadout_name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-01-20T14:06:36Z", "event":"TradeMicroResources", 
    "Offered":[ { "Name":"encryptedmemorychip", "Name_Localised":"Encrypted Memory Chip", "Category":"Component", "Count":18 }, 
                { "Name":"memorychip", "Name_Localised":"Memory Chip", "Category":"Component", "Count":18 } ], 
    "TotalCount":36, "Received":"weaponcomponent", "Received_Localised":"Weapon Component", "Count":4, "Category":"Component", "MarketID":3224235520 })]
pub struct EDLogTradeMicroResources {
    pub offered: Vec<MicroResource>,
    pub total_count: u64,
    pub received: OnFootItem,
    #[serde(rename = "Received_Localised")]
    pub received_localised: Option<EDString>,
    pub count: u64,
    pub category: MicroResourceType,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct UpgradeResource {
    name: OnFootItem,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2024-09-16T18:49:37Z", "event":"UpgradeSuit", "Name":"explorationsuit_class4", 
    "Name_Localised":"$ExplorationSuit_Class1_Name;", "SuitID":1755463743129538, "Class":5, "Cost":7500000, 
    "Resources":[ 
        { "Name":"suitschematic", "Name_Localised":"Suit Schematic", "Count":5 }, 
        { "Name":"healthmonitor", "Name_Localised":"Health Monitor", "Count":5 }
    ] })]
pub struct EDLogUpgradeSuit {
    #[serde(rename = "SuitID")]
    pub suit_id: u64,
    #[serde(rename = "Name")]
    // TODO: split suit from shiptype
    pub suit_name: ShipType,
    #[serde(rename = "Name_Localised")]
    pub suit_name_localised: EDString,
    pub class: u8,
    pub cost: Credits,
    pub resources: Vec<UpgradeResource>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-01-07T19:51:24Z", "event":"UpgradeWeapon", "Name":"wpn_m_assaultrifle_laser_fauto", 
    "Name_Localised":"TK Aphelion", "Class":5, "SuitModuleID":1853593755304357, "Cost":6250000, 
    "Resources":[ 
        { "Name":"weaponschematic", "Name_Localised":"Weapon Schematic", "Count":5 }, 
        { "Name":"ionisedgas", "Name_Localised":"Ionised Gas", "Count":5 }
    ] })]
pub struct EDLogUpgradeWeapon {
    pub name: WeaponType,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<EDString>,
    pub class: u64,
    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,
    pub cost: Credits,
    pub resources: Vec<UpgradeResource>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-09-08T18:55:11Z", "event":"UseConsumable", "Name":"energycell", "Name_Localised":"Energy Cell", "Type":"Consumable" })]
pub struct EDLogUseConsumable {
    pub name: OnFootItem,
    #[serde(rename = "Name_Localised")]
    pub name_localised: EDString,
    #[serde(rename = "Type")]
    pub item_type: BackpackItemType,
}

#[test]
fn test_embark_or_disembark() {
    use crate::log_line::{EDLogEvent, EDLogLine};

    let json = r#"{ "timestamp":"2025-09-18T19:05:29Z", "event":"Disembark", 
        "SRV":false, "Taxi":false, "Multicrew":false, 
        "ID":35, "StarSystem":"Hill Pa Hsi", "SystemAddress":9467315955121, 
        "Body":"Curie Gateway", "BodyID":37, 
        "OnStation":true, "OnPlanet":false, 
        "StationName":"Curie Gateway", "StationType":"Coriolis", "MarketID":3228628736 }"#;
    let line: EDLogLine = serde_json::from_str(json).expect("should parse");

    assert!(matches!(line.event(), EDLogEvent::Disembark(_)));

    if let EDLogEvent::Disembark(details) = line.event() {
        assert!(details.station.is_some());
        assert_eq!(
            "Curie Gateway",
            details.station.as_ref().unwrap().station_name.as_str()
        );
    }
}
