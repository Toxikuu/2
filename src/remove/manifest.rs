// src/remove/manifest.rs
//
// reads the package manifest

use crate::comms::log::vpr;
use walkdir::{DirEntry, WalkDir};
use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::package::Package;
use std::result::Result;
use crate::utils::fail::Fail;

fn is_in_wrong_hidden(entry: &DirEntry) -> bool {
    entry.path()
        .components()
        .any(|c| {
            let component = c.as_os_str().to_string_lossy();
            component.starts_with('.') && component != ".data"
        })
}

fn is_manifest(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .unwrap_or("")
        .contains("MANIFEST=")
}

// TODO: consider generating manifests from extracted tarballs, instead of from builds
// probably optimal because distributors would rather not have to ship manifests as well as tarballs
fn locate(dir: &str) -> Vec<PathBuf> {
    let mut manifests = Vec::new();

    for entry in WalkDir::new(dir)
        .max_depth(4)
        .into_iter()
        .filter_entry(|e| !is_in_wrong_hidden(e))
        .filter_map(Result::ok)
    {
        vpr!("ENTRY: {:?}", entry);
        if entry.file_type().is_file() && is_manifest(&entry) {
            let installed_path = entry.path().with_file_name("INSTALLED");
            if installed_path.exists() {
                manifests.push(entry.into_path());
            }
        }
    }

    vpr!("Located manifests: {:#?}", manifests);
    manifests
}

fn read_lines(path: &PathBuf) -> Vec<String> {
    let file = File::open(path).fail("Failed to open file");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}

fn read_all(manifests: &[PathBuf]) -> HashMap<PathBuf, Vec<String>> {
    let mut data = HashMap::new();

    for manifest in manifests {
        let lines = read_lines(manifest);
        data.insert(manifest.clone(), lines);
    }

    data
}

fn find_unique(all_data: &HashMap<PathBuf, Vec<String>>, this_manifest: &PathBuf) -> Vec<String> {
    let mut unique = Vec::new();

    let this_data = all_data.get(this_manifest).expect("Manifest not found");
    let mut all_other_lines = HashSet::new();
    for (path, lines) in all_data {
        if path != this_manifest {
            for line in lines {
                all_other_lines.insert(line);
            }
        }
    }

    for line in this_data {
        if !all_other_lines.contains(line) {
            unique.push(line.clone());
        }
    }

    unique
}

pub fn find_unique_paths(manifest: &PathBuf) -> Vec<String> {
    let manifests = locate("/usr/ports");
    let data = read_all(&manifests);
    find_unique(&data, manifest)
}

// find unique (dead) files in an old manifest
pub fn find_dead_files(package: &Package) -> Vec<String> {
    let manifests = locate(&format!("/usr/ports/{}/{}/.data", package.repo, package.name));
    
    let data = read_all(&manifests);
    let old_manifest = Path::new(&format!("/usr/ports/{}/{}/.data/MANIFEST={}", package.repo, package.name, package.data.installed_version)).to_path_buf();

    find_unique(&data, &old_manifest)
}
