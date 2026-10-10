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

## Pre-registered rule (Q14)
- Snapshot, guesses fixed now so they cannot be tuned: 2 players, 8 boars, 6 settlers, 20 buildings,
  12 ledger lines, one cordon stage; 4x multiplies every count; both schemas carry the same integers.
- Measures, per format and size: a decode turns one encoded `byte[]` into every field of every
  entity, read once into a checksum that must match across formats. A run is 1,000 warm-up decodes,
  then 10,000 timed by `Stopwatch` and counted by `GC.GetAllocatedBytesForCurrentThread()`: `t` is C#
  decode us per snapshot (elapsed / 10,000) and `a` is C# bytes allocated per decode (bytes / 10,000),
  each the median of five runs, printed to 0.1 us and whole bytes. A crash or a checksum mismatch
  voids the job; a rerun replaces the whole table. Wire bytes and Rust encode us decide nothing.
- `t` at most 250 us. Valheim runs about 60 fps on a Deck at low vegetation (setting review, "Why
  the restraint"; D15 targets the Deck), a 16.7 ms frame. Q8 sets no send rate, so assume one
  snapshot a frame, the most a client can show. Decode gets 3% of the frame, 0.5 ms (a judgment, the
  founder's to set), halved for a non-Deck runner on .NET, not Unity's runtime (Q22's 2x headroom).
- `a` at most 16,384 bytes: at one decode a frame, under 1 MiB a second of garbage for the engine's
  collector (a judgment, the founder's to set). Bytes do not depend on the machine, so no halving.
- Pick on the printed medians; a median equal to its limit is within. Prost within both limits at
  both sizes: protobuf via prost (D7's default) whatever FlatBuffers measures, so a tie (both within)
  keeps prost. Prost over any limit and FlatBuffers within all: FlatBuffers. Both over a limit
  somewhere: prost stays, since switching alone misses the budget, and the table goes to the
  producer as a snapshot-size problem.
- Runner: GitHub-hosted `ubuntu-24.04` x64, one job in the spike branch's own workflow (as W0-09's),
  formats alternating run by run; the C# decode in a `netstandard2.1` library (as `client/lib`, D4)
  called by a Release `net8.0` console. The log prints `lscpu`, `rustc -V`, and `dotnet --info`; the
  table names the runner and its CPU and links the run.

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
