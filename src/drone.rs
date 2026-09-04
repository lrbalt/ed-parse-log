use crate::common_types::DroneType;
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
