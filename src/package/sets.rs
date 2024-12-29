// src/package/sets.rs
//
// adds support for sets

use std::error::Error;
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader};
use crate::{erm, pr};

fn is(package: &str) -> bool {
    package.contains('@')
}

// unravels the special set '@all'
// unravels into all packages in that repo
fn all(repo: &str) -> Vec<String> {
    let dir = format!("/usr/ports/{}/", repo);
    let entries = read_dir(dir).unwrap();

    let packages: Vec<String> = entries
        .filter_map(|entry| {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                let file_name = entry.file_name();
                if file_name.to_string_lossy().starts_with('.') {
                    None
                } else {
                    Some(file_name.into_string().unwrap())
                }
            } else {
                None
            }
            
        })
        .map(|entry| format!("{}/{}", repo, entry)) // remove ambiguity
        .collect();

    packages
}

fn split_repo(str: &str) -> (String, String) {
    let (repo, set) = str.split_once('/').unwrap();
    (repo.to_string(), set.to_string())
}

pub fn unravel(set: &str) -> Result<Vec<String>, Box<dyn Error>> {
    if !is(set) { return Err("Not a set".into()) }

    let (repo, set) = split_repo(set);

    if set == "@all" {
        return Ok(all(&repo));
    }

    let file_path = format!("/usr/ports/{}/.sets/{}", repo, set);
    let file = File::open(file_path).expect("Nonexistent set");
    let buf = BufReader::new(file);

    let lines = buf.lines().collect::<Result<Vec<String>, _>>()?;
    // unless a set explicitly specifies another repo, the given repo is assumed
    let lines = lines
        .into_iter()
        .map(|l| {
            if l.contains('/') { l }
            else { format!("{}/{}", repo, l) }
        })
        .collect();

    Ok(lines)
}

// lists available sets for a repo
pub fn list(repo: &str) {
    let dir = format!("/usr/ports/{}/.sets", repo);
    let entries = match read_dir(dir) {
        Ok(e) => e,
        Err(_) => {
            return erm!("No sets available for '{}/'", repo);
        }
    };

    let available: Vec<String> = entries.map(|f| f.unwrap().file_name().into_string().unwrap()).collect();
    if available.is_empty() {
        return erm!("No sets available for '{}/'", repo);
    }

    available.iter().for_each(|s| pr!("{}", s));
}
