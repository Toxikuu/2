// src/globals/flags.rs
// 
// defines flags

use lazy_static::lazy_static;
use std::sync::Mutex;
use super::config::CONFIG;
use crate::cli::args::Args;
use crate::utils::fail::Fail;

#[derive(Debug)]
pub struct Flags {
    pub force: bool,
    pub quiet: bool,
    pub verbose: bool,
}

impl Flags {
    pub fn new() -> Self {
        Self {
            force: CONFIG.flags.force,
            quiet: CONFIG.flags.quiet,
            verbose: CONFIG.flags.verbose,
        }
    }
}

lazy_static! {
    pub static ref FLAGS: Mutex<Flags> = Mutex::new(Flags::new());
}

pub fn set(args: &Args) {
    FLAGS.lock().ufail("Failed to lock flags").force = args.force;
    FLAGS.lock().ufail("Failed to lock flags").quiet = args.quiet;
    FLAGS.lock().ufail("Failed to lock flags").verbose = args.verbose;
}
