// src/package/stats.rs
//! Tracks stats for packages

use std::{
    fs::{
        self,
        File,
    },
    io::Write,
    time::Duration,
};

use anyhow::Result;
use serde::{
    Deserialize,
    Serialize,
};

use super::Package;
use crate::{
    utils::comms::{
        erm,
        msg,
        pr,
    },
    utils::{
        fail::Fail,
        time::Pretty,
    },
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PackageStats {
    build_times: Vec<u64>,
}

pub fn load(package: &Package) -> Result<PackageStats> {
    let statsfile = package.data.port_dir.join(".data").join("STATS");

    if !statsfile.exists() {
        return Ok(PackageStats::default());
    }

    let contents = fs::read_to_string(statsfile)?;
    let stats: PackageStats = toml::from_str(&contents).fail("Failed to deserialize stats");
    Ok(stats)
}

pub fn save(package: &Package, stats: &PackageStats) -> Result<()> {
    let datadir = package.data.port_dir.join(".data");
    let statsfile = datadir.join("STATS");
    let toml_string = toml::to_string_pretty(&stats).fail("Failed to serialize stats");
    let mut file = File::create(statsfile)?;
    file.write_all(toml_string.as_bytes())?;
    Ok(())
}

impl PackageStats {
    /// # Description
    /// Add a benchmark, as well as its time in milliseconds
    pub fn record_build_time(&mut self, time: Duration) {
        #[allow(clippy::cast_possible_truncation)]
        let time = time.as_micros() as u64;
        self.build_times.push(time);
    }

    pub fn display(&self, package: &Package) {
        if !package.data.port_dir.join(".data").join("STATS").exists() {
            return erm!("No stats exist for '{package}'");
        }

        msg!("Stats for {package}:");
        {
            // TODO: Add last_built timestamp
            let pts = &self.build_times;
            pr!("Last generated: {}", package.timestamp);
            pr!("Average build time: {}", from_micros_f64(avg(pts)).pretty());
            pr!("Total builds: {}", pts.len());
        }
    }
}

#[allow(clippy::cast_precision_loss)]
fn avg(pts: &[u64]) -> f64 {
    if pts.is_empty() {
        return 0.;
    }
    let sum = pts.iter().sum::<u64>();
    (sum / pts.len() as u64) as f64
}

#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_truncation)]
fn from_micros_f64(micros: f64) -> Duration { Duration::from_nanos((micros * 1_000.) as u64) }
