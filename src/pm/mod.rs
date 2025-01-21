// src/pm/mod.rs
//! Defines the package manager struct and links to its endpoints

pub mod endpoints;

use crate::package::Package;
use std::rc::Rc;

/// # Description
/// The package manager struct
pub struct PM {
    pub packages: Rc<[Package]>
}

impl PM {
    /// # Description
    /// Creates a new package manager struct from an array of packages
    pub fn new(packages: &[Package]) -> Self {
        Self { packages: packages.into() }
    }
}
