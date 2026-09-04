use crate::{
    EDString,
    common_types::{
        Allegiance, BodyInformation, BodyType, Conflict, DockingDeniedReason, Faction, FactionName,
        FactionState, GovernmentType, LandingPads, Powers, StarClass, StarPos, StationEconomy,
        StationIdentification, StationInformation, StationService, StationState, SystemEconomy,
        SystemFactionName, SystemSecurity, ThargoidWar,
    },
    navigation::{JumpToStarSystem, JumpType},
    utils::string_or_struct,
};
use ed_parse_log_files_macros::{Extractable, testcase};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-26T19:20:17Z", "event":"Docked", "StationName":"$Operations_Runner_Name:#index=1;", 
    "StationName_Localised":"Frontline Runner", "StationType":"SurfaceStation", "Taxi":false, "Multicrew":false, 
    "StarSystem":"LTT 8584", "SystemAddress":633675387618, "MarketID":127000512, "StationFaction":
    { "Name":"$faction_FrontlineSolutions;", "Name_Localised":"Frontline Solutions" }, 
    "StationGovernment":"$government_Carrier;", "StationGovernment_Localised":"Private Ownership", 
    "StationAllegiance":"FrontlineSolutions", "StationServices":[ "dock", "autodock", "commodities", 
    "rearm", "refuel", "repair", "flightcontroller", "stationoperations", "stationMenu" ], 
    "StationEconomy":"$economy_Carrier;", "StationEconomy_Localised":"Private Enterprise", 
    "StationEconomies":[ { "Name":"$economy_Carrier;", "Name_Localised":"Private Enterprise", "Proportion":1.000000 } ], 
    "DistFromStarLS":5144.069571, "LandingPads":{ "Small":0, "Medium":0, "Large":4 } })]
pub struct EDLogDocked {
    #[serde(flatten)]
    pub station_identification: StationIdentification,
    pub taxi: Option<bool>,
    pub multicrew: Option<bool>,
    pub faction_state: Option<EDString>,
    pub station_state: Option<StationState>,
    pub star_system: EDString,
    pub system_address: u64,
    #[serde(deserialize_with = "string_or_struct")]
    pub station_faction: FactionName,
    pub station_government: EDString,
    #[serde(rename = "StationGovernment_Localised")]
    pub station_government_localised: EDString,
    pub station_allegiance: Option<Allegiance>,
    pub station_services: Vec<StationService>,
    pub station_economy: EDString,
    #[serde(rename = "StationEconomy_Localised")]
    pub station_economy_localised: EDString,
    pub station_economies: Option<Vec<StationEconomy>>,
    #[serde(rename = "DistFromStarLS")]
    pub dist_from_star_ls: f64,
    pub cockpit_breach: Option<bool>,
    pub wanted: Option<bool>,
    pub active_fine: Option<bool>,
    pub landing_pads: Option<LandingPads>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-07-17T18:56:25Z", "event":"DockingCancelled", "MarketID":127000000, 
    "StationName":"$Operations_Megaship_Massacre_Megaship_Reclaim_name:#index=1;", 
    "StationName_Localised":"Cargo Vessel", "StationType":"SurfaceStation" })]
pub struct EDLogDockingCancelled {
    #[serde(flatten)]
    pub station_identification: StationIdentification,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-06-30T12:38:07Z", "event":"DockingDenied", "Reason":"DockOffline", 
    "MarketID":3906562304, "StationName":"Joshi Military Complex", "StationType":"OnFootSettlement" })]
pub struct EDLogDockingDenied {
    reason: DockingDeniedReason,
    #[serde(flatten)]
    pub station_identification: StationIdentification,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-09T13:15:24Z", "event":"DockingGranted", "LandingPad":1, "MarketID":127000512, 
    "StationName":"$Operations_Runner_Name:#index=1;", "StationName_Localised":"Frontline Runner", "StationType":"SurfaceStation" })]
pub struct EDLogDockingGranted {
    pub landing_pad: u64,
    #[serde(flatten)]
    pub station_identification: StationIdentification,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-26T19:19:10Z", "event":"DockingRequested", "MarketID":127000512, 
    "StationName":"$Operations_Runner_Name:#index=1;", "StationName_Localised":"Frontline Runner", "StationType":"SurfaceStation", 
    "LandingPads":{ "Small":0, "Medium":0, "Large":4 } })]
pub struct EDLogDockingRequested {
    #[serde(flatten)]
    pub station_identification: StationIdentification,
    pub landing_pads: Option<LandingPads>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2024-02-14T20:20:50Z", "event":"DockingTimeout", "MarketID":3706278912, 
"StationName":"T6N-N7N", "StationType":"FleetCarrier" })]
pub struct EDLogDockingTimeout {
    #[serde(flatten)]
    pub station_identification: StationIdentification,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
// todo: refactor with location->edloglocation
pub struct EDLogFSDJump {
    pub taxi: Option<bool>,
    pub multicrew: Option<bool>,
    pub star_system: EDString,
    pub system_address: u64,
    pub star_pos: StarPos,
    pub system_allegiance: Allegiance,
    pub system_economy: SystemEconomy,
    #[serde(rename = "SystemEconomy_Localised")]
    pub system_economy_localised: EDString,
    pub system_second_economy: Option<SystemEconomy>,
    #[serde(rename = "SystemSecondEconomy_Localised")]
    pub system_second_economy_localised: Option<EDString>,
    pub system_government: GovernmentType,
    #[serde(rename = "SystemGovernment_Localised")]
    pub system_government_localised: EDString,
    pub system_security: SystemSecurity,
    #[serde(rename = "SystemSecurity_Localised")]
    pub system_security_localised: EDString,
    pub population: u64,
    pub wanted: Option<bool>,
    pub body: Option<EDString>,
    #[serde(rename = "BodyID")]
    pub body_id: Option<u64>,
    pub body_type: Option<BodyType>,
    #[serde(flatten)]
    pub powerplay: Option<Powers>,
    pub thargoid_war: Option<ThargoidWar>,
    pub jump_dist: f64,
    pub fuel_used: f64,
    pub fuel_level: f64,
    // whether FSD boost was used. Number seems to indicate multiplier, i.e. 4x for neutron star
    pub boost_used: Option<u64>,
    pub factions: Option<Vec<Faction>>,
    #[serde(flatten)]
    pub system_faction_name: Option<SystemFactionName>,
    pub faction_state: Option<FactionState>,
    pub conflicts: Option<Vec<Conflict>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-26T19:26:17Z", "event":"FSDTarget", "Name":"Eos Bre PD-A c14-383", 
    "SystemAddress":105358601333370, "StarClass":"K", "RemainingJumpsInRoute":23 })]
pub struct EDLogFSDTarget {
    pub name: EDString,
    pub system_address: u64,
    pub star_class: StarClass,
    pub remaining_jumps_in_route: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2022-09-15T20:16:36Z", "event":"Liftoff", "PlayerControlled":true, "Latitude":36.163647, "Longitude":-128.089386, 
    "NearestDestination":"$SAA_Unknown_Signal:#type=$SAA_SignalType_Geological;:#index=9;", "NearestDestination_Localised":"Geological Signal (9)" })]
pub struct EDLogLiftoff {
    // false if ship dismissed when player is in SRV, true if player is taking off
    player_controlled: bool,
    taxi: Option<bool>,
    multicrew: Option<bool>,
    #[serde(flatten)]
    start_system_info: Option<BodyInformation>,
    on_station: Option<bool>,
    on_planet: Option<bool>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    nearest_destination: Option<EDString>,
    #[serde(rename = "NearestDestination_Localised")]
    nearest_destination_localised: Option<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogLocation {
    #[serde(rename = "DistFromStarLS")]
    pub dist_from_star_ls: Option<f64>,
    pub docked: bool,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    #[serde(flatten)]
    pub station_information: Option<StationInformation>,
    pub taxi: Option<bool>,
    pub multicrew: Option<bool>,
    #[serde(rename = "InSRV")]
    pub in_srv: Option<bool>,
    pub on_foot: Option<bool>,
    pub star_system: EDString,
    pub system_address: Option<u64>,
    pub star_pos: StarPos,
    pub system_allegiance: Allegiance,
    pub system_economy: SystemEconomy,
    #[serde(rename = "SystemEconomy_Localised")]
    pub system_economy_localised: EDString,
    pub system_second_economy: Option<SystemEconomy>,
    #[serde(rename = "SystemSecondEconomy_Localised")]
    pub system_second_economy_localised: Option<EDString>,
    pub system_government: GovernmentType,
    #[serde(rename = "SystemGovernment_Localised")]
    pub system_government_localised: EDString,
    pub system_security: SystemSecurity,
    #[serde(rename = "SystemSecurity_Localised")]
    pub system_security_localised: EDString,
    pub population: u64,
    pub body: EDString,
    #[serde(rename = "BodyID")]
    pub body_id: Option<u64>,
    pub body_type: BodyType,
    #[serde(flatten)]
    pub powers: Option<Powers>,
    pub thargoid_war: Option<ThargoidWar>,
    pub factions: Option<Vec<Faction>>,
    #[serde(flatten)]
    pub system_faction_name: Option<SystemFactionName>,
    pub conflicts: Option<Vec<Conflict>>,
    pub faction_state: Option<FactionState>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-26T18:50:35Z", "event":"StartJump", "JumpType":"Supercruise", "Taxi":false })]
#[testcase({ "timestamp":"2026-08-26T19:26:10Z", "event":"StartJump", "JumpType":"Hyperspace", "Taxi":false, 
    "StarSystem":"Eos Bre UX-U d2-7731", "SystemAddress":265645245617435, "StarClass":"N" })]
pub struct EDLogStartJump {
    jump_type: JumpType,
    taxi: Option<bool>,
    #[serde(flatten)]
    star_system: Option<JumpToStarSystem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-26T18:50:48Z", "event":"SupercruiseExit", "Taxi":false, "Multicrew":false, 
    "StarSystem":"LTT 8584", "SystemAddress":633675387618, "Body":"LTT 8584 7", "BodyID":76, "BodyType":"Planet" })]
pub struct EDLogSupercruiseExit {
    pub taxi: Option<bool>,
    pub multicrew: Option<bool>,
    pub star_system: EDString,
    pub system_address: Option<u64>,
    pub body: EDString,
    #[serde(rename = "BodyID")]
    pub body_id: Option<u64>,
    pub body_type: BodyType,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-26T19:25:48Z", "event":"SupercruiseEntry", "Taxi":false, "Multicrew":false, 
    "StarSystem":"Eos Bre BR-E b12-0", "SystemAddress":647332648041 })]
pub struct EDLogSupercruiseEntry {
    pub taxi: Option<bool>,
    pub multicrew: Option<bool>,
    pub star_system: EDString,
    pub system_address: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-25T18:43:52Z", "event":"Touchdown", "PlayerControlled":true, "Taxi":false, "Multicrew":false, 
    "StarSystem":"Eok Gree FG-Y g1326", "SystemAddress":89006142126, "Body":"Eok Gree FG-Y g1326 ABCD 10 a", "BodyID":23, 
    "OnStation":false, "OnPlanet":true, "Latitude":-24.967541, "Longitude":121.667747 })]
pub struct EDLogTouchdown {
    pub player_controlled: bool,
    pub taxi: Option<bool>,
    pub multicrew: Option<bool>,
    #[serde(flatten)]
    pub body_information: Option<BodyInformation>,
    pub on_station: Option<bool>,
    pub on_planet: Option<bool>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub nearest_destination: Option<EDString>,
    #[serde(rename = "NearestDestination_Localised")]
    pub nearest_destination_localised: Option<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-10-17T01:49:26Z", "event":"Undocked", "StationName":"Verrazzano's Inheritance", "StationType":"SurfaceStation" })]
pub struct EDLogUndocked {
    #[serde(flatten)]
    pub station_identification: StationIdentification,
    taxi: Option<bool>,
    multicrew: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-26T19:24:55Z", "event":"NavRoute" })]
#[testcase({ "timestamp":"2026-08-26T19:24:55Z", "event":"NavRoute", "Route":[ 
    { "StarSystem":"Eos Bre BR-E b12-0", "SystemAddress":647332648041, "StarPos":[-1764.87500,137.56250,29926.75000], "StarClass":"M" },
    { "StarSystem":"Eos Bre UX-U d2-7731", "SystemAddress":265645245617435, "StarPos":[-1815.43750,127.34375,29949.84375], "StarClass":"N" } 
    ]})]
pub struct EDLogNavRoute {
    route: Option<Vec<JumpToStarSystem>>,
}

#[test]
fn test_location() {
    let json = r#"{ "timestamp":"2025-03-20T19:54:09Z", "event":"Location", "DistFromStarLS":2700.571292, "Docked":false, 
        "Taxi":false, "Multicrew":false, "StarSystem":"Kholul", "SystemAddress":2415659059547, "StarPos":[-46.93750,-7.84375,-152.18750], 
        "SystemAllegiance":"Independent", "SystemEconomy":"$economy_Colony;", "SystemEconomy_Localised":"Colony", 
        "SystemSecondEconomy":"$economy_Terraforming;", "SystemSecondEconomy_Localised":"Terraforming", "SystemGovernment":"$government_Corporate;", 
        "SystemGovernment_Localised":"Corporate", "SystemSecurity":"$SYSTEM_SECURITY_medium;", "SystemSecurity_Localised":"Medium Security", 
        "Population":178961, "Body":"Kholul 9", "BodyID":37, "BodyType":"Planet", 
        "Powers":[ "Pranav Antal", "Jerome Archer" ], "PowerplayState":"Unoccupied", "PowerplayConflictProgress":[ 
            { "Power":"Pranav Antal", "ConflictProgress":0.005875 }, 
            { "Power":"Jerome Archer", "ConflictProgress":0.478375 } ], 
        "Factions":[ 
            { "Name":"Kholul Parish", "FactionState":"None", "Government":"Theocracy", "Influence":0.080597, "Allegiance":"Independent", 
              "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", "MyReputation":15.000000, 
              "PendingStates":[ { "State":"CivilWar", "Trend":0 } ] }, 
            { "Name":"Kholul Purple United Int", "FactionState":"None", "Government":"Corporate", "Influence":0.148259, "Allegiance":"Independent", 
              "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", "MyReputation":15.000000 }, 
            { "Name":"Kholul Blue Rats", "FactionState":"Bust", "Government":"Anarchy", "Influence":0.009950, "Allegiance":"Independent", 
              "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", "MyReputation":0.000000, "ActiveStates":[ { "State":"Bust" } ] }, 
            { "Name":"Sirius Inc", "FactionState":"None", "Government":"Democracy", "Influence":0.134328, "Allegiance":"Independent", 
               "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", "MyReputation":92.936203, 
               "PendingStates":[ { "State":"Expansion", "Trend":0 } ] } 
        ], "SystemFaction":{ "Name":"SI Terraforming" }, 
        "Conflicts":[ 
            { "WarType":"civilwar", "Status":"pending", "Faction1":{ "Name":"Kholul Parish", "Stake":"", "WonDays":0 }, 
              "Faction2":{ "Name":"Kholul Movement", "Stake":"McDevitt Laboratory", "WonDays":0 } } 
        ] }"#;
    let line: crate::log_line::EDLogLine = serde_json::from_str(json).expect("Should parse");

    assert!(matches!(
        line.event(),
        crate::log_line::EDLogEvent::Location(_)
    ));
    if let crate::log_line::EDLogEvent::Location(header) = line.event() {
        assert_eq!(header.population, 178961);
        assert_eq!(header.factions.as_ref().unwrap().len(), 4);
        assert_eq!(header.powers.as_ref().unwrap().powers.len(), 2);
    }
}
