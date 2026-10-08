# Decision log

Every entry cites the report and section it came from. `docs/research/market-research.md`
is MR; `docs/research/hiring-and-stack.md` is HS. Status is Decided, Default (decided
unless the founder objects), or Pending (a spike or gate decides).

| ID | Decision | Status | Source |
|---|---|---|---|
| D1 | Concept: Longthaw, staged. Player-hosted 1-4 player scavenging game with an offline-progressing settlement and a thaw-calendar hook. Ashward (developer-hosted persistent valley) is the fallback scope cut, not the plan. | Default | MR 1, 7, 8.1; HS 8 |
| D2 | Shipped scope for v1 is the cut scope: one season, one kit, one region, one enemy family, three settler jobs, 2-player listen server only. Everything else is earned by 18+ logged hours a week for six consecutive months. | Decided | HS 6, 9.10 |
| D3 | Server is Rust: a plain tokio binary over Steam Datagram Relay via steamworks-rs GameServer sockets, engine-free sim/proto/net/persist crates, raw-UDP fallback. One-day SDR spike on app 480 before week 1. | Decided | MR 7; HS 5, 9.4 |
| D4 | Client is C# on an engine-agnostic .NET Standard 2.1 library. Engine chosen by a weeks 0-6 bake-off between Godot 4.7 C# and Unity 6 on logged founder hours; tie within 15% goes to Unity 6. Bevy and Unreal are out. No Rust GDExtension on the critical path. | Pending bake-off | HS 5, 9.2 |
| D5 | Hours: 30 a week on the project, by founder decision on 2026-10-08 (revised from a 15-hour base). The founder accepted the health risk that HS 6 names for 70+ total weekly hours with a day job. Guardrails: 42 project weeks a year, a deload fortnight every 12 weeks, the founder logs hours weekly, and the producer runs a quarterly fatigue check. If the rolling 8-week average falls under 20, the producer re-baselines the bands; that is a re-plan, not a failure. | Decided | Founder; HS 6, 9.5 |
| D6 | Review budget: 8-15 PRs a week at 15 founder hours, scaling with logged review hours to about 20-25 a week at 30 hours (HS 7 budgets ~40% of founder time for review), each at 200 changed lines or fewer (hard cap 400). Line-by-line review in sim, proto, persist, and server authority paths; evidence review elsewhere. Every rule is a hook. An evaluator agent sits in the merge gate. Releases are human-only. | Decided | HS 7, 9.6 |
| D7 | Wire format is schema-owned and cross-language: protobuf via prost by default, FlatBuffers if the spike shows snapshot decode cost. C# types generated from the Rust-owned schema in CI. | Default | HS 5, 9.2 |
| D8 | Listen server is a sidecar process spawned by the client, with a watchdog (silent-control-socket exit, PID kill on clean exit, tick-boundary saves). The in-process listen server is removed from the roadmap. | Decided | HS 5, 9.3 |
| D9 | Netcode is server-authoritative with interpolation only, tuned for 80-150 ms. No client prediction or rollback in v1. | Decided | MR 7 |
| D10 | Gates are cumulative-hour triggers: ant farm fun alone at ~250 h; velocity gate at ~400 h; two-client authoritative session at ~500 h; engine confirmation at ~600 h; 30-minute run fun alone at ~900 h; Thursday group day-7 return over 30% at ~1,100 h; a 90-day post-page wishlist-velocity gate replaces the old 25-30k wishlist gate. | Decided | HS 6, 9.1 |
| D11 | Dates are bands, computed from build-hours (D23). At 30 h/week on the cut scope (about 850 build-hours a year): two-player loop around Aug 2027, Steam page around Oct 2027, public demo Sep 2028 (P20) to mid-2029 (P50), early access Dec 2028 (P20) to Oct-Nov 2029 (P50). Next Fest registers against the P50 demo date: February 2030 by default, October 2029 if the demo lands early. February 2028 stays dropped. At 15 h/week the HS 6 bands apply instead (early access 2031-33). | Default | HS 6 sensitivity table; founder hours D5 |
| D12 | Hiring: none in the base case. First spend is per-asset art from petty cash ($300-1,500 a month before the page: capsule, settler rig, one kit, playtests) and two fixed-fee senior Rust architecture reviews. First recurring contractor, at the 90-day post-page gate on ~1,500-2,000 organic wishlists, is a client feel engineer or technical artist from the chosen engine's C# pool, evidence-reviewed. Never a second line-by-line output stream before revenue. | Decided | HS 3, 9.7, 9.8 |
| D13 | Camera: third-person follow camera with the top-down diorama as the settlement view, locked by a prototype of both at ~400 logged hours. | Default | MR 7, 8.7; HS 9.11 |
| D14 | Monetization: premium only, Valve as merchant of record, $17.99-19.99 at early access rising to $24.99 at 1.0, never below $15. No free-to-play mechanics, subscriptions, timers, or wipes. | Decided | MR 7, 8.8 |
| D15 | Platforms: Steam first, Windows plus Steam Deck Playable via Proton, native Linux as a near-free bonus. No web, mobile, or console in the first phase. | Decided | MR 7 |
| D16 | Next Fest discipline: one fest per title ever; never register under ~2,000 pre-fest wishlists or before the demo has been public a month; register against the P50 demo date. | Decided | MR 8.2; HS 9.13 |
| D17 | Pins: one Rust toolchain, one Steamworks SDK version across every binding and the redistributable, one engine minor line. At most two engine minor upgrades a year. | Decided | HS 5 |
| D18 | Rust education is not a goal. The founder reads line-by-line crates to verify them, not to learn. Agents explain decisions in PR descriptions so the founder can explain every merged change without the agent. | Decided | HS 2, 7 |
| D19 | Employment decision at the 90-day gate: a four-day week or sabbatical from a named savings runway, a publisher advance, or hobby pace on the cut scope with early access in 2031-33 accepted in writing. | Pending gate | HS 9.14 |
| D20 | Working title "Longthaw" needs a Steam, trademark, and domain search before the store page goes live. | Pending | MR 7 |

## Week 0 (before any feature work)

1. The Steam Datagram Relay spike on app 480: a headless Rust GameServer on a box with no Steam client accepting a P2P connection from a desktop client. One day. The result, pass or fail, is written into this file as D3's outcome.
2. The sidecar watchdog spike: spawn, silent-control-socket exit, PID kill on clean exit, crash recovery. One day.
3. The wire-format spike: protobuf via prost versus FlatBuffers on a representative snapshot, with C# decode generated from the schema. Half a day.
4. The engine bake-off plan: the identical vertical slice both engines must reach, the hours log, and the tie rule, written before either project is created.

## Team operating defaults (added 2026-10-08)

| ID | Decision | Status | Source |
|---|---|---|---|
| D21 | Every open question in `open-questions.md` follows the default written beside it until the founder answers it. Agents never block on an open question; they apply the default and cite the question number in the PR. | Decided | Consistency review |
| D22 | Line counting for the PR cap (D6): hand-written lines count, in every area. Generated C# types, `Cargo.lock`, engine-generated scene and `.meta` files, and fixture files do not count, but the PR lists them with their sizes and the reviewer spot-checks them. Evidence-area lines count the same as line-by-line lines. | Default | Resolves reviewer, client-engineer, devex-engineer disagreement |
| D23 | Gates and milestone bands count build-hours, computed from the founder's logged hours as (logged minus 2.5 a week) x 0.8 x 0.92, because that is how HS 6 derived the bands. `docs/plan/hours.md` stores logged hours; the build-hours column is derived. | Default | Open question 1; HS 6 |
| D24 | Agents never edit `.claude/` (agent definitions, settings, hooks), `CLAUDE.md`, `docs/research/`, or `decisions.md`. They propose changes in a PR description or an open question; the founder edits these files. | Decided | Harness flag on the devex engineer's Stop-hook question |
| D25 | A Claude Code Stop hook in `.claude/settings.json` runs `tools/hooks/pre-push` when an agent finishes, so no agent ends a task with failing checks. Approved by the founder on 2026-10-08. It is added in its own PR after the devex engineer's pre-push script exists; agents still never edit `.claude/` themselves (D24). | Decided | Founder; devex-engineer open question 5 |
