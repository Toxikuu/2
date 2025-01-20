// code/pm/endpoints.rs
//
// defines endpoints for PM

use crate::build::{logic as bl, script};
use crate::comms::log::{msg, pr};
use crate::erm;
use crate::fetch::download::download;
use crate::globals::flags::FLAGS;
use crate::package::{Package, parse::expand_set};
use crate::remove::logic as rl;
use crate::utils::fail::Fail;
use crate::utils::logger;
use crate::utils::time::Stopwatch;
use std::path::Path;
use super::PM;

impl PM {
    fn ready() {
        logger::get().detach();
    }

    pub fn install(&self) {
        Self::ready();
        self.packages.iter().for_each(|p| {
            logger::get().attach(&p.relpath);

            let mut stopwatch = Stopwatch::new();
            stopwatch.start();

            download(p, false);
            if bl::install(p) {
                stopwatch.stop();
                log::info!("Installed '{}'", p);
                msg!("Installed '{}' in {}", p, stopwatch.display());
            }
        });
    }

    pub fn update(&self) {
        Self::ready();
        self.packages.iter().for_each(|p| {
            logger::get().attach(&p.relpath);

            let mut stopwatch = Stopwatch::new();
            stopwatch.start();

            download(p, false);
            if bl::update(p) {
                stopwatch.stop();
                log::info!("Updated to '{}' in {}", p, stopwatch.display());
                msg!("Updated to '{}' in {}", p, stopwatch.display());
            }
        });
    }

    pub fn remove(&self) {
        Self::ready();
        self.packages.iter().for_each(|p| {
            logger::get().attach(&p.relpath);

            let mut stopwatch = Stopwatch::new();
            stopwatch.start();

            if rl::remove(p) {
                stopwatch.stop();
                log::info!("Removed '{}'", p);
                msg!("Removed '{}' in {}", p, stopwatch.display());
            }
        });
    }

    pub fn build(&self) {
        Self::ready();
        self.packages.iter().for_each(|p| {
            logger::get().attach(&p.relpath);

            let mut stopwatch = Stopwatch::new();
            stopwatch.start();

            download(p, false);
            if bl::build(p) {
                stopwatch.stop();
                log::info!("Built '{}'", p);
                msg!("Built '{}' in {}", p, stopwatch.display());
            }
        });
    }

    // TODO: Parallelize this
    pub fn get(&self) {
        Self::ready();
        self.packages.iter().for_each(|p| {
            logger::get().attach(&p.relpath);

            let mut stopwatch = Stopwatch::new();
            stopwatch.start();

            // TODO: add tracking for whether anything was actually downloaded
            download(p, FLAGS.lock().ufail("Failed to lock flags").force);

            stopwatch.stop();
            log::info!("Downloaded sources for '{}'", p);
        });
    }

    // TODO: Parallelize this
    pub fn prune(&self) {
        Self::ready();
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();

        let mut total_count = 0;
        self.packages.iter().for_each(|p| {
            logger::get().attach(&p.relpath);

            let count = rl::prune(p);
            log::info!("Pruned {}", p);

            total_count += count;
        });

        stopwatch.stop();

        logger::get().detach();
        msg!("Pruned {} files for {} packages in {}", total_count, self.packages.len(), stopwatch.display());
    }

    pub fn clean(&self) {
        Self::ready();
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();

        self.packages.iter().for_each(|p| {
            script::clean(p);
            log::debug!("Cleaned build for {}", p);
        });

        stopwatch.stop();

        logger::get().detach();
        msg!("Cleaned {} packages in {}", self.packages.len(), stopwatch.display());
    }

    pub fn list(&self) {
        Self::ready();
        msg!("Packages:");

        let mut pkgs = self.packages.to_vec();
        if pkgs.is_empty() { pkgs = expand_set("@every").to_vec(); }
        pkgs.sort_by(|a, b| {
            let a = format!("{}/{}", a.repo, a);
            let b = format!("{}/{}", b.repo, b);
            a.cmp(&b)
        });

        for p in &pkgs {
            let status = format_package_status(p);
            let package_info = format!("  \x1b[0;37m{}/{}", p.repo, p);
            let width = 48 - package_info.len();
            pr!("{} {:<width$} ~ {}", package_info, " ", status);
        };

        log::info!("Listed {} packages", pkgs.len());
    }

    // this intentionally does not log, though I suppose it could
    // TODO: look into the above
    pub fn logs(&self) {
        Self::ready();
        self.packages.iter().for_each(|p| {
            let log_file_str = format!("/usr/ports/{}/.logs/build.log", p.relpath);
            let log_file = Path::new(&log_file_str);
            
            if logger::display(log_file).is_err() {
                erm!("No logs exist for '{}'", p);
            }
        });
    }
}

fn format_package_status(package: &Package) -> String {
    let iv = &package.data.installed_version;

    if !package.data.is_installed {
        return "\x1b[0;30mAvailable".to_string()
    }

    if *iv != package.version {
        return "\x1b[1;31mOutdated".to_string()
    }

    format!("\x1b[1;36mInstalled {iv}")
}
