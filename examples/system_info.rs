use ed_parse_log_files::{
    common_types::{Allegiance, Powers},
    location::EDLogLocation,
    log_line::{EDLogEvent, EDLogLine},
    navigation::EDLogFSDJump,
};
use prettytable::{Table, cell, format, row};
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

fn filter_loglines(db: Mutex<Vec<EDLogLine>>, system_id: u64) -> Result<Vec<EDLogLine>, MyError> {
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
                EDLogEvent::Scan(_)
                    | EDLogEvent::DiscoveryScan(_)
                    | EDLogEvent::NavBeaconScan(_)
                    | EDLogEvent::ScanOrganic(_)
                    | EDLogEvent::FSSDiscoveryScan(_)
                    | EDLogEvent::Location(_)
                    | EDLogEvent::FSDJump(_)
            )
        })
        .map(|line| match line.event() {
            EDLogEvent::Scan(d) => Some((line, d.system_address)),
            EDLogEvent::FSSDiscoveryScan(d) => Some((line, d.system_address)),
            EDLogEvent::Location(d) => Some((line, d.system_address)),
            EDLogEvent::FSDJump(d) => Some((line, d.system_address)),
            _ => None,
        })
        //
        // filter on the data for the specific system
        //
        .filter(|found_address| {
            if let Some((_, system_address)) = found_address {
                *system_address == system_id
            } else {
                false
            }
        })
        .map(|found_data| found_data.unwrap().0)
        .cloned()
        .collect();

    lines.sort_by(|a, b| a.timestamp().cmp(b.timestamp()));

    Ok(lines)
}

fn find_system_address(db: &Mutex<Vec<EDLogLine>>, system_name: &str) -> Option<u64> {
    for line in db.lock().unwrap().iter() {
        let found_id = match line.event() {
            EDLogEvent::Scan(d) => Some((d.system_address, &d.star_system)),
            EDLogEvent::FSSDiscoveryScan(d) => Some((d.system_address, &d.system_name)),
            EDLogEvent::Location(d) => Some((d.system_address, &d.star_system)),
            EDLogEvent::FSDJump(d) => Some((d.system_address, &d.star_system)),
            _ => None,
        };
        if let Some((id, name)) = found_id {
            if name == system_name {
                return Some(id);
            }
        }
    }
    None
}

struct SystemData {
    system_address: u64,
    system_name: String,
    number_of_jumps: usize,
    first_jump: Option<EDLogLine>,
    last_jump: Option<EDLogLine>,
    last_allegiance: Option<Allegiance>,
    first_powerplay: Option<Powers>,
    last_powerplay: Option<Powers>,
}

fn collect_system_data(
    lines: &[EDLogLine],
    system_name: &str,
    system_address: u64,
) -> Option<SystemData> {
    println!("We have {} lines to extract data from", lines.len());

    let mut jumps = lines
        .iter()
        .filter(|line| matches!(line.event(), EDLogEvent::FSDJump(_)));

    let number_of_jumps = jumps.clone().count();
    let first_jump = jumps.next().cloned();
    let last_jump = if let Some(data) = jumps.next_back() {
        Some(data.clone())
    } else {
        first_jump.clone()
    };

    let mut locations = lines
        .iter()
        .filter(|line| matches!(line.event(), EDLogEvent::Location(_)));
    let first_loc = locations.next().cloned();
    let last_loc = if let Some(data) = locations.next_back() {
        Some(data.clone())
    } else {
        first_loc.clone()
    };

    let last_allegiance = last_loc
        .as_ref()
        .and_then(|l| {
            l.extract::<EDLogLocation>()
                .map(|loc| loc.system_allegiance)
        })
        //
        // look at lasts jump if last location does not mention allegiance
        //
        .or_else(|| {
            last_jump
                .as_ref()
                .and_then(|l| l.extract::<EDLogFSDJump>().map(|j| j.system_allegiance))
        });

    let first_powerplay = match (first_loc.as_ref(), first_jump.as_ref()) {
        (None, None) => None,
        (None, Some(l)) => l
            .extract::<EDLogFSDJump>()
            .and_then(|l| l.powerplay.clone()),
        (Some(l), None) => l.extract::<EDLogLocation>().and_then(|l| l.powers.clone()),
        (Some(ll), Some(lf)) => {
            if ll.timestamp() < lf.timestamp() {
                lf.extract::<EDLogFSDJump>()
                    .and_then(|l| l.powerplay.clone())
            } else {
                ll.extract::<EDLogLocation>().and_then(|l| l.powers.clone())
            }
        }
    };

    let last_powerplay = match (last_loc.as_ref(), last_jump.as_ref()) {
        (None, None) => None,
        (None, Some(l)) => l
            .extract::<EDLogFSDJump>()
            .and_then(|l| l.powerplay.clone()),
        (Some(l), None) => l.extract::<EDLogLocation>().and_then(|l| l.powers.clone()),
        (Some(ll), Some(lf)) => {
            if ll.timestamp() > lf.timestamp() {
                ll.extract::<EDLogLocation>().and_then(|l| l.powers.clone())
            } else {
                lf.extract::<EDLogFSDJump>()
                    .and_then(|l| l.powerplay.clone())
            }
        }
    };

    Some(SystemData {
        system_address,
        system_name: system_name.to_string(),
        first_jump,
        last_jump,
        number_of_jumps,
        last_allegiance,
        first_powerplay,
        last_powerplay,
    })
}

fn pp_info(pp_data: &Powers) -> String {
    format!(
        "{:?}\n{}\n{}",
        pp_data.powerplay_state,
        pp_data
            .controlling_power
            .as_ref()
            .map(|p| p.to_string())
            .unwrap_or("None".to_string()),
        pp_data.powers.join("\n")
    )
}

fn pp_conflict(pp_data: &Powers) -> String {
    if let Some(conflict) = pp_data.powerplay_conflict_progress.as_ref() {
        conflict
            .iter()
            .map(|c| format!("{} - {:.1}%", c.power, c.conflict_progress * 100.0,))
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        "None".to_string()
    }
}

fn pp_control(pp_data: &Powers) -> String {
    let progress = if let Some(progress) = pp_data.powerplay_state_control_progress.as_ref() {
        format!("{:.1}%", *progress * 100.0)
    } else {
        "n/a".to_string()
    };

    let reinforcement = if let Some(reinformcement) = pp_data.powerplay_state_reinforcement {
        format!("{}", reinformcement)
    } else {
        "n/a".to_string()
    };

    let undermining = if let Some(undermining) = pp_data.powerplay_state_undermining {
        format!("{}", undermining)
    } else {
        "n/a".to_string()
    };

    format!("Progress: {progress}\nReinforcement: {reinforcement}\nUndermining: {undermining}")
}

fn show_system_data(system_data: Option<SystemData>, system_name: &str) {
    if system_data.is_none() {
        println!("No data found for {system_name}");
    }
    let data = system_data.unwrap();

    let mut table = Table::new();
    table.add_row(row![
        "System Name\nSystem Address",
        format!("{}\n{}", data.system_name, data.system_address)
    ]);
    table.add_row(row![
        "Number of Visits\nFirst Visit\nLast Visit",
        format!(
            "{}\n{}\n{}",
            data.number_of_jumps,
            data.first_jump
                .map(|j| j.timestamp().to_string())
                .unwrap_or("n/a".to_string()),
            data.last_jump
                .map(|j| j.timestamp().to_string())
                .unwrap_or("n/a".to_string()),
        )
    ]);
    table.add_row(row![
        "Last known allegiance",
        data.last_allegiance
            .map(|a| format!("{:?}", a))
            .unwrap_or("n/a".to_string())
    ]);

    let pp = if let Some(pp_data) = data.last_powerplay {
        let mut pp = Table::new();
        let mut header = row!("", "Last data");
        let mut info = row![
            "Powerplay state\nControlling Power\nCompeting powers",
            pp_info(&pp_data)
        ];
        let mut conflict = row!["Conflict", pp_conflict(&pp_data)];
        let mut control = row!["Control", pp_control(&pp_data)];

        if let Some(pp_data) = data.first_powerplay {
            header.add_cell(cell!("First visit"));
            info.add_cell(cell!(pp_info(&pp_data)));
            conflict.add_cell(cell!(pp_conflict(&pp_data)));
            control.add_cell(cell!(pp_control(&pp_data)));
        }

        pp.add_row(header);
        pp.add_row(info);
        pp.add_row(conflict);
        pp.add_row(control);
        let mut format = *format::consts::FORMAT_NO_BORDER;
        format.padding(1, 0);
        pp.set_format(format);
        cell!(pp)
    } else {
        cell!("n/a")
    };
    table.add_row(row!("Powerplay", pp));

    table.printstd();
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!(
            "Usage: {} </path/to/log/files> <system_name>",
            env!("CARGO_PKG_NAME")
        );
        exit(-1);
    }
    let path = &args[1];
    let system_name = &args[2];

    println!("Reading log files");
    let db = read_logs(path).unwrap();

    println!("Looking for {system_name}");
    let id = if let Some(id) = find_system_address(&db, system_name) {
        println!("Found {system_name} to have address {id}");
        id
    } else {
        println!("Did not find data on system {system_name}");
        exit(-1);
    };

    println!("Filtering and sorting relevant lines");
    let system_items = filter_loglines(db, id).unwrap();

    println!("Collecting data of system {system_name}");
    let system_data = collect_system_data(&system_items, system_name, id);

    println!("Show collected data");
    show_system_data(system_data, system_name);
}
