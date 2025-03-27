// src/utils/hash.rs
//! Defines functions for handling 2's hashes
//! The hashes are URL-safe base64-encoded sha256 hashes

use crate::utils::fail::Fail;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};
use std::{fs::File, io::Read, path::Path};

pub fn twohash(file_path: &Path) -> String {
    let mut file = File::open(file_path)
        .efail(|| format!("Can't hash missing file: '{}'", file_path.display()));
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];

    while let Ok(n) = file.read(&mut buf) {
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }

    URL_SAFE_NO_PAD.encode(hasher.finalize())
}

pub fn is_commit_hash(s: &str) -> bool {
    s.len() == 40 && s.chars().all(|c| c.is_ascii_hexdigit())
}

pub fn try_truncate_commit_hash(s: &str) -> &str {
    if is_commit_hash(s) { &s[..7] } else { s }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::twohash;

    #[test]
    fn license_hash() {
        let f = PathBuf::from("/code/2/LICENSE");
        assert_eq!(twohash(&f), "OXLcl0T2SZ8Pmy2_dmlvKuetivmyPd5m1q-Gyd-zaYY");
    }

    #[test]
    fn test_safety() {
        let dangerous = ['=', '/', '+']; // '=' isn't dangerous, it's just padding
        let f = PathBuf::from("/usr/bin/test");
        let hash = twohash(&f);

        for c in dangerous {
            assert!(
                !hash.contains(c),
                "Hash '{hash}' contains dangerous character '{c}'"
            );
        }
    }
}
