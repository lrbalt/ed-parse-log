use crate::{
    common_types::{
        Allegiance, BodyInformation, BodyType, Conflict, Faction, FactionState, Powers, StarClass, StationInformation, ThargoidWar
    }, location::SystemFactionName, log_line::{EDLogEvent, Extractable}
};
use ed_parse_log_file_testcase::testcase;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogFuelScoop {
    scooped: f64,
    total: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogLiftoff {
    player_controlled: bool,
    taxi: Option<bool>,
    multicrew: Option<bool>,
    #[serde(flatten)]
    start_system_info: Option<BodyInformation>,
    on_station: Option<bool>,
    on_planet: Option<bool>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    nearest_destination: Option<String>,
    #[serde(rename = "NearestDestination_Localised")]
    nearest_destination_localised: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
// todo: refactor with BodyInformation in common types
pub struct BodyInformationOfSettlement {
    pub system_address: u64,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
    pub body_name: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[testcase({ "timestamp":"2023-03-01T15:10:23Z", "event":"ApproachSettlement", "Name":"Nahavandi Penal colony", 
            "MarketID":3790770944, "SystemAddress":7266413782417, "BodyID":9, "BodyName":"Luggerates A 3", 
            "Latitude":62.048309, "Longitude":80.228821 })]
#[testcase({ "timestamp":"2024-05-30T16:20:52Z", "event":"ApproachSettlement", 
        "Name":"$Ancient_Small_002:#index=1;", "Name_Localised":"Guardian Structure", "SystemAddress":2833906537146, 
        "BodyID":7, "BodyName":"Synuefe EU-Q c21-10 A 3", "Latitude":19.823612, "Longitude":-82.460922 })]
#[testcase({ "timestamp":"2017-10-17T01:41:51Z", "event":"ApproachSettlement", "Name":"Verrazzano's Inheritance" })]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogApproachSettlement {
    pub name: String,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,
    #[serde(flatten)]
    pub station_information: Option<StationInformation>,
    #[serde(flatten)]
    pub body_information: Option<BodyInformationOfSettlement>,
}

impl Extractable for EDLogApproachSettlement {
    fn extract(event: &EDLogEvent) -> Option<&Self> {
        if let EDLogEvent::ApproachSettlement(loc) = event {
            return Some(loc);
        }
        None
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogFSDTarget {
    name: String,
    system_address: u64,
    star_class: StarClass,
    remaining_jumps_in_route: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
// todo: refactor with location->edloglocation
pub struct EDLogFSDJump {
    pub taxi: Option<bool>,
    pub multicrew: Option<bool>,
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
    pub body: Option<String>,
    #[serde(rename = "BodyID")]
    pub body_id: Option<u64>,
    pub body_type: Option<BodyType>,
    #[serde(flatten)]
    pub powerplay: Option<Powers>,
    pub thargoid_war: Option<ThargoidWar>,
    pub jump_dist: f64,
    pub fuel_used: f64,
    pub fuel_level: f64,
    pub boost_used: Option<u64>,
    pub factions: Option<Vec<Faction>>,
    #[serde(flatten)]
    pub system_faction_name: Option<SystemFactionName>,
    pub faction_state: Option<FactionState>,
    pub conflicts: Option<Vec<Conflict>>,
}

impl Extractable for EDLogFSDJump {
    fn extract(event: &EDLogEvent) -> Option<&Self> {
        if let EDLogEvent::FSDJump(loc) = event {
            return Some(loc);
        }
        None
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDockSRV {
    #[serde(rename = "SRVType")]
    srvtype: Option<String>,
    #[serde(rename = "SRVType_Localised")]
    srvtype_localised: Option<String>,
    #[serde(rename = "ID")]
    id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogLaunchSRV {
    #[serde(rename = "SRVType")]
    srvtype: Option<String>,
    #[serde(rename = "SRVType_Localised")]
    srvtype_localised: Option<String>,
    loadout: String,
    #[serde(rename = "ID")]
    id: u64,
    player_controlled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSRVDestroyed {
    #[serde(rename = "ID")]
    id: u64,
    #[serde(rename = "SRVType")]
    srv_type: String,
    #[serde(rename = "SRVType_Localised")]
    srv_type_localised: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogTouchdown {
    player_controlled: bool,
    taxi: Option<bool>,
    multicrew: Option<bool>,
    #[serde(flatten)]
    start_system_info: Option<BodyInformation>,
    on_station: Option<bool>,
    on_planet: Option<bool>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    nearest_destination: Option<String>,
    #[serde(rename = "NearestDestination_Localised")]
    nearest_destination_localised: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum JumpType {
    Hyperspace,
    Supercruise,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct JumpToStarsystem {
    star_system: String,
    system_address: u64,
    star_class: StarClass,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogStartJump {
    jump_type: JumpType,
    taxi: Option<bool>,
    #[serde(flatten)]
    star_system: Option<JumpToStarsystem>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogJetConeBoost {
    boost_value: f64,
}

#[test]
fn test_jsd_jump() {
    let json = r#"{ "timestamp":"2024-10-13T17:51:23Z", "event":"FSDJump", "Taxi":false, "Multicrew":false, 
    "StarSystem":"Yukoukha", "SystemAddress":22945325796176, "StarPos":[-134.56250,-34.03125,-4.75000], 
    "SystemAllegiance":"Independent", "SystemEconomy":"$economy_Military;", "SystemEconomy_Localised":"Military", 
    "SystemSecondEconomy":"$economy_Agri;", "SystemSecondEconomy_Localised":"Agriculture", 
    "SystemGovernment":"$government_Corporate;", "SystemGovernment_Localised":"Corporate", 
    "SystemSecurity":"$SYSTEM_SECURITY_low;", "SystemSecurity_Localised":"Low Security", "Population":33893, 
    "Body":"Yukoukha", "BodyID":0, "BodyType":"Star", "JumpDist":13.790, "FuelUsed":0.371038, "FuelLevel":5.628962, 
    "Factions":[ 
        { "Name":"People's Madjandji Resistance", "FactionState":"None", "Government":"Democracy", "Influence":0.063555,
          "Allegiance":"Federation", "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", "MyReputation":0.000000 }, 
        { "Name":"Yukoukha Partnership", "FactionState":"None", "Government":"Anarchy", "Influence":0.009930, 
          "Allegiance":"Independent", "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", "MyReputation":0.000000 }, 
        { "Name":"Yukoukha Coordinated", "FactionState":"None", "Government":"Cooperative", "Influence":0.050645,
          "Allegiance":"Independent", "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", "MyReputation":0.000000 }, 
        { "Name":"Yukoukha Life Industries", "FactionState":"None", "Government":"Corporate", "Influence":0.009930,
          "Allegiance":"Independent", "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", "MyReputation":0.000000 }, 
        { "Name":"DaVinci Corp.", "FactionState":"Blight", "Government":"Corporate", "Influence":0.599801, "Allegiance":"Independent",
          "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", "MyReputation":61.321701, 
          "ActiveStates":[ { "State":"Blight" } ] }, 
        { "Name":"Phoenix Flight Explorers Commune", "FactionState":"None", "Government":"Cooperative",
          "Influence":0.266137, "Allegiance":"Independent", "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", 
          "MyReputation":86.373100 } 
      ], "SystemFaction":{ "Name":"DaVinci Corp.", "FactionState":"Blight" } }"#;
    assert!(serde_json::from_str::<crate::log_line::EDLogLine>(json).is_ok());

    let json = r#"{ "timestamp":"2024-08-25T17:27:53Z", "event":"FSDJump", "Taxi":false, "Multicrew":false, "StarSystem":"Dyava", 
        "SystemAddress":3107710833346, "StarPos":[132.71875,-18.09375,-101.56250], "SystemAllegiance":"Federation", 
        "SystemEconomy":"$economy_Industrial;", "SystemEconomy_Localised":"Industrial", "SystemSecondEconomy":"$economy_Colony;", 
        "SystemSecondEconomy_Localised":"Colony", "SystemGovernment":"$government_Corporate;", "SystemGovernment_Localised":"Corporate",
        "SystemSecurity":"$SYSTEM_SECURITY_high;", "SystemSecurity_Localised":"High Security", "Population":7671535, "Body":"Dyava A", 
        "BodyID":2, "BodyType":"Star", "JumpDist":24.026, "FuelUsed":0.357380, "FuelLevel":31.642620, "Factions":[ 
        ], "SystemFaction":{ "Name":"Turkish Federal Forces" } ,
        "Conflicts": [
            {"WarType": "election","Status": "active",
            "Faction1": { "Name": "Social Mombilar Revolutionary Party","Stake": "Yellow Comet Incorporated","WonDays": 1},
            "Faction2": {"Name": "Jandaquiya Shared","Stake": "Naddoddur Landing","WonDays": 3}
        }
    ]}"#;
    assert!(serde_json::from_str::<crate::log_line::EDLogLine>(json).is_ok());

    let json = r#"{ 
        "timestamp":"2024-01-01T09:28:15Z", "event":"FSDJump", "Taxi":false, "Multicrew":false, 
        "StarSystem":"BD-11 1025", "SystemAddress":1733187048130, "StarPos":[52.87500,-58.31250,-89.87500], 
        "SystemAllegiance":"", "SystemEconomy":"$economy_None;", "SystemEconomy_Localised":"None", 
        "SystemSecondEconomy":"$economy_None;", "SystemSecondEconomy_Localised":"None", 
        "SystemGovernment":"$government_None;", "SystemGovernment_Localised":"None", 
        "SystemSecurity":"$GAlAXY_MAP_INFO_state_anarchy;", "SystemSecurity_Localised":"Anarchy", 
        "Population":0, "Body":"BD-11 1025 A", "BodyID":1, "BodyType":"Star", "JumpDist":35.197, 
        "Powers": [ "Zachary Hudson"], "PowerplayState": "Exploited","ControllingPower": "Jerome Archer",
        "ThargoidWar": {
            "CurrentState": "Thargoid_Harvest","NextStateSuccess": "Thargoid_Recovery","NextStateFailure": "Thargoid_Controlled",
            "SuccessStateReached": false,"WarProgress": 0.361584,"RemainingPorts": 2,"EstimatedRemainingTime": "19 Days"
        },
        "FuelUsed":7.178949, "FuelLevel":19.462725 }"#;
    let line: crate::log_line::EDLogLine = serde_json::from_str(json).expect("Should parse");

    assert!(matches!(
        line.event(),
        crate::log_line::EDLogEvent::FSDJump(_)
    ));
    if let EDLogEvent::FSDJump(header) = line.event() {
        assert_eq!(header.taxi, Some(false));
        assert_eq!(header.body_id, Some(1));
    }
}
