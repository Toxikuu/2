# Package

## Abstract
This page explains the functions of various package flags.

## Variables
- ``$REPO`` = A package's repo
- ``$NAME`` = A package's name
- ``$VERS`` = A package's version
- ``$PORT`` = ``/var/ports/$REPO/$NAME``

### Install
The install flag (``-i`` or ``--install``) installs packages. If the package's
distribution tarball (located at ``$PORT/.dist/$NAME=$VERS.tar.zst``) doesn't
exist, the package is built. Packages are only reinstalled if ``--force`` is
passed.

### Build
The build flag (``-b`` or ``--build``) builds packages. A package is considered
built if its distribution tarball exists. An already-built package may be
rebuilt with ``--force``.

### Remove
The remove flag (``-r`` or ``--remove``) removes packages. A package is
considered removed if ``$PORT/.data/INSTALLED`` doesn't exist. If this file is
missing, but you'd still like to attempt a removal, pass ``--force`` bypasses
the install check. 2 will refuse to remove packages categorized as critical.

### Update
The update flag (``-u`` or ``--update``) updates installed packages. A package
is considered up-to-date if the version specified in ``$PORT/BUILD`` matches the
version specified in ``$PORT/.data/INSTALLED``. If ``--force`` is passed and the
package is not installed, the package is installed; if the package is already up
to date, the update logic is run.

### List
The list flag (``-l`` or ``--list``) lists packages. If no arguments are passed,
all packages are listed.

### Get
The get flag (``-g`` or ``--get``) gets packages' sources. If the sources
already exist, they are not re-downloaded unless ``--force`` is passed.

### Prune
The prune flag (``-p`` or ``--prune``) removes files for older versions of
packages. Files subject to pruning include logs, old distribution tarballs, old
manifests, and old source tarballs. Several of these are configurable.

### Clean
The clean flag (``-c`` or ``--clean``) cleans builds for packages. Builds are
always cleaned before a package is built, and optionally automatically cleaned
after.

### History
The history flag (``-H`` or ``--history``) displays history for packages. The
history is just the package's changelog, located at ``$PORT/CHANGELOG``.

### About
The about flag (``-a`` or ``--about``) displays information about packages.

### Long About
The long about flag (``-A`` or ``--long-about``) displays more information about
packages.

### Stats
The stats flag (``-s`` or ``--stats``) displays statistics about packages.

### Upstream
The upstream flag (``-U`` or ``--upstream``) checks against the upstream
versions for packages. If no packages are provided, all packages are checked.

***Note:** The upstream flag is an optional feature which is enabled by
default.***
