# 0002: steamworks 0.13.1 on Steamworks SDK 1.64

## Status
Proposed

## Decision log ID
D3 (Steam Datagram Relay through steamworks-rs GameServer sockets) and D17 (one Steamworks SDK version
across every binding and the redistributable); rule 6. No open-question default is relied on.

## Context
Rule 6: this merges before W0-08, the SDR spike on app 480 that first adds `steamworks` and whose
result is D3's outcome; W0-06's pin file waits on it too. HS 5 names `steamworks` 0.13.1, built on
SDK 1.64; it runs a GameServer with no Steam client (`Server::init`, `create_listen_socket_p2p`, since
0.12.2, MR 3.7). The C# binding (Steamworks.NET or Facepunch.Steamworks, not chosen) is on 1.65 per
Steamworks.NET's README, so D17's one pin is a real task (HS 5, 9.4): the C# side moves to 1.64, or
the Rust side to 1.65 once a `steamworks` release builds on it. Two `steamworks` versions can share
one `Cargo.lock` with no link error, only mismatched types (MR 3.7), so one version must be checked.

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
