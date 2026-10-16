#!/usr/bin/env bash
# D24 gate: .claude/, CLAUDE.md, docs/research/, and docs/plan/decisions.md are founder-edited. pr-gates.yml runs this
# file with the PR number. It is not in tools/hooks/pre-push: a push has no PR and no label to read.
# It reads commits, not the working tree: each path `git diff --no-renames --name-only` lists from HEAD's merge base
# with origin/main to HEAD (a rename lists its old and new path). A diff touching none of the four passes.
# A diff touching one passes only if the PR carries founder-approved (D29). The labels come from the GitHub API when
# the job runs, not from the event, so a re-run sees the current labels. The label is the only override: no env var,
# no comment, no author check (every actor is the founder's login, D29). Run with no PR number, it reads no label.
# Each protected path prints `RULE D24: <path> is founder-edited -- <fix>`, then the script exits 1.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
pr=${1:-}
rule() { echo "RULE D24: $1 -- $2" >&2; [ -z "${GITHUB_ACTIONS:-}" ] || echo "::error::RULE D24: $1 -- $2"; }
fail() { rule "$1" "$2"; exit 1; }
say() { echo "protected-paths.sh: $1"; }

git rev-parse -q --verify 'origin/main^{commit}' >/dev/null || fail "there is no origin/main to diff against" "run git fetch origin main, then rerun"
base=$(git merge-base origin/main HEAD) || fail "HEAD has no merge base with origin/main (a shallow clone, or unrelated history)" "run git fetch --unshallow origin, or branch from origin/main"
hit=$(git diff --no-renames --name-only -z "$base" HEAD -- .claude CLAUDE.md docs/research docs/plan/decisions.md | tr '\0' '\n') ||
  fail "git diff failed (its error is above)" "fix the checkout, then rerun"
[ -n "$hit" ] || { say "ok: nothing under .claude/, CLAUDE.md, docs/research/, or docs/plan/decisions.md changed since ${base:0:7} (D24)"; exit 0; }
if [ -n "$pr" ]; then
  labels=$(gh api "repos/{owner}/{repo}/pulls/$pr" --jq '.labels[].name') || fail "the labels of PR #$pr could not be read (gh's error is above)" "re-run this job"
  if grep -qxF founder-approved <<< "$labels"; then say "ok: PR #$pr carries founder-approved (D29); founder-edited: $(paste -sd' ' <<< "$hit")"; exit 0; fi
fi
while IFS= read -r p; do
  rule "$p is founder-edited" "drop it from this branch and propose the change in the PR description or an open question (D24); a founder PR passes once it carries founder-approved (D29)"
done <<< "$hit"
exit 1
