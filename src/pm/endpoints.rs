// code/pm/endpoints.rs
//
// defines endpoints for PM

use crate::globals::flags::FLAGS;
use crate::comms::log::{msg, pr};
use super::PM;
use crate::build::{logic as bl, script};
use crate::fetch::download::download;
use crate::remove::logic as rl;
use crate::utils::time::Stopwatch;
use crate::utils::fail::Fail;
use crate::package::{Package, parse::expand_set};

impl PM {
    pub fn install(&self) {
        self.packages.iter().for_each(|p| {
            let mut stopwatch = Stopwatch::new();
            stopwatch.start();

            download(p, false);
            if bl::install(p) {
                msg!("Installed '{}' in {}", p, stopwatch.display());
            }
        });
    }

    pub fn update(&self) {
        self.packages.iter().for_each(|p| {
            let mut stopwatch = Stopwatch::new();
            stopwatch.start();

            download(p, false);
            if bl::update(p) {
                stopwatch.stop();
                msg!("Updated to '{}' in {}", p, stopwatch.display());
            }
        });
    }

    pub fn remove(&self) {
        self.packages.iter().for_each(|p| {
            let mut stopwatch = Stopwatch::new();
            stopwatch.start();

            if rl::remove(p) {
                stopwatch.stop();
                msg!("Removed '{}' in {}", p, stopwatch.display());
            }
        });
    }

    pub fn build(&self) {
        self.packages.iter().for_each(|p| {
            let mut stopwatch = Stopwatch::new();
            stopwatch.start();

            download(p, false);
            if bl::build(p) {
                stopwatch.stop();
                msg!("Built '{}' in {}", p, stopwatch.display());
            }
        });
    }

    pub fn get(&self) {
        self.packages.iter().for_each(|p| {
            download(p, FLAGS.lock().ufail("Failed to lock flags").force);
        });
    }

    pub fn prune(&self) {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();

        let mut total_count = 0;
        self.packages.iter().for_each(|p| {
            total_count += rl::prune(p);
        });

        stopwatch.stop();
        msg!("Pruned {} files for {} packages in {}", total_count, self.packages.len(), stopwatch.display());
    }

    pub fn clean(&self) {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();

        self.packages.iter().for_each(|p| {
            script::clean(p);
        });

        stopwatch.stop();
        msg!("Cleaned {} packages in {}", self.packages.len(), stopwatch.display());
    }

    pub fn list(&self) {
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
