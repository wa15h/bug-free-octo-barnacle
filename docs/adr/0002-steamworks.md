# 0002: steamworks 0.13.1 on Steamworks SDK 1.64

## Status
Accepted

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
`crates/server` alone may depend on `steamworks = "=0.13.1"` (crates.io; MIT or Apache-2.0; maintainer
Aaro Perämaa, `Noxime`); W0-08 adds it. SDK 1.64 comes vendored inside `steamworks-sys` 0.13.0.
0.13.1 accepts any `steamworks-sys` 0.13.x, so `Cargo.lock` holds 0.13.0, and `STEAM_SDK_LOCATION`
stays unset (it swaps in another SDK). The server and the client ship the vendored files hashed below.

## Consequences
- W0-06 copies the pins below into `tools/pins.toml`. `pins.sh` fails a second `steamworks` or
  `steamworks-sys`, either one off its pin, or another redistributable hash. A pin-only PR moves them.
- Open pin task, client engineer: the C# binding's SDK 1.65 (Context); a pin-only PR closes it.

## Evidence
- https://index.crates.io/st/ea/steamworks: 0.13.1 (`5ff29921`) needs `steamworks-sys ^0.13.0`, and
  https://index.crates.io/st/ea/steamworks-sys has one 0.13.x, 0.13.0 (`ae139f05`). License, owner,
  SDK table, `steamworks-sys/build.rs`: https://github.com/Noxime/steamworks-rs/tree/bfc8210 (0.13.1).
  Steamworks.NET on SDK 1.65: https://github.com/rlabrecque/Steamworks.NET/tree/ba71581 (README).

## Pinned versions touched
`steamworks-sdk` 1.64, `steamworks` 0.13.1, `steamworks-sys` 0.13.0. Redistributable SHA-256s:
`win64/steam_api64.dll` `eb17909a76668cf9ae0b92a618a34a50f6c73d3a6787cb4dd8ce36a8b10bfb75`,
`linux64/libsteam_api.so` `ec4797f76a206eb0af627af0f0788eb5a2eabf5dee38ee0a6affbaa44a645f4e`.
