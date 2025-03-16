// src/shell/fs.rs
//! Utility functions for filesystem interactions

use anyhow::{bail, Result};
use crate::erm;
use std::{
    io::ErrorKind as IOE,
    fs::{
        create_dir,
        remove_dir, remove_file,
        read_link,
    },
    path::{Path, PathBuf},
};

/// # Description
/// Creates a directory. Ignores existent directories.
/// Does not recurse.
pub fn mkdir(dir: &Path) -> Result<()> {
    if !dir.exists() {
        create_dir(dir)?;
    }
    Ok(())
}

/// # Description
/// Removes a directory. Ignores attempts to remove missing or populated directories.
///
/// Propagates any other io error
pub fn rmdir(path: &PathBuf) -> Result<()> {
    if let Err(e) = remove_dir(path) {
        match e.kind() {
            IOE::NotFound => {
                erm!("Ignoring '{}': missing", path.display());
                bail!("Missing directory")
            }
            IOE::DirectoryNotEmpty => {
                erm!("Ignoring '{}': populated", path.display());
                bail!("Populated directory")
            },
            _ => bail!("Failed to remove '{}': {e}", path.display())
        }
    }
    Ok(())
}

/// # Description
/// Removes a file or symlink. Ignores attempts to remove missing files.
///
/// Propagates any other io error
pub fn rmf(path: &PathBuf) -> Result<()> {
    if let Err(e) = remove_file(path) {
        match e.kind() {
            IOE::NotFound => {
                erm!("Ignoring '{}': missing", path.display());
                bail!("Missing file")
            }
            _ => bail!("Failed to remove '{}': {e}", path.display())
        }
    }
    Ok(())
}

/// # Description
/// Removes a symlink, file, or directory, deciding which internally.
pub fn rm(path: &PathBuf) -> Result<()> {
    if path.is_symlink() || path.is_file() {
        rmf(path)
    } else {
        rmdir(path)
    }
}

/// # Description
/// Returns true if the given path is a directory, following symlinks.
pub fn is_dir(path: &Path) -> Result<bool> {
    Ok(
        path.is_dir() ||
        (path.is_symlink() && read_link(path)?.is_dir())
    )
}
