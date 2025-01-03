// src/package/repos.rs
//
// some functions for dealing with package repos

use crate::utils::fail::Fail;
use crate::{erm, pr};
use std::fs::read_dir;

pub fn list() {
    let dir = "/usr/ports";
    let entries = read_dir(dir).fail("Error checking for repos");

    let available: Vec<String> = entries.map(|f| f.unwrap().file_name().into_string().unwrap()).collect();
    if available.is_empty() { return erm!("No repos available!") }

    available.iter().for_each(|r| pr!("{}", r));
}
