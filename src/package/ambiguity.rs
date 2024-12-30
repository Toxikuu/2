// src/package/ambiguity.rs
//
// searches for a package among all the installed repos, prompting when extant in multiple repos

use walkdir::WalkDir;
use crate::{pr, die, select, erm};

fn locate(name: &str) -> Vec<String> {
    let mut matches = Vec::new();

    for entry in WalkDir::new("/usr/ports")
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

    if matches.is_empty() { die!("Failed to find '{}' in any repo", name) }
    if let [only] = matches.as_slice() { return only.to_string() }

    erm!("Ambiguous: '{}'", name);
    for (i, m) in matches.iter().enumerate() {
        pr!("{}. {}", i, m)
    }

    let selected = select!("Choose a package");
    let num: usize = selected.parse().unwrap_or_else(|_| die!("Invalid input! (should be a number)"));

    matches.get(num).map(|n| n.to_string()).unwrap_or_else(|| die!("Invalid selection"))
}

fn locate_set(set: &str) -> Vec<String> {
    let mut matches = Vec::new();
    let pattern = format!(".sets/{}", set);

    for entry in WalkDir::new("/usr/ports").into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(filename) = entry.file_name().to_str() {
                if filename == pattern {
                    matches.push(entry.path().to_string_lossy().strip_prefix("/usr/ports/").unwrap().replace(".sets/", "").to_string())
                }
            }
        }
    }

    matches
}

pub fn resolve_set_ambiguity(set: &str) -> String {
    let matches = locate_set(set);

    if matches.is_empty() { die!("Failed to find '{}' in any repo", set) }
    if matches.len() == 1 { return matches.first().unwrap().to_string() }

    for (i, m) in matches.iter().enumerate() {
        pr!("{}. {}", i, m)
    }

    let selected = select!("Choose a set");
    let num: usize = selected.parse().unwrap_or_else(|_| die!("Invalid input! (should be a number)"));

    matches.get(num).map(|n| n.to_string()).unwrap_or_else(|| die!("Invalid selection"))
}
