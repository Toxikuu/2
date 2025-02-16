# Listing Packages

## Abstract
This page covers listing packages.

### Explanation
The list flag (``-l`` or ``--list``) lists packages. If no arguments are passed, it lists every package.

It can be useful for quickly checking whether a package is installed.

### Usage
#### Listing Multiple Packages
```
 $ 2 -l tree whois tox/kernel
Packages:
  main/tree=2.2.1                          ~ Installed 2.2.1
  main/whois=5.5.23                        ~ Available
  tox/kernel=6.13.2                        ~ Installed 6.13.2
```

#### Listing All Packages
```
 $ 2 -l
Packages:
  main/acl=2.3.2                           ~ Installed 2.3.2
  main/alsa-lib=1.2.13                     ~ Installed 1.2.13
  main/alsa-plugins=1.2.12                 ~ Installed 1.2.12
  main/alsa-utils=1.2.13                   ~ Installed 1.2.13
  main/attr=2.5.2                          ~ Installed 2.5.2
...
```
