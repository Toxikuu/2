// src/package/provides.rs
//! Utilities for seeing what package(s) provide a specific path

use anyhow::Context;
use crate::{
    comms::out::erm,
    pm::PM,
    remove::manifest,
    utils::fail::Fail,
};
use std::{
    fs,
    path::{Path, PathBuf},
};
use super::Package;

/// # Description
/// Finds all manifests containing a given path
fn find_manifests_with_path(path: &str) -> Vec<PathBuf> {
    manifest::locate("/var/ports")
        .iter()
        .filter_map(|m| {
        let contents = fs::read_to_string(m).fail("Failed to read manifest");
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
        .context("Are you using /var/ports?")
        .fail("Failed to parse package repo from manifest path")
        .as_os_str()
        .to_string_lossy();
    let name = components.get(4)
        .context("Are you using /var/ports?")
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
            let package = get_package_from_manifest(manifests.first().fail("1 != 1"));
            let packages = [package; 1];
            PM::list_packages(&packages, &format!("1 installed package provides '{path}'"), false);
        }
        _ => {
            let packages = manifests.iter().map(|m| get_package_from_manifest(m)).collect::<Vec<_>>();
            PM::list_packages(&packages, &format!("{total} installed packages provide '{path}'"), false);
        }
    }
}
