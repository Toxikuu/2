// src/package/parse.rs
//
// parses the raw package arguments

use crate::die;

use super::Package;
use super::ambiguity::{resolve_ambiguity, resolve_set_ambiguity};
use super::sets::unravel;
use crate::utils::fail::Fail;

pub fn parse(packages: &[String]) -> Vec<Package> {
    let mut parsed_packages = Vec::new();

    for package in packages {
        if package.contains('=') { die!("Version control is not supported") }

        let package = 
        if package.ends_with('/') { &format!("{package}@all") }
        else { package };

        if package.contains('@') {
            append_set(package, &mut parsed_packages);
            continue
        }

        let package = if package.contains('/') {
            package.to_string()
        } else {
            resolve_ambiguity(package)
        };

        let (repo, name) = package.split_once('/').ufail("Package does not contain '/'");
        parsed_packages.push(Package::new(repo, name));
    }

    parsed_packages
}

fn append_set(set: &str, package_list: &mut Vec<Package>) {
    let set = if set.contains("@all") { set.to_string() } else { resolve_set_ambiguity(set) };
    let packages = unravel(&set).fail("Failed to unravel set");

    let mut unraveled_packages = Vec::new();
    for package in &packages {
        if package.contains('=') { die!("Version control is not supported") }

        let package = if package.contains('/') {
            package.to_string()
        } else {
            resolve_ambiguity(package)
        };

        let (repo, name) = package.split_once('/').ufail("Package does not contain '/'");
        unraveled_packages.push(Package::new(repo, name));
    }

    package_list.append(&mut unraveled_packages);
}
