// src/build/script.rs

use crate::shell::cmd::exec;
use crate::die;
use crate::package::Package;

fn setup(package: &Package) {
    let no_source = &package.data.source.url.is_empty();
    let relpath = format!("{}/{}", package.repo, package.name);

    let command = format!(
    r#"

    TMPDIR="/var/tmp/2"
    
    rm -rf    $TMPDIR/building/{}
    mkdir -pv $TMPDIR/building/{} $TMPDIR/extraction

    if {}; then
        echo "Package has no tarball; skipping extraction" >&2
        exit 0
    fi

    # example: /sources/testing/tree/tree=2.2.1.tar.bz2
    tar xf '/sources/{}/{}.tar.'*z* -C $TMPDIR/extraction
    mv -f $TMPDIR/extraction/*/* $TMPDIR/building/{}

    "#,
    relpath,
    relpath,
    no_source,
    relpath, package,
    relpath,
    );

    exec(&command).unwrap_or_else(|e| die!("Failed to setup for '{}': {}", package, e))
}

pub fn build(package: &Package) {
    setup(package);
    let relpath = format!("{}/{}", package.repo, package.name);

    let command = format!(
    r#"
    
    source '/usr/ports/{}/BUILD'
    SRC="/sources/{}"
    cd '/var/tmp/2/building/{}'

    2b

    # TODO: Ensure update logic removes dead files from the previous manifest, and then the manifest
    find D | cut -dD -f2- | sed '/^$/d' > '/usr/ports/{}/.data/MANIFEST={}'

    tar cpf D.tar D
    zstd --rm -f -T0 -19 -o '/usr/ports/{}/.dist/{}.tar.zst' D.tar # TODO: Add a dictionary
    "#,
    relpath,
    relpath,
    relpath,
    relpath, package.version,
    relpath, package,
    );

    exec(&command).unwrap_or_else(|e| die!("Failed to build '{}': {}", package, e));
}

pub fn prep(package: &Package) {
    let relpath = format!("{}/{}", package.repo, package.name);
    let command = format!(
    r#"

    source '/usr/ports/{}/BUILD'
    SRC="/sources/{}"
    type -t 2a > /dev/null 2>&1 || exit 0
    
    2a

    "#,
    relpath,
    relpath,
    );

    exec(&command).unwrap_or_else(|e| die!("Failed to perform preparation steps for '{}': {}", package, e));
}

pub fn post(package: &Package) {
    let relpath = format!("{}/{}", package.repo, package.name);
    let command = format!(
    r#"

    source '/usr/ports/{}/BUILD'
    SRC="/sources/{}"
    type -t daj_post > /dev/null 2>&1 || exit 0 # finish if post is undefined
    cd '/var/tmp/2/building/{}'

    daj_post

    "#,
    relpath,
    relpath,
    relpath,
    );

    exec(&command).unwrap_or_else(|e| die!("Failed to preform post-install steps for '{}': {}", package, e));
}

pub fn clean(package: &Package) {
    let relpath = format!("{}/{}", package.repo, package.name);
    let command = format!(
    r#"

    # TODO: Make cleanup toggleable in the config
    rm -rf '/var/tmp/2/building/{}'
    "#,
    relpath,
    );

    exec(&command).unwrap_or_else(|e| die!("Failed cleanup for '{}': {}", package, e))
}
