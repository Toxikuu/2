# Usage

## Abstract
This chapter walks you through basic 2 usage.

### Help
The below is the output from the help flag:
```
 $ 2 -h
Simple source-based LFS package manager

Usage: two [OPTIONS] [PACKAGE]...

Arguments:
  [PACKAGE]...  The positional argument on which most flags act

Options:
  -i, --install                  Installs packages, building them if necessary
  -b, --build                    Builds packages
  -r, --remove                   Removes packages
  -u, --update                   Updates packages
  -l, --list                     Lists packages
  -g, --get                      Downloads package sources
  -p, --prune                    Deletes package files for older versions
  -c, --clean                    Cleans the build directory
  -H, --history                  Displays the history for a package
  -a, --about                    Dsiplays the about for a package
  -A, --long-about               Dsiplays a longer about for a package
  -s, --stats                    Displays stats for a package
  -U, --upstream                 Retrieves upstream versions for packages
  -/, --list-repos               Lists all available repositories
  -@, --list-sets <REPO>...      Lists available sets for one or more repos
  -S, --sync-repos [<REPO>...]   Syncs repos
  -+, --add-repos <REPO URL>...  Adds one or more repos
  -P, --provides <PATH>...       See which packages provide a path
  -v, --verbose                  Increases output verbosity
  -q, --quiet                    Decreases output verbosity
  -f, --force                    Forces actions, useful with other flags
  -V, --version                  Displays the version
  -h, --help                     Print help (see more with '--help')

Documentation available at file:///usr/share/doc/2/index.html
```
