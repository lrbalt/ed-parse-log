use crate::{EDString, market_item_type::MarketItemType};
use ed_parse_log_files_macros::testcase_struct;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Name":"algae", "Count":1 })]
pub struct Inventory {
    name: MarketItemType,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    count: u64,
    stolen: Option<u64>,
    #[serde(rename = "MissionID")]
    mission_id: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LegalStatus {
    Unknown,
    None,
    Clean,
    Wanted,
    WantedEnemy,
    Lawless,
    Enemy,
    Hunter,
    Thargoid22,
}
