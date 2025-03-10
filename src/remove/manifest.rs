// src/remove/manifest.rs
//! Reads the package manifest

use anyhow::{Context, Result};
use crate::{
    comms::out::vpr,
    package::Package,
    utils::fail::Fail,
};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
    rc::Rc,
    string::ToString,
};
use walkdir::{DirEntry, WalkDir};

/// # Description
/// Returns true if none of the directory entry's ancestors contain .data
fn is_in_wrong_hidden(entry: &DirEntry) -> bool {
    entry.path()
        .components()
        .any(|c| {
            let component = c.as_os_str().to_string_lossy();
            component.starts_with('.') && component != ".data"
        })
}

/// # Description
/// Returns true if a directory entry is a manifest
fn is_manifest(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .unwrap_or("")
        .contains("MANIFEST=")
}

// TODO: Rewrite this without mut vec
//
/// # Description
/// Finds all manifests
///
/// Ignores entries that aren't manifests and that aren't in .data
///
/// dir is commonly ``/usr/ports``, though can also be ``$PORT/.data`` for dead files
pub fn locate(dir: &str) -> Rc<[PathBuf]> {
    let mut manifests = Vec::new();

    for entry in WalkDir::new(dir)
        .max_depth(4)
        .into_iter()
        .filter_entry(|e| !is_in_wrong_hidden(e))
        .filter_map(Result::ok)
    {
        vpr!("ENTRY: {:?}", entry);
        if entry.file_type().is_file()
            && is_manifest(&entry)
            && entry.path().with_file_name("INSTALLED").exists()
        {
            manifests.push(entry.into_path());
        }
    }

    vpr!("Located manifests: {:#?}", manifests);
    manifests.into()
}

/// # Description
/// Reads manifests and returns a hashmap of their paths and their contents
fn read_all(manifests: &[PathBuf]) -> HashMap<PathBuf, Rc<[String]>> {
    let mut data = HashMap::new();

    for manifest in manifests {
        let contents = fs::read_to_string(manifest)
            .efail(|| format!("Failed to open manifest '{}'", manifest.display()));
        let lines: Rc<[String]> = contents.lines().map(ToString::to_string).collect();
        data.insert(manifest.clone(), lines);
    }

    data
}

// TODO: rewrite this without a mutable vec
//
/// # Description
/// Finds lines (which represent package install paths) unique to this manifest
///
/// Backend for ``find_unique_paths``
///
/// Returns the unique lines in reverse order (meaning /path/to/file is above /path/to)
fn find_unique(all_data: &HashMap<PathBuf, Rc<[String]>>, this_manifest: &PathBuf) -> Result<Rc<[String]>> {
    let mut unique = Vec::new();

    // TODO: Consider not failing, rather just warning the user when a manifest isn't found
    let this_data = all_data.get(this_manifest).context("Missing manifest")?;
    let mut all_other_lines = HashSet::new();

    for (path, lines) in all_data {
        if path != this_manifest {
            lines.iter().for_each(|l| {
                all_other_lines.insert(l);
            });
        }
    }

    this_data.iter().for_each(|l| {
        if !all_other_lines.contains(l) {
            unique.push(l.clone());
        }
    });

    unique.reverse();
    Ok(unique.into())
}

/// # Description
/// Finds paths unique to a manifest
pub fn find_unique_paths(manifest: &PathBuf) -> Result<Rc<[String]>> {
    let manifests = locate("/usr/ports");
    let data = read_all(&manifests);
    find_unique(&data, manifest)
}

/// # Description
/// Finds unique files in an old manifest (dead files)
pub fn find_dead_files(package: &Package) -> Result<Rc<[String]>> {
    let manifests = locate(&format!("/usr/ports/{}/{}/.data", package.repo, package.name));

    let data = read_all(&manifests);
    let old_manifest = Path::new(&format!("/usr/ports/{}/{}/.data/MANIFEST={}", package.repo, package.name, package.data.installed_version)).to_path_buf();

    find_unique(&data, &old_manifest)
}
