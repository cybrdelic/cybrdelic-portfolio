# commitaura daily workflow guide

this is your go-to playbook for integrating commitaura into your dev loop, leveling up your commit messages with machine precision while keeping your flow smooth af. think of it as your commit-sidekick—context-aware, productivity-friendly, and always on-point.

## understanding the basics

commitaura doesn’t disrupt your git hustle; it enhances it. by analyzing your staged changes, it autogenerates commit messages that actually make sense. no ctrl+c ctrl+v vibes here—this tool plugs seamlessly into your existing workflow, acting like the coder-brain-extension you didn’t know you needed.

## standard development flow

### 1. making changes

you’re the dev, so do what you do best:

1. hammer out your changes in the codebase. keep it cracked, keep it clean.
2. when your changes slap, stage them like normal:
   ```bash
   git add .
   ```
3. sanity-check your stage before invoking the magic:
   ```bash
   git status
   ```

### 2. generating messages

now you let commitaura cook. launch it like this:

```bash
commitaura
```

here’s what goes down behind the scenes:
1. it parses your changes with a precision scalpel.
2. it crafts contextual commit messages that don’t make you cringe.
3. it previews these bangers for you to approve or tweak.

### 3. review and confirmation

this is where you sanity-check the ai’s work:
- read the message like you mean it.
- if it hits, accept it. if it’s mid, request another spin.
- tweak manually if you’re chasing perfection.
- lock it in when you’re ready.

### 4. completing the commit

done reviewing? execute:
- confirm the commit.
- optionally push to your remote with:
   ```bash
   git push
   ```
- rinse and repeat for the rest of your dev session.

## best practices

### managing changes effectively

commit kung fu 101:
1. group related changes—don’t be sloppy.
2. stage in meaningful units (bc context matters).
3. double-check your stage with `git diff`.
4. verify that commitaura’s output doesn’t miss nuance.
5. scope commits to atomic, readable chunks.

### ensuring quality

protect the integrity of your repo:
1. validate commit messages like they’re legal contracts.
2. confirm the technical accuracy—no lies allowed.
3. check ticket refs and cross-links.
4. describe changes with clarity and confidence.
5. make sure every commit adds actual value.

that’s it. go build something sick.
