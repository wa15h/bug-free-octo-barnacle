#!/usr/bin/env bash
# Rule 5 drift gate (W0-18); pr-gates.yml runs this file on PRs (the gate map does not put it in tools/hooks/pre-push).
# GitHub's web form fills a new PR body from .github/PULL_REQUEST_TEMPLATE.md, so that file must equal its source,
# docs/plan/pr-template.md, byte for byte. Otherwise it prints the diff and exits 1 with `RULE 5: <what> -- <fix>` last.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
src=docs/plan/pr-template.md copy=.github/PULL_REQUEST_TEMPLATE.md
fail() { echo "RULE 5: $1 -- $2" >&2; [ -z "${GITHUB_STEP_SUMMARY:-}" ] || echo "- RULE 5: $1 -- $2" >> "$GITHUB_STEP_SUMMARY"; exit 1; }
[ -f "$src" ] || fail "$src is missing, so $copy has no source" "restore $src from origin/main"
[ -f "$copy" ] || fail "$copy is missing, so it cannot equal $src" "run cp $src $copy and commit it"
cmp -s "$src" "$copy" && { echo "template-drift.sh: ok: $copy equals $src"; exit 0; }
diff -u "$src" "$copy" >&2 || true
fail "$copy differs from $src" "make the change in $src only, then run cp $src $copy and commit both"
