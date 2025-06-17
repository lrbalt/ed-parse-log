use ed_parse_log_files::{
    common_types::StationType,
    docking::{EDLogDocked, EDLogTechnologyBroker},
    fleet_carrier::EDLogFCMaterials,
    location::EDLogLocation,
    log_line::{EDLogEvent, EDLogLine},
    market::{EDLogColonisationConstructionDepot, EDLogMarket},
    modules::EDLogStoredModules,
    navigation::EDLogApproachSettlement,
    shipyard::EDLogStoredShips,
};
use numfmt::{Formatter, Precision, Scales};
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
            let file = match File::open(path).map_err(|e| e.into()) {
                Ok(path) => path,
                Err(e) => {
                    errors.lock().unwrap().push((
                        e,
                        file_number,
                        path.display().to_string(),
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
                            path.display().to_string(),
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

fn filter_loglines(db: Mutex<Vec<EDLogLine>>, market_id: u64) -> Result<Vec<EDLogLine>, MyError> {
    let mut lines: Vec<EDLogLine> = db
        .lock()
        .unwrap()
        .par_iter()
        .map(|line| match line.event() {
            EDLogEvent::ColonisationConstructionDepot(d) => Some((line, d.market_id)),
            EDLogEvent::Market(d) => Some((line, d.market_id)),
            EDLogEvent::Docked(d) => Some((line, d.market_id)),
            EDLogEvent::TechnologyBroker(d) => Some((line, d.market_id)),
            EDLogEvent::Location(d) => d.station_information.as_ref().map(|d| (line, d.market_id)),
            EDLogEvent::ApproachSettlement(d) => {
                d.station_information.as_ref().map(|d| (line, d.market_id))
            }
            EDLogEvent::FCMaterials(d) => Some((line, d.market_id)),
            EDLogEvent::StoredModules(d) => Some((line, d.market_id)),
            EDLogEvent::StoredShips(d) => Some((line, d.market_id)),
            _ => None,
        })
        //
        // filter on the data for the specific market
        //
        .filter(|found_market| {
            if let Some((_, found_market_id)) = found_market {
                *found_market_id == market_id
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

struct MarketData {
    ccd: Option<EDLogColonisationConstructionDepot>,
    station_name: Option<String>,
    station_type: Option<StationType>,
}

fn collect_market_data(market_items: &[EDLogLine], market_id: u64) -> MarketData {
    println!(
        "We have {} lines to extract data from for market {market_id}",
        market_items.len()
    );

    let ccd = market_items
        .iter()
        .filter(|line| matches!(line.event(), EDLogEvent::ColonisationConstructionDepot(_)))
        .next_back()
        .and_then(|l| l.extract::<EDLogColonisationConstructionDepot>())
        .cloned();

    let market = market_items
        .iter()
        .filter(|line| matches!(line.event(), EDLogEvent::Market(_)))
        .next_back()
        .and_then(|l| l.extract::<EDLogMarket>())
        .cloned();

    let station_name = market.as_ref().map(|m| &m.station_name);
    let station_type = market.as_ref().map(|m| m.station_type);

    let docked = market_items
        .iter()
        .filter(|line| matches!(line.event(), EDLogEvent::Docked(_)))
        .next_back()
        .and_then(|l| l.extract::<EDLogDocked>())
        .cloned();

    let station_name = station_name.or(docked.as_ref().map(|d| &d.station_name));
    let station_type = station_type.or(docked.as_ref().map(|m| m.station_type));

    let _techbroker = market_items
        .iter()
        .filter(|line| matches!(line.event(), EDLogEvent::TechnologyBroker(_)))
        .next_back()
        .and_then(|l| l.extract::<EDLogTechnologyBroker>())
        .cloned();

    let location = market_items
        .iter()
        .filter(|line| matches!(line.event(), EDLogEvent::Location(_)))
        .next_back()
        .and_then(|l| l.extract::<EDLogLocation>())
        .cloned();

    let station_name = station_name.or(location
        .as_ref()
        .and_then(|l| l.station_information.as_ref().map(|si| &si.station_name)));
    let station_type = station_type.or(location
        .as_ref()
        .and_then(|l| l.station_information.as_ref().map(|si| si.station_type)));

    let approach = market_items
        .iter()
        .filter(|line| matches!(line.event(), EDLogEvent::ApproachSettlement(_)))
        .next_back()
        .and_then(|l| l.extract::<EDLogApproachSettlement>())
        .cloned();

    let station_name = station_name.or(approach
        .as_ref()
        .and_then(|l| l.station_information.as_ref().map(|si| &si.station_name)));
    let station_type = station_type.or(approach
        .as_ref()
        .and_then(|l| l.station_information.as_ref().map(|si| si.station_type)));

    let _fc_mats = market_items
        .iter()
        .filter(|line| matches!(line.event(), EDLogEvent::FCMaterials(_)))
        .next_back()
        .and_then(|l| l.extract::<EDLogFCMaterials>())
        .cloned();

    let _stored_mods = market_items
        .iter()
        .filter(|line| matches!(line.event(), EDLogEvent::StoredModules(_)))
        .next_back()
        .and_then(|l| l.extract::<EDLogStoredModules>())
        .cloned();

    let _stored_ships = market_items
        .iter()
        .filter(|line| matches!(line.event(), EDLogEvent::StoredShips(_)))
        .next_back()
        .and_then(|l| l.extract::<EDLogStoredShips>())
        .cloned();

    let station_name = station_name.cloned();

    MarketData {
        ccd,
        station_name,
        station_type,
    }
}

fn show_market_data(market_data: &MarketData, market_id: u64) {
    println!("Market data for {market_id}");

    let mut dec_formatter = Formatter::new()
        .scales(Scales::none())
        .separator('.')
        .unwrap()
        .precision(Precision::Decimals(0));

    // format for inner tables
    let mut format = *format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR;
    format.padding(1, 1);

    let mut table = Table::new();
    table.add_row(row![
        "Market ID\nStation name\nStation type",
        format!(
            "{market_id}\n{}\n{}",
            market_data
                .station_name
                .as_ref()
                .unwrap_or(&"N/A".to_string()),
            market_data
                .station_type
                .map(|st| st.to_string())
                .unwrap_or("N/A".to_string())
        )
    ]);

    table.add_row(row![
        "Colonisation",
        market_data
            .ccd
            .as_ref()
            .map(|ccd| {
                if ccd.construction_complete {
                    cell!("Completed")
                } else {
                    let mut table = Table::new();
                    // format for inner tables
                    let mut format2 = *format::consts::FORMAT_NO_BORDER;
                    format2.padding(1, 0);
                    table.set_format(format2);

                    table.add_row(row![
                        "Construction progress",
                        format!("{:.2}%", ccd.construction_progress * 100.0)
                    ]);

                    let mut resources_required = Table::new();
                    resources_required.set_format(format);
                    resources_required.set_titles(row!["Resource", "Remaining", "Credit value", "Total value"]);

                    let mut resources = ccd.resources_required.clone();
                    resources.sort_by(|a, b| (a.required_amount-a.provided_amount).cmp(&(b.required_amount-b.provided_amount)).reverse());

                    let (mut sum_remaining, mut total_value) = (0,0);
                    for r in resources.iter() {
                        resources_required.add_row(row![
                            r.name_localised.as_ref().unwrap_or(&r.name),
                            r -> dec_formatter.fmt2(r.required_amount - r.provided_amount),
                            r -> dec_formatter.fmt2(r.payment.0),
                            r -> dec_formatter.fmt2(r.payment.0 * (r.required_amount - r.provided_amount) as i64)
                        ]);
                        sum_remaining += r.required_amount - r.provided_amount;
                        total_value += r.payment.0 * (r.required_amount - r.provided_amount) as i64
                    }
                    resources_required.add_row(row![]);
                    resources_required.add_row(row!["Total",r -> dec_formatter.fmt2(sum_remaining),"",r -> dec_formatter.fmt2(total_value)]);
                    table.add_row(row!["Needed resources", cell!(resources_required)]);
                    cell!(table)
                }
            })
            .unwrap_or(cell!("N/A"))
    ]);

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
    let market_id = match args[2].parse::<u64>() {
        Ok(id) => id,
        Err(_) => {
            println!("Market ID should be a number: {}", args[2]);
            exit(-1);
        }
    };

    println!("Reading log files");
    let db = read_logs(path).unwrap();

    println!("Looking for market with id {market_id}");

    println!("Filtering and sorting relevant lines");
    let market_items = filter_loglines(db, market_id).unwrap();

    println!("Collecting data of market {market_id}");
    let market_data = collect_market_data(&market_items, market_id);

    println!("Show collected data");
    show_market_data(&market_data, market_id);
}
