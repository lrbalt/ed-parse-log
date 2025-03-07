use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct EDLogRawMaterial {
    name: String,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct EDLogNonRawMaterial {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: String,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct EDLogMaterials {
    raw: Vec<EDLogRawMaterial>,
    manufactured: Vec<EDLogNonRawMaterial>,
    encoded: Vec<EDLogNonRawMaterial>,
}
