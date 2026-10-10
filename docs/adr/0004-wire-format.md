# 0004: Protobuf via prost or FlatBuffers as the wire format

## Status
Proposed

## Decision log ID
D7 (protobuf via prost by default, FlatBuffers if the spike shows snapshot decode cost; C# generated
in CI from the schema `crates/proto` owns); rule 6. Q14 has no written default (set below); Q8.

## Context
MR 7 planned postcard, which HS 5 calls Rust-only. With a C# client (D4), HS 5 and 9.2 moved the
wire to a schema-driven format, C# generated from the one Rust-owned schema; HS 5's new top risk is
the two drifting apart, held by a CI check. D7 keeps prost unless snapshot decode cost shows; HS 5
gives no number. Week-0 item 3 measures it on `claude/spike-wire-format`, a branch that never
merges, by the rule below, which the founder approves or edits in a PR comment before any result is
pushed. This ADR merges before any PR adding `prost`, `prost-build`, `prost-types`, `bytes`, or a
FlatBuffers crate (rule 6); week 1's `crates/proto`, codegen, and `client/lib` skeletons wait on it;
the founder writes D7's outcome from it (W0-19, D24).

## Pre-registered rule (Q14)
- Snapshot (guesses, fixed now): 2 players, 8 boars, 6 settlers, 20 buildings, 12 ledger lines, 1
  cordon stage; 4x quadruples all but players and cordon stage (D2, D26). Each is one flat message
  (table) of a `uint32` id and five `sint64` (`long`) fields, the same in both schemas.
- Measured in C#: `t`, decode us per snapshot, and `a`, bytes allocated per decode, over 10,000
  decodes after 1,000 warm-up, each reading every field of one `byte[]` into a checksum equal across
  formats; `t` from `Stopwatch`, `a` from `GC.GetAllocatedBytesForCurrentThread()`.
- `t` at most 250 us. A Deck frame is 16.7 ms (Valheim runs about 60 fps there at low vegetation:
  setting review, "Why the restraint"; D15). Q8 sets no send rate, so one snapshot a frame; decode
  gets 3% (0.5 ms), halved for the runner not being a Deck and .NET 8 not being Unity 6's runtime.
- `a` at most 16,384 bytes: one decode a frame then makes under 1 MiB of garbage a second. CPU speed
  does not change allocation, so no halving. 3%, the halving, and 1 MiB are judgments.
- Pick on the medians of five runs, printed to 0.1 us and whole bytes; nothing else counts. A
  format passes a size when both its medians are at or under their limits. The format passing more
  of the two sizes wins; an equal count (2-2, 1-1, 0-0) keeps prost, D7's default.
- Runner: one GitHub-hosted `ubuntu-24.04` x64 job, formats alternating by run; a `netstandard2.1`
  decoder (D4) in a Release `net8.0` console; the log prints `lscpu`, `rustc -V`, `dotnet --info`.
  The first spike-branch job not voided (crash, checksum mismatch) decides; only voided jobs rerun.

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
