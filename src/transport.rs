use crate::{EDString, common_types::Credits};
use ed_parse_log_files_macros::{Extractable, testcase};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCancelTaxi {
    pub refund: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2023-08-16T09:00:44Z", "event":"BookTaxi", "Cost":4532, "DestinationSystem":"Ratraii", 
    "DestinationLocation":"Uniyal Extraction Prospect" })]
pub struct EDLogBookTaxi {
    pub cost: Credits,
    pub destination_system: EDString,
    pub destination_location: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDropshipDeploy {
    star_system: EDString,
    system_address: u64,
    body: EDString,
    #[serde(rename = "BodyID")]
    body_id: u64,
    on_station: bool,
    on_planet: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogBookDropship {
    retreat: bool,
    cost: Credits,
    destination_system: EDString,
    destination_location: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCancelDropship {
    refund: Credits,
}
