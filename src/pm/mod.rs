// src/pm/mod.rs
//! Defines the package manager struct and links to its endpoints

pub mod endpoints;

use crate::package::Package;

/// # Description
/// The package manager struct
pub struct PM<'a> {
    pub packages: &'a [Package]
}

impl<'a> PM<'a> {
    /// # Description
    /// Creates a new package manager struct from an array of packages
    pub const fn new(packages: &'a [Package]) -> Self {
        Self { packages }
    }
}
