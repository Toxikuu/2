# Building LFS

## Abstract
This page covers setup steps to build Chapter 8 of LFS, starting from the end of
Chapter 7.

### Zstd
Zstd is pretty critical to 2's functionality. But Chapter 7 doesn't build zstd.
So, here's a quick walkthrough of how to build it.

Download the sources for zstd to $LFS/tmp/ or wherever else on the LFS system
you'd like. Extract the tarball and navigate to the sources.

Execute the following command to build and install zstd:
```
 $ make prefix=/usr && make prefix=/usr install && rm -v /usr/lib/libzstd.a
```

### Installing 2
The easiest way to get 2 onto your LFS system is with a DESTDIR install.

Ensure $LFS is mounted, then navigate to 2's sources and execute the following
command:
```
 $ ./configure && make && sudo make DESTDIR=$LFS install
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
