use crate::{
    common_types::{Allegiance, BodyType, Credits, FactionName, PowerplayState, StationInformation},
    location::{Conflict, Faction, ThargoidWar},
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
#[serde(rename_all = "PascalCase")]
pub struct CrewMember {
    crew_role: String,
    activated: bool,
    activated_props: Option<ActivatedProps>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ActivatedProps {
    enabled: bool,
    crew_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Finance {
    pub carrier_balance: Credits,
    pub reserve_balance: Credits,
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
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
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
#[serde(rename_all = "PascalCase")]
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
#[serde(rename_all = "PascalCase")]
pub struct EDLogFCMaterials {
    #[serde(rename = "MarketID")]
    market_id: u64,
    carrier_name: String,
    #[serde(rename = "CarrierID")]
    carrier_id: String,
}
