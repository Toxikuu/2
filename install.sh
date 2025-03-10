#!/usr/bin/env bash
# 2's install script
#
# shellcheck disable=SC2250,SC2310

set -e

[[ "$EUID" -ne 0 ]] && { echo 'This script must be run as root' >&2 ; exit 1 ;}

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
if [[ -e /usr/share/2/.git ]]; then
    git pull
else
    git clone https://github.com/Toxikuu/2.git .
fi

if [[ ! -e /usr/ports/main ]]; then
    git clone https://github.com/Toxikuu/2-main.git /usr/ports/main
fi

confirm 'Replace config?' && ln -sfv "$PWD"/config.toml /etc/2/
confirm 'Replace exclusions?' && ln -sfv "$PWD"/exclusions.txt /etc/2/
confirm 'Replace repo priority?' && ln -sfv "$PWD"/repo_priority.txt /etc/2/
confirm 'Install bash completions?' && install -vDm644 "$PWD"/completions/bash /usr/share/bash-completion/completions/2
confirm 'Install zsh completions?'  && install -vDm644 "$PWD"/completions/zsh  /usr/share/zsh/site-functions/_2
confirm 'Install fish completions?' && install -vDm644 "$PWD"/completions/fish /usr/share/fish/vendor_completions.d/2.fish

binstall() {
    mkdir -pv target/release
    cd target/release
    # TODO: Prefer curl over wget
    wget 'https://github.com/Toxikuu/2/releases/latest/download/two'
    chmod +x two
}

if confirm 'Compile from source (y) or use precompiled binary (n)?'; then
    if command -v rustup > /dev/null 2>&1; then
        rustup toolchain install nightly || true
        cargo +nightly build --release
    else
        echo "You don't have rustup; using precompiled binary instead" >&2
        binstall
    fi
else
    binstall
fi

cat << 'EOF' > /usr/bin/2
#!/usr/bin/env bash

if command -v sudo >/dev/null 2>&1; then
    S="sudo"
elif command -v doas >/dev/null 2>&1; then
    S="doas"
else
    S=""
fi

if [[ -n "$S" ]]; then
    exec "$S" env LOG_LEVEL="$LOG_LEVEL" /usr/share/2/target/release/two "$@"
else
    exec env LOG_LEVEL="$LOG_LEVEL" /usr/share/2/target/release/two "$@"
fi
EOF

chmod +x /usr/bin/2

popd
