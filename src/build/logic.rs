// src/build/logic.rs
//! Defines the logic for package builds

use crate::{
    comms::log::{erm, msg, cpr},
    globals::{config::CONFIG, flags::FLAGS},
    package::Package,
    remove::logic::remove_dead_files_after_update,
    shell::cmd::exec,
    utils::fail::Fail,
};
use std::path::Path;
use super::script;

// TODO: add similar return enums in the future for all pm logic operations
// pub enum InstallTristate {
//     AlreadyInstalled,
//     DistInstalled,
//     BuiltAndInstalled,
// }

/// # Description
/// Installs a package by performing a dist install. If the package isn't built, builds it and then
/// dist installs.
///
/// Returns false if the package has already been installed
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

/// # Description
/// Builds a package, calling functions in ``super::script``
///
/// Returns false if the package has already been built
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

/// # Description
/// Installs a package from its dist tarball. Also evaluates the post-install instructions.
///
/// Uses tar under the hood. Reads /etc/2/exclusions.txt. Logs the installed files to a manifest.
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

/// # Description
/// Updates a package.
///
/// Returns false if
/// - the package isn't installed and force isn't passed; otherwise continues
/// - the package is at its newest version and force isn't passed; otherwise continues
///
/// If the dist tarball for the new version exists, uses that. Otherwise, builds the package and
/// then dist installs it.
///
/// After dist installing, if the new version isn't the old version, removes any dead files by
/// calling ``remove_dead_files_after_update()``. Finally returns true.
///
/// Uses tar under the hood. Reads /etc/2/exclusions.txt. Logs the installed files to a manifest.
pub fn update(package: &Package) -> bool {
    let force = FLAGS.lock().ufail("Failed to lock flags").force;
    if !package.data.is_installed && !force {
        log::warn!("Not updating to '{}' as an older version is not installed and force wasn't passed", package);
        erm!("Missing: '{}'", package);
        return false
    }

    if package.version == package.data.installed_version && !force {
        log::warn!("Not updating to '{}' as it's already at its newest version", package);
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
