# Using Packages

## Abstract
This page covers some basic usage of package flags with packages.

***Note:** This page also has a lot of word vomit. Sorry about that. Maybe I'll
split it into multiple more digestible parts in the future.*

### Building Packages
Let's build whois. Let's also pass force to build it even if it's already been
built.
```
 $ 2 -fb whois
...
make[1]: Leaving directory '/var/ports/main/whois/.build/po'
Packaging...
[ 604K ↘ ↘  100K ]
󰗠  Built 'whois=5.5.23' in 607.132 ms
```
It's been built and packaged into a distribution tarball:
```
 $ ls '/var/ports/main/whois/.dist/whois=5.5.23.tar.zst'
 /var/ports/main/whois/.dist/whois=5.5.23.tar.zst
```
Let's run the command again, but without the force flag to see what happens.
```
 $ 2 -b whois
󰗠  Already built 'whois=5.5.23'
```
2 is lazy and, as a rule, doesn't like doing more work than it has to. So if a
task is already complete and it's not forced to re-do it, like with this build,
it's not re-done.

***Note:** An equivalent, more explicit command would be
``2 --build main/whois``.*

### Installing Packages
Let's install whois from the distribution tarball we've just built.
```
 $ 2 -i whois
󰐗  Installing 'whois=5.5.23'...
/usr
/usr/bin
/usr/bin/mkpasswd
/usr/bin/whois
...
󰗠  Installed 'whois=5.5.23' in 25.329 ms
```
Just to confirm we have it, let's query gnu's domain.
```
 $ whois gnu.org
Domain Name: gnu.org
Registry Domain ID: b890491ca78240c19c0ecb066a193ed0-LROR
Registrar WHOIS Server: http://whois.gandi.net
Registrar URL: http://www.gandi.net
...
```

### Removing Packages
Great! We're done with whois now.
```
 $ 2 -r main/whois
'usr/share/bash-completion/completions/whois' -x
'usr/share/bash-completion/completions/mkpasswd' -x
'usr/share/man/man5/whois.conf.5' -x
'usr/share/man/man1/mkpasswd.1' -x
'usr/share/man/man1/whois.1' -x
'usr/bin/whois' -x
'usr/bin/mkpasswd' -x
󰗠  Removed 'whois=5.5.23' in 59.202 ms
```

### Pruning Packages
Let's say you've installed some other packages. If you wanna follow along, go
ahead and install tree, time, and lzip.
```
 $ 2 -i tree time main/lzip
...
```
***Note:** This will install tree and time from the highest priority repo (main
by default) and lzip from main.*

***Note:** Every flag that takes packages as an argument can take multiple
packages.*

But now those packages have associated logs. I hate logs -- let's delete them
\>:)
```
 $ ls -l /var/ports/main/{lzip,tree,time}/.logs/build.log
Octal Permissions Size User Date Modified Name
0644  .rw-r--r--  2.4k root  2 Apr 19:24   /var/ports/main/lzip/.logs/build.log
0644  .rw-r--r--   24k root  2 Apr 19:24   /var/ports/main/tree/.logs/build.log
0644  .rw-r--r--   59k root  2 Apr 19:24   /var/ports/main/time/.logs/build.log

 $ 2 -p tree time lzip
Pruning log '/var/ports/main/tree/.logs/build.log'
Pruning log '/var/ports/main/time/.logs/build.log'
Pruning log '/var/ports/main/lzip/.logs/build.log'
󰗠  Pruned 3 files for 3 packages in 332194 ns
```

Pruning is just a shier version of the ``--remove`` flag. Only files that can be
safely deleted will be. That means 2 will retain the latest version of the
sources and distribution tarballs.

```
 $ ls -l /var/ports/main/{lzip,tree,time}/.{dist,sources}/*
Octal Permissions Size User Date Modified Name
0644  .rw-r--r--  116k root 16 Feb 03:53   /var/ports/main/lzip/.sources/lzip=1.25.tar.gz
0644  .rw-r--r--   80k root  2 Apr 19:24   /var/ports/main/lzip/.dist/lzip=1.25.tar.zst
0644  .rw-r--r--  597k root 16 Feb 04:13   /var/ports/main/time/.sources/time=1.9.tar.gz
0644  .rw-r--r--   22k root  2 Apr 19:24   /var/ports/main/time/.dist/time=1.9.tar.zst
0644  .rw-r--r--   66k root 29 Mar 20:06   /var/ports/main/tree/.sources/tree=2.2.1.tar.gz
0644  .rw-r--r--   48k root  2 Apr 19:24   /var/ports/main/tree/.dist/tree=2.2.1.tar.zst
```
But let's say we want to clean out this cache -- that is, we want to use new
sources and new distribution tarballs. You can do that too!
```
 $ 2 -fp tree time lzip
󰗠  Pruned 6 files for 3 packages in 372445 ns
```
The 6 files that were removed were the distribution and source tarballs. I'm
not gonna show more commands because this section is already verbose enough.
Trust me, they're gone. Or check for yourself :)

### Cleaning Packages
This flag is usually useless, but if you have auto-cleaning disabled in the
config (usually if you're inspecting builds), you can use it to explicitly
clean the builds.
```
 $ du -sh /var/ports/main/lzip/.build
1.2M    /var/ports/main/lzip/.build

 $ 2 -c lzip
󰗠  Cleaned 1 packages in 899958 ns

 $ du -sh /var/ports/main/lzip/.build
4.0K    /var/ports/main/lzip/.build
```

Okay, let's do a quick progression skip:
```
 $ sudo du -sh /var/ports/main
13G     /var/ports/main

 $ 2 -pc main/
...
Removed '/var/ports/main/prename/.build/META.yml'
Removed '/var/ports/main/prename/.build/bin/rename'
Removed '/var/ports/main/prename/.build/bin/rename.PL'
Removed '/var/ports/main/prename/.build/bin'
󰗠  Cleaned 281488 files for 289 packages in 4.553 s

 $ sudo du -sh /var/ports/main
2.9G    /var/ports/main
```
We've just pruned and cleaned every package in the main repo. Repos are covered
in [Chapter 2.4](../repos/), but I wanted to show how I usually use ``--prune``
and ``--clean``.

### Getting Packages
Getting a package entails fetching its sources. Let's get tree and time.
```
 $ 2 -g tree time
󰗠  tree=2.2.1.tar.gz                [00:00:00] [================================================================] 64.15 KiB/64.15 KiB
󰗠  time=1.9.tar.gz                  [00:00:00] [================================================================] 582.79 KiB/582.79 KiB
```
If you didn't see any output, that means you already had those sources. But
maybe we still want to get them. Maybe a download was interrupted and we have a
corrupted tarball or some shit.
```
 $ 2 -fg tree time
󰗠  tree=2.2.1.tar.gz                [00:00:00] [================================================================] 64.15 KiB/64.15 KiB
󰗠  time=1.9.tar.gz                  [00:00:00] [================================================================] 582.79 KiB/582.79 KiB
```

### Updating Packages
Eza sees regular updates, so we'll use that for our example. Let's try updating
it.
```
 $ 2 -u eza
Didn't update 'eza=0.20.21' as it's not installed
```

Oops! Looks like we should install it first. But what happens if we try to
force the update?
```
 $ 2 -fu eza
󱍷  Updating 'eza': '' -> '0.20.21'
󱠇  Building 'eza=0.20.21'...
mkdir: created directory '/tmp/2/extraction'
info: using existing install for 'nightly-x86_64-unknown-linux-gnu'
info: default toolchain set to 'nightly-x86_64-unknown-linux-gnu'

  nightly-x86_64-unknown-linux-gnu unchanged - rustc 1.86.0-nightly (049355708 2025-01-18)
...
'target/release/eza' -> '/var/ports/main/eza/.build/D/usr/bin/eza'
Packaging...
[ 2.1M ↘ ↘  816K ]
󰐗  Installing 'eza=0.20.21'...
/usr
/usr/bin
/usr/bin/eza
Removing dead files for 'eza='
  'eza=0.20.21' is not installed!
󰗠  Built and updated to 'eza=0.20.21' in 42.588 s
```
It ran the update logic despite the package not being installed. Generally, this
is what force does; it makes 2 bypass state checks to execute the desired logic.
We want the latest version of eza, and we get the latest version, even if the
output is a bit weird.

It's been about 30 seconds since I've installed eza, so it's probably outdated
by now.
```
 $ 2 -u eza
󰗠  Up-to-date: 'eza=0.20.21'
```
Tragic.

But this is to be expected since we haven't synced the main repo. I will not be
progression breaking here unfortunately. You'll have to keep reading to learn
how to sync repos (or just figure it out -- it's not *that* unintuitive).

Just as a side note, if you pass ``--install``, 2 checks if an older version of
a package is installed, and updates it instead. Which means you can use ``-fi``
and ``-fu`` pretty much interchangeably, since 2 holds your hand there.
```
 $ # TODO: Replace this with a better example of dead file removal
 $ 2 -i iana-etc
󱍷  Updating instead of installing 'iana-etc=20250213'...
󱍷  Updating 'iana-etc': '20250123' -> '20250213'
󱠇  Building 'iana-etc=20250213'...
mkdir: created directory '/tmp/2/extraction'
install: creating directory '/var/ports/main/iana-etc/.build/D'
install: creating directory '/var/ports/main/iana-etc/.build/D/etc'
'services' -> '/var/ports/main/iana-etc/.build/D/etc/services'
'protocols' -> '/var/ports/main/iana-etc/.build/D/etc/protocols'
Packaging...
[ 568K ↘ ↘  116K ]
󰐗  Installing 'iana-etc=20250213'...
/etc
/etc/services
/etc/protocols
Removing dead files for 'iana-etc=20250123'
󰗠  Built and updated to 'iana-etc=20250213' in 250.296 ms
```

### Listing Packages
Listing packages can be useful if you want to see what you have installed. I've
lost track of what exactly we've installed and removed and updated and pruned,
so let's check.
```
 $ 2 -l tree time lzip whois eza
Packages:
  main/tree=2.2.1                          ~ Installed 2.2.1
  main/time=1.9                            ~ Installed 1.9
  main/lzip=1.25                           ~ Installed 1.25
  main/whois=5.5.23                        ~ Available
  main/eza=0.20.21                         ~ Installed 0.20.21
```
Cool. But maybe I don't even know what packages I want to look at. Then what?
```
 $ 2 -l
Packages:
  main/acl=2.3.2                           ~ Installed 2.3.2
  main/alsa-lib=1.2.13                     ~ Installed 1.2.13
  main/alsa-plugins=1.2.12                 ~ Installed 1.2.12
  main/alsa-utils=1.2.13                   ~ Installed 1.2.13
  main/attr=2.5.2                          ~ Installed 2.5.2
...
  xorg/xcb-proto=1.17.0                    ~ Installed 1.17.0
  xorg/xorgproto=2024.1                    ~ Installed 2024.1
  xorg/xtrans=1.5.2                        ~ Installed 1.5.2
```
If ``--list`` isn't passed any arguments, it'll just list all the packages 💯

### Checking Logs for Packages
Remember those logs we pruned earlier? I retract my previous statement about
hating logs because I want to see the build process for tree.
```
 $ cat /var/ports/main/tree/.logs/build.log
...
cc -O3 -std=c11 -Wpedantic -Wall -Wextra -Wstrict-prototypes -Wshadow -Wconversion -DLARGEFILE_SOURCE -D_FILE_OFFSET_BITS=64 -c -o html.o html.c
cc -O3 -std=c11 -Wpedantic -Wall -Wextra -Wstrict-prototypes -Wshadow -Wconversion -DLARGEFILE_SOURCE -D_FILE_OFFSET_BITS=64 -c -o strverscmp.o strverscmp.c
cc -Wl,--as-needed -o tree tree.o list.o hash.o color.o file.o filter.o info.o unix.o xml.o json.o html.o strverscmp.o
install -d /var/ports/main/tree/.build/D/usr/bin
install -d /var/ports/main/tree/.build/D/usr/share/man/man1
install tree /var/ports/main/tree/.build/D/usr/bin/tree; \
install -m 644 doc/tree.1 /var/ports/main/tree/.build/D/usr/share/man/man1/tree.1
[ 156K ↘ ↘  48K ]
Packaging...
```

Viewing the logs is particularly useful for debugging a package build failure.
But surely that would never happen 🫠

### History
Lets say you want to see the progression of a package over time. You're in luck
because there's a changelog for that.
```
 $ 2 -H tree eza
Changelog for 'tree=2.2.1':

  [6715668b9356503e91f06d7cd7f2ece283f31903] [2025-02-09 22:50:41] | Revised tree
  [bab9e80280c923eaf102b684bb6485c5185e9064] [2025-02-09 23:02:20] | Revised tree
  [eddab077cd07ded2b27492f2561dc1a199c13eeb] [2025-02-10 01:36:25] | Generated tree=2.2.1
  [d50078aa6669a9df104aae704a925be159894a98] [2025-02-20 01:42:23] | Revised tree
  [618b637763ead3fc4af692086a9f9775d79da559] [2025-02-21 02:23:26] | Generated tree=2.2.1
  [a3f7715632beb098f86cf99465aa69be8c20e895] [2025-04-01 01:32:11] | Revised tree
  [98bd702c36d754ebb055b7e8d980558a2ed4a371] [2025-04-01 01:32:11] | Generated tree=2.2.1

Changelog for 'eza=0.21.0':

  [59a24160adc94b27ebecd2f5e827af5821a16df3] [2025-01-29 23:57:10] | Updated eza: 0.20.17 -> 0.20.18
  [4a4044597eaa31680105b895d5dd9114c397abf6] [2025-01-30 22:36:33] | Updated eza: 0.20.18 -> 0.20.19
  nothing to commit, working tree clean] [2025-02-06 01:47:50] | Updated eza: 0.20.19 -> 0.20.19
  [20e155494b996deb90146b6620a8c37c704e1c47] [2025-02-08 01:23:45] | Updated eza: 0.20.19 -> 0.20.20
  [d3d89e6f41fafd774a20d81098af30f6bcca0e92] [2025-02-10 03:29:51] | Generated eza=0.20.20
  [e608045bfe300f715d611445635e32dfbd3c3347] [2025-02-13 21:54:37] | Updated eza: 0.20.20 -> 0.20.21
  [87044cc48bd3f5c6dcdc8129a72aafc752f930e5] [2025-02-13 21:54:38] | Generated eza=0.20.21
  [78339b1086be2bd6622fd176312a64cf4e6e158c] [2025-02-13 21:56:10] | Generated eza=0.20.21
  [b81b588730be2e841dae9a88bf70f9cbd10ee445] [2025-02-14 01:33:00] | Generated eza=0.20.21
  [c04d9f48ee3e5f7a13a1399722ae3cba3a767161] [2025-02-19 00:16:03] | Revised eza
  [cd207a347830f9301ab5f0ae780729e6590a6252] [2025-02-21 02:19:48] | Generated eza=0.20.21
  [5cb74ea996bf37b1403a6659de4d1a3c91143fc3] [2025-02-21 02:32:08] | Updated eza: 0.20.21 -> 0.20.22
  [131ebf0714303512ed15f05279bea7277f3b2ca3] [2025-02-21 02:32:09] | Generated eza=0.20.22
  [e3b2c32f0abc809adb69a4de4d917dd65c38d0c6] [2025-02-25 02:45:35] | Generated eza=0.20.22
  [8439e7f9e151d19135d0f913df268918e0ad6500] [2025-03-03 08:10:28] | Updated eza: 0.20.22 -> 0.20.23
  [1fc29a520a780b842423650f93243ff992703ba4] [2025-03-03 08:10:28] | Generated eza=0.20.23
  [4886a7033cc38a059742ce83a447cd14ffc562c5] [2025-03-16 07:55:53] | Updated eza: 0.20.23 -> 0.20.24
  [c84b53a93acf520e52e4ad6a6ce0e6de74a17ce1] [2025-03-16 07:55:54] | Generated eza=0.20.24
  [1cb0704560fbf643538b866bae347e56dc6ccd31] [2025-04-02 22:36:22] | Updated eza: 0.20.24 -> 0.21.0
  [2ddce48cd24fb37b68282a046b84133879e0dea2] [2025-04-02 22:36:24] | Generated eza=0.21.0
```
You can also see a blooper from when I was working on log automation. Eza's
2025-02-06 commit hash is not supposed to look like that :C

If you're wondering how revisions, updates, and package generation works,
that's covered in [Chapter 3.3](../../advanced/ports/), though you probably
want to read Chapter 3 in order.

### Afterword
That was a lot of package commands. But it's not all of them. And even the ones
covered could return different output depending on whether a package is
installed or has already been built or other factors. And they can do different
things if you combine them with ``--force``, or show different output with
``--quiet``.

Play around with it, and if you have questions, feel free to ask :)
