// src/package/parse.rs
//
// parses the raw package arguments

use crate::die;

use super::Package;
use super::ambiguity::resolve_ambiguity;
use super::sets::unravel;

pub fn parse(packages: &[String]) -> Vec<Package> {
    let mut _packages = Vec::new();
    for package in packages {
        if package.contains('=') { die!("Version control is not supported") }

        let package = 
        if package.ends_with('/') { &format!("{}@all", package) }
        else { package };

        if package.contains('@') {
            append_set(package, &mut _packages);
            continue
        }

        let package = if package.contains('/') {
            package.to_string()
        } else {
            resolve_ambiguity(package)
        };

        let (repo, name) = package.split_once('/').unwrap();
        _packages.push(Package::new(repo, name));
    }

    _packages
}

fn append_set(set: &str, package_list: &mut Vec<Package>) {
    let packages = unravel(set).unwrap_or_else(|e| die!("Failed to unravel set '{}': {}", set, e));

    let mut _packages = Vec::new();
    for package in packages.iter() {
        if package.contains('=') { die!("Version control is not supported") }

        let package = if package.contains('/') {
            package.to_string()
        } else {
            resolve_ambiguity(package)
        };

        let (repo, name) = package.split_once('/').unwrap();
        _packages.push(Package::new(repo, name));
    }

    package_list.append(&mut _packages);
}
