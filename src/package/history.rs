// src/package/history.rs
//! Functions for viewing the history of a package

use std::{
    fmt::Write,
    fs,
};

use super::Package;
use crate::{
    globals::config::CONFIG,
    utils::fail::Fail,
};

/// # Description
/// Reads $PORT/CHANGELOG into a string
fn read_history(package: &Package) -> String {
    let changelog = package.data.port_dir.join("CHANGELOG");
    fs::read_to_string(changelog).unwrap_or_default()
}

/// # Description
/// Adds color to the history:
/// - Prompt formatting for Revised
/// - Message formatting for Added and Updated
/// - Default for everything else
fn format_history(history: &str) -> String {
    let mut formatted = String::new();

    for line in history.lines().filter(|l| l.contains("] | ")) {
        let style = if line.contains("] | Revised ") {
            &CONFIG.message.prompt.trim()
        } else if line.contains("] | Added ") || line.contains("] | Updated ") {
            &CONFIG.message.message.trim()
        } else {
            &CONFIG.message.default.trim()
        };

        writeln!(formatted, "  \x1b[0m{style}{line}").fail("Failed to write line?");
    }

    formatted
}

/// # Description
/// Retrieves and displays formatted history for a package
pub fn view(package: &Package) {
    let history = read_history(package);
    let history = format_history(&history);

    println!(
        "{}Changelog for '{package}':\n\n{history}",
        CONFIG.message.prompt
    );
}
