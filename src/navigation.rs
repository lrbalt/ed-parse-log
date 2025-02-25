use crate::common_types::{Allegiance, BodyInformation, BodyType, StarClass, StationInformation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EDLogFuelScoop {
    scooped: f64,
    total: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogLiftoff {
    player_controlled: bool,
    taxi: Option<bool>,
    multicrew: Option<bool>,
    #[serde(flatten)]
    start_system_info: Option<BodyInformation>,
    on_station: Option<bool>,
    on_planet: Option<bool>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    nearest_destination: Option<String>,
    #[serde(rename = "NearestDestination_Localised")]
    nearest_destination_localised: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EDLogApproachSettlement {
    name: String,
    #[serde(flatten)]
    station_information: Option<StationInformation>,
    system_address: u64,
    #[serde(rename = "BodyID")]
    body_id: u64,
    body_name: String,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EDLogFSDTarget {
    name: String,
    system_address: u64,
    star_class: StarClass,
    remaining_jumps_in_route: Option<u64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
// todo: refactor with location->edloglocation
pub struct EDLogFSDJump {
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
    jump_dist: f64,
    fuel_used: f64,
    fuel_level: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDockSRV {
    #[serde(rename = "SRVType")]
    srvtype: Option<String>,
    #[serde(rename = "SRVType_Localised")]
    srvtype_localised: Option<String>,
    #[serde(rename = "ID")]
    id: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogLaunchSRV {
    #[serde(rename = "SRVType")]
    srvtype: Option<String>,
    #[serde(rename = "SRVType_Localised")]
    srvtype_localised: Option<String>,
    loadout: String,
    #[serde(rename = "ID")]
    id: u64,
    player_controlled: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSRVDestroyed {
    #[serde(rename = "ID")]
    id: u64,
    #[serde(rename = "SRVType")]
    srv_type: String,
    #[serde(rename = "SRVType_Localised")]
    srv_type_localised: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogTouchdown {
    player_controlled: bool,
    taxi: Option<bool>,
    multicrew: Option<bool>,
    #[serde(flatten)]
    start_system_info: Option<BodyInformation>,
    on_station: Option<bool>,
    on_planet: Option<bool>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    nearest_destination: Option<String>,
    #[serde(rename = "NearestDestination_Localised")]
    nearest_destination_localised: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum JumpType {
    Hyperspace,
    Supercruise,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct JumpToStarsystem {
    star_system: String,
    system_address: u64,
    star_class: StarClass,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogStartJump {
    jump_type: JumpType,
    taxi: Option<bool>,
    #[serde(flatten)]
    star_system: Option<JumpToStarsystem>,
}
