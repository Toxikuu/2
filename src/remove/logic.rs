// src/remove/logic.rs
//! Logic for package removal

use anyhow::{bail, Result};
use crate::{
    comms::log::{erm, cpr, vpr},
    globals::{
        config::CONFIG,
        flags::FLAGS,
    },
    package::Package,
    utils::fail::{fail, Fail},
};
use log::warn;
use std::{
    fs::{
        create_dir,
        read_dir,
        remove_dir,
        remove_dir_all,
        remove_file,
    },
    io::ErrorKind as IOE,
    path::{Path, PathBuf},
};
use super::manifest::{find_dead_files, find_unique_paths};

// TODO: Consider using glob patterns for the below, allowing to protect against removal of boot/*
// for instance
//
/// # Description
/// Paths that are protected against removal no matter what
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

/// # Description
/// Removes a directory. Ignores attempts to remove missing or populated directories.
///
/// Propagates any other io error
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

/// # Description
/// Removes a file. Ignores attempts to remove missing files.
///
/// Propagates any other io error
fn rmf(path: &PathBuf) -> Result<()> {
    if let Err(e) = remove_file(path) {
        match e.kind() {
            IOE::NotFound => erm!("Ignoring '{}': missing", path.display()),
            _ => bail!("Failed to remove '{}': {}", path.display(), e)
        }
    }
    Ok(())
}

/// # Description
/// Removes a package
///
/// Removal entails reading a package's manifest and removing unique files. Some paths are
/// protected against removal, even if they're unique.
///
/// Returns false if the package isn't installed
/// Affected by quiet (I think; TODO: Confirm this)
///
/// **Fail Conditions:**
/// - the manifest doesn't exist
/// - failed to remove a specific path (see ``rmf()`` and ``rmdir``)
pub fn remove(package: &Package) -> bool {
    if !package.data.is_installed && !FLAGS.get().ufail("Cell issue").force {
        erm!("Not installed: '{}'", package);
        return false
    }

    let manifest = format!("/usr/ports/{}/{}/.data/MANIFEST={}", package.repo, package.name, package.version);
    let manifest_path = Path::new(&manifest);

    if !manifest_path.exists() { fail!("Manifest doesn't exist") }

    let Ok(unique) = find_unique_paths(&manifest_path.to_path_buf()) else { 
        warn!("Missing manifest for {package}");
        return false
    };

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

        cpr!("'{}' -x", p);
    });

    // NOTE: the manifest is not removed as prune will handle it
    let status_file = format!("/usr/ports/{}/{}/.data/INSTALLED", package.repo, package.name);
    remove_file(status_file).fail("Failed to remove the status file");

    if CONFIG.removal.remove_sources { remove_sources(package) }
    if CONFIG.removal.remove_dots { remove_dots(package) }

    true
}

/// # Description
/// Removes and recreates ``$PORT/.sources``
fn remove_sources(package: &Package) {
    let srcdir = format!("/usr/ports/{}/{}/.sources", package.repo, package.name);
    remove_dir_all(&srcdir).ufail("Failed to remove .sources");
    create_dir(&srcdir).ufail("Failed to recreate .sources");
}

/// # Description
/// Removes and recreates ``$PORT/.dist`` and ``$PORT/.data``
fn remove_dots(package: &Package) {
    let portdir_str = format!("/usr/ports/{}/{}", package.repo, package.name);
    let portdir = Path::new(&portdir_str);

    // lazy rm -rf .d{ata,ist}/{,.}*
    // these should never fail (unless maybe .data doesnt exist [which shouldn't happen anyway])
    remove_dir_all(portdir.join(".data")).ufail("Failed to remove .data");
    create_dir(portdir.join(".data")).ufail("Failed to recreate .data");

    remove_dir_all(portdir.join(".dist")).ufail("Failed to remove .dist");
    create_dir(portdir.join(".dist")).ufail("Failed to recreate .dist");
}

/// # Description
/// Removes dead files after an update
pub fn remove_dead_files_after_update(package: &Package) {
    if !package.data.is_installed { return erm!("'{}' is not installed!", package) }

    let Ok(dead_files) = find_dead_files(package) else { 
        warn!("Missing manifest for {package}");
        return
    };

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

        cpr!("'{}' -x", p);
    });
}

/// # Description
/// Prunes files for a package
///
/// Pruning involves removing all files from sources except the current tarball and extra files
/// Optionally also removes old manifests and deletes logs
pub fn prune(package: &Package) -> usize {
    let src_dir = PathBuf::from("/usr/ports")
        .join(&package.relpath)
        .join(".sources");

    if !src_dir.exists() {
        return 0
    }

    let extra_files: Vec<PathBuf> = package.extra.iter()
        .map(|s| {
            let file_name = Path::new(s.url.as_str())
                .file_name()
                .ufail("File in .sources ends in '..' tf??");
            src_dir.join(file_name)
        })
        .collect();

    let tarball_approx = src_dir.join(package.to_string());
    let mut pruned_count = 0;

    for entry in read_dir(&src_dir).fail(&format!("Failed to read sources directory '{src_dir:?}'")) {
        let entry = entry.ufail("Invalid source entry");
        let path = entry.path();

        if !path.is_file() {
            continue
        }

        let should_keep = path.to_string_lossy().starts_with(&*tarball_approx.to_string_lossy())
            || extra_files.iter().any(|f| f == &path);

        if should_keep {
            continue
        }

        vpr!("Pruning: {:?}", path);
        // path should:tm: never point to a dir since it's reading .sources
        remove_file(&path).fail(&format!("Failed to prune file '{path:?}'"));
        pruned_count += 1;
    }

    if CONFIG.general.prune_manifests { prune_manifests(package) }
    if CONFIG.general.prune_logs { prune_logs(package) }
    // IDEA: Maybe consider using hashes for the manifests (might be beyond scope though)

    pruned_count
}

/// # Description
/// Deletes all logs for a package
fn prune_logs(package: &Package) {
    let log_dir = PathBuf::from("/usr/ports")
        .join(&package.relpath)
        .join(".logs");

    if !log_dir.exists() {
        return
    }

    for entry in read_dir(&log_dir).fail(&format!("Failed to read log directory '{log_dir:?}'")) {
        let entry = entry.ufail("Invalid directory entry");
        let path = entry.path();

        if !path.is_file() {
            continue
        }

        if path.file_name().and_then(|f| f.to_str()).is_none() {
            continue
        }

        if path.extension().is_some_and(|x| x != "log") {
            continue
        }

        let msg = format!("Proning log '{path:?}'");
        vpr!("{msg}");
        log::debug!("{msg}");
        remove_file(&path).fail("Failed to prune log");
    }
}

/// # Description
/// Deletes all manifests except the current (and most recent if the installed version and
/// latest version differ) manifest for a package
fn prune_manifests(package: &Package) {
    let data_dir = PathBuf::from("/usr/ports")
        .join(&package.relpath)
        .join(".data");

    let protected_manifests = [
        data_dir.join(format!("MANIFEST={}", package.version)),
        data_dir.join(format!("MANIFEST={}", package.data.installed_version)),
    ];

    if !data_dir.exists() {
        return // data dir should always exist, but in case it doesn't, give up
    }

    for entry in read_dir(&data_dir).fail(&format!("Failed to read data directory '{data_dir:?}'")) {
        let entry = entry.ufail("Invalid directory entry");
        let path = entry.path();

        if !path.is_file() {
            continue
        }

        let Some(file_name) = path.file_name().and_then(|f| f.to_str()) else {
            continue
        };

        if !file_name.starts_with("MANIFEST=") {
            continue
        }

        if protected_manifests.iter().any(|p| p == &path) {
            continue
        }

        log::debug!("Pruning manifest '{path:?}'");
        remove_file(&path).fail("Failed to prune manifest");
    }
}
