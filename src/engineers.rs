use crate::common_types::ModuleEngineeringModifiers;
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogEngineerProgress {
    #[serde(flatten)]
    engineer: Option<Engineer>,
    engineers: Option<Vec<Engineer>>,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogEngineerCraft {
    slot: String,
    module: String,
    ingredients: Vec<EngineerCraftIngredient>,
    engineer: Option<String>,
    #[serde(rename = "EngineerID")]
    engineer_id: u64,
    #[serde(rename = "BlueprintID")]
    blueprint_id: u64,
    blueprint_name: String,
    level: u64,
    quality: f64,
    #[serde(flatten)]
    experimental_effect: Option<ExperimentalEffect>,
    modifiers: Vec<ModuleEngineeringModifiers>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogEngineerContribution {
    engineer: String,
    #[serde(rename = "EngineerID")]
    engineer_id: u64,
    #[serde(rename = "Type")]
    contribution_type: String,
    commodity: Option<String>,
    #[serde(rename = "Commodity_Localised")]
    commodity_localised: Option<String>,
    material: Option<String>,
    #[serde(rename = "Material_Localised")]
    material_localised: Option<String>,
    quantity: u64,
    total_quantity: u64,
}
