use crate::EDString;
use ed_parse_log_files_macros::Extractable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum UnknownItem {}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDeleteSuitLoadout {
    #[serde(rename = "SuitID")]
    suit_id: u64,
    suit_name: EDString,
    #[serde(rename = "SuitName_Localised")]
    suit_name_localised: EDString,
    #[serde(rename = "LoadoutID")]
    loadout_id: u64,
    loadout_name: EDString,
}
