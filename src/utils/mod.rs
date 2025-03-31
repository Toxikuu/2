// src/utils/mod.rs
//! Provides utilities for use elsewhere

pub mod comms;
pub mod esc;
pub mod fail;
pub mod hash;
pub mod time;

#[cfg(feature = "parallelism")]
pub mod par;
