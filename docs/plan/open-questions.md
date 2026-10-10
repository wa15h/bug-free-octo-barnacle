# Open questions and the defaults that apply until the founder answers

The consistency review of the agent team (2026-10-08) collected these from the agent
definitions. Each carries the default the team follows today, per decision D21. To change
one, edit the line to say `Answered:` with your answer, or promote it to a decision in
`decisions.md`. The agent named in parentheses is the one waiting on it.

Two questions where agents disagreed are already resolved in `decisions.md` as D22
(how lines count against the PR cap) and D23 (gates count build-hours).

1. Gate counting: D10 says cumulative-hour triggers, but the HS 6 dates reproduce only from build-hours ((logged - 2.5) x 0.8 x 0.92). Confirm gates.md counts build-hours, not raw logged hours. (producer)
2. Answered (D28): V < 0.6 at the ~400 h velocity gate re-dates the bands (D5, D11) and nothing else; no scope is cut and no fallback slice is built to hold a date. The cut scope (D2) stays the target state. (producer)
3. Hours source of truth: docs/plan/hours.md typed by you, with the crates/tools hours tool only deriving columns (the team's default); and is that tool wanted before week 6 or does the file by hand suffice until the velocity gate? (producer, test-engineer)
4. The 201-400 line band: D6 sets 200 as the norm and 400 as the hard cap but not when the band is allowed. Until told, the reviewer returns 201-400 with a split plan. (reviewer)
5. Line counting against the 200 cap: do generated C# types, Cargo.lock, fixture files and engine-generated scene/.meta files count, and do evidence-area lines cost the same budget as line-by-line lines? The defaults currently differ: reviewer counts every line, client counts hand-written lines only, devex exempts Cargo.lock only. (reviewer, client-engineer, devex)
6. Who runs headless-session and attaches proof.txt to a PR: the writer (team default), the reviewer, or the test engineer? (reviewer, test-engineer)
7. Editing or deleting a test needs founder approval per HS 7 but is not a numbered rule: should the reviewer block or flag, should test-guard.sh fail or comment, and does it become rule 10? (reviewer, devex)
8. Tick rate and snapshot send rate, and whether there is one fixed step or a fast step (movement, combat) plus a slow hearth step so catch-up is cheap. Until answered: one TICK_HZ and one HEARTH_EVERY constant; tick_hz and send_every travel in Welcome. (sim, net)
9. Catch-up cap (longest offline window simulated), whether the clamp is shown to the player, and the autosave interval in ticks: who sets them and do they live in content/? Default: clamp at one season, stated in the first catch-up ledger line. (sim, persist, game-designer)
10. Fixed scale and width is a contract shared by sim, proto and persist: does it need an ADR or a decision-log line, and who signs it? (sim)
11. Save encoding: a serde crate owned by persist (team default; the sim engineer's task 2 ADR derives serde on State) or proto messages; and text fixtures (readable, inflate line counts) or binary (opaque)? The persist task 1 ADR needs this first. (persist)
12. Is a guest's character world-bound (stored in the host's save keyed by Steam ID, the default) or player-bound (travels with the guest, needing a second save file and a merge rule)? (persist)
13. Is Steam Cloud sync of the host save in v1? It creates cross-machine conflicts the generation scheme does not resolve. (persist)
14. Pre-register the C# decode-cost threshold that flips D7 to FlatBuffers before the wire-format spike runs. (net)
15. Protocol version policy: exact match (default, simplest to explain) or a compatibility window; it is player-facing. (net)
16. Sidecar timing and transport: CONTROL_SILENCE_SECS and HEARTBEAT_SECS (defaults 10 and 2) and localhost TCP on a client-chosen port (default) versus stdin/stdout pipes. (server)
17. Raw-UDP identity: Steam auth session tickets over raw UDP, or LAN-only unverified peers refused unless --allow-unverified (default). (server)
18. SDR spike logistics and app creation: who provisions the headless Linux box; does the spike rerun on the project's own app ID once a Steamworks partner account exists; and which checkpoint queues the 'create app' row given the fee is spend (D12) and the D20 search comes first? (server, steam-release)
19. Where the per-engine bake-off hours table lives: client/BAKEOFF.md with one summed line per week in hours.md (default), or hours.md itself. (client)
20. Unity exports in CI need a seat or licence server (spend): default is bake-off exports on your machine logged as founder hours, a CI export job only for the confirmed winner. Confirm. (client, devex)
21. Nightly and CI budgets: nightly seed count and wall cap (defaults 2,000 seeds, 60 minutes) and the monthly CI-minutes budget with Windows runners at 2x. (test-engineer, devex)
22. Initial bench budgets for tick p99, bytes per tick and one-season catch-up: baseline x 2 on the pinned runner, set in the bench PR? (test-engineer)
23. Does a red nightly block every merge (default) or only merges into crates/sim, proto, persist and server? (test-engineer)
24. Rust toolchain cadence: no decision names one; default is the toolchain moves only when a pinned dependency requires it, in a pin-only PR. (devex)
25. Answered (D25): yes, the Stop hook is approved and added in its own PR once tools/hooks/pre-push exists; the founder owns .claude/ (D24). (devex)
26. Gate 0 pass line: MR 7 gives '3 of 4 testers' for Gate 1 only; default is the same 3 of 4 for Gate 0, and the Gate 2 cohort is every Thursday tester who launched on day 0. (game-designer)
27. Death in a run: no decision names the loss on death; D26 names it only for missing the last gate (gear kept, carried units left where you stood). Default: gear kept, carried units dropped where you fell and retrievable until the next cordon stage, the bound a row in gear.csv. (game-designer)
28. Opt-in stakes: MR 2 lists bounded opt-in stakes as a pillar and MR 7 defers the opt-in extraction mode, now the Hospital Quarter (D26). Does v1 carry any, or none (default) with the deferred list saying why? (game-designer)
29. Settler rig cost against the $8k year-one cap: default is a recolored kit character until the page month, with settler-rig.md written but not queued. (art-director)
30. Purchased kits: confirm a founder-held, backed-up location with a devex-written tools/ fetch script and meshes never committed; and under rule 6 is LICENSE-RECORD.md the record, with an ADR only when a kit brings an importer plugin or shader dependency (default)? (art-director, client)
31. Capsule timing: HS 9.7 pulls the capsule into the page month, MR 7 wants the hook in the first screenshot; default is the brief ready in weeks 0-6 and a founder-made HUD screenshot until the ~$400 spend is queued. (art-director)
32. Themed fests: does D16's ~2,000-wishlist, public-a-month rule bind themed-fest entries too? Default: tag the page, enter nothing. (steam-release)
33. Steam Playtest: MR 7 names a ~50-person closed alpha absent from the decision log. Is it in scope, and does it start the D16 'public a month' clock? Default: no and no. (steam-release)
34. Channels before the page: default is devlogs stay in the repo and go to the Thursday group with the build note, the group uses your existing private chat, and no public server, site or social account exists before the page and the D20 search. (marketing)
35. The 'social postcard' the brief cites from MR 7 cannot be found there (MR 7's only 'postcard' is the retired wire-format crate). What is it? Default: deferred with the Boroughs under D2. (marketing)
36. The 90-day gate middle band: HS 6 defines pass (~1,500-2,000) and fail (under ~500) but not 500-1,500. Default: no hire, no cut, re-read on the demo-page day. (marketing)
37. D12 names only a client feel engineer or technical artist as the gate contractor; HS 9.8 also allows a part-time community manager if wishlists bind. Amend D12 to include it, or leave it out? The marketing file now follows D12 as written.
