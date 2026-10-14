# The Reclaimers (working title)

A player-hosted 1-4 player online co-op scavenging game with a settlement that keeps
working between sessions, set in an overgrown city sealed after a long-over sickness, whose
ring fence lifts sector by sector each week (D26). Steam first,
premium price, Rust authoritative server, C# client. Built by one founder with a day job
and a team of coding agents. Agents write all code; the founder reviews everything.

Read `docs/plan/decisions.md` before starting any task. It is the decision log and it
wins over anything you remember about this project. The research behind it is in
`docs/research/` (market research, then the hiring-and-stack re-evaluation).

## Shipped scope for v1 (the cut scope, decided)

- One cordon season (12 real weeks), one region, one modular ruin kit, one enemy family
  (a wild boar sounder). Every setting term comes from the glossary under D26.
- The hearth: three settler jobs, a printed ledger, buildings from blueprints.
- Out-and-back runs of 30-90 minutes; slow, telegraphed, positioning-first combat.
- 2-player co-op over a listen server the host's client spawns as a sidecar process.
- Offline progression as a deterministic catch-up function, identical on every server.
- Deferred until a founder decision row names it (D31): 4-player drop-in, the free headless
  dedicated binary, the Hospital Quarter mode, a second kit, the Boroughs meta-layer,
  localization. Never in v1: client prediction or rollback, strangers, free text between
  players, in-game voice promises.

## Founder constraints that shape every task

- Founder hours are not part of the plan (D31). Nothing logs, estimates, budgets, forecasts,
  or triggers on them; every choice is made on evidence and merit, toward the target state:
  the cut scope (D2) at full quality (D28). No dates and no forecasts: gates fire when their
  evidence exists on main and the founder gives every verdict; the page, demo, and early
  access go live when their checklists are true and the founder publishes.
- Review: at most n open agent PRs (`WIP: <n>` on the queue's first line; n = 10 until the
  founder sets another; founder PRs never count). A ready PR that has waited more than five
  days freezes the queue until none has waited more than two. Each PR is 200 changed lines
  or fewer (hard cap 400; D6, D22). Review bandwidth is the binding constraint of the whole
  project. A PR that is too big does not get reviewed; it gets split.
- Rest: a deload fortnight every 12 weeks from week 0, and the founder may pause the queue
  at any time without a reason. Deload and pause days stop every waiting clock.
- Line-by-line review areas: `crates/sim`, `crates/proto`, `crates/persist`, and the
  authority paths of `crates/server`. The founder reads every line. Keep these crates
  small, boring, and heavily tested.
- Evidence-reviewed areas: client, tooling, content, docs. The founder accepts these on
  evidence: green CI, screenshots or video, metrics, and the reviewer agent's summary.
- Releases, Steam uploads, and anything that spends money are human-only actions.

## Stack (decided unless marked pending)

- Server: a plain Rust `tokio` binary. Transport over Steam Datagram Relay through
  `steamworks-rs` GameServer sockets, with a raw-UDP fallback if the week-0 spike fails.
- Simulation: engine-free Rust crates shared by every deployment. Fixed-point economy
  math, explicitly ordered systems, seeded RNG, no floating point crossing a platform
  boundary, replayable from an append-only input log.
- Wire format: schema-owned and cross-language FlatBuffers (D30). The Rust `proto` crate
  owns the schema; C# types are generated from it in CI. Never hand-write message types twice.
- Client: C# on an engine-agnostic .NET Standard 2.1 class library (protocol decode,
  Steam via Steamworks.NET or Facepunch.Steamworks, interpolation buffers, sidecar
  launch and watchdog, typed events). The engine project is a thin presentation layer.
- Engine: PENDING a weeks 0-6 bake-off between Godot 4.7 C# and Unity 6 on an identical
  vertical slice, decided by the founder's written verdict on merit against a scorecard
  signed in `client/BAKEOFF.md` before either engine project exists; a tie goes to Unity 6 (D31).
  Not Bevy. Not Unreal. No Rust GDExtension on the critical path.
- Netcode: server-authoritative, interpolation only, tuned for 80-150 ms. No prediction.
- Listen server: the client spawns the server binary as a sidecar process and owns a
  watchdog (server exits on a silent control socket; client kills the recorded PID on
  clean exit; saves are written at tick boundaries). No in-process server, ever.
- Camera: third-person follow camera by default, with the top-down diorama as the
  settlement management view; locked by a prototype of both, the first client item after
  the bake-off result.
- Pins: one Rust toolchain, one Steamworks SDK version across the Rust binding, the C#
  binding, and the redistributable, one FlatBuffers version across `flatc`, the crate, and
  the C# runtime (D30), one engine minor line. At most two engine minor
  upgrades a year, never shipping on a line that has lost patch support.

## Repository layout

```
crates/sim        deterministic simulation (settlers, jobs, ledger, cordon calendar, combat rules)
crates/proto      schema-owned wire protocol and versioning; generates C# types in CI
crates/net        replication, snapshots, interest filtering, interpolation inputs
crates/persist    saves, versioned migrations, catch-up, migration-fixture harness
crates/server     the authoritative tokio binary, Steam GameServer transport, sidecar contract
crates/tools      bots, headless harnesses, replay tools
client/lib        the engine-agnostic C# library
client/godot      Godot 4.7 C# presentation project (bake-off)
client/unity      Unity 6 presentation project (bake-off)
content/          data tables (settler jobs, buildings, kits, ledger numbers), palette, kit import rules
docs/research     the two research reports
docs/plan         decision log, milestones, weekly queues
docs/design       the game design document and tuning notes
docs/adr          architecture decision records, one file per decision
tools/            git hooks, CI scripts, PR checks
```

## Engineering rules (each one becomes a hook or CI gate; `tools/` owns them)

1. No engine types, no async, and no wall clock inside `crates/sim`.
2. Every simulation change ships with a property test or a replay fixture, and the
   nightly invariant run (conservation of mass and money, determinism across two
   processes) stays green.
3. Protocol changes bump the schema version and ship a compatibility test. Old saves
   and old clients are either migrated or rejected with a clear message, never corrupted.
4. A PR touches one concern. If it needs a second concern, it is two PRs.
5. The PR description uses the template in `docs/plan/pr-template.md`: what to read line
   by line, what to accept on evidence, which rules were touched, and the risk.
6. New dependencies need an ADR in `docs/adr/` before the PR that adds them.
7. `cargo fmt`, `cargo clippy -D warnings`, and `cargo test --workspace` pass before any
   PR is opened. The C# library builds warning-free before any client PR is opened.
8. Pinned versions change only in a PR whose sole concern is the upgrade.
9. Anything the founder cannot explain after reading it does not merge. Prefer the
   simpler design that the founder can explain to the clever one they cannot.

## How the team works

- The `producer` agent turns the plan into a weekly queue in priority order.
  Every other agent takes work from that queue, one PR at a time.
- The `reviewer` agent pre-reviews every PR before the founder sees it and writes the
  founder-facing summary. Nothing reaches the founder without it.
- The producer writes the queue's first line (D31), and every agent reads it before starting
  an item. A PR finished while the WIP cap is full waits as a draft marked
  `held: WIP limit reached`. While it says `WIP: 0 FROZEN: <reason>` (founder backlog, deload,
  or founder pause), no agent opens anything for review.
- Agents work under the founder's GitHub login; there is no separate agent account (D29). Agent
  commits are authored `Claude`, agent branches start `claude/`, and every agent PR description,
  issue, and comment opens with `Agent: <name>`. Agents never approve, merge, or apply or remove the
  `founder-approved` or `upgrade` labels. `.claude/settings.json` denies the direct merge and
  label tools, but it is a guard, not a boundary: this rule is what holds.
- Dates slip; scope and quality never do (D28). No agent proposes a shortcut or a cut to hold
  a date.
- Nothing deferred is built, promised, or translated until the founder adds a decision row
  naming it (D31).
- Agents ask the producer when a task is unclear, and never widen scope on their own.
- Agents never claim a result they did not verify. A failing test is reported as failing.
- Agents never log, estimate, or forecast founder hours (D31). They report their own work in
  changed lines and test evidence.
- Agents never edit `.claude/`, this file, `docs/research/`, or `docs/plan/decisions.md`
  (D24). Open questions follow the defaults in `docs/plan/open-questions.md` (D21).

## Commands

```
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p tools --bin headless-session   # two-client authoritative session, once it exists
dotnet build client/lib -warnaserror         # once the client library exists
```
