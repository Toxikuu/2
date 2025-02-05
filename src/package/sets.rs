// src/package/sets.rs
//! Adds support for sets

use anyhow::{bail, Context, Result};
use crate::{
    comms::log::{erm, pr},
    utils::fail::Fail,
};
use std::{
    fs::{read_dir, File},
    io::{BufRead, BufReader},
    path::Path,
    rc::Rc,
};

/// # Description
/// Returns true if a given string is a set
fn is(package: &str) -> bool {
    package.contains('@')
}

/// # Description
/// Unravels the special set '@all', which contains every package in a given repo
/// Output is in the form 'repo/package'
///
/// @all has an alias, @@
fn all(repo: &str) -> Result<Rc<[String]>> {
    let dir = format!("/usr/ports/{repo}/");
    let entries = read_dir(dir).context("Nonexistent repo")?;

    let packages: Rc<[String]> = entries
        .filter_map(|entry| {
            let entry = entry.fail("Failed to read dir entry");
            if entry.file_type().fail("Failed to get entry filetype").is_dir() {
                let file_name = entry.file_name();
                if file_name.to_string_lossy().starts_with('.') {
                    None
                } else {
                    Some(file_name.into_string().fail("Invalid unicode"))
                }
            } else {
                None
            }
            
        })
        .map(|entry| format!("{repo}/{entry}")) // remove ambiguity
        .collect();

    Ok(packages)
}

/// # Description
/// Given a set, returns all member packages
/// Sets are defined in ``/usr/ports/<repo>/.sets/<@set>``
pub fn unravel(set: &str) -> anyhow::Result<Rc<[String]>> {
    if !is(set) { bail!("Not a set") }

    if matches!(set.as_str(), "@!" | "@every") {
        return Ok(every())
    }

    if matches!(set.as_str(), "@o" | "@outdated") {
        return Ok(outdated())
    }

    if matches!(set.as_str(), "@i" | "@installed") {
        return Ok(installed())
    }

    let (repo, set) = set.split_once('/').ufail("No '/' in set");

    if matches!(set.as_str(), "@@" | "@all") {
        return all(repo)
    }

    let file_path = format!("/usr/ports/{repo}/.sets/{set}");
    let file = File::open(file_path).fail("Nonexistent set");
    let buf = BufReader::new(file);

    let lines = buf.lines().collect::<Result<Vec<String>, _>>()?;
    // unless a set explicitly specifies another repo, the given repo is assumed
    let lines = lines
        .into_iter()
        .map(|l| {
            if l.contains('/') { l }
            else { format!("{repo}/{l}") }
        })
        .collect();

    Ok(lines)
}

/// # Description
/// Lists available sets for a repo
pub fn list(repo: &str) {
    let dir = format!("/usr/ports/{repo}/.sets");
    let Ok(entries) = read_dir(dir) else {
        return erm!("No sets available for '{}/'", repo);
    };

    let available: Rc<[String]> = entries.map(|f| f.fail("Failed to read dir entry").file_name().into_string().fail("Invalid unicode")).collect();
    if available.is_empty() {
        return erm!("No sets available for '{}/'", repo);
    }

    available.iter().for_each(|s| pr!("{}", s));
}

/// # Description
/// Unravels the special set '@every', which contains every package in every repo
///
/// @every has an alias, @!
pub fn every() -> Rc<[String]> {
    super::repos::find_all()
        .iter()
        .flat_map(|r| all(r).unwrap_or_default().to_vec())
        .collect()
}

/// # Description
/// Unravels the special set '@installed', which contains every installed package in every repo
///
/// @installed has an alias, @i
pub fn installed() -> Rc<[String]> {
    super::repos::find_all()
        .iter()
        .flat_map(|r| all(r).unwrap_or_default().to_vec())
        .filter(|p| Path::new(&format!("/usr/ports/{p}/.data/INSTALLED")).exists())
        .collect()
}

/// # Description
/// Unravels the special set '@outdated', which contains every outdated package in every repo
///
/// @outdated has an alias, @o
pub fn outdated() -> Rc<[String]> {
    super::repos::find_all()
        .iter()
        .flat_map(|r| all(r).unwrap_or_default().to_vec())
        .filter(|p| {
            let (repo, name) = p.split_once('/').ufail(&format!("Misformatted package {p}"));
            super::Package::new(repo, name).is_outdated()
        })
        .collect()
}

/// # Description
/// Returns true if a given set is a special set
pub fn is_special_set(set: &str) -> bool {
    matches!(set, "@o" | "@outdated" | "@i" | "@installed" | "@!" | "@every" | "@all" | "@@")
} 
