# Changelog

## Abstract
This page explains `$PORT/CHANGELOG` format and usage.

### Generation
2lkit automatically logs changes to ports if used to make those changes.

### Commit Descriptions
If you'd like to add a description for (some[^1]) commits, start a line with '#d' at
the end of `$PORT/BUILD`.

### Format
Just look at the changelogs, they should be self-explanatory. You can either
`cat` them or run `2 -H <repo>/<name>` to view changelogs.

[^1]: Only actions affecting `$PORT/BUILD` can be given descriptions.
