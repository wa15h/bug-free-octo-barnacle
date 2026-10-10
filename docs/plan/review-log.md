# Review log

Owned by the reviewer (`.claude/agents/reviewer.md`, "Owns"). Section 1 holds the review
checklists, section 2 the founder summary format. Section 3, the per-PR entries, and the weekly
reports are added by later PRs.

## 1. Review checklists

The six checklists from the reviewer agent file as D29 amended it, as blocks to paste into a PR
review. The check text is unchanged except for citations added where a check had no rule number
or decision ID; every check carries a hook tag. Per PR, paste the every-PR block and the block for
each crate or area the diff touches, tick what passes, and give every failing check its file and
line number (reviewer loop, step 5).

### Counting lines (D22) and the size bands (D6, Q4)

- Count additions plus deletions from `git diff --numstat origin/main...HEAD`. Hand-written lines
  count in every area, and evidence-area lines count the same as line-by-line lines (D22).
- Generated C# types, `Cargo.lock`, engine-generated scene and `.meta` files, and fixture files do
  not count; the PR lists them with their sizes and the reviewer spot-checks them (D22).
- 200 or fewer: reviewed. 201-400: returned with a split plan (Q4 default, D21). Over 400:
  returned unread, without opening the diff (D6).

### Hook tags

- `hook: exists`: a hook or CI gate on `main` runs the check. As of 2026-10-10 none does.
- `hook: requested`: the reviewer asks the `tools/` owner for this hook. The tag names the week-0
  queue item, or the script or workflow in `.claude/agents/devex-engineer.md`, that would run it.
  The tag becomes `exists` once that hook is on `main`.
- `hook: manual`: no hook is requested; the reviewer checks it by hand.
- A requested hook covers the mechanical part of a check; the reviewer checks the rest by hand.

### 1.1 Every PR

```markdown
**Checklist: every PR**
- [ ] 200 changed lines or fewer; over 400 is returned unread (D6, D22).
  `hook: requested` (W0-17 `pr-size.sh`)
- [ ] One concern (rule 4); pins (rule 8) and dependencies (rule 6) are their own PRs.
  `hook: requested` (`one-concern.sh`, `one-pin.sh`)
- [ ] Five template sections present and accurate (rule 5); "Read line by line" names file and
  function, never "see diff". `hook: requested` (P3 `pr-template.sh`)
- [ ] `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace` green on the head SHA (rule 7); C# builds warning-free for any client
  PR. `hook: requested` (W0-07 `ci.yml`, W0-16 `pre-push`; the dotnet job once `client/` exists)
- [ ] New dependency: its ADR in `docs/adr/` is already merged (rule 6).
  `hook: requested` (`adr-first.sh`)
- [ ] Any test modified, weakened, or deleted: "needs founder approval" on the line after
  `Agent: reviewer` in the summary (HS 7; Q7, no written default; D6 for the merge gate).
  `hook: requested` (`test-guard.sh`)
- [ ] Any gate, allowlist, `tools/size-exempt.txt`, bench budget, or review-log checklist line
  removed or loosened: "needs founder approval" on the line after `Agent: reviewer` in the
  summary, as for a test (D6 for every rule a hook, D22 for the exempt list). `hook: manual`
- [ ] Every changed file is under the queue item's paths; any other file is scope widening,
  returned (rule 4) and counted in the weekly report. `hook: manual`
- [ ] No file under `.claude/`, `CLAUDE.md`, `docs/research/`, or `docs/plan/decisions.md` unless
  the PR carries `founder-approved` (D24, D29). `hook: requested` (W0-21 `protected-paths.sh`)
- [ ] A choice resting on an open-question default cites its number (D21). `hook: manual`
- [ ] No claim in the description that you could not verify (rule 5). `hook: manual`
- [ ] The founder can explain the change from description plus diff (rule 9, D18). `hook: manual`
```

### 1.2 `crates/sim`

The I/O, RNG, fixed-point, and order checks cite rule 2, the two-process determinism they protect.
`sim-deps.sh` greps their identifiers from `sim-forbidden.txt` and reports a hit as `RULE 1`, the
row the devex gate map files it under.

```markdown
**Checklist: `crates/sim` (rules 1, 2; D3)**
- [ ] No engine types; no `async`, `.await`, `tokio`, `std::thread` (rule 1, D3).
  `hook: requested` (`sim-deps.sh`)
- [ ] No wall clock: no `Instant`, `SystemTime`, `chrono`, `time` on any path (rule 1).
  `hook: requested` (`sim-deps.sh`)
- [ ] No I/O: no `std::fs`, `std::net`, `std::env`, `println!` outside tests (rule 2).
  `hook: requested` (`sim-deps.sh`)
- [ ] RNG is seeded and passed in; no `thread_rng`, `rand::random`, `OsRng` (rule 2).
  `hook: requested` (`sim-deps.sh`)
- [ ] Economy math is fixed-point; every `f32` or `f64` in a serialized, hashed, or ledger type is
  flagged by line (rule 2). `hook: requested` (`sim-deps.sh`)
- [ ] Systems run from the explicit order list only; no iteration over `HashMap` or `HashSet` where
  order reaches state (`BTreeMap` or `Vec` instead) (rule 2). `hook: requested` (`sim-deps.sh`)
- [ ] A property test or replay fixture is in the diff and named under "Rules touched"; new
  invariants are added to the nightly run (rule 2). `hook: requested` (`sim-tested.sh`)
- [ ] Nightly invariant run (conservation of mass and money, two-process determinism) linked green,
  or the PR states it is not yet built (rule 2). `hook: requested` (`nightly.yml`)
```

### 1.3 `crates/proto`

```markdown
**Checklist: `crates/proto` (rule 3; D7)**
- [ ] Schema version bumped in the same PR as any message change (rule 3).
  `hook: requested` (`lockstep.sh`)
- [ ] Compatibility test added: the previous version decodes or is rejected with a clear message,
  never silently corrupted (rule 3). `hook: requested` (`lockstep.sh`)
- [ ] Fields only appended or given explicit ids; removed fields kept as `deprecated`; no type changed in place (rule 3, D30).
  `hook: manual`
- [ ] C# types regenerated in CI and `client/lib` compiles; no hand-written message type anywhere
  under `client/` (D7). `hook: requested` (`lockstep.yml`)
```

### 1.4 `crates/persist`

```markdown
**Checklist: `crates/persist` (rule 3; D8)**
- [ ] Every serialized struct change ships a migration and a fixture save from the previous version
  that round-trips (rule 3). `hook: requested` (`persist-schema.sh`)
- [ ] Old saves migrate or are rejected with a clear message; a test proves which (rule 3).
  `hook: manual`
- [ ] Saves are written at tick boundaries only (D8). `hook: manual`
- [ ] Catch-up is deterministic: a test runs it twice on the same input and compares hashes (D1,
  rule 2). `hook: manual`
```

### 1.5 `crates/server` authority paths

```markdown
**Checklist: `crates/server` authority paths (D8, D9)**
- [ ] Never trusts a client-reported quantity: position, velocity, inventory count, ledger amount,
  tick, timestamp, or "I hit X" are inputs to validate, never state to apply (D9). `hook: manual`
- [ ] Every client message is validated against server state before any sim call; a rejected input
  is observable in a log or metric (D9). `hook: manual`
- [ ] All state changes go through `crates/sim`; no game rule lives in the server crate (D3, D9).
  `hook: manual`
- [ ] No prediction, reconciliation, or rollback code path (D9).
  `hook: requested` (P0 `never-in-v1.sh`)
- [ ] Sidecar contract intact: control-socket exit, PID file, tick-boundary save (D8).
  `hook: manual`
```

### 1.6 `client/`

```markdown
**Checklist: `client/` (D4, D9)**
- [ ] No outcome-deciding logic: the client renders, interpolates, and sends inputs (D9).
  `hook: manual`
- [ ] `client/lib` references no engine namespace; the engine projects hold presentation only (D4).
  `hook: requested` (the client engineer's no-engine grep)
```

## 2. Founder summary format

One summary per review, posted as a "comment" PR review (reviewer loop, step 8: GitHub refuses
"approve" and "request changes" from a PR's author, and agents work under the founder's login,
D29). Under 40 lines, with no praise (Never do). Write it from what step 2 allows (the diff, the
description, the queue entry, and the files the diff touches), the Budget sources in 2.1, and the
evidence you verified on the head SHA (step 6); never from the writer's transcript. The section 1
blocks go into it in short form: passes under "Checked, not found", and each failing check after
`Fix:` with its file and line (step 5).

### 2.1 Fields

- Line 1 is `Agent: reviewer` (D29). Line 2 is the verdict: `ready for founder review`,
  `returned: <reason>`, or `held: founder review budget reached`; a return or a hold moves the PR
  back to draft. The reason is a weekly-report category: `size`, `concern`, `rule <n>` (a rule
  number or decision ID), `template`, or `evidence`.
- When either founder-approval check in section 1.1 applies, line 2 ends `; needs founder approval`.
- A return puts the smallest fix on line 3 after `Fix:`: the change and its rule number; for
  size or concern, the files for each new PR; for evidence, the exact command or artifact.
- Founder minutes use the queue's formula, plus 5 when the founder signs something (reviewer "How
  you work"; `week-0.md`, "Budget"). Hours come from `hours.md`, else the queue's planned hours.
  "Used" counts the PRs marked ready this week, this one only when this review marks it ready; a
  PR that would take "used" past the week's review minutes is held.

```markdown
Agent: reviewer
<verdict>[; needs founder approval]
Fix: <returns only>

**Size** <n> counted lines (D22): <l> line-by-line, <e> evidence; excluded <files with lines,
spot-checked | none>; founder minutes <arithmetic = m>; queue entry <class>, <lines> lines.
**Budget** week <w>, <h> h (<source>): <used> of <total> review minutes marked ready, this PR
<included | not added>; <left> left. <sent> of <line budget> lines.
**Concern** <queue ID>: <one sentence>. <Matches the queue entry | Differs: how>.
**Read line by line** <none, or one item per function, each with file and line range>
- `<path>:<start>-<end>` `<function>`: <the one thing to check>.
**Accept on evidence** <none, or one link per piece, on the head SHA (step 6)>
- <what> on head `<sha>`: <link, or while no CI exists, the pasted output tail (step 6)>.
**Rules touched** <rule n: the test, hook, or run that proves it, or the 1.x block by hand>.
**Flags** <cause of a founder approval, open-question default (D21), scope widening | none>.
**Risk and rollback** <what breaks, how it shows, how to revert>.
**Checked, not found**
- <section 1 block number>: <each check in it that passed>.
```

### 2.2 Over-400 return notice

Over 400 counted lines, the review is these three lines; line 3 is the one-line notice (step 1).
Its fix names no files: the diff stays unopened, not even to plan the split (step 1; Never do).

```markdown
Agent: reviewer
returned: size
Fix: split into PRs of 200 or fewer (Q4). Not read: <n> counted lines (D22), over 400 (D6).
```

### 2.3 Filled example (hypothetical)

A hypothetical `crates/sim` ledger PR: queue ID, files, lines, SHA, link, and numbers are invented.

```markdown
Agent: reviewer
ready for founder review

**Size** 115 counted lines (D22): 115 line-by-line, 0 evidence; excluded none; founder minutes
115 / 250 x 60 = 28; queue entry L, 120 lines.
**Budget** week 1, 30 h (`hours.md`): 466 of 720 review minutes marked ready, this PR included;
254 left. 1,640 of 3,000 lines.
**Concern** W1-99: `Ledger::transfer` moves money between two accounts and refuses an
overdraft. Matches the queue entry.
**Read line by line**
- `crates/sim/src/ledger.rs:30-57` `Ledger::transfer`: both `checked_` calls run before either
  balance changes, so an overdraft returns `Err(Overdraft)` and leaves both untouched.
- `crates/sim/src/ledger.rs:59-66` `Ledger::total`: sums the `BTreeMap` in key order.
- `crates/sim/src/systems.rs:22-30` `order`: `ledger_settle` runs after `jobs_tick`.
- `crates/sim/tests/ledger_props.rs:1-52` `transfer_conserves_total`: the total is the same after
  every transfer, refused or not.
**Accept on evidence**
- `ci.yml` fmt, clippy, and test, green on head `a1b2c3d`:
  https://github.com/<owner>/<repo>/actions/runs/1234567890
**Rules touched** Rule 1: the 1.2 block by hand (`sim-deps.sh` not yet on `main`). Rule 2:
`transfer_conserves_total`. Rule 7: the CI run above.
**Flags** none.
**Risk and rollback** A transfer that debits without crediting breaks conservation; the
property test fails on it. Revert the PR; no message or save format changes.
**Checked, not found**
- 1.1: size, one concern, template, rule 7, no new dependency, no test edited, no gate loosened,
  files in the queue paths, no protected path, no open-question default, claims verified,
  explainable from description and diff.
- 1.2: no engine type, `async`, thread, wall clock, I/O, or unseeded RNG; no float; no `HashMap`
  or `HashSet` iteration; property test present; nightly run not built, and the PR says so.
```
