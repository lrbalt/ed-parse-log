use crate::common_types::{Credits, DroneType};
use ed_parse_log_files_macros::Extractable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(deny_unknown_fields)]
pub struct EDLogLaunchDrone {
    #[serde(rename = "Type")]
    drone_type: DroneType,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogRepairDrone {
    hull_repaired: Option<f64>,
    corrosion_repaired: Option<f64>,
    cockpit_repaired: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSellDrones {
    #[serde(rename = "Type")]
    pub drone_type: DroneType,
    pub count: u64,
    pub sell_price: Credits,
    pub total_sale: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogBuyDrones {
    #[serde(rename = "Type")]
    pub drone_type: DroneType,
    pub count: u64,
    pub buy_price: Credits,
    pub total_cost: Credits,
}
