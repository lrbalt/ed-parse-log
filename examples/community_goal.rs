use chrono::{DateTime, Utc};
use ed_parse_log_files::{
    community_goal::{CommunityGoal, EDLogCommunityGoalJoin, EDLogCommunityGoalReward},
    log_line::{EDLogEvent, EDLogLine},
};
use itertools::Itertools;
use numfmt::{Formatter, Precision, Scales};
use prettytable::{Table, row};
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use std::{
    env,
    ffi::OsStr,
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

pub fn read_logs(path: &str) -> Result<Mutex<Vec<EDLogLine>>, MyError> {
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
    if !e.is_empty() {
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

fn filter_loglines(db: Mutex<Vec<EDLogLine>>) -> Result<Vec<EDLogLine>, MyError> {
    let mut lines: Vec<EDLogLine> = db
        .lock()
        .unwrap()
        .par_iter()
        //
        // filter on the lines that contain system data
        //
        .filter(|line| {
            matches!(
                line.event(),
                EDLogEvent::CommunityGoal(_)
                    | EDLogEvent::CommunityGoalJoin(_)
                    | EDLogEvent::CommunityGoalReward(_)
            )
        })
        .cloned()
        .collect();

    lines.sort_by(|a, b| a.timestamp().cmp(b.timestamp()));

    Ok(lines)
}

#[derive(Debug)]
struct GoalData {
    cgid: u64,
    goal: Option<CommunityGoal>,
    join: Option<EDLogCommunityGoalJoin>,
    reward: Option<EDLogCommunityGoalReward>,
    goal_timestamp: Option<DateTime<Utc>>,
    join_timestamp: Option<DateTime<Utc>>,
    reward_timestamp: Option<DateTime<Utc>>,
}

fn find_goal(lines: &[EDLogLine], id: u64) -> GoalData {
    let mut data = GoalData {
        cgid: id,
        goal: None,
        join: None,
        reward: None,
        goal_timestamp: None,
        join_timestamp: None,
        reward_timestamp: None,
    };

    for line in lines.iter().rev() {
        match line.event() {
            EDLogEvent::CommunityGoal(g) => {
                for g in g.current_goals.iter() {
                    if data.goal.is_none() && g.cgid == id {
                        data.goal = Some(g.clone());
                        data.goal_timestamp = Some(*line.timestamp());
                    }
                }
            }
            EDLogEvent::CommunityGoalJoin(g) => {
                if data.join.is_none() && g.cgid == id {
                    data.join = Some(g.clone());
                    data.join_timestamp = Some(*line.timestamp());
                }
            }
            EDLogEvent::CommunityGoalReward(g) => {
                if data.reward.is_none() && g.cgid == id {
                    data.reward = Some(g.clone());
                    data.reward_timestamp = Some(*line.timestamp());
                }
            }
            _ => continue,
        }
    }

    data
}

fn line_break(line: &str, width: usize) -> String {
    let mut result = String::new();
    let mut len = 0;

    for word in line.split(' ') {
        if len + word.len() > width {
            result = result + "\n" + word;
            len = 0;
        } else {
            result = result + " " + word;
            len += word.len();
        }
    }

    result[1..].to_string()
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} </path/to/log/files>", env!("CARGO_PKG_NAME"));
        exit(-1);
    }
    let path = &args[1];

    println!("Reading log files");
    let db = read_logs(path).unwrap();

    println!("Filtering and sorting relevant lines");
    let items = filter_loglines(db).unwrap();

    let goals = items
        .iter()
        .flat_map(|line| match line.event() {
            EDLogEvent::CommunityGoal(g) => {
                g.current_goals.iter().map(|g| g.cgid).collect::<Vec<_>>()
            }
            EDLogEvent::CommunityGoalJoin(g) => vec![g.cgid],
            EDLogEvent::CommunityGoalReward(g) => vec![g.cgid],
            _ => vec![],
        })
        .unique()
        .map(|id| find_goal(&items, id))
        .collect::<Vec<_>>();

    println!(
        "Found {} lines about Community Goals for {} unique goals",
        items.len(),
        goals.len()
    );

    let mut dec_formatter = Formatter::new()
        .scales(Scales::none())
        .separator('.')
        .unwrap()
        .precision(Precision::Decimals(0));

    let mut table = Table::new();
    table.add_row(row!["ID", "Complete", "Title", "When", "Details", "Reward"]);
    for goal in goals {
        let when = format!(
            "Joined: {}\nStatus: {}\nExpire: {}\nReward: {}",
            goal.join_timestamp
                .map(|d| d.date_naive().to_string())
                .unwrap_or_else(|| "n/a".to_string()),
            goal.goal_timestamp
                .map(|d| d.date_naive().to_string())
                .unwrap_or_else(|| "n/a".to_string()),
            goal.goal
                .as_ref()
                .map(|g| g.expiry.date_naive().to_string())
                .unwrap_or_else(|| "n/a".to_string()),
            goal.reward_timestamp
                .map(|d| d.date_naive().to_string())
                .unwrap_or_else(|| "n/a".to_string()),
        );

        let details = format!(
            "Total: {}\nNumber of contributors: {}\nTop Tier: {}\nTier reached: {}",
            goal.goal
                .as_ref()
                .map(|g| dec_formatter.fmt2(g.current_total).to_string())
                .unwrap_or_else(|| "n/a".to_string()),
            goal.goal
                .as_ref()
                .map(|g| dec_formatter.fmt2(g.num_contributors).to_string())
                .unwrap_or_else(|| "n/a".to_string()),
            goal.goal
                .as_ref()
                .map(|g| g.top_tier.name.as_str())
                .unwrap_or("n/a"),
            goal.goal
                .as_ref()
                .and_then(|g| g.tier_reached.map(|s| s.as_str()))
                .unwrap_or("n/a"),
        );
        let reward = format!(
            "Player contribution: {}\nYou're in top {}%\nBonus:{}\nReward:{}",
            goal.goal
                .as_ref()
                .map(|g| dec_formatter.fmt2(g.player_contribution).to_string())
                .unwrap_or_else(|| "n/a".to_string()),
            goal.goal
                .as_ref()
                .map(|g| g.player_percentile_band.to_string())
                .unwrap_or_else(|| "n/a".to_string()),
            goal.goal
                .as_ref()
                .and_then(|g| g.bonus.map(|b| dec_formatter.fmt2(b).to_string()))
                .unwrap_or_else(|| "n/a".to_string()),
            goal.reward
                .as_ref()
                .map(|g| dec_formatter.fmt2(g.reward).to_string())
                .unwrap_or_else(|| "n/a".to_string()),
        );

        table.add_row(row![
            goal.cgid,
            goal.goal
                .as_ref()
                .map(|g| format!("{:?}", g.is_complete))
                .unwrap_or_else(|| "n/a".to_string()),
            goal.goal
                .as_ref()
                .map(|g| line_break(g.title.as_str(), 30))
                .unwrap_or_else(|| "n/a".to_string()),
            when,
            details,
            reward,
        ]);
    }
    table.printstd();
}
