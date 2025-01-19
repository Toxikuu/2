// src/remove/manifest.rs
//
// reads the package manifest

use crate::comms::log::vpr;
use crate::package::Package;
use crate::utils::fail::Fail;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::result::Result;
use walkdir::{DirEntry, WalkDir};

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

// TODO: Rewrite this without mut vec
fn locate(dir: &str) -> Rc<[PathBuf]> {
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
    manifests.into()
}

fn read_lines(path: &PathBuf) -> Rc<[String]> {
    let file = File::open(path).fail("Failed to open file");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}

fn read_all(manifests: &[PathBuf]) -> HashMap<PathBuf, Rc<[String]>> {
    let mut data = HashMap::new();

    for manifest in manifests {
        let lines = read_lines(manifest);
        data.insert(manifest.clone(), lines);
    }

    data
}

// TODO: rewrite this without a mutable vec
fn find_unique(all_data: &HashMap<PathBuf, Rc<[String]>>, this_manifest: &PathBuf) -> Rc<[String]> {
    let mut unique = Vec::new();

    let this_data = all_data.get(this_manifest).expect("Manifest not found");
    let mut all_other_lines = HashSet::new();
    for (path, lines) in all_data {
        if path != this_manifest {
            
            lines
                .iter()
                .for_each(|l| {
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
    unique.into()
}

pub fn find_unique_paths(manifest: &PathBuf) -> Rc<[String]> {
    let manifests = locate("/usr/ports");
    let data = read_all(&manifests);
    find_unique(&data, manifest)
}

// find unique (dead) files in an old manifest
pub fn find_dead_files(package: &Package) -> Rc<[String]> {
    let manifests = locate(&format!("/usr/ports/{}/{}/.data", package.repo, package.name));
    
    let data = read_all(&manifests);
    let old_manifest = Path::new(&format!("/usr/ports/{}/{}/.data/MANIFEST={}", package.repo, package.name, package.data.installed_version)).to_path_buf();

    find_unique(&data, &old_manifest)
}
