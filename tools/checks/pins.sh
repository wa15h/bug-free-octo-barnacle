#!/usr/bin/env bash
# D17 gate; the pre-push hook (W0-16) and CI call this same file. Checks that rust-toolchain.toml,
# Cargo.lock (one steamworks and one steamworks-sys by `cargo tree -d`), the vendored redistributables'
# SHA-256s, and every workflow `uses:` equal tools/pins.toml. A check with nothing to check yet says so,
# names the item that makes it bite, and passes. The first failure exits 1: `RULE D17: <what> -- <fix>`.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
P=tools/pins.toml
fail() { echo "RULE D17: $1 -- $2" >&2; exit 1; }
say() { echo "pins.sh: $1"; }
pin() { v=$(sed -n "s|^\"\{0,1\}$1\"\{0,1\} *= *\"\([^\"]*\)\".*|\1|p" "$P")
  [ -n "$v" ] || fail "$P has no $1 row${2:+, used at $2}" "add the row its ADR names first (rule 6)"; echo "$v"; }

bad=$(grep -nE '^[^#].*=' "$P" | grep -vE '= "([0-9]+(\.[0-9]+){1,2}|[0-9a-f]{40}|[0-9a-f]{64}|pending)"( *#.*)?$' || true)
[ -z "$bad" ] || fail "$P line ${bad%%:*} is not an exact version, full SHA, or \"pending\"" "write one exact value"

rust=$(pin rust)
ch=$(sed -n 's/^channel *= *"\(.*\)"/\1/p' rust-toolchain.toml 2>/dev/null || true)
echo "$rust" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$' || fail "$P rust \"$rust\" is not x.y.z" "pin the exact toolchain ADR 0001 names"
[ "$ch" = "$rust" ] || fail "rust-toolchain.toml channel \"$ch\" differs from $P rust \"$rust\"" "make both the ADR 0001 version in one pin-only PR (rule 8)"
say "ok: rust-toolchain.toml channel $ch equals $P rust"

tree=$(cargo tree -d --target all --locked --prefix none 2>&1) || fail "cargo tree -d --locked failed: $(echo "$tree" | tail -1)" "fix Cargo.lock, then rerun"
for c in steamworks:steamworks-rs steamworks-sys:steamworks-sys; do
  name=${c%%:*} row=${c#*:}; want=$(pin "$row")
  if echo "$tree" | grep -qE "^$name v"; then
    fail "cargo tree -d shows more than one $name in Cargo.lock: $(echo "$tree" | sed -n "s/^$name \(v[^ ]*\).*/\1/p" | sort -u | paste -sd' ' -)" "keep only $name $want in every crates/*/Cargo.toml (ADR 0002)"
  fi
  have=$(grep -A1 "^name = \"$name\"$" Cargo.lock | sed -n 's/^version = "\(.*\)"/\1/p' || true)
  if [ -z "$have" ]; then say "skip: no $name in Cargo.lock yet; bites when W0-08 adds steamworks to crates/server"
  elif [ "$have" != "$want" ]; then fail "Cargo.lock has $name $have, $P $row is $want" "require =$want in crates/*/Cargo.toml, or run cargo update -p $name --precise $want"
  else say "ok: Cargo.lock has one $name, $have, equal to $P $row"; fi
done

sys=$(pin steamworks-sys)
if ! grep -q '^name = "steamworks-sys"$' Cargo.lock; then
  say "skip: no steamworks-sys in Cargo.lock, so no redistributable to hash; bites when W0-08 adds steamworks"
else
  dir=$(cargo metadata --format-version 1 --locked | grep -o "\"manifest_path\":\"[^\"]*steamworks-sys-${sys}[^\"]*\"" |
    sed 's/^"manifest_path":"//; s/Cargo\.toml"$//; s/\\\\/\//g')
  while read -r rel; do
    want=$(pin "redist/$rel")
    have=$(sha256sum "${dir}lib/steam/redistributable_bin/$rel" 2>/dev/null | cut -d' ' -f1 || true)
    [ "$have" = "$want" ] || fail "$rel in steamworks-sys $sys hashes to ${have:-nothing (missing)}, $P redist/$rel is $want" "ship only the SDK $(pin steamworks-sdk) redistributable (ADR 0002)"
    say "ok: $rel in steamworks-sys $sys matches $P (SDK $(pin steamworks-sdk))"
  done < <(sed -n 's|^"redist/\([^"]*\)".*|\1|p' "$P")
fi

if ! ls .github/workflows/*.y*ml >/dev/null 2>&1; then
  say "skip: no .github/workflows yet; bites when W0-07 adds ci.yml"
else
  while read -r at u; do
    a=$(echo "${u%%@*}" | cut -d/ -f1-2) ref=${u#*@}
    echo "$ref" | grep -qE '^[0-9a-f]{40}$' || fail "$at uses $u, not a full commit SHA" "pin $a to its full SHA in $P"
    p=$(pin "$a" "$at")
    [ "$ref" = "$p" ] || fail "$at uses $a@$ref, but $P pins $p" "use the pinned SHA; a new action needs its ADR and row first (rule 6)"
  done < <(grep -HnE '^[[:space:]-]*uses:' .github/workflows/*.y*ml | grep -v 'uses:[[:space:]]*\./' |
    sed -E "s/^([^:]*:[0-9]+):.*uses:[[:space:]]*[\"']?([^\"'[:space:]#]*).*/\1 \2/")
  say "ok: every workflow uses: is a full commit SHA equal to $P"
fi

say "skip: csharp-binding and codegen pending, no dotnet-sdk, godot, or unity row; week 1's client/lib, codegen, and engine pin PRs add them"
