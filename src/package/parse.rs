// src/package/parse.rs
//
// parses the raw package arguments

use super::Package;
use super::ambiguity::{resolve_ambiguity, resolve_set_ambiguity};
use super::sets::unravel;
use crate::utils::fail::{fail, Fail};

pub fn parse(packages: &[String]) -> Box<[Package]> {
    packages.iter().flat_map(|p| {
        if p.contains('=') {
            fail!("Version control is not supported");
        }

        let p = if p.ends_with('/') {
            format!("{p}@all")
        } else {
            p.to_string()
        };

        // TODO: test 'main/@lfs'
        if p.contains('@') {
            let set = expand_set(&p);
            return set.into_iter()
        }

        let p = if p.contains('/') {
            p
        } else {
            resolve_ambiguity(&p)
        };

        let (repo, name) = p.split_once('/').ufail("Package does not contain '/'");
        vec![Package::new(repo, name)].into_iter()
    }).collect::<Vec<_>>().into()
}

fn expand_set(set: &str) -> Box<[Package]> {
    let set = if set.contains("@all") { set.to_string() } else { resolve_set_ambiguity(set) };
    let packages = unravel(&set).fail("Failed to unravel set");

    packages.iter().map(|p| {
        if p.contains('=') { fail!("Version control is not supported") }

        let p = if p.contains('/') {
            p.to_string()
        } else {
            resolve_ambiguity(p)
        };

        let (repo, name) = p.split_once('/').ufail("p does not contain '/'");
        Package::new(repo, name)
    }).collect()
}
