// src/upstream/core.rs
//! Core logic for --upstream

use std::fs;
use anyhow::{bail, Result};
use std::process::Command;
use crate::globals::config::CONFIG;
use crate::package::Package;
use crate::utils::fail::{fail, Fail};
use crate::comms::log::{pr, erm};
use serde::Deserialize;

/// # Description
/// The upstream version config, generated from .uv.toml by ``read_uv_toml()``
///
/// Upstream represents the upstream url the command will check (unless empty)
/// If command is specified, it is evaluated; if not, it uses a reasonable default
/// Bleeding is a toggle for tracking commit-level versions
#[derive(Deserialize, Debug)]
pub struct UVConfig {
    #[serde(default)]
    upstream: String,
    #[serde(default)]
    command: String,
    #[serde(default)]
    bleeding: bool,
}

/// # Description
/// The conveniently named ``sex()`` is short for static execution. It takes a command and captures
/// its output without printing that output or doing any thread shenanigans.
pub fn sex(command: &str) -> Result<String> {
    let output = Command::new("bash")
        .arg("-c")
        .arg(command).output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        log::error!("{}", error);

        bail!("Command failed with status: {}", output.status);
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// # Description
/// Generates ``UVConfig`` from .uv.toml
fn read_uv_toml(package: &Package) -> UVConfig {
    let relpath = &package.relpath;
    let uv_path = format!("/usr/ports/{relpath}/.uv.toml");

    let contents = fs::read_to_string(uv_path).unwrap_or_default();
    let cc: UVConfig = toml::de::from_str(&contents).ufail("how lmao");

    // fallback to package upstream
    if cc.upstream.is_empty() && cc.command.is_empty() {
        fail!("Invalid uv config!");
    }

    cc
}

/// # Description
/// Runs the command specified in .uv.toml
///
/// If no command is provided, runs a default command
fn run_command(cc: &UVConfig) -> Result<String> {
    if !cc.command.is_empty() {
        return sex(&cc.command);
    }

    let command =
    if cc.bleeding {
        format!("git ls-remote {} HEAD | awk '{{print $1}}'", cc.upstream)
    }
    else {
        format!("git ls-remote --tags --refs {} | sed 's|.*/||' | grep -Ev 'rc|dev|beta|alpha' | sort -V | tail -n1", cc.upstream)
    };

    sex(&command)
}

/// # Description
/// Strips the package name from a git tag (output from ``run_command()``)
/// Also strips out the 'v' prefix
fn extract_version(stdout: &str, package: &Package) -> String {
    let name = &package.name;
    let stdout = stdout.trim_start_matches('v');

    if stdout.contains(name) {
        stdout.replace(name, "")
    }
    else {
        stdout.to_string()
    }
}

/// # Description
/// High level retrieval of a package's upstream version
fn get_version(package: &Package) -> String {
    let cc = read_uv_toml(package);
    let stdout = run_command(&cc).unwrap_or_default();
    let stdout = stdout.trim();

    // the bleeding hashes don't need extracting
    if cc.bleeding {
        return stdout.to_string()
    }

    extract_version(stdout, package)
}

/// # Description
/// Handles displaying local vs upstream package versions for a package
fn display_version(package: &Package, version: &str) {
    let name = &package.name;
    let v = &package.version;
    
    if version.is_empty() {
        erm!("{name} | Failed to get version :(");
    }

    let width = 24 - name.len();
    let second_half = format_second_half(v, version);
    pr!("{name} {:<width$} | {second_half}", " ");
}

/// # Description
/// Formats the second half of the upstream version check display
fn format_second_half(v: &str, version: &str) -> String {
    if v == version {
        format!("{v} ~ {version}")
    } else {
        format!("{v} ~ \x1b[{}{version}\x1b[0m", CONFIG.message.danger.trim())
    }
}

/// # Description
/// High level function for checking and displaying upstream package versions
pub fn upstream(package: &Package) {
    let version = get_version(package);
    display_version(package, &version);
}
