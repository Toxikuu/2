// src/build/logic.rs
//! Defines the logic for package builds

use std::path::Path;

use crate::{
    comms::log::{msg, cpr},
    globals::{config::CONFIG, flags::FLAGS},
    package::Package,
    remove::logic::remove_dead_files_after_update,
    shell::cmd::exec,
    utils::fail::Fail,
};
use super::script;

pub enum InstallStatus {
    Already,
    Dist,
    Source,
    UpdateInstead,
}

pub enum UpdateStatus {
    Latest,
    NotInstalled,
    Dist,
    Source,
}

pub enum BuildStatus {
    Already,
    Source,
}

/// # Description
/// Installs a package by performing a dist install. If the package isn't built, builds it and then
/// dist installs.
///
/// Returns false if the package has already been installed
pub fn install(package: &Package) -> InstallStatus {
    if !package.data.installed_version.is_empty() && !FLAGS.get().ufail("Cell issue").force {
        if package.version == package.data.installed_version {
            InstallStatus::Already
        } else {
            InstallStatus::UpdateInstead
        }
    } else if package.dist_exists() {
        dist_install(package);
        InstallStatus::Dist
    } else {
        build(package);
        dist_install(package);
        InstallStatus::Source
    }
}

/// # Description
/// Builds a package, calling functions in ``super::script``
///
/// Returns false if the package has already been built
pub fn build(package: &Package) -> BuildStatus {
    if package.dist_exists() && !FLAGS.get().ufail("Cell issue").force {
        BuildStatus::Already
    } else {
        msg!("󱠇  Building '{}'...", package);
        script::prep(package);
        script::build(package);

        if CONFIG.general.clean_after_build {
            script::clean(package);
        }
        BuildStatus::Source
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

    msg!("󰐗  Installing '{package}'...");
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
pub fn update(package: &Package) -> UpdateStatus {
    let force = FLAGS.get().ufail("Cell issue").force;
    if !package.data.is_installed && !force {
        return UpdateStatus::NotInstalled
    }

    if package.version == package.data.installed_version && !force {
        return UpdateStatus::Latest
    }

    msg!("󱍷  Updating '{}': '{}' -> '{}'", package.name, package.data.installed_version, package.version);

    let dist_exists = Path::new(
        &format!("/usr/ports/{}/.dist/{}={}", package.relpath, package.name, package.version)
    ).exists();

    if !dist_exists {
        build(package);
    }

    dist_install(package);
    if package.version != package.data.installed_version {
        cpr!("Removing dead files for '{}={}'", package.name, package.data.installed_version);
        remove_dead_files_after_update(package);
    }

    if dist_exists { UpdateStatus::Dist } else { UpdateStatus::Source }
}
