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

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum EngineerContributionType {
    Bond,
    Bounty,
    Commodity,
    Credits,
    Materials,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2024-01-20T19:22:48Z", "event":"EngineerContribution", "Engineer":"Colonel Bris Dekker", 
    "EngineerID":300140, "Type":"Bond", "Quantity":126004, "TotalQuantity":1000000 })]
#[testcase({ "timestamp":"2024-01-03T19:35:39Z", "event":"EngineerContribution", "Engineer":"The Sarge", "EngineerID":300040, 
    "Type":"Materials", "Material":"shieldpatternanalysis", "Material_Localised":"Aberrant Shield Pattern Analysis", 
    "Quantity":50, "TotalQuantity":50 })]
#[testcase({ "timestamp":"2024-01-03T14:41:59Z", "event":"EngineerContribution", "Engineer":"Zacariah Nemo", "EngineerID":300050, 
    "Type":"Commodity", "Commodity":"xihecompanions", "Commodity_Localised":"Xihe Biomorphic Companions", "Quantity":5, 
    "TotalQuantity":25 })]
#[testcase({ "timestamp":"2023-07-23T21:19:45Z", "event":"EngineerContribution", "Engineer":"Mel Brandon", "EngineerID":300280, 
    "Type":"Bounty", "Quantity":100000, "TotalQuantity":100000 })]
pub struct EDLogEngineerContribution {
    pub engineer: EDString,
    #[serde(rename = "EngineerID")]
    pub engineer_id: u64,
    #[serde(rename = "Type")]
    pub contribution_type: EngineerContributionType,
    pub commodity: Option<EDString>,
    #[serde(rename = "Commodity_Localised")]
    pub commodity_localised: Option<EDString>,
    pub material: Option<EDString>,
    #[serde(rename = "Material_Localised")]
    pub material_localised: Option<EDString>,
    pub quantity: u64,
    pub total_quantity: u64,
}
