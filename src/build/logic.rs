// src/build/logic.rs
//! Defines the logic for package builds

use super::script;
use crate::{
    comms::out::{
        msg,
        pr,
    },
    globals::{
        config::CONFIG,
        flags::Flags,
    },
    package::{
        Package,
        stats::{
            self,
            PackageStats,
        },
    },
    remove::logic::{
        clean,
        remove_dead_files_after_update,
    },
    shell::cmd::exec,
    utils::fail::Fail,
};

pub enum InstallStatus {
    Already,
    Dist,
    BuildFirst,
    UpdateInstead,
}

pub enum UpdateStatus {
    Latest,
    NotInstalled,
    Dist,
    BuildFirst,
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
    if !package.data.installed_version.is_empty() && !Flags::grab().force {
        if package.version == package.data.installed_version {
            InstallStatus::Already
        } else {
            InstallStatus::UpdateInstead
        }
    } else if package.dist_exists() {
        dist_install(package);
        InstallStatus::Dist
    } else {
        InstallStatus::BuildFirst
    }
}

/// # Description
/// Builds a package, calling functions in ``super::script``
///
/// Returns false if the package has already been built
pub fn build(package: &Package, r#override: bool) -> (BuildStatus, Option<PackageStats>) {
    let stats = stats::load(package).fail("Failed to load package stats");

    let built = package.dist_exists() && !Flags::grab().force && !r#override;

    if built {
        (BuildStatus::Already, None)
    } else {
        msg!("󱠇  Building '{}'...", package);
        script::prep(package);
        script::build(package);

        if CONFIG.general.clean_after_build {
            clean(package);
        }
        (BuildStatus::Source, Some(stats))
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

    tar xvf {:?} -C "$PREFIX"       \
        --strip-components=1        \
        --keep-directory-symlink    \
        --numeric-owner             \
        --no-overwrite-dir          \
        --exclude-from='/etc/2/exclusions.txt' |
    sed -e 's@/$@@' \
        -e 's@^D@@' \
        -e '/^$/d'  |
    tee {:?}/.data/MANIFEST={}
    echo "{}" > {:?}/.data/INSTALLED

    ldconfig

    "#,
        CONFIG.general.prefix,
        package.data.dist,
        package.data.port_dir,
        package.version,
        package.version,
        package.data.port_dir,
    );

    msg!("󰐗  Installing '{package}'...");
    exec(&command, None).fail("Failed to perform dist install");
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
    let flags = Flags::grab();
    let force = flags.force;
    let quiet = flags.quiet;
    if !package.data.is_installed && !force {
        return UpdateStatus::NotInstalled;
    }

    if !package.is_outdated() && !force {
        return UpdateStatus::Latest;
    }

    msg!(
        "󱍷  Updating '{}': '{}' -> '{}'",
        package.name,
        package.data.installed_version,
        package.version
    );

    if !package.dist_exists() {
        return UpdateStatus::BuildFirst;
    }

    dist_install(package);
    if package.version != package.data.installed_version {
        if !quiet {
            pr!(
                "Removing dead files for '{}={}'",
                package.name,
                package.data.installed_version
            );
        }
        remove_dead_files_after_update(package);
    }

    UpdateStatus::Dist
}
