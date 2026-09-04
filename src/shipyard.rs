use crate::{EDString, ship_type::ShipType};
use ed_parse_log_files_macros::Extractable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogShipyardRedeem {
    pub ship_type: ShipType,
    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localised: Option<EDString>,
    #[serde(rename = "BundleID")]
    pub bundle_id: u64,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogShipRedeemed {
    pub ship_type: ShipType,
    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localised: Option<EDString>,
    #[serde(rename = "NewShipID")]
    pub new_ship_id: u64,
}
