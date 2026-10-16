WIP: 10

# Queue: weeks 1-6

Producer, first task 1. Week 0 is `docs/plan/week-0.md`; from week 1 the queue is this file. It holds no
founder hours, dates, or forecasts (D31): items are pulled in the order below, and none is promised to a week.
Week 1 is given in full. Weeks 2-6 do not fit in full under the 200-line cap (D22, rule 4), so they follow as
one ordered pull list: each row has the eight fields, acceptance by reference to the owner's first-task list,
and the gate it serves. Each weekly queue PR reconciles this file and promotes, splits, or reorders rows.

How to pull: read line 1 first. Take the first item in order that you own and whose After items have all merged, one
at a time. Open agent PRs never pass n, `held:` drafts included (`producer.md`, The WIP limit); under a
`WIP: 0 FROZEN: <reason>` line open nothing for review. At week start W0-08's draft, W0-18, and W0-21 hold 3 of the
10 slots. No deload falls in weeks 1-6; the first is weeks 12-13 (`producer.md`, Rest). Order (D27; `producer.md`,
Gates): Gate 0's items lead, the two-client proof's follow, and tooling Gate 0 does not need waits behind both.
Gate 0 fires when N-08 and N-09 are on main with W1-07 merged; its path: W1-01 to W1-10, N-01 to N-10.

Class: L line-by-line, E evidence, F founder-authored (no `Agent:` line, D29; never counts toward n, D31).
Lines: expected counted lines (D22). "Per <owner> N": that owner's first task N in `.claude/agents/`, plus the note.
ADR numbers 0005-0012 are reserved here (W1-19); the producer's stub is the first commit of each ADR PR.

## Carried over from week 0

Merged in week 0: W0-01 to W0-07, W0-09 to W0-17 (W0-11 then retired by D31), and W0-22.

| ID | Owner | State at week start | Next |
|---|---|---|---|
| W0-08 | server-engineer | Draft, holding a slot; waits on the founder's Mac client run | PASS or FAIL in the PR, then D3's line (W0-19); no transport work before it (N-20) |
| W0-18 | devex-engineer | In progress, holding a slot | Review and merge; N-43 waits on it |
| W0-21 | devex-engineer | In progress, holding a slot | Review; the founder labels its throwaway PR |
| W0-19 | founder | Open. D3's line waits on W0-08; D8's line can land now in its own founder PR (W0-19's text) | D8's line unblocks N-11 and N-12 |
| W0-20 | founder | Open; W0-16 has merged, so it waits on nothing | - |
| W0-12 | founder | `client/BAKEOFF.md` merged unsigned | The signature is N-21; engine pin and rung PRs wait on it (D31) |
| P0 P2 P3 | devex, reviewer | Not pulled forward | Now W1-16, W1-15, W1-17 |
| Setup | founder | `week-0.md`'s founder setup | If not done: require `ci.yml` and `pr-gates.yml`; run `tools/hooks/install.sh` |

## Week 1

| ID | Owner | PR | Class | Lines | After | Decisions | ADR-first | Gate |
|---|---|---|---|---|---|---|---|---|
| W1-01 | sim-engineer | Sim skeleton: `Fixed`, `Rng`, `State`, `Input`, `Event`, `tick`, `SYSTEM_ORDER` | L | 190 | - | D3 D9 D27 | no: zero dependencies | Gate 0 |
| W1-02 | sim-engineer | `state_hash` and `determinism_two_runs` | L | 90 | W1-01 | D3 D27 | no | Gate 0 |
| W1-03 | sim-engineer | ADR 0005: `proptest`, a sim dev-dependency | E | 40 | - | D3 D17 | is the ADR | Gate 0 |
| W1-04 | sim-engineer | ADR 0006: `serde` derive on `State` | E | 40 | - | D3 D8 D17 | is the ADR | Gate 0 |
| W1-05 | game-designer | GDD, the hearth half | E | 150 | - | D1 D2 D26 D27 | no | Gate 0 |
| W1-06 | game-designer | `content/README.md`: the table rules | E | 80 | - | D2 D18 D26 | no | Gate 0 |
| W1-07 | game-designer | Gate 0 playtest script | E | 90 | - | D10 D27 D28 D31 | no | Gate 0 |
| W1-08 | game-designer | `content/jobs.csv` and `content/ledger.csv` | E | 40 | W1-05 W1-06 | D2 D26 D27 | no | Gate 0 |
| W1-09 | sim-engineer | Ledger and `check_invariants` | L | 180 | W1-02 W1-03 W1-04 | D2 D3 D27 | 0005, 0006 | Gate 0 |
| W1-10 | test-engineer | ADR 0007: the settlement viewer | E | 40 | W1-07 | D13 D27 | is the ADR | Gate 0 |
| W1-11 | marketing | `docs/plan/marketing/thursday-group.md` | E | 100 | - | D10 D28 D31 | no | Gate 0 testers; Gate 2 |
| W1-12 | devex-engineer | Pin `flatbuffers` 25.2.10; install `flatc` in CI and on clones | E | 90 | - | D7 D17 D30 | 0004 | Two-client proof |
| W1-13 | net-engineer | `crates/proto` skeleton | L | 190 | W1-12 | D7 D30 | 0004 | Two-client proof |
| W1-14 | art-director | Kit shortlist with licences | E | 150 | - | D12 D26 | no | Bake-off rung 2 |
| W1-15 | reviewer | Review-log section 3, the hook request list (was P2) | E | 60 | - | D6 | no | Rules 1-9 hooks |
| W1-16 | devex-engineer | `never-in-v1.sh` (was P0) | E | 90 | - | D2 D4 D8 D9 D14 | no | Cut-scope hook |
| W1-17 | devex-engineer | `pr-template.sh` (was P3) | E | 60 | W1-15 | D6 | no | Rule 5 hook |
| W1-18 | reviewer | Week 1's review-log entries and weekly report | E | 60 | week 1's last review | D6 D31 | no | Saturation counter |
| W1-19 | producer | ADR README: reserve 0005-0012 | E | 15 | - | D17 D24 | no | - |

**W1-01.** Paths: `crates/sim/src/{lib,fixed,rng,state}.rs`, `crates/sim/tests/skeleton.rs`. Accept per sim 1,
with `state_hash` split to W1-02 as task 1 allows: `[dependencies]` empty and `Cargo.lock` unchanged;
`order_is_complete` passes; `Rng` reproduces xoshiro256**'s published reference outputs; a seeded loop over
10,000 pairs shows each `Fixed` op equals `i128` math or returns `Err`, never panics (rule 2 before 0005); the
determinism checklist pasted, every ripgrep empty. The description names the scale constant (founder question 2).

**W1-02.** Paths: `crates/sim/src/hash.rs`, `lib.rs`, `crates/sim/tests/determinism.rs`. Accept: FNV-1a over a
canonical byte walk of `State`, under 60 lines (`sim-engineer.md`, rule 6); `determinism_two_runs` holds at
every tick of a 10,000-tick seeded run; changing one field of each kind changes the hash; checklist pasted.

**W1-03, W1-04.** Paths: `docs/adr/0005-proptest.md`; `docs/adr/0006-serde.md`. Accept per sim 2: each under 40
lines; one exact version, and every package new to `cargo metadata` with version, crates.io owner, and licence
(README, as ADR 0002 lists its eight); Consequences says what breaks if it goes. 0005: `crates/sim` dev-dependency
only. 0006: derive on `State` for persist (Q11 default); the save encoding is ADR 0008 (N-15). No `Cargo.toml` edit.

**W1-05.** Paths: `docs/design/gdd.md`. Accept per designer 1, hearth half (the task's own split; runs half is
N-28): pillars (MR 2), the loop, the hearth (three jobs, ledger, blueprints); each section cites a decision ID;
the deferred list is exactly D2's, citing D31; setting terms only from D26's glossary; 200 lines or fewer.

**W1-06.** Paths: `content/README.md`. Accept per designer 2, table rules only (`legibility.md` is N-03,
`prohibitions.md` N-31; rule 4): each rule checkable by reading a row; each table states its row-count rule
(three jobs, twelve stages, one enemy family: D2) and carries a `why` column.

**W1-07.** Paths: `docs/design/playtests/gate-0-ant-farm.md`. Accept per designer 5: the pass line is
`gates.md`'s Gate 0 rule word for word (Q26 default); a stranger can run it; it names what the viewer must show
for each question it asks, so W1-10, N-08, and N-09 are built to it.

**W1-08.** Paths: `content/jobs.csv`, `content/ledger.csv`. Accept per designer 3, except its loader and
conservation clauses move to N-01 and N-02, whose tests load these files (no loader exists yet): rows Salvager,
Grower, Builder; columns as W1-06 sets them; each `why` names the session shape it serves (MR 2).

**W1-09.** Paths: `crates/sim/src/ledger.rs`, `lib.rs`, `crates/sim/Cargo.toml` (`proptest` dev-dependency at
0005's version), `crates/sim/tests/ledger.rs`. Accept per sim 3: the proptest keeps the conservation sum and never
panics; a rejected transfer leaves both balances untouched; `check_invariants` returns `Err` naming the account on
a planted imbalance (the first nightly invariant); `Cargo.lock` gains only 0005's packages; checklist pasted.

**W1-10.** Paths: `docs/adr/0007-settlement-viewer.md`. Accept: egui window or terminal, chosen against W1-07 and
"read from across the room" (`game-designer.md`:100); under 40 lines; every new package with version, owner, and
licence; only `crates/tools` takes it; the viewer reads `sim::view`, drives `sim::tick`, and re-implements no rule.

**W1-11.** Paths: `docs/plan/marketing/thursday-group.md`. Accept per marketing 4; until `gate-2-thursday.md`
exists the designer confirms the session note against W1-07 and `gates.md`'s Gate 2; Q34's default; no tester name.

**W1-12.** Paths: `tools/pins.toml`, `tools/checks/pins.sh`, `.github/workflows/ci.yml`, one install script under
`tools/`. Accept: ADR 0004's devex follow-up: the `flatbuffers` row set to 25.2.10 with both flatc zips' SHA-256
from ADR 0004; CI installs flatc on both runners by hash and prints its version; a wrong hash goes red naming the
zip (linked); one documented command gives a clone the same flatc; `pins.sh` green on head. Pin-only (rule 8).

**W1-13.** Paths: `crates/proto/` (`Cargo.toml`, `build.rs`, `schema/`, `src/lib.rs`, `examples/make_fixture.rs`,
`fixtures/v1/`, `tests/`). Accept per net 2: `build.rs` fails unless flatc reports 25.2.10; `fixtures/v1/schema.bfbs`
frozen; the four compatibility tests, `schema_is_frozen` red run linked; `Cargo.lock` gains only ADR 0004's packages.

**W1-14.** Paths: `content/kits/shortlist.md`. Accept per art-director 1. No purchase: the buy is N-34 (D12).

**W1-15.** Paths: `docs/plan/review-log.md`, section 3. Accept per reviewer 6; every `hook: requested` tag in
section 1 has its request (command, expected message, rule number).

**W1-16.** Paths: `tools/checks/never-in-v1.sh`, a job in `pr-gates.yml`, one line in `tools/hooks/pre-push`.
Accept: each item in `devex-engineer.md`:77 goes red on a throwaway branch with `RULE <id>: <what> -- <fix>`
(linked); `docs/` is not scanned; green on head.

**W1-17.** Paths: `tools/checks/pr-template.sh`, a job in `pr-gates.yml`. Accept per W1-15's request: a
description missing one of the five headings, out of order, or with one empty goes red naming it; red and green linked.

**W1-18.** Paths: `docs/plan/review-log.md`. Accept per reviewer 5: every hand-off field has a number; one PR.

**W1-19.** Paths: `docs/adr/README.md`. Accept: the reservation table gains 0005 W1-03, 0006 W1-04, 0007 W1-10,
0008 N-15, 0009 N-11, 0010 N-17, 0011 N-22, and 0012 N-24 with owners; next free reads 0013; nothing else changes.

## Weeks 2-6: pull order

D3's and D8's lines are W0-19's (founder). A row's Accept adds to its owner's task; "-" in After means ready.

| ID | Owner | PR and paths | Class | Lines | Accept | Decisions | ADR-first | After | Gate |
|---|---|---|---|---|---|---|---|---|---|
| N-01 | sim-engineer | Job and ledger table structs and loaders, `crates/sim/src/tables.rs` | L | 170 | per sim 4, the loader: `&str` in, `Result` out; a fourth job row is a load error; a test loads W1-08's files | D2 D3 | 0005 | W1-08 W1-09 | Gate 0 |
| N-02 | sim-engineer | Settlers, assignment, work-tick progress, `crates/sim/src/settlers.rs` | L | 180 | per sim 4: every input and output a ledger transfer; proptest: no unit created or lost over a random schedule | D2 D3 D27 | 0005 | N-01; founder question 3 | Gate 0 |
| N-03 | game-designer | `docs/design/legibility.md` | E | 100 | per designer 2: each rule checkable on a screenshot | D13 D18 | no | W1-05 | Gate 0 |
| N-04 | game-designer | `content/buildings.csv` | E | 30 | per designer 4 | D2 D26 | no | W1-06 | Gate 0 |
| N-05 | game-designer | `content/blueprints.csv` | E | 30 | per designer 4 | D2 D26 | no | N-04 | Gate 0 |
| N-06 | sim-engineer | Buildings and blueprints, `crates/sim/src/buildings.rs` | L | 180 | `sim-engineer.md` Buildings: construction is the Builder's job; proptest conserves; a test loads N-04 and N-05 | D2 D3 | 0005 | N-02 N-05 | Gate 0 |
| N-07 | sim-engineer | `view`, `HearthView`, the `expedition` default feature: `crates/sim/src/view.rs`, `Cargo.toml` | L | 120 | per sim 4: `view` and `--no-default-features` | D3 D27 | no | N-06 | Gate 0 |
| N-08 | test-engineer | Viewer: settlers and jobs, `crates/tools/src/bin/viewer.rs` | E | 180 | built to W1-07 and N-03 from `sim::view`, seeded; red and green runs; its README command | D13 D27 | 0007 | W1-10 N-07 | Gate 0 |
| N-09 | test-engineer | Viewer: the printed ledger panel, `crates/tools/src/bin/viewer.rs` | E | 150 | what W1-07 asks of the ledger; evidence as N-08 | D27 | 0007 | N-08 | Gate 0 |
| N-10 | game-designer | Gate 0 sheets, `docs/design/playtests/results/` | E | 60 | W1-07 run as written, four or more testers; the founder's verdict | D10 D28 D31 | no | N-09 W1-11 | Gate 0 evidence |
| N-11 | server-engineer | ADR 0009: `tokio`, `docs/adr/0009-tokio.md` | E | 40 | per server 3, its first PR | D3 | is the ADR | D8's line | Two-client proof |
| N-12 | server-engineer | `crates/server/SIDECAR.md` | E | 150 | per server 2; Q16 defaults | D8 | 0003 | D8's line | Two-client proof |
| N-13 | net-engineer | SIDECAR.md's control messages: `crates/proto/schema/`, `tests/`, `fixtures/` | L | 120 | every message with its fields; bump and fixture (rule 3, D30) | D7 D8 D30 | 0004 | W1-13 N-12 | Two-client proof |
| N-14 | server-engineer | Binary skeleton: `crates/server/src/{main,control,watchdog}.rs`, `tests/` | L | 190 | per server 3 | D3 D8 | 0009 | N-11 N-13 | Two-client proof |
| N-15 | persistence-engineer | ADR 0008: save encoding, `docs/adr/0008-save-encoding.md` | E | 60 | per persist 1; Q11 default | D3 D8 | is the ADR | W1-04 | Two-client proof |
| N-16 | persistence-engineer | Envelope, atomic write, loader: `crates/persist/src/{envelope,write,load}.rs` | L | 190 | per persist 2 | D8 | 0008 | N-15 | Two-client proof |
| N-17 | devex-engineer | ADR 0010: the .NET SDK pin and its CI action, `docs/adr/0010-dotnet-sdk.md` | E | 40 | one SDK version for `client/global.json`; any action at a full SHA (ADR 0001) | D7 D17 | is the ADR | - | Bake-off |
| N-18 | devex-engineer | Codegen skeleton: `client/Hearth.sln`, `client/Directory.Build.props`, `client/global.json`, `tools/codegen/`, `lockstep.yml` | E | 180 | per devex 5 | D7 D17 D30 | 0004, 0010 | W1-13 N-17 | Bake-off |
| N-19 | client-engineer | `client/lib` skeleton: `client/lib/`, `client/lib.Tests/` | E | 190 | per client 1 | D4 D7 D30 | 0004 | N-18 | Bake-off |
| N-20 | server-engineer | `Transport` trait and `RawUdp`, `crates/server/src/transport/` | E | 180 | per server 5 | D3 D9 | 0009 | D3's line, N-14 | Two-client proof |
| N-21 | founder | Sign the scorecard in `client/BAKEOFF.md` | F | 5 | the founder's own PR (D31) | D4 D31 | no | - | Bake-off |
| N-22 | devex-engineer | ADR 0011: the Godot 4.7 .NET minor-line pin, `docs/adr/0011-godot-pin.md` | E | 40 | one minor line (D17) | D4 D17 | is the ADR | N-21 | Bake-off |
| N-23 | devex-engineer | Godot pin-only PR: `tools/pins.toml` and its mirror | E | 30 | rule 8 | D17 | 0011 | N-19 N-22 | Bake-off |
| N-24 | devex-engineer | ADR 0012: the Unity 6 LTS line pin, `docs/adr/0012-unity-pin.md` | E | 40 | one LTS line (D17) | D4 D17 | is the ADR | N-21 | Bake-off |
| N-25 | devex-engineer | Unity pin-only PR: `tools/pins.toml` and its mirror | E | 30 | rule 8 | D17 | 0012 | N-19 N-24 | Bake-off |
| N-26 | client-engineer | Godot rung 1, `client/godot/` | E | 190 | per client 4 | D4 D13 | 0011 | N-23 | Bake-off |
| N-27 | client-engineer | Unity rung 1, `client/unity/` | E | 190 | per client 5 | D4 D13 | 0012 | N-25 | Bake-off |
| N-28 | game-designer | GDD, the runs half, `docs/design/gdd.md` | E | 150 | per designer 1 | D2 D9 D26 | no | W1-05 | Gate 1 |
| N-29 | game-designer | `content/cordon.csv` | E | 40 | per designer 6 | D14 D26 | no | W1-06 | Gate 1 |
| N-30 | sim-engineer | Cordon calendar, `crates/sim/src/cordon.rs` | L | 180 | per sim 5 | D26 | 0005 | N-07 N-29 | Gate 1 |
| N-31 | game-designer | `docs/design/prohibitions.md` | E | 60 | per designer 2 | D2 D14 D26 | no | W1-06 | Cut scope |
| N-32 | art-director | `docs/design/art-direction.md`, the look | E | 150 | per art-director 4, first PR | D2 D9 D13 D14 D26 | no | - | Bake-off rung 2 |
| N-33 | art-director | `content/palette/` | E | 50 | per art-director 2 | D26 | no | N-29 | Bake-off rung 2 |
| N-34 | founder | Buy the recommended kit (spend); no file | F | 0 | the founder buys; no pack file is committed (Q30) | D12 | no | W1-14 | Bake-off rung 2 |
| N-35 | art-director | `content/kits/README.md` and the kit's records | E | 150 | per art-director 3 | D12 D17 | Q30 default | N-34 | Bake-off rung 2 |
| N-36 | test-engineer | Simulator skeleton, `crates/tools/src/bin/sim.rs` | E | 190 | per test 1 | D3 | no | W1-09 | Rule 2 nightly |
| N-37 | devex-engineer | Sim allowlist: `tools/allowlist/sim.toml`, `tools/checks/sim-deps.sh` | E | 150 | per devex 6, listing 0005 and 0006 | D3 | no | W1-03 W1-04 | Rule 1 hook |
| N-38 | devex-engineer | Hearth-only CI step, `.github/workflows/ci.yml` | E | 20 | `cargo test -p sim --no-default-features` on both runners | D27 | no | N-07 | Rule 7 hook |
| N-39 | devex-engineer | `tools/checks/wip-limit.sh`, a `pr-gates.yml` job | E | 90 | per W1-15's request; first, as D31's only limit | D31 | no | W1-15 | WIP limit hook |
| N-40 | devex-engineer | `tools/checks/one-concern.sh`, a `pr-gates.yml` job | E | 90 | per W1-15's request | D6 | no | W1-15 | Rule 4 hook |
| N-41 | devex-engineer | `tools/checks/adr-first.sh`, a `pr-gates.yml` job | E | 90 | per W1-15's request | D17 | no | W1-15 | Rule 6 hook |
| N-42 | devex-engineer | `tools/checks/one-pin.sh`, a `pr-gates.yml` job | E | 60 | per W1-15's request | D17 | no | W1-15 | Rule 8 hook |
| N-43 | reviewer | Dry run: eight planted violations, `docs/plan/review-log.md` | E | 60 | per reviewer 3; test-engineer writes the synthetic PR, closed unmerged | D6 | no | W0-18 | Review gate |
| N-44 | marketing | `docs/devlog/` README and first outline | E | 120 | per marketing 3 | D2 D31 | no | D3's line | - |
| N-45 | founder | Book architecture review 1 (spend); no file | F | 0 | the founder books and pays; the producer queues each finding | D12 | no | D3's line | Architecture review 1 |

Held, not queued: steam-release's tasks, marketing tasks 1, 2, 5, 6, and art-director task 5 (the capsule brief)
wait for the two-player loop (`producer.md`, Never do; founder question 4); `test-guard.sh` waits on Q7.

## Open questions for the founder

1. Week-0 question 4, still open: the slip rule (`producer.md`, The weekly loop, step 5). W0-08 reaches its
   second reconcile at week 1's end, waiting only on your run. Is such an item exempt? A yes goes in
   `producer.md` (D24); until then the rule applies as written, and nothing is cut (D28).
2. Q10 has no written default: does `Fixed`'s scale and width need an ADR or a decision row? W1-01 proceeds
   without one (`docs/adr/README.md`: the sim skeleton needs none); a yes merges before proto carries `fixed_scale` (net 3).
3. Q8's default sets one `TICK_HZ` and one `HEARTH_EVERY` but no values, and no decision or design note gives
   them. N-02 needs both to turn W1-08's work times into ticks and waits on your answer in `open-questions.md`.
4. The capsule brief: Q31's default puts it in weeks 0-6; `producer.md` holds page preparation until the
   two-player loop. This queue holds it; say if Q31's default wins.
5. Q7 has no written default, so `test-guard.sh` is not queued. Q18 stays open; nothing here waits on it.

## Questions

Agents add `<ID>: question` here. The producer answers it or parks the item and asks the founder above.
