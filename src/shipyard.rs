use crate::{
    common_types::{Credits, StarSystemData},
    log_line::{EDLogEvent, Extractable},
    ship::ShipType,
};
use ed_parse_log_files_macros::testcase;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Ship {
    #[serde(rename = "ShipID")]
    ship_id: u64,
    ship_type: ShipType,
    #[serde(rename = "ShipType_Localised")]
    ship_type_localised: Option<String>,
    name: Option<String>,
    #[serde(flatten)]
    start_system_data: Option<StarSystemData>,
    in_transit: Option<bool>,
    value: Credits,
    hot: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogShipyardSwap {
    ship_type: ShipType,
    #[serde(rename = "ShipType_Localised")]
    ship_type_localised: Option<String>,
    #[serde(rename = "ShipID")]
    ship_id: u64,
    store_old_ship: ShipType,
    #[serde(rename = "StoreShipID")]
    store_ship_id: u64,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogShipyardTransfer {
    ship_type: ShipType,
    #[serde(rename = "ShipType_Localised")]
    ship_type_localised: Option<String>,
    #[serde(rename = "ShipID")]
    ship_id: u64,
    system: Option<String>,
    #[serde(rename = "ShipMarketID")]
    ship_market_id: u64,
    distance: f64,
    transfer_price: Credits,
    transfer_time: u64,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogShipyardNew {
    ship_type: ShipType,
    #[serde(rename = "ShipType_Localised")]
    ship_type_localised: Option<String>,
    #[serde(rename = "NewShipID")]
    new_ship_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogShipyard {
    #[serde(rename = "MarketID")]
    market_id: u64,
    station_name: String,
    star_system: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogShipyardBuy {
    ship_type: ShipType,
    #[serde(rename = "ShipType_Localised")]
    ship_type_localised: Option<String>,
    ship_price: Credits,
    store_old_ship: String,
    #[serde(rename = "StoreShipID")]
    store_ship_id: u64,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogShipyardSell {
    ship_type: ShipType,
    #[serde(rename = "ShipType_Localised")]
    ship_type_localised: Option<String>,
    #[serde(rename = "SellShipID")]
    sell_ship_id: u64,
    ship_price: Credits,
    system: Option<String>,
    #[serde(rename = "ShipMarketID")]
    ship_market_id: Option<u64>,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogShipyardRedeem {
    ship_type: ShipType,
    #[serde(rename = "ShipType_Localised")]
    ship_type_localised: Option<String>,
    #[serde(rename = "BundleID")]
    bundle_id: u64,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogShipRedeemed {
    ship_type: ShipType,
    #[serde(rename = "ShipType_Localised")]
    ship_type_localised: Option<String>,
    #[serde(rename = "NewShipID")]
    new_ship_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-06-20T21:24:02Z", "event":"SellShipOnRebuy", "ShipType":"mamba", "System":"14 Ceti", "SellShipId":3, "ShipPrice":73796265 })]
pub struct EDLogSellShipOnRebuy {
    ship_type: ShipType,
    #[serde(rename = "ShipType_Localised")]
    ship_type_localised: Option<String>,
    system: String,
    sell_ship_id: u64,
    ship_price: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogStoredShips {
    pub station_name: String,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub star_system: String,
    pub ships_here: Vec<Ship>,
    pub ships_remote: Vec<Ship>,
}

impl Extractable for EDLogStoredShips {
    fn extract(event: &EDLogEvent) -> Option<&Self> {
        if let EDLogEvent::StoredShips(loc) = event {
            return Some(loc);
        }
        None
    }
}
