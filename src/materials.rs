use ed_parse_log_files_macros::{Extractable, testcase_struct};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct RawMaterial {
    name: String,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Name":"decodedemissiondata", "Count":9 })]
pub struct NonRawMaterial {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMaterials {
    raw: Vec<RawMaterial>,
    manufactured: Vec<NonRawMaterial>,
    encoded: Vec<NonRawMaterial>,
}
