// src/package/repos.rs
//
// some functions for dealing with package repos

use crate::{erm, pr, die};
use std::fs::read_dir;

pub fn list() {
    let dir = "/usr/ports";
    let entries = match read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            die!("Error checking for repos: {}", e)
        }
    };

    let available: Vec<String> = entries.map(|f| f.unwrap().file_name().into_string().unwrap()).collect();
    if available.is_empty() {
        return erm!("No repos available!");
    }

    available.iter().for_each(|r| pr!("{}", r));
}
