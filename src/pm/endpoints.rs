// src/pm/endpoints.rs
//! Defines endpoints for PM

use crate::{
    build::{logic as bl, script},
    cli::args::Args,
    comms::out::{erm, msg, pr, vpr},
    fetch::download::{download, DownloadStatus},
    package::{history, parse::expand_set, Package},
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
const BAR: &str = "{prefix:.red} {msg:32.red} [{elapsed_precise}] [{bar:64.red/black}] {bytes}/{total_bytes}";

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
            if a.logs     { Self::logs    (p) }
            if a.history  { Self::history (p) }
        });

        if a.prune    { self.prune () }
        if a.clean    { self.clean () }
        if a.list     { self.list  () }
    }

    /// # Description
    /// Installs all packages in the PM struct
    fn install(p: &Package) {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();

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
            bl::InstallStatus::Source => {
                log::info!("Built and installed '{p}'");
                msg!("󰗠  Built and installed '{p}' in {}", stopwatch.display());
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
            bl::UpdateStatus::Source => {
                log::info!("Built and updated to '{p}'");
                msg!("󰗠  Built and updated to '{}' in {}", p, stopwatch.display());
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

        let status = bl::build(p, false);
        stopwatch.stop();
        match status {
            bl::BuildStatus::Source => {
                log::info!("Built '{p}'");
                msg!("󰗠  Built '{p}' in {}", stopwatch.display());
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
            if imply { pkgs = expand_set("//@a").to_vec(); }
            else { erm!("Nothing to list"); }
        }

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
    }

    // this intentionally does not log, though I suppose it could
    // TODO: look into the above
    //
    /// # Description
    /// Displays the logs for a package
    fn logs(p: &Package) {
        let log_file_str = format!("/usr/ports/{}/.logs/pkg.log", p.relpath);
        let log_file = Path::new(&log_file_str);

        if logger::display(log_file).is_err() {
            erm!("No logs exist for '{}'", p);
        }
    }

    /// # Description
    /// Displays the upstream version for a package, as well as the local version based on
    /// information from BUILD
    #[cfg(feature = "upstream")]
    fn upstream(&self) {
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

    fn history(p: &Package) {
        history::view(p);
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
