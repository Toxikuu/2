// src/globals/flags.rs
//! Defines flags

use once_cell::sync::OnceCell;
use crate::{
    cli::args::Args,
    globals::config::CONFIG,
    utils::fail::Fail,
};

/// # Description
/// The generic global flags object
pub static FLAGS: OnceCell<Flags> = OnceCell::new();

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
    pub fn new(args: &Args) -> Self {
        let force = args.force || CONFIG.flags.force;
        let quiet = args.quiet || CONFIG.flags.quiet;
        let verbose = args.verbose || CONFIG.flags.verbose;

        Self { force, quiet, verbose }
    }
}

pub fn set(args: &Args) {
    FLAGS.set(Flags::new(args)).ufail("FLAGS was reinitialized")
}
