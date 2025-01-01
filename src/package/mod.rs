// src/package/mod.rs
//
// defines the package type

pub mod endpoints;
pub mod ambiguity;
pub mod traits;
pub mod parse;
pub mod sets;
pub mod repos;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Package {
    pub name: String,
    pub repo: String,
    #[serde(default)]
    pub relpath: String,
    pub version: String,
    pub data: PackageData,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PackageData {
    #[serde(default)]
    pub is_installed: bool,
    #[serde(default)]
    pub installed_version: String,
    pub source: PackageSource,
    pub extra: Vec<PackageSource>,
    #[serde(default)]
    pub dist: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PackageSource {
    pub url: String,
    pub hash: String,
}
