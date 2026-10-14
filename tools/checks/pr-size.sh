#!/usr/bin/env bash
# Rule 4 size gate (D6, D22); the pre-push hook (W0-16) and pr-gates.yml run this same file, which reads only git.
# Count method: the counted lines are the additions plus deletions that `git diff --numstat -M` reports from HEAD's
# merge base with origin/main to HEAD, summed over every file that no glob in tools/size-exempt.txt matches.
# Over 200 passes with a notice (pr-gates.yml labels the PR size/over-200); over 400 exits 1 with
# `RULE 4: <what> -- <fix>`. Every exempt file is listed with its size. In CI the total, the exempt files, and any
# RULE line also go to the job summary, and `counted=<n>` to the step outputs; those only route output.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
X=tools/size-exempt.txt
note() { [ -z "${GITHUB_STEP_SUMMARY:-}" ] || echo "- $1" >> "$GITHUB_STEP_SUMMARY"; }
fail() { echo "RULE 4: $1 -- $2" >&2; note "RULE 4: $1 -- $2"; exit 1; }
out() { echo "pr-size.sh: $1"; note "$1"; }

git rev-parse -q --verify 'origin/main^{commit}' >/dev/null || fail "there is no origin/main to count against" "run git fetch origin main, then rerun"
base=$(git merge-base origin/main HEAD) || fail "HEAD shares no history with origin/main" "branch from origin/main"
inc=(); exc=()
while IFS= read -r g; do inc+=(":(glob)$g"); exc+=(":(exclude,glob)$g"); done < <(tr -d '\r' < "$X" | grep -vE '^[[:space:]]*(#|$)')
[ "${#inc[@]}" -gt 0 ] || fail "$X is missing or lists no globs, so pr-size.sh cannot apply D22" "restore $X from origin/main"
# One row per file: lines (added plus deleted; a binary file is 0), path, and the numstat it came from.
rows() { git diff --numstat -M "$base" HEAD -- "$@" | awk -F'\t' '{ b = $1 == "-"; print (b ? 0 : $1 + $2) "\t" $3 "\t" (b ? "binary" : "+" $1 " -" $2) }'; }
if ! counted=$(rows . "${exc[@]}" | sort -rn) || ! exempt=$(rows "${inc[@]}"); then fail "git diff failed (its error is above)" "fix the checkout, then rerun"; fi
read -r n files < <(awk -F'\t' 'NF { s += $1; f++ } END { print s + 0, f + 0 }' <<< "$counted")
[ -z "${GITHUB_OUTPUT:-}" ] || echo "counted=$n" >> "$GITHUB_OUTPUT"

out "$n counted lines (D22) in $files files, HEAD against its merge base ${base:0:7} with origin/main; over 200 is labelled size/over-200, over 400 fails (D6)"
while IFS=$'\t' read -r c p s; do [ -z "$p" ] || echo "pr-size.sh:   counted $c in $p ($s)"; done <<< "$counted"
[ -n "$exempt" ] || out "exempt (D22, $X): none"
while IFS=$'\t' read -r c p s; do [ -z "$p" ] || out "exempt (D22, $X), not counted: \`$p\`, $c changed ($s); the reviewer spot-checks it"; done <<< "$exempt"
top=$(head -n 3 <<< "$counted" | awk -F'\t' '{ printf "%s%s (%s)", (NR > 1 ? ", " : ""), $2, $1 }')
[ "$n" -le 400 ] || fail "$n counted lines (D22) versus origin/main, over the 400-line hard cap (D6); largest: $top" "split the branch into PRs of 200 or fewer, one concern each (Q4)"
[ "$n" -le 200 ] || out "over 200 counted lines: pr-gates.yml labels the PR size/over-200, and the reviewer returns it with a split plan (Q4)"
out "ok: $n counted lines, within the 400-line hard cap (D6)"
