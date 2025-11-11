use crate::{
    common_types::{
        Allegiance, BodyType, Conflict, Faction, FactionName, FactionState, Powers,
        StationInformation, ThargoidWar,
    },
    log_line::EDLogEvent,
    utils::string_or_struct,
};
use ed_parse_log_files_macros::Extractable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Trend {
    trend: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SystemFactionName {
    #[serde(deserialize_with = "string_or_struct")]
    pub system_faction: FactionName,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogLocation {
    #[serde(rename = "DistFromStarLS")]
    pub dist_from_star_ls: Option<f64>,
    pub docked: bool,
    pub on_foot: Option<bool>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    #[serde(flatten)]
    pub station_information: Option<StationInformation>,
    pub taxi: Option<bool>,
    pub multicrew: Option<bool>,
    #[serde(rename = "InSRV")]
    pub in_srv: Option<bool>,
    pub star_system: String,
    pub system_address: Option<u64>,
    pub star_pos: [f64; 3],
    pub system_allegiance: Allegiance,
    pub system_economy: String,
    #[serde(rename = "SystemEconomy_Localised")]
    pub system_economy_localised: String,
    pub system_second_economy: Option<String>,
    #[serde(rename = "SystemSecondEconomy_Localised")]
    pub system_second_economy_localised: Option<String>,
    pub system_government: String,
    #[serde(rename = "SystemGovernment_Localised")]
    pub system_government_localised: String,
    pub system_security: String,
    #[serde(rename = "SystemSecurity_Localised")]
    pub system_security_localised: String,
    pub population: u64,
    pub body: String,
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
    if let EDLogEvent::Location(header) = line.event() {
        assert_eq!(header.population, 178961);
    }
}
