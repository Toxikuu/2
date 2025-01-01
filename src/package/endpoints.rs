// src/package/endpoints.rs
//
// defines endpoints for the package struct

use super::Package;
use std::fs;
use std::path::Path;
use crate::die;

impl Package {
    pub fn new(repo: &str, name: &str) -> Self {
        // avoid problems with .sets, .git, etc
        if name.starts_with('.') {
            die!("Invalid package name")
        }

        let toml_path_str = format!("/usr/ports/{}/{}/info.lock", repo, name);
        let toml_path = Path::new(&toml_path_str);
        let toml_contents = fs::read_to_string(toml_path).unwrap();

        let status_path_str = format!("/usr/ports/{}/{}/.data/INSTALLED", repo, name);
        let mut package: Package = toml::de::from_str(&toml_contents).unwrap();

        package.data.is_installed = Path::new(&status_path_str).exists();
        package.data.installed_version = fs::read_to_string(status_path_str).unwrap_or_default().trim().to_string();
        package.relpath = format!("{}/{}", repo, name);
        package.data.dist = format!("/usr/ports/{}/.dist/{}.tar.zst", package.relpath, package);

        package
    }
}
