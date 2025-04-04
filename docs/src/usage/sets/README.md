# Sets

## Abstract
This page introduces sets.

### Explanation
A set is a collection of packages. These collections are defined at
``/var/ports/<repo>/.sets/<set>``. The set definition is a newline-delimited
list of packages, optionally from other repos.

2 unravels sets into their component packages and performs actions on those.

#### Special Sets
Special sets are any set that 2 generates. These sets are not defined in a
file, but in 2's code.

These are all the special sets:
```
@@, @all       -> A set containing all packages in a repo
@a, @available -> A set containing all available packages in a repo
@i, @installed -> A set containing all installed packages in a repo
@o, @outdated  -> A set containing all outdated packages in a repo
```

### Syntax
Sets are called with the '@set' syntax.

#### Examples
##### @lfs
This is the LFS set, (roughly) composed of the packages that make up Linux From
Scratch.

This is what it looks like:
```
 $ cat /var/ports/main/.sets/@lfs
man-pages
iana-etc
glibc
zlib
bzip2
xz
lz4
zstd
file
readline
m4
```

##### main/@@
This is the all set, a special set composed of every package in the main repo.
