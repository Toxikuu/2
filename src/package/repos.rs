// src/package/repos.rs
//
// some functions for dealing with package repos

use crate::utils::fail::Fail;
use crate::{erm, pr};
use std::fs::{read_dir, read_to_string};
use std::collections::HashMap;
use std::cmp::Ordering;

pub fn list() {
    let dir = "/usr/ports";
    let entries = read_dir(dir).fail("Error checking for repos");

    let available: Vec<String> = entries.map(|f| f.unwrap().file_name().into_string().unwrap()).collect();
    if available.is_empty() { return erm!("No repos available!") }

    available.iter().for_each(|r| pr!("{}", r));
}

/// # Description
/// Takes a list of packages in the form repo/name
/// Orders that list according to priority in /etc/2/repo_priority.txt
pub fn prioritize(list: &mut [String]) {
    let priorities = get_ordered_repos();
    let repo_priority: HashMap<&str, usize> = priorities
        .iter()
        .enumerate()
        .map(|(i, repo)| (repo.as_str(), i))
        .collect();

    list.sort_by(|a, b| {
        let ra = a.split('/').next().unwrap_or_default();
        let rb = b.split('/').next().unwrap_or_default();

        let pa = repo_priority.get(ra);
        let pb = repo_priority.get(rb);

        match (pa, pb) {
            (Some(a), Some(b)) => a.cmp(b),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => ra.cmp(rb),
        }
    })
}

/// # Description
/// Returns the ordered repo priorities from /etc/2/repo_priority.txt
/// Formatted as a vector of repo/
fn get_ordered_repos() -> Vec<String> {
    let contents = read_to_string("/etc/2/repo_priority.txt").fail("Failed to open /etc/2/repo_priority.txt");

    contents
        .lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .map(|s| s.trim_end_matches('/').to_string())
        .collect()
}
