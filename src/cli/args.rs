// src/cli/args.rs
//
// defines args

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    about = "Simple source-based LFS package manager",
    arg_required_else_help = true,
    after_help = "Complete documentation WILL exist in the future:tm:"
    // TODO: update the above line once documentation DOES exist
)]
pub struct Args {
    // Generic
    #[arg(short = 'v', long)]
    pub verbose: bool,

    #[arg(short = 'V', long)]
    pub version: bool,

    #[arg(short = 'q', long)]
    pub quiet: bool,

    #[arg(short = 'f', long)]
    pub force: bool,

    // Core
    #[arg(short = 'i', long)]
    pub install: bool,

    #[arg(short = 'b', long)]
    pub build: bool,

    #[arg(short = 'r', long)]
    pub remove: bool,

    #[arg(short = 'u', long)]
    pub update: bool,

    #[arg(short = 'l', long)]
    pub list: bool,

    // Extra
    #[arg(short = 'g', long)]
    pub get: bool,

    #[arg(short = 'p', long)]
    pub prune: bool,

    // Special
    // these arguments don't reference packages

    // lists available sets for a repo
    #[arg(short = '@', long)]
    pub list_sets: bool,

    // list all repos, no further arguments taken
    #[arg(short = '/', long)]
    pub list_repos: bool,

    #[arg(short = 'S', long)] // TODO: implement this, positional arg <REPO>
    pub sync_repo: bool,

    // Positional arguments (packages)
    #[arg(value_name = "PACKAGE", num_args = 0.., value_delimiter = ' ')]
    pub packages: Vec<String>,
}

impl Args {
    pub fn init() -> Args {
        Args::parse()
    }
}
