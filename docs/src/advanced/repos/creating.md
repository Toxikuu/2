# Creating Repos

## Abstract
This page covers creating a new repo, both with and without 2lkit. Repos must
start with '2-' to be recognized by 2, but their directories should exclude
'2-'.

### With 2lkit
Let's create a new repo called '2-my-awesome-creatively-named-repo'.
```
 $ 2lkit --new-repo https://github.com/<username>/2-my-awesome-creatively-named-repo.git
```

### Manually
Create an empty git repo on your preferred upstream host.

Then execute the following commands:
```
 $ cd /var/ports
 $ mkdir -pv my-awesome-creatively-named-repo
 $ cd my-awesome-creatively-named-repo

 $ git init
 $ echo '*/.*' > .gitignore
 $ echo 'shell=bash' > .shellcheckrc
 $ echo 'disable=SC2034' >> .shellcheckrc
 $ echo '# my-awesome-creatively-named-repo' > README.md
 $ curl -L 'https://www.gnu.org/licenses/gpl-3.0.txt' -o LICENSE

 $ git add .
 $ git commit -m "Initial commit"
 $ git branch -M master
 $ git remote add origin https://github.com/<username>/2-my-awesome-creatively-named-repo.git
 $ git push -u origin master
```

***Note:** Feel free to use ssh for your remote if you like.*
