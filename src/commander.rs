use crate::{
    EDString,
    common_types::{Credits, CrimeType, StationType},
    market::MicroResource,
};
use ed_parse_log_files_macros::Extractable;
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

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
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

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
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
    name: EDString,
    ship: EDString,
    rank: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDied {
    killers: Option<Vec<Killer>>,
    killer_name: Option<EDString>,
    #[serde(rename = "KillerName_Localised")]
    killer_name_localised: Option<EDString>,
    killer_ship: Option<EDString>,
    killer_rank: Option<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogResurrect {
    option: EDString,
    cost: u64,
    bankrupt: bool,
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
pub struct EDLogCrimeVictim {
    offender: EDString,
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
    combat_rank: u8,
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
