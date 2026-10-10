# 0004: Protobuf via prost or FlatBuffers as the wire format

## Status
Proposed

## Decision log ID
D7 (a schema-owned, cross-language wire format: protobuf via prost by default, FlatBuffers if the
spike shows snapshot decode cost; C# types generated in CI from the schema `crates/proto` owns);
rule 6. Q14, the decode-cost threshold, has no written default; this PR pre-registers one.

## Context
MR 7 planned postcard, which HS 5 calls Rust-only. With a C# client (D4), HS 5 and 9.2 moved the
wire to a schema-driven format with C# generated from the one Rust-owned schema, and HS 5 made
generated C# falling out of lockstep with that schema its replacement top risk, held by a CI check
(regenerate, compile, version bump). D7 keeps prost unless snapshot decode cost shows; HS 5 gives no
number. Week-0 item 3 (`decisions.md`) and W0-10 measure it on the throwaway branch
`claude/spike-wire-format`, which never merges: a representative snapshot (two players, the boar
sounder, settlers, buildings, ledger lines, cordon stage; counts are parameters, guesses labelled)
encoded in Rust with each format and decoded in C# generated from each schema, 10,000 decodes a run,
median of five runs, at cut-scope size and at 4x. One table per format and size gives bytes, Rust
encode us, C# decode us, and C# bytes allocated per decode, and names the runner. This PR opens as a
draft: the owner's first commit after this stub pre-registers the C# decode threshold that flips D7,
and the founder approves or edits it in a PR comment before any result is pushed. No send rate is
decided (Q8), so a per-decode cost does not yet convert to a per-second one. This ADR merges before
any PR adding `prost`, `prost-build`, `prost-types`, `bytes`, or a FlatBuffers crate (rule 6);
week 1's `crates/proto` skeleton, codegen pipeline, and `client/lib` skeleton wait on it, and the
founder writes D7's outcome from it (W0-19, D24).

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
