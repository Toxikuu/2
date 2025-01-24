// src/cli/args.rs
//! Provides definitions for 2's arguments

use clap::Parser;

#[derive(Parser, Debug)]
#[allow(clippy::struct_excessive_bools)]
#[command(
    about = "Simple source-based LFS package manager",
    arg_required_else_help = true,
    after_help = "Complete documentation WILL exist in the future™"
    // TODO: update the above line once documentation DOES exist
)]
/// ### Description
/// Stores flags as a bunch of booleans
/// Stores positional argument 'packages' as `Vec<String>`
pub struct Args {
    // Type: Core
    // Arguments that call core 2 functions

    /// Installs packages, building them if necessary
    ///
    /// If combined with force, does not forcibly rebuild, only forcibly installs
    #[arg(short = 'i', long)]
    pub install: bool,

    /// Builds packages
    ///
    /// If combined with force, checks for existing dist tarballs are skipped, and any existing
    /// ones are overwritten.
    #[arg(short = 'b', long)]
    pub build: bool,

    /// Removes packages
    ///
    /// Removal is done via a manifest system
    ///
    /// If combined with force, bypasses install checks and just removes the relevant files (note
    /// this would require a manifest to exist)
    ///
    /// Checks before removing to ensure directories shared by other packages remain intact
    #[arg(short = 'r', long)]
    pub remove: bool,

    /// Updates packages
    ///
    /// If combined with force, bypasses latest version checks
    ///
    /// Upon successful update, removes any dead files from the previous version. Dead file removal
    /// is skipped if forcefully updating to the same version.
    #[arg(short = 'u', long)]
    pub update: bool,

    /// Lists packages
    ///
    /// Includes their repo, version, and status
    #[arg(short = 'l', long)]
    pub list: bool,

    /// Downloads package sources
    ///
    /// If combined with force, overwrites existing sources
    #[arg(short = 'g', long)]
    pub get: bool,

    // Type: Extra
    // Arguments that call non-core 2 functions

    /// Deletes package files for older versions
    ///
    /// The files that are pruned include old manifests, logs, and sources
    ///
    /// If combined with force, removes current package sources, too
    /// TODO: Implement forced pruning
    #[arg(short = 'p', long)]
    pub prune: bool,

    /// Cleans the build directory
    #[arg(short = 'c', long)]
    pub clean: bool,

    /// Displays logs
    #[arg(short = 'L', long)]
    pub logs: bool,

    /// Retrieves upstream versions for packages
    #[cfg(feature = "upstream")]
    #[arg(short = 'U', long)]
    pub upstream: bool,

    // Type: Special
    // Arguments that don't reference packages
    // Only one special argument may be passed, and upon executing their function, they exit 2

    /// Lists all available repositories
    ///
    /// Takes no arguments
    #[arg(short = '/', long)]
    pub list_repos: bool,

    /// Lists available sets for one or more repos
    #[arg(short = '@', long, value_name = "REPO", value_delimiter = ' ', num_args = 1..)]
    pub list_sets: Vec<String>,

    /// Adds one or more repos
    #[arg(short = 'a', long, value_name = "REPO", value_delimiter = ' ', num_args = 1..)]
    pub add_repos: Vec<String>,

    /// Syncs one or more repos
    #[arg(short = 's', long, value_name = "REPO", value_delimiter = ' ', num_args = 1..)]
    pub sync_repos: Vec<String>,

    /// See which packages provide a path
    #[arg(short = 'P', long, value_name = "PATH", value_delimiter = ' ', num_args = 1..)]
    pub provides: Vec<String>,

    /// The positional argument on which most flags act
    #[arg(value_name = "PACKAGE", num_args = 0.., value_delimiter = ' ')]
    pub packages: Vec<String>,

    // Type: Generic
    // Arguments that are not specific to 2
    // They do not take positional arguments

    /// Increases output verbosity
    #[arg(short = 'v', long)]
    pub verbose: bool,

    /// Decreases output verbosity
    #[arg(short = 'q', long)]
    pub quiet: bool,

    /// Forces actions, useful with other flags
    #[arg(short = 'f', long)]
    pub force: bool,

    /// Displays the version
    #[arg(short = 'V', long)]
    pub version: bool,
}

impl Args {
    /// ### Description
    /// Parses command line arguments
    pub fn init() -> Self {
        Self::parse()
    }
}
