// src/main.rs

mod build;
mod cli;
mod fetch;
mod globals;
mod macros;
mod package;
mod pm;
mod remove;
mod shell;
mod utils;

use cli::args::Args;
use globals::flags;
use package::{parse, sets, repos};
use pm::PM;

fn main() {
    let args = Args::init();
    flags::set(&args);

    // its probably safe :grin:
    if !unsafe { libc::geteuid() == 0 } { die!("2 requires root privileges") }

    // handle special arguments first
    if args.list_sets { return args.packages.iter().for_each(|r| sets::list(r)) }
    if args.list_repos { return repos::list() }

    let packages = parse::parse(&args.packages);
    let mut pm = PM::new(&packages);

    if args.build   { pm.build  ()  }
    if args.install { pm.install()  }
    if args.update  { pm.update ()  }
    if args.get     { pm.get    ()  }
    if args.list    { pm.list   ()  }
    if args.remove  { pm.remove ()  }
}
