// src/package/ambiguity.rs
//
// searches for a package among all the installed repos, prompting when extant in multiple repos

use std::fs;
use crate::utils::fail::Fail;
use walkdir::WalkDir;
use crate::{pr, fail, select, erm};

fn locate(name: &str) -> Vec<String> {
    WalkDir::new("/usr/ports")
        .max_depth(2)
        .into_iter()
        .flatten()
        .filter(|e| {
            !e.path()
                .components()
                .any(|c| c.as_os_str().to_string_lossy().starts_with('.'))
        })
        .filter_map(|e| {
            if e.file_type().is_dir() && e.file_name() == name {
                e.path()
                    .strip_prefix("/usr/ports")
                    .ok()
                    .map(|p| p.to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect()
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

        let Ok(num) = selected.parse::<usize>() else {
            erm!("Invalid input"); continue
        };

        if let Some(m) = matches.get(num) {
            return m.to_string()
        }

        erm!("Selection out of bounds")
    }
}

/// # Description
/// Searches across all repos for a given set
/// Returns an empty vector if no sets are found, otherwise returns a vector of <repo>/@<set>
fn locate_set(set: &str) -> Vec<String> {
    let pattern = format!(".sets/{}", set);
    
    fs::read_dir("/usr/ports")
        .fail("No repos found")
        .filter_map(|r| {
            let repo = r.ok().ufail("Unknown failure in locate_set()").path();

            if repo.join(&pattern).exists() {
                repo.file_name()
                    .map(|name| format!("{}/{}", name.to_string_lossy(), set))
            } else { None }
        })
        .collect()
}

/// # Description
/// Given a set, finds its repository
/// Prompts the user if multiple repositories contain the set
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

        let Ok(num) = selected.parse::<usize>() else {
            erm!("Invalid selection"); continue
        };

        if let Some(m) = matches.get(num) {
            return m.to_string()
        }

        erm!("Selection out of bounds")
    }
}
