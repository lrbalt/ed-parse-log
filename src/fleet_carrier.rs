use crate::{
    EDString,
    common_types::{
        Allegiance, BodyType, Conflict, Credits, Faction, FactionName, PowerplayConflictProgress,
        PowerplayState, StationInformation, ThargoidWar,
    },
};
use ed_parse_log_files_macros::{Extractable, testcase};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCarrierTradeOrder {
    #[serde(rename = "CarrierID")]
    carrier_id: u64,
    black_market: bool,
    commodity: EDString, // check on MarketItemType versus OnFootItem
    #[serde(rename = "Commodity_Localised")]
    commodity_localised: Option<String>,
    cancel_trade: Option<bool>,
    sale_order: Option<u32>,
    purchase_order: Option<Credits>,
    price: Option<Credits>,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CarrierType {
    FleetCarrier,
    SquadronCarrier,
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
    body: Option<EDString>,
    system_address: u64,
    #[serde(rename = "BodyID")]
    body_id: u64,
    departure_time: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCarrierJump {
    docked: bool,
    on_foot: Option<bool>,
    #[serde(flatten)]
    station_information: Option<StationInformation>,
    taxi: Option<bool>,
    multicrew: Option<bool>,
    // TODO: location has overlapping fields
    star_system: EDString,
    system_address: u64,
    star_pos: [f64; 3],
    system_allegiance: Allegiance,
    system_economy: EDString,
    #[serde(rename = "SystemEconomy_Localised")]
    system_economy_localised: EDString,
    system_second_economy: EDString,
    #[serde(rename = "SystemSecondEconomy_Localised")]
    system_second_economy_localised: EDString,
    system_government: EDString,
    #[serde(rename = "SystemGovernment_Localised")]
    system_government_localised: EDString,
    system_security: EDString,
    #[serde(rename = "SystemSecurity_Localised")]
    system_security_localised: EDString,
    population: u64,
    body: EDString,
    #[serde(rename = "BodyID")]
    body_id: u64,
    body_type: BodyType,
    controlling_power: Option<EDString>,
    powers: Option<Vec<EDString>>, // TODO: use Powers struct here
    powerplay_state: Option<PowerplayState>,
    powerplay_state_control_progress: Option<f64>,
    powerplay_state_reinforcement: Option<u64>,
    powerplay_state_undermining: Option<u64>,
    powerplay_conflict_progress: Option<Vec<PowerplayConflictProgress>>,
    thargoid_war: Option<ThargoidWar>,
    factions: Option<Vec<Faction>>,
    system_faction: Option<FactionName>,
    conflicts: Option<Vec<Conflict>>,
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
pub struct EDLogCarrierDepositFuel {
    #[serde(rename = "CarrierID")]
    carrier_id: u64,
    amount: u64,
    total: u64,
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
pub enum CrewServiceOperation {
    Activate,
    Pause,
    Resume,
    Deactivate,
    Replace,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-08-20T13:11:21Z", "event":"CarrierCrewServices", "CarrierID":3706278912, "CarrierType":"FleetCarrier", "CrewRole":"Bartender", "Operation":"Pause", "CrewName":"Aleeah Bogdani" })]
pub struct EDLogCarrierCrewServices {
    #[serde(rename = "CarrierID")]
    carrier_id: u64,
    carrier_type: Option<CarrierType>,
    crew_role: CrewRole,
    operation: CrewServiceOperation,
    crew_name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CarrierVariant {
    CarrierDockB,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2024-01-08T12:36:19Z", "event":"CarrierBuy", "CarrierID":12341234, "BoughtAtMarket":3223259392, "Location":"Mitnahas", "SystemAddress":7267218695553, "Price":5000000000, "Variant":"CarrierDockB", "Callsign":"A1A-A1A" })]
pub struct EDLogCarrierBuy {
    #[serde(rename = "CarrierID")]
    carrier_id: u64,
    bought_at_market: u64,
    location: EDString,
    system_address: u64,
    price: Credits,
    variant: CarrierVariant,
    callsign: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCarrierModulePack {
    #[serde(rename = "CarrierID")]
    carrier_id: u64,
    operation: EDString,
    pack_theme: EDString,
    pack_tier: u64,
    cost: Option<Credits>,
    refund: Option<Credits>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCarrierDockingPermission {
    #[serde(rename = "CarrierID")]
    carrier_id: u64,
    docking_access: EDString,
    allow_notorious: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-08-21T15:02:57Z", "event":"CarrierLocation", "CarrierType":"FleetCarrier", "CarrierID":123456789, "StarSystem":"BD-11 192", "SystemAddress":908486218450, "BodyID":3 })]
pub struct EDLogCarrierLocation {
    #[serde(rename = "CarrierID")]
    carrier_id: u64,
    carrier_type: Option<CarrierType>,
    star_system: EDString,
    system_address: u64,
    #[serde(rename = "BodyID")]
    body_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCarrierNameChange {
    #[serde(rename = "CarrierID")]
    carrier_id: u64,
    name: EDString,
    callsign: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum ShipPack {}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ModulePack {
    pack_theme: EDString,
    pack_tier: u64,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ActivatedProps {
    enabled: bool,
    crew_name: EDString,
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
pub struct SpaceUsage {
    total_capacity: u64,
    crew: u64,
    cargo: u64,
    cargo_space_reserved: u64,
    ship_packs: u64,
    module_packs: u64,
    free_space: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCarrierStats {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub carrier_type: Option<CarrierType>,
    pub callsign: EDString,
    pub name: EDString,
    pub docking_access: EDString,
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
pub struct EDLogFCMaterials {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub carrier_name: EDString,
    #[serde(rename = "CarrierID")]
    pub carrier_id: EDString,
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
