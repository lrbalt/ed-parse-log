use crate::{
    EDString,
    common_types::{Credits, ModuleEngineeringModifiers},
    ship::ShipType,
};
use ed_parse_log_files_macros::{Extractable, testcase, testcase_struct};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct FuelCapacity {
    main: f64,
    reserve: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({"Engineer":"Zacariah Nemo", "EngineerID":300050, "BlueprintID":128673459, "BlueprintName":"Weapon_Overcharged", "Level":5, "Quality":1.000000, "ExperimentalEffect":"special_drag_munitions", "ExperimentalEffect_Localised":"Drag Munitions", "Modifiers":[]})]
pub struct ModuleEngineering {
    engineer: Option<EDString>,
    #[serde(rename = "EngineerID")]
    engineer_id: u64,
    #[serde(rename = "BlueprintID")]
    blueprint_id: u64,
    blueprint_name: EDString,
    level: u64,
    quality: f64,
    experimental_effect: Option<EDString>,
    #[serde(rename = "ExperimentalEffect_Localised")]
    experimental_effect_localised: Option<EDString>,
    modifiers: Vec<ModuleEngineeringModifiers>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Slot":"LargeHardpoint1", "Item":"hpt_slugshot_fixed_large_range", "On":true, "Priority":0, "AmmoInClip":3, "AmmoInHopper":180, "Health":1.000000, "Value":1536538})]
pub struct Module {
    slot: EDString,
    item: EDString,
    on: bool,
    priority: u64,
    ammo_in_clip: Option<u64>,
    ammo_in_hopper: Option<u64>,
    health: f64,
    value: Option<Credits>,
    engineering: Option<ModuleEngineering>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LoadOutStats {
    hull_value: Credits,
    modules_value: Credits,
    hull_health: f64,
    unladen_mass: f64,
    cargo_capacity: u64,
    max_jump_range: f64,
    fuel_capacity: FuelCapacity,
    rebuy: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-10-15T04:34:40Z", "event":"Loadout", "Ship":"CobraMkIII", "ShipID":1, "ShipName":"Flat Head", "ShipIdent":"UNSC-1", "Modules":[  ] })]
#[testcase({ "timestamp": "2024-01-14T18:13:22Z", "event": "Loadout", "Ship": "cobramkiii", "ShipID": 23, "ShipName": "HMS SCAVENGER", "ShipIdent": "LRB-C3", 
    "HullValue": 349718, "ModulesValue": 6316388,"HullHealth": 1.000000, "UnladenMass": 222.577606, "CargoCapacity": 60, "MaxJumpRange": 39.726162,
    "FuelCapacity": {"Main": 16.000000,"Reserve": 0.490000},"Rebuy": 333307,"Modules": []})]
pub struct EDLogLoadout {
    ship: ShipType,
    #[serde(rename = "ShipID")]
    ship_id: u64,
    ship_name: EDString,
    ship_ident: EDString,
    hot: Option<bool>,
    #[serde(flatten)]
    loadout_stats: Option<LoadOutStats>,
    modules: Vec<Module>,
}
