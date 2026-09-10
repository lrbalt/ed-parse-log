use crate::{
    EDString,
    common_types::{Credits, CrewMemberRole, CrimeType},
    fleet_carrier::CarrierType,
    odyssey::MicroResource,
};
use ed_parse_log_files_macros::{Extractable, testcase};
use serde::{Deserialize, Serialize};

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
#[testcase({ "timestamp":"2025-08-21T15:02:57Z", "event":"CarrierLocation", "CarrierType":"FleetCarrier", 
    "CarrierID":123456789, "StarSystem":"BD-11 192", "SystemAddress":908486218450, "BodyID":3 })]
pub struct EDLogCarrierLocation {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub carrier_type: Option<CarrierType>,
    pub star_system: EDString,
    pub system_address: u64,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
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
