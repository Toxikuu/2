// src/shell/cmd.rs
//! Defines functions for sending commands through bash

use crate::{
    globals::{config::CONFIG, flags::Flags},
    utils::fail::Fail,
};
use anyhow::{Context, Result, bail};
use std::{
    fs::OpenOptions as OO,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
    process::{Command, Stdio},
    thread,
};

/// # Description
/// Executes a command
///
/// Sources /usr/share/2/envs/core
///
/// Prints each line unless quiet is passed
///
/// **Fail conditions:**
/// - command failed
/// - bash wasn't found
/// - failed to source /usr/share/2/envs/core
/// - some sync shenanigans (unlikely)
/// - failing to read stderr/stdout (unlikely)
pub fn exec(command: &str, log: Option<PathBuf>) -> Result<()> {
    let quiet = Flags::grab().quiet;

    let mut child = Command::new("bash")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to spawn bash")?;

    let stdout = child.stdout.take().context("Stdout already taken?")?;
    let stderr = child.stderr.take().context("Stderr already taken?")?;

    let log_clone = log.clone();
    let stdout_thread = thread::spawn(move || {
        let mut reader = BufReader::new(stdout);
        let mut writer = log.as_ref().map(|log| {
            let f = OO::new()
                .create(true)
                .append(true)
                .open(log)
                .fail("Failed to open log file");
            BufWriter::new(f)
        });

        let mut buf = String::new();
        while reader.read_line(&mut buf).fail("Failed to read stdout") > 0 {
            let line = buf.trim_end();
            let msg = format!("{}{line}\x1b[0m", CONFIG.message.stdout);

            // write an unformatted line to the log
            if let Some(ref mut w) = writer {
                writeln!(w, "[STDOUT] {line}").fail("Failed to write to log file");
            }

            if !quiet {
                println!("{msg}");
            }

            buf.clear();
        }
    });

    let stderr_thread = thread::spawn(move || {
        let mut reader = BufReader::new(stderr);
        let mut writer = log_clone.as_ref().map(|log| {
            let f = OO::new()
                .create(true)
                .append(true)
                .open(log)
                .fail("Failed to open log file");
            BufWriter::new(f)
        });

        let mut buf = String::new();
        while reader.read_line(&mut buf).fail("Failed to read stderr") > 0 {
            let line = buf.trim_end();
            let msg = format!("{}{line}\x1b[0m", CONFIG.message.stderr);

            // write an unformatted line to the log
            if let Some(ref mut w) = writer {
                writeln!(w, "[STDERR] {line}").fail("Failed to write to log file");
            }

            if !quiet {
                println!("{msg}");
            }

            buf.clear();
        }
    });

    let status = child.wait()?;
    if !status.success() {
        log::debug!("Command failed");
        bail!("Command failed");
    }

    stdout_thread
        .join()
        .fail("Failed to join the stdout thread");
    stderr_thread
        .join()
        .fail("Failed to join the stderr thread");

    Ok(())
}

/// # Description
/// Executes a command in the context of a package
///
/// This context is just sourcing ``$PORT/BUILD`` and setting environment variables
#[macro_export]
macro_rules! pkgexec {
    ($cmd:expr, $pkg:expr) => {{
        use $crate::globals::flags::Flags;
        use $crate::shell::cmd::exec;

        let debug = if Flags::grab().verbose { 'x' } else { ' ' };
        let command = format!(
            r#"
        set -e{}
        source /usr/share/2/envs/core || exit 211

        export PORT={:?}
        export SRC="$PORT/.sources"
        export BLD="$PORT/.build"
        export D="$BLD/D"

        source "$PORT/BUILD" || exit 211

        {}
        "#,
            debug, $pkg.data.port_dir, $cmd,
        );

        let build_log = $pkg.data.port_dir.join(".logs/build.log");
        exec(&command, Some(build_log))
    }};
}

pub(crate) use pkgexec;
