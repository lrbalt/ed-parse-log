use serde::{Deserialize, Serialize};

use crate::ed_log::common_types::BodyType;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSupercruiseDestinationDrop {
    #[serde(rename = "Type")]
    dest_type: String,
    #[serde(rename = "Type_Localised")]
    dest_type_localised: Option<String>,
    threat: u64,
    #[serde(rename = "MarketID")]
    market_id: Option<u64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EDLogSupercruiseExit {
    taxi: Option<bool>,
    multicrew: Option<bool>,
    star_system: String,
    system_address: u64,
    body: String,
    #[serde(rename = "BodyID")]
    body_id: u64,
    body_type: BodyType,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EDLogSupercruiseEntry {
    taxi: Option<bool>,
    multicrew: Option<bool>,
    star_system: String,
    system_address: u64,
}
