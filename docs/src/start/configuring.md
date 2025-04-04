# Configuring

## Abstract
You may desire to configure 2. The config options are explained here, along
with some other notes. Sane defaults were selected, so configuration shouldn't
be necessary.

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
force                   = false
quiet                   = false
verbose                 = false
```

#### Message
The message section lets you specify custom formatting for 2 messages. These
messages are formatted with ANSI escape codes (and optionally other stuff).

```toml
[message]
danger                  = '\e[31;1m  '
default                 = '\e[30;3m'
message                 = '\e[36;1m'
prompt                  = '\e[35;1m'
stderr                  = '\e[31;3;1m'
stdout                  = '\e[30;3m'
verbose                 = '\e[34;1m'
```

For example, if you wanted to have red error messages starting with '🤯', you
could specify the following:
```toml
danger                  = '\e31;1m🤯 '
```

Gabe Banks's [ANSI escape code generator](https://ansi.gabebanks.net/) may be of
use here. Supported escape sequences include '\e', '\x1b', and '\u001b'.

#### Removal
The removal section is used to control 2's behavior for package removal and
pruning.
```toml
[removal]
remove_sources          = false     # removes sources on package removal
remove_dist             = true      # removes tarballs and data on package removal
prune_logs              = true      # removes (old) logs on package pruning
prune_manifests         = true      # removes (old) manifests on package pruning
prune_dist              = true      # removes (old) distribution tarballs on package pruning
```

#### General
The general section is used to control general/miscellaneous 2 behavior.
```toml
[general]
prefix                  = "/dry"    # where distribution tarballs are extracted to
clean_after_build       = false     # whether to remove $PORT/.build after a package is built
show_bug_report_message = true      # show a bug report message when 2 crashes
show_failure_location   = true      # show where in the code 2 crashed
check_hashes            = false     # whether to check hashes before builds
auto_ambiguity          = true      # automatically disambiguate packages according to repo_priority
log_level               = "info"    # one of: trace, debug, info, warn, error
alphabetize             = true      # whether to display sets in alphabetical order
```

***Warning:** Currently, '/dry' is the default prefix since 2 is not stable.
You can probably change this without incurring damage, but you should keep good
backups anyway.*

#### Upstream
The upstream section defines behavior for upstream version checking.
```toml
[upstream]
max_threads             = 256 # the maximum number of threads used for upstream version checking
stack_size              = 256  # the size (in kibibytes) of the stack for each thread
retries                 = 3 # the number of retries if a version command fails
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
usr/share/licenses
```

Since I don't care about internationalization, documentation, licenses, or info
pages, I have those paths excluded. And since I don't use zsh, I don't bother
installing any zsh-related files.

Wildcards are also supported! For example, if you don't want to install any
libtool archives, you could add the following line:
```bash
*.la
```

***Note:** Libtool archives aren't excluded by default since some packages
(cough, cough imagemagick) still use them.*

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
when I run ``2 -u kernel``, I want to update tox/kernel, which contains custom
instructions for where and how the kernel should be installed.
