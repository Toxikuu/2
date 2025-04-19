// twolib/src/repo.rs
//! Functions related to repos

use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt,
    fs::{
        read_dir,
        read_to_string,
    },
    path::PathBuf,
    rc::Rc,
};

use tracing::instrument;
use twocomms::{
    msg,
    pr,
};
use twodebug::d;
use twoerror::Fail;
use twosh::sh::exec;

const REPODIR: &str = "/var/ports";
const REPOPRIORITY: &str = "/etc/2/repo_priority.txt";

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Repo {
    name: String,
    path: PathBuf,
}

impl fmt::Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}/", self.name)
    }
}

impl Repo {
    pub fn new(name: &str) -> Self {
        let name = name.trim_end_matches('/');
        Self {
            name: name.to_string(),
            path: PathBuf::from(REPODIR).join(name),
        }
    }

    pub fn find_all() -> Rc<[Repo]> {
        read_dir(REPODIR)
            .fail("Could not read /var/ports")
            .map_while(Result::ok)
            .map(|f| f.file_name().to_string_lossy().to_string())
            .map(|f| Repo::new(&f))
            .inspect(|r| d!("Found repo", r))
            .collect::<Rc<_>>()
    }

    pub fn list_all() { Self::find_all().iter().for_each(|r| pr!("{r}")); }

    pub fn add(upstream: &str) { RepoUpstream::add(upstream) }

    pub fn sync(&self) {
        let command = format!("cd {:?} && git pull", self.path);

        msg!("󱓎 Syncing '{}'...", self.name);
        exec(&command, None).fail("Failed to sync repo");
        msg!("󰄹 Synced '{}'", self.name);
    }
}

#[derive(Debug)]
pub struct RepoUpstream<'a> {
    upstream: &'a str,
}

impl<'a> RepoUpstream<'a> {
    fn is_short(upstream_url: &str) -> bool {
        upstream_url.chars().filter(|c| *c == '/').count() == 1
    }

    // TODO: Test this
    pub fn add(upstream_url: &str) {
        let (start, mut name) = upstream_url
            .trim_end_matches('/')
            .trim_end_matches(".git")
            .rsplit_once('/')
            .fail("Invalid repo url");

        (_, name) = name.split_once("2-").fail("Invalid repo name");

        let command = if Self::is_short(upstream_url) {
            format!("git clone https://github.com/{start}/2-{name}.git {REPODIR}/{name}")
        } else {
            format!("git clone {upstream_url} {REPODIR}/{name}")
        };

        // TODO: Use a progress bar here
        msg!("󱓊 Adding '{name}/'...");
        exec(&command, None).fail("Failed to add new repo");
        msg!("󰄹 Added '{name}/'");
    }
}

/// # Description
/// Takes a list of packages in the form repo/name
/// Orders that list according to priority in ``/etc/2/repo_priority.txt``
#[instrument]
// NOTE: This intentionally uses Vec<String> instead of Vec<Repo> to more easily work with both
// Package and Set
pub fn prioritize_repos(repos: &mut [String]) {
    let priorities = read_repo_priority();
    let repo_priority: HashMap<&str, usize> = priorities
        .iter()
        .enumerate()
        .map(|(i, repo)| (repo.as_str(), i))
        .collect();

    repos.sort_by(|a, b| {
        let ra = a.split('/').next().unwrap_or_default();
        let rb = b.split('/').next().unwrap_or_default();

        let pa = repo_priority.get(ra);
        let pb = repo_priority.get(rb);

        match (pa, pb) {
            | (Some(a), Some(b)) => a.cmp(b),
            | (Some(_), _) => Ordering::Less,
            | (_, Some(_)) => Ordering::Greater,
            | (..) => ra.cmp(rb),
        }
    });
}

/// # Description
/// Returns the ordered repo priorities from the `REPOPRIORITY` file
#[instrument]
fn read_repo_priority() -> Vec<String> {
    let contents = read_to_string(REPOPRIORITY).efail(|| format!("Failed to read {REPOPRIORITY}"));

    contents
        .lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .map(|s| s.trim_end_matches('/').to_string())
        .collect()
}
