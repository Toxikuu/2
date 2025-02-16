#!/usr/bin/env bash
# this release script is meant to be used by me, and assumes several
# rust-developer-specific crates are globally installed

set -e +x

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

good() {
  echo -e "\x1b[32;1m [GOOD] $1\x1b[0m"
}

fail() {
  echo -e "\x1b[31;1m [FAIL] ${1:-Unknown failure}\x1b[0m"
  exit "${2:-1}"
}

pushd . > /dev/null
cd "$SCRIPT_DIR"

info 'Ensuring git status is good...'
if [ "$(git status -s | wc -l)" -ne 0 ]; then
  fail 'Some changes were not committed'
fi
if ! git status | grep -q 'Your branch is up to date with'; then
  info 'Pusing local changes...'
  git push
fi
good 'Git status passing'

info 'Ensuring dependencies are up to date...'
rustup override set nightly
out=$(cargo update 2>&1 | tee /dev/tty)
if ! echo "$out" | grep -q 'Locking 0 packages to'; then
  git add Cargo.lock
  git commit -m 'updated dependencies'
  git push
  good 'Automatically updated dependencies'
fi
good 'Dependency versions passing'

info 'Checking for unused dependencies...'
out=$(cargo udeps | tee /dev/tty)
if ! echo "$out" | grep -q 'All deps seem to have been used.'; then
  fail 'Unused dependencies detected'
fi
good 'Dependency use passing'

info 'Checking for security vulnerabilities...'
out=$(cargo audit | tee /dev/tty)
if echo "$out" | grep -q 'Vulnerable crates found!'; then
  fail 'Failed security audit'
fi
good 'Security audit passing'

info 'Running tests...'
cargo nextest run || fail 'Some tests failed'
good 'Tests passing'

info 'Building the release binary...'
cargo build --release
cargo strip || fail 'Failed to strip binary. Are you missing cargo-strip?'
good 'Built and stripped the binary'

info 'Organizing the release...'
rm -rf release
mkdir -v release

for i in "target/release/two" "Cargo.lock"; do
  cp -v "$i" release
done
good 'Release made'

info 'Validating all scripts...'
( find . -type f -name '*.sh' -print0
  find scripts -type f -print0
) | xargs -0 shellcheck -s bash || warn 'Scripts failed validation'
good 'Scripts passing'

popd > /dev/null
info 'Done!'
