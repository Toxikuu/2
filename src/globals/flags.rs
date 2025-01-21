// src/globals/flags.rs
//! Defines flags

use crate::cli::args::Args;
use crate::utils::fail::Fail;
use lazy_static::lazy_static;
use std::sync::Mutex;
use super::config::CONFIG;

/// # Description
/// The generic global flags struct
///
/// By default, these are taken from the config, but may be overridden with passed flags.
#[derive(Debug)]
pub struct Flags {
    pub force: bool,
    pub quiet: bool,
    pub verbose: bool,
}

impl Flags {
    /// # Description
    /// Creates a new flags struct
    pub fn new() -> Self {
        Self {
            force: CONFIG.flags.force,
            quiet: CONFIG.flags.quiet,
            verbose: CONFIG.flags.verbose,
        }
    }
}

lazy_static! {
    /// # Description
    /// The global flags object
    pub static ref FLAGS: Mutex<Flags> = Mutex::new(Flags::new());
}

/// # Description
/// Sets the generic global flags given passed args
pub fn set(args: &Args) {
    FLAGS.lock().ufail("Failed to lock flags").force = args.force;
    FLAGS.lock().ufail("Failed to lock flags").quiet = args.quiet;
    FLAGS.lock().ufail("Failed to lock flags").verbose = args.verbose;
}
