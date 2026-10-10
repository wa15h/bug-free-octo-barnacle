# 0001: Rust toolchain and CI action pins

## Status
Proposed

## Decision log ID
D17, and rule 6 for each GitHub Action. Relies on Q24's default (D21): the toolchain moves only when
a pinned dependency needs it, in a pin-only PR (rule 8).

## Context
W0-06 writes the toolchain into `rust-toolchain.toml` and `tools/pins.toml`, and W0-07's `ci.yml`
and W0-17's `pr-gates.yml` call GitHub Actions, which rule 6 counts as dependencies (`README.md`);
this ADR merges before all three. D17 allows one toolchain; HS 5 budgets toolchain, SDK, and engine
upgrades at about 8% of build hours but names no Rust cadence, so Q24's default sets it. A floating
name (the `stable` channel, an Action tag like `@v4`) can change what CI runs without a PR; an exact
`x.y.z` or a full commit SHA moves only in an upgrade PR. W0-04 asks for the second: current stable
on the PR date, each Action at a full SHA with its maintainer. Steamworks pins are ADR 0002; client
pins come in week 1.

## Decision
The choice in one paragraph. For a dependency: the package name exactly as the manifest writes it,
the exact version, the maintainer, the license, and which crates or projects may use it.

## Consequences
What becomes easier, what becomes harder or forbidden, and any follow-up item for the queue.

## Evidence
Links to the runs, logs, benchmarks, or upstream pages the decision rests on, with the commit SHA or
version each one used. Evidence described but not linked is missing.

## Pinned versions touched
Each pin this ADR sets (D17) and its current value, or "none". An upgrade PR (rule 8) edits this ADR
in place: the new value and a dated line saying why. A changed choice is a new, superseding ADR.
