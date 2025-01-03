#![feature(duration_millis_float)]
// src/main.rs

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

/// ### Description
/// Takes arguments from the environment and calls PM or other functions accordingly
fn main() {
    let args = initialize();
    // TODO: test against args to determine which need root privileges before erroring out
    if !unsafe { libc::geteuid() == 0 } { die!("2 requires root privileges") } // prolly safe :)

    // exit after executing any special argument functions
    if handle_special_args(&args) { return }

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
    vpr!("Initialized flags: {:#?}", FLAGS.lock().unwrap());

    args
}

/// ### Description
/// Handles special arguments if any were passed, returning true; otherwise returns false
fn handle_special_args(args: &Args) -> bool {
    if args.list_sets { 
        // here packages is appropriated as the repo argument for listing sets
        args.packages.iter().for_each(|r| sets::list(r));
        true
    } else if args.list_repos { 
        repos::list();
        true
    } else {
        false
    }
}
