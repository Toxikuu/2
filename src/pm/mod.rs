// code/pm/mod.rs

pub mod endpoints;

use crate::package::Package;

pub struct PM {
    pub packages: Vec<Package>
}

impl PM {
    pub fn new(packages: &[Package]) -> Self {
        Self { packages: packages.to_vec() }
    }
}
