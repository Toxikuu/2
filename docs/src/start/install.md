# Installing

## Abstract
2 can be installed either with a script or manually. The script is simpler but
less flexibile.

### Install Script
2 provides an install script. Note that the script assumes your system has
git. If your system has rustup, 2 will be compiled from source.
Otherwise, the latest release binary will be fetched.

Execute the following command to fetch and run the script:

```bash
sudo bash <(curl -fsSL 'https://github.com/Toxikuu/2/raw/refs/heads/master/install.sh')
```

The script will create the following directories if they don't exist:
- /usr/ports
- /usr/share/2
- /etc/2

It will either clone or pull [2](https://github.com/Toxikuu/2.git) to
``/usr/share/2``.

It will clone [2-main](https://github.com/Toxikuu/2-main.git) to
``/usr/ports/main`` if needed.

It will also install a bash script to ``/usr/bin/2`` which calls
``/usr/share/2/target/release/two``.

### Manual Installation
If you're unable to use the script (for example, if you don't have git), 2 may
be installed manually. The following steps assume you're the root user.

#### Preparation

Create the necessary directories by running the following command:
```bash
mkdir -pv /usr/ports /usr/share/2 /etc/2
```

#### Acquiring the Sources

Fetch the [source
tarball](https://github.com/Toxikuu/2/archive/refs/heads/master.tar.gz) with
curl, wget, or however else you like, and extract it to ``/usr/share/2``. The
following commands should suffice if you're using wget:
```bash
pushd .
TMPDIR="$(mktemp -d --suffix=-2)"
cd "$TMPDIR"

wget 'https://github.com/Toxikuu/2/archive/refs/heads/master.tar.gz'
tar xf master.tar.gz
mv -iv 2-master/{,.}* /usr/share/2

popd
```

#### Symlinking

The following commands will create all necessary symlinks:
```bash
pushd .
cd /usr/share/2

ln -sfv scripts bin
ln -siv "$PWD"/config.toml /etc/2/
ln -siv "$PWD"/exclusions.txt /etc/2/
ln -siv "$PWD"/repo_priority.txt /etc/2/

popd
```

#### Compiling or Installing the Binary

If you'd like to compile 2, run the following command to check if you have
rustup:
```bash
if command -v rustup > /dev/null 2>&1; then
    echo "OK"
else
    echo "Missing rustup!"
fi
```

If you don't have rustup and would like to use it, follow [its install
instructions](https://rustup.rs/). Once you have rustup, run the following
commands to compile 2:
```bash
pushd .
cd /usr/share/2

rustup toolchain install nightly || true
cargo +nightly build --release

popd
```

If you'd rather just use the latest binary, run the following commands to fetch
it and place it where it belongs:
```bash
pushd .
cd /usr/share/2

mkdir -pv target/release
cd target/release

curl -O 'https://github.com/Toxikuu/2/releases/download/latest/two'
chmod +x two

popd
```

***Note:** You can technically place the binary elsewhere, or directly in
``/usr/bin/``. It is placed in ``/usr/share/2/target/release`` by default since
this is where the helper script expects it, and this is where it would
otherwise be compiled to.*

#### Writing the Helper Script
Run the following commands to create the helper script:

```bash
cat << EOF > /usr/bin/2
#!/usr/bin/env bash

if command -v sudo >/dev/null 2>&1; then
  S=sudo
elif command -v doas >/dev/null 2>&1; then
  S=doas
else
  S=
fi

"\$S" LOG_LEVEL="\$LOG_LEVEL" /usr/share/2/target/release/two "\$@"
EOF

chmod +x /usr/bin/2
```

***Note:** Feel free to adjust this script. For instance, if you hate ``sudo``
with a passion but have ``doas``, you might delete the ``$S`` logic and just
call ``doas`` at the start of the last line instead.*
