// src/upstream/core.rs
//! Core logic for --upstream

use anyhow::{bail, Result};
use crate::{
    comms::log::{pr, erm, vpr},
    globals::config::CONFIG,
    package::Package,
};
use serde::Deserialize;
use std::process::Command;

/// # Description
/// The upstream version config, taken from Package by ``gen_cc()``
///
/// Upstream represents the upstream url the command will check (unless empty)
/// If command is specified, it is evaluated; if not, it uses a reasonable default
#[derive(Deserialize, Debug)]
pub struct UVConfig<'u> {
    #[serde(default)]
    upstream: &'u str,
    #[serde(default)]
    command: &'u str,
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
/// Generates ``UVConfig`` from the package struct
// Necessary allow as Package is not known at compile time since it's deserialized
#[allow(clippy::missing_const_for_fn)]
fn gen_cc(package: &Package) -> UVConfig {
    UVConfig {
        upstream: &package.upstream,
        command: &package.version_command,
    }
}

/// # Description
/// Runs the command specified in .uv.toml
///
/// If no command is provided, runs a default command
fn run_command(cc: &UVConfig) -> Result<String> {
    if !cc.command.is_empty() {
        return sex(cc.command);
    }

    let command = format!("git ls-remote --tags --refs {} | sed 's|.*/||' | grep -Ev 'rc|dev|beta|alpha' | sort -V | tail -n1", cc.upstream);
    sex(&command)
}

/// # Description
/// Strips the package name from a git tag (output from ``run_command()``)
/// Also strips out the 'v' prefix
fn extract_version<'a> (stdout: &'a str, package: &'a Package) -> &'a str {
    let name = &package.name;

    let namelen = name.len();
    let unnamed =
    if stdout.len() >= namelen
        && stdout[..namelen].eq_ignore_ascii_case(name)
    {
        &stdout[namelen..]
    } else {
        stdout
    };

    unnamed
        .trim_start_matches('-')
        .trim_start_matches('v')
}

/// # Description
/// High level retrieval of a package's upstream version
fn get_version(package: &Package) -> String {
    let cc = gen_cc(package);
    let stdout = run_command(&cc).unwrap_or_default();
    let stdout = stdout.trim();
    vpr!("stdout: {}", stdout);

    extract_version(stdout, package).to_string()
}

/// # Description
/// Handles displaying local vs upstream package versions for a package
fn display_version(package: &Package, version: &str) {
    let pkg = format!("{}/{}", package.repo, package.name);
    let v = &package.version;

    if version.is_empty() {
        return erm!("{pkg} | Failed to get version :(");
    }

    let width = 24 - pkg.len();
    let second_half = format_second_half(v, version);
    pr!("{pkg} {:<width$} | {second_half}", " ");
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
    let mut version = String::new();
    for _ in 0..CONFIG.upstream.retries {
        version = get_version(package);
        if !version.is_empty() {
            break
        }
    }
    display_version(package, &version);
}
