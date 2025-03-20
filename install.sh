#!/usr/bin/env bash
# 2's install script
#
# shellcheck disable=SC2250,SC2310

set -e

[[ "$EUID" -ne 0 ]] && { echo 'This script must be run as root' >&2 ; exit 1 ;}

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

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

pushd "${SCRIPT_DIR}" > /dev/null
mkdir -pv /var/ports /usr/share/2 /etc/2

cd /usr/share/2
if [[ -e /usr/share/2/.git ]]; then
    git pull
else
    git clone --depth 1 https://github.com/Toxikuu/2.git .
fi

if [[ ! -e /var/ports/main ]]; then
    git clone --depth 1 https://github.com/Toxikuu/2-main.git /var/ports/main
fi

confirm 'Install config?'           && install -vDm644 config.toml        /etc/2/
confirm 'Install exclusions?'       && install -vDm644 exclusions.txt     /etc/2/
confirm 'Install repo priority?'    && install -vDm644 repo_priority.txt  /etc/2/

confirm 'Install bash completions?' && install -vDm644 completions/bash   /usr/share/bash-completion/completions/2
confirm 'Install zsh completions?'  && install -vDm644 completions/zsh    /usr/share/zsh/site-functions/_2
confirm 'Install fish completions?' && install -vDm644 completions/fish   /usr/share/fish/vendor_completions.d/2.fish

binstall() {
    mkdir -pv target/release
    cd target/release
    curl -Lf -o two 'https://github.com/Toxikuu/2/releases/latest/download/two'
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

install -vDm755 launch.sh /usr/bin/2

popd > /dev/null
