# Pull request template

Every PR description has these five sections, in this order. The reviewer agent fills
in or corrects them before the founder sees the PR. An agent's PR description opens with one
line above them: `Agent: <owner agent> (<queue ID>)` (D29).

## What this changes
One paragraph. One concern. If a second concern crept in, split the PR.

## Read line by line
The files and functions in `crates/sim`, `crates/proto`, `crates/persist`, or server
authority paths that the founder must read, with one sentence each on what to check.
Write "none" if the PR touches none of them.

## Accept on evidence
What the founder can accept without reading: the CI run, screenshots or video, metrics,
benchmark output. Link each piece of evidence.

## Rules touched
Which numbered engineering rules in `CLAUDE.md` this PR exercises, and how the hook or
test proves compliance.

## Risk and rollback
What could break, how we would notice, and how to revert.
