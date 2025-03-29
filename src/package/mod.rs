// src/package/mod.rs
//! Defines the package type

pub mod ambiguity;
pub mod endpoints;
pub mod history;
pub mod parse;
pub mod provides;
pub mod repos;
pub mod sets;
pub mod stats;
pub mod traits;

use std::{
    path::PathBuf,
    sync::Arc,
};

use serde::Deserialize;

/// # Description
/// The package struct
///
/// ``relpath`` is calculated from repo and name
///
/// Contains package data
#[derive(Deserialize, Debug, Clone)]
pub struct Package {
    pub name:        String,
    pub repo:        String,
    pub version:     String,
    pub timestamp:   String,
    pub categories:  Option<Vec<String>>,
    pub description: Option<String>,

    #[serde(default)]
    pub extra:   Arc<[PackageSource]>,
    #[serde(default)]
    pub source:  PackageSource,
    #[serde(skip)]
    pub relpath: String,

    #[cfg(feature = "upstream")]
    pub upstream:        Option<String>,
    #[cfg(feature = "upstream")]
    pub version_command: Option<String>,

    #[serde(skip)]
    pub data: PackageData,
}

/// # Description
/// The package data struct contains extra information about the package
#[derive(Deserialize, Debug, Default, Clone)]
pub struct PackageData {
    #[serde(skip)]
    pub dist:              PathBuf,
    #[serde(skip)]
    pub status:            Arc<str>,
    #[serde(skip)]
    pub port_dir:          PathBuf,
    #[serde(skip)]
    pub is_installed:      bool,
    #[serde(skip)]
    pub installed_version: String,
}

/// # Description
/// The package source struct
#[derive(Deserialize, Debug, Default, Clone)]
pub struct PackageSource {
    pub url:  Arc<str>, // must be thread safe
    pub hash: String,
}
