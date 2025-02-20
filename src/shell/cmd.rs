// src/shell/cmd.rs
//! Defines functions for sending commands through bash

use anyhow::{Result, Context, bail};
use crate::{
    comms::out::cpr,
    globals::config::CONFIG,
    utils::fail::Fail,
};
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    thread,
};

/// # Description
/// Executes a command
///
/// Sources /usr/share/2/bin/e-core
///
/// Prints each line unless quiet is passed
///
/// **Fail conditions:**
/// - command failed
/// - bash wasn't found
/// - failed to source /usr/share/2/bin/e-core
/// - some sync shenanigans (unlikely)
/// - failing to read stderr/stdout (unlikely)
pub fn exec(command: &str) -> Result<()> {
    // initialize the bash environment
    let command = format!(
    r"
    {command}
    "
    );

    let mut child = Command::new("bash")
        .arg("-c")
        .arg(&command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to spawn bash")?;

    let stdout = child.stdout.take().context("Stdout already taken?")?;
    let stderr = child.stderr.take().context("Stderr already taken?")?;

    let stdout_thread = thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            let line = line.fail("Failed to read stdout");

            cpr!("{}", line);
            log::trace!("{}", line);
        }
    });

    let stderr_thread = thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            let line = line.fail("Failed to read stderr");

            let msg = format!("{}{line}", CONFIG.message.stderr);
            cpr!("{}", msg);
            log::trace!("{}", msg);
        }
    });

    let status = child.wait()?;
    if !status.success() {
        log::debug!("Command failed");
        bail!("Command failed");
    }

    stdout_thread.join().fail("Failed to join the stdout thread");
    stderr_thread.join().fail("Failed to join the stderr thread");

    Ok(())
}

/// # Description
/// Executes a command in the context of a package
///
/// This context is just sourcing ``$PORT/BUILD`` and setting environment variables
#[macro_export]
macro_rules! pkgexec {
    ($cmd:expr, $pkg:expr) => {{
        use $crate::shell::cmd::exec;
        let command = format!(
        r#"
        set -e
        source /usr/share/2/envs/core || exit 211

        export PORT={:?}
        export SRC="$PORT/.sources"
        export BLD="$PORT/.build"
        export D="$BLD/D"

        source "$PORT/BUILD" || exit 211

        {}
        "#,
        $pkg.data.port_dir,
        $cmd,
        );

        exec(&command)
    }};
}

pub(crate) use pkgexec;
