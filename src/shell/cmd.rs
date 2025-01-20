// src/shell/cmd.rs
//
// defines command functions

use anyhow::{Result, Context, bail};
use crate::comms::log::{erm, cpr};
use crate::globals::config::CONFIG;
use crate::utils::fail::Fail;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;

pub fn exec(command: &str) -> Result<()> {
    // initialize the bash environment
    let command = format!(
    r"
    source /usr/share/2/bin/e-core || exit 211
    {command}
    "
    );

    let mut child = Command::new("bash")
        .arg("-c")
        .arg(&command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed spawn bash")?;

    let stdout = child.stdout.take().context("Stdout already taken?")?;
    let stderr = child.stderr.take().context("Stderr already taken?")?;

    let stdout_thread = thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    cpr!("{}", line);
                    log::trace!("{}", line);
                }
                Err(e) => erm!("Error reading stdout: {}", e),
            }
        }
    });

    let stderr_thread = thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    let msg = format!("\x1b[{}{line}", CONFIG.message.stderr);
                    cpr!("{}", msg);
                    log::trace!("{}", msg);
                }
                Err(e) => erm!("Error reading stderr: {}", e),
            }
        }
    });

    let status = child.wait()?;
    if !status.success() {
        log::debug!("Command failed");
        bail!("Command failed");
    }

    stdout_thread.join().ufail("Failed to join the stdout thread");
    stderr_thread.join().ufail("Failed to join the stderr thread");

    Ok(())
}

#[macro_export]
macro_rules! pkgexec {
    ($cmd:expr, $pkg:expr) => {{
        use $crate::shell::cmd::exec;
        let relpath = &$pkg.relpath;
        let command = format!(
        r#"
        export PORT="/usr/ports/{}"
        export SRC="$PORT/.sources"
        export BLD="$PORT/.build"
        export D="$BLD/D"

        source "$PORT/BUILD" || exit 211

        {}
        "#,
        relpath,
        $cmd,
        );

        exec(&command)
    }};
}

pub(crate) use pkgexec;
