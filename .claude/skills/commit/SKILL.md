---
name: commit
description: Commit the current changes onto a feature/bug/task branch, push it and open a PR into main, following this repo's conventions (one-line `HQ-NNN` subject, no AI attribution). Direct pushes to main are not used. Use when the user says "commit", "commit and push", "open a PR / MR" or invokes /commit.
---

# Commit & PR

Delegates the git work to the `committer` subagent (`.claude/agents/committer.md`), which runs on a cheaper model: branching, committing, pushing and opening the PR is mechanical.

## Steps

1. Write a 1-4 sentence summary of what changed in this session and *why*. The `committer` agent starts with no memory of this conversation and uses it for the PR description.
2. Invoke the `committer` subagent with the Agent tool (`subagent_type: committer`, `run_in_background: false`) and that summary as the prompt.
3. Relay its result to the user: commit hash(es) + subject, and the PR URL.

Don't do the git steps inline. The conventions live in `committer.md`; don't duplicate them here.
