// src/globals/config.rs
//! Defines 2's config

use lazy_static::lazy_static;
use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub flags: FlagsConfig,
    pub message: MessageConfig,
    pub startup: StartupConfig,
    pub removal: RemovalConfig,
    pub general: GeneralConfig,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Deserialize, Debug)]
pub struct GeneralConfig {
    pub prefix: String,
    pub clean_after_build: bool,
    pub show_bug_report_message: bool,
    pub check_hashes: bool,
    pub auto_ambiguity: bool,
}

#[derive(Deserialize, Debug)]
pub struct RemovalConfig {
    pub remove_sources: bool,
    pub remove_dots: bool,
}

#[derive(Deserialize, Debug)]
pub struct FlagsConfig {
    pub force: bool,
    pub quiet: bool,
    pub verbose: bool,
}

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

#[derive(Deserialize, Debug)]
pub struct StartupConfig {
    pub splash: String, // path to a text file to be displayed
    pub auto_prune: bool,
    pub auto_sync: bool,
}

impl Config {
    pub fn load() -> Result<Self> {
        let content = fs::read_to_string("/etc/2/config.toml")?;
        let config: Self = toml::from_str(&content)?;

        Ok(config)
    }
}

lazy_static! {
    pub static ref CONFIG: Arc<Config> = Arc::new(
        Config::load().expect("Failed to load /etc/2/config.toml")
    );
}
