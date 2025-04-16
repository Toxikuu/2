use std::{
    fs,
    sync::{
        Arc,
        LazyLock,
    },
};

use serde::Deserialize;
use tracing::warn;

fn escape_escapes(string: &str) -> String {
    string
        .replace("\\x1b", "\x1b")
        .replace("\\e", "\x1b")
        .replace("\\u001b", "\u{001b}")
}

/// # Description
/// The config struct
/// Includes options for customizing 2
#[derive(Deserialize, Debug)]
pub struct Config {
    pub flags:    FlagsConfig,
    pub message:  MessageConfig,
    pub removal:  RemovalConfig,
    pub general:  GeneralConfig,
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
    #[cfg(not(test))]
    pub show_bug_report_message: bool,
    #[cfg(not(test))]
    pub show_failure_location: bool,
    pub check_hashes: bool,
    pub auto_ambiguity: bool,
    pub log_level: String,
    pub alphabetize: bool,
}

/// # Description
/// Part of the config struct
///
/// Config options for package removal
#[derive(Deserialize, Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct RemovalConfig {
    pub remove_sources:  bool,
    pub remove_dist:     bool,
    pub prune_manifests: bool,
    pub prune_logs:      bool,
    pub prune_dist:      bool,
}

/// # Description
/// Part of the config struct
///
/// Config options for 2 flags
#[derive(Deserialize, Debug)]
pub struct FlagsConfig {
    pub force:   bool,
    pub quiet:   bool,
    pub verbose: bool,
}

/// # Description
/// Part of the config struct
///
/// Config options for message formatting. Uses ansi escape codes.
#[derive(Deserialize, Debug)]
pub struct MessageConfig {
    pub danger:  String,
    pub default: String,
    pub message: String,
    pub prompt:  String,
    pub stderr:  String,
    pub stdout:  String,
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
    pub stack_size:  usize,
    pub retries:     usize,
}

impl Config {
    /// # Description
    /// Loads the config from /etc/2/config.toml
    ///
    /// Returns an error if the config is invalid or doesn't exist
    pub fn load() -> Self {
        let content =
            fs::read_to_string("/etc/2/config.toml").expect("Missing config at /etc/2/config.toml");
        let mut config: Self = toml::from_str(&content).expect("Invalid config");

        for field in [
            &mut config.message.message,
            &mut config.message.danger,
            &mut config.message.default,
            &mut config.message.prompt,
            &mut config.message.verbose,
            &mut config.message.stderr,
            &mut config.message.stdout,
        ] {
            *field = escape_escapes(field);
        }

        config
    }
}

pub static CONFIG: LazyLock<Arc<Config>> = LazyLock::new(|| Arc::new(Config::load()));
