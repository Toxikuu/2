// src/package/traits.rs
//! Defines traits for package

use super::Package;
use std::fmt::{Display, Formatter, Result};

impl Display for Package {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}={}", self.name, self.version,)
    }
}
