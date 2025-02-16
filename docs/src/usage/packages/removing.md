# Removing Packages

## Abstract
This page covers removing packages and describes what that process entails.

### Explanation
The remove flag (``-r`` or ``--remove``) removes a package. Installed files are
stored in a manifest generated whenever a package is installed. Removal reads
that manifest and removes files specific to that package, so long as they
aren't protected. Directories are only removed if they are empty.

If a package is not installed, 2 won't remove it.

### Usage
#### Removing a Package
The below is an example tree removal:
```
 $ 2 -r main/tree
'usr/share/man/man1/tree.1' -x
'usr/bin/tree' -x
󰗠  Removed 'tree=2.2.1' in 83.879 ms
```
If tree isn't installed, you'll a warning similar to the below:
```
Not installed: 'tree=2.2.1'
```

#### Quietly Removing Multiple Packages
The below is an example of the quiet removal of several packages:
```
 $ 2 -qr tree whois main/time
Not installed: 'tree=2.2.1'
󰗠  Removed 'whois=5.5.23' in 62.941 ms
󰗠  Removed 'time=1.9' in 49.325 ms
```

***Warning:** Currently, there are no protections against removing system-critical packages. For example, 2 **will** remove glibc without flinching. As such, you should be very careful with the remove flag.*
