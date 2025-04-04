# Maintaining Ports

## Abstract
This page covers various maintenance tasks for ports, including revisions,
updates, and upstream version checks.

### Revising Ports
To revise a port, run `2lkit -r <repo>/<name>`. Make whatever edits you like,
then save and exit. The package will be regenerated and changes will be
committed.

From there, test the package to ensure it builds and installs (`2 -fbi
<repo>/<name>`) properly, then push your local changes.

### Updating Ports
To update a port, run `2lkit -u <repo>/<name>=<version>`. This will
automatically replace the version specified after 'VERS=' in `$PORT/BUILD`.

You can also make any necessary changes while in the editor, though generally,
especially for point versions, little in the build process changes.

### Checking Upstream Versions
But how do you know when to update a package? This is where the `UPST` and
`VCMD` fields come in very handy. 2 interprets these primarily for its
`--upstream` flag.

To check the upstream version of a package, execute `2 -U <repo>/<name>`. These
versions are typically acquired with a `git ls-remote` command chain.
