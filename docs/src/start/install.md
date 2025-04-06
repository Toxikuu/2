# Installing

## Abstract
The recommended way to install 2 is with the Makefile. There is a script which
is on life support. Manual installs are also a solid option depending on your
use-case.

### Prerequisites
You'll need git and the nightly rust compiler (rustup is recommended).

### Acquiring the Sources
Download the latest release tarball through your preferred method, or use git to
clone the repository and checkout to the latest tag.

***Note:** You probably want a tagged release for some semblance of stability.*

### Makefile
To build and install 2, execute the following commands:
```
./configure
make
make install
```

You can view configuration options with by passing ``--help`` to
``./configure``. The Makefile supports the DESTDIR variable.

### Install Script
If you hate Makefiles or something, 2 also provides an install script. However,
I recommend against this as it's less actively maintained and may lead to
breakage. So uh, here be dragons.

Execute the following command to fetch and run the script:
```
sudo bash <(curl -fsSL 'https://github.com/Toxikuu/2/raw/refs/heads/master/install.sh')
```
