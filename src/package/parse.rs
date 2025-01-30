// src/package/parse.rs
//! Functions for parsing positional arguments

use crate::{
    comms::log::erm,
    utils::fail::Fail,
};
use super::{
    ambiguity::{resolve_ambiguity, resolve_set_ambiguity},
    Package,
    sets::unravel,
};

// TODO: I'd love to have sets of repos but im too dumb to figure out how to do it rn
//
/// # Description
/// Parses the raw package positional arguments into packages
///
/// Expands sets and resolves ambiguity
///
/// If a version is passed, warns the user
pub fn parse(packages: &[String]) -> Vec<Package> {
    let mut parsed = Vec::new();

    for p in packages {
        let mut p = p.as_str();

        if let Some(i) = p.find('=') {
            let msg = format!("Version control is not supported; stripping version for '{p}'");
            erm!("{}", msg); log::warn!("{}", msg);
            p = &p[..i];
        }

        let mut p = p.to_string();
        if p.ends_with('/') {
            p.push_str("@all");
        }

        if p.contains('@') {
            let mut set = expand_set(&p).to_vec();
            parsed.append(&mut set);
            continue
        }

        if ! p.contains('/') {
            p = resolve_ambiguity(&p);
        }

        let (repo, name) = p.split_once('/').ufail("Package does not contain '/'");
        parsed.push(Package::new(repo, name));
    }
    parsed
}

/// # Description
/// Expands a set into its constituent packages
pub fn expand_set(set: &str) -> Box<[Package]> {
    let set = if set.contains("@every") || set.contains("@!") || set.contains("/@all") || set.contains("/@@") {
        set.to_string()
    } else {
        resolve_set_ambiguity(set)
    };

    let packages = unravel(&set).fail("Failed to unravel set");

    packages.iter().map(|p| {
        let mut p = p.as_str();

        if let Some(i) = p.find('=') {
            let msg = format!("Version control is not supported; stripping version for set '{set}' member '{p}'");
            erm!("{}", msg); log::warn!("{}", msg);
            p = &p[..i];
        }

        let mut p = p.to_string();
        if ! p.contains('/') {
            p = resolve_ambiguity(&p);
        }

        let (repo, name) = p.split_once('/').ufail("p does not contain '/'");
        Package::new(repo, name)
    }).collect()
}
