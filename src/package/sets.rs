// src/package/sets.rs
//! Adds support for sets

use std::{
    fmt::{
        self,
        Display,
        Formatter,
    },
    fs::{
        File,
        read_dir,
    },
    io::{
        BufRead,
        BufReader,
    },
    path::Path,
    rc::Rc,
};

use anyhow::Result;

use super::{
    ambiguity::resolve_set_ambiguity,
    repos,
};
use crate::{
    comms::out::{
        erm,
        pr,
        vpr,
    },
    utils::fail::{
        BoolFail,
        Fail,
    },
};

/// # Description
/// Set struct
/// Repo is of the format 'repo'
/// Set is of the format 'set'
///
/// Displays as 'repo/set'
/// Pretty displays as 'repo/@set'
#[derive(Clone, Debug)]
pub struct Set {
    repo: String,
    set:  String,
}

impl Display for Set {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result { write!(f, "{}/{}", self.repo, self.set) }
}

impl Set {
    pub fn new(str: &str) -> Self {
        Self::is(str).or_efail(|| format!("Not a set: '{str}'"));

        // handle all repos
        if let Some(set) = str.strip_prefix("//") {
            return Self {
                repo: "//".to_string(),
                set:  set.to_string(),
            };
        }

        if let Some((repo, set)) = str.split_once('/') {
            Self {
                repo: repo.to_string(),
                set:  set.to_string(),
            }
        } else {
            let stupid_intermediate = resolve_set_ambiguity(str);
            let (repo, _) = stupid_intermediate
                .split_once('/')
                .fail("[UNREACHABLE] Somehow resolved ambiguity without returning repo/set");
            Self {
                repo: repo.to_string(),
                set:  str.to_string(),
            }
        }
    }

    pub fn dirs(&self) -> Rc<[String]> {
        let repo = &self.repo;
        if repo == "//" {
            repos::find_all()
                .iter()
                .map(|r| format!("/var/ports/{r}"))
                .collect()
        } else {
            [format!("/var/ports/{repo}")].into()
        }
    }

    /// # Description
    /// Returns true if a given string is a set
    pub fn is(str: &str) -> bool { str.contains('@') }

    /// # Description
    /// Returns true if a given string is a special set
    fn is_special(&self) -> bool {
        matches!(
            self.set.as_str(),
            "@@" | "@all" | "@o" | "@outdated" | "@i" | "@installed" | "@a" | "@available"
        )
    }

    pub fn is_special_set(set: &str) -> bool {
        matches!(
            set,
            "@@" | "@all" | "@o" | "@outdated" | "@i" | "@installed" | "@a" | "@available"
        )
    }

    fn unravel_special(&self) -> Rc<[String]> {
        let set = self.set.as_str();
        if matches!(set, "@o" | "@outdated") {
            self.outdated()
        } else if matches!(set, "@i" | "@installed") {
            self.installed()
        } else if matches!(set, "@a" | "@available") {
            self.available()
        } else if matches!(set, "@@" | "@all") {
            self.all()
        } else {
            unreachable!("I forgot to add a special set")
        }
    }

    /// # Description
    /// Given a set, returns all member packages
    /// Sets are defined in ``/var/ports/<repo>/.sets/<@set>``
    // TODO: Implement and test set recursion
    pub fn unravel(&self) -> Rc<[String]> {
        let set = &self.set;
        let repo = &self.repo;
        vpr!("Unraveling set:\n{self:#?}");
        log::debug!("Unraveling '{self}'");

        if self.is_special() {
            return self.unravel_special();
        }

        let file_path = format!("/var/ports/{repo}/.sets/{set}");
        let file = File::open(file_path).efail(|| format!("Set '{self}' does not exist"));
        let buf = BufReader::new(file);

        buf.lines()
            .map_while(Result::ok)
            .filter(|l| !(l.starts_with('#') || l.is_empty()))
            .map(|l| if l.contains('/') { l } else { format!("{repo}/{l}") })
            .collect()
    }

    /// # Description
    /// Unravels the special set '@all', which contains every package in a given repo
    /// Output is in the form 'repo/package'
    ///
    /// alias: @@
    fn all(&self) -> Rc<[String]> {
        let dirs = self.dirs();
        let entries = dirs
            .iter()
            .filter_map(|d| read_dir(d).ok()) // ignore missing repos lol
            .flatten();

        entries
            .filter_map(|e| {
                let entry = e.fail("Failed to read entry: Filesystem error?");
                if entry
                    .file_type()
                    .efail(|| format!("Failed to get filetype for '{entry:?}'"))
                    .is_dir()
                {
                    let repo = entry
                        .path()
                        .parent()
                        .efail(|| {
                            format!(
                                "[UNREACHABLE] Repo for entry '{}' does not exist?",
                                entry.path().display()
                            )
                        })
                        .file_name()
                        .efail(|| {
                            format!(
                                "[UNREACHABLE] Repo for entry '{}' does not have a filename?",
                                entry.path().display()
                            )
                        })
                        .to_str()
                        .fail("[UNREACHABLE] Invalid Unicode?")
                        .to_string();
                    let pkg = entry
                        .file_name()
                        .to_str()
                        .fail("[UNREACHABLE] Invalid Unicode?")
                        .to_string();

                    if pkg.starts_with('.') { None } else { Some(format!("{repo}/{pkg}")) }
                } else {
                    None
                }
            })
            .collect()
    }

    /// # Description
    /// Unravels the special set '@installed', which contains every installed package in a repo
    ///
    /// alias: @i
    fn installed(&self) -> Rc<[String]> {
        self.all()
            .iter()
            .filter(|p| Path::new(&format!("/var/ports/{p}/.data/INSTALLED")).exists())
            .cloned()
            .collect::<Vec<_>>()
            .into()
    }

    /// # Description
    /// Unravels the special set '@available', which contains every available package in a repo
    ///
    /// alias: @a
    fn available(&self) -> Rc<[String]> {
        self.all()
            .iter()
            .filter(|p| !Path::new(&format!("/var/ports/{p}/.data/INSTALLED")).exists())
            .cloned()
            .collect::<Vec<_>>()
            .into()
    }

    /// # Description
    /// Unravels the special set '@outdated', which contains every outdated package in a repo
    ///
    /// alias: @o
    fn outdated(&self) -> Rc<[String]> {
        self.all()
            .iter()
            .filter(|p| {
                let (repo, name) = p
                    .split_once('/')
                    .efail(|| format!("[UNREACHABLE] Misformatted package '{p}'"));
                super::Package::new(repo, name).is_outdated()
            })
            .cloned()
            .collect::<Vec<_>>()
            .into()
    }
}

/// # Description
/// Lists available sets for a repo
pub fn list(repo: &str) {
    let dir = format!("/var/ports/{repo}/.sets");
    let Ok(entries) = read_dir(dir) else {
        return erm!("No sets available for '{}/'", repo);
    };

    let available: Rc<[String]> = entries
        .map(|f| {
            f.fail("Failed to read dir entry: Filesystem error?")
                .file_name()
                .into_string()
                .fail("[UNREACHABLE] Invalid Unicode")
        })
        .collect();

    if available.is_empty() {
        return erm!("No sets available for '{}/'", repo);
    }

    available.iter().for_each(|s| pr!("{}", s));
}

#[cfg(test)]
mod tests {
    use super::Set;

    #[test]
    fn unravel_tox_all() {
        let set = Set::new("tox/@@");
        let members = set.unravel();
        dbg!(&set);
        dbg!(&members);
    }

    #[test]
    fn unravel_all_available() {
        let set = Set::new("//@a");
        let members = set.unravel();
        dbg!(&set);
        dbg!(&members);
    }

    #[test]
    fn unravel_main_outdated() {
        let set = Set::new("main/@outdated");
        let members = set.unravel();
        dbg!(&set);
        dbg!(&members);
    }

    #[test]
    fn unravel_xorg_installed() {
        let set = Set::new("xorg/@i");
        let members = set.unravel();
        dbg!(&set);
        dbg!(&members);
    }

    #[test]
    fn unravel_all_all() {
        let set = Set::new("//@all");
        let members = set.unravel();
        dbg!(&set);
        dbg!(&members);
    }

    #[test]
    fn unravel_main_lfs() {
        let set = Set::new("@lfs");
        let members = set.unravel();
        dbg!(&set);
        dbg!(&members);
    }
}
