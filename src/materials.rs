use ed_parse_log_files_macros::{Extractable, testcase_struct};
use serde::{Deserialize, Serialize};

use crate::EDString;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct RawMaterial {
    name: EDString,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Name":"decodedemissiondata", "Count":9 })]
pub struct NonRawMaterial {
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMaterials {
    raw: Vec<RawMaterial>,
    manufactured: Vec<NonRawMaterial>,
    encoded: Vec<NonRawMaterial>,
}
