// src/package/sets.rs
//
// adds support for sets

use std::error::Error;
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader};
use crate::{erm, pr};
use crate::utils::fail::Fail;

fn is(package: &str) -> bool {
    package.contains('@')
}

// unravels the special set '@all'
// unravels into all packages in that repo
fn all(repo: &str) -> Vec<String> {
    let dir = format!("/usr/ports/{repo}/");
    let entries = read_dir(dir).fail("Nonexistent repo");

    let packages: Vec<String> = entries
        .filter_map(|entry| {
            let entry = entry.ufail("Invalid entry?");
            if entry.file_type().ufail("Failed to get entry filetype").is_dir() {
                let file_name = entry.file_name();
                if file_name.to_string_lossy().starts_with('.') {
                    None
                } else {
                    Some(file_name.into_string().ufail("Invalid unicode"))
                }
            } else {
                None
            }
            
        })
        .map(|entry| format!("{repo}/{entry}")) // remove ambiguity
        .collect();

    packages
}

fn split_repo(str: &str) -> (String, String) {
    let (repo, set) = str.split_once('/').ufail("I fucked up with split_repo()");
    (repo.to_string(), set.to_string())
}

pub fn unravel(set: &str) -> Result<Vec<String>, Box<dyn Error>> {
    if !is(set) { return Err("Not a set".into()) }

    let (repo, set) = split_repo(set);

    if set == "@all" {
        return Ok(all(&repo));
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

    let available: Vec<String> = entries.map(|f| f.ufail("Invalid entry?").file_name().into_string().ufail("Invalid unicode")).collect();
    if available.is_empty() {
        return erm!("No sets available for '{}/'", repo);
    }

    for s in &available {
        pr!("{}", s);
    }
}
