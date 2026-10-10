# Review log

Owned by the reviewer (`.claude/agents/reviewer.md`, "Owns"). Section 1 holds the review
checklists. Sections 2 and 3, the per-PR entries, and the weekly reports are added by later PRs.

## 1. Review checklists

The six checklists from the reviewer agent file, as blocks to paste into a PR review. The check
text is unchanged except for a citation added where a check had none; every check carries a hook
tag. Per PR, paste the every-PR block and the block for each crate or area the diff touches, tick
what passes, and give every failing check its file and line number (reviewer loop, step 5).

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
  queue item, or the script in the devex engineer's gate map (`.claude/agents/devex-engineer.md`),
  that would run it. The tag becomes `exists` once that hook is on `main`.
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
- [ ] Any test modified, weakened, or deleted: "needs founder approval" at the top of the summary
  (HS 7; D6, Q7). `hook: requested` (`test-guard.sh`)
- [ ] Any gate, allowlist, `tools/size-exempt.txt`, bench budget, or review-log checklist line
  removed or loosened: "needs founder approval" at the top of the summary, as for a test (D6,
  D22). `hook: manual`
- [ ] Every changed file is under the queue item's paths; any other file is scope widening,
  returned (rule 4) and counted in the weekly report. `hook: manual`
- [ ] No file under `.claude/`, `CLAUDE.md`, `docs/research/`, or `docs/plan/decisions.md` unless
  the founder authored the PR (D24). `hook: requested` (W0-21 `protected-paths.sh`)
- [ ] A choice resting on an open-question default cites its number (D21). `hook: manual`
- [ ] No claim in the description that you could not verify (rule 5). `hook: manual`
- [ ] The founder can explain the change from description plus diff (rule 9, D18). `hook: manual`
```

### 1.2 `crates/sim`

```markdown
**Checklist: `crates/sim` (rules 1, 2; D3)**
- [ ] No engine types; no `async`, `.await`, `tokio`, `std::thread` (rule 1, D3).
  `hook: requested` (`sim-deps.sh`)
- [ ] No wall clock: no `Instant`, `SystemTime`, `chrono`, `time` on any path (rule 1).
  `hook: requested` (`sim-deps.sh`)
- [ ] No I/O: no `std::fs`, `std::net`, `std::env`, `println!` outside tests (rule 2, D3).
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
- [ ] Field numbers never reused; removed fields reserved (rule 3, D7). `hook: manual`
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
