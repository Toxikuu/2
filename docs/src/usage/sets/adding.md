# Adding Sets

## Abstract
This page covers how to add your own sets.

### Why Bother?
Writing your own sets lets you quickly install and keep up-to-date packages
that you care about. For instance, if you have a collection of packages that
you just can't live without -- let's say, neofetch, fastfetch, pfetch, and
catfetch -- you can write a set to store them all.

### Writing That Set
First, let's see what we're working with.
```
 $ cd /usr/ports/main/.sets
 $ ls
 @lfs
```

Since the lfs set doesn't have what we want (and it has a bunch of useless
bloat like man-pages and glibc), let's write our own set. How about we call it
'essentials'.
```
 # cat << EOF > @essentials
 > neofetch
 > pfetch
 > fastfetch
 > tox/catfetch
 > EOF
```

***Note:** The above commands assume you have the 'tox' repo added. If you
would like access to catfetch (you would like access to catfetch), read
[Chapter 2.4.2](../repos/adding.md) to learn how to add repos.*

### Installing The Set
Let's install @essentials, since I haven't posted a screenshot to r/unixporn in
almost half an hour.
```
 $ 2 -i @essentials
󰗠  neofetch=7.1.0.tar.gz            [00:00:00] [================================================================] 93.16 KiB/93.16 KiB
󰗠  pfetch=0.6.0.tar.gz              [00:00:00] [================================================================] 17.02 KiB/17.02 KiB
󰗠  fastfetch=2.36.1.tar.gz          [00:00:00] [================================================================] 1.12 MiB/1.12 MiB
󰗠  catfetch=1.0.1.tar.gz            [00:00:00] [================================================================] 13.31 KiB/13.31 KiB
󰐗  Installing 'neofetch=7.1.0'...
/usr
/usr/bin
/usr/bin/neofetch
/usr/share
/usr/share/man
/usr/share/man/man1
/usr/share/man/man1/neofetch.1
󰗠  Installed 'neofetch=7.1.0' in 19.428 ms
󰐗  Installing 'pfetch=0.6.0'...
/usr
/usr/bin
/usr/bin/pfetch
󰗠  Installed 'pfetch=0.6.0' in 17.698 ms
󰐗  Installing 'fastfetch=2.36.1'...
/usr
/usr/bin
/usr/bin/fastfetch
󰗠  Installed 'fastfetch=2.36.1' in 19.757 ms
󰐗  Installing 'catfetch=1.0.1'...
/usr
/usr/bin
/usr/bin/catfetch
󰗠  Installed 'catfetch=1.0.1' in 17.573 ms
```

Now I can post that screenshot!
