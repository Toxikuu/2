// src/cli/version.rs
//! Implements --version

const REPO: &str = env!("CARGO_PKG_REPOSITORY");
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// # Description
/// Displays 2's version
pub fn display() {
    println!(
        r"2={VERSION}
{REPO}
"
    );
}
