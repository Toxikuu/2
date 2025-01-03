// src/cli/args.rs
//! Provides definitions for 2's arguments

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    about = "Simple source-based LFS package manager",
    arg_required_else_help = true,
    after_help = "Complete documentation WILL exist in the future:tm:"
    // TODO: update the above line once documentation DOES exist
)]
/// ### Description
/// Stores flags as a bunch of booleans
/// Stores positional argument 'packages' as `Vec<String>`
pub struct Args {
    // Type: Generic
    // Arguments that are not specific to 2
    // They do not take positional arguments

    /// ### Type
    /// Generic
    ///
    /// ### Description
    /// Increases output verbosity
    ///
    /// In the code, verbose output is called from vpr!()
    #[arg(short = 'v', long)]
    pub verbose: bool,

    /// ### Type
    /// Generic
    ///
    /// ### Description
    /// Displays the version
    #[arg(short = 'V', long)]
    pub version: bool,

    /// ### Type
    /// Generic
    ///
    /// ### Description
    /// Decreases output verbosity
    ///
    /// In practice, this mostly just hides output from the shell commands
    #[arg(short = 'q', long)]
    pub quiet: bool,

    /// ### Type
    /// Generic
    ///
    /// ### Description
    /// Forces actions
    ///
    /// Used in combination with other flags to perform more "forceful" actions
    #[arg(short = 'f', long)]
    pub force: bool,

    // Type: Core
    // Arguments that call core 2 functions

    /// ### Type
    /// Core
    ///
    /// ### Description
    /// Installs packages, building them if necessary, optionally forcibly
    ///
    /// If combined with force, does not forcibly rebuild, only forcibily installs
    #[arg(short = 'i', long)]
    pub install: bool,

    /// ### Type
    /// Core
    ///
    /// ### Description
    /// Builds packages, optionally forcibly
    ///
    /// If combined with force, checks for existing dist tarballs are skipped, and any existing
    /// ones are overwritten.
    #[arg(short = 'b', long)]
    pub build: bool,

    /// ### Type
    /// Core
    ///
    /// ### Description
    /// Removes packages via a manifest system, optionally forcibly
    /// 
    /// If combined with force, bypasses install checks and just removes the relevant files (note
    /// this would require a manifest to exist)
    ///
    /// Checks before removing to ensure directories shared by other packages remain intact
    #[arg(short = 'r', long)]
    pub remove: bool,

    /// ### Type
    /// Core
    ///
    /// ### Description
    /// Updates packages, optionally forcibly
    /// 
    /// If combined with force, bypasses latest version checks
    ///
    /// Upon successful update, removes any dead files from the previous version. Dead file removal
    /// is skipped if forcefully updating to the same version.
    #[arg(short = 'u', long)]
    pub update: bool,

    /// ### Type
    /// Core
    ///
    /// ### Description
    /// Lists packages, including their repo, version, and status
    #[arg(short = 'l', long)]
    pub list: bool,

    /// ### Type
    /// Core
    ///
    /// ### Description
    /// Downloads package sources, optionally forcibly
    ///
    /// If combined with force, overwrites existing sources
    #[arg(short = 'g', long)]
    pub get: bool,

    // Type: Extra
    // Arguments that call non-core 2 functions

    /// ### Type
    /// Extra
    ///
    /// ### Description
    /// Deletes package sources for older versions, optionally forcibly
    ///
    /// If combined with force, removes current package sources, too
    /// ### CURRENTLY UNIMPLEMENTED
    // TODO: implement forced pruning
    #[arg(short = 'p', long)]
    pub prune: bool,

    /// ### Type
    /// Extra
    ///
    /// ### Description
    /// Cleans out the .build folder
    #[arg(short = 'c', long)]
    pub clean: bool,

    // Type: Special
    // Arguments that don't reference packages
    // Only one special argument may be passed, and upon executing their function, they exit 2

    /// ### Type
    /// Special
    ///
    /// ### Description
    /// Takes positional argument 'repositories' (coopted from packages)
    /// Lists available sets for those repositories
    #[arg(short = '@', long)]
    pub list_sets: bool,

    /// ### Type
    /// Special
    ///
    /// ### Description
    /// Takes no arguments
    /// Lists all available repositories
    #[arg(short = '/', long)]
    pub list_repos: bool,

    /// ### Type
    /// Special
    ///
    /// ### Description
    /// Takes positional argument 'repositories' (coopted from packages)
    /// Syncs those repositories
    /// ### CURRENTLY UNIMPLEMENTED
    #[arg(short = 'S', long)] // TODO: implement this, positional arg <REPO>
    pub sync_repo: bool,

    /// ### Type
    /// Positional
    ///
    /// ### Description
    /// The positional arguments passed after all flags
    /// 
    /// Normally, these are treated as packages. However, if a special argument is passed, they are
    /// reinterpreted accordingly
    #[arg(value_name = "PACKAGE", num_args = 0.., value_delimiter = ' ')]
    pub packages: Vec<String>,
}

impl Args {
    /// ### Description
    /// Parses command line arguments
    pub fn init() -> Args {
        Args::parse()
    }
}
