// src/package/endpoints.rs
//! Defines endpoints for the package struct

use crate::{
    comms::out::vpr,
    utils::fail::{BoolFail, Fail},
};
use std::{
    fs,
    path::{Path, PathBuf},
};
use super::Package;

impl Package {
    /// # Description
    /// Creates a package given its repo and name
    pub fn new(repo: &str, name: &str) -> Self {
        // avoid problems with .sets, .git, etc
        name.starts_with('.').and_fail("Invalid package name");

        let toml_path = PathBuf::from("/usr/ports")
            .join(repo)
            .join(name)
            .join("info.lock");
        let toml_contents = fs::read_to_string(&toml_path).fail("Failed to read info.lock");

        let mut package: Self = toml::de::from_str(&toml_contents).fail("Invalid syntax in info.lock");
        let status_path = toml_path.with_file_name(".data/INSTALLED");

        vpr!("Status path: {:?}", status_path);
        package.data.is_installed = status_path.exists();

        let relpath = format!("{repo}/{name}");

        package.repo = repo.to_string();
        package.name = name.to_string();

        package.data.installed_version = fs::read_to_string(&status_path).unwrap_or_default().trim().to_string();
        package.data.dist = format!("/usr/ports/{relpath}/.dist/{package}.tar.zst");
        package.relpath = relpath;

        package
    }

    pub fn is_outdated(&self) -> bool {
        self.data.is_installed && self.data.installed_version != self.version
    }

    pub fn dist_exists(&self) -> bool {
        Path::new(&self.data.dist).exists()
    }
}
