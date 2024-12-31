// code/pm/endpoints.rs
//
// defines endpoints for PM

use crate::globals::flags::FLAGS;
use crate::{msg, pr};
use super::PM;
use crate::build::logic as bl;
use crate::fetch::download::download;
use crate::remove::logic as rl;
use crate::utils::time::Stopwatch;

impl PM {
    pub fn install(&self) {
        for package in self.packages.iter() {
            let mut stopwatch = Stopwatch::new();
            stopwatch.start();

            download(package, false);
            if bl::install(package) {
                stopwatch.stop();
                msg!("Installed '{}' in {} s", package, stopwatch.elapsed().as_secs_f32())
            }
            stopwatch.reset();
        }
    }

    pub fn update(&self) {
        for package in self.packages.iter() {
            let mut stopwatch = Stopwatch::new();
            stopwatch.start();

            download(package, false);
            if bl::update(package) {
                stopwatch.stop();
                msg!("Updated to '{}' in {} s", package, stopwatch.elapsed().as_secs_f32())
            }
            stopwatch.reset();
        }
    }

    pub fn remove(&self) {
        for package in self.packages.iter() {
            let mut stopwatch = Stopwatch::new();
            stopwatch.start();

            if rl::remove(package) {
                stopwatch.stop();
                msg!("Removed '{}' in {} s", package, stopwatch.elapsed().as_secs_f32())
            }
            stopwatch.reset()
        }
    }

    pub fn build(&self) {
        for package in self.packages.iter() {
            let mut stopwatch = Stopwatch::new();
            stopwatch.start();

            download(package, false);
            if bl::build(package) {
                stopwatch.stop();
                msg!("Built '{}' in {} s", package, stopwatch.elapsed().as_secs_f32())
            }
            stopwatch.reset();
        }
    }

    pub fn get(&self) {
        for package in self.packages.iter() {
            download(package, FLAGS.lock().unwrap().force);
        }
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
            pr!("{} {:<width$} ~ {}", package_info, " ", status)
        }
    }
}