// src/utils/logger.rs
//! Logging-related utilities

use anyhow::Result;
use crate::{
    comms::out::erm,
    globals::config::CONFIG, shell::fs::mkdir,
};
use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Config, Appender, Root, Logger as L4L},
    encode::pattern::PatternEncoder,
};
use regex::Regex;
use std::{
    collections::VecDeque,
    fs::{File, OpenOptions as OO},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    str::FromStr,
    sync::{LazyLock, Once}
};
use super::fail::Fail;

const MASTER_LOG: &str = "/tmp/2/master.log";
static LOG_INIT: Once = Once::new();
/// # Description
/// Regex pattern for matching against 2's logs
static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{3} (TRACE|DEBUG|INFO |WARN |ERROR) \| \[.+").fail("Invalid regex"));

/// # Description
/// Retrieve the log level
///
/// The log level may be specified through the ``LOG_LEVEL`` environment variable or through the
/// config. If it's unset, it defaults to trace.
///
/// Trace is the recommended log level. I really should have used custom ones but whatever.
fn get_log_level() -> LevelFilter {
    let log_level = std::env::var("LOG_LEVEL");
    let log_level = log_level.as_deref().unwrap_or(&CONFIG.general.log_level);

    LevelFilter::from_str(log_level).unwrap_or_else(|_| {
        if !log_level.is_empty() {
            let msg = format!("Invalid log level '{log_level}'; defaulting to trace");
            erm!("{msg}");
            log::warn!("{msg}");
        }
        LevelFilter::Trace
    })
}

pub fn init() {
    LOG_INIT.call_once(|| {
        let log_file = PathBuf::from(MASTER_LOG);
        let log_dir = log_file.parent().fail("Log file has no parent?");
        mkdir(log_dir).fail("Failed to create log dir");

        OO::new().create(true).append(true)
            .open(&log_file)
            .fail(&format!("Failed to open {log_file:?}"));

        let config = build_config().fail("Failed to build initial config");
        log4rs::init_config(config).fail("Failed to initialize logger");
    });
}

/// # Description
/// Builds the logger config
fn build_config() -> Result<Config> {
    // https://docs.rs/log4rs/1.0.0/log4rs/encode/pattern/index.html#formatters
    let pattern = "{({d(%Y-%m-%d %H:%M:%S.%3f)} {({l}):5.5} | [{M}@{L}]):64.64} ~ {m}{n}";

    let master_log_file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .append(true)
        .build(MASTER_LOG)?;

    let config_builder = Config::builder()
        .appender(Appender::builder().build("master", Box::new(master_log_file)))
        // tell ureq to stfu
        .logger(L4L::builder().build("rustls::webpki::server_verifier", LevelFilter::Info))
        .logger(L4L::builder().build("rustls::client",      LevelFilter::Info))
        .logger(L4L::builder().build("rustls::conn",        LevelFilter::Info))
        .logger(L4L::builder().build("ureq::pool",          LevelFilter::Info))
        .logger(L4L::builder().build("ureq::tls",           LevelFilter::Info))
        .logger(L4L::builder().build("ureq::unversioned",   LevelFilter::Warn))
        .logger(L4L::builder().build("ureq_proto::util",    LevelFilter::Info))
        .logger(L4L::builder().build("ureq_proto::client",  LevelFilter::Info));

    Ok(config_builder.build(Root::builder().appender("master").build(get_log_level()))?)
}

// /// # Description
// /// Retrieve the logger object
// ///
// /// ```rust
// /// // Example usage
// /// logger::get().detach()
// /// ```
// pub fn get<'s>() -> &'s Logger {
//     LOGGER.get().fail("Logger not initialized (my bad)")
// }

// DISPLAYING LOGS

/// # Description
/// Struct for log entries
/// Delimited by the entry regex
#[derive(Debug)]
struct LogEntry {
    level: LevelFilter,
    message: String,
}

impl LogEntry {
    /// # Description
    /// Returns a formatted log message based on the log level
    fn color(&self) -> String {
        let message = &self.message;
        match self.level {
            LevelFilter::Trace => format!("\x1b[0m{}{message}", CONFIG.message.stdout .trim()),
            LevelFilter::Debug => format!("\x1b[0m{}{message}", CONFIG.message.verbose.trim()),
            LevelFilter::Info  => format!("\x1b[0m{}{message}", CONFIG.message.message.trim()),
            LevelFilter::Warn  => format!("\x1b[0m{}{message}", CONFIG.message.prompt .trim()),
            LevelFilter::Error => format!("\x1b[0m{}{message}", CONFIG.message.danger .trim()),
            LevelFilter::Off   => message.to_string(),
        }
    }
}

/// # Description
/// Collects all logs from a log file's bufreader
/// Ignores invalid lines
/// Panics if a log entry is missing a log level
fn collect_logs<R: BufRead>(reader: R)-> VecDeque<LogEntry> {
    reader
        .lines()
        .map_while(Result::ok)
        .fold((VecDeque::new(), String::new()), |(mut logs, mut curr), line| {
            if RE.is_match(&line) && !curr.is_empty() {
                logs.push_back(LogEntry {
                    level: extract_log_level(&curr).unwrap_or(LevelFilter::Warn),
                    message: curr.clone(),
                });
                curr.clear();
            }
            curr.push_str(&line);
            curr.push('\n');
            (logs, curr)
        })
        .0
    // let mut logs = VecDeque::new();
    // let mut curr = String::new();
    //
    // for line in reader.lines().map_while(Result::ok) {
    //     if RE.captures(&line).is_some()
    //     && !curr.is_empty() {
    //             logs.push_back(curr.clone());
    //             curr.clear();
    //     }
    //
    //     curr.push_str(&line);
    //     curr.push('\n');
    // }
    //
    // if !curr.is_empty() {
    //     logs.push_back(curr);
    // }
    //
    // logs.into_iter()
    //     .filter_map(|entry| {
    //         extract_log_level(&entry)
    //             .map(|level| LogEntry { level, message: entry })
    //     })
    //     .collect()
}

/// # Description
/// Displays formatted logs for a log file
pub fn display(log_file: &Path) {
    let log_level = get_log_level();
    let f = File::open(log_file).fail("Failed to open file");
    let reader = BufReader::new(f);
    let mut log_entries = collect_logs(reader);
    log_entries.iter_mut()
        .filter(|e| e.level <= log_level)
        .for_each(|e| {
            e.message = e.color();
            print!("{}", e.message);
        });
}

/// # Description
/// Parses the log level for a line from a log file
fn extract_log_level(entry: &str) -> Option<LevelFilter> {
    match entry {
        e if e.contains(" TRACE ") => Some(LevelFilter::Trace),
        e if e.contains(" DEBUG ") => Some(LevelFilter::Debug),
        e if e.contains(" INFO  ") => Some(LevelFilter::Info),
        e if e.contains(" WARN  ") => Some(LevelFilter::Warn),
        e if e.contains(" ERROR ") => Some(LevelFilter::Error),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::erm;

    use super::{display, MASTER_LOG};

    // return result for test skipping
    #[test]
    fn display_master_log() {
        let master_log = PathBuf::from(MASTER_LOG);
        if !master_log.exists() {
            erm!("Skipping test: master_log doesn't exist");
        }

        display(&master_log);
    }
}
