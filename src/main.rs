// src/main.rs
//
//! # 2
//! 2 is a (mostly) source-based package manager for LFS
//!
//! It's got some fancy features:
//! - distribution tarballs
//! - per-package changelogs
//! - TODO: finish this list

#![feature(duration_millis_float)]
#![feature(str_as_str)]

#![deny(
    clippy::perf,
    clippy::todo,
    clippy::complexity,
)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    missing_docs,
    // clippy::cargo,
)]

mod build;
mod cli;
mod comms;
mod fetch;
mod globals;
mod package;
mod pm;
mod remove;
mod shell;
#[cfg(feature = "upstream")]
mod upstream;
mod utils;

use cli::{
    args::Args,
    version as v,
};
use globals::flags::{self, FLAGS};
use package::{parse, sets, repos, provides};
use pm::PM;
use utils::logger;

/// ### Description
/// Takes arguments from the environment and calls PM or other functions accordingly
fn main() {
    let args = initialize();
    // TODO: test against args to determine which need root privileges before erroring out
    if !unsafe { libc::geteuid() == 0 } { fail!("2 requires root privileges") } // prolly safe :)

    handle_special_args(&args);

    let packages = parse::parse(&args.packages);
    PM::new(&packages, &args).run();

    logger::get().detach();
    log::info!("Finished all tasks\n\n\t----------------\n");
}

/// ### Description
/// Initializes arguments and sets flags
/// Also initializes the logger
fn initialize() -> Args {
    logger::init("/var/log/2/master.log");
    logger::get();

    log::info!("Process initiated");
    log::debug!("Command line: {:?}", std::env::args().collect::<Vec<String>>().join(" "));
    log::debug!("Initialized logger");

    let args = Args::init();
    flags::set(&args);

    log::debug!("Initialized args: {:#?}", args);
    log::debug!("Initialized flags: {:#?}", FLAGS);

    args
}

/// ### Description
/// Handles special arguments if any were passed
fn handle_special_args(args: &Args) {
    if args.version { v::display() }

    args.provides.iter  ().for_each(|p| provides::provides(p));
    args.add_repos.iter ().for_each(|r| repos::add (r));
    args.sync_repos.iter().for_each(|r| repos::sync(r));
    args.list_sets.iter ().for_each(|r| sets::list (r));

    if args.list_repos { repos::list() }
}
