# Uninstalling

## Abstract
If for whatever reason you've decided 2 isn't for you, this page walks you through the uninstallation process.

### Script
An uninstallation script is provided in the source directory. If you still have the source directory, you may execute it:
```bash
bash /usr/share/2/uninstall.sh
```

### Manual
If you'd rather manually uninstall 2, the following commands should suffice:
```bash
rm -rvf /usr/share/2 \
        /usr/bin/2   \
        /etc/2
```

If you'd also like to uninstall 2's ports, remove the specific repositories, or the entire directory:
```bash
rm -rvf /usr/ports
```
***Warning:** If you have other files stored in ``/usr/ports``, you'll probably want to only delete 2's package repos.*
