use crate::{
    EDString,
    common_types::{ScanType, ShipScanType, SignalType},
};
use ed_parse_log_files_macros::{Extractable, testcase};
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

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-11-13T20:22:43Z", "event":"ScanOrganic", "ScanType":"Sample", "Genus":"$Codex_Ent_Ingensradices_Genus_Name;", "Genus_Localised":"Radicoida", "Species":"$Codex_Ent_Ingensradices_Unicus_Name;", "Species_Localised":"Radicoida Unica", "Variant":"$Codex_Ent_Ingensradices_Unicus_Name;", "Variant_Localised":"Radicoida Unica", "WasLogged":false, "SystemAddress":147882789259, "Body":3 })]
pub struct EDLogScanOrganic {
    pub scan_type: ScanType,
    pub genus: EDString,
    #[serde(rename = "Genus_Localised")]
    pub genus_localised: EDString,
    pub species: EDString,
    #[serde(rename = "Species_Localised")]
    pub species_localised: EDString,
    pub variant: Option<EDString>,
    #[serde(rename = "Variant_Localised")]
    pub variant_localised: Option<EDString>,
    pub system_address: u64,
    pub body: u64,
    pub was_logged: Option<bool>,
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
pub struct EDLogAsteroidCracked {
    body: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMiningRefined {
    #[serde(rename = "Type")]
    material_type: EDString,
    #[serde(rename = "Type_Localised")]
    material_type_localised: EDString,
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
