// src/package/mod.rs
//
// defines the package type

pub mod endpoints;
pub mod ambiguity;
pub mod traits;
pub mod parse;
pub mod sets;
pub mod repos;

use std::rc::Rc;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Package {
    pub name: Rc<str>,
    pub repo: Rc<str>,
    #[serde(default)]
    pub relpath: Rc<str>,
    pub version: Rc<str>,
    pub data: PackageData,
}

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

#[derive(Deserialize, Debug, Clone)]
pub struct PackageSource {
    pub url: Rc<str>,
    pub hash: Rc<str>,
}
