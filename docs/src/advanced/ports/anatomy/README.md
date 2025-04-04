# Anatomy

## Abstract
This page covers the anatomy of a port, explaining what the files and folders
do.

### Anatomy
Packages are defined and stored in `$PORT`, which is `/var/ports/$REPO/$NAME`.
Below is the file hierarchy for main/tree:
```
 /var/ports/main/tree/
├──  .build/
├──  .data/
│   ├──  INSTALLED
│   ├──  MANIFEST=2.2.1
│   └──  STATS
├──  .dist/
│   └──  tree=2.2.1.tar.zst
├──  .logs/
│   └──  build.log
├──  .sources/
│   └──  tree=2.2.1.tar.gz
├──  BUILD
├──  CHANGELOG
└──  LOCK
```

#### .build
This directory houses the package build. This is where a package is compiled
from source before being packaged into a distribution tarball.

#### .data
This directory stores data about a package, including manifests, install status,
and other data.

#### .dist
This directory stores the distribution tarball for a package. This tarball is
extracted whenever a package is installed.

#### .logs
This directory houses the build log. In the future, it may include other logs.

#### .sources
This directory houses a package's sources. Sources include the source code
tarball, any patches, and any other assets.

#### BUILD
This file defines a package, as well as its build instructions. Feel free to
glance inside it. The BUILD file is documented in depth on [the next
page](./build).

#### CHANGELOG
This file is an automatically generated log of changes to BUILD.

#### LOCK
This file is another automatically generated file. It is a toml-formatted lock,
containing information about a package parsed by 2. This file is generated from
BUILD.
