use serde::{Deserialize, Serialize};

use crate::ed_log::common_types::StarSystemData;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Ship {
    #[serde(rename = "ShipID")]
    ship_id: u64,
    ship_type: String,
    #[serde(rename = "ShipType_Localised")]
    ship_type_localised: Option<String>,
    name: Option<String>,
    #[serde(flatten)]
    start_system_data: Option<StarSystemData>,
    in_transit: Option<bool>,
    value: u64,
    hot: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogShipyardSwap {
    ship_type: String,
    #[serde(rename = "ShipType_Localised")]
    ship_type_localised: Option<String>,
    #[serde(rename = "ShipID")]
    ship_id: u64,
    store_old_ship: String,
    #[serde(rename = "StoreShipID")]
    store_ship_id: u64,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogShipyardTransfer {
    ship_type: String,
    #[serde(rename = "ShipType_Localised")]
    ship_type_localised: Option<String>,
    #[serde(rename = "ShipID")]
    ship_id: u64,
    system: Option<String>,
    #[serde(rename = "ShipMarketID")]
    ship_market_id: u64,
    distance: f64,
    transfer_price: u64,
    transfer_time: u64,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogShipyardNew {
    ship_type: String, // TODO: ShipType
    #[serde(rename = "ShipType_Localised")]
    ship_type_localised: Option<String>,
    #[serde(rename = "NewShipID")]
    new_ship_id: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogShipyard {
    #[serde(rename = "MarketID")]
    market_id: u64,
    station_name: String,
    star_system: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogShipyardBuy {
    ship_type: String, // TODO: ShipType
    #[serde(rename = "ShipType_Localised")]
    ship_type_localised: Option<String>,
    ship_price: u64,
    store_old_ship: String,
    #[serde(rename = "StoreShipID")]
    store_ship_id: u64,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogShipyardSell {
    ship_type: String,
    #[serde(rename = "SellShipID")]
    sell_ship_id: u64,
    ship_price: u64,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogShipyardRedeem {
    ship_type: String, // TODO ShipType
    #[serde(rename = "BundleID")]
    bundle_id: u64,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogShipRedeemed {
    ship_type: String, // TODO ShipType
    #[serde(rename = "NewShipID")]
    new_ship_id: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogStoredShips {
    station_name: String,
    #[serde(rename = "MarketID")]
    market_id: u64,
    star_system: String,
    ships_here: Vec<Ship>,
    ships_remote: Vec<Ship>,
}
