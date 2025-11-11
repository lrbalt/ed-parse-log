use crate::common_types::ModuleEngineeringModifiers;
use ed_parse_log_files_macros::Extractable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum EngineerProgress {
    Invited,
    Unlocked,
    Known,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Engineer {
    engineer: Option<String>,
    #[serde(rename = "EngineerID")]
    engineer_id: Option<u64>,
    progress: EngineerProgress,
    rank_progress: Option<u64>,
    rank: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogEngineerProgress {
    #[serde(flatten)]
    pub engineer: Option<Engineer>,
    pub engineers: Option<Vec<Engineer>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EngineerCraftIngredient {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ExperimentalEffect {
    apply_experimental_effect: String,
    experimental_effect: String,
    #[serde(rename = "ExperimentalEffect_Localised")]
    experimental_effect_localised: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogEngineerCraft {
    pub slot: String,
    pub module: String,
    pub ingredients: Vec<EngineerCraftIngredient>,
    pub engineer: Option<String>,
    #[serde(rename = "EngineerID")]
    pub engineer_id: u64,
    #[serde(rename = "BlueprintID")]
    pub blueprint_id: u64,
    pub blueprint_name: String,
    pub level: u64,
    pub quality: f64,
    #[serde(flatten)]
    pub experimental_effect: Option<ExperimentalEffect>,
    pub modifiers: Vec<ModuleEngineeringModifiers>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogEngineerContribution {
    pub engineer: String,
    #[serde(rename = "EngineerID")]
    pub engineer_id: u64,
    #[serde(rename = "Type")]
    pub contribution_type: String,
    pub commodity: Option<String>,
    #[serde(rename = "Commodity_Localised")]
    pub commodity_localised: Option<String>,
    pub material: Option<String>,
    #[serde(rename = "Material_Localised")]
    pub material_localised: Option<String>,
    pub quantity: u64,
    pub total_quantity: u64,
}
