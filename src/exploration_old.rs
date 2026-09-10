use crate::{
    EDString,
    common_types::{ShipScanType, SignalType},
};
use ed_parse_log_files_macros::Extractable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogScanned {
    scan_type: ShipScanType,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDatalinkScan {
    message: EDString,
    #[serde(rename = "Message_Localised")]
    message_localised: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ProspectedMaterial {
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    proportion: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogProspectedAsteroid {
    materials: Vec<ProspectedMaterial>,
    motherlode_material: Option<EDString>,
    #[serde(rename = "MotherlodeMaterial_Localised")]
    motherlode_material_localised: Option<EDString>,
    content: EDString,
    #[serde(rename = "Content_Localised")]
    content_localised: EDString,
    remaining: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDataScanned {
    #[serde(rename = "Type")]
    data_type: EDString,
    #[serde(rename = "Type_Localised")]
    data_type_localised: Option<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogStationBernalSphere {
    system_address: u64,
    signal_name: EDString,
    signal_type: SignalType,
    is_station: bool,
}
