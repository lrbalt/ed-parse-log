use chrono::Local;
use ed_parse_log_files::log_line::EDLogLine;
use numfmt::{Formatter, Precision, Scales};
use rayon::prelude::*;
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
enum MyError {
    #[error("IO Error")]
    IOError(#[from] std::io::Error),
    #[error("Parse Error {0}")]
    ParseError(#[from] serde_json::Error),
}

#[allow(clippy::to_string_in_format_args)]
fn start() -> Result<(), std::io::Error> {
    let mut fs_formatter = Formatter::new()
        .scales(Scales::binary())
        .precision(Precision::Significance(4))
        .suffix("B")
        .unwrap();
    let mut bps_formatter = Formatter::new()
        .scales(Scales::binary())
        .precision(Precision::Significance(4))
        .suffix("B/s")
        .unwrap();
    let mut lps_formatter = Formatter::new()
        .scales(Scales::metric())
        .precision(Precision::Significance(4))
        .suffix("lines/s")
        .unwrap();
    let mut dec_formatter = Formatter::new()
        .scales(Scales::none())
        .separator('.')
        .unwrap()
        .precision(Precision::Decimals(0));
    let mut perc_formatter = Formatter::percentage()
        .comma(true)
        .precision(Precision::Significance(3));
    let mut time_formatter = Formatter::new()
        .scales(Scales::new(1000, vec!["ms", "s"]).unwrap())
        .comma(true);

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} /path/to/log/files", env!("CARGO_PKG_NAME"));
        exit(-1);
    }
    let path = &args[1];

    let errors = Mutex::new(Vec::new());
    let db: Mutex<Vec<EDLogLine>> = Mutex::new(Vec::new());

    let start = Local::now();
    println!("Starting at {start}");

    let files = read_dir(path)?
        .map(|e| e.map(|e| e.path()))
        .collect::<Result<Vec<_>, _>>()?;

    let log_files: Vec<_> = files
        .par_iter()
        .filter(|f| f.extension() == Some(OsStr::new("log")))
        .collect();

    let bytes_to_read: u64 = log_files
        .par_iter()
        .map(|f| {
            File::open(f)
                .and_then(|f| f.metadata())
                .map(|m| m.len())
                .unwrap_or(0)
        })
        .sum();

    println!(
        "Reading log from {} containing {} entries of which {} are log files containing {} bytes",
        path,
        dec_formatter.fmt2(files.len() as f64).to_string(),
        dec_formatter.fmt2(log_files.len() as f64),
        fs_formatter.fmt2(bytes_to_read as f64),
    );

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
    let lines = db.lock().unwrap();

    let end = Local::now();
    println!("Completed at {end}");
    let delta = end - start;
    println!(
        "Reading al logs took {}",
        time_formatter.fmt2(delta.num_milliseconds() as f64)
    );
    let total = e.len() + lines.len();
    println!(
        "Read {} lines of which {} had parsing errors: {}",
        dec_formatter.fmt2(total as f64).to_string(),
        dec_formatter.fmt2(e.len() as f64),
        perc_formatter.fmt2(e.len() as f64 / total as f64),
    );
    let lps = total as f64 / delta.num_milliseconds() as f64 * 1000.0;
    let bps = bytes_to_read as f64 / delta.num_milliseconds() as f64 * 1000.0;
    println!(
        "Performance: {} and {}",
        lps_formatter.fmt2(lps).to_string(),
        bps_formatter.fmt2(bps),
    );

    for (index, error) in e.iter().take(5).enumerate() {
        eprintln!(
            "\nError #{index}: {}\n\nin file {}:{} at line {}\n{}",
            error.0, error.1, error.2, error.3, error.4,
        );
    }

    Ok(())
}

fn main() {
    start().unwrap();
}
