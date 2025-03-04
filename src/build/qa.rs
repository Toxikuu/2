// src/build/qa.rs
//! Quality assurance checks for builds

use anyhow::{Result, bail};
use crate::package::Package;
use std::{
    fs::{read_dir, File},
    io::Read,
    path::{Path, PathBuf}
};
use walkdir::WalkDir;

pub fn destdir_has_stuff(p: &Package) -> bool {
    let destdir = PathBuf::from(&p.data.port_dir).join(".build").join("D");
    let Ok(dir) = read_dir(destdir) else {
        return false
    };
    dir.into_iter().next().is_some()
}

#[allow(clippy::upper_case_acronyms)]
enum ELF {
    M32,
    M64,
}

pub fn libs_ok(p: &Package) -> bool {
    let destdir = PathBuf::from(&p.data.port_dir).join(".build").join("D");
    let usr = destdir.join("usr");

    // ensure no m64 ELF files exist in lib32
    if collect_libs(&usr.join("lib32"))
        .into_iter()
        .any(|l| matches!(check_elf(&l), Ok(ELF::M64)))
    { return false }

    // ensure no m32 ELF files exist in lib
    if collect_libs(&usr.join("lib"))
        .into_iter()
        .any(|l| matches!(check_elf(&l), Ok(ELF::M32)))
    { return false }

    true
}

fn collect_libs(dir: &Path) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .max_depth(8)
        .into_iter()
        .flatten()
        .filter(|e| {
            let path = e.path();
            !path.is_dir() && !path.is_symlink()
        })
        .filter(|f| {
            let fname = f.file_name().to_string_lossy();
            fname.contains(".so") || fname.contains(".a")
        })
        .map(|e| e.path().to_path_buf())
        .collect()
}

fn check_elf(path: &Path) -> Result<ELF> {
    let mut buf = [0u8; 5];
    let mut f = File::open(path)?;
    f.read_exact(&mut buf)?;

    if buf[0..4] != [0x7F, b'E', b'L', b'F'] {
        bail!("Not an ELF")
    }

    match buf[4] {
        1 => Ok(ELF::M32),
        2 => Ok(ELF::M64),
        _ => bail!("tf wrong with this elf: {path:?}")
    }
}
