# 0001: Rust toolchain and CI action pins

## Status
Accepted

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
Rust `1.99.0` from the Rust project (MIT or Apache-2.0), with `rustfmt` and `clippy`, profile `minimal`,
for every crate, installed by each runner's own rustup from `rust-toolchain.toml`. Two Actions by GitHub
(MIT): `actions/checkout` `v7.0.1` in `ci.yml` and `pr-gates.yml` (every job needs the code, `pr-size.sh`
the merge base) and `actions/cache` `v6.1.0` in `ci.yml` (cargo registry, `target/`); `pr-gates.yml` labels
with the runner's `gh`. Rejected: `dtolnay/rust-toolchain`, `Swatinem/rust-cache`, third-party and redundant.

## Consequences
Both CI runners and every clone use one compiler, rustfmt, and clippy (the images alone ship 1.99.0 and
1.98.1). A move is a pin-only PR editing this file. A later NuGet cache reuses `actions/cache`; any new
Action needs its ADR first (rule 6). Not pinned: the `-latest` images, with their rustup and `gh`.

## Evidence
[Stable manifest](https://static.rust-lang.org/dist/channel-rust-1.99.0.toml), equal to `channel-rust-stable.toml` on 2026-10-10: `date = "2026-10-01"`, `rust 1.99.0`.
Runner images at `8197087`: [Ubuntu 24.04](https://github.com/actions/runner-images/blob/8197087fc536320d1441203fdb5da9ae1b44b863/images/ubuntu/Ubuntu2404-Readme.md) (`ubuntu-latest`) Rust 1.99.0; [Windows 2025 VS2026](https://github.com/actions/runner-images/blob/8197087fc536320d1441203fdb5da9ae1b44b863/images/windows/Windows2025-VS2026-Readme.md) (`windows-latest`, per the [README](https://github.com/actions/runner-images/blob/8197087fc536320d1441203fdb5da9ae1b44b863/README.md)) Rust 1.98.1; rustup 1.29.1 and `gh` on both.

## Pinned versions touched
- `rust` = `1.99.0`, set 2026-10-10.
- `actions/checkout` = [`3d3c42e5aac5ba805825da76410c181273ba90b1`](https://github.com/actions/checkout/commit/3d3c42e5aac5ba805825da76410c181273ba90b1), lightweight tag `v7.0.1` (`git ls-remote`), [MIT](https://github.com/actions/checkout/blob/3d3c42e5aac5ba805825da76410c181273ba90b1/LICENSE).
- `actions/cache` = [`55cc8345863c7cc4c66a329aec7e433d2d1c52a9`](https://github.com/actions/cache/commit/55cc8345863c7cc4c66a329aec7e433d2d1c52a9), lightweight tag `v6.1.0` (`git ls-remote`), [MIT](https://github.com/actions/cache/blob/55cc8345863c7cc4c66a329aec7e433d2d1c52a9/LICENSE).
