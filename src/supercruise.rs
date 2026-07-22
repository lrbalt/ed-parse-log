use crate::{EDString, common_types::BodyType};
use ed_parse_log_files_macros::{Extractable, testcase};
use serde::{Deserialize, Serialize};

#[testcase({ "timestamp":"2023-07-30T20:54:01Z", "event":"SupercruiseDestinationDrop", "Type":"Wrangell Terminal", "Threat":0, "MarketID":3228997120 })]
#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSupercruiseDestinationDrop {
    #[serde(rename = "Type")]
    pub dest_type: EDString,
    #[serde(rename = "Type_Localised")]
    pub dest_type_localised: Option<EDString>,
    pub threat: u64,
    #[serde(rename = "MarketID")]
    pub market_id: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSupercruiseExit {
    pub taxi: Option<bool>,
    pub multicrew: Option<bool>,
    pub star_system: EDString,
    pub system_address: Option<u64>,
    pub body: EDString,
    #[serde(rename = "BodyID")]
    pub body_id: Option<u64>,
    pub body_type: BodyType,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSupercruiseEntry {
    pub taxi: Option<bool>,
    pub multicrew: Option<bool>,
    pub star_system: EDString,
    pub system_address: Option<u64>,
}
