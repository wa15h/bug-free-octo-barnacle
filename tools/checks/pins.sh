#!/usr/bin/env bash
# D17 gate; the pre-push hook (W0-16) and CI run this same file. Each mirror must equal tools/pins.toml:
# rust-toolchain.toml (no legacy rust-toolchain); Cargo.lock (one steamworks and one steamworks-sys, each from
# crates.io; `cargo tree -d --depth 0` lists only duplicates); the vendored redistributable SHA-256s, STEAM_SDK_LOCATION
# unset; every `uses:` under .github/workflows and .github/actions. CRLF reads as LF. The first loop reads every row
# before any check runs, so a missing row fails now. A check with nothing to check yet says so, names the item that
# makes it bite, and passes. The first failure exits 1: `RULE D17: <what> -- <fix>`.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
P=tools/pins.toml; pins=$(tr -d '\r' < "$P" || true)
fail() { echo "RULE D17: $1 -- $2" >&2; exit 1; }
say() { echo "pins.sh: $1"; }
pin() { v=$(echo "$pins" | sed -n "s|^\"\{0,1\}$1\"\{0,1\} *= *\"\([^\"]*\)\".*|\1|p")
  [ -n "$v" ] || fail "$P has no $1 row${2:+, used at $2}" "add the row its ADR names first (rule 6)"; echo "$v"; }

bad=$(echo "$pins" | grep -nE '^[^#].*=' | grep -vE '= "([0-9]+(\.[0-9]+){1,2}|[0-9a-f]{40}|[0-9a-f]{64}|pending)"( *#.*)?$' || true)
[ -z "$bad" ] || fail "$P line ${bad%%:*} is not an exact version, full SHA, or \"pending\"" "write one exact value"
rels=(win64/steam_api64.dll linux64/libsteam_api.so)
for r in rust actions/checkout actions/cache steamworks-sdk steamworks-rs steamworks-sys "${rels[@]/#/redist/}" csharp-binding codegen; do pin "$r" >/dev/null; done
rust=$(pin rust)
[ ! -e rust-toolchain ] || fail "rust-toolchain exists, and rustup reads it before rust-toolchain.toml" "delete rust-toolchain; rust-toolchain.toml is the one mirror (ADR 0001)"
ch=$(sed -n 's/^channel *= *"\([^"]*\)".*/\1/p' rust-toolchain.toml 2>/dev/null || true)
echo "$rust" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$' || fail "$P rust \"$rust\" is not x.y.z" "pin the exact toolchain ADR 0001 names"
[ "$ch" = "$rust" ] || fail "rust-toolchain.toml channel \"$ch\" differs from $P rust \"$rust\"" "make both the ADR 0001 version in one pin-only PR (rule 8)"
say "ok: rust-toolchain.toml channel $ch equals $P rust"

tree=$(cargo tree -d --depth 0 --all-features --target all --locked --prefix none 2>&1) || fail "cargo tree -d --locked failed: $(grep -m1 '^error' <<< "$tree" || tail -n1 <<< "$tree")" "fix Cargo.lock, then rerun"
for name in steamworks steamworks-sys; do
  if [ "$name" = steamworks ]; then row=steamworks-rs; want=$(pin "$row"); fix="depend on steamworks = \"=$want\" in crates/server alone (ADR 0002)"
  else row=$name; want=$(pin "$row"); fix="let only crates/server's steamworks bring it, then cargo update -p steamworks-sys --precise $want (ADR 0002)"; fi
  dup=$(echo "$tree" | sed -n "s/^$name \(v[^ ]*\).*/\1/p" | sort -u | paste -sd' ' -)
  [ -z "$dup" ] || fail "cargo tree -d shows more than one $name in Cargo.lock: $dup" "$fix"
  have=$(grep -A1 "^name = \"$name\"" Cargo.lock | sed -n 's/^version = "\([^"]*\)".*/\1/p' | paste -sd' ' - || true)
  if [ -z "$have" ]; then say "skip: no $name in Cargo.lock yet; bites when W0-08 adds steamworks to crates/server"; continue; fi
  [ "$have" = "$want" ] || fail "Cargo.lock has $name $have, $P $row is $want" "$fix"
  src=$(grep -A2 "^name = \"$name\"" Cargo.lock | sed -n 's/^source = "\([^"]*\)".*/\1/p' || true)
  [ "$src" = registry+https://github.com/rust-lang/crates.io-index ] || fail "Cargo.lock takes $name from ${src:-a path}, not crates.io" "take $name from crates.io: no [patch], git, or path source (ADR 0002)"
  say "ok: Cargo.lock has one $name, $have from crates.io, equal to $P $row"
done

sys=$(pin steamworks-sys); sdk=$(pin steamworks-sdk)
if ! grep -q '^name = "steamworks-sys"' Cargo.lock; then
  say "skip: no steamworks-sys in Cargo.lock, so no redistributable to hash; bites when W0-08 adds steamworks"
else
  if [ -n "${STEAM_SDK_LOCATION:-}" ] || grep -qs STEAM_SDK_LOCATION .cargo/config.toml .cargo/config; then
    fail "STEAM_SDK_LOCATION is set (environment or .cargo/config.toml), so steamworks-sys builds another SDK" "unset it; ship the SDK $sdk vendored in steamworks-sys $sys (ADR 0002)"; fi
  dir=$(cargo metadata --format-version 1 --locked | grep -o "\"manifest_path\":\"[^\"]*steamworks-sys-${sys}[^\"]*\"" |
    sed 's/^"manifest_path":"//; s/Cargo\.toml"$//; s/\\\\/\//g') || true
  [ -n "$dir" ] || fail "cargo metadata shows no steamworks-sys-$sys directory to hash" "build steamworks-sys from the crates.io registry, with no source replacement (ADR 0002)"
  for rel in "${rels[@]}"; do
    want=$(pin "redist/$rel")
    have=$(sha256sum "${dir}lib/steam/redistributable_bin/$rel" 2>/dev/null | cut -d' ' -f1 || true)
    [ "$have" = "$want" ] || fail "$rel in steamworks-sys $sys hashes to ${have:-nothing (missing)}, $P redist/$rel is $want" "ship only the SDK $sdk redistributable (ADR 0002)"
    say "ok: $rel in steamworks-sys $sys matches $P (SDK $sdk)"
  done
fi

if [ -z "$(find .github/workflows .github/actions -name '*.y*ml' 2>/dev/null)" ]; then
  say "skip: no .github/workflows yet; bites when W0-07 adds ci.yml"
else
  while read -r at u; do
    a=$(echo "${u%%@*}" | cut -d/ -f1-2) ref=${u#*@}
    echo "$ref" | grep -qE '^[0-9a-f]{40}$' || fail "$at uses $u, not a full commit SHA" "pin $a to its full SHA in $P"
    p=$(pin "$a" "$at")
    [ "$ref" = "$p" ] || fail "$at uses $a@$ref, but $P pins $p" "use the pinned SHA; a new action needs its ADR and row first (rule 6)"
  done < <(find .github/workflows .github/actions -name '*.y*ml' -exec grep -HnoE "(^[[:space:]-]*|[{,][[:space:]]*)[\"']?uses[\"']?[[:space:]]*:[[:space:]]*[^[:space:],}#]*" {} + 2>/dev/null |
    sed -E "s/^([^:]*:[0-9]+):.*uses[\"']?[[:space:]]*:[[:space:]]*[\"']?([^\"']*).*/\1 \2/" | grep -v ' \./' | sort -t: -k1,1 -k2,2n || true)
  say "ok: every uses: in .github/workflows and .github/actions is a full commit SHA equal to $P"
fi

say "skip: dotnet-sdk comes with client/global.json and the codegen pipeline in week 1 (ADR 0004); godot and unity in week 1's pin-only PRs; codegen waits on ADR 0004 (W0-10), csharp-binding on the client engineer's binding ADR (ADR 0002 open pin task)"
