// src/pm/mod.rs
//! Defines the package manager struct and links to its endpoints

pub mod endpoints;

use crate::{
    package::Package,
    globals::flags::FLAGS,
    utils::fail::Fail,
};

#[cfg(feature = "parallelism")]
pub use {
    crate::utils::par::build_pool,
    rayon::ThreadPool,
};

/// # Description
/// The package manager struct
pub struct PM<'a> {
    pub packages: &'a [Package],
    pub force: bool,
    #[cfg(feature = "parallelism")]
    pub thread_pool: ThreadPool,
}

impl<'a> PM<'a> {
    /// # Description
    /// Creates a new package manager struct from an array of packages
    #[cfg(not(feature = "parallelism"))]
    pub const fn new(packages: &'a [Package]) -> Self {
        Self { packages }
    }

    /// # Description
    /// Creates a new package manager struct from an array of packages
    #[cfg(feature = "parallelism")]
    pub fn new(packages: &'a [Package]) -> Self {
        Self { 
            packages,
            force: FLAGS.get().ufail("Cell issue").force,
            thread_pool: build_pool(packages),
        }
    }
}
