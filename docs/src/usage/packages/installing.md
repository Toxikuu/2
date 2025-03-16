# Installing Packages

## Abstract
This page covers installing packages and describes what that process entails.

### Explanation
The install flag (``-i`` or ``--install``) installs a package from a
distribution tarball. These distribution tarballs are stored in
``/var/ports/<repo>/<name>/.dist/<name>=<version>.tar.zst``. If this tarball
doesn't exist, 2 will build the package from source.

If a package is already installed and it's not outdated, 2 won't install it
unless you pass the force flag (``-f`` or ``--force``).

### Usage
The syntax for a package install is as follows:
```
2 -i, --install <PACKAGE1> <PACKAGE2> <...>
```

### Usage
#### Installing a Package
The below is an example tree install:
```
 $ 2 -i tree
󰐗  Installing 'tree=2.2.1'...
/usr
/usr/bin
/usr/bin/tree
/usr/share
/usr/share/man
/usr/share/man/man1
/usr/share/man/man1/tree.1
󰗠  Installed 'tree=2.2.1' in 17.876 ms
```
If tree has already been installed, you'll see a similar message to the
following instead:
```
󰗠  Already installed 'tree=2.2.1'
```

#### Installing Multiple Packages
The below command installs several alsa packages:
```
$ 2 -i alsa-lib alsa-plugins alsa-utils
```

#### Installing a Package from a Specific Repo
The below command installs whois from the main repo:
```
$ 2 -i main/whois
```

#### Forced Installs
The whois package from the main repo was already installed in example 3. But
maybe you want to forcibly (re)install it. The below command would do that:
```
$ 2 -fi main/whois
```
