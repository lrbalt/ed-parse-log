use crate::ed_log::common_types::{
    Allegiance, BodyType, FactionName, FactionState, PowerplayState, StationInformation,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Powers {
    #[serde(flatten)]
    powers: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Faction {
    name: String,
    faction_state: FactionState,
    government: String,
    influence: f64,
    allegiance: String,
    happiness: String,
    #[serde(rename = "Happiness_Localised")]
    happiness_localised: Option<String>,
    my_reputation: f64,
    recovering_states: Option<Vec<FactionRecoveringState>>,
    active_states: Option<Vec<FactionActiveState>>,
    pending_states: Option<Vec<FactionPendingState>>,
}

#[derive(Serialize, Deserialize)]
pub struct Trend {
    trend: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct FactionRecoveringState {
    state: FactionState,
    trend: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct FactionActiveState {
    state: FactionState,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct FactionPendingState {
    state: FactionState,
    trend: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WarType {
    Election,
    War,
    CivilWar,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConflictStatus {
    #[serde(rename = "")]
    None,
    Active,
    Pending,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ConflictFaction {
    name: String,
    stake: String,
    won_days: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Conflict {
    war_type: WarType,
    status: ConflictStatus,
    faction1: ConflictFaction,
    faction2: ConflictFaction,
}

#[derive(Serialize, Deserialize)]
pub enum ThargoidWarState {
    Unknown,
    #[serde(rename = "")]
    None,
    #[serde(rename = "Thargoid_Harvest")]
    Harvest,
    #[serde(rename = "Thargoid_Recovery")]
    Recovery,
    #[serde(rename = "Thargoid_Controlled")]
    Controlled,
    #[serde(rename = "Thargoid_Stronghold")]
    Stronghold,
    #[serde(rename = "Thargoid_Probing")]
    Probing,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ThargoidWar {
    current_state: ThargoidWarState,
    next_state_success: ThargoidWarState,
    next_state_failure: ThargoidWarState,
    success_state_reached: bool,
    war_progress: f64,
    remaining_ports: u64,
    estimated_remaining_time: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogLocation {
    docked: bool,
    on_foot: Option<bool>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    #[serde(rename = "DistFromStarLS")]
    dist_from_star_ls: Option<f64>,
    #[serde(flatten)]
    station_info: Option<StationInformation>,
    taxi: Option<bool>,
    multicrew: Option<bool>,
    #[serde(rename = "InSRV")]
    in_srv: Option<bool>,
    star_system: String,
    system_address: u64,
    star_pos: [f64; 3],
    system_allegiance: Allegiance,
    system_economy: String,
    #[serde(rename = "SystemEconomy_Localised")]
    system_economy_localised: String,
    system_second_economy: String,
    #[serde(rename = "SystemSecondEconomy_Localised")]
    system_second_economy_localised: String,
    system_government: String,
    #[serde(rename = "SystemGovernment_Localised")]
    system_government_localised: String,
    system_security: String,
    #[serde(rename = "SystemSecurity_Localised")]
    system_security_localised: String,
    population: u64,
    body: String,
    #[serde(rename = "BodyID")]
    body_id: u64,
    body_type: BodyType,
    controlling_power: Option<String>,
    powers: Option<Vec<String>>,
    powerplay_state: Option<PowerplayState>,
    thargoid_war: Option<ThargoidWar>,
    factions: Option<Vec<Faction>>,
    system_faction: Option<FactionName>,
    conflicts: Option<Vec<Conflict>>,
}
