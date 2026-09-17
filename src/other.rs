use crate::{
    EDString,
    common_types::{
        CombatRank, Credits, CrewMemberRole, CrimeType, DroneType, GameMode, ShipScanType,
        StationInformation,
    },
    market_item::MarketItemType,
    material::AllMaterialNames,
    ship_module::{ShipModule, ShipModuleSlot},
    ship_type::ShipType,
};
use ed_parse_log_files_macros::{Extractable, testcase};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-01-23T15:42:09Z", "event":"AfmuRepairs", "Module":"$explorer_nx_cockpit_name;", 
    "FullyRepaired":true, "Health":1.000000 })]
#[testcase({ "timestamp":"2026-03-27T18:42:08Z", "event":"AfmuRepairs", "Module":"$int_dockingcomputer_advanced_name;", "Module_Localised":"Docking Computer", "FullyRepaired":true, "Health":1.000000 })]
pub struct EDLogAfmuRepairs {
    pub module: ShipModule,
    #[serde(rename = "Module_Localised")]
    pub module_localised: Option<EDString>,
    // If the AFMU runs out of ammo, the module may not be fully repaired.
    pub fully_repaired: bool,
    // 0.0 .. 1.0
    pub health: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
// todo: refactor with BodyInformation in common types
pub struct BodyInformationOfSettlement {
    pub system_address: u64,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
    pub body_name: EDString,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[testcase({ "timestamp":"2023-03-01T15:10:23Z", "event":"ApproachSettlement", "Name":"Nahavandi Penal colony", 
            "MarketID":3790770944, "SystemAddress":7266413782417, "BodyID":9, "BodyName":"Luggerates A 3", 
            "Latitude":62.048309, "Longitude":80.228821 })]
#[testcase({ "timestamp":"2024-05-30T16:20:52Z", "event":"ApproachSettlement", 
        "Name":"$Ancient_Small_002:#index=1;", "Name_Localised":"Guardian Structure", "SystemAddress":2833906537146, 
        "BodyID":7, "BodyName":"Synuefe EU-Q c21-10 A 3", "Latitude":19.823612, "Longitude":-82.460922 })]
#[testcase({ "timestamp":"2017-10-17T01:41:51Z", "event":"ApproachSettlement", "Name":"Verrazzano's Inheritance" })]
#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogApproachSettlement {
    pub name: EDString,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<EDString>,
    #[serde(flatten)]
    pub station_information: Option<StationInformation>,
    #[serde(flatten)]
    pub body_information: Option<BodyInformationOfSettlement>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2023-03-29T16:41:02Z", "event":"ChangeCrewRole", "Role":"Helm", "Telepresence":true })]
pub struct EDLogChangeCrewRole {
    pub role: CrewMemberRole,
    pub telepresence: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-05-03T14:02:48Z", "event":"CommitCrime", "CrimeType":"recklessWeaponsDischarge", "Faction":"Interplanetary Explorations", "Fine":100 })]
pub struct EDLogCommitCrime {
    pub crime_type: CrimeType,
    pub faction: EDString,
    pub victim: Option<EDString>,
    #[serde(rename = "Victim_Localised")]
    pub victim_localised: Option<EDString>,
    pub fine: Option<Credits>,
    pub bounty: Option<Credits>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogContinued {
    pub part: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCrewLaunchFighter {
    pub crew: EDString,
    #[serde(rename = "ID")]
    pub id: u64,
    pub telepresence: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2023-03-29T16:35:12Z", "event":"CrewMemberJoins", "Crew":"A Player", "Telepresence":true })]
pub struct EDLogCrewMemberJoins {
    pub crew: EDString,
    pub role: Option<CrewMemberRole>,
    pub telepresence: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2023-03-20T15:08:34Z", "event":"CrewMemberQuits", "Crew":"A Player", "Telepresence":true })]
pub struct EDLogCrewMemberQuits {
    pub crew: EDString,
    pub telepresence: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2023-03-20T14:57:48Z", "event":"CrewMemberRoleChange", "Crew":"A Player", "Role":"Idle", "Telepresence":true })]
pub struct EDLogCrewMemberRoleChange {
    pub crew: EDString,
    pub role: CrewMemberRole,
    pub telepresence: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-03-17T13:44:57Z", "event":"CrimeVictim", "Offender":"MiniMe", "CrimeType":"assault", "Bounty":200 })]
pub struct EDLogCrimeVictim {
    pub offender: EDString,
    pub crime_type: CrimeType,
    pub bounty: Option<Credits>,
    pub fine: Option<Credits>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-09T13:50:55Z", "event":"DatalinkScan", "Message":"$DATAPOINT_GAMEPLAY_complete;", 
    "Message_Localised":"Alert: All Data Point telemetry links established, Intel package created." })]
pub struct EDLogDatalinkScan {
    pub message: EDString,
    #[serde(rename = "Message_Localised")]
    pub message_localised: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-11-10T14:26:50Z", "event":"DatalinkVoucher", "Reward":9975, "VictimFaction":"", "PayeeFaction":"Empire" })]
pub struct EDLogDatalinkVoucher {
    pub reward: Credits,
    pub victim_faction: EDString,
    pub payee_faction: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
pub enum ScannedDataType {
    #[serde(rename = "$Datascan_AbandonedDataLog;")]
    #[strum(to_string = "Abandoned Data Log")]
    AbandonedDataLog,
    #[serde(rename = "$Datascan_ANCIENTCODEX;")]
    #[strum(to_string = "Ancient Codex")]
    AncientCodex,
    #[serde(rename = "$Datascan_AncientPylon;")]
    #[strum(to_string = "Ancient Pylon")]
    AncientPylon,
    #[serde(rename = "$Datascan_Settlement_Unknown;")]
    #[strum(to_string = "Barnacle Site")]
    BarnacleSite,
    #[serde(rename = "$Datascan_DataPoint;")]
    DataPoint,
    #[serde(rename = "$Datascan_ListeningPost;")]
    #[strum(to_string = "Listening Post")]
    ListeningPost,
    #[serde(rename = "$Datascan_ShipUplink;")]
    #[strum(to_string = "Ship Uplink")]
    ShipUplink,
    #[serde(rename = "$Datascan_Unknown_Uplink;")]
    #[strum(to_string = "Thargoid Uplink")]
    ThargoidUplink,
    #[serde(rename = "$Datascan_TouristBeacon;")]
    #[strum(to_string = "Tourist Beacon")]
    TouristBeacon,
    #[serde(rename = "$Datascan_WreckedShip;")]
    #[strum(to_string = "Wrecked Ship")]
    WreckedShip,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-08T10:17:46Z", "event":"DataScanned", "Type":"$Datascan_ListeningPost;", "Type_Localised":"Listening Post" })]
pub struct EDLogDataScanned {
    #[serde(rename = "Type")]
    pub data_type: ScannedDataType,
    #[serde(rename = "Type_Localised")]
    pub data_type_localised: Option<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(deny_unknown_fields)]
#[testcase({ "timestamp":"2025-06-20T20:47:12Z", "event":"DockFighter", "ID":5 })]
pub struct EDLogDockFighter {
    #[serde(rename = "ID")]
    pub id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({"timestamp":"2026-07-01T18:47:22Z","event":"DockSRV","SRVType":"lander01","SRVType_Localised":"Nomad","ID":31})]
#[testcase({"timestamp": "2026-08-27T20:58:53Z","event": "DockSRV","SRVType": "mev_rhino","SRVType_Localised": "SRV Rhino","ID": 86})]
pub struct EDLogDockSRV {
    #[serde(rename = "SRVType")]
    pub srv_type: Option<ShipType>,
    #[serde(rename = "SRVType_Localised")]
    pub srv_type_localised: Option<EDString>,
    #[serde(rename = "ID")]
    pub id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2022-08-07T18:10:11Z", "event":"EndCrewSession", "OnCrime":false })]
#[testcase({ "timestamp":"2023-03-29T16:44:05Z", "event":"EndCrewSession", "OnCrime":false, "Telepresence":true })]
pub struct EDLogEndCrewSession {
    pub on_crime: bool,
    pub telepresence: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(deny_unknown_fields)]
#[testcase({"timestamp":"2024-03-10T04:46:46Z","event":"FighterRebuilt","Loadout":"four","ID":30})]
pub struct EDLogFighterRebuilt {
    #[serde(rename = "Loadout")]
    pub loadout: EDString,
    #[serde(rename = "ID")]
    pub id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-11-13T16:19:40Z", "event":"FuelScoop", "Scooped":0.146537, "Total":32.000000 })]
pub struct EDLogFuelScoop {
    pub scooped: f64,
    pub total: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FriendStatus {
    Online,
    Offline,
    Requested,
    Added,
    Lost,
    Declined,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({"timestamp":"2025-04-19T00:22:58Z","event":"Friends","Status":"Online","Name":"MyBestFriend"})]
pub struct EDLogFriends {
    pub status: FriendStatus,
    pub name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({"timestamp":"2026-06-28T11:23:01Z","event":"JetConeBoost","BoostValue":4})]
pub struct EDLogJetConeBoost {
    pub boost_value: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-04-07T17:14:53Z", "event":"JetConeDamage", "Module":"$modularcargobaydoor_name;", "Module_Localised":"Cargo Hatch" })]
pub struct EDLogJetConeDamage {
    pub module: ShipModule,
    #[serde(rename = "Module_Localised")]
    pub module_localised: Option<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2023-03-29T16:35:07Z", "event":"JoinACrew", "Captain":"MyCaptain", "Telepresence":true })]
pub struct EDLogJoinACrew {
    pub captain: EDString,
    pub telepresence: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogKickCrewMember {
    pub crew: EDString,
    pub on_crime: bool,
    pub telepresence: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(deny_unknown_fields)]
#[testcase({ "timestamp":"2025-07-04T21:28:42Z", "event":"LaunchDrone", "Type":"Collection" })]
pub struct EDLogLaunchDrone {
    #[serde(rename = "Type")]
    pub drone_type: DroneType,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({"timestamp":"2026-07-01T18:39:09Z","event":"LaunchFighter","Loadout":"starter","ID":31,"PlayerControlled":true})]
pub struct EDLogLaunchFighter {
    pub loadout: EDString,
    #[serde(rename = "ID")]
    pub id: u64,
    pub player_controlled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-11-13T16:30:47Z", "event":"LaunchSRV", "SRVType":"testbuggy", "SRVType_Localised":"SRV Scarab", 
    "Loadout":"starter", "ID":25, "PlayerControlled":true })]
#[testcase({"timestamp": "2026-08-27T20:13:14Z","event": "LaunchSRV","SRVType": "mev_rhino","SRVType_Localised": "SRV Rhino",
    "Loadout": "advanced","ID": 86,"PlayerControlled": true
})]
pub struct EDLogLaunchSRV {
    #[serde(rename = "SRVType")]
    pub srv_type: Option<ShipType>,
    #[serde(rename = "SRVType_Localised")]
    pub srv_type_localised: Option<EDString>,
    pub loadout: EDString,
    #[serde(rename = "ID")]
    pub id: u64,
    pub player_controlled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-09-02T19:26:27Z", "event":"LaunchVessel", "VesselType":"lander01", 
    "VesselType_Localised":"Nomad", "Loadout":"base", "ID":55, "PlayerControlled":true })]
pub struct EDLogLaunchVessel {
    vessel_type: ShipType,
    #[serde(rename = "VesselType_Localised")]
    srvtype_localised: Option<EDString>,
    loadout: EDString,
    #[serde(rename = "ID")]
    id: u64,
    player_controlled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct InstalledModule {
    pub slot: ShipModuleSlot,
    pub item: ShipModule,
    pub power: Option<f64>,
    pub priority: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-09-07T16:48:50Z", "event":"ModuleInfo", "Modules":[ 
    { "Slot":"MainEngines", "Item":"int_engine_size7_class5", "Power":10.214400, "Priority":0 }, 
    { "Slot":"Radar", "Item":"int_sensors_size8_class5", "Power":2.070000, "Priority":0 }
] })]
#[testcase({ "timestamp":"2026-09-07T16:48:50Z", "event":"ModuleInfo" })]
pub struct EDLogModuleInfo {
    pub modules: Option<Vec<InstalledModule>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MusicTrack {
    Codex,
    #[serde(rename = "Combat_CapitalShip")]
    CombatCapitalShip,
    #[serde(rename = "Combat_Dogfight")]
    CombatDogfight,
    #[serde(rename = "Combat_Hunters")]
    CombatHunters,
    #[serde(rename = "Combat_LargeDogFight")]
    CombatLargeDogFight,
    #[serde(rename = "Combat_SRV")]
    CombatSRV,
    #[serde(rename = "Combat_Unknown")]
    CombatUnknown,
    CQCMenu,
    #[serde(rename = "Damaged_Starport")]
    DamagedStarport,
    DestinationFromHyperspace,
    DestinationFromSupercruise,
    DockingComputer,
    Exploration,
    #[serde(rename = "FleetCarrier_Managment")]
    FleetCarrierManagment,
    GalacticPowers,
    GalaxyMap,
    GuardianSites,
    Interdiction,
    #[serde(rename = "Lifeform_FogCloud")]
    LifeformFogCloud,
    MainMenu,
    NoInGameMusic,
    NoTrack,
    OnFoot,
    Squadrons,
    Starport,
    Supercruise,
    SystemMap,
    SystemAndSurfaceScanner,
    TGFactory,
    #[serde(rename = "Thargoid_GroundCombat")]
    ThargoidGroundCombat,
    #[serde(rename = "Titan_Encounter")]
    TitanEncounter,
    #[serde(rename = "Titan_Graveyard")]
    TitanGraveyard,
    #[serde(rename = "Unknown_Encounter")]
    UnknownEncounter,
    #[serde(rename = "Unknown_Exploration")]
    UnknownExploration,
    #[serde(rename = "Unknown_Settlement")]
    UnknownSettlement,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-09-09T18:45:14Z", "event":"Music", "MusicTrack":"Starport" })]
pub struct EDLogMusic {
    music_track: MusicTrack,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({"timestamp":"2025-04-13T22:27:00Z","event":"NpcCrewRank","NpcCrewName":"Emmeline Donaldson","NpcCrewId":80085792,"RankCombat":2})]
pub struct EDLogNpcCrewRank {
    npc_crew_name: EDString,
    npc_crew_id: u64,
    rank_combat: CombatRank,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({"timestamp":"2026-07-01T18:38:01Z","event":"NpcCrewPaidWage","NpcCrewName":"Otha Waller","NpcCrewId":15481250,"Amount":123})]
pub struct EDLogNpcCrewPaidWage {
    npc_crew_name: EDString,
    npc_crew_id: u64,
    amount: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-09-06T17:42:07Z", "event":"Promotion", "Explore":12 })]
// TODO: use enum instead of u8
pub struct EDLogPromotion {
    pub combat: Option<u8>,
    pub trade: Option<u8>,
    pub explore: Option<u8>,
    pub soldier: Option<u8>,
    pub exobiologist: Option<u8>,
    pub empire: Option<u8>,
    pub federation: Option<u8>,
    pub cqc: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ProspectedMaterial {
    // TODO: split raw materials from MarketItemType,
    pub name: MarketItemType,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<EDString>,
    pub proportion: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
pub enum AstroidContent {
    #[serde(rename = "$AsteroidMaterialContent_Low;")]
    #[strum(to_string = "Material Content: Low")]
    Low,
    #[serde(rename = "$AsteroidMaterialContent_Medium;")]
    #[strum(to_string = "Material Content: Medium")]
    Medium,
    #[serde(rename = "$AsteroidMaterialContent_High;")]
    #[strum(to_string = "Material Content: High")]
    High,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-09-07T12:34:05Z", "event":"ProspectedAsteroid", "Materials":[ 
    { "Name":"lepidolite", "Proportion":15.779820 }, { "Name":"indite", "Proportion":16.678970 }, 
    { "Name":"silver", "Proportion":10.041161 } ], 
    "Content":"$AsteroidMaterialContent_Medium;", "Content_Localised":"Material Content: Medium", "Remaining":100.000000 })]
pub struct EDLogProspectedAsteroid {
    pub materials: Vec<ProspectedMaterial>,
    pub motherlode_material: Option<MarketItemType>,
    #[serde(rename = "MotherlodeMaterial_Localised")]
    pub motherlode_material_localised: Option<EDString>,
    pub content: AstroidContent,
    #[serde(rename = "Content_Localised")]
    pub content_localised: EDString,
    pub remaining: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-09T17:22:29Z", "event":"QuitACrew", "Captain":"" })]
pub struct EDLogQuitACrew {
    pub captain: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-07-04T14:07:13Z", "event":"RebootRepair", "Modules":[ "Radar", "Military01", "TinyHardpoint3", "TinyHardpoint2", "TinyHardpoint1", "SmallHardpoint2", "SmallHardpoint1", "LargeHardpoint3", "LargeHardpoint2", "LargeHardpoint1" ] })]
pub struct EDLogRebootRepair {
    pub modules: Vec<ShipModuleSlot>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TextChannel {
    Npc,
    Wing,
    StarSystem,
    Local,
    Player,
    VoiceChat,
    Friend,
    Squadron,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2023-07-30T14:40:36Z", "event":"ReceiveText", "From":"$ShipName_PassengerLiner_Cruise;", 
             "From_Localised":"Cruise Ship", "Message":"$CruiseLiner_SCPatrol05;", 
             "Message_Localised":"This is your captain. Due to some unforeseen delays, we will be arriving at our next destination later than scheduled.", 
             "Channel":"npc" })]
pub struct EDLogReceiveText {
    pub from: EDString,
    #[serde(rename = "From_Localised")]
    pub from_localised: Option<EDString>,
    pub message: EDString,
    #[serde(rename = "Message_Localised")]
    pub message_localised: Option<EDString>,
    pub channel: TextChannel,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-04-20T19:18:43Z", "event":"RepairDrone", "HullRepaired":300.000977 })]
pub struct EDLogRepairDrone {
    hull_repaired: Option<f64>,
    corrosion_repaired: Option<f64>,
    cockpit_repaired: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-09-09T18:33:17Z", "event":"ReservoirReplenished", "FuelMain":28.610003, "FuelReservoir":1.130000 })]
pub struct EDLogReservoirReplenished {
    fuel_main: f64,
    fuel_reservoir: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2024-12-13T18:27:19Z", "event":"Resurrect", "Option":"rebuy", "Cost":4326918, "Bankrupt":false })]
pub struct EDLogResurrect {
    pub option: EDString,
    pub cost: Credits,
    pub bankrupt: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-20T18:21:51Z", "event":"Scanned", "ScanType":"Cargo" })]
pub struct EDLogScanned {
    pub scan_type: ShipScanType,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-09-08T18:10:22Z", "event":"SendText", "To":"wing", "Message":"í'm stuck in menu", "Sent":true })]
pub struct EDLogSendText {
    pub to: EDString,
    pub message: EDString,
    pub sent: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SynthesisMaterial {
    pub name: AllMaterialNames,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<EDString>,
    pub count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-31T17:36:32Z", "event":"Synthesis", "Name":"Heat Sink Basic", "Materials":[ 
{ "Name":"basicconductors", "Name_Localised":"Basic Conductors", "Count":2 }, 
{ "Name":"heatconductionwiring", "Name_Localised":"Heat Conduction Wiring", "Count":2 } ] })]
pub struct EDLogSynthesis {
    pub name: EDString,
    pub materials: Vec<SynthesisMaterial>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display, PartialEq)]
pub enum USSType {
    #[serde(rename = "$USS_Type_VeryValuableSalvage;")]
    #[strum(to_string = "High grade emissions")]
    VeryValueableSalvage,
    #[serde(rename = "$USS_Type_ValuableSalvage;")]
    #[strum(to_string = "Encoded emissions")]
    ValueableSalvage,
    #[serde(rename = "$USS_Type_Salvage;")]
    #[strum(to_string = "Degraded emissions")]
    Salvage,
    #[serde(rename = "$USS_Type_MissionTarget;")]
    #[strum(to_string = "Mission target")]
    MissionTarget,
    #[serde(rename = "$USS_Type_TradingBeacon;")]
    #[strum(to_string = "Trading beacon")]
    TradingBeacon,
    #[serde(rename = "$USS_Type_Ceremonial;")]
    #[strum(to_string = "Ceremonial comms")]
    Ceremonial,
    #[serde(rename = "$USS_Type_WeaponsFire;")]
    #[strum(to_string = "Weapons fire")]
    WeaponsFire,
    #[serde(rename = "$USS_Type_Aftermath;")]
    #[strum(to_string = "Combat aftermath")]
    Aftermath,
    #[serde(rename = "$USS_Type_Refugee;")]
    #[strum(to_string = "Refugee convoy pattern")]
    Refugee,
    #[serde(rename = "$USS_Type_DistressSignal;")]
    #[strum(to_string = "Distress call")]
    DistressSignal,
    #[serde(rename = "$USS_Type_Convoy;")]
    #[strum(to_string = "Convoy dispersal pattern")]
    Convoy,
    #[serde(rename = "$USS_Type_PowerConvoy;")]
    #[strum(to_string = "Power Convoy")]
    PowerplayConvoy,
    #[serde(rename = "$USS_Type_PowerplayConvoyDistressSignal;")]
    #[strum(to_string = "Power Convoy Distress Signal")]
    PowerplayConvoyDistressSignal,
    #[serde(rename = "$USS_Type_PowerEmissions;")]
    #[strum(to_string = "Power Wreckage Signature")]
    PowerEmissions,
    #[serde(rename = "$USS_Type_PowerWeaponsFire;")]
    #[strum(to_string = "Power Weapons Fire Signature")]
    PowerWeaponsFire,
    #[serde(rename = "$USS_Type_NonHuman;")]
    #[strum(to_string = "Nonhuman signal source")]
    NonHuman,
    #[serde(rename = "$USS_Type_AXShips;")]
    #[strum(to_string = "AX ship signatures")]
    AXShips,
    #[serde(rename = "$USS_Type_AXWeaponsFire;")]
    #[strum(to_string = "AX weapons fire")]
    AXWaeponsFire,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(deny_unknown_fields)]
#[testcase({ "timestamp":"2025-06-28T21:18:42Z", "event":"USSDrop", "USSType":"$USS_Type_PowerEmissions;", "USSType_Localised":"Power Wreckage Signature", "USSThreat":1 })]
pub struct EDLogUSSDrop {
    #[serde(rename = "USSType")]
    pub uss_type: USSType,
    #[serde(rename = "USSType_Localised")]
    pub uss_type_localised: EDString,
    #[serde(rename = "USSThreat")]
    pub uss_threat: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum VehicleType {
    Fighter,
    Mothership,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({"timestamp":"2024-03-20T00:04:00Z","event":"VehicleSwitch","To":"Fighter"})]
pub struct EDLogVehicleSwitch {
    pub to: VehicleType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({"timestamp":"2025-04-19T00:28:26Z","event":"WingJoin","Others":["AnOtherPerson"]})]
pub struct EDLogWingJoin {
    pub others: Vec<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CargoTransfer {
    #[serde(rename = "Type")]
    pub cargo_type: MarketItemType,
    #[serde(rename = "Type_Localised")]
    pub cargo_type_localised: Option<EDString>,
    pub count: u32,
    pub direction: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-04-24T18:05:02Z", "event":"CargoTransfer", "Transfers":[ 
    { "Type":"militaryintelligence", "Type_Localised":"Military Intelligence", "Count":1, "Direction":"tocarrier" } ] })]
pub struct EDLogCargoTransfer {
    pub transfers: Vec<CargoTransfer>,
}

#[testcase({ "timestamp":"2023-07-30T20:54:01Z", "event":"SupercruiseDestinationDrop", "Type":"Wrangell Terminal", "Threat":0, "MarketID":3228997120 })]
#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSupercruiseDestinationDrop {
    #[serde(rename = "Type")]
    pub dest_type: EDString,
    #[serde(rename = "Type_Localised")]
    pub dest_type_localised: Option<EDString>,
    pub threat: u64,
    #[serde(rename = "MarketID")]
    pub market_id: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-09-02T18:51:13Z", "event":"GameModeChange", "GameMode":"MainGame" })]
pub struct EDLogGameModeChange {
    game_mode: GameMode,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMarketID {}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct RequiredResource {
    pub name: EDString,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<EDString>,
    pub required_amount: u64,
    pub provided_amount: u64,
    pub payment: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ContributedResource {
    name: MarketItemType,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    amount: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-04-07T17:15:29Z", "event":"ColonisationConstructionDepot", "MarketID":3964275458, "ConstructionProgress":0.000000, 
    "ConstructionComplete":false, "ConstructionFailed":false, "ResourcesRequired":[ 
        { "Name":"$aluminium_name;", "Name_Localised":"Aluminium", "RequiredAmount":1351, "ProvidedAmount":0, "Payment":3239 }
    ] })]
pub struct EDLogColonisationConstructionDepot {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub construction_progress: f64,
    pub construction_complete: bool,
    pub construction_failed: bool,
    pub resources_required: Vec<RequiredResource>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogColonisationContribution {
    #[serde(rename = "MarketID")]
    market_id: u64,
    contributions: Vec<ContributedResource>,
}
