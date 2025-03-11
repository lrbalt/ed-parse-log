use crate::{common_types::ModuleEngineeringModifiers, ship::ShipType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct FuelCapacity {
    main: f64,
    reserve: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ModuleEngineering {
    engineer: Option<String>,
    #[serde(rename = "EngineerID")]
    engineer_id: u64,
    #[serde(rename = "BlueprintID")]
    blueprint_id: u64,
    blueprint_name: String,
    level: u64,
    quality: f64,
    experimental_effect: Option<String>,
    #[serde(rename = "ExperimentalEffect_Localised")]
    experimental_effect_localised: Option<String>,
    modifiers: Vec<ModuleEngineeringModifiers>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Module {
    slot: String,
    item: String,
    on: bool,
    priority: u64,
    ammo_in_clip: Option<u64>,
    ammo_in_hopper: Option<u64>,
    health: f64,
    value: Option<u64>,
    engineering: Option<ModuleEngineering>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct EDLogLoadout {
    ship: ShipType,
    #[serde(rename = "ShipID")]
    ship_id: u64,
    ship_name: String,
    ship_ident: String,
    hull_value: Option<u64>,
    modules_value: Option<u64>,
    hull_health: f64,
    hot: Option<bool>,
    unladen_mass: f64,
    cargo_capacity: u64,
    max_jump_range: f64,
    fuel_capacity: FuelCapacity,
    rebuy: u64,
    modules: Vec<Module>,
}
