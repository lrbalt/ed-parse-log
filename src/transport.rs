use ed_parse_log_files_macros::Extractable;
use serde::{Deserialize, Serialize};

use crate::common_types::Credits;

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCancelTaxi {
    refund: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogBookTaxi {
    cost: Credits,
    destination_system: String,
    destination_location: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDropshipDeploy {
    star_system: String,
    system_address: u64,
    body: String,
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
    destination_system: String,
    destination_location: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCancelDropship {
    refund: Credits,
}
