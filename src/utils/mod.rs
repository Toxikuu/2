// src/utils/mod.rs
//! Provides utilities for use elsewhere

pub mod fail;
pub mod logger;
pub mod time;

#[cfg(feature = "parallelism")]
pub mod par;
