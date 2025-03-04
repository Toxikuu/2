// src/remove/logic.rs
//! Logic for package removal

use anyhow::{bail, Result};
use crate::{
    comms::out::{pr, erm, vpr},
    globals::{
        config::CONFIG,
        flags::Flags,
    },
    package::Package,
    utils::fail::{BoolFail, Fail},
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
    let category = check_categories(package);
    if !package.data.is_installed && !Flags::grab().force {
        warn!("Not installed: '{package}'");
        erm!("Not installed: '{package}'");
        return false
    }

    if category == Categories::Critical {
        warn!("Refusing to remove critical package: '{package}'");
        erm!("Refusing to remove critical package: '{package}'");
        return false
    }

    if category == Categories::Core {
        warn!("Removing core package: '{package}'");
        erm!("Removing core package: '{package}'");
    }

    let manifest_name = format!("MANIFEST={}", package.version);
    let manifest = package.data.port_dir.join(".data").join(manifest_name);

    manifest.exists().or_fail("Manifest doesn't exist");

    let Ok(unique) = find_unique_paths(&manifest) else {
        warn!("Missing manifest for {package}");
        return false
    };

    let quiet = Flags::grab().quiet;
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

        if !quiet {
            pr!("'{}' -x", p);
        }
    });

    // NOTE: the manifest is not removed as prune handles that
    let status_file = package.data.port_dir.join(".data").join("INSTALLED");
    rmf(&status_file).fail("Failed to remove the status file");

    if CONFIG.removal.remove_sources { remove_sources(package) }
    if CONFIG.removal.remove_dist { remove_dist(package) }

    true
}

/// # Description
/// Removes and recreates ``$PORT/.sources``
fn remove_sources(package: &Package) {
    let srcdir = package.data.port_dir.join(".sources");
    remove_dir_all(&srcdir).fail("Failed to remove .sources");
    create_dir(&srcdir).fail("Failed to recreate .sources");
}

fn remove_dist(package: &Package) {
    let distdir = package.data.port_dir.join(".dist");
    if !distdir.exists() {
        return erm!("Dist dir doesn't exist for '{package}'")
    }

    let Ok(dists) = read_dir(distdir) else {
        return erm!("Failed to read dist dir for '{package}'")
    };

    for d in dists.flatten() {
        let d = d.path();
        if let Err(e) = rmf(&d) {
            warn!("Failed to remove dist '{}': {e}", d.display());
            erm!("Failed to remove dist '{}': {e}", d.display());
        }
    }
}

/// # Description
/// Removes dead files after an update
pub fn remove_dead_files_after_update(package: &Package) {
    if !package.data.is_installed { return erm!("'{}' is not installed!", package) }

    let Ok(dead_files) = find_dead_files(package) else {
        warn!("Missing manifest for '{package}'");
        return
    };

    let quiet = Flags::grab().quiet;
    dead_files.iter().for_each(|p| {
        let pfx = Path::new(&CONFIG.general.prefix);
        let p = p.trim_start_matches('/');
        let path = pfx.join(p);

        if KEPT.iter().any(|&s| path.ends_with(s)) {
            erm!("Retaining protected path: {path:?}");
            return
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

        if !quiet {
            pr!("'{}' -x", p);
        }
    });
}

/// # Description
/// Prunes files for a package
///
/// Pruning involves removing all files from sources except the current tarball and extra files
/// Optionally also removes old manifests and deletes logs
///
/// If force is true, will also prune the current tarball
pub fn prune(package: &Package) -> usize {
    let src_dir = package.data.port_dir.join(".sources");
    if !src_dir.exists() {
        return 0
    }

    let extra_files: Vec<PathBuf> = package.extra.iter()
        .map(|s| {
            let file_name = Path::new(s.url.as_str())
                .file_name()
                .fail("File in .sources ends in '..' tf??");
            src_dir.join(file_name)
        })
        .collect();

    let tarball_approx = src_dir.join(package.to_string());
    let mut pruned_count = 0;

    for entry in read_dir(&src_dir).fail(&format!("Failed to read sources directory '{src_dir:?}'")) {
        let entry = entry.fail("Invalid source entry");
        let path = entry.path();

        if !path.is_file() {
            warn!("Detected non-file {path:?} in {src_dir:?}");
            continue
        }

        let should_keep = path.to_string_lossy().starts_with(&*tarball_approx.to_string_lossy())
            || extra_files.iter().any(|f| f == &path);

        // don't continue if force is passed, meaning the path gets pruned
        if should_keep && !Flags::grab().force {
            continue
        }

        vpr!("Pruning: {:?}", path);
        // path should:tm: never point to a dir since it's reading .sources
        rmf(&path).fail(&format!("Failed to prune file '{path:?}'"));
        pruned_count += 1;
    }

    if CONFIG.removal.prune_manifests { pruned_count += prune_manifests(package) }
    if CONFIG.removal.prune_logs { pruned_count += prune_logs(package) }
    if CONFIG.removal.prune_dist { pruned_count += prune_dist(package) }

    pruned_count
}

/// # Description
/// Deletes all logs for a package
fn prune_logs(package: &Package) -> usize {
    let log_dir = package.data.port_dir.join(".logs");
    if !log_dir.exists() {
        return 0
    }

    let mut pruned_count = 0;
    for entry in read_dir(&log_dir).fail(&format!("Failed to read log directory '{log_dir:?}'")) {
        let entry = entry.fail("Invalid directory entry");
        let path = entry.path();

        if !path.is_file() {
            continue
        }

        if path.file_name().and_then(|f| f.to_str()).is_none()
        || path.extension().is_some_and(|x| x != "log") {
            continue
        }

        let msg = format!("Proning log '{path:?}'");
        vpr!("{msg}");
        log::debug!("{msg}");
        rmf(&path).fail("Failed to prune log");
        pruned_count += 1;
    }
    pruned_count
}

/// # Description
/// Deletes all manifests except the current (and most recent if the installed version and
/// latest version differ) manifest for a package
fn prune_manifests(package: &Package) -> usize {
    let data_dir = package.data.port_dir.join(".data");
    if !data_dir.exists() {
        return 0 // data dir should always exist, but in case it doesn't, give up
    }

    // these manifests aren't pruned, regardless of force
    let protected_manifests = [
        data_dir.join(format!("MANIFEST={}", package.version)),
        data_dir.join(format!("MANIFEST={}", package.data.installed_version)),
    ];

    let mut pruned_count = 0;
    for entry in read_dir(&data_dir).fail(&format!("Failed to read data directory '{data_dir:?}'")) {
        let entry = entry.fail("Invalid directory entry");
        let path = entry.path();

        if !path.is_file() {
            warn!("Detected non-file {path:?} in {data_dir:?}");
            continue
        }

        let Some(file_name) = path.file_name().and_then(|f| f.to_str()) else {
            continue
        };

        if !file_name.starts_with("MANIFEST=")
        || protected_manifests.iter().any(|p| p == &path) {
            continue
        }

        log::debug!("Pruning manifest '{path:?}'");
        rmf(&path).fail("Failed to prune manifest");
        pruned_count += 1;
    }
    pruned_count
}

fn prune_dist(package: &Package) -> usize {
    let dist_dir = package.data.port_dir.join(".dist");
    if !dist_dir.exists() {
        return 0 // data dir should always exist, but in case it doesn't, give up
    }

    let protected_dists = [
        dist_dir.join(format!("{}={}.tar.zst", package.name, package.version)),
        dist_dir.join(format!("{}={}.tar.zst", package.name, package.data.installed_version)),
    ];

    let mut pruned_count = 0;
    for entry in read_dir(&dist_dir).fail(&format!("Failed to read data directory '{dist_dir:?}'")).flatten() {
        let path = entry.path();

        if !path.is_file() {
            warn!("Detected non-file {path:?} in {dist_dir:?}");
            continue
        }

        let Some(file_name) = path.file_name().and_then(|f| f.to_str()) else {
            continue
        };

        if !file_name.ends_with(".tar.zst")
        || protected_dists.iter().any(|p| p == &path) {
            continue
        }

        log::debug!("Pruning dist '{path:?}'");
        rmf(&path).fail("Failed to prune dist");
        pruned_count += 1;
    }
    pruned_count
}

#[derive(PartialEq)]
enum Categories {
    Critical,
    Core,
    Uncategorized,
    Other,
}

fn check_categories(p: &Package) -> Categories {
    p.categories
        .as_ref()
        .map_or(Categories::Uncategorized, |catg| {
            if catg.iter().any(|s| s == "critical") {
                Categories::Critical
            } else if catg.iter().any(|s| s == "core") {
                Categories::Core
            } else {
                Categories::Other
            }
        })
}
