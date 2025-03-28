use chrono::{DateTime, Duration, Utc};
use ed_parse_log_files::{
    commander::{
        COMBAT_RANK, EDLogPowerplay, EDLogRank, EDLogReputation, EMPIRE_RANK, EXOBIOLOGIST_RANK,
        EXPLORE_RANK, FEDERATION_RANK, SOLDIER_RANK, TRADE_RANK, power_play_rank_range,
    },
    common_types::{Credits, Merits},
    log_line::{EDLogEvent, EDLogLine},
};
use prettytable::{Table, cell, row};
use rayon::prelude::*;
use std::{
    env,
    ffi::OsStr,
    fmt::Display,
    fs::{File, read_dir},
    io::{BufRead, BufReader},
    process::exit,
    sync::Mutex,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO Error")]
    IOError(#[from] std::io::Error),
    #[error("Parse Error {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("Errors found while reading log files")]
    ErrorsFoundInLogFiles,
}

pub fn format_duration(d: &Duration) -> String {
    let weeks = d.num_weeks();
    let d = *d - Duration::weeks(weeks);
    let days = d.num_days();
    let d = d - Duration::days(days);
    let hours = d.num_hours();
    let d = d - Duration::hours(hours);
    let minutes = d.num_minutes();
    let d = d - Duration::minutes(minutes);
    let secs = d.num_seconds();

    match (weeks, days, hours, minutes, secs) {
        (0, 0, 0, m, s) => format!("{m}:{s}"),
        (0, 0, h, _m, _s) => format!("{h}h"),
        (0, d, h, _m, _s) => format!("{d}d {h}:h"),
        (w, d, h, _m, _s) => format!("{w}w {d}d {h}h"),
    }
}

fn progression_string_map(a: u64, b: u64, map: &[&str]) -> String {
    if a == b {
        map[a as usize].to_string()
    } else {
        format!("{} → {}", map[a as usize], map[b as usize])
    }
}

fn progression_string_str(a: &str, b: &str) -> String {
    if a == b {
        a.to_string()
    } else {
        format!("{} → {}", a, b)
    }
}

fn progression_string_num(a: u64, b: u64) -> String {
    if a == b {
        format!("{}", a)
    } else {
        format!("{} → {}", a, b)
    }
}

fn progression_string_credits(a: Credits, b: Credits) -> String {
    if a == b {
        a.to_human_readable_string().to_string()
    } else {
        format!(
            "{} → {} ({})",
            a.to_human_readable_string(),
            b.to_human_readable_string(),
            (b - a).to_human_readable_string()
        )
    }
}

fn progression_string_merits(a: Merits, b: Merits) -> String {
    if a == b {
        a.value().to_string()
    } else {
        format!("{} → {} ({})", a, b, (b - a))
    }
}

fn progression_string_f64(a: f64, b: f64) -> String {
    if a == b {
        format!("{:.1}%", a)
    } else {
        format!("{:.1}% → {:.1}%", a, b)
    }
}

fn progression_string_perc(a: u64, b: u64) -> String {
    if a == b {
        format!("{}%", a)
    } else {
        format!("{}% → {}%", a, b)
    }
}

fn progression_string_duration(a: Duration, b: Duration) -> String {
    if a == b {
        format!("{}%", format_duration(&a))
    } else {
        format!("{}% → {}%", format_duration(&a), format_duration(&b))
    }
}

pub struct CreditProgress {
    start: Credits,
    end: Credits,
}

pub struct CreditsProgress {
    wealth: Option<CreditProgress>,
    credits: Option<CreditProgress>,
    fleet_carrier: Option<CreditProgress>,
}

impl CreditsProgress {
    pub fn new(data: &[EDLogLine]) -> Option<CreditsProgress> {
        let mut reps = data
            .iter()
            .filter(|line| matches!(line.event(), EDLogEvent::Statistics(_)));
        let first = reps.next();
        let last = reps.last();

        let wealth = first.and_then(|first| {
            let last = last.unwrap_or(first);
            match (first.event(), last.event()) {
                (EDLogEvent::Statistics(first), EDLogEvent::Statistics(last)) => {
                    Some(CreditProgress {
                        start: first.bank_account.current_wealth,
                        end: last.bank_account.current_wealth,
                    })
                }
                _ => None,
            }
        });

        let mut reps = data.iter().filter(|line| {
            matches!(
                line.event(),
                EDLogEvent::CarrierStats(_) | EDLogEvent::CarrierFinance(_)
            )
        });
        let first = reps.next();
        let last = reps.last();

        let fleet_carrier = first.and_then(|first| {
            let last = last.unwrap_or(first);
            match (first.event(), last.event()) {
                (EDLogEvent::CarrierStats(first), EDLogEvent::CarrierStats(last)) => {
                    Some(CreditProgress {
                        start: first.finance.carrier_balance,
                        end: last.finance.carrier_balance,
                    })
                }
                (EDLogEvent::CarrierFinance(first), EDLogEvent::CarrierFinance(last)) => {
                    Some(CreditProgress {
                        start: first.carrier_balance,
                        end: last.carrier_balance,
                    })
                }
                (EDLogEvent::CarrierFinance(first), EDLogEvent::CarrierStats(last)) => {
                    Some(CreditProgress {
                        start: first.carrier_balance,
                        end: last.finance.carrier_balance,
                    })
                }
                (EDLogEvent::CarrierStats(first), EDLogEvent::CarrierFinance(last)) => {
                    Some(CreditProgress {
                        start: first.finance.carrier_balance,
                        end: last.carrier_balance,
                    })
                }
                _ => None,
            }
        });

        let mut reps = data
            .iter()
            .filter(|line| matches!(line.event(), EDLogEvent::LoadGame(_)));
        let first = reps.next();
        let last = reps.last();

        let credits = first.and_then(|first| {
            let last = last.unwrap_or(first);
            match (first.event(), last.event()) {
                (EDLogEvent::LoadGame(first), EDLogEvent::LoadGame(last)) => Some(CreditProgress {
                    start: first.credits,
                    end: last.credits,
                }),
                _ => None,
            }
        });

        Some(CreditsProgress {
            wealth,
            credits,
            fleet_carrier,
        })
    }
}

impl Display for CreditsProgress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Wealth: {}\nCredits: {}\nFleet Carrier: {}",
            self.wealth
                .as_ref()
                .map(|w| progression_string_credits(w.start, w.end))
                .unwrap_or_else(|| "n/a".to_string()),
            self.credits
                .as_ref()
                .map(|w| progression_string_credits(w.start, w.end))
                .unwrap_or_else(|| "n/a".to_string()),
            self.fleet_carrier
                .as_ref()
                .map(|w| progression_string_credits(w.start, w.end))
                .unwrap_or_else(|| "n/a".to_string()),
        )
    }
}

#[derive(Debug)]
pub struct Reputation {
    federation: f64,
    empire: f64,
    independent: f64,
    alliance: f64,
}

impl From<&EDLogReputation> for Reputation {
    fn from(value: &EDLogReputation) -> Self {
        Reputation {
            federation: value.federation.unwrap_or(0.0),
            empire: value.empire.unwrap_or(0.0),
            independent: value.independent.unwrap_or(0.0),
            alliance: value.alliance.unwrap_or(0.0),
        }
    }
}

#[derive(Debug)]
pub struct ReputationProgress {
    start: Reputation,
    end: Reputation,
}

impl ReputationProgress {
    pub fn new(data: &[EDLogLine]) -> Option<ReputationProgress> {
        let mut reps = data
            .iter()
            .filter(|line| matches!(line.event(), EDLogEvent::Reputation(_)));
        let first = reps.next();
        let last = reps.last();

        first.and_then(|first| {
            let last = last.unwrap_or(first);
            match (first.event(), last.event()) {
                (EDLogEvent::Reputation(first), EDLogEvent::Reputation(last)) => {
                    Some(ReputationProgress {
                        start: first.into(),
                        end: last.into(),
                    })
                }
                _ => None,
            }
        })
    }
}

impl Display for ReputationProgress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Federation: {}\nEmpire: {}\nIndependent: {}\nAlliance: {}",
            progression_string_f64(self.start.federation, self.end.federation),
            progression_string_f64(self.start.empire, self.end.empire),
            progression_string_f64(self.start.independent, self.end.independent),
            progression_string_f64(self.start.alliance, self.end.alliance),
        )
    }
}

#[derive(Debug)]
pub struct PowerPlayProgress {
    power: String,
    rank: u64,
    merits: Merits,
    time_pledged: Duration,
}

impl PowerPlayProgress {
    /// show progress towards next level
    pub fn progress(&self) -> f64 {
        let (start, end) = power_play_rank_range(self.rank);
        let delta = end - start;
        let current = self.merits.value() - start;
        (current as f64 / delta as f64) * 100.0
    }
}

impl From<&EDLogPowerplay> for PowerPlayProgress {
    fn from(value: &EDLogPowerplay) -> Self {
        PowerPlayProgress {
            power: value.power.clone(),
            rank: value.rank,
            merits: value.merits,
            time_pledged: value.time_pledged,
        }
    }
}

#[derive(Debug)]
pub struct PowerPlay {
    start: PowerPlayProgress,
    end: PowerPlayProgress,
}

impl PowerPlay {
    pub fn new(data: &[EDLogLine]) -> Option<PowerPlay> {
        let mut reps = data
            .iter()
            .filter(|line| matches!(line.event(), EDLogEvent::Powerplay(_)));
        let first = reps.next();
        let last = reps.last();

        first.and_then(|first| {
            let last = last.unwrap_or(first);
            match (first.event(), last.event()) {
                (EDLogEvent::Powerplay(first), EDLogEvent::Powerplay(last)) => Some(PowerPlay {
                    start: first.into(),
                    end: last.into(),
                }),
                _ => None,
            }
        })
    }
}

impl Display for PowerPlay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Power: {}\nRank: {}\nRank progress: {:.1}% ({})\nMerits: {}\nTime Pledged: {}",
            progression_string_str(&self.start.power, &self.end.power),
            progression_string_num(self.start.rank, self.end.rank),
            self.end.progress(),
            power_play_rank_range(self.end.rank).1,
            progression_string_merits(self.start.merits, self.end.merits),
            progression_string_duration(self.start.time_pledged, self.end.time_pledged),
        )
    }
}

#[derive(Debug)]
pub struct Rank {
    pub combat: u64,
    pub trade: u64,
    pub explore: u64,
    pub soldier: u64,
    pub exobiologist: u64,
    pub empire: u64,
    pub federation: u64,
    pub cqc: u64,
}

impl From<&EDLogRank> for Rank {
    fn from(value: &EDLogRank) -> Self {
        Self {
            combat: value.combat,
            trade: value.trade,
            explore: value.explore,
            soldier: value.soldier.unwrap_or(0),
            exobiologist: value.exobiologist.unwrap_or(0),
            empire: value.empire,
            federation: value.federation,
            cqc: value.cqc,
        }
    }
}

#[derive(Debug)]
pub struct RankProgress {
    start: Rank,
    end: Rank,
}

impl RankProgress {
    pub fn new(data: &[EDLogLine]) -> Option<RankProgress> {
        let mut reps = data
            .iter()
            .filter(|line| matches!(line.event(), EDLogEvent::Rank(_)));
        let first = reps.next();
        let last = reps.last();

        first.and_then(|first| {
            let last = last.unwrap_or(first);
            match (first.event(), last.event()) {
                (EDLogEvent::Rank(first), EDLogEvent::Rank(last)) => Some(RankProgress {
                    start: first.into(),
                    end: last.into(),
                }),
                _ => None,
            }
        })
    }
}

impl Display for RankProgress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let combat = progression_string_map(self.start.combat, self.end.combat, &COMBAT_RANK);
        let trade = progression_string_map(self.start.trade, self.end.trade, &TRADE_RANK);
        let explore = progression_string_map(self.start.explore, self.end.explore, &EXPLORE_RANK);
        let soldier = progression_string_map(self.start.soldier, self.end.soldier, &SOLDIER_RANK);
        let exobiology = progression_string_map(
            self.start.exobiologist,
            self.end.exobiologist,
            &EXOBIOLOGIST_RANK,
        );
        let empire = progression_string_map(self.start.empire, self.end.empire, &EMPIRE_RANK);
        let federation =
            progression_string_map(self.start.federation, self.end.federation, &FEDERATION_RANK);
        let cqc = progression_string_num(self.start.cqc, self.end.cqc);
        write!(
            f,
            "Combat: {}\nTrade: {}\nExplore: {}\nSoldier: {}\nExobiology: {}\nEmpire: {}\nFederation: {}\nCQC: {}",
            combat, trade, explore, soldier, exobiology, empire, federation, cqc
        )
    }
}

#[derive(Debug)]
pub struct RankProgression {
    combat: u64,
    trade: u64,
    explore: u64,
    soldier: u64,
    exobiologist: u64,
    empire: u64,
    federation: u64,
    cqc: u64,
}

impl From<&EDLogRank> for RankProgression {
    fn from(value: &EDLogRank) -> Self {
        Self {
            combat: value.combat,
            trade: value.trade,
            explore: value.explore,
            soldier: value.soldier.unwrap_or(0),
            exobiologist: value.exobiologist.unwrap_or(0),
            empire: value.empire,
            federation: value.federation,
            cqc: value.cqc,
        }
    }
}

#[derive(Debug)]
pub struct RankProgressionChange {
    start: RankProgression,
    end: RankProgression,
}

impl RankProgressionChange {
    pub fn new(data: &[EDLogLine]) -> Option<RankProgressionChange> {
        let mut reps = data
            .iter()
            .filter(|line| matches!(line.event(), EDLogEvent::Progress(_)));
        let first = reps.next();
        let last = reps.last();

        first.and_then(|first| {
            let last = last.unwrap_or(first);
            match (first.event(), last.event()) {
                (EDLogEvent::Progress(first), EDLogEvent::Progress(last)) => {
                    Some(RankProgressionChange {
                        start: first.into(),
                        end: last.into(),
                    })
                }
                _ => None,
            }
        })
    }
}

impl Display for RankProgressionChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let combat = progression_string_perc(self.start.combat, self.end.combat);
        let trade = progression_string_perc(self.start.trade, self.end.trade);
        let explore = progression_string_perc(self.start.explore, self.end.explore);
        let soldier = progression_string_perc(self.start.soldier, self.end.soldier);
        let exobiologist = progression_string_perc(self.start.exobiologist, self.end.exobiologist);
        let empire = progression_string_perc(self.start.empire, self.end.empire);
        let federation = progression_string_perc(self.start.federation, self.end.federation);
        let cqc = progression_string_perc(self.start.cqc, self.end.cqc);

        write!(
            f,
            "Combat: {}\nTrade: {}\nExplore: {}\nSoldier: {}\nExobiology: {}\nEmpire: {}\nFederation: {}\nCQC: {}",
            combat, trade, explore, soldier, exobiologist, empire, federation, cqc,
        )
    }
}

pub fn read_logs() -> Result<Mutex<Vec<EDLogLine>>, MyError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} /path/to/log/files", env!("CARGO_PKG_NAME"));
        exit(-1);
    }
    let path = &args[1];

    let errors = Mutex::new(Vec::new());
    let db: Mutex<Vec<EDLogLine>> = Mutex::new(Vec::new());

    let files = read_dir(path)?
        .map(|e| e.map(|e| e.path()))
        .collect::<Result<Vec<_>, _>>()?;

    let log_files: Vec<_> = files
        .iter()
        .filter(|f| f.extension() == Some(OsStr::new("log")))
        .collect();

    log_files
        .iter()
        .enumerate()
        .par_bridge()
        .for_each(|(file_number, path)| {
            let path_str = path.display().to_string();

            let file = match File::open(path).map_err(|e| e.into()) {
                Ok(path) => path,
                Err(e) => {
                    errors.lock().unwrap().push((
                        e,
                        file_number,
                        "".to_string(),
                        0,
                        "".to_string(),
                    ));
                    return;
                }
            };
            let reader = BufReader::new(file);
            let empty = "".to_string();

            reader
                .lines()
                .enumerate()
                .par_bridge()
                .for_each(|(linenumber, line)| {
                    let orig_line = line.as_ref().unwrap_or(&empty).clone();
                    let ed_line: Result<EDLogLine, MyError> = line
                        .map_err(|e| e.into())
                        .and_then(|line| serde_json::from_str(&line).map_err(|e| e.into()));
                    match ed_line {
                        Ok(line) => db.lock().unwrap().push(line),
                        Err(e) => errors.lock().unwrap().push((
                            e,
                            file_number,
                            path_str.clone(),
                            linenumber,
                            orig_line,
                        )),
                    };
                })
        });

    let e = errors.lock().unwrap();
    if e.len() > 0 {
        println!("Errors found while parsing log files. First 5 are:");
        for (index, error) in e.iter().take(5).enumerate() {
            eprintln!(
                "\nError #{index}: {}\n\nin file {}:{} at line {}\n{}",
                error.0, error.1, error.2, error.3, error.4,
            );
        }
        return Err(MyError::ErrorsFoundInLogFiles);
    }

    Ok(db)
}

fn filter_progress(db: Mutex<Vec<EDLogLine>>) -> Result<Vec<EDLogLine>, MyError> {
    let lines = db.lock().unwrap();
    let progress = lines
        .par_iter()
        .filter(|line| {
            matches!(
                line.event(),
                EDLogEvent::Progress(_)
                    | EDLogEvent::Rank(_)
                    | EDLogEvent::Reputation(_)
                    | EDLogEvent::Powerplay(_)
                    | EDLogEvent::Statistics(_)
                    | EDLogEvent::CarrierFinance(_)
                    | EDLogEvent::CarrierStats(_)
                    | EDLogEvent::LoadGame(_)
            )
        })
        .cloned()
        .collect();

    Ok(progress)
}

pub fn get_data_in_period(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    data: &[EDLogLine],
) -> &[EDLogLine] {
    if data.is_empty() {
        return &[];
    } else {
        assert!(start < end);
        assert!(data[0].timestamp() < data[data.len() - 1].timestamp());
    }

    let start_idx = data.partition_point(|m| *m.timestamp() < start);
    if start_idx == data.len() {
        return &[];
    }

    let end_part = &data[start_idx..];
    let mut end_idx = end_part.partition_point(|m| *m.timestamp() <= end);

    if end_idx == end_part.len() {
        return end_part;
    } else {
        end_idx += start_idx;
    }

    &data[start_idx..end_idx]
}

fn show_progress(mut progress_items: Vec<EDLogLine>) {
    println!("Sorting");
    progress_items.par_sort_by(|a, b| a.timestamp().partial_cmp(b.timestamp()).unwrap());

    let today = Utc::now();
    let columns = [
        ("Last day", today - Duration::days(1)),
        ("Last 7 days", today - Duration::days(7)),
        ("Last 30 days", today - Duration::days(30)),
        ("Last 365 days", today - Duration::days(365)),
    ];
    let mut rows = [
        row![""],
        row!["Credits"],
        row!["Rank"],
        row!["Rank\nProgress"],
        row!["Reputation"],
        row!["Power Play"],
    ];

    let mut table = Table::new();
    for (title, end_date) in columns.iter() {
        let data = get_data_in_period(*end_date, today, &progress_items);
        let credits = CreditsProgress::new(data)
            .map(|p| p.to_string())
            .unwrap_or_else(|| "n/a".into());
        let progress = RankProgressionChange::new(data)
            .map(|p| p.to_string())
            .unwrap_or_else(|| "n/a".into());
        let rank = RankProgress::new(data)
            .map(|p| p.to_string())
            .unwrap_or_else(|| "n/a".into());
        let power_play = PowerPlay::new(data)
            .map(|p| p.to_string())
            .unwrap_or_else(|| "n/a".into());
        let reputation = ReputationProgress::new(data)
            .map(|p| p.to_string())
            .unwrap_or_else(|| "n/a".into());

        rows[0].add_cell(cell!(title));
        rows[1].add_cell(cell!(credits));
        rows[2].add_cell(cell!(rank));
        rows[3].add_cell(cell!(progress));
        rows[4].add_cell(cell!(reputation));
        rows[5].add_cell(cell!(power_play));
    }
    for row in rows {
        table.add_row(row);
    }
    table.printstd();
}

pub fn main() {
    println!("Reading log files");
    let db = read_logs().unwrap();
    println!("Filtering progress lines");
    let progress_items = filter_progress(db).unwrap();
    println!("Show progress");
    show_progress(progress_items);
}
