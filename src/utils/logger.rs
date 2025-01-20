// src/utils/logger.rs

use anyhow::Result;
use crate::globals::config::CONFIG;
use crate::comms::log::erm;
use log4rs::{
    append::file::FileAppender,
    encode::pattern::PatternEncoder,
    config::{Config, Appender, Root},
    Handle,
};
use std::fs;
use std::io::{self, Write};
use std::path::{PathBuf, Path};
use std::str::FromStr;
use std::sync::{Mutex, Once, OnceLock};
use super::fail::Fail;

static LOGGER: OnceLock<Logger> = OnceLock::new();
static LOG_INIT: Once = Once::new();

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

    pub fn attach(&self, relpath: &str) {
        *self.relpath.lock().ufail("Failed to lock relpath mutex") = Some(relpath.to_string());
        self.refresh().ufail("Failed to refresh logger");
    }

    pub fn detach(&self) {
        *self.relpath.lock().ufail("Failed to lock relpath mutex") = None;
        self.refresh().ufail("Failed to refresh logger");
    }

    pub fn init(&self) {
        LOG_INIT.call_once(|| {
            let config = self.build_config().ufail("Failed to build initial config");
            let handle = log4rs::init_config(config).ufail("Failed to initialize logger");
            *self.handle.lock().ufail("Failed to lock handle mutex") = Some(handle);
        });
    }

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

    fn refresh(&self) -> Result<()> {
        let config = self.build_config()?;
        if let Some(handle) = self.handle.lock().ufail("Failed to lock handle mutex").as_ref() {
            handle.set_config(config);
        }
        Ok(())
    }
}

pub fn get<'s>() -> &'s Logger {
    LOGGER.get().ufail("Logger not initialized (my bad)")
}

pub fn init(master_log: impl Into<PathBuf>) {
    let logger = Logger::new(master_log);
    LOGGER.set(logger).ufail("Logger already initialized (my bad)");
    LOGGER.get().ufail("Failed to access logger instance").init();
}

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
