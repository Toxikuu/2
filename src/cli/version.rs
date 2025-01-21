// src/cli/version.rs
//! Implements --version

const NAME: &str = env!("CARGO_PKG_NAME");
const REPO: &str = env!("CARGO_PKG_REPOSITORY");
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// # Description
/// Displays 2's version
pub fn display() {
println!(
r"
{NAME}={VERSION}
{REPO}
"
);
}
