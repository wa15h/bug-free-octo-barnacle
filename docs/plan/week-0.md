# Week 0 queue

Producer, 2026-10-08. Planned for 2026-10-12 to 2026-10-18; the founder's first `hours.md` row sets
the real dates. This is week 0's `queue.md`; from week 1 the queue lives in `docs/plan/queue.md`.
Week 0 comes before any feature work (`decisions.md`, "Week 0"). Agents take one item at a time in
the order below, and an item waits until everything in its "After" column has merged. "Per <agent>
task N" means the acceptance in that agent's first-task list, plus what follows it here. Under a
`FROZEN:` line below the title (founder review budget spent), open nothing; in-flight work waits as draft.

**Gap flag: no `hours.md` row exists yet** (W0-11 creates it). This queue is sized on D5's planned 30
hours, not logged hours; week 1 is re-sized from the week-0 row. Deload fortnight: weeks 12-13 (D5).

## Budget

| Measure | Arithmetic | Week 0 |
|---|---|---|
| Project hours | D5 revised, planned | 30 |
| Review time | 30 x 0.4 (HS 7, D6) | 12 h = 720 min |
| Line budget | 12 h x 250 lines an hour (HS 7) | 3,000 changed lines |
| PR band | D6 revised | 20-25 |
| Committed below | 21 PRs: 19 by agents, 2 by the founder | 1,825 lines; 245 min first-pass review |
| Build-hours if 30 are logged | (30 - 2.5) x 0.8 x 0.92 (D23) | 20.2; cumulative 20.2 of Gate 0's ~250 |

Founder minutes: line-by-line at lines / 250 x 60 (the reviewer's formula); evidence at 5 up to 50
lines, 10 up to 150, 15 above, plus 5 when the founder signs something. The producer cap of 3,000 /
200 = 15 PRs assumes full-size PRs; week 0 averages 87 lines, so the D6 band sets the count. Lines
(61% of budget) and review minutes (34%) are low on purpose: there is no logged baseline yet.

## Founder time (allotments for checking fit, not estimates of logged hours)

| Item | Allotment |
|---|---|
| First-pass review of W0-01 to W0-18 and W0-21 (Min column) | 4 h 5 min |
| Return-round reserve (one round per PR, the reviewer's rule) | 1 h |
| Writing W0-19 and W0-20 | 1 h |
| SDR run for W0-08: Linux machine or VM with no Steam client, desktop with Steam, repro, logs | 3 h, cap 4 h |
| Setup: pinned toolchain, Steam client; after W0-07 and W0-17 protect `main`, require both workflows, create labels `size/over-200`, `founder-approved`, `upgrade`, run `tools/hooks/install.sh`; a GitHub identity for agents that is not yours, and `FOUNDER_LOGIN` for W0-21 | 2 h |
| Starting 19 agent items, answering the Questions block, the `hours.md` row and week-end summary | 1 h 45 min |
| D23 overhead (devlog, Thursday group, community) | 2 h 30 min |
| Total planned | 15 h 20 min of 30 |

Review uses about 5 of the 12 review hours; nothing in scope is pushed for lack of hours. Latency is
the risk: W0-02, 06, 07, 16, 17 are five serial merges, so at one founder pass a day a return round
carries W0-18 and W0-21 into week 1, charged first there. If W0-01 to W0-21 have merged and hours
remain, pull forward P0-P3 (25 PRs, 2,185 lines); any not pulled forward wait behind D27's items in week 1.

## Queue

Class: L = line-by-line, E = evidence, F = founder-authored. Min = founder minutes (writing time for F).

| ID | Owner | PR | Class | Lines | Min | After | Decisions | ADR-first |
|---|---|---|---|---|---|---|---|---|
| W0-01 | producer | Week-0 queue (this file) | E | 200 | 15 | - | D5 D6 D21 D23 D27 | no |
| W0-02 | devex-engineer | Cargo workspace, six empty crates | L 30 + E | 110 | 20 | - | D3 D17 | no dependency added |
| W0-03 | producer | ADR template and README | E | 70 | 10 | - | D3 D7 D8 D17 | n/a |
| W0-04 | devex-engineer | ADR 0001: Rust toolchain and CI action pins | E | 40 | 10 | 03 | D17 | is the ADR |
| W0-05 | server-engineer | ADR 0002: steamworks 0.13.1 on SDK 1.64 | E | 40 | 10 | 03 | D3 D17 | is the ADR |
| W0-06 | devex-engineer | Toolchain pins and `pins.sh` | E | 90 | 10 | 02 04 05 | D17 | 0001, 0002 |
| W0-07 | devex-engineer | CI: fmt, clippy, test on Linux and Windows | E | 90 | 10 | 06 | D6 D15 D17 | 0001 |
| W0-08 | server-engineer | Week-0 item 1: SDR spike on app 480 | E | 150 | 15 | 05 07 | D3 | 0002 |
| W0-09 | server-engineer | Week-0 item 2: sidecar watchdog spike, ADR 0003 | E | 60 | 15 | 02 03 | D8 | is the ADR |
| W0-10 | net-engineer | Week-0 item 3: wire-format spike, ADR 0004 | E | 60 | 20 | 02 03 | D7 | is the ADR |
| W0-11 | producer | Hours log | E | 55 | 10 | - | D5 D23 | no |
| W0-12 | client-engineer | Week-0 item 4: engine bake-off plan | E, signed | 120 | 20 | 11 | D4 D10 D17 | no |
| W0-13 | producer | Gate tracker | E | 160 | 15 | 11 | D2 D5 D10 D11 D19 D23 D28 | no |
| W0-14 | reviewer | Review checklists (review-log section 1) | E | 190 | 15 | - | D6 D18 D22 | no |
| W0-15 | reviewer | Founder summary format (section 2) | E | 70 | 10 | 14 | D6 D18 | no |
| W0-16 | devex-engineer | Pre-push hook and installer | E | 70 | 10 | 07 | D6 D25 | no |
| W0-17 | devex-engineer | PR size gate | E | 120 | 15 | 16 | D6 D22 | 0001 |
| W0-18 | devex-engineer | PR template copy and drift check | E | 45 | 5 | 17 | D6 | no |
| W0-19 | founder | Spike outcomes into `decisions.md` | F | 10 | 30 | 08 09 10 | D3 D7 D8 D24 | no |
| W0-20 | founder | D25 Stop hook in `.claude/settings.json` | F | 20 | 30 | 16 | D24 D25 | no |
| W0-21 | devex-engineer | Protected-paths gate (D24) | E | 55 | 10 | 17 | D24 | no |

**W0-01.** Paths: `docs/plan/week-0.md`. Accept: every item has ID, owner, paths, class, lines,
acceptance, decisions, ADR-first; lines at most 3,000 and PRs inside 20-25; each spike names the
decision its outcome writes; the budget reproduces from D5, D6, D23, HS 7; 200 lines or fewer.

**W0-02.** Paths: `Cargo.toml`, `Cargo.lock` (exempt, D22), `crates/*/Cargo.toml` and `src/lib.rs`
(`server`: `src/main.rs`), `rustfmt.toml`, `.editorconfig`, `.gitignore`. Accept per devex task 1;
Windows is proven by W0-07's first run. Line by line: the sim, proto, persist, server manifests and roots.

**W0-03.** Paths: `docs/adr/0000-template.md`, `docs/adr/README.md`. Accept per producer task 4; the
README also says the producer opens each ADR PR with the stub and the owner fills it in the same PR,
and that 0001-0004 are reserved by this queue.

**W0-04.** Paths: `docs/adr/0001-build-tooling-pins.md`. Accept: under 40 lines; the exact Rust
toolchain (current stable on the PR date, moved only when a pinned dependency needs it, Q24 default)
and every GitHub Action W0-07 and W0-17 use, each at a full commit SHA with its maintainer.

**W0-05.** Paths: `docs/adr/0002-steamworks.md`. Accept per server-engineer task 1(a): under 40
lines; one `steamworks` version and one redistributable workspace-wide; the C# binding's SDK 1.65
named as an open pin task for the client engineer.

**W0-06.** Paths: `rust-toolchain.toml`, `tools/pins.toml`, `tools/checks/pins.sh`. Accept per devex
task 2, minus `client/global.json` (week 1, with the codegen pipeline).

**W0-07.** Paths: `.github/workflows/ci.yml`. Accept per devex task 3, with the ADR already merged
as W0-04 (rule 6); no dotnet job until `client/` exists; no Steam client, app ID, or secret.

**W0-08 SDR spike; the outcome writes D3.** Paths: `crates/server/examples/sdr_spike.rs`,
`crates/server/Cargo.toml`. Accept per server-engineer task 1(b), plus: CI green with no Steam client;
one `steamworks` in `cargo tree -d`. The founder runs it on a machine they already own, no spend (Q18
has no written default). FAIL is a result, not a slip. Past the 4-hour cap, the run and D3's line move
to week 1, flagged at the top of its queue, and no transport work starts before D3's line exists.

**W0-09 Sidecar watchdog spike; the outcome writes D8.** Paths: `docs/adr/0003-sidecar-watchdog.md`.
Code lives on branch `spike/sidecar-watchdog`, never a PR, never merged. Accept: a std-only Rust
parent and child (no new dependency) run on ubuntu and windows by the branch's own workflow, with
logs and exit codes for spawn and `Ready` over localhost TCP on a parent-chosen port; a silent control
socket, child exits 2 within `CONTROL_SILENCE_SECS + 1` (Q16 defaults 10 and 2); `Shutdown` exits 0,
and a child that ignores it is killed by recorded PID and start time, never a reused PID; a killed
parent's child exits by watchdog; a dead-PID lock is cleared on restart, a live one exits 3. The ADR,
under 60 lines: a result table per OS, run links, what the C# launcher must copy, the D8 paragraph.
The client engineer reviews the parent-side findings.

**W0-10 Wire-format spike; the outcome writes D7.** Paths: `docs/adr/0004-wire-format.md`. Accept per
net-engineer task 1, with the ADR under 60 lines, plus: the ADR PR opens as a draft whose first commit
pre-registers the C# decode-cost threshold that flips D7 (Q14 has no written default); the founder
approves or edits it in a PR comment before results are pushed; the runner is named in the table.

**W0-11.** Paths: `docs/plan/hours.md`. Accept per producer task 2, plus: a rolling 8-week average
column (D5's re-baseline trigger, under 20), bake-off hours per engine summed from `client/BAKEOFF.md`
(Q19 default), and a blank week-0 row.

**W0-12 Bake-off plan; the outcome writes D4 at the ~600 h gate.** Paths: `client/BAKEOFF.md`. Accept
per client-engineer task 3, plus: the tie rule as a formula with a worked example; exports on the
founder's machine, logged as founder hours (Q20 default); hours feed W0-11.

**W0-13.** Paths: `docs/plan/gates.md`. Accept per producer task 3, plus: bands at the rolling pace and
at 15, 20, and 30 h/week, the 30 h/week band reproducing D11; the D5 re-baseline trigger and quarterly
fatigue check, re-dating only (D28); the review-saturation counter at the D6 revised cap; 200 lines or fewer.

**W0-14.** Paths: `docs/plan/review-log.md`, section 1. Accept per reviewer task 1, plus: line
counting follows D22 and the 201-400 band follows Q4's default.

**W0-15.** Paths: `docs/plan/review-log.md`, section 2. Accept per reviewer task 2.

**W0-16.** Paths: `tools/hooks/pre-push`, `tools/hooks/install.sh`. Accept: `install.sh` sets
`core.hooksPath tools/hooks`; `pre-push` runs the W0-07 commands and `pins.sh`, with git's arguments
and also standalone with no arguments and empty stdin (W0-20 calls it that way); the first failure
exits non-zero with `RULE <n>: <what> -- <fix>`; under two minutes on the skeleton; red and green pasted.

**W0-17.** Paths: `tools/checks/pr-size.sh`, `tools/size-exempt.txt`, `.github/workflows/pr-gates.yml`,
one line in `pre-push`. Accept per devex task 4, except the exempt list follows D22 (`Cargo.lock`,
generated C#, engine scene and `.meta` files, `fixtures/`, listed with sizes in the job summary).

**W0-18.** Paths: `.github/PULL_REQUEST_TEMPLATE.md`, `tools/checks/template-drift.sh`, a job in
`pr-gates.yml`. Accept: the copy equals `docs/plan/pr-template.md`; a new PR opens with the five
sections; a one-word edit to either file goes red naming both; green on head.

**W0-19, founder only (D24).** Paths: `docs/plan/decisions.md`. Accept: D3's outcome from W0-08, D8's
from ADR 0003, D7's from ADR 0004, each linking its PR; no other row changes; the reviewer
pre-reviews. An outcome not in by week end goes in a week-1 founder PR.

**W0-20, founder only (D24), opened after W0-16 merges.** Paths: `.claude/settings.json`. Accept: one
Stop hook runs `tools/hooks/pre-push` from `$CLAUDE_PROJECT_DIR` with stdin closed, so the hook's JSON
is not read as git ref lines; a failing check blocks the stop and shows its message to the agent
(Claude Code blocks only on exit code 2 or a `"decision": "block"` reply, so pre-push's exit 1 is
mapped); an explicit timeout above two minutes; red and green: a session leaving an unformatted file
is held with the fmt message, a clean one stops. `SubagentStop` runs it too (D25: "when an agent
finishes"; the team runs as subagents); the PR says how a repeat block (`stop_hook_active`) ends.

**W0-21.** Paths: `tools/checks/protected-paths.sh`, a job in `pr-gates.yml`. Accept: a diff touching
`.claude/`, `CLAUDE.md`, `docs/research/`, or `decisions.md` fails `RULE D24: <path> is founder-edited`
unless the PR author is `FOUNDER_LOGIN`; red on an agent branch, green on head and on a founder branch.

## Pull forward only if W0-01 to W0-21 have merged and hours remain

| ID | Owner | PR | Class | Lines | Min | Waits on |
|---|---|---|---|---|---|---|
| P0 | devex-engineer | `never-in-v1.sh`: no bevy, godot or gdext package, cdylib crate, `.gdextension`, prediction or rollback identifier, engine netcode, lobby search, voice, or Steam inventory API (D2 D4 D8 D9 D14) | E | 90 | 10 | W0-17 |
| P1 | producer | `docs/plan/milestones.md` (producer task 5); the founder signs the D11 line | E | 150 | 20 | nothing; V needs logged weeks first |
| P2 | reviewer | Review-log section 3, the hook request list (reviewer task 6) | E | 60 | 10 | W0-14 |
| P3 | devex-engineer | `tools/checks/pr-template.sh`: five headings, none empty (rule 5) | E | 60 | 10 | P2 |

## Pushed to week 1 (waits on a week-0 outcome, or is feature work)

- D27 first: sim skeleton and ADRs, ledger, the three settler jobs, the test engineer's settlement viewer.
- server-engineer: the `tokio` ADR, `SIDECAR.md`, the binary skeleton. Wait on ADR 0003 and D8's line.
- net-engineer: `crates/proto` skeleton; devex-engineer: codegen pipeline, C# solution files,
  `client/global.json`; client-engineer: `client/lib` skeleton. Wait on ADR 0004.
- devex-engineer: Godot 4.7 and Unity 6 pin-only PRs (rule 8); client-engineer: rung 1 in each engine.
  Wait on W0-12 and `client/lib`; the bake-off hours start there.
- Feature work after D27's items: persist save-encoding ADR, test simulator, then old P4 (sim allowlist).
- reviewer: the dry run with eight planted violations. Needs W0-14 to W0-18 merged.
- marketing: the devlog outline, built from W0-19. Game-designer, art-director, and marketing tasks 3-4
  start week 1; steam-release and marketing's page-era tasks wait until the page is ~200 build-hours off.
- The first D12 architecture review (spend: the founder books and pays), at the end of the weeks 0-6 spikes.

## Open questions for the founder

1. Q14 and Q18 have no written default; W0-10 and W0-08 state this queue's assumption until answered.
2. Resolved 2026-10-08: the skeptic review's patch was applied with the team (agent files now plan at
   30 h, and `CLAUDE.md` carries the in-week freeze, agent GitHub identity, and eligibility-only add-backs).
3. D2's add-back trigger fires near week 26; D28 makes the cut scope the target. Eligibility only, unless you raise it.

## Questions

Agents add `W0-NN: question` here. The producer answers or parks it.
