// src/package/ambiguity.rs
//! Responsible for resolving ambiguity in packages and sets

use crate::{
    comms::{
        r#in::select, log::{erm, pr}
    },
    globals::config::CONFIG,
    package::repos::{self, prioritize},
    utils::fail::{fail, Fail}
};
use std::fs;
use walkdir::WalkDir;

/// # Description
/// Searches across all repos for a given package
/// Returns all packages matching the name in the form 'repo/name'
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

/// # Description
/// Given a package, finds its repository
/// Prompts the user if multiple repositories contain the package
pub fn resolve_ambiguity(name: &str) -> String {
    let mut matches = locate(name);
    prioritize(&mut matches);

    if matches.is_empty() { fail!("Failed to find '{}' in any repo", name) }
    if let [only] = matches.as_slice() { return only.to_string() }

    erm!("Ambiguous: '{}'", name);
    for (i, m) in matches.iter().enumerate() {
        pr!("{}. {}", i, m);
    }

    if CONFIG.general.auto_ambiguity {
        let m = matches.first().ufail("Schrodinger's empty vector");
        pr!("Auto-selected '{}'", m);
        return m.to_string()
    }

    // i went for maximum readability with this loop (and the next one)
    // literally none of the rest of this codebase is readable LOL
    // i just felt like having fun here :)
    loop {
        let selected = select!("Choose a package");

        let Ok(num) = selected.parse::<usize>() else {
            erm!("Invalid input"); continue
        };

        let Some(m) = matches.get(num) else {
            erm!("Selection out of bounds"); continue
        };

        return m.clone()
    }
}

/// # Description
/// Searches across all repos for a given set
/// Returns an empty vector if no sets are found, otherwise returns a vector of <repo>/@<set>
fn locate_set(set: &str) -> Vec<String> {
    let pattern = format!(".sets/{set}");

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
    let mut matches =
        if super::sets::Set::is_special_set(set) {
            let repos = repos::find_all();
            repos.iter()
                .map(|r| format!("{r}/{set}"))
                .collect::<Vec<_>>()
        } else {
            locate_set(set)
        };

    if matches.is_empty() {
        fail!("Failed to find '@{}' in any repo", set);
    }

    if let [only] = matches.as_slice() { return only.to_string() }

    prioritize(&mut matches);

    erm!("Ambiguous: '{}'", set);
    for (i, m) in matches.iter().enumerate() {
        pr!("{}. {}", i, m);
    }

    if CONFIG.general.auto_ambiguity {
        let m = matches.first().ufail("Schrodinger's empty vector");
        pr!("Auto-selected '{}'", m);
        return m.clone()
    }

    loop {
        let selected = select!("Choose a set");

        let Ok(num) = selected.parse::<usize>() else {
            erm!("Invalid selection"); continue
        };

        let Some(m) = matches.get(num) else {
            erm!("Selection out of bounds"); continue
        };

        return m.clone()
    }
}
