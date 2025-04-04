# Packages

## Abstract
This part of the usage guide covers handling packages.

## Variables
- $REPO = A package's repo
- $NAME = A package's name
- $VERS = A package's version
- $PORT = /var/ports/$REPO/$NAME

These are used throughout this chapter.

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
