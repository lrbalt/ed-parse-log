use crate::{EDString, common_types::Credits, odyssey::MicroResource};
use ed_parse_log_files_macros::Extractable;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDeliverPowerMicroResources {
    total_count: u64,
    micro_resources: Vec<MicroResource>,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub enum MarketItemCategory {
    #[serde(alias = "$MARKET_category_chemicals;")]
    Chemicals,
    #[serde(alias = "$MARKET_category_consumer_items;")]
    #[strum(to_string = "Consumer items")]
    Consumeritems,
    #[serde(alias = "$MARKET_category_drugs;")]
    #[strum(to_string = "Legal drugs")]
    Legaldrugs,
    #[serde(alias = "$MARKET_category_foods;")]
    Foods,
    #[serde(alias = "$MARKET_category_industrial_materials;")]
    #[strum(to_string = "Industrial materials")]
    Industrialmaterials,
    #[serde(alias = "$MARKET_category_machinery;")]
    Machinery,
    #[serde(alias = "$MARKET_category_metals;")]
    Metals,
    #[serde(alias = "$MARKET_category_medicines;")]
    Medicines,
    #[serde(alias = "$MARKET_category_minerals;")]
    Minerals,
    #[serde(alias = "$MARKET_category_salvage;")]
    Salvage,
    #[serde(alias = "$MARKET_category_slaves;")]
    #[strum(to_string = "Slavery")]
    Slavery,
    #[serde(alias = "$MARKET_category_technology;")]
    Technology,
    #[serde(alias = "$MARKET_category_textiles;")]
    Textiles,
    #[serde(alias = "$MARKET_category_waste;")]
    Waste,
    #[serde(alias = "$MARKET_category_weapons;")]
    Weapons,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMarketID {}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct RequiredResource {
    pub name: EDString,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<EDString>,
    pub required_amount: u64,
    pub provided_amount: u64,
    pub payment: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ContributedResource {
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    amount: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogColonisationConstructionDepot {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub construction_progress: f64,
    pub construction_complete: bool,
    pub construction_failed: bool,
    pub resources_required: Vec<RequiredResource>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogColonisationContribution {
    #[serde(rename = "MarketID")]
    market_id: u64,
    contributions: Vec<ContributedResource>,
}
