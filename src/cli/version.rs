// src/cli/version.rs
//! Implements --version

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const REPO: &str = env!("CARGO_PKG_REPOSITORY");

pub fn display() {
println!(
r"
{NAME}={VERSION}
{REPO}
"
);
}
