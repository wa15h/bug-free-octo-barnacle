WIP: 10

# Week 0 queue

Producer, 2026-10-08. Planned for 2026-10-12 to 2026-10-18, its real dates, from which `producer.md`'s
deload calendar counts. This is week 0's `queue.md`; from week 1 the queue lives in `docs/plan/queue.md`.
Week 0 comes before any feature work (`decisions.md`, "Week 0"). Agents take one item at a time in
the order below, and an item waits until everything in its "After" column has merged. "Per <agent>
task N" means the acceptance in that agent's first-task list, plus what follows it here. While the
first line reads `WIP: 0 FROZEN: <reason>` (D31), open nothing; in-flight work waits as draft.

**D31 (2026-10-14): founder hours leave the plan.** The week's budget and founder-time allotments are
deleted (git history keeps them) in favour of the first line, `WIP: 10`; open agent PRs, held drafts,
and freezes follow `producer.md`, "The WIP limit (D31)". The Min column, W0-11 (`hours.md`, now
deleted), and merged items' hours clauses are retired history. P1 is withdrawn (no `milestones.md`,
bands, or V): P0, P2, and P3 are pulled forward once W0-01 to W0-21 have merged and the WIP limit has
room; any not pulled forward wait behind D27's items in week 1. Deload fortnight: weeks 12-13
(`producer.md`, "Rest (D31)").

Founder setup, unchanged by D31: install the pinned toolchain and the Steam client; protect `main`
(require a pull request, no bypass, no required approvals: GitHub refuses self-approval, D29); create
the labels `size/over-200`, `founder-approved`, and `upgrade`; after W0-07 and W0-17, require both
workflows and run `tools/hooks/install.sh`.

## Queue

Class: L = line-by-line, E = evidence, F = founder-authored. Min = founder minutes (retired by D31).

| ID | Owner | PR | Class | Lines | Min | After | Decisions | ADR-first |
|---|---|---|---|---|---|---|---|---|
| W0-01 | producer | Week-0 queue (this file) | E | 200 | 15 | - | D5 D6 D21 D23 D27 | no |
| W0-02 | devex-engineer | Cargo workspace, six empty crates | L 30 + E | 110 | 20 | - | D3 D17 | no dependency added |
| W0-03 | producer | ADR template and README | E | 70 | 10 | - | D3 D7 D8 D17 | n/a |
| W0-04 | devex-engineer | ADR 0001: Rust toolchain and CI action pins | E | 40 | 10 | 03 | D17 | is the ADR |
| W0-05 | server-engineer | ADR 0002: steamworks 0.13.1 on SDK 1.64 | E | 40 | 10 | 03 | D3 D17 | is the ADR |
| W0-06 | devex-engineer | Toolchain pins and `pins.sh` | E | 90 | 10 | 02 04 05 | D17 | 0001, 0002 |
| W0-07 | devex-engineer | CI: fmt, clippy, test on Linux and Windows | E | 90 | 10 | 06 | D6 D15 D17 | 0001 |
| W0-08 | server-engineer | Week-0 item 1: SDR spike on app 480 | E | 150 | 15 | 05 07 22 | D3 | 0002, amended by W0-22 |
| W0-09 | server-engineer | Week-0 item 2: sidecar watchdog spike, ADR 0003 | E | 60 | 15 | 02 03 | D8 | is the ADR |
| W0-10 | net-engineer | Week-0 item 3: wire-format measurement, ADR 0004 | E | 60 | 20 | 02 03 | D7 D30 | is the ADR |
| W0-11 | producer | Hours log | E | 55 | 10 | - | D5 D23 | no |
| W0-12 | client-engineer | Week-0 item 4: engine bake-off plan | E, signed | 120 | 20 | 11 | D4 D10 D17 | no |
| W0-13 | producer | Gate tracker | E | 160 | 15 | 11 | D2 D5 D10 D11 D19 D23 D28 | no |
| W0-14 | reviewer | Review checklists (review-log section 1) | E | 190 | 15 | - | D6 D18 D22 | no |
| W0-15 | reviewer | Founder summary format (section 2) | E | 70 | 10 | 14 | D6 D18 | no |
| W0-16 | devex-engineer | Pre-push hook and installer | E | 70 | 10 | 07 | D6 D25 | no |
| W0-17 | devex-engineer | PR size gate | E | 120 | 15 | 16 | D6 D22 | 0001 |
| W0-18 | devex-engineer | PR template copy and drift check | E | 45 | 5 | 17 | D6 | no |
| W0-19 | founder | Spike outcomes into `decisions.md` | F | 10 | 30 | 08 09 | D3 D8 D24 D29 | no |
| W0-20 | founder | D25 Stop hook in `.claude/settings.json` | F | 20 | 30 | 16 | D24 D25 D29 | no |
| W0-21 | devex-engineer | Protected-paths gate (D24) | E | 55 | 10 | 17 | D24 D29 | no |
| W0-22 | server-engineer | ADR amendment for W0-08: steamworks' eight packages | E | 20 | - | 05 10 | D3 D17 D30 | is the ADR |

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
has no written default). FAIL is a result, not a slip. If week 1's queue is written while W0-08's PR
shows no PASS or FAIL (an unrun spike or an INCONCLUSIVE run), W0-08 and D3's line (W0-19) carry over,
flagged at the top of that queue; W0-08 is never cut for waiting (D28), and no transport work starts
before D3's line exists. Founder question 4 asks how the slip rule treats it while it waits.

**W0-09 Sidecar watchdog spike; the outcome writes D8.** Paths: `docs/adr/0003-sidecar-watchdog.md`.
Code lives on branch `claude/spike-sidecar-watchdog`, never a PR, never merged. Accept: a std-only Rust
parent and child (no new dependency) run on ubuntu and windows by the branch's own workflow, with
logs and exit codes for spawn and `Ready` over localhost TCP on a parent-chosen port; a silent control
socket, child exits 2 within `CONTROL_SILENCE_SECS + 1` (Q16 defaults 10 and 2); `Shutdown` exits 0,
and a child that ignores it is killed by recorded PID and start time, never a reused PID; a killed
parent's child exits by watchdog; a dead-PID lock is cleared on restart, a live one exits 3. The ADR,
under 60 lines: a result table per OS, run links, what the C# launcher must copy, the D8 paragraph.
The client engineer reviews the parent-side findings.

**W0-10 Wire-format measurement; D30 decided FlatBuffers.** Paths: `docs/adr/0004-wire-format.md`. Accept
per net-engineer task 1 as D30 narrows it: FlatBuffers only, no comparison and no threshold (Q14 answered);
the ADR under 60 lines (not task 1's 40) pins the `flatbuffers` crate, `flatc`, and `Google.FlatBuffers` at one version;
the table carries the measured numbers from a linked run and names the runner.

**W0-11.** Paths: `docs/plan/hours.md`. Accept per producer task 2, plus: a rolling 8-week average
column (D5's re-baseline trigger, under 20), bake-off hours per engine summed from `client/BAKEOFF.md`
(Q19 default), and a blank week-0 row.

**W0-12 Bake-off plan; the outcome writes D4 at the ~600 h gate (retired by D31).** Paths: `client/BAKEOFF.md`. Accept
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
`pr-gates.yml`. Accept: the copy equals `docs/plan/pr-template.md`; a new PR opens with the `Agent:`
line and the five sections; a one-word edit to either file goes red naming both; green on head.

**W0-19, founder only (D24).** Paths: `docs/plan/decisions.md`. Accept: D3's outcome from W0-08, D8's
from ADR 0003, each linking its PR (D7's outcome is D30, written 2026-10-10); no other row changes; carries `founder-approved` (D29); the reviewer
pre-reviews. An outcome not in by week end goes in a week-1 founder PR.

**W0-20, founder only (D24), opened after W0-16 merges.** Paths: `.claude/settings.json`. Accept: carries
`founder-approved` (D29); one Stop hook runs `tools/hooks/pre-push` from `$CLAUDE_PROJECT_DIR` with stdin closed, so the hook's JSON
is not read as git ref lines; a failing check blocks the stop and shows its message to the agent
(Claude Code blocks only on exit code 2 or a `"decision": "block"` reply, so pre-push's exit 1 is
mapped); an explicit timeout above two minutes; red and green: a session leaving an unformatted file
is held with the fmt message, a clean one stops. `SubagentStop` runs it too (D25: "when an agent
finishes"; the team runs as subagents); the PR says how a repeat block (`stop_hook_active`) ends.

**W0-21.** Paths: `tools/checks/protected-paths.sh`, a job in `pr-gates.yml`. Accept: a diff touching
`.claude/`, `CLAUDE.md`, `docs/research/`, or `decisions.md` fails `RULE D24: <path> is founder-edited`
unless the PR carries `founder-approved` (D29); rechecked on `labeled` and `unlabeled`; red without the label, green once the founder labels the throwaway PR, and green on head.

## Pull forward only if W0-01 to W0-21 have merged and the WIP limit has room (D31)

| ID | Owner | PR | Class | Lines | Min | Waits on |
|---|---|---|---|---|---|---|
| P0 | devex-engineer | `never-in-v1.sh`: no bevy, godot or gdext package, cdylib crate, `.gdextension`, prediction or rollback identifier, engine netcode, lobby search, voice, or Steam inventory API (D2 D4 D8 D9 D14) | E | 90 | 10 | W0-17 |
| P1 | producer | Withdrawn by D31: no `milestones.md`, bands, or V | - | - | - | - |
| P2 | reviewer | Review-log section 3, the hook request list (reviewer task 6) | E | 60 | 10 | W0-14 |
| P3 | devex-engineer | `tools/checks/pr-template.sh`: five headings, none empty (rule 5) | E | 60 | 10 | P2 |

## Pushed to week 1 (waits on a week-0 outcome, or is feature work)

- D27 first: sim skeleton and ADRs, ledger, the three settler jobs, the test engineer's settlement viewer.
- server-engineer: the `tokio` ADR, `SIDECAR.md`, the binary skeleton. Wait on ADR 0003 and D8's line.
- net-engineer: `crates/proto` skeleton; devex-engineer: codegen pipeline, C# solution files,
  `client/global.json`; client-engineer: `client/lib` skeleton. Wait on ADR 0004.
- devex-engineer: Godot 4.7 and Unity 6 pin-only PRs (rule 8); client-engineer: rung 1 in each engine.
  Wait on `client/lib` and the founder's signed scorecard in `client/BAKEOFF.md` on main (D31).
- Feature work after D27's items: persist save-encoding ADR, test simulator, then old P4 (sim allowlist).
- reviewer: the dry run with eight planted violations. Needs W0-14 to W0-18 merged.
- marketing: the devlog outline, built from W0-19. Game-designer, art-director, and marketing tasks 3-4
  start week 1; steam-release and marketing's page-era tasks wait for the two-player loop (`producer.md`, D31).
- The first D12 architecture review (spend: the founder books and pays), at the end of the weeks 0-6 spikes.

## Open questions for the founder

1. Q18 has no written default; W0-08 states this queue's assumption until answered. Q14 is answered by D30.
2. Resolved 2026-10-08: the skeptic review's patch was applied with the team (agent files then planned
   at 30 h, which D31 retires). `CLAUDE.md` then carried the in-week freeze and eligibility-only
   add-backs, which D31 replaced (the WIP limit and FROZEN lines; add-backs only by a founder decision
   row), and the agent login rule, which D29 replaced on 2026-10-10.
3. Answered (D31): there is no add-back trigger; a deferred item enters only through a founder decision row naming it.
4. The slip rule in `producer.md`'s "The weekly loop", step 5, splits or re-scopes an
   item in progress at its second reconcile without passing review. Is an item whose only remaining
   step is a founder action (a run, a signature, a verdict), such as W0-08, exempt while it waits? A
   queue line cannot grant that (D24): a yes goes in `producer.md`. Until then the rule applies as written.

## Questions

Agents add `W0-NN: question` here. The producer answers or parks it.

**W0-08: requests 1 to 6**, quoted verbatim from W0-08's draft description (1 to 5) and the
reviewer's pre-review of it (6), 2026-10-14. The producer answered 1, 3, 4, 5, and 6 on 2026-10-14;
the founder answered 2 on 2026-10-15.

**W0-08: request 1.**
> Producer, answer first: "W0-08: keep ports 54320 and 54321, the 60 s client timeout, the 1 ms
> poll, the 500 ms close wait, and the receive caps 8 and 1 as spike-local constants, or name them
> in a contract?" The founder's run waits on this answer. If the answer changes `sdr_spike.rs`, the
> change lands as new commits on this branch and the throwaway branch before the run, and the SHAs
> in the steps change with them (The founder's run).

Producer, 2026-10-14: keep all of them spike-local in `crates/server/examples/sdr_spike.rs` (:21
poll, :45 ports, :77 cap 8, :99 60 s, :108 cap 1, :113 500 ms), with the throwaway workflow's
30-minute window; none goes into a contract. `sdr_spike.rs` does not change, so `6f8d106` and
`fcfb2bc` stay the run's SHAs and steps 1-8 are final as written. Conditions: none of these numbers
is copied into `crates/server/src`, `SIDECAR.md`, `crates/proto`, or any test; W0-08's Risk list
stays their record; when the transport is queued after D3's line, its ports, timeouts, and caps are
asked fresh (server-engineer.md:109-110, :124) and land in `SIDECAR.md` or an ADR inside D3, never
inherited from the spike. They were coded before the question was asked; that is noted and needs no
rework. Why: the example never enters `src/` (server-engineer.md:16-17) and nothing builds against
it. The relayed connection is addressed by Steam ID and P2P virtual port 0 (task 1(b)); 54320 and
54321 are only the GameServer's own UDP ports, from steamworks-rs `examples/game-server` at
`bfc8210`. D3's PASS or FAIL depends on none of them. A contract would mean a `SIDECAR.md` or D3 ADR
edit outside W0-08's two paths (rule 4), and would pre-empt the real values, such as Q16's 10 and 2,
before D3 has an outcome.

**W0-08: request 2.**
> Founder: does SteamCMD's `steamclient.so` still count as "no Steam client installed"
> (server-engineer.md:174-175)? The runner depends on it. Q18 (open-questions.md:28, who provisions
> the headless Linux box) has no written default; re-running the workflow means you accept a
> GitHub-hosted runner as that box for this spike. Then run steps 1-8.

Founder, in chat, 2026-10-15, as relayed to the producer: "Yes on the runner." This answers request
2 for the W0-08 spike only: the run goes ahead on the GitHub-hosted runner with SteamCMD's
`steamclient.so`, which for this spike stands in for W0-08's "a machine they already own". Q18 for
the shipped headless box stays open (Open questions for the founder, 1). Request 5's answer stands.
Once this file merges, the founder's run (W0-08's steps 1-8) waits on nothing in this queue.

**W0-08: request 3.**
> Producer: queue the ADR amendment now as its own docs PR, to merge before this PR is opened (hold
> 2). It is written: commit `c1f3043` on the local branch `claude/adr-0002-steamworks-deps` (ADR
> 0002 +16 lines, ADR 0004:28 one line). It needs a push of that branch and a PR, which this item
> may not do. The server engineer owns ADR 0002; the net engineer, who owns ADR 0004, reviews its
> one line. ADR 0002 grows from 39 to 55 lines; W0-05's "under 40 lines" applied to that item, so
> the docs item's queue entry sets its own limit. The only other route is a founder ruling that an
> ADR'd crate's transitive packages are covered, which would also change `docs/adr/README.md:9`.

Producer, 2026-10-14: queued as W0-22 (the row above); W0-08 now waits on it. The server engineer,
who owns ADR 0002 (W0-05), pushes `claude/adr-0002-steamworks-deps` and opens its PR with `Agent:
server-engineer (W0-22)`; W0-08's push limit does not bind a separate item. On 2026-10-15 its head
is `45aac66`: `c1f3043` plus a fix showing where each package is required, authored `Claude` (D29).
No producer stub and no new number: it edits two Accepted ADRs in place and the choice is unchanged
(README:32-33), so 0005 stays free. The founder-ruling route is not needed. W0-22 gates when W0-08's
PR opens, not the founder's run. Once W0-22 merges, W0-08's writer removes hold 2 and its Rule 6
bullet reads "met: W0-22, #<n>"; no code, lock, or SHA changes, so re-review covers the description
only. Why: rule 6 and README:9 count every package new to `cargo metadata`, and README:25 puts the
ADR first. The lock diff between main and `6f8d106` is exactly `steamworks`, `steamworks-sys`, and
these eight, with one `bitflags`. ADR 0004:28 already names `flatbuffers`' transitive packages, so
this follows practice.

**W0-22.** Paths: `docs/adr/0002-steamworks.md`, line 28 of `docs/adr/0004-wire-format.md`. Accept:
nothing else changes (no code, `Cargo.lock`, `tools/pins.toml`, or pin); 20 counted lines at most
(`45aac66`: +18 -1). ADR 0002 names exactly the eight packages W0-08's lock at `6f8d106` adds beyond
`steamworks` 0.13.1 and `steamworks-sys` 0.13.0: `bitflags` 2.13.2, `thiserror` 2.0.21,
`thiserror-impl` 2.0.21, `paste` 1.0.15, `syn` 3.0.9, `quote` 1.0.47, `proc-macro2` 1.0.107,
`unicode-ident` 1.0.27. Each has its version, crates.io owner, licence, and linked or compile-time
only, and the ADR shows from steamworks-rs `bfc8210`'s `Cargo.toml` and crates.io where each is
required and that none is optional. The reviewer checks the list against the lock's package diff and
each crates.io page. ADR 0004:28 keeps every word and adds only that `bitflags` 2.13.2 is also used
through `server`'s `steamworks`; the net engineer, who owns ADR 0004 (W0-10), confirms that line in
a PR comment opening `Agent: net-engineer (W0-22)`, which is not an approval (D29). Both ADRs stay
Accepted under their own numbers; ADR 0002 ends at 60 lines or fewer (56) and ADR 0004 stays at 59.
`ci.yml` green on head. No `founder-approved`: `docs/adr/` is not a D24 path. Min is "-" because D31
retires founder minutes. It merges before W0-08's PR opens.

**W0-08: request 4.**
> Producer: say in the D31 rewrite of week-0.md when an unrun W0-08 spike carries over to week 1,
> now that D31 retires the cap at week-0.md:104-105. Task 1(b)'s "repro steps the founder can rerun
> in an hour" (server-engineer.md:178) is also a D31-retired clause; that file is a D24 path, so the
> founder edits it.

Producer, 2026-10-14: answered in the D31 retire-hours follow-up, not here. The carry-over rule
replaces W0-08's "Past the 4-hour cap" sentence (lines 104-105 at `9ed9034`) on
`claude/d31-retire-hours`, the D31 rewrite of this file, and takes effect when that PR merges. This entry does not repeat it, so
the two branches never edit the same lines. The rest of the answer, the draft-until-`client.log`
hold and the INCONCLUSIVE fallback, is in W0-08's description. "In an hour" at
server-engineer.md:178 is the founder's D24 edit, drafted on `claude/d31-build-agents` as "rerun
from the PR alone".

**W0-08: request 5.**
> Producer: does rule 6 apply to a throwaway-branch workflow (SteamCMD and `lib32gcc-s1`, Rules
> touched), and if it does, must its ADR merge before the founder re-runs the workflow?

Producer, 2026-10-14: no. Rule 6 does not apply to `claude/spike-sdr-server`'s workflow, so it needs
no ADR, and nothing has to merge before the founder re-runs it. Conditions: the branch is never a PR
and never merges, and nothing on main refers to it; W0-08's description keeps disclosing both
dependencies (SteamCMD from Valve's CDN, unpinned and self-updating, version 1791918946, with
`steamclient.so`'s SHA-256 printed every run; apt's `lib32gcc-s1`); if anything on main ever depends
on SteamCMD or its `steamclient.so` (a `ci.yml` job, a `tools/` script, a promoted spike workflow),
that PR's ADR merges first (README:10-11). Why: rule 6 and README:25 tie the ADR to a PR ("before
the PR that adds them"); README:10-11 defines a dependency, not when its ADR is due. Precedent:
W0-10's never-merged `claude/spike-wire-format` workflow used `flatc`, `flatbuffers`, and `bitflags`
before ADR 0004 merged (#15). W0-09's "no new dependency" was that item's own clause, not a reading
of rule 6.

**W0-08: request 6.**
> Producer: does server-engineer.md:131's integration-test line apply to a spike PR whose only
> possible test would need a Steam client (server-engineer.md:158)?

Producer, 2026-10-14: no, :131 ("An integration test in the diff that fails on the old code and
passes on the new") does not apply to W0-08. Its evidence is the run: CI building the example on
both runners, the runner's server log, the macOS build, and the founder's client log and round trip.
The rest of the definition of done holds: :130 (size, one concern, the five sections, checklists),
:132 (fmt and clippy `-D warnings` clean and `cargo test --workspace` green at head: clippy and test
on Windows and Linux, fmt on Linux only, ci.yml:46; clippy `--all-targets` and `cargo test` build
the example), and :133 (the ADR merged first, W0-22; one `steamworks` in `Cargo.lock`).
W0-08's Definition-of-done bullet ends "no integration test; answered as request 6, the run logs are
the evidence". W0-08 only: every PR into `crates/server/src` or `tests/` keeps :131 in full, the
future `SteamRelay` transport included (tested through `RawUdp`, :104); a later spike gets its own
answer in its queue item. Why: :131 and :158 cannot both hold for this PR, and the queue item
governs its acceptance: task 1(b) (:170-182) names logs, a round trip, and repro steps, not a test,
and the spike lives in `examples/`, never `src/` (:16-17). Writing the exemption into
server-engineer.md:131 would be the founder's D24 edit, if wanted.
