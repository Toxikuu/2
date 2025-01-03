// src/package/ambiguity.rs
//
// searches for a package among all the installed repos, prompting when extant in multiple repos

use walkdir::WalkDir;
use crate::{pr, fail, select, erm};

fn locate(name: &str) -> Vec<String> {
    let mut matches = Vec::new();

    for entry in WalkDir::new("/usr/ports")
        .max_depth(2)
        .into_iter()
        .flatten()
        .filter(|e| {
            !e.path()
            .components()
            .any(|component| component.as_os_str().to_string_lossy().starts_with('.'))
        })
    {
        if entry.file_type().is_dir() {
            let filename = entry.file_name().to_string_lossy();
            if filename == name {
                matches.push(
                    entry.path()
                        .to_string_lossy()
                        .strip_prefix("/usr/ports/")
                        .unwrap()
                        .to_string()
                )
            }
        }
    }

    matches
}

pub fn resolve_ambiguity(name: &str) -> String {
    let matches = locate(name);

    if matches.is_empty() { fail!("Failed to find '{}' in any repo", name) }
    if let [only] = matches.as_slice() { return only.to_string() }

    erm!("Ambiguous: '{}'", name);
    for (i, m) in matches.iter().enumerate() {
        pr!("{}. {}", i, m)
    }

    loop {
        let selected = select!("Choose a package");

        if let Ok(num) = selected.parse::<usize>() {
            if let Some(m) = matches.get(num) {
                return m.to_string()
            } else {
                erm!("Selection out of bounds")
            }
        } else {
            erm!("Invalid input")
        }
    }
}

// TODO: use these
fn locate_set(set: &str) -> Vec<String> {
    let mut matches = Vec::new();
    let pattern = format!(".sets/{}", set);

    for entry in WalkDir::new("/usr/ports")
        .max_depth(2)
        .into_iter()
        .flatten()
        .filter(|e| {
            !e.path()
            .components()
            .any(|component|
                component.as_os_str().to_string_lossy().starts_with('.')
            ) && !e.path().is_dir()
        })
    {
        let filename = entry.file_name().to_string_lossy();
        if filename == pattern {
            matches.push(
                entry.path()
                    .to_string_lossy()
                    .strip_prefix("/usr/ports/")
                    .unwrap()
                    .replace(".sets/", "")
                    .to_string()
            );
        }
    }

    matches
}

// fn locate_set(set: &str) -> Vec<String> {
//     let mut matches = Vec::new();
//     let pattern = format!(".sets/{}", set);
//
//     for entry in WalkDir::new("/usr/ports").max_depth(2).into_iter().filter_map(|e| e.ok()) {
//         if entry.file_type().is_file() {
//             if let Some(filename) = entry.file_name().to_str() {
//                 if filename == pattern {
//                     matches.push(entry.path().to_string_lossy().strip_prefix("/usr/ports/").unwrap().replace(".sets/", "").to_string())
//                 }
//             }
//         }
//     }
//
//     matches
// }

pub fn resolve_set_ambiguity(set: &str) -> String {
    let matches = locate_set(set);

    if matches.is_empty() { fail!("Failed to find '{}' in any repo", set) }
    if matches.len() == 1 { return matches.first().unwrap().to_string() }

    erm!("Ambiguous: '{}'", set);
    for (i, m) in matches.iter().enumerate() {
        pr!("{}. {}", i, m)
    }

    loop {
        let selected = select!("Choose a set");

        if let Ok(num) = selected.parse::<usize>() {
            if let Some(m) = matches.get(num) {
                return m.to_string()
            } else {
                erm!("Selection out of bounds")
            }
        } else {
            erm!("Invalid input")
        }
    }
}
