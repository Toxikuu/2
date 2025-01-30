// src/utils/logger.rs
//! Logging-related utilities

use anyhow::Result;
use crate::{
    comms::log::erm,
    globals::config::CONFIG,
};
use log4rs::{
    append::file::FileAppender,
    encode::pattern::PatternEncoder,
    config::{Config, Appender, Root},
    Handle,
};
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    str::FromStr,
    sync::{Mutex, Once, OnceLock},
};
use super::fail::Fail;

static LOGGER: OnceLock<Logger> = OnceLock::new();
static LOG_INIT: Once = Once::new();

/// # Description
/// Retrieve the log level
///
/// The log level may be specified through the ``LOG_LEVEL`` environment variable or through the
/// config. If it's unset, it defaults to trace.
///
/// Trace is the recommended log level. I really should have used custom ones but whatever.
fn get_log_level() -> log::LevelFilter {
    let log_level = std::env::var("LOG_LEVEL");
    let log_level = log_level.as_deref().unwrap_or(&CONFIG.general.log_level);

    log::LevelFilter::from_str(log_level).unwrap_or_else(|_| {
        if !log_level.is_empty() {
            let msg = format!("Invalid log level '{log_level}'; defaulting to trace");
            erm!("{}", msg);
            log::warn!("{}", msg);
        }
        log::LevelFilter::Trace
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
        *self.relpath.lock().ufail("Failed to lock relpath mutex") = Some(relpath.to_string());
        self.refresh().ufail("Failed to refresh logger");
    }

    /// # Description
    /// Detaches the logger from any package
    /// 
    /// Logs will only write to the master log when detached
    pub fn detach(&self) {
        *self.relpath.lock().ufail("Failed to lock relpath mutex") = None;
        self.refresh().ufail("Failed to refresh logger");
    }

    /// # Description
    /// Initializes the logger
    pub fn init(&self) {
        LOG_INIT.call_once(|| {
            let config = self.build_config().ufail("Failed to build initial config");
            let handle = log4rs::init_config(config).ufail("Failed to initialize logger");
            *self.handle.lock().ufail("Failed to lock handle mutex") = Some(handle);
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
            .appender(Appender::builder().build("master", Box::new(master_log_file)));

        let mut root_builder = Root::builder().appender("master");

        if let Some(ref rp) = *self.relpath.lock().ufail("Failed to lock relpath mutex") {
            let build_log_str = format!("/usr/ports/{rp}/.logs/pkg.log");
            let build_log = Path::new(&build_log_str);
            let log_dir = build_log.parent().ufail("Broken relpath");

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
        if let Some(handle) = self.handle.lock().ufail("Failed to lock handle mutex").as_ref() {
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
    LOGGER.get().ufail("Logger not initialized (my bad)")
}

/// # Description
/// Initialize the logger object
pub fn init(master_log: impl Into<PathBuf>) {
    let logger = Logger::new(master_log);
    LOGGER.set(logger).ufail("Logger already initialized (my bad)");
    LOGGER.get().ufail("Failed to access logger instance").init();
}

/// # Description
/// Formats the lines for logs
///
/// Used with ``display()``
fn color_lines(file: &Path) -> Result<String> {
    let mut contents = fs::read_to_string(file)?;

    contents = contents.lines().map(|l| {
             if l.contains(" TRACE ") { format!("\x1b[0;{}{l}\n", CONFIG.message.stdout .trim()) }
        else if l.contains(" DEBUG ") { format!("\x1b[0;{}{l}\n", CONFIG.message.verbose.trim()) }
        else if l.contains(" INFO  ") { format!("\x1b[0;{}{l}\n", CONFIG.message.message.trim()) }
        else if l.contains(" WARN  ") { format!("\x1b[0;{}{l}\n", CONFIG.message.prompt .trim()) }
        else if l.contains(" ERROR ") { format!("\x1b[0;{}{l}\n", CONFIG.message.danger .trim()) }
        else { format!("{l}\n") }
    }).collect();

    Ok(contents)
}

/// # Description
/// Displays formatted logs
///
/// Used with the -L flag to view a package's logs
pub fn display(file: &Path) -> Result<()> {
    let log_level = get_log_level();

    color_lines(file)?.lines().for_each(|l| {
        if let Some(level) = extract_log_level(l) {
            if level <= log_level {
                writeln!(io::stdout(), "{l}\x1b[0m").ufail("Failed to write to stdout");
            }
        }
    });

    Ok(())
}

/// # Description
/// Parses the log level for a line from a log file
fn extract_log_level(line: &str) -> Option<log::LevelFilter> {
    if line.contains(" TRACE ") {
        Some(log::LevelFilter::Trace)
    } else if line.contains(" DEBUG ") {
        Some(log::LevelFilter::Debug)
    } else if line.contains(" INFO  ") {
        Some(log::LevelFilter::Info)
    } else if line.contains(" WARN  ") {
        Some(log::LevelFilter::Warn)
    } else if line.contains(" ERROR ") {
        Some(log::LevelFilter::Error)
    } else {
        None
    }
}
