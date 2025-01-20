// src/remove/logic.rs
//
// logic for package removal

use anyhow::{bail, Result};
use crate::comms::log::{erm, pr, vpr};
use crate::globals::config::CONFIG;
use crate::globals::flags::FLAGS;
use crate::package::Package;
use crate::utils::fail::{fail, Fail};
use std::fs::{create_dir, read_dir, remove_dir, remove_dir_all, remove_file};
use std::io::ErrorKind as IOE;
use std::path::{Path, PathBuf};
use super::manifest::{find_dead_files, find_unique_paths};

const KEPT: [&str; 23] = [
    "/",
    "/bin",
    "/boot",
    "/dev",
    "/etc",
    "/lib",
    "/lib32",
    "/opt",
    "/proc",
    "/root",
    "/run",
    "/sbin",
    "/sys",
    "/sys",
    "/usr",
    "/usr/bin",
    "/usr/lib",
    "/usr/lib32",
    "/usr/libexec",
    "/usr/ports",
    "/usr/share",
    "/usr/share/pkgconfig",
    "/var",
];

fn rmdir(path: &PathBuf) -> Result<()> {
    if let Err(e) = remove_dir(path) {
        match e.kind() {
            IOE::NotFound => erm!("Ignoring '{}': missing", path.display()),
            IOE::DirectoryNotEmpty => erm!("Ignoring '{}': populated", path.display()),
            _ => bail!("Failed to remove '{}': {}", path.display(), e)
        }
    }
    Ok(())
}

fn rmf(path: &PathBuf) -> Result<()> {
    if let Err(e) = remove_file(path) {
        match e.kind() {
            IOE::NotFound => erm!("Ignoring '{}': missing", path.display()),
            _ => bail!("Failed to remove '{}': {}", path.display(), e)
        }
    }
    Ok(())
}

pub fn remove(package: &Package) -> bool {
    if !package.data.is_installed && !FLAGS.lock().ufail("Failed to lock flags").force { 
        erm!("Not installed: '{}'", package);
        return false
    }

    let manifest = format!("/usr/ports/{}/{}/.data/MANIFEST={}", package.repo, package.name, package.version);
    let manifest_path = Path::new(&manifest);

    if !manifest_path.exists() { fail!("Manifest doesn't exist") }

    let unique = find_unique_paths(&manifest_path.to_path_buf());

    unique.iter().for_each(|p| {
        let pfx = Path::new(&CONFIG.general.prefix);
        let p = p.trim_start_matches('/');
        let path = pfx.join(p);

        if KEPT.iter().any(|&s| path.ends_with(s)) {
            erm!("Retaining protected path: {:?}", path); return
        }

        if path.is_symlink() {
            rmf(&path).fail("Failed to remove symlink");
        }
        
        if path.is_file() {
            rmf(&path).fail("Failed to remove file");
        }

        if path.is_dir() {
            rmdir(&path).fail("Failed to remove directory");
        }

        pr!("'{}' -x", p);
    });

    // NOTE: the manifest is not removed as prune will handle it
    let status_file = format!("/usr/ports/{}/{}/.data/INSTALLED", package.repo, package.name);
    remove_file(status_file).fail("Failed to remove the status file");

    if CONFIG.removal.remove_sources { remove_sources(package) }
    if CONFIG.removal.remove_dots { remove_dots(package) }

    true
}

fn remove_sources(package: &Package) {
    let srcdir = format!("/usr/ports/{}/{}/.sources", package.repo, package.name);
    remove_dir_all(&srcdir).ufail("Failed to remove .sources");
    create_dir(&srcdir).ufail("Failed to recreate .sources");
}

fn remove_dots(package: &Package) {
    let portdir_str = format!("/usr/ports/{}/{}", package.repo, package.name);
    let portdir = Path::new(&portdir_str);

    // lazy rm -rf .d{ata,ist}/{,.}*
    // these should never fail (unless maybe .data doesnt exist)
    remove_dir_all(portdir.join(".data")).ufail("Failed to remove .data");
    create_dir(portdir.join(".data")).ufail("Failed to recreate .data");

    remove_dir_all(portdir.join(".dist")).ufail("Failed to remove .dist");
    create_dir(portdir.join(".dist")).ufail("Failed to recreate .dist");
}

pub fn remove_dead_files_after_update(package: &Package) {
    if !package.data.is_installed { return erm!("'{}' is not installed!", package) }

    let dead_files = find_dead_files(package);

    dead_files.iter().for_each(|p| {
        let pfx = Path::new(&CONFIG.general.prefix);
        let p = p.trim_start_matches('/');
        let path = pfx.join(p);

        if KEPT.iter().any(|&s| path.ends_with(s)) {
            erm!("Retaining protected path: {:?}", path); return
        }

        if path.is_symlink() {
            rmf(&path).fail("Failed to remove symlink");
        }
        
        if path.is_file() {
            rmf(&path).fail("Failed to remove file");
        }

        if path.is_dir() {
            rmdir(&path).fail("Failed to remove directory");
        }

        pr!("'{}' -x", p);
    });
}

pub fn prune(package: &Package) -> usize {
    let src = format!("/usr/ports/{}/.sources", package.relpath);

    let extra = &package.data.extra;
    let extra_files: Vec<String> = extra.iter().map(|s| {
        let file_name = Path::new(s.url.as_str())
            .file_name()
            .ufail("File in .sources ends in '..' tf??")
            .to_string_lossy();
        format!("{src}/{file_name}")
    }).collect();
    let tarball_approx = format!("{src}/{package}");

    let mut count = 0;
    for entry in read_dir(src).fail("Failed to read sources directory") {
        let entry = entry.ufail("Invalid source entry");
        let path = entry.path();

        let is_tarball = path.to_string_lossy().starts_with(&tarball_approx);
        let is_extra_file = extra_files.iter().any(|f| Path::new(f) == path);

        if !is_tarball && !is_extra_file {
            vpr!("Pruning: {:?}", path);
            // path should:tm: never point to a dir since it's reading .sources
            remove_file(path).ufail("Failed to prune file");
            count += 1;
        }
    }

    if CONFIG.general.prune_manifests { prune_manifests(package); }
    if CONFIG.general.prune_logs { prune_logs(package); }
    // IDEA: Maybe consider using hashes for the manifests (might be beyond scope though)

    count
}

fn prune_logs(package: &Package) {
    let log_dir_str = format!("/usr/ports/{}/.logs", package.relpath);
    let log_dir = Path::new(&log_dir_str);

    if !log_dir.exists() { return }
    for entry in read_dir(log_dir).fail("Failed to read log directory") {
        let entry = entry.ufail("Invalid directory entry");
        let path = entry.path();

        let is_log = path.file_name().ufail("File in .data ends in '..' tf??").to_string_lossy().ends_with(".log");

        if is_log {
            let msg = format!("Pruning log '{path:?}'");
            vpr!("{}", msg);
            log::debug!("{}", msg);
            remove_file(path).fail("Failed to prune log");
        }
    }
}

fn prune_manifests(package: &Package) {
    let data = format!("/usr/ports/{}/.data", package.relpath);
    
    let protected_manifest = format!("{}/MANIFEST={}", data, package.version);
    for entry in read_dir(data).fail("Failed to read data directory") {
        let entry = entry.ufail("Invalid directory entry");
        let path = entry.path();

        let is_manifest = path.file_name().ufail("File in .data ends in '..' tf??").to_string_lossy().starts_with("MANIFEST=");
        let is_protected = path.to_string_lossy() == protected_manifest;

        if is_manifest && !is_protected {
            let msg = format!("Pruning manifest '{path:?}'");
            vpr!("{}", msg);
            log::debug!("{}", msg);
            remove_file(path).fail("Failed to prune manifest");
        }
    }
}
