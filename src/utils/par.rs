// src/pm/par.rs
//! Adds support functions for parallelism

use crate::{comms::out::vpr, globals::config::CONFIG, package::Package, utils::fail::Fail};
use rayon::{ThreadPool, ThreadPoolBuilder};

pub fn build_pool(packages: &[Package]) -> ThreadPool {
    vpr!("Building thread pool...");
    let num_threads = CONFIG.upstream.max_threads.min(packages.len());
    ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .stack_size(CONFIG.upstream.stack_size * 1024)
        .build()
        .fail("Failed to build thread pool")
}
