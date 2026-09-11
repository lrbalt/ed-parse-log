use crate::{
    EDString,
    common_types::{
        CQCRank, CombatRank, Credits, EmpireRank, ExobiologistRank, ExploreRank, FederationRank,
        FuelCapacity, Merits, Power, SoldierRank, TradeRank,
    },
    log_line::{GameMode, LoadGameShip},
    material::{EncodedMaterial, ManufacturedMaterial, RawMaterial},
    modules::Module,
    ship::Inventory,
    ship_type::ShipType,
    station_services::Mission,
    statistics::{
        StatisticSearchAndRescue, StatisticsBankAccount, StatisticsCQCStats, StatisticsCombat,
        StatisticsCrafting, StatisticsCrew, StatisticsCrime, StatisticsExobiology,
        StatisticsExploration, StatisticsFleetcarrier, StatisticsMaterialTraderStats,
        StatisticsMining, StatisticsMulticrew, StatisticsPassengers, StatisticsSmuggling,
        StatisticsSquadron, StatisticsTgEncounters, StatisticsTrading,
    },
    utils::duration_as_secs,
};
use chrono::Duration;
use ed_parse_log_files_macros::{Extractable, testcase, testcase_struct};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-10-14T18:41:37Z", "event":"Cargo", "Inventory":[  ] })]
pub struct EDLogCargo {
    pub vessel: Option<EDString>,
    pub count: Option<u64>,
    pub inventory: Option<Vec<Inventory>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(deny_unknown_fields)]
#[testcase({ "timestamp":"2016-06-10T14:32:03Z", "event":"ClearSavedGame", "Name":"HRC1", "FID":"F44396" })]
pub struct EDLogClearSavedGame {
    #[serde(rename = "FID")]
    pub fid: EDString,
    #[serde(rename = "Name")]
    pub name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-26T18:48:28Z", "event":"Commander", "FID":"F1234567", "Name":"ItIsILeClerc" })]
pub struct EDLogCommander {
    #[serde(rename = "FID")]
    pub fid: EDString,
    #[serde(rename = "Name")]
    pub name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LoadOutStats {
    pub hull_value: Option<Credits>,
    pub modules_value: Option<Credits>,
    pub hull_health: f64,
    // Mass of Hull and Modules, excludes fuel and cargo
    pub unladen_mass: f64,
    pub cargo_capacity: u64,
    // based on zero cargo, and just enough fuel for 1 jump
    pub max_jump_range: f64,
    pub fuel_capacity: FuelCapacity,
    pub rebuy: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-10-15T04:34:40Z", "event":"Loadout", "Ship":"CobraMkIII", "ShipID":1, 
    "ShipName":"Flat Head", "ShipIdent":"UNSC-1", "Modules":[  ] })]
#[testcase({ "timestamp": "2024-01-14T18:13:22Z", "event": "Loadout", "Ship": "cobramkiii", "ShipID": 23, 
    "ShipName": "SCAVENGER", "ShipIdent": "LRB-C3", "HullValue": 349718, "ModulesValue": 6316388,
    "HullHealth": 1.000000, "UnladenMass": 222.577606, "CargoCapacity": 60, "MaxJumpRange": 39.726162,
    "FuelCapacity": {"Main": 16.000000,"Reserve": 0.490000},"Rebuy": 333307,"Modules": []})]
#[testcase({"timestamp": "2026-07-12T19:02:58Z","event": "Loadout", "Ship": "explorer_nx",
    "ShipID": 46, "ShipName": "my-caspian", "ShipIdent": "lrb-ce", "ModulesValue": 5585370,
    "HullHealth": 1.000000, "UnladenMass": 1480.151978, "CargoCapacity": 24, "MaxJumpRange": 76.526161, 
    "FuelCapacity": {"Main": 128.000000,"Reserve": 1.140000},"Rebuy": 279269,"Modules": []})]
pub struct EDLogLoadout {
    pub ship: ShipType,
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
    pub ship_name: EDString,
    pub ship_ident: EDString,
    pub hot: Option<bool>,
    #[serde(flatten)]
    pub loadout_stats: Option<LoadOutStats>,
    pub modules: Vec<Module>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-26T18:03:48Z", "event":"Materials", 
    "Raw":[ { "Name":"carbon", "Count":298 }, { "Name":"sulphur", "Count":282 } ], 
    "Manufactured":[ 
        { "Name":"wornshieldemitters", "Name_Localised":"Worn Shield Emitters", "Count":284 }, 
        { "Name":"mechanicalscrap", "Name_Localised":"Mechanical Scrap", "Count":270 }
    ],
    "Encoded":[ 
        { "Name":"shieldsoakanalysis", "Name_Localised":"Inconsistent Shield Soak Analysis", "Count":244 }, 
        { "Name":"consumerfirmware", "Name_Localised":"Modified Consumer Firmware", "Count":241 }
    ] })]
pub struct EDLogMaterials {
    pub raw: Vec<RawMaterial>,
    pub manufactured: Vec<ManufacturedMaterial>,
    pub encoded: Vec<EncodedMaterial>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-10-02T10:37:58Z", "event":"Missions", 
    "Active":[ { "MissionID":65380900, "Name":"Mission_Courier_name", "PassengerMission":false, "Expires":82751 } ], 
    "Failed":[ ],
    "Complete":[ ] })]
pub struct EDLogMissions {
    pub active: Vec<Mission>,
    pub failed: Vec<Mission>,
    pub complete: Vec<Mission>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(deny_unknown_fields)]
#[testcase({ "timestamp":"2016-06-10T14:32:03Z", "event":"NewCommander", "Name":"HRC1", "FID":"F44396", "Package":"ImperialBountyHunter" })]
pub struct EDLogNewCommander {
    #[serde(rename = "FID")]
    pub fid: EDString,
    #[serde(rename = "Name")]
    pub name: EDString,
    #[serde(rename = "Package")]
    pub package: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-10-14T18:41:37Z", "event":"LoadGame", "Commander":"JournalServer", "Ship":"CobraMkIII", "ShipID":1, "ShipName":"Flat Head", "ShipIdent":"UNSC-1", "FuelLevel":16.000000, "FuelCapacity":16.000000, "GameMode":"Open", "Credits":766731, "Loan":0 })]
#[testcase({ "timestamp":"2022-11-10T18:50:06Z", "event":"LoadGame", "FID":"F1234567", "Commander":"Myself", "Horizons":true, "Odyssey":true, "Credits":1234431, "Loan":0, "language":"English/UK", "gameversion":"4.0.0.1450", "build":"r286858/r0 " })]
#[testcase({ "timestamp":"2022-09-12T18:45:38Z", "event":"LoadGame", "FID":"F1234567", "Commander":"MySelf", "Horizons":true, "Ship":"FerDeLance", "Ship_Localised":"Fer-de-Lance", "ShipID":34, "ShipName":"", "ShipIdent":"", "FuelLevel":7.689338, "FuelCapacity":8.000000, "GameMode":"Group", "Group":"REINIER", "Credits":123321, "Loan":0 })]
#[testcase({ "timestamp":"2025-11-30T20:10:08Z", "event":"LoadGame", "FID":"F1234567", "Commander":"MySelf", "Horizons":true, "Odyssey":true, "Ship":"Python_NX", "Ship_Localised":"Python Mk II", "ShipID":12, "ShipName":"MyName", "ShipIdent":"IDENT1", "FuelLevel":16.000000, "FuelCapacity":16.000000, "GameMode":"Solo", "Credits":12341234, "Loan":0, "language":"English/UK", "gameversion":"4.2.2.1", "build":"r321306/r0 " })]
#[testcase({ "timestamp":"2022-11-08T19:15:39Z", "event":"LoadGame", "FID":"F9900129", "Commander":"MySelf", "Horizons":true, "Ship":"TestBuggy", "Ship_Localised":"SRV Scarab", "ShipID":10, "ShipName":"", "ShipIdent":"", "FuelLevel":0.000000, "FuelCapacity":0.000000, "GameMode":"Solo", "Credits":95073937, "Loan":0 })]
pub struct EDLogLoadGame {
    pub commander: EDString,
    pub name: Option<EDString>, // not in ed-journal-schemas
    #[serde(rename = "FID")]
    pub fid: Option<EDString>,
    pub horizons: Option<bool>,
    pub odyssey: Option<bool>,
    #[serde(flatten)]
    pub ship: Option<LoadGameShip>,
    pub start_landed: Option<bool>,
    pub start_dead: Option<bool>,
    pub game_mode: Option<GameMode>,
    pub group: Option<EDString>,
    pub credits: Credits,
    pub loan: Credits,
    #[serde(rename = "language")]
    pub language: Option<EDString>,
    #[serde(rename = "gameversion")]
    pub gameversion: Option<EDString>,
    #[serde(rename = "build")]
    pub build: Option<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PassengerMissionType {
    Business,
    Criminal,
    Medical,
    MinorCelebrity,
    Scientist,
    Soldier,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({"MissionID":951945534,"Type":"Criminal","VIP":true,"Wanted":true,"Count":7})]
pub struct Passenger {
    #[serde(rename = "MissionID")]
    pub mission_id: u64,
    #[serde(rename = "Type")]
    pub mission_type: PassengerMissionType,
    #[serde(rename = "VIP")]
    pub vip: bool,
    pub wanted: bool,
    pub count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({"timestamp":"2024-01-24T18:08:05Z","event":"Passengers","Manifest":[{"MissionID":951945534,"Type":"Criminal","VIP":true,"Wanted":true,"Count":7}]})]
pub struct EDLogPassengers {
    pub manifest: Vec<Passenger>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-26T17:58:22Z", "event":"Powerplay", "Power":"Jerome Archer", "Rank":123, "Merits":1231231, "TimePledged":50714872 })]
pub struct EDLogPowerplay {
    pub power: Power,
    pub rank: u64,
    pub merits: Merits,
    pub votes: Option<u64>,
    #[serde(with = "duration_as_secs")]
    pub time_pledged: Duration,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-09-13T18:29:43Z", "event":"Rank", "Combat":9, "Trade":12, "Explore":9, "Soldier":8, "Exobiologist":8, "Empire":12, "Federation":12, "CQC":0 })]
pub struct EDLogRank {
    pub combat: CombatRank,
    pub trade: TradeRank,
    pub explore: ExploreRank,
    pub soldier: Option<SoldierRank>,
    pub exobiologist: Option<ExobiologistRank>,
    pub empire: EmpireRank,
    pub federation: FederationRank,
    #[serde(rename = "CQC")]
    pub cqc: CQCRank,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-26T19:21:35Z", "event":"Reputation", "Empire":75.000000, "Federation":77.075302, "Independent":0.000000, "Alliance":75.000000 })]
// This gives the player's reputation (on a scale of -100..+100) with the superpowers
pub struct EDLogReputation {
    pub federation: Option<f64>,
    pub empire: Option<f64>,
    pub independent: Option<f64>,
    pub alliance: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogStatistics {
    #[serde(rename = "Bank_Account")]
    pub bank_account: StatisticsBankAccount,
    pub combat: StatisticsCombat,
    pub crime: StatisticsCrime,
    pub smuggling: StatisticsSmuggling,
    pub trading: StatisticsTrading,
    pub mining: StatisticsMining,
    pub exploration: StatisticsExploration,
    pub passengers: StatisticsPassengers,
    #[serde(rename = "Search_And_Rescue")]
    pub search_and_rescue: StatisticSearchAndRescue,
    pub squadron: Option<StatisticsSquadron>,
    #[serde(rename = "TG_ENCOUNTERS")]
    pub tg_encounters: Option<StatisticsTgEncounters>,
    pub crafting: StatisticsCrafting,
    pub crew: StatisticsCrew,
    pub multicrew: StatisticsMulticrew,
    #[serde(rename = "Material_Trader_Stats")]
    pub material_trader_stats: StatisticsMaterialTraderStats,
    #[serde(rename = "CQC")]
    pub cqc_stats: Option<StatisticsCQCStats>,
    #[serde(rename = "FLEETCARRIER")]
    pub fleetcarrier: Option<StatisticsFleetcarrier>,
    pub exobiology: Option<StatisticsExobiology>,
}

#[test]
fn test_loadoput() {
    let json = r#"{ "timestamp":"2026-07-12T19:02:58Z", "event":"Loadout", "Ship":"explorer_nx", "ShipID":46, 
        "ShipName":"my-caspian", "ShipIdent":"lrb-ce", "ModulesValue":5585370, "HullHealth":1.000000, 
        "UnladenMass":1480.151978, "CargoCapacity":24, "MaxJumpRange":76.526161, "FuelCapacity":{ "Main":128.000000, 
        "Reserve":1.140000 }, "Rebuy":279269, "Modules":[ ] }"#;
    let line: crate::log_line::EDLogLine = serde_json::from_str(json).expect("Should parse");

    assert!(matches!(
        line.event(),
        crate::log_line::EDLogEvent::Loadout(_)
    ));

    let loadout = line
        .event()
        .extract::<EDLogLoadout>()
        .expect("should be loadout inside");

    assert_eq!(loadout.ship_id, 46);
    assert_eq!(
        loadout
            .loadout_stats
            .as_ref()
            .expect("it to be there")
            .cargo_capacity,
        24
    );
}
