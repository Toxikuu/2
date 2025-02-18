// src/package/repos.rs
//! Functions for dealing with package repos

use crate::{
    comms::out::{erm, pr, msg},
    shell::cmd::exec,
    utils::fail::Fail,
};
use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::{read_dir, read_to_string},
    rc::Rc,
};

/// # Description
/// Returns a vector of all repositories under /usr/ports
pub fn find_all() -> Rc<[String]> {
    let dir = "/usr/ports";
    let entries = read_dir(dir).fail("Error checking for repos");

    let repos: Rc<[String]> = entries.map(|f| f.fail("Invalid entry?").file_name().into_string().fail("Invalid unicode?")).collect();
    if repos.is_empty() {
        erm!("No repos available!");
    }
    repos
}

/// # Description
/// Lists all repositories
pub fn list() {
    find_all().iter().for_each(|r| pr!("{}", r));
}

/// # Description
/// Takes a list of packages in the form repo/name
/// Orders that list according to priority in ``/etc/2/repo_priority.txt``
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
            (Some(_), _) => Ordering::Less,
            (_, Some(_)) => Ordering::Greater,
            (_, _) => ra.cmp(rb),
        }
    });
}

/// # Description
/// Returns the ordered repo priorities from ``/etc/2/repo_priority.txt``
/// Formatted as a vector of repo/
fn get_ordered_repos() -> Vec<String> {
    let contents = read_to_string("/etc/2/repo_priority.txt").fail("Failed to open /etc/2/repo_priority.txt");

    contents
        .lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .map(|s| s.trim_end_matches('/').to_string())
        .collect()
}

/// # Description
/// Takes the url of a git repo and adds it to /usr/ports
/// Requires git to work
pub fn add(repo_url: &str) {
    let (_, repo_name) = repo_url
        .trim_end_matches('/')
        .trim_end_matches(".git")
        .rsplit_once('/')
        .fail("Invalid repo url");

    let (_, repo_name) = repo_name
        .split_once("2-")
        .fail("Invalid repo name");

    // TODO: Consider normalizing git urls
    // Normalization would involve assuming a prefix of https:// if none are provided, as well as
    // assuming the domain of github.com if not provided, allowing users to simply do something
    // like `2 -a toxikuu/2-main`, which would normalize to "https://github.com/toxikuu/2-main.git"
    //
    // This should be done in a separate function
    let command = format!("git clone {repo_url} /usr/ports/{repo_name}");

    msg!("󰐗  Adding '{repo_name}/'...");
    log::info!("Adding '{repo_name}/'...");
    exec(&command).fail("Failed to add repo");
    msg!("󰗠  Added '{repo_name}/'");
    log::info!("Added '{repo_name}/'");
}

/// # Description
/// Syncs an installed git repo. Requires git to work.
pub fn sync(repo: &str) {
    let command = format!("cd /usr/ports/{repo} && git pull");

    msg!("󱍸  Syncing '{repo}'...");
    log::info!("Syncing '{repo}'...");
    exec(&command).fail("Failed to sync repo");
    msg!("󰗠  Synced '{repo}'");
    log::info!("Synced '{repo}'");
}
