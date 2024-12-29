// src/globals/flags.rs
// 
// defines flags

use lazy_static::lazy_static;
use std::sync::Mutex;
use super::config::CONFIG;
use crate::cli::args::Args;

#[derive(Debug)]
pub struct Flags {
    pub force: bool,
    pub quiet: bool,
    pub verbose: bool,
}

impl Flags {
    pub fn new() -> Self {
        Flags {
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
    FLAGS.lock().unwrap().force = args.force;
    FLAGS.lock().unwrap().quiet = args.quiet;
    FLAGS.lock().unwrap().verbose = args.verbose;
}
