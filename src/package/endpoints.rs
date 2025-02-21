// src/package/endpoints.rs
//! Defines endpoints for the package struct

use crate::{
    comms::out::{vpr, msg, pr},
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

        let relpath = format!("{repo}/{name}");
        let port_dir = PathBuf::from("/usr/ports").join(&relpath);

        let toml_path = port_dir.join("LOCK");
        let toml_contents = fs::read_to_string(&toml_path).fail("Failed to read LOCK");

        let mut package: Self = toml::de::from_str(&toml_contents).fail("Invalid syntax in LOCK");
        let status_path = port_dir.join(".data").join("INSTALLED");

        package.relpath = relpath;

        let dist_tb = format!("{name}={}.tar.zst", package.version);

        vpr!("Status path: {:?}", status_path);
        package.data.is_installed = status_path.exists();

        package.data.installed_version = fs::read_to_string(&status_path).unwrap_or_default().trim().to_string();
        package.data.dist = port_dir.join(".dist").join(&dist_tb);
        package.data.port_dir = port_dir;
        package.status();

        package
    }

    pub fn is_outdated(&self) -> bool {
        self.data.is_installed && self.data.installed_version != self.version
    }

    pub fn dist_exists(&self) -> bool {
        Path::new(&self.data.dist).exists()
    }

    /// # Description
    /// Returns a package's (formatted) status
    fn status(&mut self) {
        self.data.status = {
            let iv = &self.data.installed_version;

            let status = if !self.data.is_installed {
                "\x1b[0;30mAvailable".to_string()
            } else if self.is_outdated() {
                format!("\x1b[1;31mOutdated ({iv})")
            } else {
                format!("\x1b[1;36mInstalled {iv}")
            };
            status.into()
        }
    }

    pub fn summarize(&self) {
        let status = &self.data.status;
        let sty = if status.contains("Available") { "\x1b[30m" }
            else if status.contains("Outdated") { "\x1b[1;31m" }
            else { "\x1b[1;36m" };

        msg!("{sty} 󰏖 {}/{}={}", self.repo, self.name, self.version);

        if let Some(desc) = &self.description { pr!("\x1b[37m {desc}") }
        else { pr!("\x1b[37m No description provided") }

        // TODO: Convert upstream to option
        pr!("\x1b[37m󰘬 {}", self.upstream);
    }
}
