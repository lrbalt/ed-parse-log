use crate::{
    EDString,
    common_types::{Credits, PilotRank, Power},
    mission::{BountyPilot, BountyReward},
    ship::LegalStatus,
    ship_module::{ShipModule, serde_ship_module},
    ship_type::ShipType,
    startup::CombatRank,
};
use ed_parse_log_files_macros::{Extractable, testcase, testcase_struct};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-26T18:51:13Z", "event":"Bounty", "Rewards":[ 
    { "Faction":"Imperial Grey Wolves", "Reward":50160 }, 
    { "Faction":"LTT 8584 Crimson General Org", "Reward":166584 } ], 
    "PilotName":"$npc_name_decorate:#name=Florian Poprat;", "PilotName_Localised":"Florian Poprat", 
    "Target":"cobramkiii", "Target_Localised":"Cobra Mk III", "TotalReward":216744, 
    "VictimFaction":"Pirates of LTT 8584", "SharedWithOthers":1 })]
pub struct EDLogBounty {
    pub rewards: Option<Vec<BountyReward>>,
    pub reward: Option<f64>,
    #[serde(flatten)]
    pub pilot: Option<BountyPilot>,
    pub target: ShipType,
    #[serde(rename = "Target_Localised")]
    pub target_localised: Option<EDString>,
    pub total_reward: Option<Credits>,
    pub victim_faction: EDString,
    #[serde(rename = "VictimFaction_Localised")]
    pub victim_faction_localised: Option<EDString>,
    pub shared_with_others: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2020-10-06T18:50:04Z", "event":"CapShipBond", "Reward":500000, 
    "AwardingFaction":"Eurybia Blue Mafia", "VictimFaction":"Keltim Empire League" })]
pub struct EDLogCapitalShipBond {
    pub reward: Credits,
    pub awarding_faction: EDString,
    pub victim_faction: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Name":"Cmdr ilovetogank", "Ship":"krait_mkii", "Rank":"Dangerous" })]
pub struct Killer {
    pub name: EDString,
    pub ship: ShipType,
    pub rank: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "KillerName":"$UNKNOWN;", "KillerName_Localised":"Unknown", "KillerShip":"unknownsaucer", "KillerRank":"Elite" })]
pub struct SingleKiller {
    pub killer_name: EDString,
    #[serde(rename = "KillerName_Localised")]
    pub killer_name_localised: EDString,
    pub killer_ship: ShipType,
    pub killer_rank: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2024-12-16T17:30:36Z", "event":"Died", "KillerName":"$UNKNOWN;", "KillerName_Localised":"Unknown", "KillerShip":"unknownsaucer", "KillerRank":"Elite" })]
#[testcase({ "timestamp":"2024-03-03T10:57:41Z", "event":"Died", "Killers":[ { "Name":"Cmdr ilovetogank", "Ship":"krait_mkii", "Rank":"Dangerous" }, { "Name":"Cmdr ganker2", "Ship":"cutter", "Rank":"Elite" } ] })]
pub struct EDLogDied {
    pub killers: Option<Vec<Killer>>,
    #[serde(flatten)]
    pub killer: Option<SingleKiller>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-05-16T12:02:49Z", "event":"EscapeInterdiction", "Interdictor":"Alfred", "IsPlayer":false })]
pub struct EDLogEscapeInterdiction {
    pub interdictor: EDString,
    #[serde(rename = "Interdictor_Localised")]
    pub interdictor_localised: Option<EDString>,
    pub is_player: bool,
    pub is_thargoid: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-01-05T20:04:31Z", "event":"FactionKillBond", "Reward":52500, 
    "AwardingFaction":"Sirius Inc", "VictimFaction":"Race Marshalls" })]
pub struct EDLogFactionKillBond {
    pub reward: Credits,
    pub awarding_faction: EDString,
    #[serde(rename = "AwardingFaction_Localised")]
    pub awarding_faction_localised: Option<EDString>,
    pub victim_faction: EDString,
    #[serde(rename = "VictimFaction_Localised")]
    pub victim_faction_localised: Option<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(deny_unknown_fields)]
#[testcase({ "timestamp":"2023-12-31T13:56:52Z", "event":"FighterDestroyed", "ID":21 })]
pub struct EDLogFighterDestroyed {
    #[serde(rename = "ID")]
    pub id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-09T19:32:09Z", "event":"HullDamage", "Health":0.398718, "PlayerPilot":true, "Fighter":false })]
pub struct EDLogHullDamage {
    pub health: f64,
    pub player_pilot: bool,
    pub fighter: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-02-13T18:56:58Z", "event":"Interdiction", "Success":true, "IsPlayer":false, "Faction":"Li Yong-Rui", "Power":"Independent" })]
pub struct EDLogInterdiction {
    pub success: bool,
    pub is_player: bool,
    pub faction: EDString,
    pub power: Option<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-04-14T18:06:06Z", "event":"Interdicted", "Submitted":true, "Interdictor":"Joey Challis", "IsPlayer":false, "Faction":"Dragons of Darahk" })]
pub struct EDLogInterdicted {
    pub submitted: bool,
    pub interdictor: Option<EDString>,
    #[serde(rename = "Interdictor_Localised")]
    pub interdictor_localised: Option<EDString>,
    pub is_player: bool,
    pub combat_rank: Option<u8>,
    pub faction: Option<EDString>,
    pub is_thargoid: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({"timestamp":"2025-04-12T21:29:13Z","event":"PVPKill","Victim":"NakedRaider","CombatRank":8})]
pub struct EDLogPVPKill {
    pub victim: EDString,
    pub combat_rank: CombatRank,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-26T19:09:12Z", "event":"ShieldState", "ShieldsUp":true })]
pub struct EDLogShieldState {
    pub shields_up: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Subsystem":"$int_powerdistributor_size5_class3_name;", 
    "Subsystem_Localised":"Power Distributor", "SubsystemHealth":98.958328 })]
pub struct TargetedSubsystem {
    #[serde(with = "serde_ship_module")]
    pub subsystem: ShipModule,
    #[serde(rename = "Subsystem_Localised")]
    pub subsystem_localised: EDString,
    pub subsystem_health: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({"Ship":"vulture", "ScanStage":3})]
pub struct TargetedShip {
    pub ship: ShipType,
    #[serde(rename = "Ship_Localised")]
    pub ship_localised: Option<EDString>,
    pub scan_stage: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({"PilotName":"$ShipName_Military_Independent;", "PilotName_Localised":"System Defence Force", "PilotRank":"Elite"})]
pub struct TargetedPilot {
    pub pilot_name: EDString,
    #[serde(rename = "PilotName_Localised")]
    pub pilot_name_localised: EDString,
    pub pilot_rank: PilotRank,
    #[serde(rename = "SquadronID")]
    pub squadron_id: Option<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({"ShieldHealth":0.000000, "HullHealth":95.128372})]
pub struct TargetedHealth {
    pub shield_health: f64,
    pub hull_health: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({"Faction":"Foxworks Celestial", "LegalStatus":"Lawless", "Bounty":0})]
pub struct TargetedStatus {
    pub legal_status: LegalStatus,
    pub faction: Option<EDString>,
    pub power: Option<Power>,
    pub bounty: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2022-09-21T20:29:46Z", "event":"ShipTargeted", "TargetLocked":true, 
    "Ship":"vulture", "ScanStage":3, "PilotName":"$ShipName_Military_Independent;", 
    "PilotName_Localised":"System Defence Force", "PilotRank":"Elite", "ShieldHealth":0.000000, 
    "HullHealth":95.128372, "Faction":"Foxworks Celestial", "LegalStatus":"Lawless", "Bounty":0, 
    "Subsystem":"$int_powerdistributor_size5_class3_name;", 
    "Subsystem_Localised":"Power Distributor", "SubsystemHealth":98.958328 })]
pub struct EDLogShipTargeted {
    pub target_locked: bool,
    #[serde(flatten)]
    pub targeted_ship: Option<TargetedShip>,
    #[serde(flatten)]
    pub targeted_pilot: Option<TargetedPilot>,
    #[serde(flatten)]
    pub targeted_health: Option<TargetedHealth>,
    #[serde(flatten)]
    pub targeted_status: Option<TargetedStatus>,
    #[serde(flatten)]
    pub subsystem: Option<TargetedSubsystem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-06-26T17:29:05Z", "event":"SRVDestroyed", "ID":36, "SRVType":"testbuggy", "SRVType_Localised":"SRV Scarab" })]
pub struct EDLogSRVDestroyed {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(rename = "SRVType")]
    pub srv_type: ShipType,
    #[serde(rename = "SRVType_Localised")]
    pub srv_type_localised: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AttackTarget {
    Fighter,
    Mothership,
    You,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-07-25T12:50:03Z", "event":"UnderAttack", "Target":"You" })]
pub struct EDLogUnderAttack {
    target: Option<AttackTarget>,
}
