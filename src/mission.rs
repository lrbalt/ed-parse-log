use crate::common_types::Credits;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Mission {
    #[serde(rename = "MissionID")]
    mission_id: u64,
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    passenger_mission: bool,
    expires: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMissions {
    active: Vec<Mission>,
    failed: Vec<Mission>,
    complete: Vec<Mission>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct MissionTarget {
    target_type: String,
    #[serde(rename = "TargetType_Localised")]
    target_type_localised: String,
    target_faction: String,
    target: String,
    #[serde(rename = "Target_Localised")]
    target_localised: String,
    kill_count: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct MissionDestination {
    destination_system: String,
    new_destination_system: Option<String>,
    destination_station: Option<String>,
    new_destination_station: Option<String>,
    destination_settlement: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CommodityMission {
    commodity: String,
    #[serde(rename = "Commodity_Localised")]
    commodify_localised: Option<String>,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMissionAccepted {
    faction: String,
    name: String,
    localised_name: String,
    #[serde(flatten)]
    mission_target: Option<MissionTarget>,
    #[serde(flatten)]
    mission_destination: Option<MissionDestination>,
    #[serde(flatten)]
    commodity: Option<CommodityMission>,
    donation: Option<String>,
    expiry: Option<String>,
    wing: bool,
    influence: String,
    reputation: String,
    reward: Option<u64>,
    #[serde(flatten)]
    passenger_mission_info: Option<PassengerMissionInformation>,
    #[serde(rename = "MissionID")]
    mission_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CommodityReward {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct MaterialReward {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    category: String,
    #[serde(rename = "Category_Localised")]
    category_localised: String,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Effect {
    effect: String,
    #[serde(rename = "Effect_Localised")]
    effect_localised: String,
    trend: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Influence {
    system_address: u64,
    trend: String,
    influence: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct FactionEffect {
    faction: String,
    effects: Vec<Effect>,
    influence: Vec<Influence>,
    reputation_trend: String,
    reputation: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMissionCompleted {
    faction: String,
    name: String,
    localised_name: Option<String>,
    #[serde(rename = "MissionID")]
    mission_id: u64,
    donation: Option<String>,
    donated: Option<u64>,
    #[serde(flatten)]
    mission_target: Option<MissionTarget>,
    #[serde(flatten)]
    mission_destination: Option<MissionDestination>,
    commodity: Option<String>,
    #[serde(rename = "Commodity_Localised")]
    commodity_localised: Option<String>,
    count: Option<u64>,
    reward: Option<u64>,
    commodity_reward: Option<Vec<CommodityReward>>,
    materials_reward: Option<Vec<MaterialReward>>,
    faction_effects: Vec<FactionEffect>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMissionFailed {
    name: String,
    localised_name: Option<String>,
    #[serde(rename = "MissionID")]
    mission_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMissionAbandoned {
    name: String,
    localised_name: Option<String>,
    #[serde(rename = "MissionID")]
    mission_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMissionRedirected {
    #[serde(rename = "MissionID")]
    mission_id: u64,
    name: String,
    localised_name: Option<String>,
    #[serde(rename = "LocalisedName_Localised")]
    localised_name_localised: Option<String>,
    new_destination_station: String,
    new_destination_system: String,
    old_destination_station: String,
    old_destination_system: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSearchAndRescue {
    #[serde(rename = "MarketID")]
    market_id: u64,
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: String,
    count: u64,
    reward: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct BountyReward {
    faction: String,
    reward: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct BountyPilot {
    pilot_name: String,
    #[serde(rename = "PilotName_Localised")]
    pilot_name_localised: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogBounty {
    rewards: Option<Vec<BountyReward>>,
    reward: Option<f64>,
    #[serde(flatten)]
    pilot: Option<BountyPilot>,
    target: String,
    #[serde(rename = "Target_Localised")]
    target_localised: Option<String>,
    total_reward: Option<u64>,
    victim_faction: String,
    #[serde(rename = "VictimFaction_Localised")]
    victim_faction_localised: Option<String>,
    shared_with_others: Option<u64>,
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
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct VoucherFaction {
    faction: String,
    amount: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogRedeemVoucher {
    #[serde(rename = "Type")]
    voucher_type: VoucherType,
    amount: u64,
    faction: Option<String>,
    factions: Option<Vec<VoucherFaction>>,
    broker_percentage: Option<f32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDatalinkVoucher {
    reward: u64,
    victim_faction: String,
    payee_faction: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCapitalShipBond {
    reward: Credits,
    awarding_faction: String,
    victim_faction: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogFactionKillBond {
    reward: u64,
    awarding_faction: String,
    #[serde(rename = "AwardingFaction_Localised")]
    awarding_faction_localised: Option<String>,
    victim_faction: String,
    #[serde(rename = "VictimFaction_Localised")]
    victim_faction_localised: Option<String>,
}
