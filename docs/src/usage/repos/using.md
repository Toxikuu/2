# Using Repos

## Abstract
Explanations and examples for repo usage.

### Explanation
Repos are another way of categorizing packages. I, for instance, decided to
split the 9 quintillion xorg-related packages into their own repo, ``xorg/``.
You're very much encouraged to make your own repos[^1], as well as to add other
repos (like ``tox/`` \[not biased\]).

The real appeal of personal repos is package customization. Two explicitly
avoids doing use-flag shenanigans or recursive configuration because that shit's
complicated. As a compromise, I'm encouraging people to make their own repos
with packages tailored to their systems. In a way this might still be better
than being practically forced to build every llvm target under the sun (even if
you only care about one of them \[cough, cough, gentoo\])

### Usage
You might recognize that I've been using repos since like the first chapter. If
you haven't -- damn, fuck you too I guess.

But yeah here's an example anyway:

#### List tox/
```
 $ 2 -l tox/
Packages:
  tox/catfetch=1.1.0                       ~ Installed 1.1.0
  tox/flameshot=12.1.0                     ~ Installed 12.1.0
  tox/kernel=6.13.9                        ~ Outdated (6.13.3)
  tox/llvm=20.1.2                          ~ Available
  tox/mesa=25.0.3                          ~ Outdated (25.0.1)
  tox/rust=1.85.1                          ~ Outdated (1.85.0)
  tox/shellcheck=0.10.0                    ~ Installed 0.10.0
  tox/st=0.9.2                             ~ Installed 0.9.2
  tox/sudo=1.9.16p2                        ~ Installed 1.9.16p2
```

[^1]: See [TODO] to learn how to make your own repos.
