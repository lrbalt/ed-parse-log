use crate::common_types::EngineerModification;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AvailableStoredModule {
    star_system: String,
    #[serde(rename = "MarketID")]
    market_id: u64,
    transfer_cost: u64,
    transfer_time: u64,
    #[serde(flatten)]
    engineer_modification: Option<EngineerModification>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct IntransitModule {
    in_transit: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StoredModule {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: String,
    storage_slot: u64,
    #[serde(flatten)]
    available: Option<AvailableStoredModule>,
    #[serde(flatten)]
    intransit: Option<IntransitModule>,
    buy_price: u64,
    hot: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogStoredModules {
    #[serde(rename = "MarketID")]
    market_id: u64,
    station_name: String,
    star_system: String,
    items: Vec<StoredModule>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SwapOutItem {
    swap_out_item: String,
    #[serde(rename = "SwapOutItem_Localised")]
    swap_out_item_localised: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogModuleRetrieve {
    #[serde(rename = "MarketID")]
    market_id: u64,
    slot: String,
    retrieved_item: String,
    #[serde(rename = "RetrievedItem_Localised")]
    retrieved_item_localised: String,
    ship: String,
    #[serde(rename = "ShipID")]
    ship_id: u32,
    hot: bool,
    #[serde(flatten)]
    engineer_modification: Option<EngineerModification>,
    #[serde(flatten)]
    swap_out_item: Option<SwapOutItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SellItem {
    sell_item: String,
    #[serde(rename = "SellItem_Localised")]
    sell_item_localised: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogModuleBuy {
    slot: String,
    stored_item: Option<String>,
    #[serde(rename = "StoredItem_Localised")]
    stored_item_localised: Option<String>,
    buy_item: String,
    #[serde(rename = "BuyItem_Localised")]
    buy_item_localised: String,
    #[serde(flatten)]
    sell_item: Option<SellItem>,
    sell_price: Option<u64>,
    #[serde(rename = "MarketID")]
    market_id: u64,
    buy_price: u64,
    ship: String,
    #[serde(rename = "ShipID")]
    ship_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogModuleSell {
    #[serde(rename = "MarketID")]
    market_id: u64,
    slot: String,
    sell_item: String,
    #[serde(rename = "SellItem_Localised")]
    sell_item_localised: String,
    sell_price: u64,
    ship: String,
    #[serde(rename = "ShipID")]
    ship_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogModuleSellRemote {
    storage_slot: u64,
    sell_item: String,
    #[serde(rename = "SellItem_Localised")]
    sell_item_localised: String,
    server_id: u64,
    sell_price: u64,
    ship: String,
    #[serde(rename = "ShipID")]
    ship_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogModuleSwap {
    #[serde(rename = "MarketID")]
    market_id: u64,
    from_slot: String,
    to_slot: String,
    from_item: String,
    #[serde(rename = "FromItem_Localised")]
    from_item_localised: String,
    to_item: String,
    #[serde(rename = "ToItem_Localised")]
    to_item_localised: Option<String>,
    ship: String,
    #[serde(rename = "ShipID")]
    ship_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogModuleStore {
    #[serde(rename = "MarketID")]
    market_id: u64,
    slot: String,
    stored_item: String,
    #[serde(rename = "StoredItem_Localised")]
    stored_item_localised: String,
    ship: String,
    #[serde(rename = "ShipID")]
    ship_id: u32,
    hot: bool,
    #[serde(flatten)]
    engineer_modification: Option<EngineerModification>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogModuleBuyAndStore {
    buy_item: String,
    #[serde(rename = "BuyItem_Localised")]
    buy_item_localised: String,
    #[serde(rename = "MarketID")]
    market_id: u64,
    buy_price: u64,
    ship: String,
    #[serde(rename = "ShipID")]
    ship_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct EDLogFetchRemoteModule {
    storage_slot: u64,
    stored_item: String,
    #[serde(rename = "StoredItem_Localised")]
    stored_item_localised: String,
    server_id: u64,
    transfer_cost: u64,
    transfer_time: u64,
    ship: String,
    #[serde(rename = "ShipID")]
    ship_id: u64,
}
