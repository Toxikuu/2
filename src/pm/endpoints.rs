// src/pm/endpoints.rs
//! Defines endpoints for PM

use crate::{
    build::{logic as bl, script},
    cli::args::Args,
    comms::out::{erm, msg, pr, vpr},
    fetch::download::{download, DownloadStatus},
    globals::config::CONFIG,
    package::{
        history,
        parse::expand_set,
        stats,
        Package
    },
    remove::logic as rl,
    utils::{
        fail::Fail,
        logger,
        time::Stopwatch,
    }
};
#[cfg(feature = "upstream")]
use crate::upstream::core::check_upstream;
use indicatif::ProgressStyle;
use once_cell::sync::Lazy;
#[cfg(feature = "parallelism")]
use rayon::prelude::*;
use std::path::PathBuf;
use super::PM;

/// # Description
/// The format for the download bar
const BAR: &str = "{prefix:.red} {msg:32.red} [{elapsed_precise}] [{bar:64.red/black}] {bytes}/{total_bytes}";

static STY: Lazy<ProgressStyle> = Lazy::new(|| {
    ProgressStyle::with_template(BAR)
        .fail("Invalid bar template")
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
    /// High-level PM function
    ///
    /// It interprets all PM-related cli flags and calls the necessary PM methods therefrom
    pub fn run(&self) {
        self.fetch_all_sources_if_needed(self.args);
        // args order matters
        let a = self.args;

        if a.remove {
            self.packages.iter().for_each(Self::remove);
        }

        #[cfg(feature = "upstream")]
        if a.upstream { self.upstream () }
        if a.get      { self.get      () }

        self.packages.iter().for_each(|p| {
            Self::ready();
            logger::get().attach(&p.relpath);

            if a.build    { Self::build   (p) }
            if a.install  { Self::install (p) }
            if a.update   { Self::update  (p) }
            if a.history  { Self::history (p) }
            if a.about    { p.about() }
            if a.stats    { Self::stats (p) }
        });

        if a.prune    { self.prune () }
        if a.clean    { self.clean () }
        if a.list     { self.list  () }
        if a.logs     { self.logs  () }
    }

    /// # Description
    /// Installs all packages in the PM struct
    fn install(p: &Package) {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();

        // Note: build and install both return stats because they check it anyway to see if
        // anything should be done
        let status = bl::install(p);
        stopwatch.stop();
        match status {
            bl::InstallStatus::Already => {
                log::warn!("Already installed '{p}'");
                msg!("󰗠  Already installed '{p}'");
            },
            bl::InstallStatus::Dist => {
                log::info!("Installed '{p}'");
                msg!("󰗠  Installed '{p}' in {}", stopwatch.display());
            }
            bl::InstallStatus::BuildFirst => {
                PM::build(p);
                PM::install(p);
            }
            bl::InstallStatus::UpdateInstead => {
                log::warn!("Updating instead of installing '{p}'...");
                msg!("󱍷  Updating instead of installing '{p}'...");
                PM::update(p);
            }
        }
    }

    /// # Description
    /// Updates all packages in the PM struct
    fn update(p: &Package) {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();

        let status = bl::update(p);
        stopwatch.stop();
        match status {
            bl::UpdateStatus::BuildFirst => {
                PM::build(p);
                PM::update(p);
            },
            bl::UpdateStatus::Dist => {
                log::info!("Updated to '{p}'");
                msg!("󰗠  Updated to '{p}' in {}", stopwatch.display());
            }
            bl::UpdateStatus::Latest => {
                log::info!("Up-to-date: '{p}'");
                msg!("󰗠  Up-to-date: '{p}'");
            }
            bl::UpdateStatus::NotInstalled => {
                log::warn!("Didn't update '{p}' as it's not installed");
                erm!("Didn't update '{p}' as it's not installed");
            }
        }
    }

    /// # Description
    /// Removes all packages in the PM struct
    fn remove(p: &Package) {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();

        if rl::remove(p) {
            stopwatch.stop();
            log::info!("Removed '{}'", p);
            msg!("󰗠  Removed '{}' in {}", p, stopwatch.display());
        }
    }

    /// # Description
    /// Builds all packages in the PM struct
    fn build(p: &Package) {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();

        let (status, package_stats) = bl::build(p, false);
        stopwatch.stop();
        match status {
            bl::BuildStatus::Source => {
                log::info!("Built '{p}'");
                msg!("󰗠  Built '{p}' in {}", stopwatch.display());

                let mut package_stats = package_stats.fail("Oops!");
                package_stats.record_build_time(stopwatch.elapsed());
                stats::save(p, &package_stats).fail("Failed to save stats");
            }
            bl::BuildStatus::Already => {
                log::info!("Already built '{p}'");
                msg!("󰗠  Already built '{p}'");
            }
        }
    }

    /// # Description
    /// Gets (downloads sources for) all packages in the PM struct
    ///
    /// This is separate from ``fetch_all_sources_if_needed()``
    fn get(&self) {
        Self::ready();
        self.packages.iter().for_each(|p| {
            logger::get().attach(&p.relpath);
            log::info!("Downloading sources for '{p}'...");
            vpr!("Downloading sources for '{p}'...");

            let status = download(p, self.args.force, &STY);
            if matches!(status, DownloadStatus::Nothing) {
                log::info!("Didn't download sources for '{p}'");
            } else {
                log::info!("Downloaded sources for '{p}'");
            }
        });
    }

    /// # Description
    /// Fetches the sources for all packages if certain cli flags are passed
    /// This logs "fetching" instead of "downloading" to differentiate between this and ``get()``
    fn fetch_all_sources_if_needed(&self, args: &Args) {
        // 'if needed' means one of these are passed
        if ! (args.install || args.update || args.build) {
            log::debug!("Sources were not automatically fetched as they were not needed");
            return
        }

        self.packages.iter().for_each(|p| {
            logger::get().attach(&p.relpath);
            log::info!("Automatically fetching sources for '{p}'...");
            vpr!("Automatically fetching sources for '{p}'...");

            // TODO: add tracking for whether anything was actually downloaded
            download(p, false, &STY);
            log::info!("Automatically fetched sources for '{p}'");
        });
    }

    // TODO: Parallelize this
    // TODO: Add force support (which would prune current sources as well)
    //
    /// # Description
    /// Prunes files for all packages in the PM struct
    fn prune(&self) {
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

        Self::ready();
        stopwatch.stop();
        msg!("󰗠  Pruned {} files for {} packages in {}", total_count, self.packages.len(), stopwatch.display());
    }

    /// # Description
    /// Cleans the build directory for all packages in the PM struct
    fn clean(&self) {
        Self::ready();
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();

        self.packages.iter().for_each(|p| {
            script::clean(p);
            log::debug!("Cleaned build for {}", p);
        });

        stopwatch.stop();

        Self::ready();
        msg!("󰗠  Cleaned {} packages in {}", self.packages.len(), stopwatch.display());
    }

    /// # Description
    /// Lists out all the packages in the PM struct
    ///
    /// If there are no packages, lists all of them
    pub fn list(&self) {
        Self::ready();

        let packages = self.packages;
        let imply = self.args.packages.is_empty();
        Self::list_packages(packages, "Packages", imply);

        log::info!("Listed {} packages", packages.len());
    }

    /// # Description
    /// Lists all provided packages
    ///
    /// If none are provided, lists every package
    pub fn list_packages(packages: &[Package], msg: &str, imply: bool) {
        msg!("{msg}:");

        let mut pkgs = packages.to_vec();
        if pkgs.is_empty() {
            if imply { pkgs = expand_set("//@@").to_vec(); }
            else { erm!("Nothing to list"); }
        }

        if CONFIG.general.alphabetize {
            pkgs.sort_by(|a, b| {
                let a = format!("{}/{}", a.repo, a);
                let b = format!("{}/{}", b.repo, b);
                a.cmp(&b)
            });
        }

        for p in &pkgs {
            let package_info = format!("  \x1b[0;37m{}/{}", p.repo, p);
            let width = 48 - package_info.len();
            pr!("{} {:<width$} ~ {}", package_info, " ", p.data.status);
        };
    }

    // this intentionally does not log, though I suppose it could
    // TODO: look into the above
    //
    /// # Description
    /// Displays the logs for a package
    pub fn logs(&self) {
        Self::ready();

        let pkgs = self.packages;
        if pkgs.is_empty() {
            let log_file = PathBuf::from("/var/log/2/master.log");
            return logger::display(&log_file);
        }

        for p in pkgs {
            let log_file = p.data.port_dir.join(".logs").join("pkg.log");
            if !log_file.exists() {
                erm!("No logs for {p}");
                continue
            }

            logger::display(&log_file);
        }
    }

    /// # Description
    /// Displays the upstream version for a package, as well as the local version based on
    /// information from BUILD
    #[cfg(feature = "upstream")]
    fn upstream(&self) {
        Self::ready();

        let pkgs = if self.packages.is_empty() {
            expand_set("//@@")
        } else {
            self.packages.into()
        };

        let len = pkgs.len();
        vpr!("Checking upstream for {len} packages...");

        #[cfg(not(feature = "parallelism"))]
        {
            pkgs.iter().for_each(|p| {
                vpr!("Checking upstream version for {p}...");
                check_upstream(p);
            });
            msg!("Checked upstream versions for all packages");
        }

        #[cfg(feature = "parallelism")]
        {
            self.thread_pool.install(|| {
                pkgs.into_par_iter().for_each(|p| {
                    vpr!("Checking upstream version for {p}...");
                    check_upstream(p);
                });
            });
            msg!("Checked upstream versions for all packages");
        }
    }

    fn history(p: &Package) {
        history::view(p);
    }

    fn stats(p: &Package) {
        let stats = stats::load(p).fail("Failed to load package stats");
        stats.display(p);
    }
}
