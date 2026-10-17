# Architecture decision records

One file per decision, `docs/adr/NNNN-<slug>.md`, copied from `0000-template.md`; numbers run in
order and are never reused. An ADR records a dependency choice (rule 6) or how a `decisions.md` row
is carried out, and never edits that file: a changed decision is a row the founder writes (D24).

## When an ADR is required

- Rule 6, a new dependency: a package new to `cargo metadata` (dev- and build-dependencies included),
  a new `PackageReference`, an engine package or plugin, a third-party GitHub Action, or anything else
  a workflow or `tools/` script depends on; an art kit only when it brings an importer plugin or a
  shader dependency (Q30 default, D21).
- D17 pins: choosing a pinned version (the Rust toolchain; the Steamworks SDK with every binding and
  the redistributable; the engine minor line; one FlatBuffers version for `flatc`, the `flatbuffers`
  crate, and `Google.FlatBuffers`, by D30). An upgrade PR edits that ADR in place (step 3 below).
- Inside D3, D7, D8, or D30: a design choice of how the area works, such as the runtime setup or the
  snapshot layout, even with no new dependency (founder, 2026-10-10). D3 is the tokio server binary,
  SDR transport, the raw-UDP fallback, and the engine-free sim, proto, net, and persist crates; D7
  the Rust-owned schema and C# codegen; D8 the sidecar process and its watchdog; D30 the wire format,
  FlatBuffers, which replaced D7's format clause. Routine work in them with
  no new dependency needs none: W0-02's empty crates, the sim skeleton, and work carrying out a merged
  spike ADR (`SIDECAR.md` under 0003, the proto skeleton under 0004). Q10 asks if `Fixed` needs one.
- The ant-farm viewer in `crates/tools`, even with no dependency (test-engineer, sim-engineer files).

The ADR merges before the PR that needs it. When unsure, ask the producer in the Questions block.

## Who writes it

1. The producer opens the ADR PR with the stub: Title, Status (Proposed), Decision log ID, Context.
2. The owning engineer fills Decision, Consequences, Evidence, and Pinned versions touched in the
   same PR, and sets Status to Accepted. A stub never merges on its own.
3. An upgrade PR (rule 8) edits the pin's ADR in place: the new version and a dated line saying why.
   A changed choice is a new ADR; the old one's Status becomes "Superseded by NNNN" in that PR.

## Numbers reserved by the queues, `docs/plan/week-0.md` and `docs/plan/queue.md` (next free: 0015)

| Number | File | Queue item | Owner |
|---|---|---|---|
| 0001 | `0001-build-tooling-pins.md` | W0-04 | devex-engineer |
| 0002 | `0002-steamworks.md` | W0-05 | server-engineer |
| 0003 | `0003-sidecar-watchdog.md` | W0-09 | server-engineer |
| 0004 | `0004-wire-format.md` | W0-10 | net-engineer |
| 0005 | `0005-proptest.md` | W1-04 | sim-engineer |
| 0006 | `0006-serde.md` | W1-05 | sim-engineer |
| 0007 | `0007-settlement-viewer.md` | W1-11 | test-engineer |
| 0008 | `0008-save-encoding.md` | N-16 | persistence-engineer |
| 0009 | `0009-tokio.md` | N-11 | server-engineer |
| 0010 | `0010-dotnet-sdk.md` | N-19 | devex-engineer |
| 0011 | `0011-godot-pin.md` | N-24 | devex-engineer |
| 0012 | `0012-unity-pin.md` | N-26 | devex-engineer |
| 0013 | `0013-dotnet-tests.md` | N-20 | client-engineer |
| 0014 | `0014-rawudp-transport.md` | N-54 | server-engineer |
