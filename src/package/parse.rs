// src/package/parse.rs
//! Functions for parsing positional arguments

use crate::{
    comms::out::erm,
    utils::fail::Fail,
};
use super::{
    ambiguity::resolve_ambiguity,
    Package,
    sets::Set,
};

// TODO: Rather than sets of repos, I should make it so the user can limit the sets to specific
// repos (ex. main/@outdated)
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
            erm!("{msg}"); log::warn!("{msg}");
            p = &p[..i];
        }

        let mut p = p.to_string();
        if p.ends_with('/') {
            p.push_str("@@");
        }

        if p.contains('@') {
            let mut set = expand_set(&p).to_vec();
            parsed.append(&mut set);
            continue
        }

        if ! p.contains('/') {
            p = resolve_ambiguity(&p);
        }

        let (repo, name) = p.split_once('/').fail("Invalid syntax");
        parsed.push(Package::new(repo, name));
    }
    parsed
}

/// # Description
/// Expands a set into its constituent packages
pub fn expand_set(set: &str) -> Box<[Package]> {
    let packages = Set::new(set).unravel().fail(&format!("Failed to unravel '{set}'"));

    packages.iter().map(|p| {
        let mut p = p.as_str();

        if let Some(i) = p.find('=') {
            let msg = format!("Version control is not supported; stripping version for '{set}' member '{p}'");
            erm!("{}", msg); log::warn!("{}", msg);
            p = &p[..i];
        }

        let mut p = p.to_string();
        if ! p.contains('/') {
            p = resolve_ambiguity(&p);
        }

        let (repo, name) = p.split_once('/').fail("p does not contain '/'");
        Package::new(repo, name)
    }).collect()
}
