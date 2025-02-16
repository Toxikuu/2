# Updating Packages

## Abstract
This page covers updating packages and describes what that process entails.

### Explanation
The update flag (``-u`` or ``--update``) updates a package. If a distribution
tarball for the new version exists, the new version is installed and then stale
files from the old version are deleted. Otherwise, the package is built first.

If a package is already at its latest version, 2 won't update it unless you
pass the force flag (``-f`` or ``--force``).

### Syntax
The syntax for a package update is as follows:
```2 -u, update <PACKAGE1> <PACKAGE2> <...>```

### Usage
#### Updating a Package
To update hwdata to its latest version, execute the following command:
```
$ 2 -u hwdata
```
If hwdata hasn't been installed, you will be warned with a message similar to the below:
```
Didn't update 'hwdata=0.392' as it's not installed
```
If hwdata is already at its latest version, you'll see a message like the below:
```
󰗠  Up-to-date: 'hwdata=0.392'
```

#### Forced Updates
Let's say you just really want hwdata. You can pass ``-fu`` to forcibly update
it. This will install hwdata if it's not installed, or forcibly update it, even
if to the same version:
```
$ 2 -fu hwdata
```
Note that you may see some strange output, though:
```
 $ 2 -fu hwdata
󱍷  Updating 'hwdata': '' -> '0.392'
󰐗  Installing 'hwdata=0.392'...
/usr
/usr/share
/usr/share/pkgconfig
/usr/share/pkgconfig/hwdata.pc
/usr/share/hwdata
/usr/share/hwdata/oui.txt
/usr/share/hwdata/pci.ids
/usr/share/hwdata/usb.ids
/usr/share/hwdata/iab.txt
/usr/share/hwdata/pnp.ids
Removing dead files for 'hwdata='
'hwdata=0.392' is not installed!
󰗠  Updated to 'hwdata=0.392' in 27.902 ms
```
