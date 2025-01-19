// src/package/sets.rs
//
// adds support for sets

use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader};
use crate::comms::log::{erm, pr};
use crate::utils::fail::Fail;
use anyhow::{bail, Context, Result};
use std::rc::Rc;

fn is(package: &str) -> bool {
    package.contains('@')
}

// unravels the special set '@all'
// unravels into all packages in that repo
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

fn split_repo(str: &str) -> (String, String) {
    let (repo, set) = str.split_once('/').ufail("I fucked up with split_repo()");
    (repo.to_string(), set.to_string())
}

pub fn unravel(set: &str) -> anyhow::Result<Rc<[String]>> {
    if !is(set) { bail!("Not a set") }

    let (repo, set) = split_repo(set);

    if set == "@all" {
        return all(&repo);
    }

    let file_path = format!("/usr/ports/{repo}/.sets/{set}");
    let file = File::open(file_path).expect("Nonexistent set");
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

// lists available sets for a repo
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
