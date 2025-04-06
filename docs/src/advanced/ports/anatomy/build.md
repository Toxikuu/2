# Build

## Abstract
This page covers syntax and variables and stuff surrounding `$PORT/BUILD`.

### Metadata
Let's look at `main/bc`'s BUILD.
```
NAME="bc"
VERS="7.0.3"
CATG="core"
DESC="Arbitrary precision numeric processing language"
UPST="https://github.com/gavinhoward/bc.git"
VCMD="git ls-remote --tags --refs '$UPST' | sed 's@.*/@@' | grep -Ev '[a-z]+' | tail -n1"

SOURCE="https://github.com/gavinhoward/bc/releases/download/$VERS/bc-$VERS.tar.xz"
EXTRA=()
```

#### Explanations
```
NAME    - Package name              (required)
VERS    - Package version           (required)
CATG    - Package category          (optional)
DESC    - Package description       (recommended)
UPST    - Package upstream          (optional)
VCMD    - Package version command   (optional)

SOURCE  - Package tarball URI       (optional)
EXTRA   - Extra sources             (optional)
```

#### Notes
##### UPST & VCMD
`UPST` is usually the package's git repo. If there's no repo, you can use a
webpage.

`VCMD` is used whenever `2 -U <repo>/<name>` returns an incorrect version.
Because various tagging schemes exist, this field is sometimes necessary. It
should generally reference `UPST`.

##### SOURCE & EXTRA
You generally want at least one of these, though there are some exceptions.

You can only have one `SOURCE`, but you can have as many `EXTRA`s as you like.
For example usage, see the BUILDs for `main/llvm`, `main/yajl`, and `main/rust`.

***Note:** Git repos are currently not supported as sources, sorry. However, it
is possible to hack their usage in, as is currently done with main/wezterm.*

##### CATG & DESC
There is currently no defined list of categories. There will be a complete list
eventually, but this isn't high priority right now. So for now, use whatever
categories you like for `CATG`.

Try to keep `DESC` short and to the point. Arch Linux, Gentoo, Homebrew, and the
package's upstream are good references for descriptions.

##### VERS
Try to avoid using Gentoo-style 9999 versions as they are not currently
supported. For "live" package builds, use full commit hashes. See `main/tmux`,
`main/x264`, and `main/luajit` for examples.

***Note:** GitHub supports per-commit tarballs, as shown in the aforementioned
examples.*
