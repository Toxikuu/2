// src/package/endpoints.rs
//! Defines endpoints for the package struct

use crate::{
    comms::log::vpr,
    utils::fail::{fail, Fail},
};
use std::{
    fs,
    path::Path,
};
use super::Package;

impl Package {
    /// # Description
    /// Creates a package given its repo and name
    pub fn new(repo: &str, name: &str) -> Self {
        // avoid problems with .sets, .git, etc
        if name.starts_with('.') { fail!("Invalid package name") }

        let toml_path_str = format!("/usr/ports/{repo}/{name}/info.lock");
        let toml_path = Path::new(&toml_path_str);
        if !toml_path.exists() { fail!("{} does not exist", &toml_path_str) }
        let toml_contents = fs::read_to_string(toml_path).ufail(&format!("Something is very wrong with {}", &toml_path_str));

        let mut package: Self = toml::de::from_str(&toml_contents).ufail("Invalid syntax in info.lock");
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
}
