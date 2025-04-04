# Creating Repos

## Abstract
This page covers creating a new repo, both with and without 2lkit.

### With 2lkit
Let's create a new repo called 'my-awesome-creatively-named-repo'.
```
 $ 2lkit --new-repo <username>/my-awesome-creatively-named-repo
```

### Manually
Create an empty git repo on your preferred upstream host.

Then execute the following commands:
```
 $ cd /var/ports
 $ mkdir -pv my-awesome-creatively-named-repo
 $ cd my-awesome-creatively-named-repo
 $ git init
 $ git remote set-url origin https://github.com/<username>/my-awesome-creatively-named-repo.git
 $ echo '*/.*' > .gitignore
 $ echo '# my-awesome-creatively-named-repo' > README.md
 $ wget 'https://www.gnu.org/licenses/gpl-3.0.txt' -O LICENSE
 $ git push
```

***Note:** Feel free to use ssh for your remote origin if you like.*
