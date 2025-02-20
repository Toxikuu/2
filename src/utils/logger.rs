// src/utils/logger.rs
//! Logging-related utilities

use anyhow::Result;
use crate::{
    comms::out::erm,
    globals::config::CONFIG,
};
use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Config, Appender, Root, Logger as L4L},
    encode::pattern::PatternEncoder,
    Handle,
};
use regex::Regex;
use std::{
    collections::VecDeque,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    str::FromStr,
    sync::{LazyLock, Mutex, Once, OnceLock}
};
use super::fail::Fail;

static LOGGER: OnceLock<Logger> = OnceLock::new();
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
            erm!("{}", msg);
            log::warn!("{}", msg);
        }
        LevelFilter::Trace
    })
}

/// # Description
/// The logger struct
///
/// Takes the master log file path (/var/log/2/master.log)
/// Optionally takes a relative path (for attaching to per-package logs)
/// Optionally takes a handle (for refreshing the config)
#[derive(Debug)]
pub struct Logger {
    relpath: Mutex<Option<String>>,
    handle: Mutex<Option<Handle>>,
    master: PathBuf,
}

impl Logger {
    pub fn new(master: impl Into<PathBuf>) -> Self {
        Self {
            relpath: Mutex::new(None),
            handle: Mutex::new(None),
            master: master.into(),
        }
    }

    /// # Description
    /// Attaches the logger to a specific package
    ///
    /// This writes logs to that package's log file, located at ``$PORT/.logs/pkg.log`` and to the
    /// master log
    pub fn attach(&self, relpath: &str) {
        *self.relpath.lock().fail("Failed to lock relpath mutex") = Some(relpath.to_string());
        self.refresh().fail("Failed to refresh logger");
        log::debug!("Log attached");
    }

    /// # Description
    /// Detaches the logger from any package
    ///
    /// Logs will only write to the master log when detached
    pub fn detach(&self) {
        *self.relpath.lock().fail("Failed to lock relpath mutex") = None;
        self.refresh().fail("Failed to refresh logger");
        log::debug!("Log detached");
    }

    /// # Description
    /// Initializes the logger
    pub fn init(&self) {
        LOG_INIT.call_once(|| {
            let config = self.build_config().fail("Failed to build initial config");
            let handle = log4rs::init_config(config).fail("Failed to initialize logger");
            *self.handle.lock().fail("Failed to lock handle mutex") = Some(handle);
        });
    }

    /// # Description
    /// Builds the logger config
    fn build_config(&self) -> Result<Config> {
        let log_dir = Path::new("/var/log/2");
        if !log_dir.exists() {
            fs::create_dir_all(log_dir)?;
        }

        let master_log = self.master.clone();

        // https://docs.rs/log4rs/1.0.0/log4rs/encode/pattern/index.html#formatters
        let pattern = "{({d(%Y-%m-%d %H:%M:%S.%3f)} {({l}):5.5} | [{M}@{L}]):64.64} ~ {m}{n}";

        let master_log_file = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new(pattern)))
            .append(true)
            .build(master_log)?;

        let mut config_builder = Config::builder()
            .appender(Appender::builder().build("master", Box::new(master_log_file)))
            // tell ureq to stfu
            .logger(L4L::builder().build("rustls::client",      LevelFilter::Info))
            .logger(L4L::builder().build("ureq::pool",          LevelFilter::Info))
            .logger(L4L::builder().build("ureq::tls",           LevelFilter::Info))
            .logger(L4L::builder().build("ureq::unversioned",   LevelFilter::Warn))
            .logger(L4L::builder().build("ureq_proto::util",    LevelFilter::Info))
            .logger(L4L::builder().build("ureq_proto::client",  LevelFilter::Info));

        let mut root_builder = Root::builder()
            .appender("master");

        if let Some(ref rp) = *self.relpath.lock().fail("Failed to lock relpath mutex") {
            let build_log_str = format!("/usr/ports/{rp}/.logs/pkg.log");
            let build_log = Path::new(&build_log_str);
            let log_dir = build_log.parent().fail("Broken relpath");

            fs::create_dir_all(log_dir)?;

            let build_log_file = FileAppender::builder()
                .encoder(Box::new(PatternEncoder::new(pattern)))
                .append(true)
                .build(build_log)?;

            config_builder = config_builder
                .appender(Appender::builder().build("relpath", Box::new(build_log_file)));
            root_builder = root_builder.appender("relpath");
        }

        Ok(config_builder.build(root_builder.build(get_log_level()))?)
    }

    /// # Description
    /// Refreshes the logger config
    fn refresh(&self) -> Result<()> {
        let config = self.build_config()?;
        if let Some(handle) = self.handle.lock().fail("Failed to lock handle mutex").as_ref() {
            handle.set_config(config);
        }
        Ok(())
    }
}

/// # Description
/// Retrieve the logger object
///
/// ```rust
/// // Example usage
/// logger::get().detach()
/// ```
pub fn get<'s>() -> &'s Logger {
    LOGGER.get().fail("Logger not initialized (my bad)")
}

/// # Description
/// Initialize the logger object
pub fn init(master_log: impl Into<PathBuf>) {
    let logger = Logger::new(master_log);
    LOGGER.set(logger).fail("Logger was already initialized");
    LOGGER.get().fail("Failed to access logger instance").init();
}


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
    let mut logs = VecDeque::new();
    let mut curr = String::new();

    for line in reader.lines().map_while(Result::ok) {
        if RE.captures(&line).is_some()
        && !curr.is_empty() {
                logs.push_back(curr.clone());
                curr.clear();
        }

        curr.push_str(&line);
        curr.push('\n');
    }

    if !curr.is_empty() {
        logs.push_back(curr);
    }

    logs.into_iter()
        .filter_map(|entry| {
            extract_log_level(&entry)
                .map(|level| LogEntry { level, message: entry })
        })
        .collect()
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
    [
        (" TRACE ", LevelFilter::Trace),
        (" DEBUG ", LevelFilter::Debug),
        (" INFO  ", LevelFilter::Info),
        (" WARN  ", LevelFilter::Warn),
        (" ERROR ", LevelFilter::Error)
    ]
        .iter()
        .find_map(|(tag, level)| entry.contains(tag).then_some(*level))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::erm;

    use super::display;

    // return result for test skipping
    #[test]
    fn display_master_log() {
        let master_log = PathBuf::from("/var/log/2/master.log");
        if !master_log.exists() {
            erm!("Skipping test: master_log doesn't exist");
        }

        display(&master_log);
    }
}
