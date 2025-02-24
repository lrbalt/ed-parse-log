use serde::{Deserialize, Serialize};

use crate::ed_log::common_types::{CrimeType, StationType};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EDLogCommander {
    #[serde(rename = "FID")]
    fid: String,
    #[serde(rename = "Name")]
    name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EDLogNewCommander {
    #[serde(rename = "FID")]
    fid: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Package")]
    package: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogRank {
    combat: u64,
    trade: u64,
    explore: u64,
    soldier: Option<u64>,
    exobiologist: Option<u64>,
    empire: u64,
    federation: u64,
    #[serde(rename = "CQC")]
    cqc: u64,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogReputation {
    federation: Option<f64>,
    empire: Option<f64>,
    independent: Option<f64>,
    alliance: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplay {
    power: String,
    rank: u64,
    merits: u64,
    votes: Option<u64>,
    time_pledged: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplayJoin {
    power: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplayCollect {
    power: String,
    #[serde(rename = "Type")]
    power_type: String,
    #[serde(rename = "Type_Localised")]
    power_type_localised: String,
    count: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplayDeliver {
    power: String,
    #[serde(rename = "Type")]
    power_type: String,
    #[serde(rename = "Type_Localised")]
    power_type_localised: String,
    count: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplayFastTrack {
    power: String,
    cost: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Killer {
    name: String,
    ship: String,
    rank: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDied {
    killers: Option<Vec<Killer>>,
    killer_name: Option<String>,
    #[serde(rename = "KillerName_Localised")]
    killer_name_localised: Option<String>,
    killer_ship: Option<String>,
    killer_rank: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogResurrect {
    option: String,
    cost: u64,
    bankrupt: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplaySalary {
    power: String,
    amount: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCommitCrime {
    crime_type: CrimeType,
    faction: String,
    victim: Option<String>,
    #[serde(rename = "Victim_Localised")]
    victim_localised: Option<String>,
    bounty: Option<u64>,
    fine: Option<u64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCrimeVictim {
    offender: String,
    crime_type: CrimeType,
    bounty: Option<u64>,
    fine: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub enum CrewMemberRole {
    Helm,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CrewMember {
    name: String,
    role: CrewMemberRole,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct StationEmbarkOrDisembark {
    station_name: String,
    station_type: StationType,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub enum FriendStatus {
    Online,
    Offline,
    Requested,
    Added,
    Lost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogFriends {
    status: FriendStatus,
    name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogHoloscreenHacked {
    power_before: Option<String>,
    power_after: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCommunityGoalReward {
    #[serde(rename = "CGID")]
    cgid: u64,
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    system: String,
    reward: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCommunityGoalJoin {
    #[serde(rename = "CGID")]
    cgid: u64,
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    system: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogInvitedToSquadron {
    squadron_name: String,
}

#[derive(Serialize, Deserialize)]
pub enum VehicleType {
    Fighter,
    Mothership,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogVehicleSwitch {
    to: VehicleType,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogEndCrewSession {
    on_crime: bool,
}