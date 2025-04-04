# Uninstalling

## Abstract
If for whatever reason you've decided 2 isn't for you, this page walks you
through the uninstallation process.

### Makefile
There exists a make uninstall target:
```
sudo make uninstall
```

### Script
An uninstallation script is provided in the source directory. If you still have
the source directory, you may execute it:
```
./uninstall.sh
```

### Manual
If you'd rather manually uninstall 2, the following commands should suffice:
```
rm -rvf /usr/share/2 \
        /usr/bin/2   \
        /etc/2
```

If you'd also like to uninstall 2's ports, remove the specific repositories, or
the entire directory:
```
rm -rvf /var/ports
```
