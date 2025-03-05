// src/build/script.rs
//! Interfaces with $PORT/BUILD

use crate::{
    fetch::download::normalize_tarball,
    globals::config::CONFIG,
    package::Package,
    remove::logic::clean,
    shell::cmd::pkgexec,
    utils::{
        fail::{BoolFail, Fail},
        hash::twohash,
    },
};
use std::path::Path;
use super::qa;

/// ### Description
/// Checks the hashes for package sources, failing if they don't match known ones
///
/// Known hashes are sourced from Package which is deserialized from LOCK
/// ``2lkit -g <package>`` is responsible for generating the LOCK
fn check_hashes(package: &Package, no_source: bool) {
    // helper closure for checking hashes
    let passes = |filename: &str, knownhash: &str| -> bool {
        let file_path = package.data.port_dir.join(".sources").join(filename);
        twohash(&file_path) == knownhash
    };

    if !no_source {
        let tarball = package.source.url.split('/').next_back().fail("Invalid url");
        let filename = &normalize_tarball(package, tarball);
        let knownhash = &package.source.hash;
        passes(filename, knownhash).or_fail(&format!("Hash checks failed for '{filename}' for '{package}'"));
    }

    package.extra.iter().for_each(|source| {
        let filename = &Path::new(source.url.as_str()).file_name().fail("Invalid file name").to_string_lossy();
        let knownhash = &source.hash;
        passes(filename, knownhash).or_fail(&format!("Hash checks failed for '{filename}' for '{package}'"));
    });
}

/// ### Description
/// Sets up for a build
///
/// The setup process involves checking hashes, cleaning, and extracting the sources to the build directory
fn setup(package: &Package) {
    let no_source = package.source.url.is_empty();
    if CONFIG.general.check_hashes { check_hashes(package, no_source) }
    clean(package);

    let command = format!(
    r#"

    XTR="/tmp/2/extraction"
    rm -rf "$XTR"

    if {no_source}; then
        echo "Package has no tarball; skipping extraction" >&2
        exit 0
    fi

    if [ -n "$EXTRACT" ]; then
        echo "Extraction explicitly disabled" >&2
        exit 0
    fi

    mkdir -pv "$XTR"

    # example: /usr/ports/testing/tree/.sources/tree=2.2.1.tar.bz2
    tar xf "$SRC/{package}.tar."*z* -C $XTR
    shopt -s dotglob
    mv -f $XTR/*/* "$BLD"/

    "#
    );

    pkgexec!(&command, package).fail(&format!("Build for '{package}' died in setup"));
}

/// ### Description
/// Evaluates build instructions
///
/// Defined in BUILD under ``2b()``
///
/// Build instructions should DESTDIR install to "$BLD/D"
pub fn build(package: &Package) {
    setup(package);

    let command = r#"cd "$BLD"; 2b"#;
    pkgexec!(command, package).fail(&format!("Build for '{package}' died"));

    qa::destdir_has_stuff(package).or_fail(&format!("QA: Detected empty destdir for '{package}'"));
    qa::libs_ok(package).or_fail(&format!("QA: Detected wrong-architecture libraries for '{package}'"));

    let command = format!(r#"
    cd "$BLD"
    ORIG=$(du -sh D | awk '{{print $1}}')
    TB="$PORT/.dist/{package}.tar.zst"

    echo -e "Packaging..."
    tar cf - D | zstd --rm -f -T0 -19 -o "$TB" >/dev/null 2>&1

    FINL=$(du -sh "$TB" | awk '{{print $1}}')
    echo -e "\x1b[0;37;1m[ $ORIG ↘ ↘  $FINL ]\x1b[0m" >&2
    "#
    );

    pkgexec!(&command, package).fail(&format!("Packaging for '{package}' died"));
}

/// ### Description
/// Evaluates pre-install instructions
///
/// These instructions are defined in BUILD under the function ``2a()``.
pub fn prep(package: &Package) {
    let command =
    r#"

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
    r"

    type -t 2z > /dev/null 2>&1 || exit 0 # finish if post is undefined
    2z

    ".to_string();

    pkgexec!(&command, package).fail(&format!("Build for '{package}' died in post-install"));
}
