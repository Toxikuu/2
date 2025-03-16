# Packages

## Abstract
This part of the usage guide covers handling packages.

### Syntax
Packages are formed of two parts, the repo and the name, but you usually only need to pass the name. If multiple repos contain the same package name, 2 will disambiguate them, either by prompt or automatically. The repo and name are delimited by a '/'.

#### Examples
##### main/whois
The whois package from the main repo:
```
main/whois
```

##### tox/kernel
The kernel package from the tox repo:
```
tox/kernel
```

##### tree
The tree package from any repo:
```
tree
```
***Note:** The main repo provides tree, but if tree also exists in another
repo, it will be disambiguated.*

### Anatomy
If you'd like to know more about how a package works, read [Chapter
3.4](../../advanced/ports).

# TODO: Move the below to 3.4
Packages are defined and stored in the ``/var/ports/<repo>/<name>`` directory.
Below is the file hierarchy for main/tree:
```
 /var/ports/main/tree/
├──  .build/
├──  .data/
│   └──  MANIFEST=2.2.1
├──  .dist/
│   └──  tree=2.2.1.tar.zst
├──  .logs/
│   └──  pkg.log
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
This directory houses data about a package, including manifests, install
status, and other data.

#### .dist
This directory houses the distribution tarball for a package. This tarball is
extracted whenever a package is installed.

#### .logs
This directory houses 2's logs for a package in a file called pkg.log.

#### .sources
This directory houses a package's sources. Sources include the source code
tarball and any patches.

#### BUILD
This file defines a package, as well as its build instructions. Feel free to
glance inside it. The BUILD file is documented in depth in [Chapter
3.4](../../advanced/ports).

#### CHANGELOG
This file is an automatically generated log of changes to BUILD. It is also
covered in depth in [Chapter 3.4](../../advanced/ports).

#### LOCK
This file is another automatically generated file. It is a toml-formatted lock,
containing information about a package parsed by 2. It is documented in depth
in [Chapter 3.4](../../advanced/ports/).
