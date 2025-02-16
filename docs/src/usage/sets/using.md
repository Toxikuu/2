# Using sets

## Abstract
This page covers using sets. It also details some common use cases.

### Usage
#### Listing @lfs
The below command lists all packages in @lfs:
```
 $ 2 -l @lfs
Packages:
  main/acl=2.3.2                           ~ Installed 2.3.2
  main/attr=2.5.2                          ~ Installed 2.5.2
  main/autoconf=2.72                       ~ Installed 2.72
  main/automake=1.17                       ~ Installed 1.17
  main/bash=5.2.37                         ~ Installed 5.2.37
  main/bc=7.0.3                            ~ Installed 7.0.3
  main/binutils=2.44                       ~ Installed 2.44
  main/bison=3.8.2                         ~ Installed 3.8.2
...
```
***Note:** When listing sets, 2 alphabetizes. When performing actions on sets,
order is maintained. Whether to alphabetize when listing is configurable.*

#### Building the Available Set
Let's first take a look at the contents of this set:
```
 $ 2 -l main/@a
Packages:
  main/llvm=19.1.7                         ~ Available
  main/rust=1.84.1                         ~ Available
  main/time=1.9                            ~ Available
  main/whois=5.5.23                        ~ Available
```

There are only 4 available packages, and rust has already been built. Surely
llvm won't take hours to compile!
```
 $ 2 -b main/@a
󰗠  Already built 'rust=1.84.1'
󱠇  Building 'llvm=19.1.7'...
mkdir: created directory '/tmp/2/extraction'
...
```
***Note:** Llvm took hours to compile.*

#### Checking for Outdated Packages
Let's check for outdated packages across all repos:
```
 $ 2 -l //@o
Packages:
  main/kernel=6.13.2                       ~ Outdated (6.13.1)
```
But wait, what is the '//' syntax? This only mildly cursed syntax tells 2 to
check across every repo when parsing sets.

Okay, only one package is outdated. Let's fix that:
```
 $ 2 -qu //@o
󱍷  Updating 'kernel': '6.13.1' -> '6.13.2'
󱠇  Building 'kernel=6.13.2'...
󰐗  Installing 'kernel=6.13.2'...
󰗠  Built and updated to 'kernel=6.13.2' in 8.444 s
```
***Note:** I set the kernel speedrun world record here because the kernel build instructions reuse kernel sources whenever possible. Since I already built and installed tox/kernel, main/kernel reused those sources.*
