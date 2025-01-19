// src/build/logic.rs
//! Defines the logic for package builds

use crate::globals::config::CONFIG;
use crate::comms::log::{erm, msg, cpr};
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
    if package.data.installed_version == package.version && !FLAGS.lock().ufail("Failed to lock flags").force {
        erm!("Already installed '{}'", package);
        false
    } else if Path::new(package.data.dist.as_str()).exists() {
        dist_install(package);
        true
    } else {
        build(package);
        dist_install(package);
        true
    }
}

pub fn build(package: &Package) -> bool {
    if Path::new(package.data.dist.as_str()).exists() && !FLAGS.lock().ufail("Failed to lock flags").force {
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
    mkdir -pv "$PREFIX"

    tar xvf {} -C "$PREFIX"         \
        --strip-components=1        \
        --keep-directory-symlink    \
        --exclude-from='/etc/2/exclusions.txt' |
    sed -e 's@/$@@' \
        -e 's@^D@@' \
        -e '/^$/d'  |
    tee /usr/ports/{}/.data/MANIFEST={}
    echo "{}" > /usr/ports/{}/.data/INSTALLED

    # TODO: consider only using ldconfig once after a chain of dist installs, and after building and installing
    ldconfig

    "#,
    CONFIG.general.prefix,
    package.data.dist,
    package.relpath, package.version,
    package.version, package.relpath,
    );

    exec(&command).fail("Failed to perform dist install");
    script::post(package);
}

pub fn update(package: &Package) -> bool {
    let force = FLAGS.lock().ufail("Failed to lock flags").force;
    if !package.data.is_installed && !force {
        erm!("Missing: '{}'", package);
        return false
    }

    if package.version == package.data.installed_version && !force {
        erm!("Current: '{}'", package);
        return false
    }

    msg!("Updating '{}': '{}' -> '{}'", package.name, package.data.installed_version, package.version);

    if !Path::new(package.data.dist.as_str()).exists() {
        build(package);
    }

    dist_install(package);
    if package.version != package.data.installed_version {
        cpr!("Removing dead files for '{}={}'", package.name, package.data.installed_version);
        remove_dead_files_after_update(package);
    }
    true
}
