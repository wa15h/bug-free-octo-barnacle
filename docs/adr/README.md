# Architecture decision records

One file per decision, `docs/adr/NNNN-<slug>.md`, copied from `0000-template.md`. Numbers have four
digits, go in order, and are never reused. An ADR records how a `decisions.md` row is carried out. It
never edits that file: when an outcome changes a decision, the founder writes the row (D24).

## When an ADR is required

- Rule 6, a new dependency: a package new to `cargo metadata` (dev- and build-dependencies included),
  a new `PackageReference`, an engine package or plugin, a third-party GitHub Action, or anything else
  a workflow or `tools/` script depends on. An art kit needs one only when it brings an importer
  plugin or a shader dependency (Q30 default, D21).
- D17 pins: choosing a pinned version (the Rust toolchain; the Steamworks SDK with every binding and
  the redistributable; the engine minor line), or moving one in a way its ADR did not already decide.
- Anything inside D3 (the server binary, SDR transport, the raw-UDP fallback), D7 (the wire format
  and C# codegen), or D8 (the sidecar process and its watchdog), even with no new dependency.

The ADR merges before the PR that needs it. When unsure, ask the producer in the item's Questions block.

## Who writes it

1. The producer queues the item and opens the ADR PR with the stub: Title, Status (Proposed),
   Decision log ID, and Context.
2. The owning engineer fills Decision, Consequences, Evidence, and Pinned versions touched in the
   same PR, and sets Status to Accepted. A stub never merges on its own.
3. A merged ADR changes only its Status line. A changed decision is a new ADR, and the old one's
   Status becomes "Superseded by NNNN" in the same PR.

## Reserved numbers

The week-0 queue (`docs/plan/week-0.md`) reserves 0001-0004. The next free number is 0005.

| Number | File | Queue item | Owner |
|---|---|---|---|
| 0001 | `0001-build-tooling-pins.md` | W0-04 | devex-engineer |
| 0002 | `0002-steamworks.md` | W0-05 | server-engineer |
| 0003 | `0003-sidecar-watchdog.md` | W0-09 | server-engineer |
| 0004 | `0004-wire-format.md` | W0-10 | net-engineer |
