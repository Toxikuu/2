# 2=0.0.88
## Changes
- Added efail
    - Used it
    - Improved existing error messages
- Added long about

# 2=0.0.87
## Changes
- Updated the xorg environment QA check
    - Now enforces the xorg configure options array
- Added a function for copying m32 libraries
    - Handles fallbacks
    - Implemented for `mi` and `ni`
- Fixed a critical issue with m32 `mi`
    - It no longer copies everything from 32DEST, instead just copying the libraries (oops!)
- More robustly check for pre- and post-install functions
    - Also added an output line denoting the function execution

# 2=0.0.86
## Changes
- Added a QA check framework for environments
    - Implemented a check for xorg
- Comment support in sets
- Create dotdirs before fetching sources (bugfix)
- Tweaked CHANGES.md formatting to better conform to GitHub release formatting
- Minor refactoring

# 2=0.0.85
## Changes
- Improvements to install.sh
- Added shell::fs to store commonly used fs functions and simplified code accordingly
- Fixed an argument name conflict for repo addition
    - -a -> -+
    - Yes, that's cursed, and yes, it's subject to change
- Added support for short-form repo addition
    - For example, 2 -+ Toxikuu/2-xorg
- Explicitly create dotdirs
    - Should remedy most random crashes due to nonexistent directories
    - Also simplifies other parts of the codebase
- Introduce support for package aliases
    - Package aliases are simply symlinks in the repo dir
    - Currently, ripgrep is an alias of rg

# 2=0.0.84
## Changes
- Fixed overquoting in logs
    - Caused by 'Path' debug display paired with single quotes
- Rewrote package cleaning logic
    - Now displays the number of files cleaned when called with -c

# 2=0.0.83
## Changes
- Made master.log transient
    - It now lives in /tmp/2 so it gets wiped on reboots
- Removed pkg.log and replaced it with a simpler build.log
- Simplified logging logic since the logger no longer has to dynamically connect to multiple files
- Tweaks across the codebase to support the changes

# 2=0.0.82
## Changes
- Fixed a OOM issue with upstream version checking (caused by invalid string formatting)
- Tweaked default config
- Verbosity tweaks
- Non-parallelism bugfix


# 2=0.0.81
## Changes
- Allowed more escape codes for text formatting in config.toml
- Used Option for upstream instead of empty String


# 2=0.0.80
## Notes
I got lazy about committing again, so several patch version bumps.

## Changes
- Added bash debugging when the verbose flag is passed
- Updates to build and package manager logic
- Added package stats
- Moved summary -> about
- Added qa checks
- Tweaked the build abstractions
- More stuff I forgot


# 2=0.0.76
## Changes
- Tweaked scripts to satisfy shellcheck
- Implied all for sync_repos


# 2=0.0.75
## Changes
- Enforced stricter code quality in shell scripts through pedantic shellcheck flags
- Added shell completions


# 2=0.0.74
## Notes
I made a few fairly large changes. Not as bad as with 0.0.70, but I was still quite bad about versioning (oops!). I'm gonna call it 3 patches.

## Changes
- Moved nextest.toml -> .config/nextest.toml
- Quieted some logs for rustls
- Fixed a history formatting bugfix
- Added new fields for packages
    - The new fields are description, dependencies, categories, and timestamps
- Added package category checking before removal
    - You can no longer (easily) remove glibc!
- Added the `--summary` flag
- Minor refactoring
- Moved info.lock -> LOCK
- Fixes and tweaks in the env files


# 2=0.0.71
## Changes
- Added the field port_dir to package
    - Refactored stuff to use it
- Introduced the currently-unused modtime function
- Fixed a typo in the previous changelog


# 2=0.0.70
## Notes
This is the first version for which this changelog will now exist. I may or may
not continue with the little patch notes in Cargo.toml.

I revamped a significant amount of the codebase, but this isn't in a state
where I'm comfortable incrementing a minor version (though the changes were
certainly enough to warrant it), so 10 patch versions it is!

There are also significant breaking changes with this version.

## Changes
- Simplified fail utilities
    - Ufail no longer exists
    - Refactored relevant code
    - FailType no longer exists either
    - Migrated the rest of the codebase to support it
    - Added a configure option to hide failure location
    - Also added BoolFail
- Consistent tab formatting
    - The standard is now 4 spaces everywhere
- Overhauled build API
    - All ports must be migrated
    - Used environments with functions instead of files
    - Added, improved, and simplified abstractions
- Significant improvements to log-display logic
    - You can actually see multiline log entries now!
- General refactoring
- Release script
    - I also added a release script
