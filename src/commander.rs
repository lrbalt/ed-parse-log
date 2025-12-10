use crate::{
    EDString,
    common_types::{Credits, CrimeType, StationType},
    market::MicroResource,
};
use ed_parse_log_files_macros::{Extractable, testcase};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(deny_unknown_fields)]
pub struct EDLogCommander {
    #[serde(rename = "FID")]
    fid: EDString,
    #[serde(rename = "Name")]
    name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(deny_unknown_fields)]
pub struct EDLogNewCommander {
    #[serde(rename = "FID")]
    fid: EDString,
    #[serde(rename = "Name")]
    name: EDString,
    #[serde(rename = "Package")]
    package: EDString,
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

pub type CombatRank = u8;
pub type TradeRank = u8;
pub type ExploreRank = u8;
pub type SoldierRank = u8;
pub type ExobiologistRank = u8;
pub type EmpireRank = u8;
pub type FederationRank = u8;
pub type CQCRank = u8;

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-09-13T18:29:43Z", "event":"Rank", "Combat":9, "Trade":12, "Explore":9, "Soldier":8, "Exobiologist":8, "Empire":12, "Federation":12, "CQC":0 })]
pub struct EDLogRank {
    pub combat: CombatRank,
    pub trade: TradeRank,
    pub explore: ExploreRank,
    pub soldier: Option<SoldierRank>,
    pub exobiologist: Option<ExobiologistRank>,
    pub empire: EmpireRank,
    pub federation: FederationRank,
    #[serde(rename = "CQC")]
    pub cqc: CQCRank,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
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

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogReputation {
    pub federation: Option<f64>,
    pub empire: Option<f64>,
    pub independent: Option<f64>,
    pub alliance: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Killer {
    pub name: EDString,
    pub ship: EDString,
    pub rank: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2024-12-16T17:30:36Z", "event":"Died", "KillerName":"$UNKNOWN;", "KillerName_Localised":"Unknown", "KillerShip":"unknownsaucer", "KillerRank":"Elite" })]
#[testcase({ "timestamp":"2024-03-03T10:57:41Z", "event":"Died", "Killers":[ { "Name":"Cmdr ilovetogank", "Ship":"krait_mkii", "Rank":"Dangerous" }, { "Name":"Cmdr ganker2", "Ship":"cutter", "Rank":"Elite" } ] })]
pub struct EDLogDied {
    pub killers: Option<Vec<Killer>>,
    pub killer_name: Option<EDString>,
    #[serde(rename = "KillerName_Localised")]
    pub killer_name_localised: Option<EDString>,
    pub killer_ship: Option<EDString>,
    pub killer_rank: Option<EDString>,
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
pub struct EDLogRequestPowerMicroResources {
    total_count: u64,
    micro_resources: Vec<MicroResource>,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCommitCrime {
    crime_type: CrimeType,
    faction: EDString,
    victim: Option<EDString>,
    #[serde(rename = "Victim_Localised")]
    victim_localised: Option<EDString>,
    bounty: Option<Credits>,
    fine: Option<Credits>,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CrewMemberRole {
    Active,
    Helm,
    OnShoreLeave,
    OnFoot,
    Idle,
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
    station_name: EDString,
    station_type: StationType,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogEmbarkOrDisembark {
    #[serde(rename = "SRV")]
    srv: bool,
    taxi: bool,
    multicrew: bool,
    crew: Option<Vec<CrewMember>>,
    #[serde(rename = "ID")]
    id: Option<u64>,
    star_system: EDString,
    system_address: u64,
    body: EDString,
    #[serde(rename = "BodyID")]
    body_id: u64,
    on_station: bool,
    on_planet: bool,
    #[serde(flatten)]
    station: Option<StationEmbarkOrDisembark>,
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
pub struct EDLogFriends {
    status: FriendStatus,
    name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogAppliedToSquadron {
    squadron_name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogInvitedToSquadron {
    squadron_name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSharedBookmarkToSquadron {
    squadron_name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum VehicleType {
    Fighter,
    Mothership,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogVehicleSwitch {
    to: VehicleType,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCrewMemberQuits {
    crew: EDString,
    telepresence: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogQuitACrew {
    captain: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogJoinACrew {
    captain: EDString,
    telepresence: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCrewFire {
    #[serde(rename = "Name")]
    name: EDString,
    #[serde(rename = "CrewID")]
    crew_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCrewAssign {
    name: EDString,
    #[serde(rename = "CrewID")]
    crew_id: u64,
    role: CrewMemberRole,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCrewHire {
    name: EDString,
    #[serde(rename = "CrewID")]
    crew_id: u64,
    faction: EDString,
    cost: Credits,
    combat_rank: CombatRank,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogChangeCrewRole {
    role: CrewMemberRole,
    telepresence: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCrewMemberRoleChange {
    crew: EDString,
    role: CrewMemberRole,
    telepresence: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCrewMemberJoins {
    crew: EDString,
    role: Option<CrewMemberRole>,
    telepresence: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogEndCrewSession {
    on_crime: bool,
    telepresence: Option<bool>,
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
