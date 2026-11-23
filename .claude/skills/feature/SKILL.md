---
name: feature
description: Start or finish a feature branch that groups several task/bug PRs into one larger change. `/feature start <slug>` branches feature/<slug> off main; `/feature finish` opens the feature → main PR. Use when the user says "start a feature", "finish the feature", "merge the feature into main" or invokes /feature.
---

# Feature

A feature (`feature/<slug>`) groups several `task/` and `bug/` branches into one logical change. Task PRs target the feature branch and run only the fast CI checks; the feature → `main` PR also runs the per-OS build check. Branch flow and merge methods are in `CLAUDE.md` (Git).

## `/feature start <slug>`

1. Working tree must be clean and on `main`; otherwise stop and say so.
2. `git pull`, then `git checkout -b feature/<slug>` and `git push -u origin feature/<slug>`.
3. Check `.tasks/plan.md` has a `### <Name> (`feature/<slug>`)` section. If not, tell the user so the plan can be updated in the first task.
4. Tell the user they're on `feature/<slug>`; the next `/commit` creates a task branch off it.

## `/feature finish`

1. Must be on `feature/<slug>` with a clean tree, and no open PRs whose base is this feature (`gh pr list --base feature/<slug>`). Otherwise stop and list what's left.
2. `git pull`, then `git fetch origin main` and `git merge origin/main`. On conflicts, stop and report them; don't resolve them yourself.
3. `git push`.
4. `gh pr create --base main --head feature/<slug>`:
   - Title: `<Feature name>` in a few words, e.g. `Routines`.
   - Body: 1-3 sentences on what the feature delivers, then the list of its commits from `git log --format='- %s' origin/main..HEAD --no-merges`. No AI attribution.
5. `git checkout main`, report the PR URL, and remind the user to merge it with a **merge commit** (not squash), so `main` keeps the individual `HQ-NNN` commits.
