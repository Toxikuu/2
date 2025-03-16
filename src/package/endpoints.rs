// src/package/endpoints.rs
//! Defines endpoints for the package struct

use crate::{
    comms::out::{msg, pr},
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
        name.starts_with('.').and_efail(|| format!("Invalid package name '{name}'"));

        let port_dir = PathBuf::from("/usr/ports").join(repo).join(name);
        let lock_path = port_dir.join("LOCK");
        log::debug!("Determined LOCK path for '{repo}/{name}': '{}'", lock_path.display());
        let contents = fs::read_to_string(&lock_path).efail(|| format!("Failed to read LOCK for '{repo}/{name}'"));

        let mut package: Self = toml::de::from_str(&contents).efail(|| format!("Invalid syntax in LOCK for '{repo}/{name}'"));

        package.relpath = format!("{}/{}", &package.repo, &package.name);
        let port_dir = PathBuf::from("/usr/ports").join(&package.repo).join(&package.name);
        let dist_tb = format!("{}={}.tar.zst", package.name, package.version);

        let status_path = port_dir.join(".data").join("INSTALLED");
        package.data.is_installed = status_path.exists();

        package.data.installed_version = fs::read_to_string(&status_path).unwrap_or_default().trim().to_string();
        package.data.dist = port_dir.join(".dist").join(&dist_tb);
        package.data.port_dir = port_dir;
        package.status();

        log::debug!("Generated new package: {package:#?}");
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

    pub fn about(&self) {
        let status = &self.data.status;
        let sty = if status.contains("Available") { "\x1b[30m" }
            else if status.contains("Outdated") { "\x1b[1;31m" }
            else { "\x1b[1;36m" };

        msg!("{sty} 󰏖 {}/{}={}", self.repo, self.name, self.version);
        pr!("\x1b[37m {}", self.description.as_deref().unwrap_or("No description"));
        pr!("\x1b[37m󰘬 {}", self.upstream.as_deref().unwrap_or("No upstream"));
    }

    pub fn long_about(&self) {
        let status = &self.data.status;
        let sty = if status.contains("Available") { "\x1b[30m" }
            else if status.contains("Outdated") { "\x1b[1;31m" }
            else { "\x1b[1;36m" };

        msg!("{sty} 󰏖 {}/{}={}", self.repo, self.name, self.version);
        pr!("\x1b[37m {}", self.description.as_deref().unwrap_or("No description"));
        pr!("\x1b[37m󰘬 {}", self.upstream.as_deref().unwrap_or("No upstream"));

        pr!("\n\x1b[37m {}", self.data.port_dir.display());
        if self.data.dist.exists() {
            pr!("\x1b[37m {}", self.data.dist.display());
        }

        let categories = self.categories
            .as_ref()
            .map_or("No categories".to_owned(), |c| c.join(", "));

        pr!("\x1b[37m󰓻 {categories}");
        // pr!("\x1b[37m {}") // license
    }
}
