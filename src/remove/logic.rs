// src/remove/logic.rs
//
// logic for package removal

use crate::package::Package;
use crate::{die, erm, pr};
use std::fs::{create_dir, remove_dir, remove_dir_all, remove_file};
use std::path::Path;
use super::manifest::{find_dead_files, find_unique_paths};
use crate::globals::config::CONFIG;
use crate::utils::die::Fail;

const KEPT: [&str; 14] = [
    "/usr",
    "/usr/bin",
    "/bin",
    "/etc",
    "/usr/share",
    "/usr/ports",
    "/usr/lib",
    "/usr/lib32",
    "/usr/libexec",
    "/sbin",
    "/sys",
    "/dev",
    "/",
    "usr/share/pkgconfig"
];

pub fn remove(package: &Package) -> bool {
    if !package.data.is_installed { 
        erm!("Not installed: '{}'", package);
        return false
    }

    let manifest = format!("/usr/ports/{}/{}/.data/MANIFEST={}", package.repo, package.name, package.version);
    let manifest_path = Path::new(&manifest);

    if !manifest_path.exists() { die!("Manifest doesn't exist") }

    let mut unique = find_unique_paths(&manifest_path.to_path_buf());
    // find finds files in ascending order, but they must be deleted in descending order to avoid
    // false alerts for populated directories
    unique.reverse();

    unique.iter().for_each(|p| {
        let prefix = Path::new(&CONFIG.general.prefix);
        let p = p.strip_prefix('/').unwrap_or(p);
        // NOTE: the above line is necessary because .join will just use an absolute path instead
        // of allowing double slashes (really shitty design choice imo??)
        let path = prefix.join(p);

        if KEPT.iter().any(|&s| path.ends_with(s)) {
            return erm!("Retaining protected path: {:?}", path);
        }
        
        // TODO: is failing optimal error handling here?
        // i think yes, since it should be unreachable
        if path.is_file() { remove_file(path).fail("Failed to remove file") } // NOTE: should be unreachable as root

        else if path.is_dir() {
            if let Err(e) = remove_dir(&path) {
                if e.to_string() == "Directory not empty (os error 39)" {
                    pr!("Ignoring '{}': populated", path.display())
                } else {
                    erm!("Failed to remove '{}': {}", path.display(), e)
                }
            }
        }

        pr!("'{}' -x", p)
    });

    // NOTE: the manifest is not removed as prune will handle it
    let status_file = format!("/usr/ports/{}/{}/.data/INSTALLED", package.repo, package.name);
    remove_file(status_file).fail("Failed to remove the status file!");

    if CONFIG.removal.remove_sources { remove_sources(package) }
    if CONFIG.removal.remove_dots { remove_dots(package) }

    true
}

fn remove_sources(package: &Package) {
    let srcdir = format!("/usr/ports/{}/{}/.sources", package.repo, package.name);
    remove_dir_all(srcdir).fail("Failed to remove sources!")
}

fn remove_dots(package: &Package) {
    let portdir_str = format!("/usr/ports/{}/{}", package.repo, package.name);
    let portdir = Path::new(&portdir_str);

    // lazy rm -rf .d*/{*,.*}
    // these should never fail
    remove_dir_all(portdir.join(".data")).fail("Failed to remove .data");
    create_dir(portdir.join(".data")).fail("Failed to recreate .data");

    remove_dir_all(portdir.join(".dist")).fail("Failed to remove .dist");
    create_dir(portdir.join(".dist")).fail("Failed to recreate .dist")
}

pub fn remove_dead_files_after_update(package: &Package) {
    if !package.data.is_installed { return erm!("'{}' is not installed!", package) }

    let mut dead_files = find_dead_files(package);
    dead_files.reverse();

    dead_files.iter().for_each(|p| {
        let prefix = Path::new(&CONFIG.general.prefix);
        let p = p.strip_prefix('/').unwrap_or(p);
        let path = prefix.join(p);

        if KEPT.iter().any(|&s| path.ends_with(s)) {
            return erm!("Retaining protected path: {:?}", path);
        }
        
        // TODO: look at this in the context of the next if statement and decide whether to fail or erm!
        if path.is_file() { remove_file(path).fail("Failed to remove dead file") } // NOTE: should be unreachable as root

        else if path.is_dir() {
            if let Err(e) = remove_dir(&path) {
                if e.to_string() == "Directory not empty (os error 39)" {
                    pr!("Ignoring '{}': populated", path.display())
                } else {
                    erm!("Failed to remove '{}': {}", path.display(), e)
                }
            }
        }

        pr!("'{}' -x", p)
    });
}

// TODO: actually implement pruning lol
// pub fn prune(package: &Package) {
//
// }
