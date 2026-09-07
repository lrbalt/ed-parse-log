use crate::{EDString, common_types::Credits};
use ed_parse_log_files_macros::{Extractable, testcase};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MicroResourceType {
    Data,
    Item,
    Component,
    Consumable,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct MicroResource {
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    category: MicroResourceType,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogBuyMicroResources {
    pub total_count: Option<u64>,
    #[serde(flatten)]
    pub micro_resource: Option<MicroResource>,
    pub micro_resources: Option<Vec<MicroResource>>,
    pub price: Credits,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogTradeMicroResources {
    offered: Vec<MicroResource>,
    total_count: u64,
    received: EDString,
    #[serde(rename = "Received_Localised")]
    received_localised: Option<EDString>,
    count: u64,
    category: MicroResourceType,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-01-21T20:24:20Z", "event":"SellMicroResources", "TotalCount":44, 
    "MicroResources":[ 
        { "Name":"compactlibrary", "Name_Localised":"Compact Library", "Category":"Item", "Count":1 }, 
        { "Name":"insight", "Category":"Item", "Count":1 } ], 
    "Price":479000, "MarketID":3228823296 })]
pub struct EDLogSellMicroResources {
    pub total_count: u64,
    pub micro_resources: Vec<MicroResource>,
    pub price: Credits,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDeliverPowerMicroResources {
    total_count: u64,
    micro_resources: Vec<MicroResource>,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub enum MarketItemCategory {
    #[serde(alias = "$MARKET_category_chemicals;")]
    Chemicals,
    #[serde(alias = "$MARKET_category_consumer_items;")]
    #[strum(to_string = "Consumer items")]
    Consumeritems,
    #[serde(alias = "$MARKET_category_drugs;")]
    #[strum(to_string = "Legal drugs")]
    Legaldrugs,
    #[serde(alias = "$MARKET_category_foods;")]
    Foods,
    #[serde(alias = "$MARKET_category_industrial_materials;")]
    #[strum(to_string = "Industrial materials")]
    Industrialmaterials,
    #[serde(alias = "$MARKET_category_machinery;")]
    Machinery,
    #[serde(alias = "$MARKET_category_metals;")]
    Metals,
    #[serde(alias = "$MARKET_category_medicines;")]
    Medicines,
    #[serde(alias = "$MARKET_category_minerals;")]
    Minerals,
    #[serde(alias = "$MARKET_category_salvage;")]
    Salvage,
    #[serde(alias = "$MARKET_category_slaves;")]
    #[strum(to_string = "Slavery")]
    Slavery,
    #[serde(alias = "$MARKET_category_technology;")]
    Technology,
    #[serde(alias = "$MARKET_category_textiles;")]
    Textiles,
    #[serde(alias = "$MARKET_category_waste;")]
    Waste,
    #[serde(alias = "$MARKET_category_weapons;")]
    Weapons,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMarketID {}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct RequiredResource {
    pub name: EDString,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<EDString>,
    pub required_amount: u64,
    pub provided_amount: u64,
    pub payment: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ContributedResource {
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    amount: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogColonisationConstructionDepot {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub construction_progress: f64,
    pub construction_complete: bool,
    pub construction_failed: bool,
    pub resources_required: Vec<RequiredResource>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogColonisationContribution {
    #[serde(rename = "MarketID")]
    market_id: u64,
    contributions: Vec<ContributedResource>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SoldBioData {
    pub genus: EDString,
    #[serde(rename = "Genus_Localised")]
    pub genus_localised: EDString,
    pub species: EDString,
    #[serde(rename = "Species_Localised")]
    pub species_localised: EDString,
    pub variant: Option<EDString>,
    #[serde(rename = "Variant_Localised")]
    pub variant_localised: Option<EDString>,
    pub value: Credits,
    pub bonus: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSellOrganicData {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub bio_data: Vec<SoldBioData>,
}

#[test]
// read these market data and check if they parse.
fn test_market_examples() {
    let json = [
        include_str!("../testdata/Market - rare goods.json"),
        include_str!("../testdata/Market - robardin-rock.json"),
        include_str!("../testdata/Market - on surface mining.json"),
    ];

    let lines: Result<Vec<crate::log_line::EDLogLine>, _> =
        json.into_iter().map(serde_json::from_str).collect();

    if let Ok(lines) = lines {
        for line in lines {
            assert!(matches!(
                line.event(),
                crate::log_line::EDLogEvent::Market(_)
            ));
        }
    } else {
        use chrono::{DateTime, Utc};

        // create untyped struct to parse market data using strings

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "PascalCase", deny_unknown_fields)]
        #[allow(unused)]
        pub struct SimpleEDLogMarket {
            #[serde(rename = "MarketID")]
            pub market_id: u64,
            pub station_name: String,
            pub station_type: crate::common_types::StationType,
            pub carrier_docking_access: Option<crate::common_types::CarrierDockingAccess>,
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
                let name: Result<crate::market_item_type::MarketItemType, _> =
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
