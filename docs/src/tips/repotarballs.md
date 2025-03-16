# Less

## Abstract
This page explains how to migrate repo tarballs to use git.

### Why?
There are a few reasons you might prefer tarballs to git for repos. The most
likely is that you installed 2 on a system without git. (For instance, I've
done this on a stage2 LFS system, then used 2 to install chapter 8.)

But then you might want to switch to using git so that you can fetch the latest
changes in the repos.

### Migrating
Navigate to the repo you want to migrate and initialize the git repo:
```
 $ git init
hint: Using 'master' as the name for the initial branch. This default branch name
hint: is subject to change. To configure the initial branch name to use in all
hint: of your new repositories, which will suppress this warning, call:
hint:
hint: 	git config --global init.defaultBranch <name>
hint:
hint: Names commonly chosen instead of 'master' are 'main', 'trunk' and
hint: 'development'. The just-created branch can be renamed via this command:
hint:
hint: 	git branch -m <name>
Initialized empty Git repository in /var/ports/tox/.git/
```

Then, add the upstream repo as a remote and fetch:
```
 $ git remote add origin https://github.com/Toxikuu/2-tox.git
 $ git fetch origin
remote: Enumerating objects: 420, done.
remote: Counting objects: 100% (420/420), done.
remote: Compressing objects: 100% (223/223), done.
remote: Total 420 (delta 218), reused 391 (delta 189), pack-reused 0 (from 0)
Receiving objects: 100% (420/420), 86.74 KiB | 70.00 KiB/s, done.
Resolving deltas: 100% (218/218), done.
From https://github.com/Toxikuu/2-tox
 * [new branch]      master     -> origin/master
```

Reset your working directory to match:
```
 $ git reset --hard origin
HEAD is now at 84df37e Logged d201559c4d7f51acfbd2d0e1576775b72e5a2b64
```

***Note**: The above command will reset any local changes you've made, but will
not reset any files that aren't tracked by upstream.*

Finally, make it work with 2 by setting a sane upstream:
```
 $ git branch --set-upstream-to=origin/master master
branch 'master' set up to track 'origin/master'.
```

To explain the above command, 2 just runs git pull. This lets you set a
different branch, for instance, origin/stable. If you don't set an upstream,
git pull will complain that you've not set any tracking information for the
current branch.
