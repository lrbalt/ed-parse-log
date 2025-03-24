use crate::common_types::{
    Allegiance, BodyType, Conflict, Faction, FactionName, Powers, StationInformation, ThargoidWar,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Trend {
    trend: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogLocation {
    #[serde(rename = "DistFromStarLS")]
    dist_from_star_ls: Option<f64>,
    docked: bool,
    on_foot: Option<bool>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    #[serde(flatten)]
    station_info: Option<StationInformation>,
    taxi: Option<bool>,
    multicrew: Option<bool>,
    #[serde(rename = "InSRV")]
    in_srv: Option<bool>,
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
    #[serde(flatten)]
    powers: Option<Powers>,
    thargoid_war: Option<ThargoidWar>,
    factions: Option<Vec<Faction>>,
    system_faction: Option<FactionName>,
    conflicts: Option<Vec<Conflict>>,
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
    }
}
