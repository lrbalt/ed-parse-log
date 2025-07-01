use crate::common_types::{Credits, CrimeType, Merits, StationType};
use crate::market::MicroResource;
use crate::utils::duration_as_secs;
use chrono::Duration;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EDLogCommander {
    #[serde(rename = "FID")]
    fid: String,
    #[serde(rename = "Name")]
    name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EDLogNewCommander {
    #[serde(rename = "FID")]
    fid: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Package")]
    package: String,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogRank {
    pub combat: u64,
    pub trade: u64,
    pub explore: u64,
    pub soldier: Option<u64>,
    pub exobiologist: Option<u64>,
    pub empire: u64,
    pub federation: u64,
    #[serde(rename = "CQC")]
    pub cqc: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPromotion {
    combat: Option<u8>,
    trade: Option<u8>,
    explore: Option<u8>,
    soldier: Option<u8>,
    exobiologist: Option<u8>,
    empire: Option<u8>,
    federation: Option<u8>,
    cqc: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogReputation {
    pub federation: Option<f64>,
    pub empire: Option<f64>,
    pub independent: Option<f64>,
    pub alliance: Option<f64>,
}

pub fn power_play_rank_range(rank: u64) -> (u64, u64) {
    match rank {
        0 => (0, 0),
        1 => (0, 2000),
        2 => (2000, 5000),
        3 => (5000, 9000),
        4 => (9000, 15000),
        _ => (15000 + (rank - 5) * 8000, 23000 + (rank - 5) * 8000),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplay {
    pub power: String,
    pub rank: u64,
    pub merits: Merits,
    pub votes: Option<u64>,
    #[serde(with = "duration_as_secs")]
    pub time_pledged: Duration,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplayJoin {
    power: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplayCollect {
    power: String,
    #[serde(rename = "Type")]
    power_type: String,
    #[serde(rename = "Type_Localised")]
    power_type_localised: String,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplayDeliver {
    power: String,
    #[serde(rename = "Type")]
    power_type: String,
    #[serde(rename = "Type_Localised")]
    power_type_localised: String,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplayMerits {
    power: String,
    merits_gained: Merits,
    total_merits: Merits,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplayFastTrack {
    power: String,
    cost: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Killer {
    name: String,
    ship: String,
    rank: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDied {
    killers: Option<Vec<Killer>>,
    killer_name: Option<String>,
    #[serde(rename = "KillerName_Localised")]
    killer_name_localised: Option<String>,
    killer_ship: Option<String>,
    killer_rank: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogResurrect {
    option: String,
    cost: u64,
    bankrupt: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogRequestPowerMicroResources {
    total_count: u64,
    micro_resources: Vec<MicroResource>,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplayDefect {
    from_power: String,
    to_power: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplayLeave {
    power: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplayRank {
    power: String,
    rank: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplaySalary {
    power: String,
    amount: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCommitCrime {
    crime_type: CrimeType,
    faction: String,
    victim: Option<String>,
    #[serde(rename = "Victim_Localised")]
    victim_localised: Option<String>,
    bounty: Option<Credits>,
    fine: Option<Credits>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCrimeVictim {
    offender: String,
    crime_type: CrimeType,
    bounty: Option<u64>,
    fine: Option<u64>,
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
    name: String,
    role: CrewMemberRole,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct StationEmbarkOrDisembark {
    station_name: String,
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
    star_system: String,
    system_address: u64,
    body: String,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogFriends {
    status: FriendStatus,
    name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogHoloscreenHacked {
    power_before: Option<String>,
    power_after: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogAppliedToSquadron {
    squadron_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogInvitedToSquadron {
    squadron_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSharedBookmarkToSquadron {
    squadron_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum VehicleType {
    Fighter,
    Mothership,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogVehicleSwitch {
    to: VehicleType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCrewMemberQuits {
    crew: String,
    telepresence: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogQuitACrew {
    captain: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogJoinACrew {
    captain: String,
    telepresence: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCrewFire {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "CrewID")]
    crew_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCrewAssign {
    name: String,
    #[serde(rename = "CrewID")]
    crew_id: u64,
    role: CrewMemberRole,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCrewHire {
    name: String,
    #[serde(rename = "CrewID")]
    crew_id: u64,
    faction: String,
    cost: Credits,
    combat_rank: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogChangeCrewRole {
    role: CrewMemberRole,
    telepresence: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCrewMemberRoleChange {
    crew: String,
    role: CrewMemberRole,
    telepresence: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCrewMemberJoins {
    crew: String,
    role: Option<CrewMemberRole>,
    telepresence: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogEndCrewSession {
    on_crime: bool,
    telepresence: Option<bool>,
}

#[test]
fn test_power_plat_rank_range() {
    assert_eq!((15000, 23000), power_play_rank_range(5));
    assert_eq!((55000, 63000), power_play_rank_range(10));
    assert_eq!((375000, 383000), power_play_rank_range(50));
    assert_eq!((775000, 783000), power_play_rank_range(100));
}

#[test]
fn test_powerplay() {
    use crate::log_line::{EDLogEvent, EDLogLine};

    let json10 = r#"{ "timestamp":"2024-09-16T14:34:52Z", "event":"Powerplay", "Power":"Aisling Duval", "Rank":10, "Merits":0, "Votes":0, "TimePledged":21406314 }"#;
    let json20 = r#"{ "timestamp":"2025-03-10T18:21:04Z", "event":"Powerplay", "Power":"Jerome Archer", "Rank":89, "Merits":687268, "TimePledged":4578634 }"#;
    let line10: EDLogLine = serde_json::from_str(json10).expect("Should parse");
    let line20: EDLogLine = serde_json::from_str(json20).expect("Should parse");

    assert!(matches!(line10.event(), EDLogEvent::Powerplay(_)));
    assert!(matches!(line20.event(), EDLogEvent::Powerplay(_)));
    if let EDLogEvent::Powerplay(pp) = line10.event() {
        assert_eq!(10, pp.rank);
        assert_eq!(Some(0), pp.votes);
    }
    if let EDLogEvent::Powerplay(pp) = line20.event() {
        assert_eq!(89, pp.rank);
        assert_eq!(None, pp.votes, "PP2.0 dropped votes");
    }
}
