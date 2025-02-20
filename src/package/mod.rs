// src/package/mod.rs
//! Defines the package type

pub mod ambiguity;
pub mod endpoints;
pub mod history;
pub mod parse;
pub mod provides;
pub mod repos;
pub mod sets;
pub mod traits;

use serde::Deserialize;
use std::{path::PathBuf, sync::Arc};

/// # Description
/// The package struct
///
/// ``relpath`` is calculated from repo and name
///
/// Contains package data
#[derive(Deserialize, Debug, Clone)]
pub struct Package {
    pub name: String,
    pub repo: String,
    #[serde(skip)]
    pub relpath: String,

    pub version: String,
    #[serde(default)]
    pub source: PackageSource,
    #[serde(default)]
    pub extra: Arc<[PackageSource]>,

    // I prolly should use Option<> but I'm fucking lazy
    #[cfg(feature = "upstream")]
    #[serde(default)]
    pub upstream: String,
    #[cfg(feature = "upstream")]
    #[serde(default)]
    pub version_command: String,

    #[serde(skip)]
    pub data: PackageData,
}

/// # Description
/// The package data struct contains extra information about the package
#[derive(Deserialize, Debug, Default, Clone)]
pub struct PackageData {
    #[serde(skip)]
    pub is_installed: bool,
    #[serde(skip)]
    pub installed_version: String,
    #[serde(skip)]
    pub dist: PathBuf,
    #[serde(skip)]
    pub port_dir: PathBuf,
}

/// # Description
/// The package source struct
#[derive(Deserialize, Debug, Default, Clone)]
pub struct PackageSource {
    pub url: Arc<str>, // must be thread safe
    pub hash: String,
}
