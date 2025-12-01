use crate::{EDString, common_types::Credits};
use ed_parse_log_files_macros::{Extractable, testcase};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Mission {
    #[serde(rename = "MissionID")]
    mission_id: u64,
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    passenger_mission: bool,
    expires: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMissions {
    active: Vec<Mission>,
    failed: Vec<Mission>,
    complete: Vec<Mission>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct MissionTarget {
    target_type: EDString,
    #[serde(rename = "TargetType_Localised")]
    target_type_localised: EDString,
    target_faction: EDString,
    target: EDString,
    #[serde(rename = "Target_Localised")]
    target_localised: EDString,
    kill_count: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct MissionDestination {
    destination_system: EDString,
    new_destination_system: Option<EDString>,
    destination_station: Option<EDString>,
    new_destination_station: Option<EDString>,
    destination_settlement: Option<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CommodityMission {
    commodity: EDString,
    #[serde(rename = "Commodity_Localised")]
    commodify_localised: Option<EDString>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PassengerType {
    Refugee,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct PassengerMissionInformation {
    passenger_count: u64,
    #[serde(rename = "PassengerVIPs")]
    passenger_vips: bool,
    passenger_wanted: bool,
    passenger_type: PassengerType,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-10-17T01:47:41Z", "event":"MissionAccepted", 
    "Faction":"Independent Murung Values Party", "Name":"Mission_Delivery_Boom", 
    "LocalisedName":"Boom time delivery of 6 units of Biowaste", "Commodity":"$Biowaste_Name;", 
    "Commodity_Localised":"Biowaste", "Count":6, "TargetFaction":"Belu Silver Federal Industry", 
    "DestinationSystem":"57 Zeta Serpentis", "DestinationStation":"Musabayev Dock", 
    "Expiry":"2017-10-18T01:45:04Z", "Influence":"Low", "Reputation":"Low", 
    "Reward":157815, "MissionID":228681523 })]
pub struct EDLogMissionAccepted {
    faction: EDString,
    name: EDString,
    localised_name: EDString,
    #[serde(flatten)]
    mission_target: Option<MissionTarget>,
    #[serde(flatten)]
    mission_destination: Option<MissionDestination>,
    #[serde(flatten)]
    commodity: Option<CommodityMission>,
    donation: Option<EDString>,
    expiry: Option<EDString>,
    wing: Option<bool>,
    influence: EDString,
    reputation: EDString,
    reward: Option<u64>,
    #[serde(flatten)]
    passenger_mission_info: Option<PassengerMissionInformation>,
    #[serde(rename = "MissionID")]
    mission_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CommodityReward {
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct MaterialReward {
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    category: EDString,
    #[serde(rename = "Category_Localised")]
    category_localised: EDString,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Effect {
    effect: EDString,
    #[serde(rename = "Effect_Localised")]
    effect_localised: EDString,
    trend: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Influence {
    system_address: u64,
    trend: EDString,
    influence: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct FactionEffect {
    faction: EDString,
    effects: Vec<Effect>,
    influence: Vec<Influence>,
    reputation_trend: EDString,
    reputation: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMissionCompleted {
    faction: EDString,
    name: EDString,
    localised_name: Option<EDString>,
    #[serde(rename = "MissionID")]
    mission_id: u64,
    donation: Option<EDString>,
    donated: Option<u64>,
    #[serde(flatten)]
    mission_target: Option<MissionTarget>,
    #[serde(flatten)]
    mission_destination: Option<MissionDestination>,
    commodity: Option<EDString>,
    #[serde(rename = "Commodity_Localised")]
    commodity_localised: Option<EDString>,
    count: Option<u64>,
    reward: Option<u64>,
    commodity_reward: Option<Vec<CommodityReward>>,
    materials_reward: Option<Vec<MaterialReward>>,
    faction_effects: Option<Vec<FactionEffect>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMissionFailed {
    name: EDString,
    localised_name: Option<EDString>,
    #[serde(rename = "MissionID")]
    mission_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMissionAbandoned {
    name: EDString,
    localised_name: Option<EDString>,
    #[serde(rename = "MissionID")]
    mission_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMissionRedirected {
    #[serde(rename = "MissionID")]
    mission_id: u64,
    name: EDString,
    localised_name: Option<EDString>,
    #[serde(rename = "LocalisedName_Localised")]
    localised_name_localised: Option<EDString>,
    new_destination_station: EDString,
    new_destination_system: EDString,
    old_destination_station: EDString,
    old_destination_system: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSearchAndRescue {
    #[serde(rename = "MarketID")]
    market_id: u64,
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: EDString,
    count: u64,
    reward: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct BountyReward {
    pub faction: EDString,
    pub reward: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct BountyPilot {
    pilot_name: EDString,
    #[serde(rename = "PilotName_Localised")]
    pilot_name_localised: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogBounty {
    pub rewards: Option<Vec<BountyReward>>,
    pub reward: Option<f64>,
    #[serde(flatten)]
    pub pilot: Option<BountyPilot>,
    pub target: EDString,
    #[serde(rename = "Target_Localised")]
    pub target_localised: Option<EDString>,
    pub total_reward: Option<Credits>,
    pub victim_faction: EDString,
    #[serde(rename = "VictimFaction_Localised")]
    pub victim_faction_localised: Option<EDString>,
    pub shared_with_others: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VoucherType {
    #[serde(rename = "CombatBond")]
    CombatBond,
    Bounty,
    Settlement,
    Codex,
    Scannable,
    Trade,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct VoucherFaction {
    faction: EDString,
    amount: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-06-28T21:45:34Z", "event":"RedeemVoucher", "Type":"trade", "Amount":14229 })]
#[testcase({ "timestamp":"2025-11-09T20:07:12Z", "event":"RedeemVoucher", "Type":"bounty", "Amount":1500, "Factions":[ { "Faction":"", "Amount":1500 } ], "BrokerPercentage":25.000000 })]
pub struct EDLogRedeemVoucher {
    #[serde(rename = "Type")]
    pub voucher_type: VoucherType,
    pub amount: Credits,
    pub faction: Option<EDString>,
    pub factions: Option<Vec<VoucherFaction>>,
    pub broker_percentage: Option<f32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDatalinkVoucher {
    pub reward: Credits,
    pub victim_faction: EDString,
    pub payee_faction: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCapitalShipBond {
    reward: Credits,
    awarding_faction: EDString,
    victim_faction: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Passenger {
    #[serde(rename = "MissionID")]
    mission_id: u64,
    #[serde(rename = "Type")]
    mission_type: EDString, // TODO: use enum
    wanted: bool,
    #[serde(rename = "VIP")]
    vip: bool,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPassengers {
    manifest: Vec<Passenger>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPVPKill {
    victim: EDString,
    combat_rank: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogFactionKillBond {
    reward: u64,
    awarding_faction: EDString,
    #[serde(rename = "AwardingFaction_Localised")]
    awarding_faction_localised: Option<EDString>,
    victim_faction: EDString,
    #[serde(rename = "VictimFaction_Localised")]
    victim_faction_localised: Option<EDString>,
}
