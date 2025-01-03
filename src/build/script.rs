// src/build/script.rs
//! Interfaces with $PORT/BUILD

use crate::shell::cmd::exec;
use crate::{pkgexec, die};
use crate::package::Package;
use std::fs::{create_dir, remove_dir_all};
use std::path::Path;
use crate::fetch::download::normalize_tarball;
use crate::utils::fail::Fail;

/// ### Description
/// Checks the hashes for package sources, dying if they don't match known ones
///
/// Known hashes are sourced from Package which is deserialized from info.toml
///
/// The m-gen script is responsible for initializing these hashes
fn check_hashes(package: &Package, no_source: bool, relpath: &str) {
    /// ### Description
    /// Helper subfunction for checking hashes
    // TODO: consider using a closure instead, capturing relpath
    fn core(filename: &str, knownhash: &str, relpath: &str) -> std::io::Result<()> {
        // pkgexec is explicitly not used here as it sets more variables than is necessary
        let command = format!(r#"

        SRC="/usr/ports/{}/.sources"
        u-val "$SRC/{}" "{}"

        "#,
        relpath,
        filename, knownhash
        );

        exec(&command)
    }

    if !no_source {
        let tarball = package.data.source.url.split('/').last().fail("Invalid url");
        let filename = normalize_tarball(package, tarball);
        let knownhash = &package.data.source.hash;
        core(&filename, knownhash, relpath).fail("Hash checks failed")
    }

    for source in &package.data.extra {
        let filename = Path::new(&source.url).file_name().fail("Invalid file name").to_string_lossy();
        let knownhash = &source.hash;
        core(&filename, knownhash, relpath).fail("Hash checks failed")
    }
}

fn setup(package: &Package) {
    let no_source = package.data.source.url.is_empty();
    // TODO: make hash checks configurable
    check_hashes(package, no_source, &package.relpath);
    clean(package);

    let command = format!(
    r#"

    XTR="/tmp/2/extraction"
    rm -rf "$XTR"
    mkdir -pv "$XTR"

    if {}; then
        echo "Package has no tarball; skipping extraction" >&2
        exit 0
    fi

    # example: /usr/ports/testing/tree/.sources/tree=2.2.1.tar.bz2
    tar xf "$SRC/{}.tar."*z* -C $XTR
    shopt -s dotglob
    mv -f $XTR/*/* "$BLD"/

    "#,
    no_source,
    package,
    );

    pkgexec!(&command, package).unwrap_or_else(|e| die!("Build for '{}' died in setup: {}", package, e))
}

pub fn build(package: &Package) {
    setup(package);
    let command = format!(
    r#"
    
    source "$PORT/BUILD"
    cd "$BLD"

    2b

    tar cpf D.tar D
    zstd --rm -f -T0 -19 -o "$PORT/.dist/{}.tar.zst" D.tar # TODO: Add a dictionary
    "#,
    package,
    );

    pkgexec!(&command, package).unwrap_or_else(|e| die!("Build for '{}' died: {}", package, e))
}

pub fn prep(package: &Package) {
    let command =
    r#"

    source "$PORT/BUILD"

    type -t 2a > /dev/null 2>&1 || exit 0
    
    2a

    "#.to_string();

    pkgexec!(&command, package).fail("Build died while performing preparation steps!")
}

pub fn post(package: &Package) {
    let command =
    r#"

    source "$PORT/BUILD"

    type -t 2z > /dev/null 2>&1 || exit 0 # finish if post is undefined

    2z

    "#.to_string();

    pkgexec!(&command, package).unwrap_or_else(|e| die!("Build for '{}' died in post-install: {}", package, e))
}

pub fn clean(package: &Package) {
    let dir = format!("/usr/ports/{}/.build", package.relpath);
    remove_dir_all(&dir).unwrap_or_else(|e| die!("Failed to clean '{}': {}", package, e));
    create_dir(&dir).ufail("Failed to recreate .build");
}
