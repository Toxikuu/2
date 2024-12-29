// src/remove/manifest.rs
//
// reads the package manifest

use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::package::Package;

fn locate(dir: &str) -> Vec<PathBuf> {
    let mut manifests = Vec::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(filename) = entry.file_name().to_str() {
                if filename.contains("MANIFEST=") {
                    manifests.push(entry.into_path())
                }
            }
        }
    }

    manifests
}

fn read_lines(path: &PathBuf) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    // reader.lines().filter_map(|l| l.ok()).collect()
    reader.lines().map_while(Result::ok).collect()
}

fn read_all(manifests: Vec<PathBuf>) -> HashMap<PathBuf, Vec<String>> {
    let mut data = HashMap::new();

    for manifest in manifests.iter() {
        let lines = read_lines(manifest);
        data.insert(manifest.to_path_buf(), lines);
    }

    data
}

fn find_unique(all_data: HashMap<PathBuf, Vec<String>>, this_manifest: &PathBuf) -> Vec<String> {
    let mut unique = Vec::new();

    let this_data = all_data.get(this_manifest).expect("Manifest not found");
    let mut all_other_lines = HashSet::new();
    for (path, lines) in &all_data {
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
    let data = read_all(manifests);
    find_unique(data, manifest)
}

// find unique (dead) files in an old manifest
pub fn find_dead_files(package: &Package) -> Vec<String> {
    let manifests = locate(&format!("/usr/ports/{}/{}/.data", package.repo, package.name));
    
    let data = read_all(manifests);
    let old_manifest = Path::new(&format!("/usr/ports/{}/{}/.data/MANIFEST={}", package.repo, package.name, package.data.installed_version)).to_path_buf();

    find_unique(data, &old_manifest)
}
