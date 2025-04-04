# 2lkit

## Abstract
2lkit is a tool I wrote to make maintenance less painful.

### Installation
Check the [project page](https://github.com/Toxikuu/2lkit/) and acquire the
sources. Build it with cargo, and add the binary to your path. Maybe even write
a bash script to wrap it, as done with 2.

***Note:** 2lkit is currently less ergonomic than 2, since it's only intended
for maintainers. It'll probably become easier build, install, and use in the
future.*

### Usage
```
 $ 2lkit -h

Maintainer utilities for 2

Usage: 2lkit [OPTIONS]

Options:
  -g, --generate <REPO/NAME>...
  -a, --add <REPO/NAME=VERS>...
  -A, --alias <REPO/NAME> <REPO/NAME>
  -r, --revise <REPO/NAME>...
  -v, --view <REPO/NAME>...
  -u, --update <REPO/NAME=VERS>...
  -R, --remove <REPO/NAME>...
  -m, --move <REPO/NAME> <REPO/NAME>
  -c, --cp <REPO/NAME> <REPO/NAME>
      --restore <REPO/NAME> <COMMIT>
  -h, --help                           Print help
  -V, --version                        Print version
```

#### Generate
Generating a port sources `$PORT/BUILD` and writes variables to
`$PORT/LOCK`. It also fetches the sources and hashes them. This flag should
rarely be called manually, as the other flags should call generation when
needed.

#### Add
Use this flag to add a port. It will open `$PORT/BUILD` in `$EDITOR`,
falling back to nvim if unset. From there, fill out metadata and build
instructions.

#### Alias
Use this flag to symlink (or alias) two ports. For instance, to alias `ripgrep`
to `rg`, you'd run `2lkit -A main/ripgrep main/rg`.

#### Revise
This flag is used to edit `$PORT/BUILD`.

#### View
This flag opens `$PORT/BUILD` in read-only mode in nvim.

#### Update
This flag updates a package to its latest version.

#### Remove
This flag removes a port, recursively deleting `$PORT`.

#### Move
This flag moves a port. It can be used to rename a port or move it to another
repo.

#### Copy
This flag is the same as move, but it copies a port. This is particularly useful
whenever you want to tweak the build for a package from another repo. Simply
copy it to your own and edit it to your heart's content.

#### Restore
This flag is used to restore a port to a specific commit. These commits can be
found in `$PORT/CHANGELOG`. This flag should rarely be used.

### Pushing
To push local changes, just `git push` as normal. 2lkit does not automate this.
