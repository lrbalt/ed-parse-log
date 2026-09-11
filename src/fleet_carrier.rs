use crate::{
    EDString,
    common_types::{
        Allegiance, BodyType, Conflict, Credits, Faction, FactionName, GovernmentType, Powers,
        StarPos, StationInformation, SystemSecurity, ThargoidWar,
    },
};
use chrono::{DateTime, Utc};
use ed_parse_log_files_macros::{Extractable, testcase};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CarrierType {
    FleetCarrier,
    SquadronCarrier,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CrewRole {
    Repair,
    VoucherRedemption,
    Exploration,
    Rearm,
    Refuel,
    Outfitting,
    VistaGenomics,
    Shipyard,
    Bartender,
    PioneerSupplies,
    BlackMarket,
    Captain,
    Commodities,
    CarrierFuel,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ShipPack {
    pack_theme: EDString,
    pack_tier: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ModulePack {
    pack_theme: EDString,
    pack_tier: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-04-19T12:23:01Z", "event":"CarrierJump", "Docked":false, "OnFoot":true, "StarSystem":"HR 6394", 
    "SystemAddress":491496950139, "StarPos":[-78.18750,82.56250,196.06250], "SystemAllegiance":"Independent", 
    "SystemEconomy":"$economy_Extraction;", "SystemEconomy_Localised":"Extraction", "SystemSecondEconomy":"$economy_Industrial;", 
    "SystemSecondEconomy_Localised":"Industrial", "SystemGovernment":"$government_Corporate;", 
    "SystemGovernment_Localised":"Corporate", "SystemSecurity":"$SYSTEM_SECURITY_low;", "SystemSecurity_Localised":"Low Security", 
    "Population":45926389, "Body":"HR 6394", "BodyID":0, "BodyType":"Star", "Powers":[ "Yuri Grom", "Nakato Kaine" ], 
    "PowerplayState":"Unoccupied", "PowerplayConflictProgress":[ { "Power":"Yuri Grom", "ConflictProgress":0.000000 }, 
    { "Power":"Nakato Kaine", "ConflictProgress":0.000000 } ], "Factions":[ { "Name":"Mercenaries of the 26th Division", 
    "FactionState":"None", "Government":"Corporate", "Influence":0.508000, "Allegiance":"Independent", 
    "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", "MyReputation":0.000000, "PendingStates":[ 
        { "State":"Expansion", "Trend":0 } ] }, { "Name":"HIP 82629 Shared", "FactionState":"None", 
        "Government":"Cooperative", "Influence":0.140000, "Allegiance":"Independent", 
        "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", "MyReputation":0.000000 }, 
        { "Name":"The Lab ADM", "FactionState":"None", "Government":"Anarchy", "Influence":0.088000, 
        "Allegiance":"Independent", "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", "MyReputation":0.000000 }, 
        { "Name":"HIP 83137 Purple Organisation", "FactionState":"None", "Government":"Anarchy", "Influence":0.047000, 
        "Allegiance":"Independent", "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", "MyReputation":0.000000 }, 
        { "Name":"HIP 83137 Co-operative", "FactionState":"None", "Government":"Cooperative", "Influence":0.217000, 
        "Allegiance":"Independent", "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", "MyReputation":-2.317700 } ], 
        "SystemFaction":{ "Name":"Mercenaries of the 26th Division" } })]
pub struct EDLogCarrierJump {
    pub taxi: Option<bool>,
    // TODO: location and fsdjump have overlapping fields
    pub star_system: EDString,
    pub system_address: u64,
    pub star_pos: StarPos,
    pub system_allegiance: Allegiance,
    pub system_economy: EDString,
    #[serde(rename = "SystemEconomy_Localised")]
    pub system_economy_localised: EDString,
    pub system_second_economy: EDString,
    #[serde(rename = "SystemSecondEconomy_Localised")]
    pub system_second_economy_localised: EDString,
    pub system_government: GovernmentType,
    #[serde(rename = "SystemGovernment_Localised")]
    pub system_government_localised: EDString,
    pub system_security: SystemSecurity,
    #[serde(rename = "SystemSecurity_Localised")]
    pub system_security_localised: EDString,
    pub population: u64,
    pub body: EDString,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
    pub body_type: BodyType,
    #[serde(flatten)]
    pub powerplay: Option<Powers>,
    pub thargoid_war: Option<ThargoidWar>,
    pub factions: Option<Vec<Faction>>,
    pub conflicts: Option<Vec<Conflict>>,
    pub system_faction: Option<FactionName>,
    #[serde(flatten)]
    pub station_information: Option<StationInformation>,
    pub docked: bool,
    pub on_foot: Option<bool>,
    pub multicrew: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CarrierVariant {
    CarrierDockB,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2024-01-08T12:36:19Z", "event":"CarrierBuy", "CarrierID":12341234, "BoughtAtMarket":3223259392, 
    "Location":"Mitnahas", "SystemAddress":7267218695553, "Price":5000000000, "Variant":"CarrierDockB", "Callsign":"A1A-A1A" })]
pub struct EDLogCarrierBuy {
    pub bought_at_market: u64,
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub location: EDString,
    pub system_address: u64,
    pub price: Credits,
    pub variant: CarrierVariant,
    pub callsign: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum DockingAccess {
    All,
    None,
    Friends,
    Squadron,
    SquadronFriends,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SpaceUsage {
    total_capacity: u64,
    crew: u64,
    cargo: u64,
    cargo_space_reserved: u64,
    ship_packs: u64,
    module_packs: u64,
    free_space: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Finance {
    pub carrier_balance: Credits,
    pub reserve_balance: Credits,
    pub available_balance: Credits,
    pub reserve_percent: Option<f64>,
    #[serde(rename = "TaxRate_rearm")]
    pub tax_rate_rearm: Option<u64>,
    #[serde(rename = "TaxRate_refuel")]
    pub tax_rate_refuel: Option<u64>,
    #[serde(rename = "TaxRate_repair")]
    pub tax_rate_repair: Option<u64>,
    #[serde(rename = "TaxRate_shipyard")]
    pub tax_rate_shipyard: Option<u64>,
    #[serde(rename = "TaxRate_outfitting")]
    pub tax_rate_outfitting: Option<u64>,
    #[serde(rename = "TaxRate_pioneersupplies")]
    pub tax_rate_pioneer_supplies: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ActivatedProps {
    enabled: bool,
    crew_name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CrewMember {
    crew_name: Option<EDString>,
    crew_role: CrewRole,
    activated: bool,
    enabled: Option<bool>,
    activated_props: Option<ActivatedProps>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-09-06T17:54:42Z", "event":"CarrierStats", "CarrierID":3706278912, "CarrierType":"FleetCarrier", 
    "Callsign":"A1A-A1A", "Name":"MyFleetCarrier", "DockingAccess":"all", "AllowNotorious":false, "FuelLevel":581, 
    "JumpRangeCurr":500.000000, "JumpRangeMax":500.000000, "PendingDecommission":false, 
    "SpaceUsage":{ "TotalCapacity":25000, "Crew":6370, "Cargo":9864, "CargoSpaceReserved":0, "ShipPacks":0, "ModulePacks":0, "FreeSpace":8766 }, 
    "Finance":{ "CarrierBalance":30383683869, "ReserveBalance":30431940067, "AvailableBalance":-48256198, "TaxRate_pioneersupplies":0, "TaxRate_rearm":25, "TaxRate_refuel":25, "TaxRate_repair":25 }, 
    "Crew":[ { "CrewRole":"BlackMarket", "Activated":false }, { "CrewRole":"Captain", "Activated":true, "Enabled":true, "CrewName":"Kirk Strickland" }, { "CrewRole":"Refuel", "Activated":true, "Enabled":true, "CrewName":"Akemi Cunningham" }, { "CrewRole":"Repair", "Activated":true, "Enabled":true, "CrewName":"Guinevere Shepherd" }, { "CrewRole":"Rearm", "Activated":true, "Enabled":true, "CrewName":"Marlee Bullock" }, { "CrewRole":"Commodities", "Activated":true, "Enabled":true, "CrewName":"Owen Grimes" }, { "CrewRole":"VoucherRedemption", "Activated":true, "Enabled":true, "CrewName":"Clementine Chandler" }, { "CrewRole":"Exploration", "Activated":true, "Enabled":true, "CrewName":"Medha Frost" }, { "CrewRole":"Shipyard", "Activated":true, "Enabled":false, "CrewName":"Chevelle Rivera" }, { "CrewRole":"Outfitting", "Activated":true, "Enabled":false, "CrewName":"Drew Gill" }, { "CrewRole":"CarrierFuel", "Activated":true, "Enabled":true, "CrewName":"Lauren Adkins" }, { "CrewRole":"VistaGenomics", "Activated":true, "Enabled":true, "CrewName":"Ramiro Bentley" }, { "CrewRole":"PioneerSupplies", "Activated":true, "Enabled":true, "CrewName":"Jenessa Alford" }, { "CrewRole":"Bartender", "Activated":true, "Enabled":false, "CrewName":"Aleeah Bogdani" } ], 
    "ShipPacks":[ { "PackTheme":"Zorgon Peterson - Cargo", "PackTier":1 } ], 
    "ModulePacks":[ { "PackTheme":"ExplosiveWeaponry", "PackTier":2 }  ] })]
pub struct EDLogCarrierStats {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub carrier_type: Option<CarrierType>,
    pub callsign: EDString,
    pub name: EDString,
    pub docking_access: DockingAccess,
    pub allow_notorious: bool,
    pub fuel_level: u64,
    pub jump_range_curr: f64,
    pub jump_range_max: f64,
    pub pending_decommission: bool,
    pub space_usage: SpaceUsage,
    pub finance: Finance,
    pub crew: Vec<CrewMember>,
    pub ship_packs: Vec<ShipPack>,
    pub module_packs: Vec<ModulePack>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2024-08-25T17:21:24Z", "event":"CarrierJumpRequest", "CarrierID":123456789, "SystemName":"Eurybia", "Body":"Eurybia 2", "SystemAddress":1458309141194, "BodyID":7, "DepartureTime":"2024-08-25T17:46:10Z" })]
#[testcase({ "timestamp":"2025-08-21T19:35:04Z", "event":"CarrierJumpRequest", "CarrierType":"FleetCarrier", "CarrierID":123456789, "SystemName":"Prooe Drye LV-C c1-2", "Body":"Prooe Drye LV-C c1-2", "SystemAddress":631192163082, "BodyID":0, "DepartureTime":"2025-08-21T20:08:10Z" })]
pub struct EDLogCarrierJumpRequest {
    #[serde(rename = "CarrierID")]
    carrier_id: u64,
    carrier_type: Option<CarrierType>,
    system_name: EDString,
    system_address: u64,
    body: Option<EDString>,
    #[serde(rename = "BodyID")]
    body_id: u64,
    departure_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2020-03-11T15:12:26Z", "event":"CarrierDecommission", "CarrierID":3700005632,
"ScrapRefund":1746872629, "ScrapTime":1584601200 })]
pub struct EDLogCarrierDecommission {
    #[serde(rename = "CarrierID")]
    carrier_id: u64,
    scrap_refund: Credits,
    scrap_time: u64, // timestamp. TODO: is this Duration in secs?
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2020-03-11T15:12:38Z", "event":"CarrierCancelDecommission", "CarrierID":3700005632 })]
pub struct EDLogCarrierCancelDecommission {
    #[serde(rename = "CarrierID")]
    carrier_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-08-21T10:38:45Z", "event":"CarrierBankTransfer", "CarrierID":123456789, "CarrierType":"FleetCarrier", "Deposit":12610000, "PlayerBalance":18879865108, "CarrierBalance":18879410990 })]
pub struct EDLogCarrierBankTransfer {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub carrier_type: Option<CarrierType>,
    pub deposit: Option<Credits>,
    pub withdraw: Option<Credits>,
    pub player_balance: Credits,
    pub carrier_balance: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({"timestamp":"2025-04-07T14:42:30Z","event":"CarrierDepositFuel","CarrierID":3710508288,"Amount":317,"Total":1000})]
pub struct EDLogCarrierDepositFuel {
    #[serde(rename = "CarrierID")]
    carrier_id: u64,
    amount: u64,
    total: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CrewServiceOperation {
    Activate,
    Deactivate,
    Pause,
    Resume,
    Replace,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-08-20T13:11:21Z", "event":"CarrierCrewServices", "CarrierID":3706278912, "CarrierType":"FleetCarrier", 
"CrewRole":"Bartender", "Operation":"Pause", "CrewName":"Aleeah Bogdani" })]
pub struct EDLogCarrierCrewServices {
    #[serde(rename = "CarrierID")]
    carrier_id: u64,
    carrier_type: Option<CarrierType>,
    operation: CrewServiceOperation,
    crew_role: CrewRole,
    crew_name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-08-21T10:38:49Z", "event":"CarrierFinance", "CarrierID":123456789, "CarrierType":"FleetCarrier", "CarrierBalance":18879410990, "ReserveBalance":18879410990, "AvailableBalance":0, "ReservePercent":100, "TaxRate_rearm":25, "TaxRate_refuel":25, "TaxRate_repair":25 })]
pub struct EDLogCarrierFinance {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub carrier_type: Option<CarrierType>,
    pub carrier_balance: Credits,
    pub reserve_balance: Credits,
    pub available_balance: Credits,
    pub reserve_percent: u8,
    // following not in manual, but found in logs
    #[serde(rename = "TaxRate_pioneersupplies")]
    pub tax_rate_pioneersupplies: Option<u8>,
    #[serde(rename = "TaxRate_rearm")]
    pub tax_rate_rearm: Option<u8>,
    #[serde(rename = "TaxRate_refuel")]
    pub tax_rate_refuel: Option<u8>,
    #[serde(rename = "TaxRate_repair")]
    pub tax_rate_repair: Option<u8>,
    #[serde(rename = "TaxRate_shipyard")]
    pub tax_rate_shipyard: Option<u8>,
    #[serde(rename = "TaxRate_outfitting")]
    pub tax_rate_outfitting: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CarrierPackOperation {
    BuyPack,
    SellPack,
    RestockPack,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2020-03-16T09:25:39Z", "event":"CarrierShipPack", "CarrierID":3700005632, "Operation":"BuyPack",
"PackTheme":"Zorgon Peterson - Cargo", "PackTier":1, "Cost":1668880 })]
pub struct EDLogCarrierShipPack {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub carrier_type: Option<CarrierType>,
    pub operation: CarrierPackOperation,
    pub pack_theme: EDString,
    pub pack_tier: u8,
    pub cost: Option<Credits>,
    pub refunc: Option<Credits>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({"timestamp":"2025-02-13T00:59:09Z","event":"CarrierModulePack","CarrierID":3710967808,"Operation":"SellPack","PackTheme":"Mining Tools","PackTier":3,"Refund":9491076})]
pub struct EDLogCarrierModulePack {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub carrier_type: Option<CarrierType>,
    pub operation: CarrierPackOperation,
    pub pack_theme: EDString,
    pub pack_tier: u8,
    pub cost: Option<Credits>,
    pub refund: Option<Credits>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2020-03-16T14:52:36Z", "event":"CarrierTradeOrder", "CarrierID":3700005632, "BlackMarket":false,
"Commodity":"mineraloil", "Commodity_Localised":"Mineral Oil", "PurchaseOrder":70, "Price":228 })]
pub struct EDLogCarrierTradeOrder {
    #[serde(rename = "CarrierID")]
    carrier_id: u64,
    black_market: bool,
    commodity: EDString, // TODO: could be both MarketItemType or OnFootItem
    #[serde(rename = "Commodity_Localised")]
    commodity_localised: Option<String>,
    cancel_trade: Option<bool>,
    sale_order: Option<u32>,
    purchase_order: Option<Credits>,
    price: Option<Credits>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({"timestamp":"2024-05-05T22:48:37Z","event":"CarrierDockingPermission","CarrierID":3702454272,"DockingAccess":"none","AllowNotorious":true})]
pub struct EDLogCarrierDockingPermission {
    #[serde(rename = "CarrierID")]
    carrier_id: u64,
    docking_access: DockingAccess,
    allow_notorious: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({"timestamp":"2023-11-25T19:24:55Z","event":"CarrierNameChange","CarrierID":3702454272,"Name":"MyFleetCarrier","Callsign":"A1A-A1A"})]
pub struct EDLogCarrierNameChange {
    #[serde(rename = "CarrierID")]
    carrier_id: u64,
    name: EDString,
    callsign: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-09-01T19:09:57Z", "event":"CarrierJumpCancelled", "CarrierType":"FleetCarrier", "CarrierID":3706278912 })]
pub struct EDLogCarrierJumpCancelled {
    #[serde(rename = "CarrierID")]
    carrier_id: u64,
    carrier_type: Option<CarrierType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-08-21T15:02:57Z", "event":"CarrierLocation", "CarrierType":"FleetCarrier", 
    "CarrierID":123456789, "StarSystem":"BD-11 192", "SystemAddress":908486218450, "BodyID":3 })]
pub struct EDLogCarrierLocation {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub carrier_type: Option<CarrierType>,
    pub star_system: EDString,
    pub system_address: u64,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
}

#[test]
fn test_exploration() {
    let json2 = r#"{
    "timestamp": "2024-10-13T19:05:00Z",
    "event": "CarrierStats",
    "CarrierID": 1234567,
    "Callsign": "MFC-MFC",
    "Name": "My First Carrier",
    "DockingAccess": "all",
    "AllowNotorious": false,
    "FuelLevel": 662,
    "JumpRangeCurr": 500.000000,
    "JumpRangeMax": 500.000000,
    "PendingDecommission": false,
    "SpaceUsage": {
        "TotalCapacity": 25000,
        "Crew": 6370,
        "Cargo": 3160,
        "CargoSpaceReserved": 0,
        "ShipPacks": 0,
        "ModulePacks": 0,
        "FreeSpace": 15470
    },
    "Finance": {
        "CarrierBalance": 803361,
        "ReserveBalance": 322176,
        "AvailableBalance": 481185,
        "ReservePercent": 100,
        "TaxRate_pioneersupplies": 0,
        "TaxRate_rearm": 25,
        "TaxRate_refuel": 25,
        "TaxRate_repair": 25
    },
    "Crew": [
        {
            "CrewRole": "BlackMarket",
            "Activated": false
        },
        {
            "CrewRole": "Captain",
            "Activated": true,
            "Enabled": true,
            "CrewName": "Kirk Strickland"
        },
        {
            "CrewRole": "Refuel",
            "Activated": true,
            "Enabled": true,
            "CrewName": "Akemi Cunningham"
        },
        {
            "CrewRole": "Repair",
            "Activated": true,
            "Enabled": true,
            "CrewName": "Guinevere Shepherd"
        },
        {
            "CrewRole": "Rearm",
            "Activated": true,
            "Enabled": true,
            "CrewName": "Marlee Bullock"
        },
        {
            "CrewRole": "Commodities",
            "Activated": true,
            "Enabled": true,
            "CrewName": "Owen Grimes"
        },
        {
            "CrewRole": "VoucherRedemption",
            "Activated": true,
            "Enabled": true,
            "CrewName": "Clementine Chandler"
        },
        {
            "CrewRole": "Exploration",
            "Activated": true,
            "Enabled": true,
            "CrewName": "Medha Frost"
        },
        {
            "CrewRole": "Shipyard",
            "Activated": true,
            "Enabled": false,
            "CrewName": "Chevelle Rivera"
        },
        {
            "CrewRole": "Outfitting",
            "Activated": true,
            "Enabled": false,
            "CrewName": "Drew Gill"
        },
        {
            "CrewRole": "CarrierFuel",
            "Activated": true,
            "Enabled": true,
            "CrewName": "Lauren Adkins"
        },
        {
            "CrewRole": "VistaGenomics",
            "Activated": true,
            "Enabled": true,
            "CrewName": "Ramiro Bentley"
        },
        {
            "CrewRole": "PioneerSupplies",
            "Activated": true,
            "Enabled": true,
            "CrewName": "Jenessa Alford"
        },
        {
            "CrewRole": "Bartender",
            "Activated": true,
            "Enabled": true,
            "CrewName": "Aleeah Bogdani"
        }
    ],
    "ShipPacks": [],
    "ModulePacks": []
}"#;
    let line2: crate::log_line::EDLogLine = serde_json::from_str(json2).expect("Should parse");

    assert!(matches!(
        line2.event(),
        crate::log_line::EDLogEvent::CarrierStats(_)
    ));
    if let crate::log_line::EDLogEvent::CarrierStats(header) = line2.event() {
        assert_eq!(header.name.as_str(), "My First Carrier");
        assert_eq!(header.crew.len(), 14);
        assert_eq!(header.finance.carrier_balance, Credits(803361));
    }
}

#[test]
fn test_new_carrier_stats() {
    let json2 = r#"{
    "timestamp": "2025-08-21T15:15:01Z",
    "event": "CarrierStats",
    "CarrierID": 1234567,
    "CarrierType": "FleetCarrier",
    "Callsign": "MFC-MFC",
    "Name": "My First Carrier",
    "DockingAccess": "all",
    "AllowNotorious": false,
    "FuelLevel": 1000,
    "JumpRangeCurr": 500.000000,
    "JumpRangeMax": 500.000000,
    "PendingDecommission": false,
    "SpaceUsage": {
        "TotalCapacity": 25000,
        "Crew": 6370,
        "Cargo": 6470,
        "CargoSpaceReserved": 0,
        "ShipPacks": 0,
        "ModulePacks": 0,
        "FreeSpace": 12160
    },
    "Finance": {
        "CarrierBalance": 19076712640,
        "ReserveBalance": 19076712640,
        "AvailableBalance": 0,
        "TaxRate_rearm": 25,
        "TaxRate_refuel": 25,
        "TaxRate_repair": 25
    },
    "Crew": [
        {
            "CrewRole": "BlackMarket",
            "Activated": false
        },
        {
            "CrewRole": "Captain",
            "Activated": true,
            "Enabled": true,
            "CrewName": "Kirk Strickland"
        },
        {
            "CrewRole": "Refuel",
            "Activated": true,
            "Enabled": true,
            "CrewName": "Akemi Cunningham"
        },
        {
            "CrewRole": "Repair",
            "Activated": true,
            "Enabled": true,
            "CrewName": "Guinevere Shepherd"
        },
        {
            "CrewRole": "Rearm",
            "Activated": true,
            "Enabled": true,
            "CrewName": "Marlee Bullock"
        },
        {
            "CrewRole": "Commodities",
            "Activated": true,
            "Enabled": true,
            "CrewName": "Owen Grimes"
        },
        {
            "CrewRole": "VoucherRedemption",
            "Activated": true,
            "Enabled": true,
            "CrewName": "Clementine Chandler"
        },
        {
            "CrewRole": "Exploration",
            "Activated": true,
            "Enabled": true,
            "CrewName": "Medha Frost"
        },
        {
            "CrewRole": "Shipyard",
            "Activated": true,
            "Enabled": false,
            "CrewName": "Chevelle Rivera"
        },
        {
            "CrewRole": "Outfitting",
            "Activated": true,
            "Enabled": false,
            "CrewName": "Drew Gill"
        },
        {
            "CrewRole": "CarrierFuel",
            "Activated": true,
            "Enabled": true,
            "CrewName": "Lauren Adkins"
        },
        {
            "CrewRole": "VistaGenomics",
            "Activated": true,
            "Enabled": true,
            "CrewName": "Ramiro Bentley"
        },
        {
            "CrewRole": "PioneerSupplies",
            "Activated": true,
            "Enabled": false,
            "CrewName": "Jenessa Alford"
        },
        {
            "CrewRole": "Bartender",
            "Activated": true,
            "Enabled": false,
            "CrewName": "Aleeah Bogdani"
        }
    ],
    "ShipPacks": [],
    "ModulePacks": []
}"#;
    let line2: crate::log_line::EDLogLine = serde_json::from_str(json2).expect("Should parse");

    assert!(matches!(
        line2.event(),
        crate::log_line::EDLogEvent::CarrierStats(_)
    ));
    if let crate::log_line::EDLogEvent::CarrierStats(header) = line2.event() {
        assert_eq!(header.name.as_str(), "My First Carrier");
        assert_eq!(header.crew.len(), 14);
        assert_eq!(header.finance.carrier_balance, Credits(19076712640));
    }
}
