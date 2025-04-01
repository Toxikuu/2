// src/remove/manifest.rs
//! Reads the package manifest

use std::{
    collections::{
        HashMap,
        HashSet,
    },
    fs,
    path::{
        Path,
        PathBuf,
    },
    rc::Rc,
    string::ToString,
};

use anyhow::{
    Context,
    Result,
};
use tracing::instrument;
use walkdir::{
    DirEntry,
    WalkDir,
};

use crate::{
    package::Package,
    utils::fail::Fail,
};

/// # Description
/// Returns true if none of the directory entry's ancestors contain .data
fn is_in_wrong_hidden(entry: &DirEntry) -> bool {
    entry.path().components().any(|c| {
        let component = c.as_os_str().to_string_lossy();
        component.starts_with('.') && component != ".data"
    })
}

/// # Description
/// Returns true if a directory entry is a manifest
fn is_manifest(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .unwrap_or("")
        .contains("MANIFEST=")
}

/// # Description
/// Finds all manifests
///
/// Ignores entries that aren't manifests and that aren't in .data
///
/// dir is commonly ``/var/ports``, though can also be ``$PORT/.data`` for dead
/// files
#[instrument]
pub fn locate(dir: &str) -> Rc<[PathBuf]> {
    WalkDir::new(dir)
        .max_depth(4)
        .into_iter()
        .filter_entry(|e| !is_in_wrong_hidden(e))
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.file_type().is_file()
                && is_manifest(entry)
                && entry.path().with_file_name("INSTALLED").exists()
        })
        .map(walkdir::DirEntry::into_path)
        .collect::<Rc<[PathBuf]>>()
}

/// # Description
/// Reads manifests and returns a hashmap of their paths and their contents
#[instrument]
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

/// # Description
/// Finds lines (which represent package install paths) unique to this manifest
///
/// Backend for ``find_unique_paths``
///
/// Returns the unique lines in reverse order (meaning /path/to/file is above /path/to)
#[instrument]
fn find_unique(
    all_data: &HashMap<PathBuf, Rc<[String]>>,
    this_manifest: &PathBuf,
) -> Result<Rc<[String]>> {
    let this_data = all_data.get(this_manifest).context("Missing manifest")?;
    let all_other_lines: HashSet<_> = all_data
        .iter()
        .filter(|(path, _)| *path != this_manifest)
        .flat_map(|(_, lines)| lines.iter())
        .collect();

    let unique = this_data
        .iter()
        .filter(|l| !all_other_lines.contains(l))
        .cloned()
        .rev()
        .collect::<Rc<[String]>>();

    Ok(unique)
}

/// # Description
/// Finds paths unique to a manifest
pub fn find_unique_paths(manifest: &PathBuf) -> Result<Rc<[String]>> {
    let manifests = locate("/var/ports");
    let data = read_all(&manifests);
    find_unique(&data, manifest)
}

/// # Description
/// Finds unique files in an old manifest (dead files)
#[instrument]
pub fn find_dead_files(package: &Package) -> Result<Rc<[String]>> {
    let manifests = locate(&format!(
        "/var/ports/{}/{}/.data",
        package.repo, package.name
    ));

    let data = read_all(&manifests);
    let old_manifest = Path::new(&format!(
        "/var/ports/{}/{}/.data/MANIFEST={}",
        package.repo, package.name, package.data.installed_version
    ))
    .to_path_buf();

    find_unique(&data, &old_manifest)
}
