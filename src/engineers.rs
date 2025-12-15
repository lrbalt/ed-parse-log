use crate::{EDString, common_types::ModuleEngineeringModifiers};
use ed_parse_log_files_macros::{Extractable, testcase, testcase_struct};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum EngineerProgress {
    Invited,
    Unlocked,
    Known,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Engineer":"Tod 'The Blaster' McQuinn", "EngineerID":300260, "Progress":"Known" })]
pub struct Engineer {
    engineer: Option<EDString>,
    #[serde(rename = "EngineerID")]
    engineer_id: Option<u64>,
    progress: EngineerProgress,
    rank_progress: Option<u64>,
    rank: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2022-06-13T18:58:38Z", "event":"EngineerProgress", "Engineer":"Felicity Farseer", "EngineerID":300100, "Progress":"Invited" })]
#[testcase({ "timestamp":"2022-06-15T18:12:12Z", "event":"EngineerProgress", "Engineers":[ { "Engineer":"Tod 'The Blaster' McQuinn", "EngineerID":300260, "Progress":"Known" } ] })]
pub struct EDLogEngineerProgress {
    #[serde(flatten)]
    pub engineer: Option<Engineer>,
    pub engineers: Option<Vec<Engineer>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EngineerCraftIngredient {
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ExperimentalEffect {
    apply_experimental_effect: EDString,
    experimental_effect: EDString,
    #[serde(rename = "ExperimentalEffect_Localised")]
    experimental_effect_localised: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogEngineerCraft {
    pub slot: EDString,
    pub module: EDString,
    pub ingredients: Vec<EngineerCraftIngredient>,
    pub engineer: Option<EDString>,
    #[serde(rename = "EngineerID")]
    pub engineer_id: u64,
    #[serde(rename = "BlueprintID")]
    pub blueprint_id: u64,
    pub blueprint_name: EDString,
    pub level: u64,
    pub quality: f64,
    #[serde(flatten)]
    pub experimental_effect: Option<ExperimentalEffect>,
    pub modifiers: Vec<ModuleEngineeringModifiers>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogEngineerContribution {
    pub engineer: EDString,
    #[serde(rename = "EngineerID")]
    pub engineer_id: u64,
    #[serde(rename = "Type")]
    pub contribution_type: EDString,
    pub commodity: Option<EDString>,
    #[serde(rename = "Commodity_Localised")]
    pub commodity_localised: Option<EDString>,
    pub material: Option<EDString>,
    #[serde(rename = "Material_Localised")]
    pub material_localised: Option<EDString>,
    pub quantity: u64,
    pub total_quantity: u64,
}
