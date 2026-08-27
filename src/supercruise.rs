use crate::EDString;
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
