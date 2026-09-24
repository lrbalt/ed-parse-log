use crate::{EDString, common_types::Credits, market_item::MarketItemType};
use ed_parse_log_files_macros::{Extractable, testcase};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-06-08T17:12:44Z", "event":"AsteroidCracked", "Body":"Omicron Capricorni B B 1 A Ring" })]
pub struct EDLogAsteroidCracked {
    // name of nearest body
    pub body: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-06-28T19:16:15Z", "event":"BuyTradeData", "System":"Quator", "Cost":100 })]
pub struct EDLogBuyTradeData {
    pub system: EDString,
    pub cost: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(deny_unknown_fields)]
#[testcase({ "timestamp":"2025-12-25T11:30:48Z", "event":"CollectCargo", "Type":"USSCargoBlackBox", 
    "Type_Localised":"Black Box", "Stolen":true, "MissionID":1040003416 })]
#[testcase({ "timestamp":"2026-09-21T20:57:11Z", "event":"CollectCargo", "Type":"USSCargoRareArtwork", 
    "Type_Localised":"Rare Artwork", "Stolen":false, "MissionID":1066613618 })]
pub struct EDLogCollectCargo {
    #[serde(rename = "Type")]
    pub cargo_type: MarketItemType,
    #[serde(rename = "Type_Localised")]
    pub cargo_type_localised: Option<EDString>,
    #[serde(rename = "Stolen")]
    pub stolen: bool,
    #[serde(rename = "MissionID")]
    pub mission_id: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-06-07T23:31:33Z", "event":"EjectCargo", "Type":"alliancetradeagreements", 
             "Type_Localised":"Alliance Trade Agreements", "Count":2, "Abandoned":false, "PowerplayOrigin":"" })]
pub struct EDLogEjectCargo {
    #[serde(rename = "Type")]
    pub cargo_type: MarketItemType,
    #[serde(rename = "Type_Localised")]
    pub cargo_type_localised: Option<EDString>,
    pub count: u64,
    pub abandoned: bool,
    // If the cargo is related to powerplay delivery from outlying systems back to the centre:
    pub powerplay_origin: Option<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-08T09:51:16Z", "event":"MarketBuy", "MarketID":128781521, "Type":"reactivearmour", "Type_Localised":"Reactive Armour", "Count":60, "BuyPrice":1626, "TotalCost":97560 })]
pub struct EDLogMarketBuy {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    #[serde(rename = "Type")]
    pub buy_type: MarketItemType,
    #[serde(rename = "Type_Localised")]
    pub buy_type_localised: Option<EDString>,
    pub count: u64,
    pub buy_price: Credits,
    pub total_cost: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-10-17T03:29:58Z", "event":"MarketSell", "Type":"biowaste", "Count":1, "SellPrice":10, "TotalSale":10, "AvgPricePaid":0 })]
#[testcase({ "timestamp":"2017-10-17T03:29:58Z", "event":"MarketSell","Type":"mineraloil", "Count":9, "SellPrice":72, "TotalSale":648, "AvgPricePaid":0, "StolenGoods":true, "BlackMarket":true })]
pub struct EDLogMarketSell {
    #[serde(rename = "MarketID")]
    pub market_id: Option<u64>,
    #[serde(rename = "Type")]
    pub sell_type: MarketItemType,
    #[serde(rename = "Type_Localised")]
    pub sell_type_localised: Option<EDString>,
    pub count: u64,
    pub sell_price: Credits,
    pub total_sale: Credits,
    pub avg_price_paid: Credits,
    pub illegal_goods: Option<bool>,
    pub stolen_goods: Option<bool>,
    pub black_market: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-04-20T18:01:40Z", "event":"MiningRefined", "Type":"$tritium_name;", "Type_Localised":"Tritium" })]
#[testcase({"timestamp": "2026-08-27T20:37:02Z","event": "MiningRefined","Type": "$tantalum_name;","Type_Localised": "Tantalum"})]
pub struct EDLogMiningRefined {
    #[serde(rename = "Type")]
    pub material_type: MarketItemType,
    #[serde(rename = "Type_Localised")]
    pub material_type_localised: EDString,
}
