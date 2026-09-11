use crate::{EDString, common_types::Credits};
use ed_parse_log_files_macros::testcase_struct;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Faction":"Imperial Grey Wolves", "Reward":50160 })]
pub struct BountyReward {
    pub faction: EDString,
    pub reward: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({"PilotName":"$npc_name_decorate:#name=Florian Poprat;", "PilotName_Localised":"Florian Poprat"})]
pub struct BountyPilot {
    pilot_name: EDString,
    #[serde(rename = "PilotName_Localised")]
    pilot_name_localised: EDString,
}
