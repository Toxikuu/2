// src/package/provides.rs
//! Utilities for seeing what package(s) provide a specific path

use anyhow::Context;
use std::fs;
use std::path::{Path, PathBuf};

use crate::comms::log::erm;
use crate::pm::PM;
use crate::remove::manifest;
use crate::utils::fail::Fail;
use super::Package;

/// # Description
/// Finds all manifests containing a given path
fn find_manifests_with_path(path: &str) -> Vec<PathBuf> {
    manifest::locate("/usr/ports")
        .iter()
        .filter_map(|m| {
        let contents = fs::read_to_string(m).ufail("Failed to read manifest");
        let lines = contents.lines();
        for line in lines {
            if line == path {
                return Some(m.clone());
            }
        }
        None
    }).collect::<Vec<_>>()
}

/// # Description
/// Returns a package given its manifest
fn get_package_from_manifest(manifest: &Path) -> Package {
    let components = manifest.components().collect::<Vec<_>>();

    let repo = components.get(3)
        .context("Are you using /usr/ports?")
        .fail("Failed to parse package repo from manifest path")
        .as_os_str()
        .to_string_lossy();
    let name = components.get(4)
        .context("Are you using /usr/ports?")
        .fail("Failed to parse package name from manifest path")
        .as_os_str()
        .to_string_lossy();

    Package::new(&repo, &name)
}

/// # Description
/// Lists the packages that provide a given path
///
/// Handles display, returns nothing
pub fn provides(path: &str) {
    let manifests = find_manifests_with_path(path);

    let total = manifests.len();
    match total {
        0 => erm!("No installed packages provide '{path}'"),
        1 => {
            let package = get_package_from_manifest(manifests.first().ufail("1 != 1"));
            let packages = [package; 1];
            PM::new(&packages).list(&format!("1 installed package provides '{path}'")); // stupid solution but idc lol
        }
        _ => {
            let packages = manifests.iter().map(|m| get_package_from_manifest(m)).collect::<Vec<_>>();
            PM::new(&packages).list(&format!("{total} installed packages provide '{path}'")); // again
        }
    }
}
