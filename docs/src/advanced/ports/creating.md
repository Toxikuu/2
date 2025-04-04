# Creating Ports

## Abstract
This page details the process of creating ports.

### General Advice
- 2lkit is your friend; use it
- Packages should be DESTDIR installed to `$D`
- Make use of env files
- Take your time when writing ports
- Read through `$PORT/BUILD`s for other ports for examples

### Writing a Port
Let's write `main/rsync`.

First, we'll run `2lkit -a main/rsync=3.4.1`. This puts us into `$EDITOR`, for
me nvim:
```
NAME="rsync"
VERS="3.4.1"
DESC=""
CATG=""
UPST=""
DEPS=()

SOURCE=""
EXTRA=()

2b() {

}
```

I'll fill out the metadata fields.
```
NAME="rsync"
VERS="3.4.1"
DESC="Fast incremental file transfer"
CATG="utils cli"
UPST="https://github.com/RsyncProject/rsync.git"
DEPS=(
    "popt"
    "o^xxhash"
)

SOURCE="https://www.samba.org/ftp/rsync/src/rsync-$VERS.tar.gz"
EXTRA=()
```

At this point, it's time to write the build function, `2b()`.
```
2b() {

_cfg_opts=(
    $(ii xxhash &>/dev/null || echo --disable-xxhash)
    --without-included-zlib
)

cfg ${_cfg_opts[@]}
mk
mi

}
```

I've been using the `_cfg_opts` array to store most configure options. I'd
recommend you do the same, as it lets you easily add comments and such.

#### What's happening?
I use the `ii` command to check if xxhash is installed. If it isn't,
`--disable-xxhash` is added to `_cfg_opts`.

I then pass that array to `cfg`, which is a wrapper around `./configure`. By
default, `cfg` passes `--prefix=/usr` and `--disable-static` to `./configure`.
The `${_cfg_opts[@]}` are appended.

Then I call `mk`, which is just a wrapper around `make`. Lastly, I call `mi`,
which is a wrapper around `make install`, which automatically handles DESTDIR.

### With
The `with` command is used to source more envs. For instance, if I wanted to
install a package written in rust, I'd specify `with rust`. If I wanted to use
`meson` and `ninja`, I'd specify `with meson ninja`.

By default, the `/usr/share/2/envs/core` is the only env that's sourced. This
env contains the standard configure and make wrappers, as well as some nice
utilities like `ii`.

I'd highly recommend referencing the wrapper definitions in `/usr/share/2/envs`

### Other Functions
Other functions may also be of use when writing build scripts. `2z()` in
particular comes up often. This function is evaluated after a package is
installed. Run a grep across a repo to find examples of its use -- an exercise
for the reader :) -- and an exercise in writing bad documentation for me :)
