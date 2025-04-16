// twolib/src/package/mod.rs
//! The Package struct

pub mod build;
pub mod dl;
pub mod history;
pub mod methods;
pub mod parse;
pub mod provides;
pub mod remove;
pub mod stats;
mod traits;
pub mod upstream;

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
// TODO: Implement Package::from_name() -> Self
#[derive(Deserialize, Debug, Clone)]
pub struct Package {
    pub name:         String,
    pub repo:         String,
    pub version:      String,
    pub timestamp:    String,
    pub categories:   Option<Vec<String>>,
    pub description:  Option<String>,
    pub dependencies: Option<Vec<Dependency>>,

    // TODO: Use Option<Source>
    #[serde(default)]
    pub source:  Source,
    #[serde(default)]
    pub extra:   Arc<[String]>,
    #[serde(skip)]
    pub relpath: String,

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

#[derive(Debug, Deserialize, Clone)]
pub enum SourceKind {
    Git,
    Zip,
    Tarball,
}

impl Default for SourceKind {
    fn default() -> Self { Self::Tarball }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Source {
    pub kind: SourceKind,
    pub url:  String,
}

#[derive(Debug, Clone, Deserialize)]
// NOTE: String and relpath are intentional as they decrease complexity hopefully
pub struct Dependency {
    relpath:  String,
    optional: bool,
    runtime:  bool,
}
