// code/pm/endpoints.rs
//
// defines endpoints for PM

use crate::globals::flags::FLAGS;
use crate::{msg, pr};
use super::PM;
use crate::build::{logic as bl, script};
use crate::fetch::download::download;
use crate::remove::logic as rl;
use crate::utils::time::Stopwatch;

impl PM {
    pub fn install(&self) {
        for package in &self.packages {
            let mut stopwatch = Stopwatch::new();
            stopwatch.start();

            download(package, false);
            if bl::install(package) {
                msg!("Installed '{}' in {}", package, stopwatch.display());
            }
        }
    }

    pub fn update(&self) {
        for package in &self.packages {
            let mut stopwatch = Stopwatch::new();
            stopwatch.start();

            download(package, false);
            if bl::update(package) {
                stopwatch.stop();
                msg!("Updated to '{}' in {}", package, stopwatch.display());
            }
        }
    }

    pub fn remove(&self) {
        for package in &self.packages {
            let mut stopwatch = Stopwatch::new();
            stopwatch.start();

            if rl::remove(package) {
                stopwatch.stop();
                msg!("Removed '{}' in {}", package, stopwatch.display());
            }
        }
    }

    pub fn build(&self) {
        for package in &self.packages {
            let mut stopwatch = Stopwatch::new();
            stopwatch.start();

            download(package, false);
            if bl::build(package) {
                stopwatch.stop();
                msg!("Built '{}' in {}", package, stopwatch.display());
            }
        }
    }

    pub fn get(&self) {
        for package in &self.packages {
            download(package, FLAGS.lock().unwrap().force);
        }
    }

    pub fn prune(&self) {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();

        let mut total_count = 0;
        for package in &self.packages {
            total_count += rl::prune(package);
        }

        stopwatch.stop();
        msg!("Pruned {} files for {} packages in {}", total_count, self.packages.len(), stopwatch.display());
    }

    pub fn clean(&self) {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();

        for package in &self.packages {
            script::clean(package);
        }

        stopwatch.stop();
        msg!("Cleaned {} packages in {}", self.packages.len(), stopwatch.display());
    }

    pub fn list(&mut self) {
        msg!("Packages:");

        self.packages.sort_by(|a, b| {
            let a = format!("{}/{}", a.repo, a);
            let b = format!("{}/{}", b.repo, b);
            a.cmp(&b)
        });

        for package in &self.packages {
            let status = if package.data.is_installed { &format!("\x1b[1;36mInstalled {}", package.data.installed_version) } else { "\x1b[0;30mAvailable" };
            let package_info = format!("  \x1b[0;37m{}/{}", package.repo, package);
            let width = 48 - package_info.len();
            pr!("{} {:<width$} ~ {}", package_info, " ", status);
        }
    }
}
