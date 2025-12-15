use crate::EDString;
use ed_parse_log_files_macros::{Extractable, testcase};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ShipLockerItem {
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    #[serde(rename = "OwnerID")]
    owner_id: u64,
    #[serde(rename = "MissionID")]
    mission_id: Option<u64>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LockerContent {
    items: Vec<ShipLockerItem>,
    components: Vec<ShipLockerItem>,
    consumables: Vec<ShipLockerItem>,
    data: Vec<ShipLockerItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-12-09T19:58:35Z", "event":"ShipLocker" })]
#[testcase({ "timestamp":"2025-12-09T20:19:43Z", "event":"ShipLocker", 
    "Items":[ { "Name":"geneticsample", "Name_Localised":"Biological Sample", "OwnerID":0, "Count":20 }, { "Name":"lazarus", "OwnerID":0, "Count":6 }], 
    "Components":[ { "Name":"graphene", "OwnerID":0, "Count":40 }, { "Name":"carbonfibreplating", "Name_Localised":"Carbon Fibre Plating", "OwnerID":0, "Count":10 }], 
    "Consumables":[ { "Name":"healthpack", "Name_Localised":"Medkit", "OwnerID":0, "Count":100 } ], 
    "Data":[ { "Name":"biometricdata", "Name_Localised":"Biometric Data", "OwnerID":0, "Count":8 }] })]
pub struct EDLogShipLocker {
    #[serde(flatten)]
    content: Option<LockerContent>,
}
