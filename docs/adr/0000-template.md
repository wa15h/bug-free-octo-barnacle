# NNNN: <title, the choice in a few words>

Copy to `docs/adr/NNNN-<slug>.md` and replace all guidance; `README.md` says who writes each section.

## Status
Proposed (the stub), Accepted (set by the owner before merge; an unmerged ADR decides nothing), or
Superseded by NNNN.

## Decision log ID
The `docs/plan/decisions.md` row this serves (for example D3, D7, D8, D17), or "none, rule 6 only".
Cite any open-question default the ADR relies on by number (D21).

## Context
What forces a choice now: the rule or decision, the queue item or PR that waits on it, and the
options considered. Plain sentences the founder can explain without the author (rule 9, D18).

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
