// src/package/sets.rs
//! Adds support for sets

use anyhow::Result;
use crate::{
    comms::out::{erm, pr, vpr},
    utils::fail::{Fail, fail, ufail},
};
use std::{
    fmt::{self, Display, Formatter},
    fs::{read_dir, File},
    io::{BufRead, BufReader},
    path::Path,
    rc::Rc
};
use super::{
    ambiguity::resolve_set_ambiguity,
    repos
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
    set: String,
}

impl Display for Set {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.repo, self.set)
    }
}

impl Set {
    pub fn new(str: &str) -> Self {
        if !Self::is(str) {
            fail!("Not a set: '{str}'");
        }

        // handle all repos
        if let Some(set) = str.strip_prefix("//") {
            return Self {
                repo: "//".to_string(),
                set: set.to_string(),
            }
        }

        if let Some((repo, set)) = str.split_once('/') {
            Self {
                repo: repo.to_string(),
                set: set.to_string(),
            }
        } else {
            let stupid_intermediate = resolve_set_ambiguity(str);
            let (repo, _) = stupid_intermediate.split_once('/').ufail("Somehow resolved ambiguity without returning repo/set");
            Self {
                repo: repo.to_string(),
                set: str.to_string(),
            }
        }
    }

    pub fn dirs(&self) -> Rc<[String]> {
        let repo = &self.repo;
        if repo == "//" {
            repos::find_all().iter().map(|r| format!("/usr/ports/{r}")).collect()
        } else {
            [format!("/usr/ports/{repo}")].into()
        }
    }

    /// # Description
    /// Returns true if a given string is a set
    pub fn is(str: &str) -> bool {
        str.contains('@')
    }

    /// # Description
    /// Returns true if a given string is a special set
    fn is_special(&self) -> bool {
        matches!(self.set.as_str(), "@a" | "@all" | "@o" | "@outdated" | "@i" | "@installed")
    }

    pub fn is_special_set(set: &str) -> bool {
        matches!(set, "@a" | "@all" | "@o" | "@outdated" | "@i" | "@installed")
    }

    fn unravel_special(&self) -> Rc<[String]> {
        let set = self.set.as_str();
        if matches!(set, "@o" | "@outdated") {
            self.outdated()
        } else if matches!(set, "@i" | "@installed") {
            self.installed()
        } else if matches!(set, "@a" | "@all") {
            self.all()
        } else {
            ufail!("I forgot to add a special set")
        }
    }

    /// # Description
    /// Given a set, returns all member packages
    /// Sets are defined in ``/usr/ports/<repo>/.sets/<@set>``
    pub fn unravel(&self) -> Result<Rc<[String]>> {
        let set = &self.set;
        let repo = &self.repo;
        vpr!("Unraveling set:\n{self:#?}");
        log::debug!("Unraveling '{self}'");

        if self.is_special() {
            return Ok(self.unravel_special())
        }

        let file_path = format!("/usr/ports/{repo}/.sets/{set}");
        let file = File::open(file_path).fail("Nonexistent set");
        let buf = BufReader::new(file);

        let lines = buf.lines().collect::<Result<Vec<String>, _>>()?;
        Ok(
            lines.into_iter()
                .map(|l| {
                    if l.contains('/') { l }
                    else { format!("{repo}/{l}") }
                })
                .collect()
        )
    }

    /// # Description
    /// Unravels the special set '@all', which contains every package in a given repo
    /// Output is in the form 'repo/package'
    ///
    /// alias: @a
    fn all(&self) -> Rc<[String]> {
        let dirs = self.dirs();
        let entries = dirs.iter()
            .filter_map(|d| read_dir(d).ok()) // ignore missing repos lol
            .flatten();

        entries
            .filter_map(|e| {
                let entry = e.fail("Failed to read entry");
                if entry.file_type().fail("Failed to get entry filetype").is_dir() {
                    let repo = entry.path()
                        .parent()
                        .ufail("Very strange repo layout?")
                        .file_name()
                        .ufail("Missing filename?")
                        .to_str()
                        .ufail("Unicode")
                        .to_string();
                    let pkg = entry.file_name()
                        .to_str()
                        .ufail("Unicode")
                        .to_string();

                    if pkg.starts_with('.') {
                        None
                    } else {
                        Some(format!("{repo}/{pkg}"))
                    }
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
            .filter(|p| Path::new(&format!("/usr/ports/{p}/.data/INSTALLED")).exists())
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
                let (repo, name) = p.split_once('/').ufail(&format!("Misformatted package {p}"));
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

#[cfg(test)]
mod tests {
    use super::Set;

    #[test]
    fn unravel_tox_all() {
        let set = Set::new("tox/@a");
        let members = set.unravel();
        dbg!(&set);
        dbg!(&members);
        assert!(members.is_ok());
    }

    #[test]
    fn unravel_main_outdated() {
        let set = Set::new("main/@outdated");
        let members = set.unravel();
        dbg!(&set);
        dbg!(&members);
        assert!(members.is_ok());
    }

    #[test]
    fn unravel_xorg_installed() {
        let set = Set::new("xorg/@i");
        let members = set.unravel();
        dbg!(&set);
        dbg!(&members);
        assert!(members.is_ok());
    }

    #[test]
    fn unravel_all_all() {
        let set = Set::new("//@all");
        let members = set.unravel();
        dbg!(&set);
        dbg!(&members);
        assert!(members.is_ok());
    }

    #[test]
    fn unravel_main_lfs() {
        let set = Set::new("@lfs");
        let members = set.unravel();
        dbg!(&set);
        dbg!(&members);
        assert!(members.is_ok());
    }
}
