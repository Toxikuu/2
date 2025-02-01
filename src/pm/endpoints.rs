// src/pm/endpoints.rs
//! Defines endpoints for PM

use crate::{
    build::{logic as bl, script},
    comms::log::{msg, pr, erm, vpr},
    fetch::download::download,
    globals::flags::FLAGS,
    package::{Package, parse::expand_set, history},
    remove::logic as rl,
    utils::{
        fail::Fail,
        logger,
        time::Stopwatch,
    },
};
#[cfg(feature = "upstream")]
use crate::upstream::core::upstream;
use indicatif::ProgressStyle;
use once_cell::sync::Lazy;
#[cfg(feature = "parallelism")]
use rayon::prelude::*;
use std::path::Path;
use super::PM;

/// # Description
/// The format for the download bar
const BAR: &str = "{msg:.red} [{elapsed_precise}] [{wide_bar:.red/black}] {bytes}/{total_bytes} ({eta})";

static STY: Lazy<ProgressStyle> = Lazy::new(|| {
    ProgressStyle::with_template(BAR)
        .ufail("Invalid bar template")
        .progress_chars("=>-")
});

impl PM<'_> {
    /// # Description
    /// Private function to reset the PM struct to a ready state between actions
    ///
    /// Currently, this just detaches the logger from a specific package
    fn ready() {
        logger::get().detach();
    }

    /// # Description
    /// Installs all packages in the PM struct
    pub fn install(&self) {
        Self::ready();
        self.packages.iter().for_each(|p| {
            logger::get().attach(&p.relpath);

            let mut stopwatch = Stopwatch::new();
            stopwatch.start();

            download(p, false, &STY);
            if bl::install(p) {
                stopwatch.stop();
                log::info!("Installed '{}'", p);
                msg!("Installed '{}' in {}", p, stopwatch.display());
            }
        });
    }

    /// # Description
    /// Updates all packages in the PM struct
    pub fn update(&self) {
        Self::ready();
        self.packages.iter().for_each(|p| {
            logger::get().attach(&p.relpath);

            let mut stopwatch = Stopwatch::new();
            stopwatch.start();

            download(p, false, &STY);
            if bl::update(p) {
                stopwatch.stop();
                log::info!("Updated to '{}'", p);
                msg!("Updated to '{}' in {}", p, stopwatch.display());
            }
        });
    }

    /// # Description
    /// Removes all packages in the PM struct
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

    /// # Description
    /// Builds all packages in the PM struct
    pub fn build(&self) {
        Self::ready();
        self.packages.iter().for_each(|p| {
            logger::get().attach(&p.relpath);

            let mut stopwatch = Stopwatch::new();
            stopwatch.start();

            download(p, false, &STY);
            if bl::build(p) {
                stopwatch.stop();
                log::info!("Built '{}'", p);
                msg!("Built '{}' in {}", p, stopwatch.display());
            }
        });
    }

    /// # Description
    /// Gets (downloads sources for) all packages in the PM struct
    pub fn get(&self) {
        Self::ready();
        self.packages.iter().for_each(|p| {
            logger::get().attach(&p.relpath);
            log::info!("Downloading sources for '{p}'...");
            vpr!("Downloading sources for '{p}'...");

            let force = FLAGS.get().ufail("Cell issue").force;
            // TODO: add tracking for whether anything was actually downloaded
            download(p, force, &STY);

            log::info!("Downloading sources for '{p}'...");
        });
    }
    

    // TODO: Parallelize this
    /// # Description
    /// Prunes files for all packages in the PM struct
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

    /// # Description
    /// Cleans the build directory for all packages in the PM struct
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

    /// # Description
    /// Lists out all the packages in the PM struct
    ///
    /// If there are no packages, lists all of them
    pub fn list(&self, msg: &str) {
        Self::ready();
        msg!("{msg}:");

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
    //
    /// # Description
    /// Displays the logs for a package
    pub fn logs(&self) {
        Self::ready();
        self.packages.iter().for_each(|p| {
            let log_file_str = format!("/usr/ports/{}/.logs/pkg.log", p.relpath);
            let log_file = Path::new(&log_file_str);
            
            if logger::display(log_file).is_err() {
                erm!("No logs exist for '{}'", p);
            }
        });
    }

    /// # Description
    /// Displays the upstream version for a package, as well as the local version based on
    /// information from BUILD
    #[cfg(feature = "upstream")]
    pub fn upstream(&self) {
        Self::ready();
        #[cfg(not(feature = "parallelism"))]
        self.packages.iter().for_each(|p| {
            upstream(p);
        });

        #[cfg(feature = "parallelism")]
        self.thread_pool.install(|| {
            self.packages.par_iter().for_each(|p| {
                upstream(p);
            });
        });
    }

    pub fn history(&self) {
        Self::ready();
        self.packages.iter().for_each(|p| {
            history::view(p);
        });
    }
}

/// # Description
/// Displays the logs for a package
fn format_package_status(package: &Package) -> String {
    let iv = &package.data.installed_version;

    if !package.data.is_installed {
        return "\x1b[0;30mAvailable".to_string()
    }

    if *iv != package.version {
        return format!("\x1b[1;31mOutdated ({iv})")
    }

    format!("\x1b[1;36mInstalled {iv}")
}
