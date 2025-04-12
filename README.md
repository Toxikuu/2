# 2

## Info
2 is a source-based package manager for LFS. It packages packages into
distribution tarballs, allowing for rapid reinstalls, while not compromising on
the perks of low-level build-process control. It uses /var/ports to store
package repositories, which contain build information for individual packages.

## Dependencies
Build:
- Rust (nightly)

Runtime:
- Tar
- Zstd
- Bash

## Installation
2 is still in early development, and you should expect breaking changes.

You can either install 2 with `install.sh` or the Makefile. I recommend the
Makefile as it's more robust, and I may deprecate `install.sh` in the future.
```bash
./configure &&
make        &&
sudo make install
```

To view more options, run `./configure --help`.

Here's the install script if you don't heed here be dragons warnings:
```bash
sudo bash <(curl -fsSL 'https://github.com/Toxikuu/2/raw/refs/heads/master/install.sh')
```

## Updating
The best way to update 2 is with 2:
```bash
2 -u 2
```

Though you can use `install.sh` for updates as well, this is not recommended,
especially if you used the Makefile to install 2.

## Credits
- Huge thanks to the maintainers and readers of the
  [*LFS books](https://www.linuxfromscratch.org/),
  both for inspiration and support.
- Thanks to the authors of the various rust crates 2 depends on.
