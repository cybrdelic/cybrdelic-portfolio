# setup & configuration guide

commitaura is built to slide into your dev flow with minimal hassle. this guide gets you from zero to ai-powered commits in no time, with clear steps for installation and customization.

## system requirements

check your setup before diving in—this isn’t amateur hour. you’ll need:
- **git**: v2.24.0+
- **rust + cargo**: for building commitaura’s backend muscle.
- **anthropic api key**: your ticket to ai commit wizardry.
- **terminal**: zsh or bash for max compatibility.
- **internet**: bc ai needs to phone home sometimes.

## installation process

the setup process is frictionless and does all the heavy lifting for you.

### package installation

commitaura is distributed via cargo, so you know it’s legit.
run this and let it work:
```bash
cargo install commitaura
```

### git integration

this is where the magic happens—commitaura hooks directly into git. the integration:
- deploys custom git hooks for seamless commit workflows.
- sets up smart commit message templates.
- enables advanced diff analysis for context.
- aligns with your commit signing configs.

## configuration options

dial in commitaura to match your workflow.

### message formatting

you can tweak the style of generated messages via the config file:
- adjust format and structure.
- reorder sections (subject, body, refs).
- set char limits to keep messages concise.
- toggle emoji usage (bc 🔥 or nah).

### performance settings

keep it snappy with these tunables:
- **cache**: manage size + duration for repeated tasks.
- **timeouts**: avoid hanging on bad api calls.
- **concurrency**: control parallel ops for big repos.
- **rate limits**: throttle as needed for api safety.

### language preferences

get specific about how commitaura talks:
- output in your preferred language.
- enforce technical terminology consistency.
- follow your team’s doc style rules.
- add custom terms to the lexicon.

## post-installation verification

run these checks to confirm it’s ready to roll:
1. check the version:
   ```bash
   commitaura --version
   ```
2. validate api connectivity—it should hum.
3. test the git hook integration on a dummy repo.
4. generate a test commit message to flex its skills.

## next steps

once you’re set up, head over to the daily workflow guide. it’s got the full breakdown for:
- smooth commit workflows.
- message editing like a boss.
- handling batch changes.
- tying it into ci/cd bc automation = life.

get it configured, get it customized, and let it rip.
