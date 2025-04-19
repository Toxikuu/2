// twolib/src/ambiguity.rs

use std::fs;

use tracing::{
    debug,
    instrument,
};
use twocomms::{
    erm,
    pr,
    select,
};
use twoconfig::CONFIG;
use twodebug::d;
use twoerror::{
    BoolFail,
    Fail,
};
use twosh::fs::is_dir;
use walkdir::WalkDir;

use crate::{
    repo::{
        Repo,
        prioritize_repos,
    },
    set::Set,
};

/// # Description
/// Searches across all repos for a given package
/// Returns all packages matching the name in the form 'repo/name'
#[instrument]
fn locate(name: &str) -> Vec<String> {
    WalkDir::new("/var/ports")
        .max_depth(2)
        .into_iter()
        .flatten()
        .filter(|e| {
            !e.path()
                .components()
                .any(|c| c.as_os_str().to_string_lossy().starts_with('.'))
        })
        .filter_map(|e| {
            if is_dir(e.path()).unwrap_or(false) && e.file_name() == name {
                e.path()
                    .strip_prefix("/var/ports")
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
#[instrument]
pub fn resolve_ambiguity(name: &str) -> String {
    let mut matches = locate(name);
    prioritize_repos(&mut matches);

    matches
        .is_empty()
        .and_efail(|| format!("Could not find '{name}' in any repo"));
    if let [only] = matches.as_slice() {
        return only.to_string();
    }

    erm!("Ambiguous: '{name}'");
    for (i, m) in matches.iter().enumerate() {
        pr!("{i}. {m}");
    }

    if CONFIG.general.auto_ambiguity {
        let m = matches
            .first()
            .fail("[UNREACHABLE] Schrodinger's empty vector");
        pr!("Auto-selected '{m}'");
        return m.to_string();
    }

    loop {
        let selected = select!("Choose a package");

        let Ok(num) = selected.parse::<usize>() else {
            erm!("Invalid input");
            continue;
        };

        let Some(m) = matches.get(num) else {
            erm!("Selection out of bounds");
            continue;
        };

        return m.clone();
    }
}

/// # Description
/// Searches across all repos for a given set
/// Returns an empty vector if no sets are found, otherwise returns a vector of Set
#[instrument]
fn locate_set(set: &str) -> Vec<Set> {
    let pattern = format!(".sets/{set}");

    fs::read_dir("/var/ports")
        .fail("No package repos found")
        .filter_map(|r| {
            let repo = r.fail("Failed to read an entry in '/var/ports'").path();

            if repo.join(&pattern).exists() {
                repo.file_name()
                    .map(|name| format!("{}/{set}", name.to_string_lossy()))
            } else {
                None
            }
        })
        .map(|s| Set::new(&s))
        .collect()
}

/// # Description
/// Given a set, finds its repository
/// Prompts the user if multiple repositories contain the set
#[instrument]
pub fn resolve_set_ambiguity(set: &str) -> Repo {
    debug!("Resolving ambiguity for set '{set}'");
    let mut matches: Vec<String> = if Set::is_special_set(set) {
        Repo::find_all()
            .iter()
            .map(|r| format!("{r}/{set}"))
            .collect()
    } else {
        locate_set(set).iter().map(|s| s.to_string()).collect()
    };
    debug!("Found matches for '{set}': '{matches:#?}'");
    d!(matches);

    matches
        .is_empty()
        .and_efail(|| format!("Failed to find '{set}' in any repo"));
    if let [only] = matches.as_slice() {
        d!(only);
        debug!("Matched '{set}' on only arm: '{only}'");
        // BUG: locate_set() returns matches, consisting of Set, which contain `repo/@set`, leading
        // to invalid sets of the format `repo/@set/@set`
        // NOTE: The above bug is somewhat fixed by the below, but it's a cursed solution bc i cba
        // to actually fix it rn
        let (repo_part, _) = only.split_once('/').fail("[UNREACHABLE] Repo without /");
        return Repo::new(repo_part);
    }

    d!("Before prioritization:", matches);
    prioritize_repos(&mut matches);
    d!("After prioritization:", matches);

    erm!("Ambiguous: '{set}'");
    for (i, m) in matches.iter().enumerate() {
        pr!("{i}. {m}");
    }

    if CONFIG.general.auto_ambiguity {
        let m = matches
            .first()
            .fail("[UNREACHABLE] Schrodinger's empty vector");
        pr!("Auto-selected '{m}'");

        d!(m);
        return Repo::new(m);
    }

    loop {
        let selected = select!("Choose a set");

        let Ok(num) = selected.parse::<usize>() else {
            erm!("Invalid selection");
            continue;
        };

        let Some(m) = matches.get(num) else {
            erm!("Selection out of bounds");
            continue;
        };

        d!(m);
        return Repo::new(m);
    }
}
