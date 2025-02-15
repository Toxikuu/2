# Configuring

## Abstract
You may desire to configure 2. The config options are explained here, along
with some other notes. Sane defaults were selected, so configuration shouldn't
be necessary.

### Symlinks & Upstream
Default configurations are provided in the source tarball. These were symlinked
to /etc/2 in the installation steps. However, they don't have to be symlinks.
In fact, it's probably better to use copies instead of symlinks if you'd like
your configs not to break on updates. However, if you use copies, upstream
changes to the defaults are not reflected, and nor are new config options,
which may also lead to breakage.

### Configuration Files
There's a main configuration file, as well as some text files that 2 parses:
- ``/etc/2/config.toml`` is the main config
- ``/etc/2/exclusions.txt`` allows you to exclude certain paths when installing
packages
- ``/etc/2/repo_priority.txt`` allows you to define repo priority for package
disambiguation

### The Main Config
The main config is split up by section:

#### Flags
The flags section lets you specify defaults for some generic flags.
```toml
[flags]
force = false
quiet = false
verbose = false
```

#### Message
The message section lets you specify custom formatting for 2 messages. These
messages are formatted with ansi escape codes (and optionally other stuff).

***Note:** The '\x1b[' portion of the escape code is handled by 2. This is
subject to change in the future.*
```toml
[message]
danger = "31;1m  "
default = "30;3m"
message = "36;1m"
prompt = "35;1m"
stderr = "31;3;1m"
stdout = "30;3m"
verbose = "34;1m"
```

For example, if you wanted to have red error messages starting with '🤯', you
could specify the following:
```toml
danger = "31;1m🤯 "
```

Gabe Banks's [ansi escape code generator](https://ansi.gabebanks.net/) may be
of use here.

#### Startup
The startup section is for executing certain actions on startup. It is
currently not implemented and ignored.

***Warning:** Startup isn't implemented yet. For now, these config options are
ignored.*
```toml
[startup]
splash = "/usr/share/2/splash"
auto_prune = false
auto_sync = false
```

Splash should point to a text file containing a splash screen or motd. The auto
actions are performed automatically whenever 2 is called.

#### Removal
The removal section is used to control 2's behavior for package removal.
```toml
[removal]
remove_sources = false
remove_dots = false
```

Package sources are removed if the package is removed if ``remove_sources`` is
true. Package distribution tarballs and data are removed if ``remove_dots`` is
true.

#### General
The general section is used to control general/miscellaneous 2 behavior.
```toml
[general]
prefix = "/dry" # where distribution tarballs are extracted to
clean_after_build = false
show_bug_report_message = true
check_hashes = false # whether to check hashes before builds
auto_ambiguity = true # automatically disambiguate ambiguous packages according to repo_priority
log_level = "trace" # one of: trace, debug, info, warn, error
prune_logs = true # whether to remove logs for a package when pruning
prune_manifests = true # whether to remove old manifests for a package when pruning
```

***Warning:** Currently, '/dry' is the default prefix since 2 is not stable.
You can probably change this without incurring damage, but you should keep good
backups anyway.*

#### Upstream
The upstream section defines behavior for upstream version checking.
```toml
[upstream]
max_threads = 64 # the maximum number of threads used for upstream version checking
stack_size = 512 # the size of the stack for each thread
retries = 4 # the number of retries if a version command fails
```


***Note:** Upstream is an optional cargo feature. This section only applies if
it has been enabled. The release binaries have it enabled.*

### The Exclusions File
The exclusions file contains paths to exclude from dist tarball extraction.

The below is my exclusions.txt file:
```bash
# exclusions.txt
# Paths to exclude from dist tarball extraction
# Supports wildcards

usr/share/doc
usr/share/info
usr/share/locale
usr/share/zsh
```

Since I don't care about i18n, documentation, or info pages, I have those paths
excluded. And since I don't use zsh, I don't bother installing any zsh-related
files.

Wildcards are also supported! For example, if you don't want to install any
libtool archives, you could add the following line:
```bash
*.la
```

***Note:** Libtool archives aren't excluded by default since some packages
(cough, cough imagemagick) still use them despite them being mostly obsolete.*

### The Repo Priority File
This file is used to define repo priority for disambiguation. Packages are
disambiguated whenever a package with the same name exists in multiple repos.
The repo priority file becomes especially useful if ``auto_ambiguity`` is
enabled in the main config. If ``auto_ambiguity`` is disabled, this is still
moderately useful, since you'll see the highest priority repos at the top when
manually disambiguating.

The below is my repo_priority.txt:
```bash
# list repos, highest to lowest priority
# comments and empty lines are ignored

tox/
main/
opt/
xorg/
```

Notice the repo 'tox/'. This is my personal repo, containing packages whose
build instructions I've tailored to my system. It has highest priority, since
when I run ``2 -u kernel``, I want to use tox/kernel which contains custom
instructions for where and how the kernel should be installed.
