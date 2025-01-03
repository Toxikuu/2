// src/build/logic.rs
//
// defines the logic for package builds

use crate::globals::config::CONFIG;
use crate::{erm, msg, cpr};
use crate::globals::flags::FLAGS;
use crate::package::Package;
use crate::shell::cmd::exec;
use crate::remove::logic::remove_dead_files_after_update;
use super::script;
use std::path::Path;
use crate::utils::fail::Fail;

// TODO: add similar return enums in the future for all pm logic operations
// pub enum InstallTristate {
//     AlreadyInstalled,
//     DistInstalled,
//     BuiltAndInstalled,
// }

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
        erm!("Already built '{}'", package);
        false
    } else {
        msg!("Building '{}'", package);
        script::prep(package);
        script::build(package);

        if CONFIG.general.clean_after_build {
            script::clean(package);
        }
        true
    }
}

fn dist_install(package: &Package) {
    let command = format!(
    r#"

    PREFIX={}

    mkdir -pv $PREFIX
    tar xvf {} -C $PREFIX --strip-components=1 --exclude-from='{}'
    echo "{}" > /usr/ports/{}/.data/INSTALLED

    "#,
    CONFIG.general.prefix,
    package.data.dist, CONFIG.general.exclusions,
    package.version, package.relpath,
    );

    exec(&command).fail("Failed to perform dist install");
    script::post(package);
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
