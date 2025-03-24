use crate::common_types::{
    Allegiance, BodyType, Conflict, Credits, Faction, FactionName, PowerplayState,
    StationInformation, ThargoidWar,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCarrierFinance {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCarrierTradeOrder {
    #[serde(rename = "CarrierID")]
    carrier_id: u64,
    black_market: bool,
    commodity: String,
    #[serde(rename = "Commodity_Localised")]
    commodity_localised: Option<String>,
    cancel_trade: Option<bool>,
    sale_order: Option<u32>,
    price: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCarrierBankTransfer {
    #[serde(rename = "CarrierID")]
    carrier_id: u64,
    deposit: u64,
    player_balance: u64,
    carrier_balance: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCarrierJumpRequest {
    #[serde(rename = "CarrierID")]
    carrier_id: u64,
    system_name: String,
    body: Option<String>,
    system_address: u64,
    #[serde(rename = "BodyID")]
    body_id: u64,
    departure_time: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCarrierJump {
    docked: bool,
    on_foot: Option<bool>,
    #[serde(flatten)]
    station_information: Option<StationInformation>,
    taxi: Option<bool>,
    multicrew: Option<bool>,
    // TODO: location has overlapping fields
    star_system: String,
    system_address: u64,
    star_pos: [f64; 3],
    system_allegiance: Allegiance,
    system_economy: String,
    #[serde(rename = "SystemEconomy_Localised")]
    system_economy_localised: String,
    system_second_economy: String,
    #[serde(rename = "SystemSecondEconomy_Localised")]
    system_second_economy_localised: String,
    system_government: String,
    #[serde(rename = "SystemGovernment_Localised")]
    system_government_localised: String,
    system_security: String,
    #[serde(rename = "SystemSecurity_Localised")]
    system_security_localised: String,
    population: u64,
    body: String,
    #[serde(rename = "BodyID")]
    body_id: u64,
    body_type: BodyType,
    powers: Option<Vec<String>>,
    powerplay_state: Option<PowerplayState>,
    thargoid_war: Option<ThargoidWar>,
    factions: Option<Vec<Faction>>,
    system_faction: Option<FactionName>,
    conflicts: Option<Vec<Conflict>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCarrierJumpCancelled {
    #[serde(rename = "CarrierID")]
    carrier_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CrewServiceOperation {
    Activate,
    Pause,
    Resume,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCarrierCrewServices {
    #[serde(rename = "CarrierID")]
    carrier_id: u64,
    crew_role: CrewRole,
    operation: CrewServiceOperation,
    crew_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCarrierBuy {
    #[serde(rename = "CarrierID")]
    carrier_id: u64,
    bought_at_market: u64,
    location: String,
    system_address: u64,
    price: u64,
    variant: String,
    callsign: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCarrierLocation {
    #[serde(rename = "CarrierID")]
    carrier_id: u64,
    star_system: String,
    system_address: u64,
    #[serde(rename = "BodyID")]
    body_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCarrierNameChange {
    #[serde(rename = "CarrierID")]
    carrier_id: u64,
    name: String,
    callsign: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum ShipPack {}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum ModulePack {}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CrewMember {
    crew_name: Option<String>,
    crew_role: String,
    activated: bool,
    enabled: Option<bool>,
    activated_props: Option<ActivatedProps>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ActivatedProps {
    enabled: bool,
    crew_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Finance {
    pub carrier_balance: Credits,
    pub reserve_balance: Credits,
    pub reserve_percent: Option<f64>,
    pub available_balance: Credits,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCarrierStats {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub callsign: String,
    pub name: String,
    pub docking_access: String,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogFCMaterials {
    #[serde(rename = "MarketID")]
    market_id: u64,
    carrier_name: String,
    #[serde(rename = "CarrierID")]
    carrier_id: String,
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
        assert_eq!(&header.name, "My First Carrier");
        assert_eq!(header.crew.len(), 14);
        assert_eq!(header.finance.carrier_balance, Credits(803361));
    }
}
