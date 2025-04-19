// src/main.rs

#![doc = include_str!("../README.md")]
#![feature(duration_millis_float)]
#![deny(clippy::perf, clippy::todo, clippy::complexity)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    unused,
    missing_docs,
    clippy::cargo
)]
#![allow(clippy::semicolon_if_nothing_returned)]

use std::str::FromStr;

use tracing::{
    debug,
    info,
    level_filters::LevelFilter,
    warn,
};
use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling,
};
use tracing_subscriber::{
    EnvFilter,
    fmt,
};
use twoconfig::CONFIG;
use twoerror::BoolFail;
use twolib::{
    cli::{
        args::Args,
        version as v,
    },
    package::{
        parse,
        provides,
    },
    pm::PM,
    repo::Repo,
    set,
};

/// ### Description
/// Takes arguments from the environment and calls PM or other functions accordingly
fn main() {
    let (mut args, guard) = initialize();
    unsafe { libc::geteuid() == 0 }.or_fail("2 requires root privileges");

    handle_special_args(&mut args);

    let packages = parse::parse(&args.packages);
    PM::new(&packages, &args).run();
    info!("Finished all tasks\n\n");
    drop(guard);
}

/// ### Description
/// Initializes arguments and sets flags
/// Also initializes the logger
fn initialize() -> (Args, WorkerGuard) {
    let guard = init_logging();

    info!("Initializing...");
    debug!(
        "Command line: {}",
        std::env::args().collect::<Vec<String>>().join(" ")
    );

    let args = Args::init();

    debug!("Initialized args: {args:#?}");
    // debug!("Initialized flags: {FLAGS:#?}");

    (args, guard)
}

/// ### Description
/// Initializes logging
///
/// Log level priority:
/// Environment variable -> Config -> Default
fn init_logging() -> WorkerGuard {
    let file_appender = rolling::never("/tmp/2", "log");
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

    let level = LevelFilter::from_str(&CONFIG.general.log_level).unwrap_or(LevelFilter::DEBUG);
    let filter = EnvFilter::builder()
        .with_default_directive(level.into())
        .with_env_var("LOG_LEVEL")
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_level(true)
        .with_target(true)
        .with_line_number(true)
        .with_timer(fmt::time::uptime())
        .with_writer(file_writer)
        .compact()
        .init();

    guard
}

/// ### Description
/// Handles special arguments if any were passed
fn handle_special_args(args: &mut Args) {
    if args.version {
        v::display()
    }

    args.provides.iter().for_each(|p| provides::provides(p));
    args.add_repos.iter().for_each(|r| Repo::add(r));
    if let Some(repos) = &mut args.sync_repos {
        let mut repos: Vec<Repo> = repos.iter().map(|r| Repo::new(r)).collect();
        if repos.is_empty() {
            repos = Repo::find_all().to_vec();
        }
        repos.iter().for_each(Repo::sync);
    }
    args.list_sets.iter().for_each(|r| set::list(r));

    if args.list_repos {
        Repo::list_all()
    }
}
