// code/pm/mod.rs

pub mod endpoints;

use crate::package::Package;
use std::rc::Rc;

pub struct PM {
    pub packages: Rc<[Package]>
}

impl PM {
    pub fn new(packages: &[Package]) -> Self {
        Self { packages: packages.into() }
    }
}
