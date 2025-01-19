# Less

## Abstract
Less works quite well with 2. This chapter contains a few tips for using them
together.

### Listing
My preferred way to view packages is with the following command:

```bash
2 -l | less -R
```

Since no argument is passed to -l, the special set *@every* is implied,
displaying all my packages. This output is then piped through `less -R`, which
makes navigation easier.

### Building
The same applies for building, though I use this only for debugging:

```bash
2 -fbi whois | less -R
```

Here, I'm forcibly building and installing whois. I can then page through the
build output in less, easily observing stderr, packaged files, etc.

### History
If a package has a particularly long history, it can be convenient to page through that as well:

```bash
2 -H whois | less -R
```

Here, I can view the history of whois.
[//]: # TODO: Implement history.
