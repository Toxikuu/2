// src/upstream/core.rs
//! Core logic for --upstream

use anyhow::{bail, Result};
use crate::{
    comms::out::{pr, erm, vpr},
    globals::config::CONFIG,
    package::Package,
    utils::{
        fail::Fail,
        hash::{try_truncate_commit_hash, is_commit_hash}
    },
};
use log::debug;
use serde::Deserialize;
use std::{
    io::Read,
    process::{Command, Stdio},
    time::{Duration, Instant},
};

/// # Description
/// The upstream version config, taken from Package by ``gen_cc()``
///
/// Upstream represents the upstream url the command will check (unless empty)
/// If command is specified, it is evaluated; if not, it uses a reasonable default
#[derive(Deserialize, Debug)]
pub struct UVConfig<'u> {
    upstream: Option<&'u str>,
    command: Option<&'u str>,
    commit: bool,
}

/// # Description
/// The conveniently named ``sex()`` is short for static execution. It takes a command and captures
/// its output without printing that output or doing any thread shenanigans.
fn sex(command: &str, timeout: u8) -> Result<String> {
    // vpr!("Spawning static command '{command}'...");
    let start = Instant::now();
    let timeout_duration = Duration::from_secs(u64::from(timeout));

    let mut child = Command::new("bash")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    loop {
        if let Ok(Some(_)) = child.try_wait() {
            // command finished
            let mut output = String::new();
            if let Some(mut stdout) = child.stdout.take() {
                stdout.read_to_string(&mut output)?;
            }
            return Ok(output)
        }

        if start.elapsed() >= timeout_duration {
            child.kill()?;
            bail!("Command timed out")
        }
    }
}

/// # Description
/// Generates ``UVConfig`` from the package struct
fn gen_cc(package: &Package) -> UVConfig {
    UVConfig {
        upstream: package.upstream.as_deref(),
        command: package.version_command.as_deref(),
        commit: is_commit_hash(&package.version),
    }
}

/// # Description
/// Runs the command specified in .uv.toml
///
/// If no command is provided, runs a default command
fn run_command(cc: &UVConfig, timeout: u8) -> Result<String> {
    let command = cc.command.map_or_else(|| {
        let upstream = cc.upstream.fail("[UNREACHABLE] Upstream should always be some here");
        
        if cc.commit {
            format!("git ls-remote {upstream} HEAD | grep '\\sHEAD$' | cut -c1-7")
        } else {
            format!("git ls-remote --tags --refs {upstream} | sed 's|.*/||' | grep -Ev 'rc|dev|beta|alpha' | sort -V | tail -n1")
        }
    }, 
        std::string::ToString::to_string
    );

    sex(&command, timeout)
}

/// # Description
/// Strips the package name from a git tag (output from ``run_command()``)
/// Also strips out the 'v' prefix
fn extract_version<'a> (stdout: &'a str, package: &'a Package) -> &'a str {
    let name = &package.name;
    vpr!("Extracting version from '{stdout}' for '{package}'...");

    let namelen = name.len();
    let unnamed =
    if stdout.len() >= namelen
        && stdout[..namelen].eq_ignore_ascii_case(name)
    {
        &stdout[namelen..]
    } else {
        stdout
    };

    let extracted = unnamed
        .trim_start_matches('-')
        .trim_start_matches('v');

    vpr!("Extracted to '{extracted}'");
    extracted
}

/// # Description
/// High level retrieval of a package's upstream version
fn get_version(package: &Package) -> String {
    let cc = gen_cc(package);
    let stdout = run_command(&cc, 16).unwrap_or_default();
    let stdout = stdout.trim();
    vpr!("Version command stdout for {package}: {stdout}");

    extract_version(stdout, package).to_string()
}

/// # Description
/// Handles displaying local vs upstream package versions for a package
fn display_version(package: &Package, version: &str) {
    let max_pkg_len = 32;

    vpr!("Displaying version '{version}' for '{package}'...");
    let pkg = format!("{}/{}", package.repo, package.name);
    let pkg = if pkg.len() > max_pkg_len {
        format!("{}...", &pkg[..max_pkg_len])
    } else {
        pkg
    };

    let v = &package.version;

    if version.is_empty() {
        return erm!("{pkg} | Failed to get version :(");
    }

    // NOTE: If you're experiencing an OOM with upstream, width is likely the culprit. Increase the
    // value such that width isn't negative.
    let width = (max_pkg_len + 4) - pkg.len();
    let second_half = format_second_half(v, version);
    pr!("{pkg} {:<width$} | {second_half}", " ");
}

/// # Description
/// Formats the second half of the upstream version check display
fn format_second_half(v: &str, version: &str) -> String {
    let v = try_truncate_commit_hash(v);
    let version = try_truncate_commit_hash(version);

    if v == version {
        format!("{v} ~ {version}")
    } else {
        format!("{v} ~ {}{version}\x1b[0m", CONFIG.message.danger.trim())
    }
}

/// # Description
/// High level function for checking and displaying upstream package versions
pub fn check_upstream(package: &Package) {
    if package.upstream.is_none() {
        debug!("No upstream specified for '{package}'");
        return
    }

    for _ in 0..CONFIG.upstream.retries {
        let version = get_version(package);
        if version.is_empty() {
            continue
        }
        return display_version(package, &version);
    }
    erm!("Failed to check upstream version for '{package}'");
}
