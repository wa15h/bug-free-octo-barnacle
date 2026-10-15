#!/usr/bin/env bash
# Rule 4 size gate (D6, D22); the pre-push hook (W0-16) and pr-gates.yml run this same file. It reads commits, not
# the working tree. Each git diff takes attributes from the empty tree (git --attr-source, git 2.40 or later), so no
# committed or worktree .gitattributes is read; only git's own checks mark a file binary: a NUL byte in its first 8000
# bytes, or a size over core.bigFileThreshold. The local $GIT_DIR/info/attributes, core.attributesFile, and
# core.bigFileThreshold still apply and can make the hook and CI disagree; CI's fresh checkout is the count that gates.
# Count method: the counted lines are the additions plus deletions that `git diff --numstat -M` reports from HEAD's
# merge base with origin/main to HEAD, summed over every file whose path (a renamed file's new path) no glob in
# tools/size-exempt.txt at HEAD matches.
# Over 200 passes with a notice (pr-gates.yml labels the PR size/over-200); over 400 exits 1 with
# `RULE 4: <what> -- <fix>`. Every exempt file is listed with its size: lines added and deleted, or bytes if binary.
# In CI the total, the exempt files, and any RULE line also go to the job summary, and `counted=<n>` to the step
# outputs; those only route output.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
X=tools/size-exempt.txt
note() { [ -z "${GITHUB_STEP_SUMMARY:-}" ] || echo "- $1" >> "$GITHUB_STEP_SUMMARY"; }
fail() { echo "RULE 4: $1 -- $2" >&2; note "RULE 4: $1 -- $2"; exit 1; }
out() { echo "pr-size.sh: $1"; note "$1"; }

git rev-parse -q --verify 'origin/main^{commit}' >/dev/null || fail "there is no origin/main to count against" "run git fetch origin main, then rerun"
base=$(git merge-base origin/main HEAD) || [ "$(git rev-parse --is-shallow-repository)" = false ] ||
  fail "this clone is shallow, so git finds no merge base of HEAD and origin/main" "run git fetch --unshallow origin, then rerun"
[ -n "$base" ] || fail "HEAD shares no history with origin/main" "branch from origin/main"
inc=()
while IFS= read -r g; do inc+=(":(glob)$g"); done < <(git cat-file blob "HEAD:$X" 2>/dev/null | tr -d '\r' | grep -vE '^[[:space:]]*(#|$)')
[ "${#inc[@]}" -gt 0 ] || fail "$X is missing at HEAD or lists no globs, so pr-size.sh cannot apply D22" "restore $X from origin/main and commit it"
# One numstat for the whole diff, so a rename stays one row (with -z its paths follow on their own lines, old then new).
# A row is exempt when its path is among the changed paths a glob matches (--no-renames lists each side on its own).
if ! raw=$(git --attr-source="$(git hash-object -t tree /dev/null)" diff --numstat -z -M "$base" HEAD | tr '\0' '\n') ||
  ! ex=$(git --attr-source="$(git hash-object -t tree /dev/null)" diff --no-renames --name-only -z "$base" HEAD -- "${inc[@]}" | tr '\0' '\n'); then
  fail "git diff failed (its error is above)" "fix the checkout, or update git to 2.40 or later (--attr-source), then rerun"
fi
counted=; exempt=
while IFS=$'\t' read -r a d p; do
  [ -n "$a" ] || continue
  [ -n "$p" ] || { IFS= read -r p; IFS= read -r p; } # a rename: the old path, then the new path, which is kept
  if [ "$a" != - ]; then r="$((a + d))"$'\t'"$p"$'\t'"+$a -$d"
  elif b=$(git cat-file -s "HEAD:$p" 2>/dev/null); then r="0"$'\t'"$p"$'\t'"binary, $b bytes at HEAD"
  else r="0"$'\t'"$p"$'\t'"binary, deleted, $(git cat-file -s "$base:$p") bytes at the merge base"; fi
  case $'\n'$ex$'\n' in *$'\n'"$p"$'\n'*) exempt+=$r$'\n' ;; *) counted+=$r$'\n' ;; esac
done <<< "$raw"
counted=$(printf '%s' "$counted" | sort -rn)
read -r n files < <(awk -F'\t' 'NF { s += $1; f++ } END { print s + 0, f + 0 }' <<< "$counted")
[ -z "${GITHUB_OUTPUT:-}" ] || echo "counted=$n" >> "$GITHUB_OUTPUT"

out "$n counted lines (D22) in $files files, HEAD against its merge base ${base:0:7} with origin/main; over 200 is labelled size/over-200, over 400 fails (D6)"
while IFS=$'\t' read -r _ p s; do [ -z "$p" ] || echo "pr-size.sh:   counted: $p ($s)"; done <<< "$counted"
[ -n "$exempt" ] || out "exempt (D22, $X): none"
while IFS=$'\t' read -r _ p s; do [ -z "$p" ] || out "exempt (D22, $X), not counted: \`$p\` ($s); the reviewer spot-checks it"; done <<< "$exempt"
top=$(head -n 3 <<< "$counted" | awk -F'\t' '{ printf "%s%s (%s)", (NR > 1 ? ", " : ""), $2, $1 }')
[ "$n" -le 400 ] || fail "$n counted lines (D22) versus origin/main, over the 400-line hard cap (D6); largest: $top" "split the branch into PRs of 200 or fewer, one concern each (Q4)"
[ "$n" -le 200 ] || out "over 200 counted lines: pr-gates.yml labels the PR size/over-200, and the reviewer returns it with a split plan (Q4)"
out "ok: $n counted lines, within the 400-line hard cap (D6)"
