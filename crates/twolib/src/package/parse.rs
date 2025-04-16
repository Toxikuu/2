// twolib/src/package/parse.rs
//! Functions for parsing raw positional arguments into Packages

use tracing::{
    instrument,
    warn,
};
use twodebug::{
    a,
    d,
};

use super::Package;
use crate::{
    ambiguity::resolve_ambiguity,
    set::Set,
};

/// # Description
/// Parses the raw package positional arguments into packages
///
/// Expands sets and resolves ambiguity
///
/// If a version is passed, warns the user
#[instrument]
pub fn parse(packages: &[String]) -> Vec<Package> {
    let mut parsed = Vec::new();

    for p in packages {
        let mut p = p.as_str();

        if let Some(i) = p.find('=') {
            warn!("Version control is not supported; stripping version for '{p}'");
            p = &p[..i];
        }

        let mut p = p.to_string();
        if p.ends_with('/') {
            p.push_str("@@");
        }

        if p.contains('@') {
            let mut set = expand_set(&p).to_vec();
            parsed.append(&mut set);
            continue;
        }

        if !p.contains('/') {
            p = resolve_ambiguity(&p);
        }

        a!(p.contains('/'));
        parsed.push(Package::from_relpath(&p))
    }
    d!(parsed);
    parsed
}

/// # Description
/// Expands a set into its constituent packages
#[instrument]
pub fn expand_set(set: &str) -> Box<[Package]> {
    let packages = Set::new(set).unravel();
    d!("Unraveled set:", packages);

    let pkgs = packages
        .iter()
        .map(|p| {
            let mut p = p.as_str();

            if let Some(i) = p.find('=') {
                warn!(
                    "Version control is not supported; stripping version for '{set}' member '{p}'"
                );
                p = &p[..i];
            }

            let mut p = p.to_string();
            if !p.contains('/') {
                p = resolve_ambiguity(&p);
            }

            a!(p.contains('/'));
            Package::from_relpath(&p)
        })
        .collect();

    d!("Fully expanded set:", pkgs);
    pkgs
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_efibootmgr() {
        let package = parse(&["testing/efibootmgr".to_string()]);
        d!(format!("{package:#?}"));
        panic!()
    }
}
