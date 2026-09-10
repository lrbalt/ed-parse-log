use chrono::{DateTime, Utc};
use ed_parse_log_files::{
    common_types::{CarrierDockingAccess, Credits, StationType},
    log_line::{EDLogEvent, EDLogLine},
    market::MarketItemCategory,
    market_item_type::MarketItemType,
};
use serde::Deserialize;

#[test]
// read these shiplocker data and check if they parse.
fn test_shiplocker_examples() {
    let json = [include_str!("../testdata/ShipLocker.json")];

    let lines: Result<Vec<EDLogLine>, _> = json.into_iter().map(serde_json::from_str).collect();

    if let Ok(lines) = lines {
        for line in lines {
            assert!(matches!(line.event(), EDLogEvent::ShipLocker(_)));
        }
    } else {
        // parse again, but retain all errors this time
        let lines: Vec<Result<EDLogLine, _>> = json.into_iter().map(serde_json::from_str).collect();
        for line in lines {
            assert!(line.is_ok())
        }
    }
}

#[test]
// read these market data and check if they parse.
fn test_market_examples() {
    let json = [
        include_str!("../testdata/Market - rare goods.json"),
        include_str!("../testdata/Market - robardin-rock.json"),
        include_str!("../testdata/Market - on surface mining.json"),
    ];

    let lines: Result<Vec<EDLogLine>, _> = json.into_iter().map(serde_json::from_str).collect();

    if let Ok(lines) = lines {
        for line in lines {
            assert!(matches!(line.event(), EDLogEvent::Market(_)));
        }
    } else {
        // create untyped struct to parse market data using strings

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "PascalCase", deny_unknown_fields)]
        #[allow(unused)]
        pub struct SimpleEDLogMarket {
            #[serde(rename = "MarketID")]
            pub market_id: u64,
            pub station_name: String,
            pub station_type: StationType,
            pub carrier_docking_access: Option<CarrierDockingAccess>,
            pub star_system: String,
            pub items: Option<Vec<SimpleMarketItem>>,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "PascalCase", deny_unknown_fields)]
        #[allow(unused)]
        pub struct SimpleMarketItem {
            #[serde(rename = "id")]
            id: u64,
            #[serde(rename = "Name")]
            pub market_item_name: String,
            #[serde(rename = "Name_Localised")]
            pub market_item_name_localised: Option<String>,
            pub category: String,
            #[serde(rename = "Category_Localised")]
            pub category_localised: Option<String>,
            buy_price: Credits,
            sell_price: Credits,
            mean_price: Credits,
            stock_bracket: u64,
            demand_bracket: u64,
            stock: u64,
            demand: u64,
            consumer: bool,
            producer: bool,
            rare: bool,
        }

        #[derive(Deserialize, Debug)]
        #[allow(unused)]
        pub struct SimpleEDLogLine {
            timestamp: DateTime<Utc>,
            #[serde(flatten)]
            event: SimpleEDLogEvent,
        }

        #[derive(Deserialize, Debug)]
        #[serde(tag = "event", deny_unknown_fields)]
        pub enum SimpleEDLogEvent {
            Market(SimpleEDLogMarket),
        }

        // parse market data into simple structs

        let lines: Vec<SimpleEDLogLine> = json
            .into_iter()
            .map(|j| serde_json::from_str(j).expect("Simple variant of datastructure should parse"))
            .collect();

        let mut err = false;
        for line in lines {
            let SimpleEDLogEvent::Market(data) = line.event;

            // check all items in the simple market data and try to parse into market_item_type
            for item in data.items.expect("testdata should contain market_items") {
                let name: Result<MarketItemType, _> =
                    serde_json::from_str(&format!("\"{}\"", item.market_item_name));
                let name_loc = item.market_item_name_localised.clone();

                if name.is_err()
                    || (item.market_item_name_localised.is_some()
                        && name.unwrap().to_string() != name_loc.unwrap())
                {
                    err = true;

                    let pcname = item
                        .market_item_name_localised
                        .as_ref()
                        .map(|s| s.split(" ").collect::<Vec<_>>().join(""));

                    println!(
                        "#[serde(alias = \"{}\")]#[strum(to_string = \"{}\")]{},",
                        item.market_item_name,
                        item.market_item_name_localised
                            .as_ref()
                            .unwrap_or(&item.market_item_name),
                        pcname.unwrap_or(item.market_item_name.clone())
                    );
                }

                let cat: Result<MarketItemCategory, _> =
                    serde_json::from_str(&format!("\"{}\"", item.category));
                let cat_loc = item.category_localised.clone();

                let pcname = item
                    .category_localised
                    .as_ref()
                    .map(|s| s.split(" ").collect::<Vec<_>>().join(""));

                if cat.is_err()
                    || (item.category_localised.is_some()
                        && cat.unwrap().to_string() != cat_loc.unwrap())
                {
                    err = true;
                    println!(
                        "#[serde(alias = \"{}\")]#[strum(to_string=\"{}\")]{},",
                        item.category,
                        item.category_localised.as_ref().unwrap_or(&item.category),
                        pcname.unwrap_or(item.category.clone())
                    );
                }
            }
        }

        assert!(!err);
    }
}
