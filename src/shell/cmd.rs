// src/shell/cmd.rs
//
// defines command functions

use std::io::{self, BufRead};
use std::process::{Command, Stdio};
use std::thread;
use crate::{erm, cpr};
use log::{debug, error};

pub fn exec(command: &str) -> io::Result<()> {
    // initialize the bash environment
    let command = format!(r#"
    source /usr/share/2/bin/e-core || exit 211
    {}
    "#, command);

    let mut child = Command::new("bash")
        .arg("-c")
        .arg(&command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let stdout_thread = thread::spawn(move || {
        let reader = io::BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    cpr!("{}", line);
                    debug!("{}", line)
                }
                Err(e) => erm!("Error reading stdout: {}", e),
            }
        }
    });

    let stderr_thread = thread::spawn(move || {
        let reader = io::BufReader::new(stderr);
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    cpr!("\x1b[{}{}", CONFIG.message.stderr, line);
                    debug!("[ERR] {}", line)
                }
                Err(e) => erm!("Error reading stderr: {}", e),
            }
        }
    });

    let status = child.wait()?;
    if !status.success() {
        error!("Command `{}` failed!", command);
        return Err(io::Error::new(io::ErrorKind::Other, "Command failed"));
    }

    stdout_thread.join().unwrap();
    stderr_thread.join().unwrap();

    Ok(())
}
