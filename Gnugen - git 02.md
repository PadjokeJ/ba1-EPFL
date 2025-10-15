#gnugen 

## origin/main

remotes
```bash
git remote -v
```

You can add remotes

```bash
git remote add <name> <link>
```

You can have multiple remotes.

## Push/Pull

``push`` -> publish to remote
``pull`` -> get from remote

## Stash

```bash
git stash [push|pop]
```
allows to temporarily store changes when pulling

## Conflicts

Happens when two files have been changed differently.

## Branching

### rebase

merge into a branch

```bash 
git rebase <branch name>
```

==forcing the push using ``--push`` is necessary here==

you then accept the changes on the branch

```bash
git merge <branch name>
```

## Useful commands

- Add changes to last commit
```bash
git commit --ammend
```
- Restore a file from latest commit (discarding changes)
```bash
git restore
```
- Print history of commits
```bash
git log --graph --oneline
```
- Get differences between last commit
```bash
git diff <file name>
```

- jumping to commits
```bash
git checkout [<hash>|HEAD~<position>]
```

- tagging
```bash
git tag -a <tag> -m <message>
```
- you can switch to a branch with a tag name
```bash 
git checkout <tag>
```

>[!Warning] You have to push the tags manually using the tags flag

# Best practices

>[!Fail] DON'T
> - use ``push force``
> - work on the same branch as someone
> - work on main

>[!Check] DO
>- Small commits
>- Meaningful messages
>- Use many branches
>- Use merge/pull req
>- Add branch protections in github

