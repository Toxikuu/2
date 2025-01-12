#![feature(duration_millis_float)]
// src/main.rs

#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    // clippy::cargo,
)]
#![deny(
    clippy::perf,
    clippy::todo,
    clippy::complexity,
)]

mod build;
mod cli;
mod fetch;
mod globals;
mod macros;
mod package;
mod pm;
mod remove;
mod shell;
mod utils;

use cli::args::Args;
use cli::version as v;
use globals::flags;
use package::{parse, sets, repos};
use pm::PM;
use utils::fail::Fail;

/// ### Description
/// Takes arguments from the environment and calls PM or other functions accordingly
fn main() {
    let args = initialize();
    // TODO: test against args to determine which need root privileges before erroring out
    // TODO: make the bug reporting message toggleable in the config
    if !unsafe { libc::geteuid() == 0 } { fail!("2 requires root privileges") } // prolly safe :)

    // exit after executing any special argument functions
    handle_special_args(&args);

    let packages = parse::parse(&args.packages);
    let mut pm = PM::new(&packages);

    if args.version { v::display() }
    if args.build   { pm.build  () }
    if args.install { pm.install() }
    if args.update  { pm.update () }
    if args.get     { pm.get    () }
    if args.prune   { pm.prune  () }
    if args.clean   { pm.clean  () }
    if args.list    { pm.list   () }
    if args.remove  { pm.remove () }
}

/// ### Description
/// Initializes arguments and sets flags
fn initialize() -> Args {
    let args = Args::init();
    flags::set(&args);

    vpr!("Initialized args: {:#?}", args);
    vpr!("Initialized flags: {:#?}", FLAGS.lock().ufail("Flag lock failure"));

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
