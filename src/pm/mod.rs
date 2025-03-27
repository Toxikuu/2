// src/pm/mod.rs
//! Defines the package manager struct and links to its endpoints

pub mod endpoints;

use crate::{cli::args::Args, package::Package};

#[cfg(feature = "parallelism")]
pub use {crate::utils::par::build_pool, rayon::ThreadPool};

/// # Description
/// The package manager struct
pub struct PM<'a> {
    pub packages: &'a [Package],
    pub args: &'a Args,
    #[cfg(feature = "parallelism")]
    pub thread_pool: ThreadPool,
}

impl<'a> PM<'a> {
    /// # Description
    /// Creates a new package manager struct from an array of packages
    #[cfg(not(feature = "parallelism"))]
    pub const fn new(packages: &'a [Package], args: &'a Args) -> Self {
        Self { packages, args }
    }

    /// # Description
    /// Creates a new package manager struct from an array of packages
    #[cfg(feature = "parallelism")]
    pub fn new(packages: &'a [Package], args: &'a Args) -> Self {
        Self {
            packages,
            args,
            thread_pool: build_pool(packages),
        }
    }
}
