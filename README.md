# 2

## Info
2 is a source-based package manager for LFS. It packages packages into
distribution tarballs, allowing for rapid reinstalls, while not compromising on
the perks of low-level build-process control. It uses /usr/ports to store
package repositories, which contain build-information for individual packages.

## Installation
2 is still in early development, and you should expect breaking changes.

**WARNING**
I'm in the process of transitioning the m-* scripts to their own rust crate, so
don't use them as a lot of breaking changes were made.

However, here's a work-in-progress install script:
```bash
sudo bash <(curl -fsSL 'https://github.com/Toxikuu/2/raw/refs/heads/master/install.sh')
```

## Credits
- Huge thanks to the maintainers and readers of the [*LFS
books](https://www.linuxfromscratch.org/), both for inspiration and support.
- Thanks to the authors of the various rust crates 2 depends on.
