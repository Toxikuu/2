// src/package/parse.rs
//
// parses the raw package arguments

use crate::comms::log::erm;
use crate::utils::fail::Fail;
use super::Package;
use super::ambiguity::{resolve_ambiguity, resolve_set_ambiguity};
use super::sets::unravel;

// TODO: I'd love to have sets of repos but im too dumb to figure out how to do it rn
pub fn parse(packages: &[String]) -> Vec<Package> {
    let mut parsed = Vec::new();

    for p in packages {
        let mut p = p.as_str();

        if let Some(i) = p.find('=') {
            erm!("Version control is not supported; stripping version");
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
            erm!("Version control is not supported; stripping version");
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
