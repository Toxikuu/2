// src/main.rs
//
#![doc = include_str!("../README.md")]
#![feature(duration_millis_float)]
#![feature(str_as_str)]
#![deny(clippy::perf, clippy::todo, clippy::complexity)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    unused,
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
use globals::flags::{
    self,
    FLAGS,
};
use package::{
    parse,
    provides,
    repos,
    sets,
};
use pm::PM;
use utils::{
    fail::BoolFail,
    logger,
};

/// ### Description
/// Takes arguments from the environment and calls PM or other functions accordingly
fn main() {
    let mut args = initialize();
    unsafe { libc::geteuid() == 0 }.or_fail("2 requires root privileges");

    handle_special_args(&mut args);

    let packages = parse::parse(&args.packages);
    PM::new(&packages, &args).run();
    log::info!("Finished all tasks\n\n\t----------------\n");
}

/// ### Description
/// Initializes arguments and sets flags
/// Also initializes the logger
fn initialize() -> Args {
    logger::init();

    log::info!("Process initiated");
    log::debug!(
        "Command line: {:?}",
        std::env::args().collect::<Vec<String>>().join(" ")
    );
    log::debug!("Initialized logger");

    let args = Args::init();
    flags::set(&args);

    log::debug!("Initialized args: {args:#?}");
    log::debug!("Initialized flags: {FLAGS:#?}");

    args
}

/// ### Description
/// Handles special arguments if any were passed
fn handle_special_args(args: &mut Args) {
    if args.version {
        v::display()
    }

    args.provides.iter().for_each(|p| provides::provides(p));
    args.add_repos.iter().for_each(|r| repos::add(r));
    if let Some(repos) = &mut args.sync_repos {
        if repos.is_empty() {
            *repos = repos::find_all().to_vec();
        }
        repos.iter().for_each(|r| repos::sync(r));
    }
    args.list_sets.iter().for_each(|r| sets::list(r));

    if args.list_repos {
        repos::list()
    }
}
