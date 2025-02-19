# 2=0.0.71
**Changes**
- Added the field port_dir to package
    - Refactored stuff to use it
- Introduced the currently-unused modtime function
- Fixed a typo in the previous changelog

# 2=0.0.70
**Notes**
This is the first version for which this changelog will now exist. I may or may
not continue with the little patch notes in Cargo.toml.

I revamped a significant amount of the codebase, but this isn't in a state
where I'm comfortable incrementing a minor version (though the changes were
certainly enough to warrant it), so 10 patch versions it is!

There are also significant breaking changes with this version.

**Changes**
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
