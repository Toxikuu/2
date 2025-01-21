// src/package/mod.rs
//! defines the package type

pub mod ambiguity;
pub mod endpoints;
pub mod parse;
pub mod provides;
pub mod repos;
pub mod sets;
pub mod traits;

use serde::Deserialize;
use std::rc::Rc;

/// # Description
/// The package struct
/// 
/// ``relpath`` is calculated from repo and name
///
/// Contains package data
#[derive(Deserialize, Debug, Clone)]
pub struct Package {
    pub name: Rc<str>,
    pub repo: Rc<str>,
    #[serde(default)]
    pub relpath: String,
    pub version: Rc<str>,
    pub data: PackageData,
}

/// # Description
/// The package data struct
///
/// Contains package source
#[derive(Deserialize, Debug, Clone)]
pub struct PackageData {
    #[serde(default)]
    pub is_installed: bool,
    #[serde(default)]
    pub installed_version: Rc<str>,
    pub source: PackageSource,
    pub extra: Rc<[PackageSource]>,
    #[serde(default)]
    pub dist: Rc<str>,
}

/// # Description
/// The package source struct
#[derive(Deserialize, Debug, Clone)]
pub struct PackageSource {
    pub url: Rc<str>,
    pub hash: Rc<str>,
}
