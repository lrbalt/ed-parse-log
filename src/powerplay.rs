use crate::{EDString, common_types::Merits, utils::duration_as_secs};
use chrono::Duration;
use ed_parse_log_files_macros::Extractable;
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
pub struct EDLogPowerplay {
    pub power: EDString,
    pub rank: u64,
    pub merits: Merits,
    pub votes: Option<u64>,
    #[serde(with = "duration_as_secs")]
    pub time_pledged: Duration,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplayJoin {
    power: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplayCollect {
    power: EDString,
    #[serde(rename = "Type")]
    power_type: EDString,
    #[serde(rename = "Type_Localised")]
    power_type_localised: EDString,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplayDeliver {
    power: EDString,
    #[serde(rename = "Type")]
    power_type: EDString,
    #[serde(rename = "Type_Localised")]
    power_type_localised: EDString,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplayMerits {
    power: EDString,
    merits_gained: Merits,
    total_merits: Merits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplayFastTrack {
    power: EDString,
    cost: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplayDefect {
    from_power: EDString,
    to_power: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplayLeave {
    power: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplayRank {
    power: EDString,
    rank: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogPowerplaySalary {
    power: EDString,
    amount: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
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
