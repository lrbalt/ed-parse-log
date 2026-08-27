use crate::{
    EDString,
    common_types::{Merits, Power},
};
use ed_parse_log_files_macros::{Extractable, testcase};
use serde::{Deserialize, Serialize};

pub fn power_play_rank_range(rank: u64) -> (u64, u64) {
    match rank {
        0 => (0, 0),
        1 => (0, 2000),
        2 => (2000, 5000),
        3 => (5000, 9000),
        4 => (9000, 15000),
        _ => (15000 + (rank - 5) * 8000, 23000 + (rank - 5) * 8000),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-01-16T18:30:30Z", "event":"PowerplayJoin", "Power":"Jerome Archer" })]
pub struct EDLogPowerplayJoin {
    power: Power,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-03-23T14:29:05Z", "event":"PowerplayCollect", "Power":"Jerome Archer", 
    "Type":"republicanfieldsupplies", "Type_Localised":"Archer's Field Supplies", "Count":16 })]
pub struct EDLogPowerplayCollect {
    power: Power,
    #[serde(rename = "Type")]
    power_type: EDString,
    #[serde(rename = "Type_Localised")]
    power_type_localised: EDString,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-04-15T18:12:47Z", "event":"PowerplayDeliver", "Power":"Jerome Archer", 
    "Type":"poweremployeedata", "Type_Localised":"Power Association Data", "Count":3 })]
pub struct EDLogPowerplayDeliver {
    power: Power,
    #[serde(rename = "Type")]
    power_type: EDString,
    #[serde(rename = "Type_Localised")]
    power_type_localised: EDString,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-09T13:16:28Z", "event":"PowerplayMerits", "Power":"Jerome Archer", "MeritsGained":345, "TotalMerits":1231231 })]
pub struct EDLogPowerplayMerits {
    pub power: Power,
    pub merits_gained: Merits,
    pub total_merits: Merits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2024-02-09T17:53:18Z", "event":"PowerplayFastTrack", "Power":"Aisling Duval", "Cost":150000 })]
pub struct EDLogPowerplayFastTrack {
    power: Power,
    cost: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplayDefect {
    from_power: Power,
    to_power: Power,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplayLeave {
    power: Power,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplayRank {
    pub power: Power,
    pub rank: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2024-10-17T17:53:31Z", "event":"PowerplaySalary", "Power":"Aisling Duval", "Amount":1000 })]
pub struct EDLogPowerplaySalary {
    power: Power,
    amount: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-11-27T20:19:04Z", "event":"HoloscreenHacked", "PowerBefore":"Edmund Mahon", "PowerAfter":"Jerome Archer" })]
pub struct EDLogHoloscreenHacked {
    power_before: Option<EDString>,
    power_after: EDString,
}

#[test]
fn test_power_plat_rank_range() {
    assert_eq!((15000, 23000), power_play_rank_range(5));
    assert_eq!((55000, 63000), power_play_rank_range(10));
    assert_eq!((375000, 383000), power_play_rank_range(50));
    assert_eq!((775000, 783000), power_play_rank_range(100));
}

#[test]
fn test_powerplay() {
    use crate::log_line::{EDLogEvent, EDLogLine};

    let json10 = r#"{ "timestamp":"2024-09-16T14:34:52Z", "event":"Powerplay", "Power":"Aisling Duval", "Rank":10, "Merits":0, "Votes":0, "TimePledged":21406314 }"#;
    let json20 = r#"{ "timestamp":"2025-03-10T18:21:04Z", "event":"Powerplay", "Power":"Jerome Archer", "Rank":89, "Merits":687268, "TimePledged":4578634 }"#;
    let line10: EDLogLine = serde_json::from_str(json10).expect("Should parse");
    let line20: EDLogLine = serde_json::from_str(json20).expect("Should parse");

    assert!(matches!(line10.event(), EDLogEvent::Powerplay(_)));
    assert!(matches!(line20.event(), EDLogEvent::Powerplay(_)));
    if let EDLogEvent::Powerplay(pp) = line10.event() {
        assert_eq!(10, pp.rank);
        assert_eq!(Some(0), pp.votes);
    }
    if let EDLogEvent::Powerplay(pp) = line20.event() {
        assert_eq!(89, pp.rank);
        assert_eq!(None, pp.votes, "PP2.0 dropped votes");
    }
}
