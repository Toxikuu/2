# Auxiliary Build Files

## Abstract
This page describes the usage of build files and assets that exist outside of
`$PORT/build` or `$PORT/.sources`.

### The Gist
So 2lkit doesn't have a good way to handle these, and the behavior isn't
standardized in 2.

For now, just put auxiliary build files somewhere (not in a hidden directory)
under `$PORT`. See `main/linux-pam` for an example. Then just reference those in
`$PORT/BUILD`. Auxiliary files should be added while the editor is open so that
they get included 2lkit's automatic commit.

### Tip
Generally, if a file's contents are not constant, they should be defined within
`$PORT/BUILD` -- in other words, don't `sed` files in `$PORT` unless they're
adjusted after being built.
