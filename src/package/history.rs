// src/package/history.rs
//! Functions for viewing the history of a package

use super::Package;
use std::{
    fmt::Write,
    fs,
};
use crate::globals::config::CONFIG;

/// # Description
/// Reads $PORT/CHANGELOG into a string
fn read_history(package: &Package) -> String {
    let changelog = format!("/usr/ports/{}/CHANGELOG", package.relpath);
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

        let _ = writeln!(formatted, "  \x1b[{style}{line}");
    }

    formatted
}

/// # Description
/// Retrieves and displays formatted history for a package
pub fn view(package: &Package) {
    let history = read_history(package);
    let history = format_history(&history);

    println!("\x1b[{}Changelog for '{package}':\n\n{history}", CONFIG.message.prompt);
}
