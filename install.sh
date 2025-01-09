#!/usr/bin/env bash

[ "$EUID" -ne 0 ] && { echo 'This script must be run as root' >&2 ; exit 1 ;}

pushd .
mkdir -pv /usr/{ports,share/2} /etc/2

cd /usr/share/2
if [ -e /usr/share/2/.git ]; then
    git pull
else
    git clone https://github.com/Toxikuu/2.git .
fi

if [ ! -e /usr/ports/main ]; then
    git clone https://github.com/Toxikuu/2-main.git /usr/ports/main
fi

ln -sfv scripts bin
ln -siv "$PWD"/config.toml /etc/2/
ln -siv "$PWD"/exclusions.txt /etc/2/

if ! cargo build --release; then
    # wget 'RELEASE URL'
fi

cat << EOF > /usr/bin/2
#!/usr/bin/env bash

sudo -H /usr/share/2/target/release/two "$@"
EOF

popd
