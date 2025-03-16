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
[ 540K ↘ ↘  72K ]
󰗠  Built 'whois=5.5.23' in 671.682 ms
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

***Note:** An equivalent command would be ``2 --build main/whois`` which is
just more explicit.*

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
Just to confirm we have it, let's query github's domain.
```
 $ whois github.com
   Domain Name: GITHUB.COM
   Registry Domain ID: 1264983250_DOMAIN_COM-VRSN
   Registrar WHOIS Server: whois.markmonitor.com
   Registrar URL: http://www.markmonitor.com
...
```

### Removing Packages
Great, now that we know github.com is registered by GitHub, Inc., we're done
with whois.
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
***Note:** This will install tree and time from the highest priority repo (main by
default) and lzip from main.*

***Note:** Every flag that takes packages as an argument can take multiple
packages.*

But now those packages have associated logs. I hate logs (they were a pain to
implement for 2). Let's delete them >:)
```
 $ ls -l /var/ports/main/{lzip,tree,time}/.logs/pkg.log
Octal Permissions Size User Date Modified Name
0644  .rw-r--r--  1.1k root 16 Feb 02:39   /var/ports/main/lzip/.logs/pkg.log
0644  .rw-r--r--   650 root 16 Feb 02:39   /var/ports/main/tree/.logs/pkg.log
0644  .rw-r--r--  1.0k root 16 Feb 02:39   /var/ports/main/time/.logs/pkg.log

 $ 2 -p tree time lzip
󰗠  Pruned 3 files for 3 packages in 230289 ns

 $ ls -l /var/ports/main/{lzip,tree,time}/.logs/pkg.log
"/var/ports/main/lzip/.logs/pkg.log": No such file or directory (os error 2)
"/var/ports/main/tree/.logs/pkg.log": No such file or directory (os error 2)
"/var/ports/main/time/.logs/pkg.log": No such file or directory (os error 2)
```
Pruning is kinda like softcore but for the ``--remove`` flag. Only files that
can be safely deleted will be. That means 2 will retain the latest version of
the sources and distribution tarballs.
```
 $ ls -l /var/ports/main/{lzip,tree,time}/.{dist,sources}/*
Octal Permissions Size User Date Modified Name
0644  .rw-r--r--  116k root 16 Feb 02:38   /var/ports/main/lzip/.sources/lzip=1.25.tar.gz
0644  .rw-r--r--   80k root 16 Feb 02:38   /var/ports/main/lzip/.dist/lzip=1.25.tar.zst
0644  .rw-r--r--  597k root 16 Feb 02:38   /var/ports/main/time/.sources/time=1.9.tar.gz
0644  .rw-r--r--   22k root 16 Feb 02:39   /var/ports/main/time/.dist/time=1.9.tar.zst
0644  .rw-r--r--   66k root 16 Feb 02:38   /var/ports/main/tree/.sources/tree=2.2.1.tar.gz
0644  .rw-r--r--   43k root 16 Feb 02:39   /var/ports/main/tree/.dist/tree=2.2.1.tar.zst
```
But let's say we want to clean out this cache -- that is, we want to use new
sources and a new distribution tarball for some reason. You can do that too!
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
 $ find /var/ports/main/lzip/.build -maxdepth 1 | wc -l
38

 $ 2 -c lzip
󰗠  Cleaned 1 packages in 899958 ns

 $ find /var/ports/main/lzip/.build/ -maxdepth 1 | wc -l
1
```
Ok let's do a quick progression skip:
```
 $ 2 -pc main/
󰗠  Cleaned 157 packages in 97.277 ms
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
Looks like it ran the update logic despite the package not being installed.
Generally, this is what force does; it makes 2 bypass state checks to execute
the desired logic. We want the latest version of eza, and we get the latest
version, even if the output is a bit weird.

It's been about 30 seconds since I've installed eza, so it's probably been
updated again.
```
 $ 2 -u eza
󰗠  Up-to-date: 'eza=0.20.21'
```
Tragic.

But this is to be expected since we haven't synced the main repo. I will not be
progression breaking here unfortunately. You'll have to keep reading to learn
how to sync repos (or just figure it out -- it's not *that* unintuitive).

Just as a side note, if you pass ``--install``, 2 checks if an older version of
a package is installed, and updates it if so. Which means you can use ``-fi``
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
If ``--list`` isn't passed any arguments, it'll just list all the packages 💯.

### Checking Logs for Packages
Remember those logs we pruned earlier? I retract my previous statement about
hating logs because I want to check how long it took to build tree.
```
 $ 2 -L tree
...
2025-02-16 03:34:04.479 TRACE | [two::shell::cmd@55]             ~ mkdir: created directory '/tmp/2/extraction'
...
2025-02-16 03:34:04.960 TRACE | [two::shell::cmd@55]             ~ install -d /var/ports/main/tree/.build/D/usr/share/man/man1
2025-02-16 03:34:04.961 TRACE | [two::shell::cmd@55]             ~ install tree /var/ports/main/tree/.build/D/usr/bin/tree; \
2025-02-16 03:34:04.961 TRACE | [two::shell::cmd@55]             ~ install -m 644 doc/tree.1 /var/ports/main/tree/.build/D/usr/share/man/man1/tree.1
2025-02-16 03:34:04.966 TRACE | [two::shell::cmd@55]             ~ Packaging...
2025-02-16 03:34:05.029 TRACE | [two::shell::cmd@66]             ~ [ 144K ↘ ↘  44K ]
2025-02-16 03:34:05.029 INFO  | [two::pm::endpoints@152]         ~ Built 'tree=2.2.1'
```
Inspecting the log timestamps, it looks like it took 550 ms. Not bad at all.

Viewing the logs is particularly useful if you want to try to debug a package
build failure. But surely that would never happen 🫠

### History
Lets say you want to see the progression of a package over time. You're in luck
because there's a changelog for that.
```
 $ 2 -H tree eza
Changelog for 'tree=2.2.1':

  [6715668b9356503e91f06d7cd7f2ece283f31903] [2025-02-09 22:50:41] | Revised tree
  [bab9e80280c923eaf102b684bb6485c5185e9064] [2025-02-09 23:02:20] | Revised tree
  [eddab077cd07ded2b27492f2561dc1a199c13eeb] [2025-02-10 01:36:25] | Generated tree=2.2.1

Changelog for 'eza=0.20.21':

  [59a24160adc94b27ebecd2f5e827af5821a16df3] [2025-01-29 23:57:10] | Updated eza: 0.20.17 -> 0.20.18
  [4a4044597eaa31680105b895d5dd9114c397abf6] [2025-01-30 22:36:33] | Updated eza: 0.20.18 -> 0.20.19
  nothing to commit, working tree clean] [2025-02-06 01:47:50] | Updated eza: 0.20.19 -> 0.20.19
  [20e155494b996deb90146b6620a8c37c704e1c47] [2025-02-08 01:23:45] | Updated eza: 0.20.19 -> 0.20.20
  [d3d89e6f41fafd774a20d81098af30f6bcca0e92] [2025-02-10 03:29:51] | Generated eza=0.20.20
  [e608045bfe300f715d611445635e32dfbd3c3347] [2025-02-13 21:54:37] | Updated eza: 0.20.20 -> 0.20.21
  [87044cc48bd3f5c6dcdc8129a72aafc752f930e5] [2025-02-13 21:54:38] | Generated eza=0.20.21
  [78339b1086be2bd6622fd176312a64cf4e6e158c] [2025-02-13 21:56:10] | Generated eza=0.20.21
  [b81b588730be2e841dae9a88bf70f9cbd10ee445] [2025-02-14 01:33:00] | Generated eza=0.20.21
```
You can also see a blooper from when I was working on log automation. (Eza's
commit hash on 2025-02-06 is not supposed to look like that 😭)

If you're wondering how revisions, updates, and package generation works,
that's covered in [Chapter 3.3](../../advanced/ports/), though you probably
want to read Chapter 3 in order.

### Afterword
That was a lot of package commands. But it's not all of them. Well it is, but
the commands can return different stuff depending on context, like whether a
package is installed or has already been built. And they can do different
things if you combine them with ``--force``, or show different output with
``--quiet``.

Play around with it, and if you have questions, feel free to ask :)
