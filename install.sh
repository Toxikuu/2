#!/usr/bin/env bash

[ "$EUID" -ne 0 ] && { echo 'This script must be run as root' >&2 ; exit 1 ;}

confirm() {
  local default="${2:-n}"
  local prompt="${1:-Are you sure?}"

  default="${default,,}"

  if [[ "$default" == "y" ]]; then
    prompt+=" [Y/n] "
  else
    prompt+=" [y/N] "
  fi

  while true; do
    read -r -p "$prompt" ans
    ans=${ans,,}

    [[ -z "$ans" ]] && ans="$default"

    case "$ans" in
      y|yes) return 0 ;;
      n|no) return 1 ;;
      *) echo "Please answer yes or no" >&2 ;;
    esac
  done
}

pushd .
mkdir -pv /usr/ports /usr/share/2 /etc/2

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

confirm 'Replace config?' && ln -sfv "$PWD"/config.toml /etc/2/
confirm 'Replace exclusions?' && ln -sfv "$PWD"/exclusions.txt /etc/2/
confirm 'Replace repo priority?' && ln -sfv "$PWD"/repo_priority.txt /etc/2/

if confirm 'Compile from source (y) or use precompiled binary (n)?'; then
    if ! command -v rustup > /dev/null 2>&1; then
        echo "You don't have rustup; using precompiled binary instead" >&2
        mkdir -pv target/release
        cd target/release
        wget 'https://github.com/Toxikuu/2/releases/latest/download/two'
    fi

    rustup toolchain install nightly || true
    cargo +nightly build --release
fi

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

popd
