use ed_parse_log_files_macros::Extractable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ShipLockerItem {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    #[serde(rename = "OwnerID")]
    owner_id: u64,
    #[serde(rename = "MissionID")]
    mission_id: Option<u64>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LockerContent {
    items: Vec<ShipLockerItem>,
    components: Vec<ShipLockerItem>,
    consumables: Vec<ShipLockerItem>,
    data: Vec<ShipLockerItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogShipLocker {
    #[serde(flatten)]
    content: Option<LockerContent>,
}
