# Less

## Abstract
This page is really just me glazing ``less -R``.

### Listing
My preferred way to view packages is with the following command:

```bash
2 -l | less -R
```

Since no argument is passed to -l, the special set *@every* is implied,
displaying all my packages. This output is then piped through `less -R`, which
makes navigation easier while keeping any colored output.

### Logs & History
It's also useful for long package history or logs, especially in that you can
grep for what you need with less.

```bash
2 -L whois | less -R
2 -H whois | less -R
```
