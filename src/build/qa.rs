// src/build/qa.rs
//! Quality assurance checks for builds

use std::{
    fs::{
        File,
        read_dir,
        read_to_string,
    },
    io::Read,
    path::{
        Path,
        PathBuf,
    },
};

use anyhow::{
    Result,
    bail,
};
use tracing::{
    debug,
    warn,
};
use walkdir::WalkDir;

use crate::{
    package::Package,
    utils::fail::Fail,
};

pub fn envs_properly_initialized(p: &Package) -> bool {
    let build_file = PathBuf::from(&p.data.port_dir).join("BUILD");
    let contents = read_to_string(build_file).fail("Failed to read BUILD");
    let lines = contents.lines().collect::<Vec<_>>();

    if check_env(&lines, "xorg", &["${XORG_CONFIG", "[@]}"]) {
        debug!("Passed QA check 'envs_properly_initialized'");
        true
    } else {
        warn!("Failed QA check 'envs_properly_initialized'");
        false
    }
}

fn check_env(lines: &[&str], env: &str, r#use: &[&str]) -> bool {
    let mut found_with = false;
    let mut found_use = false;

    let mut previous = "";
    for &line in lines {
        if line.contains("with ")
            && line.contains(env)
            && !line.contains('#')
            && !previous.contains("2qa skip")
        {
            found_with = true;
        }

        if r#use.iter().all(|u| line.contains(u)) {
            found_use = true;
        }
        previous = line;
    }

    found_with == found_use
}

pub fn destdir_has_stuff(p: &Package) -> bool {
    let destdir = PathBuf::from(&p.data.port_dir).join(".build").join("D");
    let has_stuff = read_dir(destdir)
        .map(|mut d| d.next().is_some())
        .unwrap_or(false);

    if has_stuff {
        debug!("Passed QA check 'destdir_has_stuff'");
        true
    } else {
        warn!("Failed QA check 'destdir_has_stuff'");
        false
    }
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
    {
        warn!("Found m64 ELF files in lib32");
        return false;
    }

    // ensure no m32 ELF files exist in lib
    if collect_libs(&usr.join("lib"))
        .into_iter()
        .any(|l| matches!(check_elf(&l), Ok(ELF::M32)))
    {
        warn!("Found m32 ELF files in lib");
        return false;
    }

    debug!("Passed QA check 'libs_ok'");
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
        | 1 => Ok(ELF::M32),
        | 2 => Ok(ELF::M64),
        | _ => bail!("tf wrong with this elf: {path:?}"),
    }
}
