use crate::{common_types::ModuleEngineeringModifiers, ship::ShipType};
use ed_parse_log_files_macros::testcase;
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
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LoadOutStats {
    hull_value: u64,
    modules_value: u64,
    hull_health: f64,
    unladen_mass: f64,
    cargo_capacity: u64,
    max_jump_range: f64,
    fuel_capacity: FuelCapacity,
    rebuy: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-10-15T04:34:40Z", "event":"Loadout", "Ship":"CobraMkIII", "ShipID":1, "ShipName":"Flat Head", "ShipIdent":"UNSC-1", "Modules":[  ] })]
#[testcase({ "timestamp": "2024-01-14T18:13:22Z", "event": "Loadout", "Ship": "cobramkiii", "ShipID": 23, "ShipName": "HMS SCAVENGER", "ShipIdent": "LRB-C3", 
    "HullValue": 349718, "ModulesValue": 6316388,"HullHealth": 1.000000, "UnladenMass": 222.577606, "CargoCapacity": 60, "MaxJumpRange": 39.726162,
    "FuelCapacity": {"Main": 16.000000,"Reserve": 0.490000},"Rebuy": 333307,"Modules": []})]
pub struct EDLogLoadout {
    ship: ShipType,
    #[serde(rename = "ShipID")]
    ship_id: u64,
    ship_name: String,
    ship_ident: String,
    hot: Option<bool>,
    #[serde(flatten)]
    loadout_stats: Option<LoadOutStats>,
    modules: Vec<Module>,
}
