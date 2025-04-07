# Building LFS

## Abstract
This page covers setup steps to build Chapter 8 of LFS, starting from the end of
Chapter 7.

### Installing 2
The easiest way to get 2 onto your LFS system is with a DESTDIR install.

Ensure $LFS is mounted, then navigate to 2's sources and execute the following
command:
```
 $ ./configure && make && sudo make DESTDIR=$LFS install
```

### Zstd
At this point, you can chroot into your LFS system and list packages with 2.
But to build or install packages, you'll need zstd.

LFS Chapter 7 does not build zstd. Thankfully, zstd can be built immediately
without requiring any dependencies:

```
 $ 2 -pg main/zstd
 $ cd /tmp
 $ tar xf /var/ports/main/zstd/.sources/zstd=*.tar.gz
 $ make prefix=/usr
 $ make prefix=/usr install
 $ rm -vf /usr/lib/libzstd.a
```

And you should now be able to install stuff:
```
 $ 2 -i iana-etc
󱠇  Building 'iana-etc=20250403'...
mkdir: created directory '/tmp/2/extraction'
install: creating directory '/var/ports/main/iana-etc/.build/D'
install: creating directory '/var/ports/main/iana-etc/.build/D/etc'
'services' -> '/var/ports/main/iana-etc/.build/D/etc/services'
'protocols' -> '/var/ports/main/iana-etc/.build/D/etc/protocols'
Packaging...
[ 568K ↘ ↘  116K ]
󰗠  Built 'iana-etc=20250403' in 286.343 ms
󰐗  Installing 'iana-etc=20250403'...
/etc
/etc/protocols
/etc/services
󰗠  Installed 'iana-etc=20250403' in 42.446 ms
```

### Building LFS
Now that 2 has been installed to $LFS, you can install @lfs.
```
 $ 2 -i main/@lfs
```

***Note:** The above command should be executed from within the $LFS chroot.*

### Extra Steps
Simply installing all the packages won't make the system bootable though. You'll
need to create `/etc/fstab`, install lfs-bootscripts, probably tweak your kernel,
set a password, etc.
