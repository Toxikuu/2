// src/main.rs

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
mod utils;

use cli::args::Args;
use cli::version as v;
use globals::flags::{self, FLAGS};
use package::{parse, sets, repos};
use pm::PM;
use utils::fail::Fail;
use utils::logger;

/// ### Description
/// Takes arguments from the environment and calls PM or other functions accordingly
fn main() {
    let args = initialize();
    // TODO: test against args to determine which need root privileges before erroring out
    if !unsafe { libc::geteuid() == 0 } { fail!("2 requires root privileges") } // prolly safe :)

    // exit after executing any special argument functions
    handle_special_args(&args);

    let packages = parse::parse(&args.packages);
    let pm = PM::new(&packages);

    if args.version { v::display() }
    if args.build   { pm.build  () }
    if args.install { pm.install() }
    if args.update  { pm.update () }
    if args.get     { pm.get    () }
    if args.prune   { pm.prune  () }
    if args.clean   { pm.clean  () }
    if args.list    { pm.list   () }
    if args.logs    { pm.logs   () }
    if args.remove  { pm.remove () }

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
    log::debug!("Initialized flags: {:#?}", FLAGS.lock().ufail("Failed to lock flags"));

    args
}

/// ### Description
/// Handles special arguments if any were passed, returning true; otherwise returns false
fn handle_special_args(args: &Args) {

    args.add_repos.iter().for_each(|r| repos::add(r));
    args.sync_repos.iter().for_each(|r| repos::sync(r));
    args.list_sets.iter().for_each(|r| sets::list(r));

    if args.list_repos {
        repos::list();
    }
}
