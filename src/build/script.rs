// src/build/script.rs
//! Interfaces with $PORT/BUILD

use crate::shell::cmd::exec;
use crate::globals::config::CONFIG;
use crate::shell::cmd::pkgexec;
use crate::package::Package;
use std::fs::{create_dir, remove_dir_all};
use std::path::Path;
use crate::fetch::download::normalize_tarball;
use crate::utils::fail::{fail, Fail};
use anyhow::Result;

/// ### Description
/// Checks the hashes for package sources, dying if they don't match known ones
///
/// Known hashes are sourced from Package which is deserialized from info.lock
///
/// The m-gen script is responsible for initializing these hashes
fn check_hashes(package: &Package, no_source: bool, relpath: &str) {
    /// ### Description
    /// Helper subfunction for checking hashes
    // TODO: consider using a closure instead, capturing relpath
    fn core(filename: &str, knownhash: &str, relpath: &str) -> Result<()> {
        // pkgexec is explicitly not used here as it sets more variables than is necessary
        let command = format!(r#"

        SRC="/usr/ports/{relpath}/.sources"
        u-val "$SRC/{filename}" "{knownhash}"

        "#
        );

        exec(&command)
    }

    if !no_source {
        let tarball = package.data.source.url.split('/').next_back().fail("Invalid url");
        let filename = normalize_tarball(package, tarball);
        let knownhash = &package.data.source.hash;
        core(&filename, knownhash, relpath).fail("Hash checks failed");
    }

    for source in &package.data.extra {
        let filename = Path::new(&source.url).file_name().fail("Invalid file name").to_string_lossy();
        let knownhash = &source.hash;
        core(&filename, knownhash, relpath).fail("Hash checks failed");
    }
}

/// ### Description
/// Sets up for a build
///
/// The setup process involves checking hashes, cleaning, and extracting the sources to the build directory
fn setup(package: &Package) {
    let no_source = package.data.source.url.is_empty();
    if CONFIG.general.check_hashes { check_hashes(package, no_source, &package.relpath) }
    clean(package);

    let command = format!(
    r#"

    XTR="/tmp/2/extraction"
    rm -rf "$XTR"
    mkdir -pv "$XTR"

    if {no_source}; then
        echo "Package has no tarball; skipping extraction" >&2
        exit 0
    fi

    if [ -n "$EXTRACT" ]; then
        echo "Extraction explicitly disabled" >&2
        exit 0
    fi

    # example: /usr/ports/testing/tree/.sources/tree=2.2.1.tar.bz2
    tar xf "$SRC/{package}.tar."*z* -C $XTR
    shopt -s dotglob
    mv -f $XTR/*/* "$BLD"/

    "#
    );

    pkgexec!(&command, package).unwrap_or_else(|e| fail!("Build for '{}' died in setup: {}", package, e));
}

/// ### Description
/// Evaluates build instructions
///
/// Defined in BUILD under ``2b()``
///
/// Build instructions should DESTDIR install to "$BLD/D"
pub fn build(package: &Package) {
    setup(package);
    let command = format!(
    r#"
    
    source "$PORT/BUILD"
    cd "$BLD"

    2b

    cd "$BLD"
    ORIG=$(du -sh D | awk '{{print $1}}')
    TB="$PORT/.dist/{package}.tar.zst"

    tar cpf D.tar D
    zstd --rm -f -T0 -19 -o "$TB" D.tar >/dev/null 2>&1

    FINL=$(du -sh "$TB" | awk '{{print $1}}')
    echo -e "\x1b[0;37;1m[ $ORIG ↘ ↘  $FINL ]\x1b[0m" >&2
    "#
    );

    pkgexec!(&command, package).unwrap_or_else(|e| fail!("Build for '{}' died: {}", package, e));
}

/// ### Description
/// Evaluates pre-install instructions
///
/// TODO: Finish writing this
pub fn prep(package: &Package) {
    let command =
    r#"

    source "$PORT/BUILD"
    mkdir -pv "$PORT"/.{data,dist,build}

    type -t 2a > /dev/null 2>&1 || exit 0
    
    2a

    "#.to_string();

    pkgexec!(&command, package).fail("Build died while performing preparation steps!");
}

/// ### Description
/// Evaluates post-install instructions
///
/// These instructions are defined in BUILD under the function ``2z()``
///
/// The instructions should not interact with the build, but rather should perform any necessary
/// post-install actions
pub fn post(package: &Package) {
    let command =
    r#"

    source "$PORT/BUILD"

    type -t 2z > /dev/null 2>&1 || exit 0 # finish if post is undefined

    2z

    "#.to_string();

    pkgexec!(&command, package).unwrap_or_else(|e| fail!("Build for '{}' died in post-install: {}", package, e));
}

/// ### Description
/// Cleans a build
///
/// Deletes and recreates the .build directory
pub fn clean(package: &Package) {
    let dir = format!("/usr/ports/{}/.build", package.relpath);
    remove_dir_all(&dir).unwrap_or_else(|e| fail!("Failed to clean '{}': {}", package, e));
    create_dir(&dir).ufail("Failed to recreate .build");
}
