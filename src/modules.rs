use crate::{
    common_types::{Credits, EngineerModification},
    log_line::EDLogEvent,
};
use ed_parse_log_files_macros::{Extractable, testcase};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct AvailableStoredModule {
    star_system: String,
    #[serde(rename = "MarketID")]
    market_id: u64,
    transfer_cost: u64,
    transfer_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct IntransitModule {
    in_transit: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct StoredModule {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: String,
    storage_slot: u64,
    #[serde(flatten)]
    available: Option<AvailableStoredModule>,
    #[serde(flatten)]
    engineer_modification: Option<EngineerModification>,
    #[serde(flatten)]
    intransit: Option<IntransitModule>,
    buy_price: u64,
    hot: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogStoredModules {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub station_name: String,
    pub star_system: String,
    pub items: Vec<StoredModule>,
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
    market_id: Option<u64>,
    slot: String,
    retrieved_item: String,
    #[serde(rename = "RetrievedItem_Localised")]
    retrieved_item_localised: String,
    ship: String,
    #[serde(rename = "ShipID")]
    ship_id: u32,
    hot: Option<bool>,
    #[serde(flatten)]
    engineer_modification: Option<EngineerModification>,
    #[serde(flatten)]
    swap_out_item: Option<SwapOutItem>,
    cost: Option<Credits>,
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
pub struct MassStoredModule {
    slot: String,
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: String,
    hot: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMassModuleStore {
    #[serde(rename = "MarketID")]
    market_id: u64,
    ship: String,
    #[serde(rename = "ShipID")]
    ship_id: u32,
    items: Vec<MassStoredModule>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-10-17T02:57:13Z", "event":"ModuleStore", "Slot":"Slot06_Size2", "StoredItem":"$int_repairer_size2_class3_name;", "StoredItem_Localised":"AFM Unit", "Ship":"cobramkiii", "ShipID":1 })]
pub struct EDLogModuleStore {
    #[serde(rename = "MarketID")]
    market_id: Option<u64>,
    slot: String,
    stored_item: String,
    #[serde(rename = "StoredItem_Localised")]
    stored_item_localised: String,
    ship: String,
    #[serde(rename = "ShipID")]
    ship_id: u32,
    hot: Option<bool>,
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
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
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

#[test]
fn test_exploration() {
    let json = r#"{ "Name":"$int_cargorack_size6_class1_name;", "Name_Localised":"Cargo Rack", "StorageSlot":158, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":362591, "Hot":false }"#;
    let line: StoredModule = serde_json::from_str(json).expect("Should parse");
    assert_eq!(line.storage_slot, 158);

    let json = r#"{ "timestamp":"2023-07-10T14:05:54Z", "event":"StoredModules", "MarketID":3702691328, "StationName":"T6Y-35X", "StarSystem":"LP 932-12", "Items":[  ] }"#;
    let _line: crate::log_line::EDLogLine = serde_json::from_str(json).expect("Should parse");

    let json = r#"{ "timestamp":"2023-07-10T14:05:54Z", "event":"StoredModules", "MarketID":3702691328, "StationName":"T6Y-35X", "StarSystem":"LP 932-12", "Items":[ 
        { "Name":"$int_powerplant_size8_class2_name;", "Name_Localised":"Power Plant", "StorageSlot":139, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":3675145, "Hot":false, "EngineerModifications":"PowerPlant_Boosted", "Level":4, "Quality":1.000000 }, 
        { "Name":"$int_sensors_size7_class5_name;", "Name_Localised":"Sensors", "StorageSlot":182, "StarSystem":"LHS 547", "MarketID":3224113408, "TransferCost":7717, "TransferTime":420, "BuyPrice":8272137, "Hot":false, "EngineerModifications":"Sensor_LightWeight", "Level":5, "Quality":1.000000 }, 
        { "Name":"$int_hyperdrive_size7_class1_name;", "Name_Localised":"FSD", "StorageSlot":178, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":0, "Hot":false }, 
        { "Name":"$int_cargorack_size7_class1_name;", "Name_Localised":"Cargo Rack", "StorageSlot":162, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":0, "Hot":false }, 
        { "Name":"$int_cargorack_size6_class1_name;", "Name_Localised":"Cargo Rack", "StorageSlot":158, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":362591, "Hot":false }, 
        { "Name":"$int_hyperdrive_size5_class5_name;", "Name_Localised":"FSD", "StorageSlot":200, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":5103953, "Hot":false, "EngineerModifications":"FSD_LongRange", "Level":5, "Quality":1.000000 }, 
        { "Name":"$int_shieldcellbank_size5_class5_name;", "Name_Localised":"Shield Cell Bank", "StorageSlot":8, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":1241317, "Hot":false }, 
        { "Name":"$int_engine_size5_class4_name;", "Name_Localised":"Thrusters", "StorageSlot":188, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":1701318, "Hot":false, "EngineerModifications":"Engine_Dirty", "Level":3, "Quality":1.000000 }, 
        { "Name":"$int_shieldgenerator_size5_class3_fast_name;", "Name_Localised":"Bi-Weave Shield", "StorageSlot":122, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":850659, "Hot":false, "EngineerModifications":"ShieldGenerator_Thermic", "Level":3, "Quality":1.000000 }, 
        { "Name":"$int_sensors_size5_class3_name;", "Name_Localised":"Sensors", "StorageSlot":154, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":198611, "Hot":false, "EngineerModifications":"Sensor_LongRange", "Level":3, "Quality":0.988000 }, 
        { "Name":"$int_cargorack_size5_class1_name;", "Name_Localised":"Cargo Rack", "StorageSlot":129, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":111566, "Hot":false }, 
        { "Name":"$int_expmodulestabiliser_size5_class3_name;", "Name_Localised":"Experimental Weapon Stabiliser", "StorageSlot":1, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":4000000, "Hot":false }, 
        { "Name":"$int_fuelscoop_size4_class5_name;", "Name_Localised":"Fuel Scoop", "StorageSlot":187, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":2433010, "Hot":false }, 
        { "Name":"$int_powerdistributor_size4_class5_name;", "Name_Localised":"Power Distributor", "StorageSlot":155, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":443328, "Hot":false, "EngineerModifications":"PowerDistributor_HighFrequency", "Level":3, "Quality":1.000000 }, 
        { "Name":"$int_modulereinforcement_size4_class2_name;", "Name_Localised":"Module Reinforcement", "StorageSlot":153, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":165750, "Hot":false }, 
        { "Name":"$int_cargorack_size4_class1_name;", "Name_Localised":"Cargo Rack", "StorageSlot":193, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":29178, "Hot":false }, 
        { "Name":"$int_refinery_size3_class5_name;", "Name_Localised":"Refinery", "StorageSlot":170, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":2143260, "Hot":false }, 
        { "Name":"$int_fuelscoop_size3_class5_name;", "Name_Localised":"Fuel Scoop", "StorageSlot":157, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":767511, "Hot":false }, 
        { "Name":"$int_engine_size3_class5_name;", "Name_Localised":"Thrusters", "StorageSlot":177, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":507912, "Hot":false }, 
        { "Name":"$int_hyperdrive_size3_class5_name;", "Name_Localised":"FSD", "StorageSlot":176, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":507912, "Hot":false }, 
        { "Name":"$int_engine_size3_class5_name;", "Name_Localised":"Thrusters", "StorageSlot":143, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":507912, "Hot":false }, 
        { "Name":"$int_hyperdrive_size3_class5_name;", "Name_Localised":"FSD", "StorageSlot":142, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":507912, "Hot":false }, 
        { "Name":"$int_powerplant_size3_class5_name;", "Name_Localised":"Power Plant", "StorageSlot":144, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":480411, "Hot":false }, 
        { "Name":"$int_sensors_size3_class5_name;", "Name_Localised":"Sensors", "StorageSlot":174, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":158331, "Hot":false }, 
        { "Name":"$int_dronecontrol_collection_size3_class5_name;", "Name_Localised":"Collector", "StorageSlot":159, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":73440, "Hot":false }, 
        { "Name":"$hpt_plasmaaccelerator_fixed_large_name;", "Name_Localised":"Plasma Acc", "StorageSlot":140, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":3051200, "Hot":false }, 
        { "Name":"$int_modulereinforcement_size3_class2_name;", "Name_Localised":"Module Reinforcement", "StorageSlot":195, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":71400, "Hot":false }, 
        { "Name":"$int_cargorack_size3_class1_name;", "Name_Localised":"Cargo Rack", "StorageSlot":192, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":8978, "Hot":false }, 
        { "Name":"$int_fuelscoop_size3_class1_name;", "Name_Localised":"Fuel Scoop", "StorageSlot":168, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":3386, "Hot":false }, 
        { "Name":"$int_cargorack_size3_class1_name;", "Name_Localised":"Cargo Rack", "StorageSlot":194, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":0, "Hot":false }, 
        { "Name":"$int_expmodulestabiliser_size3_class3_name;", "Name_Localised":"Experimental Weapon Stabiliser", "StorageSlot":3, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":2000000, "Hot":false }, 
        { "Name":"$int_fuelscoop_size2_class5_name;", "Name_Localised":"Fuel Scoop", "StorageSlot":199, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":284844, "Hot":false }, 
        { "Name":"$int_powerdistributor_size2_class5_name;", "Name_Localised":"Power Distributor", "StorageSlot":175, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":56547, "Hot":false }, 
        { "Name":"$hpt_beamlaser_gimbal_medium_name;", "Name_Localised":"Beam Laser", "StorageSlot":186, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":425510, "Hot":false, "EngineerModifications":"Weapon_LongRange", "Level":3, "Quality":1.000000 }, 
        { "Name":"$int_hullreinforcement_size2_class2_name;", "Name_Localised":"Hull Reinforcement", "StorageSlot":127, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":30600, "Hot":false, "EngineerModifications":"HullReinforcement_HeavyDuty", "Level":1, "Quality":1.000000 }, 
        { "Name":"$int_hullreinforcement_size2_class1_name;", "Name_Localised":"Hull Reinforcement", "StorageSlot":164, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":12000, "Hot":false }, 
        { "Name":"$int_cargorack_size2_class1_name;", "Name_Localised":"Cargo Rack", "StorageSlot":169, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":0, "Hot":false }, 
        { "Name":"$hpt_multicannon_gimbal_medium_name;", "Name_Localised":"Multi-Cannon", "StorageSlot":126, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":48450, "Hot":false, "EngineerModifications":"Weapon_Overcharged", "Level":4, "Quality":0.787000 }, 
        { "Name":"$int_buggybay_size2_class1_name;", "Name_Localised":"Planetary Vehicle Hangar", "StorageSlot":167, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":18000, "Hot":false }, 
        { "Name":"$asp_armour_grade3_name;", "Name_Localised":"Military Grade Composite", "StorageSlot":165, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":5995038, "Hot":false, "EngineerModifications":"Armour_HeavyDuty", "Level":4, "Quality":0.726000 }, 
        { "Name":"$int_dronecontrol_prospector_size1_class5_name;", "Name_Localised":"Prospector", "StorageSlot":191, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":9600, "Hot":false }, 
        { "Name":"$int_lifesupport_size1_class4_name;", "Name_Localised":"Life Support", "StorageSlot":141, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":8078, "Hot":false }, 
        { "Name":"$int_dronecontrol_collection_size1_class4_name;", "Name_Localised":"Collector", "StorageSlot":185, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":4800, "Hot":false }, 
        { "Name":"$typex_armour_grade1_name;", "Name_Localised":"Lightweight Alloy", "StorageSlot":183, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":0, "Hot":false }, 
        { "Name":"$cutter_armour_grade1_name;", "Name_Localised":"Lightweight Alloy", "StorageSlot":148, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":0, "Hot":false, "EngineerModifications":"Armour_HeavyDuty", "Level":1, "Quality":0.970000 }, 
        { "Name":"$anaconda_armour_grade1_name;", "Name_Localised":"Lightweight Alloys", "StorageSlot":132, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":0, "Hot":false, "EngineerModifications":"Armour_HeavyDuty", "Level":4, "Quality":1.000000 }, 
        { "Name":"$hpt_beamlaser_gimbal_small_name;", "Name_Localised":"Beam Laser", "StorageSlot":125, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":74650, "Hot":false }, 
        { "Name":"$hpt_pulselaser_gimbal_small_name;", "Name_Localised":"Pulse Laser", "StorageSlot":134, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":5610, "Hot":false }, 
        { "Name":"$int_detailedsurfacescanner_tiny_name;", "Name_Localised":"Surface Scanner", "StorageSlot":181, "InTransit":true, "BuyPrice":212500, "Hot":false, "EngineerModifications":"Sensor_Expanded", "Level":5, "Quality":1.000000 }, 
        { "Name":"$hpt_mrascanner_size0_class5_name;", "Name_Localised":"Pulse Wave", "StorageSlot":160, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":1097095, "Hot":false }, 
        { "Name":"$hpt_crimescanner_size0_class5_name;", "Name_Localised":"K-Warrant Scanner", "StorageSlot":147, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":932531, "Hot":false, "EngineerModifications":"Sensor_LongRange", "Level":2, "Quality":0.327900 }, 
        { "Name":"$hpt_cloudscanner_size0_class4_name;", "Name_Localised":"Wake Scanner", "StorageSlot":198, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":310844, "Hot":false, "EngineerModifications":"Sensor_LongRange", "Level":1, "Quality":1.000000 }, 
        { "Name":"$hpt_electroniccountermeasure_tiny_name;", "Name_Localised":"ECM", "StorageSlot":130, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":12500, "Hot":false }, 
        { "Name":"$hpt_chafflauncher_tiny_name;", "Name_Localised":"Chaff", "StorageSlot":133, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":8500, "Hot":false }, 
        { "Name":"$hpt_heatsinklauncher_turret_tiny_name;", "Name_Localised":"Heatsink", "StorageSlot":180, "StarSystem":"LP 932-12", "MarketID":3702691328, "TransferCost":0, "TransferTime":0, "BuyPrice":2975, "Hot":false } 
        ] }"#;
    let line: crate::log_line::EDLogLine = serde_json::from_str(json).expect("Should parse");

    assert!(matches!(
        line.event(),
        crate::log_line::EDLogEvent::StoredModules(_)
    ));
    if let crate::log_line::EDLogEvent::StoredModules(header) = line.event() {
        assert_eq!(header.items.len(), 55);
        assert_eq!(header.market_id, 3702691328);
    }
}
