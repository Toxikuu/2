# Special

## Abstract
Special flags are any flags that don't take packages as arguments. This page
explains what they do.

### List-repos
The list-repos flag (``-/`` or ``--list-repos``) lists the available repos in
``/var/ports``. It takes no arguments.

### List-sets
The list-sets flag (``-@`` or ``--list-sets``) lists the available sets for one
or more repos. The repo argument may end with '/', but it doesn't have to.

### Add-repos
The add-repos flag (``-a`` or ``--add-repos``) adds one or more repos. Under
the hood, this uses git to clone a remote repo into ``/var/ports/``. The
argument should be the link to the repo.

### Sync-repos
The sync-repos flag (``-s`` or ``--sync-repos``) syncs one or more repos. Under
the hood, this uses git to pull the latest changes. The argument should be a
repo, optionally ending with '/'.

### Provides
The provides flag (``-P`` or ``--provides``) shows which packages provide a
given path. This checks against package manifests to see which packages provide
a file path. It accepts one or more arguments, which are file paths.
