// src/pm/endpoints.rs
//! Defines endpoints for PM

use indicatif::ProgressStyle;
use once_cell::sync::Lazy;
#[cfg(feature = "parallelism")]
use rayon::prelude::*;
use tracing::{
    debug,
    info,
    warn,
};

use super::PM;
#[cfg(feature = "upstream")]
use crate::upstream::core::check_upstream;
use crate::{
    build::logic as bl,
    cli::args::Args,
    fetch::download::{
        DownloadStatus,
        download,
    },
    globals::config::CONFIG,
    package::{
        Package,
        history,
        parse::expand_set,
        stats,
    },
    remove::logic as rl,
    shell::fs::mkdir,
    utils::{
        comms::{
            erm,
            msg,
            pr,
            vpr,
        },
        fail::Fail,
        hash::try_truncate_commit_hash,
        time::Stopwatch,
    },
};

/// # Description
/// The format for the download bar
const BAR: &str =
    "{prefix:.red} {msg:32.red} [{elapsed_precise}] [{bar:64.red/black}] {bytes}/{total_bytes}";

static STY: Lazy<ProgressStyle> = Lazy::new(|| {
    ProgressStyle::with_template(BAR)
        .fail("[UNREACHABLE] Invalid bar template")
        .progress_chars("=>-")
});

impl PM<'_> {
    /// # Description
    /// High-level PM function
    ///
    /// It interprets all PM-related cli flags and calls the necessary PM methods therefrom
    pub fn run(&self) {
        let a = self.args;
        self.create_all_dotdirs_if_needed();
        self.fetch_all_sources_if_needed(a);

        if a.remove {
            self.packages.iter().for_each(Self::remove);
        }

        #[cfg(feature = "upstream")]
        if a.upstream {
            self.upstream()
        }
        if a.get {
            self.get()
        }

        self.packages.iter().for_each(|p| {
            if a.build {
                Self::build(p)
            }
            if a.install {
                Self::install(p)
            }
            if a.update {
                Self::update(p)
            }
            if a.history {
                Self::history(p)
            }
            if a.about {
                p.about()
            }
            if a.long_about {
                p.long_about()
            }
            if a.stats {
                Self::stats(p)
            }
        });

        if a.prune {
            self.prune()
        }
        if a.clean {
            self.clean()
        }
        if a.list {
            self.list()
        }
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
            | bl::InstallStatus::Already => {
                msg!("󰗠  Already installed '{p}'");
            },
            | bl::InstallStatus::Dist => {
                msg!("󰗠  Installed '{p}' in {}", stopwatch.display());
            },
            | bl::InstallStatus::BuildFirst => {
                PM::build(p);
                PM::install(p);
            },
            | bl::InstallStatus::UpdateInstead => {
                msg!("󱍷  Updating instead of installing '{p}'...");
                PM::update(p);
            },
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
            | bl::UpdateStatus::BuildFirst => {
                PM::build(p);
                PM::update(p);
            },
            | bl::UpdateStatus::Dist => {
                msg!("󰗠  Updated to '{p}' in {}", stopwatch.display());
            },
            | bl::UpdateStatus::Latest => {
                msg!("󰗠  Up-to-date: '{p}'");
            },
            | bl::UpdateStatus::NotInstalled => {
                erm!("Didn't update '{p}' as it's not installed");
            },
        }
    }

    /// # Description
    /// Removes all packages in the PM struct
    fn remove(p: &Package) {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();

        if rl::remove(p) {
            stopwatch.stop();
            msg!("󰗠  Removed '{p}' in {}", stopwatch.display());
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
            | bl::BuildStatus::Source => {
                msg!("󰗠  Built '{p}' in {}", stopwatch.display());

                let mut package_stats = package_stats
                    .efail(|| format!("[UNREACHABLE] Stats for '{p}' should be some but isn't?"));
                package_stats.record_build_time(stopwatch.elapsed());
                stats::save(p, &package_stats).efail(|| format!("Failed to save stats for '{p}'"));
            },
            | bl::BuildStatus::Already => {
                msg!("󰗠  Already built '{p}'");
            },
        }
    }

    /// # Description
    /// Gets (downloads sources for) all packages in the PM struct
    ///
    /// This is separate from ``fetch_all_sources_if_needed()``
    fn get(&self) {
        self.packages.iter().for_each(|p| {
            vpr!("Downloading sources for '{p}'...");

            let status = download(p, self.args.force, &STY);
            if matches!(status, DownloadStatus::Nothing) {
                info!("Didn't download sources for '{p}'");
            } else {
                info!("Downloaded sources for '{p}'");
            }
        });
    }

    /// # Description
    /// Prunes files for all packages in the PM struct
    fn prune(&self) {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();

        let mut total_count = 0;
        self.packages.iter().for_each(|p| {
            let count = rl::prune(p);
            info!("Pruned {p}");

            total_count += count;
        });

        stopwatch.stop();
        msg!(
            "󰗠  Pruned {total_count} files for {} packages in {}",
            self.packages.len(),
            stopwatch.display()
        );
    }

    /// # Description
    /// Cleans the build directory for all packages in the PM struct
    fn clean(&self) {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();

        let mut cleaned = 0;
        self.packages.iter().for_each(|p| {
            cleaned += rl::clean(p);
            debug!("Cleaned build for {p}");
        });

        stopwatch.stop();

        msg!(
            "󰗠  Cleaned {cleaned} files for {} packages in {}",
            self.packages.len(),
            stopwatch.display()
        );
    }

    /// # Description
    /// Lists out all the packages in the PM struct
    ///
    /// If there are no packages, lists all of them
    pub fn list(&self) {
        let packages = self.packages;
        let imply = self.args.packages.is_empty();
        Self::list_packages(packages, "Packages", imply);
    }

    /// # Description
    /// Lists all provided packages
    ///
    /// If none are provided, lists every package
    pub fn list_packages(packages: &[Package], msg: &str, imply: bool) {
        msg!("{msg}:");

        let mut pkgs = packages.to_vec();
        if pkgs.is_empty() {
            if imply {
                pkgs = expand_set("//@@").to_vec();
            } else {
                erm!("Nothing to list");
            }
        }

        if CONFIG.general.alphabetize {
            pkgs.sort_by(|a, b| {
                let a = format!("{}/{}", a.repo, a);
                let b = format!("{}/{}", b.repo, b);
                a.cmp(&b)
            });
        }

        let max_width: usize = 48;
        for p in &pkgs {
            let package_info = format!(
                "  \x1b[0;37m{}/{}={}",
                p.repo,
                p.name,
                try_truncate_commit_hash(&p.version)
            );

            let package_info_len = package_info.len();
            let width = if package_info_len > max_width { 0 } else { max_width - package_info_len };

            pr!("{package_info} {:<width$} ~ {}", " ", p.data.status);
        }
    }

    /// # Description
    /// Displays the upstream version for a package, as well as the local version based on
    /// information from BUILD
    #[cfg(feature = "upstream")]
    fn upstream(&self) {
        let pkgs = if self.packages.is_empty() { expand_set("//@@") } else { self.packages.into() };

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

    fn history(p: &Package) { history::view(p); }

    fn stats(p: &Package) {
        stats::load(p)
            .efail(|| format!("Failed to load package stats for '{p}'"))
            .display(p);
    }

    /// # Description
    /// Fetches the sources for all packages if certain cli flags are passed
    /// This logs "fetching" instead of "downloading" to differentiate between this and ``get()``
    fn fetch_all_sources_if_needed(&self, args: &Args) {
        // 'if needed' means one of these are passed
        if !(args.install || args.update || args.build) {
            debug!("Sources were not automatically fetched as they were not needed");
            return;
        }

        self.packages.iter().for_each(|p| {
            // don't bother downloading sources if the dist exists and the sources aren't needed
            if p.dist_exists() && !args.build {
                return;
            }

            if !matches!(download(p, false, &STY), DownloadStatus::Nothing) {
                info!("Automatically fetched sources for '{p}'");
            }
        });
    }

    /// # Description
    /// Creates necessary dotdirs for all packages contained in PM
    fn create_all_dotdirs_if_needed(&self) {
        // 'if needed' means they don't exist
        const DOTDIRS: [&str; 5] = [".build", ".data", ".dist", ".logs", ".sources"];

        self.packages.iter().for_each(|p| {
            for d in &DOTDIRS {
                let dir = p.data.port_dir.join(d);
                mkdir(&dir)
                    .efail(|| format!("Failed to create dotdir '{}' for '{p}'", dir.display()));
            }
        });
    }
}
