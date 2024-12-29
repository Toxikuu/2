// src/build/logic.rs
//
// defines the logic for package builds

use crate::globals::config::CONFIG;
use crate::{erm, die, msg, cpr};
use crate::globals::flags::FLAGS;
use crate::package::Package;
use crate::shell::cmd::exec;
use crate::remove::logic::remove_dead_files_after_update;
use super::script;
use std::path::Path;

pub fn install(package: &Package) -> bool {
    if package.data.installed_version == package.version && !FLAGS.lock().unwrap().force {
        erm!("Already installed '{}'", package);
        false
    } else if Path::new(&package.data.dist).exists() {
        dist_install(package);
        true
    } else {
        build(package);
        dist_install(package);
        true
    }
}

pub fn build(package: &Package) -> bool {
    if Path::new(&package.data.dist).exists() && !FLAGS.lock().unwrap().force {
        erm!("Built '{}'", package);
        false
    } else {
        msg!("Building '{}'", package);
        script::prep(package);
        script::build(package);
        true
    }
}

fn dist_install(package: &Package) {
    msg!("Installing '{}'", package);
    let relpath = format!("{}/{}", package.repo, package.name);
    let command = format!(
    r#"

    PREFIX={}

    mkdir -pv $PREFIX
    tar xvf /usr/ports/{}/.dist/{}.tar.zst -C $PREFIX --strip-components=1
    echo "{}" > /usr/ports/{}/.data/INSTALLED

    "#,
    CONFIG.general.prefix,
    relpath, package,
    package.version, relpath,
    );

    exec(&command).unwrap_or_else(|e| die!("Failed to dist-install '{}': {}", package, e));
    script::post(package); // TODO: confirm this works
}

pub fn update(package: &Package) -> bool {
    if !package.data.is_installed && !FLAGS.lock().unwrap().force {
        erm!("Missing: '{}'", package);
        return false
    }

    if package.version == package.data.installed_version && !FLAGS.lock().unwrap().force {
        erm!("Current: '{}'", package);
        return false
    }

    msg!("Updating '{}': '{}' -> '{}'", package.name, package.data.installed_version, package.version);

    if !Path::new(&package.data.dist).exists() {
        build(package);
    }

    dist_install(package);
    if package.version != package.data.installed_version {
        cpr!("Removing dead files for '{}={}'", package.name, package.data.installed_version);
        remove_dead_files_after_update(package);
    }
    true
}
