// src/globals/config.rs
//! Defines 2's config

use anyhow::{Result, Context};
use crate::utils::fail::Fail;
use serde::Deserialize;
use std::{
    fs,
    sync::{Arc, LazyLock},
};

/// # Description
/// The config struct
///
/// Includes options for customizing 2
#[derive(Deserialize, Debug)]
pub struct Config {
    pub flags: FlagsConfig,
    pub message: MessageConfig,
    pub removal: RemovalConfig,
    pub general: GeneralConfig,
    #[cfg(feature = "upstream")]
    pub upstream: UpstreamConfig,
}

/// # Description
/// Part of the config struct
///
/// General config options
#[allow(clippy::struct_excessive_bools)]
#[derive(Deserialize, Debug)]
pub struct GeneralConfig {
    pub prefix: String,
    pub clean_after_build: bool,
    pub show_bug_report_message: bool,
    pub check_hashes: bool,
    pub auto_ambiguity: bool,
    pub log_level: String,
    pub prune_manifests: bool,
    pub prune_logs: bool,
}

/// # Description
/// Part of the config struct
///
/// Config options for package removal
#[derive(Deserialize, Debug)]
pub struct RemovalConfig {
    pub remove_sources: bool,
    pub remove_dots: bool,
}

/// # Description
/// Part of the config struct
///
/// Config options for 2 flags
#[derive(Deserialize, Debug)]
pub struct FlagsConfig {
    pub force: bool,
    pub quiet: bool,
    pub verbose: bool,
}

/// # Description
/// Part of the config struct
///
/// Config options for message formatting. Uses ansi escape codes. \x1b[ is implied, with the code
/// itself belonging to the user. The terminating m is not implied.
#[derive(Deserialize, Debug)]
pub struct MessageConfig {
    pub danger: String,
    pub default: String,
    pub message: String,
    pub prompt: String,
    pub stderr: String,
    pub stdout: String,
    pub verbose: String,
}

/// # Description
/// Part of the config struct
///
/// Config options for upstream version checking
#[cfg(feature = "upstream")]
#[derive(Deserialize, Debug)]
pub struct UpstreamConfig {
    pub max_threads: usize,
    pub stack_size: usize,
    pub retries: usize,
}

impl Config {
    /// # Description
    /// Loads the config from /etc/2/config.toml
    ///
    /// Returns an error if the config is invalid or doesn't exist
    pub fn load() -> Result<Self> {
        let content = fs::read_to_string("/etc/2/config.toml").context("Missing config")?;
        let config: Self = toml::from_str(&content).context("Invalid config")?;

        Ok(config)
    }
}

pub static CONFIG: LazyLock<Arc<Config>> = LazyLock::new(|| Arc::new(
    Config::load().fail("Failed to load /etc/2/config.toml")
));
//
// lazy_static! {
//     /// # Description
//     /// Shared config object
//     ///
//     /// It's evaluated at runtime and available across files
//     pub static ref CONFIG: Arc<Config> = Arc::new(
//         Config::load().fail("Failed to load /etc/2/config.toml")
//     );
// }
