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

### Logs & History
Similarly, you can view logs and history. Basically, just use `less -R` with
most colored output :)

```bash
2 -L whois | less -R
2 -H whois | less -R
```

[//]: # TODO: Implement history.
