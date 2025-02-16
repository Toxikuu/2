#!/usr/bin/env bash
# this release script is meant to be used by me, and assumes several
# rust-developer-specific crates are globally installed

set -e

# assume the script isn't moved out of the source dir because why would it be
# stolen from https://stackoverflow.com/questions/59895/how-do-i-get-the-directory-where-a-bash-script-is-located-from-within-the-script
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# helper functions
warn() {
  echo -e "\x1b[31;1m [WARN] $1\x1b[0m"
}

info() {
  echo -e "\x1b[36;1m [INFO] $1\x1b[0m"
}

fail() {
  echo -e "\x1b[31;1m [FAIL] ${1:-Unknown failure}\x1b[0m"
  exit "${2:-1}"
}

pushd . > /dev/null
cd "$SCRIPT_DIR"

# ensure dependencies are up-to-date
rustup override set nightly
cargo update

# check for unused dependencies
out=$(cargo udeps | tee /dev/tty)
if ! echo "$out" | grep -q 'All deps seem to have been used.'; then
  fail 'Unused dependencies detected'
fi

# check for security vulnerabilities
out=$(cargo audit | tee /dev/tty)
if echo "$out" | grep -q 'Vulnerable crates found!'; then
  fail 'Failed security audit'
fi

cargo build --release
cargo strip || fail 'Failed to strip binary. Are you missing cargo-strip?'

# organize release
rm -rf release
mkdir -v release

for i in "target/release/two" "Cargo.lock"; do
  cp -v "$i" release
done

# check all bash scripts
find . -print0 -type f -name '*.sh' | xargs -0 shellcheck -s bash

popd > /dev/null
