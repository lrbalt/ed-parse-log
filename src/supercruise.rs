use crate::common_types::BodyType;
use ed_parse_log_file_testcase::testcase;
use serde::{Deserialize, Serialize};

#[testcase({ "timestamp":"2023-07-30T20:54:01Z", "event":"SupercruiseDestinationDrop", "Type":"Wrangell Terminal", "Threat":0, "MarketID":3228997120 })]
#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSupercruiseEntry {
    taxi: Option<bool>,
    multicrew: Option<bool>,
    star_system: String,
    system_address: u64,
}