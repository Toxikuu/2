// src/build/script.rs

use crate::shell::cmd::exec;
use crate::die;
use crate::package::Package;
use std::path::Path;
use crate::fetch::download::normalize_tarball;

// TODO: Add relpath to the package struct
fn check_hashes(package: &Package, no_source: bool, relpath: &str) {
    fn core(filename: &str, knownhash: &str, relpath: &str) -> std::io::Result<()> {
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
        let tarball = package.data.source.url.split('/').last().unwrap();
        let filename = normalize_tarball(package, tarball);
        let knownhash = &package.data.source.hash;
        core(&filename, knownhash, relpath).unwrap_or_else(|_| die!("Extra hash didn't match!"));
    }

    for source in &package.data.extra {
        let filename = Path::new(&source.url).file_name().unwrap().to_string_lossy();
        let knownhash = &source.hash;
        core(&filename, knownhash, relpath).unwrap_or_else(|_| die!("Extra hash didn't match!"));
    }
}

fn setup(package: &Package) {
    let no_source = package.data.source.url.is_empty();
    let relpath = format!("{}/{}", package.repo, package.name);
    // TODO: make hash checks configurable
    check_hashes(package, no_source, &relpath);
    clean(package);

    let command = format!(
    r#"

    PORT="/usr/ports/{}"
    BLD="$PORT/.build"
    EXTRACTION_DIR="/tmp/2/extraction"
    SRC="$PORT/.sources"
    rm -rf "$EXTRACTION_DIR"
    mkdir -pv "$EXTRACTION_DIR"

    if {}; then
        echo "Package has no tarball; skipping extraction" >&2
        exit 0
    fi

    # example: /usr/ports/testing/tree/.sources/tree=2.2.1.tar.bz2
    tar xf "$SRC/{}.tar."*z* -C $EXTRACTION_DIR
    mv -f $EXTRACTION_DIR/*/* "$BLD"/

    "#,
    relpath,
    no_source,
    package,
    );

    exec(&command).unwrap_or_else(|e| die!("Failed to setup for '{}': {}", package, e))
}

pub fn build(package: &Package) {
    setup(package);
    let relpath = format!("{}/{}", package.repo, package.name);

    let command = format!(
    r#"
    
    # TODO: Consider defining these variables within exec instead
    PORT="/usr/ports/{}"
    SRC="$PORT/.sources"
    BLD="$PORT/.build"
    source "$PORT/BUILD"

    cd "$BLD"

    2b

    # TODO: Ensure update logic removes dead files from the previous manifest, and then the manifest
    find D | cut -dD -f2- | sed '/^$/d' > "$PORT/.data/MANIFEST={}"

    tar cpf D.tar D
    zstd --rm -f -T0 -19 -o "$PORT/.dist/{}.tar.zst" D.tar # TODO: Add a dictionary
    "#,
    relpath,
    package.version,
    package,
    );

    exec(&command).unwrap_or_else(|e| die!("Failed to build '{}': {}", package, e));
}

pub fn prep(package: &Package) {
    let relpath = format!("{}/{}", package.repo, package.name);
    let command = format!(
    r#"

    PORT="/usr/ports/{}"
    SRC="$PORT/.sources"
    source "$PORT/BUILD"

    type -t 2a > /dev/null 2>&1 || exit 0
    
    2a

    "#,
    relpath,
    );

    exec(&command).unwrap_or_else(|e| die!("Failed to perform preparation steps for '{}': {}", package, e));
}

pub fn post(package: &Package) {
    let relpath = format!("{}/{}", package.repo, package.name);
    let command = format!(
    r#"

    PORT="/usr/ports/{}"
    SRC="$PORT/.sources"
    source "$PORT/BUILD"

    type -t daj_post > /dev/null 2>&1 || exit 0 # finish if post is undefined

    daj_post

    "#,
    relpath,
    );

    exec(&command).unwrap_or_else(|e| die!("Failed to preform post-install steps for '{}': {}", package, e));
}

pub fn clean(package: &Package) {
    let relpath = format!("{}/{}", package.repo, package.name);
    let command = format!(
    r#"

    # TODO: Make cleanup toggleable in the config
    rm -rf /usr/ports/{}/.build/{{*,.*}}
    "#,
    relpath,
    );

    exec(&command).unwrap_or_else(|e| die!("Failed cleanup for '{}': {}", package, e))
}
