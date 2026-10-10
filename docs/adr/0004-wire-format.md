# 0004: FlatBuffers as the wire format

## Status
Accepted

## Decision log ID
D30 (founder, 2026-10-10; supersedes D7, answers Q14); rule 6; D17 for the pins. Q8 stays open.

## Context
D7 kept protobuf via prost unless snapshot decode cost showed; this ADR's draft pre-registered a C#
threshold to decide it. D30 picks FlatBuffers now and drops the comparison and the threshold. The
spike still measures FlatBuffers decode cost, so this ADR has real numbers and the C# codegen path is
proven. It merges before any PR adding `flatbuffers` or `Google.FlatBuffers` (rule 6); week 1's `crates/proto`, codegen, and `client/lib` skeletons wait on it.

## Measurement
- Snapshot (guesses): 2 players, 8 boars, 6 settlers, 20 buildings, 12 ledger lines, 1 cordon stage
  (49); 4x quadruples all but players and stage (187) (D2, D26). Each is one table: `uint32` id 1 to
  n, five `int64` fields uniform over `i32` from seed 1; one id-sorted vector (id order, as net's `Snapshot.entities`; no `server_tick` or `hearth`).
- Per snapshot, median of five runs of 10,000 after 1,000 warm-up: Rust encode us; C# `t`, decode us
  (`Stopwatch`), and `a`, bytes allocated (`GC.GetAllocatedBytesForCurrentThread()`). A decode wraps
  one `byte[]` and reads every field into Rust's checksum, in flatc's `netstandard2.1` decoder run by a Release `net8.0` console.
  Limit: that console loads `Google.FlatBuffers`' `lib/net8.0` build (a `dotnet restore` of the spike projects, PR #15), not the `netstandard2.1` build Unity would load.

## Decision
FlatBuffers, by D30, at one version, `25.2.10` (D17), from the google/flatbuffers project (Google
LLC), Apache-2.0, tag `v25.2.10` at commit `1c514626e83c20fffa8557e75641848e1e15cd5e`:
- `flatbuffers = "=25.2.10"` (crates.io owners CasperN, rw, dbaileychess, team `github:flatbuffers:rust-publish`) in `crates/proto` only; `net` uses
  `proto`'s re-export. It brings `bitflags` 2.13.2 (crates.io owners KodrAus, rust-lang-owner) and, build-only, `rustc_version` 0.4.1 (Kimundi, djc) and `semver` 1.0.28 (dtolnay) (spike lock), each MIT OR Apache-2.0, used only through `proto`'s `flatbuffers`.
- `flatc` 25.2.10 (`Linux.flatc.binary.g++-13.zip`, `Windows.flatc.binary.zip`) only in `crates/proto`'s build and tests (in CI and on clones) and in `tools/codegen/gen-csharp.sh`.
- `Google.FlatBuffers` `[25.2.10]` (NuGet owner `google`, authors Google LLC) in `client/lib` only. Not used:
  `prost`, `prost-build`, `prost-types`, `bytes`. Another FlatBuffers library is a new ADR.

## Consequences
- `crates/proto`: the schema is `schema/*.fbs`; `build.rs` runs flatc (failing unless it reports
  25.2.10) into `OUT_DIR`; nothing generated is committed. Each shipped version freezes its `.bfbs` in
  `fixtures/v<N>/`, replacing protobuf's `descriptor.bin`; `descriptor_is_compatible` runs `flatc --conform`
  on it. Evolution: append a field at its table's end or mark it `(deprecated)`, keeping its slot;
  never remove, reorder, or retype one. The spike run shows `--conform` rejecting a removal and a retype.
- Codegen: CI installs flatc from its OS's zip by SHA-256 and generates C# into `client/lib/Generated/`
  (gitignored) for the lockstep job. Follow-ups: devex, that install on both CI runners from the zips pinned below, and the same flatc on PATH on clones (with `tools/codegen/gen-csharp.sh`) so `cargo build` and `cargo test --workspace` run there (rule 7);
  client engineer, `packages.lock.json` if the founder wants a NuGet hash mirror (its contentHash, `z3ykbSRsy6oqMzeZ84a+K8rnMJ/ysVikar8r2yUhHz0HSxIjBpj53mZvpu6LjLEm5fLLWQsCdelJwL1nu7lloA==`, leaves out the signature); reviewer, `review-log.md` 1.3 says deprecated, not reserved; producer, `docs/adr/README.md` names D30 for the wire format.
- `client/lib`: accessors are structs over the received `byte[]`; a decode allocated only the `ByteBuffer` and its allocator (72 bytes).

## Evidence
Run [38065990593](https://github.com/wa15h/bug-free-octo-barnacle/actions/runs/38065990593) on `claude/spike-wire-format` (never merged) head [`e43f01ba182a1e614fa96f303f30f9f19ae0629d`](https://github.com/wa15h/bug-free-octo-barnacle/tree/e43f01ba182a1e614fa96f303f30f9f19ae0629d/spike/wire-format): rustc 1.99.0, .NET SDK 10.0.401 running .NET 8.0.31, flatc 25.2.10.

| Runner | Size | Entities | Bytes | Rust encode us | C# decode us `t` | C# bytes allocated `a` |
|---|---|---|---|---|---|---|
| `ubuntu-24.04` 20261004.327.1, AMD EPYC 9V74, 4 vCPU | cut scope | 49 | 2,592 | 2.0 | 3.6 | 72 |
| same job | 4x | 187 | 9,768 | 7.6 | 13.5 | 72 |

Nothing gates on these (D30). The draft's limits, for reference only and no longer binding: `t` 250 us, `a` 16,384 bytes.
Upstream: [crate](https://crates.io/crates/flatbuffers/25.2.10) and its dependencies [bitflags](https://crates.io/crates/bitflags/2.13.2), [rustc_version](https://crates.io/crates/rustc_version/0.4.1), [semver](https://crates.io/crates/semver/1.0.28) (line 28's owners and licences), [NuGet](https://www.nuget.org/packages/Google.FlatBuffers/25.2.10) (its nuspec names the tag commit), [release](https://github.com/google/flatbuffers/releases/tag/v25.2.10), [licence](https://github.com/google/flatbuffers/blob/v25.2.10/LICENSE).

## Pinned versions touched
- `flatc` = `25.2.10`, set 2026-10-10: one pin, the one generator row in `tools/pins.toml` (D17). It moves only in a pin-only PR (rule 8) that moves every mirror and hash below with it.
- Mirror, `Cargo.lock`: `flatbuffers` 25.2.10, crates.io SHA-256 `1045398c1bfd89168b5fd3f1fc11f6e70b34f6f66300c87d44d3de849463abf1`.
- Hashes, held in `tools/pins.toml` beside the `flatc` row as the redistributable's are, and checked by the CI install: the flatc zips by SHA-256 (the release publishes no digest, so these hash the bytes received on 2026-10-10): [`Linux.flatc.binary.g++-13.zip`](https://github.com/google/flatbuffers/releases/download/v25.2.10/Linux.flatc.binary.g++-13.zip) `6f01258d7475806f375d6da66a61df47add8016edd73f1774673f37b80b9a711` (checked by run 38065990593, `flatc.zip: OK`); [`Windows.flatc.binary.zip`](https://github.com/google/flatbuffers/releases/download/v25.2.10/Windows.flatc.binary.zip) `bd9b3cb6bf5ab8009f1927b6e00207db968c3d5c93e3bc215dae084cc681249a` (downloaded 2026-10-10; no run checks it until the devex Windows CI install).
- Mirror, the `.csproj`: `Google.FlatBuffers` `[25.2.10]`, the version only (a `PackageReference` holds no hash). For reference, not a mirror: the nuget.org [catalog](https://api.nuget.org/v3/catalog0/data/2025.02.11.04.33.13/google.flatbuffers.25.2.10.json) SHA-512 of the signed package, `+VZF2UrDEVsImRwzS2owEQ79JivOtw78SzLwYaoTr1BvWBUNanlOh7Yp1ozotZdWp+BcbxeG4OXWH659cwcODA==`.
