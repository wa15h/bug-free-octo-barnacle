# Market Research Report: An Online Multiplayer Rust Game Built by One Founder

Prepared 2026-10-07 for the founder. Written for a senior backend and data engineer who is new to game development and to Rust. Numbers first; every figure carries a link; estimator outputs and unverified figures are flagged as such.

## 1. Executive summary

Build **Longthaw**, the top-ranked concept (second-pass average 8.83 of 10): a player-hosted, 1-4 player, top-down scavenging game with a settlement that keeps working between sessions, set in a county that thaws week by week, sold on Steam at $19.99 with a Rust authoritative server shipped as a listen server inside the client plus a free dedicated binary. Build it on the skeptics' terms, not the pitch's. Three adversarial reviewers attacked the three load-bearing claims of each top-four concept (market, ship, multiplayer model) and refuted all twelve; no claim survived. The refutations are not fatal to Longthaw, but they change the plan: the twelve-month deliverable becomes a live Steam page, a free Prologue app and a wishlist pile rather than a paid Early Access launch; EA ships only when wishlists reach 25-30k or a publisher signs, realistically Q1 2028; the County meta-layer is redesigned so a lying player-hosted server cannot harm it and carries no free text; in-game Steam voice and client prediction leave the critical path; the stack is frozen on one Bevy version; and the week is capped at 45-50 hours. The market evidence that drove the choice is unchanged by the skeptics: online co-op survival-craft is the highest hit-rate lane on Steam ([20.8% of 2025 releases reached 1,000 reviews vs 2.99% Steam-wide](https://howtomarketagame.com/2026/01/27/what-the-hell-happened-in-2025/)), the two nearest solo-scale comps ([ZERO Sievert, 600k+ copies](https://underdog.ltd/presskit.html); [Escape from Duckov, 3M copies in 23 days](https://en.wikipedia.org/wiki/Escape_from_Duckov)) shipped no multiplayer, and the player-hosted model carries no hosting bill, no population floor and no anti-cheat burden. What the skeptics changed is the arithmetic of year one: at the pitch's own 15k-wishlist gate, published conversion rates produce roughly [$50k gross](https://newsletter.gamediscover.co/p/the-state-of-steam-wishlist-conversions), not $100k.

| Rank | Concept | Lens | Second-pass avg (C / F / FF) | Skeptic claims survived |
|---|---|---|---|---|
| 1 | Longthaw | merge | 8.83 (9 / 9 / 8.5) | 0 of 3 (market, ship, mp-model all refuted) |
| 2 | Weirhold | merge | 8.00 (8.5 / 8 / 7.5) | 0 of 3 |
| 3 | Ashward | merge | 7.67 (5.5 / 8.5 / 9) | 0 of 3 |

C = commercial, F = feasibility, FF = founder-fit. The fourth finalist, DEADWEIGHT (6.33), also survived 0 of 3.

## 2. Founder profile, taste DNA, and the budget correction

**Profile.** Solo developer plus AI coding agents, 25 years of backend and data engineering, new to Rust and to game development, 12-month horizon. Hard constraints: online multiplayer from day one with a Rust authoritative server; buildable by one person plus agents; no art team; commercial viability drives design; learn Rust deeply. Today is 2026-10-07.

**Taste DNA.** The four favorites (Fallout, Rise of Kingdoms, The Division, SimCity) are four wrappers around one loop: build a home base, go out into a systemic world to gather, come back and compound. Seven pillars: the base as the anchor of identity (FO4 settlements, FO76 CAMPs, RoK city, SimCity); legible systems (GlassBox agents, junk-to-components, explicit gear math); compounding months-long progression; the out-and-back expedition; solo-viable play with a social multiplier (Fallout alone, Division in 1-2 hour squads, RoK as async alliance check-ins); bounded opt-in stakes (Dark Zone, KvK) rather than ubiquitous danger; post-collapse worlds told through environment. Session shape is three tiers feeding one base: 2-10 minute async check-ins, 30-90 minute active runs, occasional 2-4 hour deep sessions. Likely dislikes: timer-gating and pay-to-skip, aim-skill PvP as the core, full-loot open PvP, heavy authored narrative, always-online shipped before the server is ready (SimCity 2013, FO76), wipes, content-treadmill live service. Quantic Foundry placement: Mastery-Achievement and Immersion-Creativity with Discovery as the bridge; Action-Social is the weakest cluster.

**Budget correction.** The profile said 20 hours per week (about 1,000 hours per year). The founder has clarified 3,000 hours per year, about 58 hours per week. Consequences the judges and skeptics drew: (a) at 1,000 hours the honest answer was "pick one half of the taste and stage the other as a wish"; at 3,000 hours both halves of the taste (persistent shared world plus embodied co-op run) fit in one staged game inside 15-18 months, so the merged pitches that keep both halves moved to the top; (b) agents multiply throughput on server, persistence, protocol, tests and tooling, and barely help with game feel, combat tuning, shaders, asset integration and Bevy version churn, so by Amdahl's law the real schedule is bounded by roughly 1,200-1,500 founder-only hours in the co-op pitches; (c) the only randomized trial of experienced developers using AI tools found them [19% slower while believing they were 20% faster](https://metr.org/blog/2025-07-10-early-2025-ai-experienced-os-dev-study/), and METR's 2026 follow-up estimated [+18% for returning developers with a confidence interval from -9% to +38%](https://birchtree.me/blog/an-update-from-the-study-that-said-devs-were-actually-slower-with-coding-agents/), so the 2-3x multiplier in the pitches is an assumption, not a measurement; (d) 58 hours per week sits above the [~50-hour point where Pencavel finds output per hour falling](https://docs.iza.org/dp8129.pdf) (not fetched; medium confidence) and inside the [55-hour band WHO/ILO associate with +35% stroke risk](https://www.who.int/news/item/17-05-2021-long-working-hours-increasing-deaths-from-heart-disease-and-stroke-who-ilo), so burnout is a plan risk, not a footnote; (e) Steam overhead (Steamworks, store page, trailer, Deck Verified, Next Fest demo) still costs 4-6 weeks regardless of budget.

**Ranking before and after the correction** (average of three judges; the three merged pitches were written after the correction and scored only in the second pass):

| Concept | First pass (1,000 h) | Second pass (3,000 h) | Move |
|---|---|---|---|
| Longthaw (merge) | n/a | 8.83 | new, #1 |
| Weirhold (merge) | n/a | 8.00 | new, #2 |
| Ashward (merge) | n/a | 7.67 | new, #3 |
| DEADWEIGHT | 6.33 (#4) | 6.33 (#4) | flat |
| Wintergrid | 6.50 (#3) | 6.17 (#5) | down |
| Haulback | 7.17 (#1) | 6.17 (#6) | down 5 |
| Last County | 5.33 (#8) | 5.50 (#7) | up |
| Fallow | 6.17 (#5) | 5.33 (#8) | down 3 |
| Watershed | 5.33 (#7) | 5.17 (#9) | down |
| Downriver | 7.00 (#2) | 5.00 (#10) | down 8 |
| RUSTBELT | 5.83 (#6) | 5.00 (#11) | down 5 |
| Graymarket | 4.67 (#10) | 4.50 (#12) | down |
| Cordon | 4.67 (#9) | 3.83 (#13) | down |

The pattern: pitches written as minimum-viable cuts for 1,000 hours (Haulback, Fallow, Downriver) lost ground because they left two-thirds of the new capacity unspent; developer-hosted combat pitches (Cordon, Graymarket) did not benefit because their deciding hours are gunfeel, which is founder-only; the merges won because they are the co-op scavenger with the first-pass misses fixed.

## 3. Market landscape

Method caveat that applies to every lane: the research sessions could not reach Steam, SteamDB, Gamalytic, VG Insights, Sensor Tower, GameDiscoverCo or most press sites directly (egress blocked), so many figures come from search snippets citing those pages, from GitHub-hosted copies of estimator CSVs (Gamalytic snapshots from roughly late 2024 and December 2025), and from a nightly Steam catalog dump ([leinstay/steamdb](https://github.com/leinstay/steamdb), 189,793-190,125 rows, generated 2026-10-04 to 10-06). Developer-announced milestones are high confidence; estimator revenues are medium at best and diverge up to 2x between tools (Enshrouded [$119.5M on Sensor Tower](https://app.sensortower.com/vgi/game/enshrouded) vs ~$53M elsewhere).

### 3.1 Steam city and colony builders

Winner-take-most and worsening. The genre's top is reachable by tiny teams but almost always single-player, after long Early Access, with buyers punishing simulation bugs harder than they reward scope.

| Metric | Value | Source |
|---|---|---|
| City Builder releases reaching 1,000 reviews | 10 of 370 (2.7%) in 2024; 2 of 397 (0.5%) in 2025 | [howtomarketagame](https://howtomarketagame.com/2026/01/27/what-the-hell-happened-in-2025/) |
| Colony Sim same bar | 1 of 158 (1.5%) 2024; 2 of 193 (1.0%) 2025 | same |
| Median lifetime net revenue | Colony Sim $44k, City Builder $22k (GDCo); Colony Sim $8k (Gamalytic, broader cut) | [GDCo](https://newsletter.gamediscover.co/p/which-genre-should-your-next-pc-game), [Gamalytic](https://gamedevreports.substack.com/p/gamalytic-67-of-games-on-steam-earned) |
| Tiny-team hits | Manor Lords 3M (solo + contractors); Against the Storm 2M (6 people) | [gamespress](https://www.gamespress.com/Manor-Lords-celebrates-three-million-sales), [gamespress](https://www.gamespress.com/ko-KR/against-the-storm-2-million-copies-sold-milestone) |
| EA graduates 2025 | 225 games, median 437 days in EA, only 20% earned more at 1.0; median 1.0 month = 40% of EA month | [gameworldobserver](https://gameworldobserver.com/2025/12/10/according-to-gamediscoverco-most-games-on-steam-earn-less-after-leaving-early-access-than-they-do-at-launch) |
| Multiplayer builders | Kingdoms Reborn 300k copies, desync threads 2023-2026; Settlers: New Allies 48% positive; Anno 117 65% of 5,501 | [unrealengine](https://www.unrealengine.com/en-US/developer-interviews/inside-kingdoms-reborn-s-game-dev-s-journey-of-discovery-and-city-building), [steambase](https://steambase.io/games/the-settlers-new-allies/steam-charts), [ubisoft](https://news.ubisoft.com/en-us/article/7FK06qQE7cN6hvjbVJrI0i/anno-117-pax-romana-celebrates-117-million-players) |
| Co-op on Steam overall | 6% of 2023 releases, 36% of units; median co-op ~40k units vs ~5k single-player; $4.1B H1 2025 | [gamedeveloper](https://www.gamedeveloper.com/business/study-finds-co-op-games-keep-growing-in-numbers-and-sales-on-steam), [wnhub](https://wnhub.io/news/stores-and-publishing/item-48428) |

Verdict: for a classic city builder multiplayer is neutral as a purchase driver and a liability with weak netcode; for a base-building/survival hybrid, 2-4 player co-op is a real differentiator. The only current post-apocalyptic settlement reference, [After Inc: Revival (78% positive, $17.99)](https://store.steampowered.com/app/3337140/After_Inc_Revival/), is single-player.

### 3.2 Online co-op survival-craft (OWSC)

The highest hit-rate genre on Steam, dominated by player-hosted servers, with a settlement layer as the 2024-26 differentiator.

| Metric | Value | Source |
|---|---|---|
| OWSC 1,000-review rate | 15 of 72 (20.8%) in 2025; 23 of 94 (24.5%) in 2024 | [2025](https://howtomarketagame.com/2026/01/27/what-the-hell-happened-in-2025/), [2024](https://howtomarketagame.com/2025/01/15/what-the-hell-happened-in-2024/) |
| OWSC median gross | ~$102k, highest of any genre | [pokeindie](https://pokeindie.com/blog/game-genre-study-2026-which-genres-are-worth-building) |
| Steam baseline 2025 | 20,282 releases; 608 (2.99%) reached 1,000 reviews; median revenue $249 | [voxbooster](https://voxbooster.com/blog/indie-game-statistics-2026/) |
| Tiny-team breakouts | Valheim est. 17.0M / $219.9M (5 at launch); Abiotic Factor 1.4M (~12); Necesse 2M (solo start); Sunkenland ~545k (7) | [steamdata](https://steamdata.ai/game/892970/valheim), [games.gg](https://games.gg/news/abiotic-factor-million-sales/), [massivelyop](https://massivelyop.com/2025/11/09/sandbox-mmo-lite-necesse-has-sold-an-eye-popping-2-million-copies-so-far/), [levvvel](https://levvvel.com/sunkenland-statistics/) |
| Official-server cost | Palworld ~$478k for one month | [gamesradar](https://www.gamesradar.com/palworld-dev-ceo-jokes-server-fees-could-bankrupt-the-studio-after-it-spends-dollar478000-to-never-let-the-service-go-down-no-matter-what/) |
| Always-online backlash | Nightingale 47,569 peak CCU to 2,493; 65 layoffs | [steamcharts](https://steamcharts.com/app/1928980), [pcgamer](https://www.pcgamer.com/games/survival-crafting/despite-making-significant-changes-to-survival-game-nightingale-developer-inflexion-games-says-it-hasnt-been-commercially-successful-enough-to-avoid-dozens-of-layoffs/) |
| Launch playbook | Abiotic Factor demo drew ~250k wishlists in its first week | [GDCo](https://newsletter.gamediscover.co/p/how-abiotic-factor-sold-600k-copies) |
| Recovery from a weak launch | 28 games in all of 2024 (0.156%) | [howtomarketagame](https://howtomarketagame.com/2026/02/12/only-28-games-recovered-from-a-bad-launch-in-2024-what-do-they-have-in-common/) |

Host-only co-op with no persistence when the host logs off is a recurring complaint ([Bellwright shipped with no dedicated servers for 2+ years](https://pindrop.gg/bellwright/guides/multiplayer) while reaching [1M copies](https://www.gamespress.com/Medieval-Survival-RPG-Bellwright-Surpasses-1-Million-Units-Sold-on-Ste)). Post-apocalyptic is a proven sub-lane ([VEIN est. 614k / $9.1M in under a year](https://app.sensortower.com/vgi/game/vein)) but not the hook. Timeline risk: every solo breakout took far longer than 12 months (Necesse 13 years to 1.0, Dinkum 8).

### 3.3 Looter and extraction co-op

PvP-first extraction from small teams is a graveyard; PvE-first top-down extraction-lite is where solo-scale hits live, and the Division fantasy is unsupplied on PC.

| Metric | Value | Source |
|---|---|---|
| ZERO Sievert | solo dev; $1.3M first week; 600k+ copies; no co-op ever shipped | [wnhub](https://wnhub.io/news/other/item-1533), [presskit](https://underdog.ltd/presskit.html) |
| Escape from Duckov | 5 devs; 2M in two weeks, 3M by 2025-11-08; 301,322 peak CCU; single-player | [gamedeveloper](https://www.gamedeveloper.com/business/escape-from-duckov-has-sold-2-million-copies-in-two-weeks), [pcgamer](https://www.pcgamer.com/games/action/escape-from-duckov-quacks-past-300-000-concurrent-players-only-10-days-after-release/) |
| PvP extraction shutdowns | The Cycle: Frontier (cheaters); Marauders 500k sold vs 80k expected, CCU collapse; Marathon ~1.2M / $55M, under 1,000 CCU in 6 months | [gamedeveloper](https://www.gamedeveloper.com/business/yager-sunsetting-the-cycle-frontier-after-cheaters-cause-irreparable-damage), [heroiclabs](https://heroiclabs.com/blog/marauders-case-study/), [gameshub](https://www.gameshub.com/news/article/marathon-sales-estimate-1-2-million-copies-steam-ps5-2026-2862962/) |
| Capitalized winners | Arc Raiders 12.4M copies | [gamingbolt](https://gamingbolt.com/arc-raiders-has-sold-more-than-12-4-million-copies) |
| Division on PC | Heartland cancelled; Resurgence Steam port 46% positive, 4,117 peak CCU | [shanethegamer](https://www.shanethegamer.com/pc-mac/the-division-resurgence-lands-on-steam-splits-players-with-46-positive-score/) |
| Top-down co-op comps | The Ascent 12 devs, 1M copies, $5M week one | [gamedeveloper](https://www.gamedeveloper.com/business/curve-games-boasts-strong-sales-for-i-the-ascent-i-and-i-human-fall-flat-i-) |
| Indie MP shooter survival | 1 of 7 that reached 1,000 reviews still alive (Q2 2025) | [howtomarketagame](https://howtomarketagame.com/2025/08/04/2025-q2-games-that-are-selling/) |

Even the genre's indie success, Dark and Darker, [lost money in 2024 with 86 staff](https://vortexgaming.io/en/postdetail/498074). Punishing real-time offline mechanics get review-bombed ([The Forever Winter removed its water timer twice](https://www.pcgamer.com/games/third-person-shooter/the-forever-winter-my-favourite-extraction-shooter-just-overhauled-its-most-contentious-feature-for-the-second-time-it-was-a-hell-of-a-rollercoaster-to-make-the-adjustment/)). Persistent small-shard top-down looters by tiny teams have a long precedent ([STALCRAFT: X, 95,600+ reviews, 10-14k CCU](https://www.gamespress.com/Free-to-Play-MMOFPS-STALCRAFT-X-Celebrates-10th-Anniversary-with-Huge-)).

### 3.4 Persistent-world strategy

Enormous mobile appetite monetized by pay-to-win, a 20-year browser lineage still alive, and almost no small-team paid entry on PC. Note: this lane's revenue and player-count figures were largely unverifiable in-session (Sensor Tower, Steam, SteamDB blocked).

| Metric | Value | Source |
|---|---|---|
| Rust server tech | SpacetimeDB 1.0 on 2025-03-03, 25.3k stars; BitCraft server open-sourced (Apache 2.0, 597 stars) | [releases](https://github.com/clockworklabs/SpacetimeDB/releases/tag/v1.0.0), [BitCraftPublic](https://github.com/clockworklabs/BitCraftPublic) |
| Solo Rust MMO | OpenMMO: 1.8k stars, 2,622 commits in ~14 months, tokio/axum server | [OpenMMO](https://github.com/Julian-adv/OpenMMO) |
| Solo Rust Travian clone | parabellum: axum/sqlx CQRS, created 2023-02, still "mostly playable" 2026-09 | [parabellum](https://github.com/andreapavoni/parabellum) |
| Clone demand | 53 OGame-clone and 38 Travian-clone repos; OGameX pushed 2026-10-05 | [OGameX](https://github.com/lanedirt/OGameX) |
| RoK automation | 196 repos, top ones grind bots (148 stars) and OCR trackers | [RoK bot](https://github.com/Dylan-Zheng/Rise-of-Kingdoms-Bot) |
| Non-P2W models | Screeps buy-once + CPU Unlock; server open-sourced, last push 2026-04 | [docs](https://github.com/screeps/docs/blob/master/source/control.md), [repo](https://github.com/screeps/screeps) |
| Foxhole architecture | 3 live shards, War API refreshed every 3-60 s with ETags | [warapi](https://github.com/clapfoot/warapi) |
| Slow-tick lineage | Longturn Freeciv at 23-hour turns; Neptune's Pride, Prosperous Universe tooling pushed Aug-Sep 2026 | [freeciv21](https://github.com/longturn/freeciv21/blob/master/docs/Contributing/game-admin.rst), [PrUn docs](https://github.com/simulogics/prosperousuniverse-docs) |
| Mobile 4X scale | RoK >$1B lifetime; Whiteout Survival >$1B in 2024 (UNVERIFIED, low confidence) | [sensortower](https://sensortower.com/blog) |

A GitHub search for "browser strategy game persistent server language:rust" returned zero results, and only two Rust MMO repos exceed 150 stars. The VC-funded big persistent worlds (Dual Universe, Starbase, Last Oasis, Pax Dei, Seed) mostly collapsed or stalled (unverified, low confidence).

### 3.5 The multiplayer-builder gap

Every major single-player builder has a community multiplayer retrofit fighting desync because it bolts lockstep onto a non-deterministic sim; that is the exact bug a server-authoritative design removes. All sales figures below are Gamalytic estimates captured in [a GitHub-hosted snapshot (Nov 2024-Feb 2025)](https://github.com/VIV-T/BUT3_Developpement_web/tree/64d0a457f8f9c5a159c424cd2d9974d0764d4e9b/1_Preparation_Donnees/donneesBrutes/scrapping_gamesData_Gamalytics) and a [December 2025 snapshot](https://github.com/kgs112233/dataproject5team/blob/189aaa24dfbfe79b634e29af488beb78f7232d99/output/output_after.csv); medium confidence.

| Title | Est. copies / revenue / avg playtime | Model |
|---|---|---|
| Eco | 682,636 / $17.8M / 74.9h (highest of any non-factory builder) | persistent server, 7+ years EA, 82% |
| Kingdoms Reborn | 572,472 / $9.3M / 22.6h; 91% | solo-dev origin, up to 8 players on shared map |
| Colony Survival | 321,849 / $5.6M / 37.1h | 2-person team, player-hosted servers |
| Sapiens | 64,097 / $1.3M; 85% | solo, multiplayer from EA launch |
| Cities: Skylines / RimWorld / ONI | 10.7M / $107M; 4.0M / $112.7M / 200h; 4.9M / $64.3M | single-player |
| Factorio / Satisfactory | 4.1M / $111.5M / 139h; 5.5M / $119.6M (Dec 2025: $170.8M) | co-op factory |

Demand proof: [RimWorld Multiplayer has 112 open issues after 7 years](https://github.com/rwmt/Multiplayer/releases) and a separate [RimWorld Together server mod with 2,086 commits since Feb 2024](https://github.com/RimWorld-Together/Rimworld-Together); [BeaverBuddies logged nine desync issues Oct 2025-Aug 2026 and only ever tested two players](https://github.com/thomaswp/BeaverBuddies/wiki); [TimberTogether (created 2026-09-21) pivots to "a colony each on a shared map, barter at trading posts"](https://github.com/timbermods/TimberTogether); [ONI multiplayer is still "Early WIP" after three years](https://github.com/onimp/oni_multiplayer); [14 of 20 CSM issues searched are desyncs](https://github.com/CitiesSkylinesMultiplayer/CSM/issues?q=is%3Aissue+desync); [Nebula for DSP has 945 stars and a headless server](https://github.com/NebulaModTeam/nebula). Verdict: multiplayer-native pure city builders cap at roughly $5-18M lifetime; "persistent server with many cities on one world" is the least-supplied flavor, and [SimCity 2013](https://en.wikipedia.org/wiki/SimCity_(2013_video_game)) (low confidence, not fetched) is the warning about shipping always-online before the server is ready.

### 3.6 Multiplayer postmortems

The structural lesson: every tiny-team multiplayer hit runs the simulation on a player's machine; every shutdown in the record was centrally hosted and population-dependent.

| Finding | Value | Source |
|---|---|---|
| Valheim model | 3 people at founding, hoped for 10-20k copies, "you play it on your own server"; Core Keeper defaults to Steam Datagram Relay | [GDC transcript](https://github.com/dklassic/GDC-transcript/blob/master/static/src/transcript/YoOCUpdYYm4.txt), [core-keeper-dedicated](https://github.com/escapingnetwork/core-keeper-dedicated) |
| Mesh P2P trap | For Honor needed ~8 months of a full AAA team to migrate to dedicated servers while live | [GDC transcript](https://github.com/dklassic/GDC-transcript/blob/master/static/src/transcript/EOJ1Y3xRpwc.txt) |
| Persistent-world cost | AWS reference: $1.80 per CCU-month at 1,500 CCU, 64% of it egress; Division 2 packs ~1,000 players per 40-core box with daily 1,000-bot load tests | [AWS](https://github.com/aws-solutions-library-samples/guidance-for-persistent-world-game-hosting-on-aws), [GDC transcript](https://github.com/dklassic/GDC-transcript/blob/master/static/src/transcript/bcXxyKqgV0c.txt) |
| Anti-cheat burden | DayZ bans 1.39% of licenses, ~400 cheat products, public cheat sites grossing up to 1.25M/yr | [GDC transcript](https://github.com/dklassic/GDC-transcript/blob/master/static/src/transcript/0M0xBMEuWdU.txt) |
| Launch failures | Rocket League F2P: two near-total outages at integration points despite 5x load tests | [GDC transcript](https://github.com/dklassic/GDC-transcript/blob/master/static/src/transcript/W52Lm505300.txt) |
| Indie backend advice | The Maestros: monolith, not microservices; "don't be on the bleeding edge of online services" | [GDC transcript](https://github.com/dklassic/GDC-transcript/blob/master/static/src/transcript/zto7ZP2-ooM.txt) |
| Steam-only services | SDR relay, auth and signaling are not in open-source GameNetworkingSockets | [GNS](https://github.com/ValveSoftware/GameNetworkingSockets) |
| Shutdown lifetimes | Concord ~2 weeks; Rumbleverse ~6.5 months; The Cycle ~15 months (unverified, from memory) | [playstation blog](https://blog.playstation.com/2024/09/03/an-important-update-on-concord/) |

Decision rule from this lane: ship the Rust server as a library embedded in the host's client and as a standalone dedicated binary, over Steam Datagram Relay with friends-list lobbies, and host no simulation; a persistent official layer is viable only as a low-tick, low-bandwidth meta-server; never build anything whose fun requires strangers online at the same time.

### 3.7 Rust engine and netcode stack

Bevy is production-usable but churns; the netcode stack is consolidated but unproven in shipped persistent games.

| Component | State (2026-10-05) | Source |
|---|---|---|
| Bevy | 0.19.1 stable (2026-08-13); 0.20.0-rc.2 (2026-09-28); breaking releases "approximately once every 3 months"; MSRV 1.95 | [crates.io](https://crates.io/api/v1/crates/bevy) |
| Editor | prototypes repo archived 2026-04-16; Foundation "drastically underfunded", editor MVP undated | [archive](https://github.com/bevyengine/bevy_editor_prototypes), [6th birthday](https://raw.githubusercontent.com/bevyengine/bevy-website/main/content/news/2026-08-10-bevys-sixth-birthday/index.md) |
| lightyear | 0.30.1, prediction/rollback, Steam/WebTransport; 1.2k stars; two listed users, no shipped commercial title | [lightyear](https://github.com/cBournhonesque/lightyear) |
| bevy_replicon | 0.44.2, server-authoritative replication, no prediction; 207,751 downloads | [bevy_replicon](https://github.com/projectharmonia/bevy_replicon) |
| aeronet | 0.21.0 (Bevy 0.19), WebSocket/WebTransport/Steam transports | [aeronet](https://github.com/aecsocket/aeronet) |
| Ecosystem lag after Bevy 0.19 | avian 1 day, aeronet 5, lightyear 7, bevy_rapier 23; naia and matchbox none after 100+ days | [lightyear versions](https://crates.io/api/v1/crates/lightyear/versions) |
| Physics | avian3d 111k downloads/90d vs bevy_rapier3d 62k; bevy_rapier folded into rapier monorepo 2026-09-24 | [avian](https://github.com/Jondolf/avian), [avian3d](https://crates.io/api/v1/crates/avian3d) |
| Web | Safari 26.4 added WebTransport (2026-03-24); wtransport README: "not considered completely production-ready" | [Apple](https://developer.apple.com/documentation/safari-release-notes/safari-26_4-release-notes) |
| Alternatives | Godot 4.7.2 + gdext 0.5.5 "usable", web export experimental (nightly + emscripten); Fyrox 1.0 (2026-03) with 5.6k downloads/90d; Comfy archived 2025-11-24 | [gdext web](https://raw.githubusercontent.com/godot-rust/book/master/src/toolchain/export-web.md), [fyrox](https://crates.io/api/v1/crates/fyrox) |
| SpacetimeDB | 2.10.2; BSL 1.1, one production instance until 2031-09-15 | [LICENSE](https://raw.githubusercontent.com/clockworklabs/SpacetimeDB/master/LICENSE.txt) |
| Shipped Bevy titles | Tiny Glade (Bevy ECS + custom renderer, 1M+ wishlists pre-launch); Tunnet; no persistent-world or MMO game found | [4th birthday](https://raw.githubusercontent.com/bevyengine/bevy-website/main/content/news/2024-08-10-bevys-fourth-birthday/index.md) |

Hosting prices could not be fetched (all provider pages blocked); the working assumption from prior knowledge is a [Hetzner 2-vCPU/4GB VPS at roughly EUR 4-8/month and dedicated boxes at EUR 13-60/month](https://www.hetzner.com/cloud/), low confidence.

### 3.8 Platforms and monetization

Steam is the only platform where a solo Rust online game can be paid in a 12-month window. Figures from the [leinstay/steamdb dump](https://github.com/leinstay/steamdb); the revenue proxy (reviews x 35 x list price) is a Boxleiter-style estimator, medium confidence.

| Metric | Value |
|---|---|
| Steam releases per year | 12,801 (2023); 17,157 (2024); 19,942 (2025); 19,077 (2026 to Oct 4) |
| All 2024 releases | median 13 reviews; 3.8% reached 1,000; paid median gross proxy $1,679; 9.7% ≥ $100k; 2.5% ≥ $1M |
| Paid online-MP 2024 (n=1,303) | median 18 reviews; 21.9% ≥ $100k; 10.4% ≥ $1M |
| Co-op vs PvP-only, paid 2024 | co-op median 27 reviews (p90 2,539); PvP-only median 9 (p90 449) |
| Hit rate by price, paid online-MP 2024-25 | <$5: 1.7%; $15-19.99: 18.6%; $20-29.99: 41.2%; $30-39.99: 59.3% |
| 298 paid online-MP successes (≥1,000 reviews) | median price $24.99; 81.9% have Online Co-op; 29.9% were/are EA; 5.7% native Linux |
| F2P online-MP hits 2024-25 | 101 vs 298 paid; the F2P hits are NetEase/Tencent-scale |
| Subscription persistent indies | Screeps 1,982 reviews; Project Gorgon 2,456; Embers Adrift 108; Mortal Online 2 60% positive |
| One-time premium persistent | Foxhole $29.99, 44,068 reviews, ~1.5M owners; Project Zomboid 377,063 reviews |

Web-first can be a zero-cost acquisition engine: [OpenFront reached 200-250k daily browser players and 190,000 Steam wishlists with no marketing budget](https://github.com/openfrontio/OpenFrontIO/blob/main/resources/public/press/index.html), monetizing on web via ads and Stripe and on Steam via [Steam microtransactions, with "the desktop build must never reach Stripe"](https://github.com/openfrontio/OpenFrontIO/blob/main/src/client/Payments.ts). Valve as merchant of record removes VAT, refunds and chargebacks from a solo developer's plate; [Screeps routes its subscription through Xsolla for the same reason](https://github.com/screeps/docs/blob/master/source/tos.md). Store fees: [Apple 15% under $1M](https://developer.apple.com/app-store/small-business-program/); Steam 30% and the other tiers are training knowledge (not fetched). Mobile needs paid UA at [roughly $2-5 CPI](https://www.businessofapps.com/marketplace/user-acquisition/research/cost-per-install/) (low confidence) and consoles need NDA toolchains that [Bevy does not target](https://github.com/bevyengine/bevy); both are deferred. Derived hosting cost (prices unverified, low confidence): a Rust server on a Hetzner-class included-traffic box runs about $0.05-0.50 per CCU-month, versus $1-6 on a hyperscaler where egress at 20 KB/s per player alone is ~$5 per CCU-month.

### 3.9 2025-26 trends

Cheap 2-4 player online co-op is the most repeatable small-team breakout; building genres have the best hit rates on tiny supply; idle, cozy, survivors-likes, deckbuilders and tower defense are saturated. Computed from the same catalog snapshot ([steamdb](https://steamdb.info/) as the nominal source; tags are sparse for 2025-26 entries).

| Signal | Value | Source |
|---|---|---|
| Friendslop hits | PEAK 374,876 reviews at $7.99; R.E.P.O. 185,642 at $9.99; Schedule I (solo) 204,041 at $19.99 with 4-player co-op | [PEAK](https://store.steampowered.com/app/3527290), [R.E.P.O.](https://vginsights.com/game/3241660), [Schedule I](https://store.steampowered.com/app/3164500) |
| Co-op multiplier (2025 releases) | ≥500 reviews: 12.5% online co-op vs 3.5% single-player; ≥5,000: 3.32% vs 0.36% | [steamdb](https://steamdb.info/) |
| Co-op in each year's top 50 | 20 / 21 / 23 / 30 / 25 / 22 (2021-2026) | same |
| OWSC tag hit rate | 31.9% / 23.4% / 34.2% of 47 / 64 / 38 releases (2022-24) | [Valheim](https://store.steampowered.com/app/892970) |
| Saturated lanes | idle supply 127 to 1,357 (hit rate 8.7% to 4.3%); cozy 112 to 870 (16.1% to 3.4%); Bullet Hell 8.2% to 4.0%; TD 2.0% in 2025 | [TD](https://vginsights.com/game/960090) |
| Genre revenue ceilings (VG Insights top-100 median) | Massively Multiplayer $6.8M (44 titles ≥ $10M); Survival $5.0M; City Builder $2.2M; 4X $1.9M; TD $0.8M | [vginsights](https://vginsights.com/) |
| Supply shock | 22,441 releases in 2026 through Oct 4; single-player reaches 500 reviews 2-3% of the time | [steamdb](https://steamdb.info/) |
| Price profile of 5,000+-review hits | 2025 median $24.99 (30% ≤ $10); 2026 median $14.99 (42% ≤ $10); self-published 50-55% | same |

Extraction shooters are an AAA trend line (Tencent, Embark, Crytek). [Valheim's lead engineer](https://store.steampowered.com/app/892970) describes client-owned-area networking rather than a server-authoritative model and credits a $20 price and small download; the authoritative Rust server must buy something players feel (persistence, no desync, dedicated servers) or it is pure cost.

## 4. The gaps we found (ranked)

1. **An online co-op version of top-down PvE extraction-lite with a settlement layer.** [ZERO Sievert (solo, 600k+)](https://underdog.ltd/presskit.html) and [Duckov (5 people, 3M)](https://en.wikipedia.org/wiki/Escape_from_Duckov) both lack multiplayer while forums ask for it; online co-op lifts the 5,000-review hit rate [~9x](https://steamdb.info/); OWSC is the [best hit-rate genre](https://howtomarketagame.com/2026/01/27/what-the-hell-happened-in-2025/). This is the gap the winner fills. Skeptic caveat: the 2024-25 cohort of paid $15-19.99 self-published EA co-op launches had a [median of 139 reviews with 12% reaching 1,000](https://github.com/leinstay/steamdb), so the gap is real but the median outcome is modest.
2. **"The world persists while the host is offline."** [Bellwright](https://pindrop.gg/bellwright/guides/multiplayer), Forever Skies and Len's Island are host-only; a free dedicated binary from a shared sim crate makes this true at zero developer cost. Skeptic caveat: most friend groups will run the listen server, so the promise must also be true as deterministic catch-up on load.
3. **A settlement/NPC colony layer on survival.** The 2024-26 differentiator: [Bellwright 1M](https://www.gamespress.com/Medieval-Survival-RPG-Bellwright-Surpasses-1-Million-Units-Sold-on-Ste), [Necesse 2M](https://massivelyop.com/2025/11/09/sandbox-mmo-lite-necesse-has-sold-an-eye-popping-2-million-copies-so-far/), Soulmask 1M, Dinkum 1.4M. Matches the founder's strongest pillar.
4. **The Division fantasy on PC.** [Heartland cancelled, Resurgence at 46%](https://www.shanethegamer.com/pc-mac/the-division-resurgence-lands-on-steam-splits-players-with-46-positive-score/); a premium, non-FPS, PvE-first take with a fenced Dark Zone has a clear position. Served inside the winner by the Meltline zone rather than as the headline.
5. **Persistent server with many cities on one world, polished.** Only [Eco (~683k, 74.9h playtime, 7+ years EA)](https://github.com/VIV-T/BUT3_Developpement_web/tree/64d0a457f8f9c5a159c424cd2d9974d0764d4e9b/1_Preparation_Donnees/donneesBrutes/scrapping_gamesData_Gamalytics) supplies it; communities build it themselves ([RimWorld Together](https://github.com/RimWorld-Together/Rimworld-Together), [TimberTogether](https://github.com/timbermods/TimberTogether)). Skeptics demoted it: the only solo paid comp of the async shape, [Ymir, has 1,079 reviews at 76% after 7.5 years in EA](https://store.steampowered.com/app/378360/Ymir/), and paid persistent-strategy comps sit at 754-1,982 reviews. Served as a post-launch meta-layer, not a product.
6. **Rise of Kingdoms without pay-to-win on PC.** [196 RoK repos, mostly grind bots](https://github.com/Dylan-Zheng/Rise-of-Kingdoms-Bot), are a design brief for what to remove; but the audience lives on mobile and no paid PC comp exists.
7. **"No desync, server-authoritative" as a selling point.** Every multiplayer builder that shipped was punished for netcode ([Kingdoms Reborn](https://steamcommunity.com/app/1307890/discussions/0/3823048293511738403/), Settlers, Anno 117). The commercial judge is right that this is a review-risk mitigator, not a purchase driver; it belongs in the first hour of play, not the headline.

## 5. All concepts considered

Second-pass judge scores (commercial / feasibility / founder-fit) under the 3,000-hour budget. Asterisk marks the three merged pitches.

| Concept | Lens | One-line hook | C | F | FF | Avg |
|---|---|---|---|---|---|---|
| Longthaw* | merge | Scavenge the ruins each week's thaw uncovers with 3 friends; settlement works while you sleep; pin it to a shared County | 9 | 9 | 8.5 | 8.83 |
| Weirhold* | merge | Settlement on a dam whose gates decide which drowned ruins are reachable; County Board post-launch | 8.5 | 8 | 7.5 | 8.00 |
| Ashward* | merge | Persistent burned-over valley regreening as towns grow, phone check-ins; embodied salvage runs in phase two | 5.5 | 8.5 | 9 | 7.67 |
| DEADWEIGHT | trend-first | Duckov meets Valheim: tide clock, deadweight meter, proximity voice, settlement on timers | 7.5 | 6.5 | 5 | 6.33 |
| Wintergrid | taste-first | Watchable settlers plus 45-minute frozen-city expeditions, player-hosted | 6.5 | 6 | 6 | 6.17 |
| Haulback | scope-first | Fallout 4 settlements meets Duckov, friends-only, EA at month 9 | 7 | 7 | 4.5 | 6.17 |
| Last County | backend-first | A 2,000-settlement ruined county on a 15-second tick; settlers work while offline | 4.5 | 5 | 7 | 5.50 |
| Fallow | scope-first | Slow-tick valley, 10-minute check-ins, RoK shared map with SimCity sim, no P2W | 3 | 7.5 | 5.5 | 5.33 |
| Watershed | taste-first | Persistent river valley from your phone plus 45-minute PC salvage runs; free browser tier | 5 | 2.5 | 8 | 5.17 |
| Downriver | gap-first | Shared river-valley shard of up to 200 towns, no desync, no wipes | 4 | 4.5 | 6.5 | 5.00 |
| RUSTBELT | trend-first | RoK without pay-to-win on PC, Factorio-lite automation, shared regional economy | 4 | 5 | 6 | 5.00 |
| Graymarket | backend-first | 30-minute raids into a quarantined city that never stops simulating; safehouse shop sells while you sleep | 6 | 3.5 | 4 | 4.50 |
| Cordon | gap-first | Top-down co-op Division: 30-minute ops, Holds other players visit, cheat-proof shards | 5.5 | 3 | 3 | 3.83 |

## 6. Finalists in depth

Each finalist carries three adversarial verdicts. All twelve came back "refuted". Where a mitigation is sound we adopt it; where it changes the concept's rank we say so.

### 6.1 Longthaw (avg 8.83)

**Hook and loop.** Hallam County, a northern lakes region ten years after a volcanic winter; each real week the thaw line exposes new ruins and floods others on a twelve-week season that reseeds. Out: 30-60 minute runs from the hearth into kit-built ruins, scrapping typed junk under a deadweight meter, slow telegraphed top-down combat, a meltwater clock that only runs in-session. Home: settlers as visible agents with printed output, blueprints, a ledger; progression persists 24/7 on a dedicated server or catches up deterministically on a listen server. Opt-in stakes in the Meltline (90-second pump extraction, drop-on-death). The County (merge layer): pin your hearth to a shared map for caravans, compacts of 20, one seasonal Great Work, visitable snapshots, phone check-ins.

**Multiplayer model.** One sim crate, three deployments: embedded listen server (1-4), free headless dedicated binary (up to 8), and a developer-hosted County meta-service syncing kilobytes per hearth per hour. Steam Datagram Relay and lobbies via steamworks-rs, friends-list only, no matchmaking, no strangers.

**Comparables.** [Duckov 3M in 23 days](https://en.wikipedia.org/wiki/Escape_from_Duckov); [ZERO Sievert $1.3M first week](https://wnhub.io/news/other/item-1533); [Abiotic Factor 1.4M with ~12 people](https://games.gg/news/abiotic-factor-million-sales/); [Necesse 2M](https://massivelyop.com/2025/11/09/sandbox-mmo-lite-necesse-has-sold-an-eye-popping-2-million-copies-so-far/); [Valheim est. 17M](https://steamdata.ai/game/892970/valheim); [Schedule I 204,041 reviews, solo](https://store.steampowered.com/app/3164500); [Foxhole](https://github.com/clapfoot/warapi) for the meta-layer pattern.

**Market evidence.** OWSC [20.8% hit rate](https://howtomarketagame.com/2026/01/27/what-the-hell-happened-in-2025/); co-op [~9x 5,000-review multiplier](https://steamdb.info/); [$20-29.99 band at 41.2% vs 1.7% under $5](https://github.com/leinstay/steamdb); [Abiotic Factor's demo drew ~250k wishlists in a week](https://newsletter.gamediscover.co/p/how-abiotic-factor-sold-600k-copies); the postmortems lane's player-hosted-plus-thin-meta-server rule.

**12-month scope as pitched (3,000 h).** Q1: plain-Rust sim crate, egui viewer by week 5, Bevy client, one ruin kit, 2-player replicon over local UDP, Steam page by mid-December, Thursday test group from week 2, Gate 1 solo-fun test. Q2: SDR lobbies, 4-player drop-in, voice, dedicated binary with Docker and migration harness, three kits, five settler jobs, Deck pass, closed alpha of 50. Q3: Meltline, two more kits, hand-rolled prediction, June Next Fest demo, County v1 built by agents as a dark feature; Gate 3 at 15k wishlists. Q4: content freeze, Deck Verified, EA in late September 2027 at $19.99 with six kits and 20-25 hours. Budgeted at ~2,560 productive hours with deload weeks, ~330 h of Steam overhead, ~40 h per Bevy upgrade.

**Skeptic verdicts.**

*Market: refuted (medium).* Strongest argument: wishlists, not features, set year-one revenue. At the [median 0.10x wishlist-to-week-1 conversion for titles over $10](https://newsletter.gamediscover.co/p/the-state-of-steam-wishlist-conversions), a [2.7x week-1-to-year-1 multiplier](https://gamedevreports.substack.com/p/gamediscoverco-games-long-tail-revenue) and [~69% of US list realized across regions](https://www.steampageanalyzer.com/tools/regional-pricing-calculator), the pitch's 15k gate yields ~1,500 week-one units and ~$50k gross (~$30k net); clearing $100k gross needs ~30k launch wishlists. A new page gets [100-1,200 wishlists in two weeks](https://www.steampageanalyzer.com/blog/how-many-wishlists-before-launch) and the [median June 2026 Next Fest demo gained ~200](https://wnhub.io/news/stores-and-publishing/item-51221). Every solo comp that cleared $100k in its first EA year had 3-7 years of pre-launch work plus a publisher or PR partner ([ZERO Sievert with Modern Wolf, 277,966 wishlists at launch](https://gameworldobserver.com/tag/cabo-studio); [The Last Plague: Blight with UberStrategist](https://www.uberstrategist.com/press-releases/the-last-plague-blight-by-solo-developer-original-studios-out-now-on-steam-in-early-access)). Mitigation adopted: raise the go-gate to 30k wishlists or a signed publisher; ship a free standalone Prologue in Q2-Q3 2027 and run it through two Next Fests ([Tinkerlands' prologue held ~1,000 daily CCU before an EA that reached an est. 266k units](https://nodal.gg/game/tinkerlands-2617700)); move EA to Q1-Q2 2028; open publisher talks at Gate 2 (March 2027); state the per-unit model honestly as ~$12.40 gross and ~$7.40 net.

*Ship: refuted (high for the 9-12 month EA clause; the 3-month prototype clause survives).* Strongest argument: no solo or tiny-team comparable reached EA with less than Longthaw's scope in under two years ([ZERO Sievert ~2 years solo, single-player, GameMaker](https://gameworldobserver.com/2023/01/23/zero-sievert-1-million-revenue-cabo-studio-extraction-shooter-steam); [Valheim ~4 years, 5 people](https://gamesbeat.com/valheim-finally-hits-1-0-after-five-years-in-early-access-interview/); [Tiny Glade ~2.5 years, 2 people, Bevy, no multiplayer](https://80.lv/articles/relaxing-castle-builder-tiny-glade-has-finally-arrived-on-steam/); [Garden Wars: "multiplayer makes everything take 5 times as long"](https://www.gamedeveloper.com/design/valuable-but-not-profitable-garden-wars-postmortem)). The agent multiplier is contradicted by [METR's RCT](https://metr.org/blog/2025-07-10-early-2025-ai-experienced-os-dev-study/). A concrete stack tax: [renet_steam pins steamworks 0.12.2](https://raw.githubusercontent.com/lucaspoffo/renet/master/renet_steam/Cargo.toml) while [bevy_steamworks 0.17 pins 0.13.1](https://raw.githubusercontent.com/HouraiTeahouse/bevy_steamworks/main/Cargo.toml), two incompatible Client types with no Cargo warning. The plan's own June gate pushes the median outcome to 2028. Mitigation adopted: redefine the sellable EA build as hearth + 3 kits + one enemy family + 2-4 player listen server over SDR, ~10-12 hours, with the dedicated binary at EA+60 days and County at EA+6 months; 18 months to EA as the base plan; cap the week at 45-50 hours; measure the agent multiplier for 8 weeks and re-plan from the observed ratio; pick one Steam transport path in week 1 with a CI job that fails on duplicate steamworks versions; add a week-6 Gate 0 on the ant farm alone.

*MP model: refuted (high on the anti-cheat leg; medium on population).* Strongest argument: the County's inputs are HMAC-signed receipts from a server the player owns and can patch, so a receipt proves the host said it, not that it happened; every client-held economy has been flooded ([Diablo II open Battle.net](https://diablo2.diablowiki.net/B.net), [Fallout 76 dupes](https://primagames.com/news/duped-items-deleted-new-fallout-76-patch), [ARK's wipe three weeks after launch](https://massivelyop.com/2015/06/23/excessive-duping-leads-to-ark-survival-evolved-wipe/)). A contribution-scored Great Work is a leaderboard the audience already bots. Free-text mail and names make a solo developer the operator of a user-to-user service under the [UK Online Safety Act with no small-company exemption](https://www.ofcom.org.uk/online-safety/the-online-safety-act-and-gaming-know-the-risks-know-the-rules-know-how-to-comply). [steamworks-rs 0.13 has no voice module](https://github.com/Noxime/steamworks-rs/issues?q=voice), so "host-relayed Steam voice" is unbudgeted FFI. Trade layers degrade non-linearly at low population ([Eco servers falling from 150 to 20 players](https://devtrackers.gg/eco/p/3eeb00d5-player-retention-ideas)). Mitigation adopted: the County mints its own fungible credit on a fixed per-hearth schedule and only ever debits what it minted; Great Works score participation (count of hearths that contributed a capped share), never quantity, with no leaderboard; compacts capped at 8-10; no free text (generated names, canned phrases, curated name-a-settler); County opens only above a hearth floor (~300); voice dropped from EA in favor of Steam/Discord overlay; a version-handshake and SteamCMD update script for dedicated servers.

**Net assessment.** Longthaw stays the recommendation because its Phase 1 is the one multiplayer model the evidence supports and its mitigations are all cuts and gates rather than new systems. The pitch's calendar does not survive; the concept does.

### 6.2 Weirhold (avg 8.00)

**Hook and loop.** Carrow Weir: a concrete dam, lock and powerhouse as the settlement; the reservoir level is the county's biggest lever, draining a drowned mill town or flooding it. The same hearth loop as Longthaw with ice, skating and a boat; the Intake as the PvE Dark Zone; a weekly freeze/thaw. Phase 2: the County Board (caravans with escrow, compacts of 20, a seasonal Great Work, visitable snapshots) opens in the first or second EA update.

**Multiplayer model.** Identical to Longthaw: listen server plus free dedicated binary over SDR, Board as a thin axum/sqlx service on one Hetzner box.

**Comparables and market evidence.** The same set ([Duckov](https://en.wikipedia.org/wiki/Escape_from_Duckov), [ZERO Sievert](https://underdog.ltd/presskit.html), [Valheim](https://steamdata.ai/game/892970/valheim), [Abiotic Factor](https://games.gg/news/abiotic-factor-million-sales/), [Necesse](https://massivelyop.com/2025/11/09/sandbox-mmo-lite-necesse-has-sold-an-eye-popping-2-million-copies-so-far/)); the [82% co-op share of paid online-MP hits](https://github.com/leinstay/steamdb); the [For Honor finding that over half of live-period players were friend-invited](https://github.com/dklassic/GDC-transcript/blob/master/static/src/transcript/EOJ1Y3xRpwc.txt).

**12-month scope.** Q1 sim crate and Bevy client with one kit and Steam page by mid-December; Q2 SDR, 4-player, dedicated binary, three kits, Deck pass, alpha of 50; Q3 Intake, two more kits, prediction, Next Fest, Board as dark feature, gate at 15k wishlists; Q4 EA late September 2027 at $19.99 with six kits, Board in closed beta. Year-one cash under $8k.

**Skeptic verdicts.**

*Market: refuted (medium).* Developer net per $19.99 unit is [~$8-9 after Steam's cut, VAT and refunds](https://www.steampageanalyzer.com/blog/steam-revenue-share-explained), so $100k net needs ~11,500-12,500 units (190-400 reviews) and the 7-15k gate yields $10-60k net at [0.07-0.13x EA conversion and 2.46-3.6x year-one multiplier](https://newsletter.gamediscover.co/p/what-long-tail-should-you-expect). In the catalog dump, the 2024-25 cohort of paid $14.99-29.99 online co-op survival-building launches (N=39) has a median of 424 reviews and 41% at 1,000+, but self-published members (N=20) median 311 with 8 under 100, and first-time self-published (N=5) median 13. [Nested Lands (veteran team plus META Publishing, EA Feb 2026) sits at an est. $105.9k gross after 7 months](https://raijin.gg/app/2553040/Nested_Lands). Every solo comp above $100k had a publisher or PR partner ([Romestead: 250k wishlists, Three Friends publisher, est. $1.1M in 8 weeks](https://raijin.gg/app/1805320/Romestead)). Mitigation: 25k-wishlist gate, a free "Weirhold: Prologue" in the October 2027 Next Fest with the dam-drain as the trailer shot, EA Q1 2028, publisher talks at 5k wishlists, page localization (ZH/RU/DE/PT-BR).

*Ship: refuted (high).* The budget of ~2,560 hours is below [ZERO Sievert's ~4,000 hours for the single-player half alone](https://howtomarketagame.com/2023/01/18/how-a-pixel-art-shooter-achieved-a-million-dollar-launch-the-zero-sievert-story/) and 15-20x below [Abiotic Factor's ~2.5 years with 10-12 people](https://en.wikipedia.org/wiki/Abiotic_Factor); [Sapiens, the nearest custom-engine settler-sim-plus-multiplayer analogue, took a decade solo](https://majic-jungle.prezly.com/sapiens-press-kit). [bevy_replicon has no prediction and its only prediction extension is unmaintained](https://github.com/projectharmonia/bevy_replicon). [Mike Bithell was hospitalized a year into full-time indie work](https://www.mcvuk.com/development-news/how-mike-bithells-never-ending-crunch-on-volume-sent-him-to-hospital/); [Barone burned out after Stardew](https://www.shortform.com/blog/how-was-stardew-valley-made/). Mitigation: EA at month 15-18 gated on a month-9 demo with 7k+ wishlists; 2-3 kits and 8-12 hours at EA; interpolation-only combat tuned for 80-150 ms; freeze Bevy for Q2-Q4; Board as a year-2 product.

*MP model: refuted (medium-high).* Save-editing tools appear within weeks for every player-hosted game ([palworld-save-tools 869 stars](https://github.com/cheahjs/palworld-save-tools), [valheim-save-tools](https://github.com/Kakoen/valheim-save-tools)), so Board inputs are attacker-controlled and "no settlement is ever wiped" removes the only remedy. The dual listen/dedicated build is the shape of [Palworld's host-save GUID migration bug (945 stars, 68 open issues)](https://github.com/xNul/palworld-host-save-fix). [LFGSS shut down rather than carry OSA liability](https://www.techdirt.com/company/microcosm/). Mitigation: Board-minted scrip with fixed-rate conversion and daily caps, cooperative thresholds instead of contribution ranks, preset-phrase mail, Steam ID as the sole identity in both server modes, 2-3 h/week of named ops.

**Net assessment.** Weirhold is Longthaw with a stronger single image (the drain) and weaker gates. Its mitigations are the same as Longthaw's; the reservoir lever is worth stealing as a Season-2 Great Work.

### 6.3 Ashward (avg 7.67)

**Hook and loop.** A burned-over timber valley regreening tile by tile as 48 player towns plus NPC towns grow; a town is an agent-simulated diorama with a ledger and shortfall forecast; salvage parties resolve on printed odds; one valley day every 10 real minutes; fire season and winter as shared pressure; great works that need several towns; towns can never be lost. Phase 2 (H1 2028): the ruin hex next to your town becomes a 20 Hz 1-4 player instance; the Burn as the opt-in contested zone.

**Multiplayer model.** Developer-hosted persistent shards from day one (one Rust process per 48-town valley, WebSocket-first over axum, event-sourced Postgres), plus a free private-valley binary on SQLite with a configurable tick.

**Comparables and market evidence.** [Eco ~683k / 74.9h](https://github.com/VIV-T/BUT3_Developpement_web/tree/64d0a457f8f9c5a159c424cd2d9974d0764d4e9b/1_Preparation_Donnees/donneesBrutes/scrapping_gamesData_Gamalytics), [Kingdoms Reborn ~572k](https://github.com/VIV-T/BUT3_Developpement_web/tree/64d0a457f8f9c5a159c424cd2d9974d0764d4e9b/1_Preparation_Donnees/donneesBrutes/scrapping_gamesData_Gamalytics), [Foxhole 44,068 reviews](https://github.com/leinstay/steamdb), [OpenFront's 190k wishlists](https://github.com/openfrontio/OpenFrontIO/blob/main/resources/public/press/index.html), the [4X tag's 29.7-36.8% hit rate on 19-49 releases](https://steamdb.info/) and [Massively Multiplayer's $6.8M top-100 median](https://vginsights.com/).

**12-month scope.** Q1 plain-Rust valley sim plus tokio/axum/sqlx server, egui viewer, Bevy 2D client, five friends on a 60x dev valley, Steam page mid-December; Q2 NPC towns, market, caravans, great work, private-valley binary, agent-built phone web client, closed alpha of 50-100; Q3 Steam auth, Deck pass (100 h), Valley API, bot load tests, Next Fest demo valley at 4x speed; Q4 EA late September 2027 at $19.99 with two official valleys on one Hetzner box, Phase 2 spike with the alpha group. One stack per quarter (plain Rust, then Bevy, then Steamworks, then replication).

**Skeptic verdicts.**

*Market: refuted (medium).* The shipped product is a slow-tick 2D async town game whose one solo precedent, [Ymir (solo, $34.99, EA since 2019-03-16), has 1,079 reviews at 76%, one review in the last 30 days and is still in EA after 7.5 years](https://store.steampowered.com/app/378360/Ymir/) with an estimated [$594k lifetime gross](https://github.com/leinstay/steamdb) after five years of pre-launch work. [Screeps is estimated at $570k-$1.45M lifetime across ten years](https://raijin.gg/app/464350/Screeps_World); [simulogics called the Prosperous Universe subscription a "necessity" because funding was running out](https://massivelyop.com/?p=311532). Among paid $9.99-44.99 MMO-category releases of 2024-25, ten reached 250 reviews and nine are funded studios; paid "Asynchronous Multiplayer" releases have a median of 28 reviews (n=188). The browser-demo funnel cannot assume OpenFront's 200k DAU. Mitigation: invert the order so the EA SKU is a co-op town builder for 1-4 friends on private valleys with official valleys as a second mode, pull a 10-15 minute embodied salvage slice into the Next Fest demo, raise the gate to 25k wishlists or 2k sustained browser DAU, budget year one at $40-80k net.

*Ship: refuted (medium-high; the 3-month slice survives).* The exact niche has [82 persistent-server strategy/builder titles in the catalog, 37 paid, 7 ever reaching 1,000 reviews](https://github.com/leinstay/steamdb); every multiplayer-builder comp is a multi-year EA by a team; the closest Rust analogue, [parabellum, is 22 months into its rewrite and still incomplete](https://github.com/andreapavoni/parabellum); [OpenFront took 25 months and 279 commit authors to reach Steam](https://github.com/openfrontio/OpenFrontIO); [OpenMMO's 2,622 commits include 1,619 co-authored by Claude](https://github.com/Julian-adv/OpenMMO), evidence that an agent-assisted solo server is possible but not that fun is found faster, because a 10-minutes-a-day valley is validated only by running real-speed valley years with humans. Mitigation: ship the private valley, not the official shard, in year one (agent-built web client only, no Bevy, no Deck, no official valleys), replace the wishlist gate with a fun gate (three friend groups each complete a valley year and start another), official valleys and the Bevy client in year two.

*MP model: refuted (medium-high).* Fixed 48 permanent slots plus no wipes plus great works that need several towns cannot all hold once Steam decay reaches a valley: [Eco shows 26 reviews in the last 30 days against 13,212 lifetime, Kingdoms Reborn 33 of 9,113, Prosperous Universe 9 of 1,109, BitCraft 51 of 3,757 at 53% recent positive](https://github.com/leinstay/steamdb), so a 48-human valley becomes 5-10 humans and 40 frozen ghost towns that NPC towns cannot replace. Every long-lived slow-tick game broke one of these promises ([Screeps' operator-managed novice and respawn areas](https://docs.screeps.com/start-areas.html); Travian rounds with a multihunter staff role). Slow-tick strategy is the most-botted genre on GitHub ([214 "travian bot" repos](https://github.com/Erol444/TravianBotSharp), [140 "ogame bot"](https://github.com/ogame-tbot/TBot)); [OpenFront needed a Gatekeeper module, Turnstile, fingerprinting and persistent-ID bans](https://github.com/openfrontio/OpenFrontIO). Hetzner dedicated datacenters are [Germany and Finland only](https://github.com/elsbrock/hetzner-radar), so "EU and NA on one box" puts NA on ~100-130 ms. Mitigation: valley seasons of 12 real weeks with a persistent hearth carried forward; dormancy after 14 days (town held in trust by NPCs, reclaimable); elastic valleys opening at 12-16 humans; an attention budget identical for every town so scripted check-ins gain nothing; Steam-ownership-only official valleys; a moderation and OSA/DSA floor before any public server; a US-region box.

**Net assessment.** Ashward is the best plan if the founder decides he does not want to ship combat, and its Rust curriculum is the deepest. But its commercial claim fails on the one solo comp that exists, and its shard design fails on population decay. Demoted to third; its dormancy-and-seasons design and its one-stack-per-quarter sequencing are adopted into the recommendation.

### 6.4 DEADWEIGHT (avg 6.33)

**Hook and loop.** "Escape from Duckov meets Valheim" in a drowned container port: every run is a bet on how much you can carry before the tide comes in; sealed cargo must be winched at a noisy crane; range-limited proximity voice; a settlement on timers at The Stacks. Player-hosted 1-4 co-op, $17.99 EA at month 12.

**Multiplayer model.** Same listen-plus-dedicated model over SDR; an opt-in 8-player Rogue Tide flag post-launch, community-hosted only.

**Comparables and market evidence.** [Duckov](https://en.wikipedia.org/wiki/Escape_from_Duckov), [ZERO Sievert](https://underdog.ltd/presskit.html), [Lethal Company 487,160 reviews, solo](https://vginsights.com/game/1966720); [co-op median 27 vs PvP-only 9 reviews](https://github.com/leinstay/steamdb); the [Forever Winter water-timer backlash](https://www.pcgamer.com/games/third-person-shooter/the-forever-winter-my-favourite-extraction-shooter-just-overhauled-its-most-contentious-feature-for-the-second-time-it-was-a-hell-of-a-rollercoaster-to-make-the-adjustment/) as the reason the tide only runs in-session.

**12-month scope.** Month 3: sim crate, lightyear over SDR with 2 players, one district kit, deadweight, tide, boat, ledger. Month 6: 4 players, three districts, settlers on a dedicated server, crane extraction, proximity voice, Steam page. Month 9: Next Fest demo, gear crafting, storm reshuffle, refinery district, Docker image. Month 12: EA at $17.99, five districts, 20-25 hours.

**Skeptic verdicts.**

*Market: refuted (medium).* In the catalog dump, paid + Online Co-op + EA + $15-19.99 + self-published 2024 launches (n=33) have a median of 139 lifetime reviews, 27% reached 300 and 12% reached 1,000; calibrating with [HumanitZ at ~$396 gross per review](https://gamalytic.com/game/1766060), $100k gross needs 180-250 reviews inside 12 months, a 70th-80th percentile outcome. The only DEADWEIGHT-shaped hit, HumanitZ (5,693 reviews, est. 148,574 copies / $2.25M), is a two-person team plus publisher after ~3 years in Unity. Drowned-city comps are weak ([Floodland 1,001 reviews at 56%](https://store.steampowered.com/app/2080690); Flotsam 2,746 over five years of EA). Bevy's own [sixth-birthday post lists nine shipped titles with 2-60 reviews each](https://bevyengine.org/news/bevys-sixth-birthday/). Mitigation: page by month 3, free co-op demo in two Next Fests, EA only at 20-25k wishlists, density over breadth, publisher pitch after the first fest, plan on 5-10k copies and $50-80k gross.

*Ship: refuted (high).* [lightyear shipped ten breaking minor releases between July 2025 and September 2026](https://github.com/cBournhonesque/lightyear/releases); [Steam in headless mode was broken from 2025-08-12 to 2026-08-03 (#1156)](https://github.com/cBournhonesque/lightyear/issues/1156) and [running Steam IO beside UDP IO silently disables server input processing (#1693, open)](https://github.com/cBournhonesque/lightyear/issues/1693); [no Rust voice bindings exist](https://raw.githubusercontent.com/Noxime/steamworks-rs/master/src/user.rs); [Bevy system order is nondeterministic by default](https://github.com/bevy-cheatbook/bevy-cheatbook/blob/main/src/programming/system-order.md), which collides with "deterministic catch-up"; [Veloren, the reference Rust authoritative multiplayer game, is at 0.18 after eight years and 18,558 commits](https://github.com/veloren/veloren). Mitigation: pin Bevy 0.20 for the whole year, bevy_replicon with interpolation only, drop lightyear, drop voice and walking settlers from EA, 2-player over plain UDP by month 3 with SDR at month 4-5, EA Q1 2028.

*MP model: refuted (medium).* The 24/7 promise exists only on the dedicated binary, which is a second product with a service contract ([Palworld's Docker image carries 100+ env vars and still logs join failures after updates](https://github.com/thijsvanloef/palworld-server-docker/issues/947)); [aeronet_steam ships its session config marked "CURRENTLY BROKEN" because of steamworks-rs #169](https://github.com/aecsocket/aeronet/blob/main/crates/aeronet_steam/src/config.rs); most 1-4 friend groups will run the listen server, where the game is host-owned co-op plus an offline timer. Mitigation: make offline progression a deterministic event-timer ledger that advances identically on any load; world save in Steam Cloud with explicit host handoff; dedicated binary as a post-EA community milestone with a minimal contract; a transport trait so Steam IO can swap between crates.

**Net assessment.** The most streamable design and the cleanest hosting model, but it spends the founder's hours on tide shaders, voice FFI and content density, and its shared-world pillars are gone. Its tide clock, proximity-voice intent and "progress between sessions, not 24/7" framing are folded into the recommendation; the pitch itself stays fourth.

## 7. Recommendation

**Concept: Longthaw, mitigated.** A $19.99 top-down 1-4 player scavenging game with a settlement that progresses between sessions, in a county that thaws week by week on a reseeded 12-week season. Phase 1 (the hearth) is the product; the County is a free Season-1 update, redesigned as a participation layer. Changes versus the pitch, all taken from the skeptic mitigations: EA scope is hearth + three ruin kits + one or two enemy families + 1-4 player listen server, 10-12 hours; the dedicated binary ships at EA+60 days with a version handshake and SteamCMD script; the Meltline is the first EA update; the County arrives at EA+6 months, mints its own credit on a fixed per-hearth schedule, scores Great Works by participation with no leaderboard, caps compacts at 8-10, carries no free text, and opens only above ~300 pinned hearths; in-game Steam voice is not promised (Steam/Discord overlay plus positional pings); offline progression is a pure deterministic catch-up function that is identical on listen and dedicated servers, marketed as "your base progresses between sessions"; the year-2 open hearths for strangers are deferred indefinitely. The name needs a Steam, trademark and domain search before the page goes live (nobody budgeted it; two days).

**Engine.** Bevy client plus Bevy headless server sharing one engine-free simulation crate, with three rules that address the churn. (1) Pin one Bevy version for the year: adopt 0.20 once it stabilizes (rc.2 shipped [2026-09-28](https://crates.io/api/v1/crates/bevy)), upgrade once before EA, budget ~40 hours per upgrade, and skip 0.21/0.22 until after launch. (2) Keep the simulation crate free of Bevy types and the wire protocol engine-agnostic (postcard over SDR/QUIC), so the Rust learning curve lives in the sim, the server and the persistence layer rather than in the client, and so a Godot 4.7 client via gdext remains a fallback if the Bevy client is not playable with two players by month 4; the founder-fit judge's point stands that a thin client protects the curriculum, and this is the thick-client plan with the same protection. (3) Netcode: bevy_replicon with server-authoritative replication and client interpolation only, combat tuned to feel right at 80-150 ms, no lightyear and no prediction in year one; one Steam transport path chosen in week 1 with a single steamworks version pinned workspace-wide and a CI job that fails on duplicates (the [renet_steam 0.12.2 vs bevy_steamworks 0.13.1 conflict](https://raw.githubusercontent.com/lucaspoffo/renet/master/renet_steam/Cargo.toml) is real); Avian over rapier; no wtransport, WebTransport or WASM on the critical path. Determinism must be specified, not asserted: integer or fixed-point economy math, explicitly ordered systems, no float crossing a native/WASM boundary. Learning rule: the founder writes the first version of every novel subsystem (sim core, replication, persistence, catch-up) by hand and reads every agent diff in those crates; agents own tests, fixtures, bots, CI, tooling and the County service; measure hours per merged feature for the first eight weeks and re-plan from the observed multiplier, budgeting 1.2x until measured.

**Platforms.** Steam first: Windows, Steam Deck "Playable" at EA via Proton (gamepad UI from Q2, Vulkan from wgpu) with Verified when Valve's queue allows, native Linux as a near-free bonus. No web client, no mobile app, no consoles in the first 12 months; a plain-HTML County page for phone check-ins when the County opens.

**Art direction.** Top-down three-quarter diorama with a fixed camera; one or two low-poly modular kits (Synty/Kenney class) recolored to a single palette, varied by procedural placement, snowpack depth, meltwater level and a weathering shader; the thaw is the look (white and slate in week one, ochre and green by week twelve); one commissioned settler rig and animation set so the ant farm has charm; one commissioned capsule and key art set. Mod surface stops at data tables because pack licenses prohibit redistributing meshes. Year-one cash under $8k.

**Monetization.** Premium only, Valve as merchant of record: $19.99 at EA (never lower; [sub-$15 online games reach 1,000 reviews under 8% of the time](https://github.com/leinstay/steamdb)), $24.99 at 1.0, a supporter edition, cosmetics after 1.0, no F2P, no subscription, no timers, no wipes. Honest per-unit model: ~$12.40 gross and ~$7.40 net after regional pricing, Steam's cut, VAT and refunds, so $100k gross is ~8,000 year-one units and needs ~30k launch wishlists at median conversion.

**Multiplayer model.** Listen server embedded in the host's client (solo is the same server on localhost), free headless dedicated binary, Steam Datagram Relay and lobbies, friends-list invites only, no matchmaking, no strangers, nothing that needs anyone else online. The County is a low-tick meta-service syncing kilobytes per hearth per hour and never running simulation.

**Hosting cost.** Simulation: $0 (player-hosted; SDR is free to Steam partners, medium confidence since partner docs were blocked). County: one Hetzner cloud or AX-class box at roughly EUR 15-50 per month, Postgres on-box, offsite backups ~$5 per month, domain $15 per year, under $700 for the year (provider prices unverified, low confidence). For comparison, the [AWS persistent-world reference costs $1.80 per CCU-month at 1,500 CCU with 64% egress](https://github.com/aws-solutions-library-samples/guidance-for-persistent-world-game-hosting-on-aws); the derived Hetzner-class figure is $0.05-0.50 per CCU-month. Founder ops: ~3 hours per week after the County opens, one restore drill per quarter.

**Twelve-month milestones (October 2026 to September 2027, ~2,400-2,560 productive hours).**

- *Q1, Oct-Dec 2026.* Rust ramp (expect 200-300 hours of slower-than-planned work). Plain-Rust sim crate with no engine and no async: settlers, jobs, typed components, buildings, ledger, thaw-stage model, fixed step, proptest conservation, versioned serde saves. egui viewer by week 5; Gate 0 at week 6: is the ant farm fun on its own? If not, the settlement half is the product and the expedition half is cut, not the reverse. Bevy 0.20 client with one ruin kit, movement, deadweight, first combat loop against two enemy types; two-player replicon over local UDP by December. Thursday test group from week 2. Art direction locked by week 8; Steam page live by week 10 with capsule art, six screenshots, a 45-second trailer; weekly devlog. Name clearance. Two days re-verifying Duckov, ZERO Sievert, Kingdoms Reborn and Eco from primary sources. Measure the agent multiplier. Gate 1 at month 3: a 30-minute run is fun alone (3 of 4 testers) or pivot to valley-first on the same crate.
- *Q2, Jan-Mar 2027.* One Steam transport path (SDR, lobbies, auth tickets), 4-player drop-in/drop-out, two more kits, five settler jobs, gear and perks with printed math, the weekly thaw stage, save-migration harness in CI, deterministic catch-up, Deck gamepad pass (80 h, tested monthly on hardware), closed alpha of ~50, late-March City Builder and Colony Sim Fest. Gate 2 at month 6: the Thursday group plays unasked and D7 return above 30% in the alpha; publisher outreach (Hooded Horse, Modern Wolf, Fireshine class) at 5k wishlists.
- *Q3, Apr-Jun 2027.* Free standalone Prologue (own app ID: the hearth, one kit, 2-player co-op, 2-4 hours) submitted to the June Next Fest with a co-op streamer key drop and shareable hearth postcards; performance pass; Meltline prototype; County v1 built by agents as a dark feature in its mitigated form. Gate 3 at end of June: 25-30k wishlists or a signed publisher means EA in September; 10-25k means EA in Q1 2028 after the October Next Fest; under 10k means keep iterating the Prologue and treat 2028 as a sub-$50k year.
- *Q4, Jul-Sep 2027.* Content freeze three weeks before any launch, refund-window polish, second trailer. If Gate 3 is green: EA in the second half of September at $19.99 with three kits and 10-12 hours, dedicated binary at EA+60 days. Otherwise: Prologue stays live as the permanent front door, Deck pass completed, the Meltline and a fourth kit land in the demo, EA in Q1 2028. Minimum shippable build at ~2,000 hours if the pace fails: hearth + two kits + 2-player co-op + Prologue.

**Sustainable pace at 58 hours a week.** Do not plan 58. Plan 45-50 with one full day off and every sixth week a deload; that is the ~2,400-2,560 productive hours the pitch already assumed, so nothing is lost on paper and the plan stops depending on the band where [Pencavel's data shows per-hour output falling](https://docs.iza.org/dp8129.pdf) and [WHO/ILO show the health risk rising](https://www.who.int/news/item/17-05-2021-long-working-hours-increasing-deaths-from-heart-disease-and-stroke-who-ilo). [GDC's 2026 survey found 94% of developers reporting at least one burnout symptom while only 11% work over 50 hours](https://respawn.outlookindia.com/amp/story/gaming/gaming-originals/2026-gdc-survey-exposes-workforce-strain-and-technological-shifts). Month one, the steepest part of the Rust curve, should be a short week. Gates slip dates and never add hours; if that rule is abandoned, this is no longer the plan. If any part of the 3,000 hours sits on top of a day job rather than instead of one, every schedule above slips by at least a quarter. Name the cash runway now: there is no revenue before EA, and the June gate is where financial pressure will try to override the data.

## 8. Decisions the founder must make now

1. **Scavenger first (Longthaw) or valley first (Ashward)?** Default: Longthaw. Its Phase 1 is the only multiplayer model the evidence supports and it ships the Tier 2 loop he plays most. Choose Ashward only if, after reading both, he would rather run a living valley than play Thursday-night runs, and accept that the embodied layer is then 15-18 months out.
2. **Hold September 2027 for paid EA, or gate it?** Default: gate it. Twelve months buys a Steam page, a free Prologue, two Next Fests and a wishlist pile; EA ships at 25-30k wishlists or with a publisher, realistically Q1 2028. A thin EA cannot be fixed later ([only 20% of graduates do better at 1.0](https://gameworldobserver.com/2025/12/10/according-to-gamediscoverco-most-games-on-steam-earn-less-after-leaving-early-access-than-they-do-at-launch); [0.156% recovered from a weak launch](https://howtomarketagame.com/2026/02/12/only-28-games-recovered-from-a-bad-launch-in-2024-what-do-they-have-in-common/)).
3. **Which code does the founder write by hand?** Default: the first version of the sim core, replication, persistence and catch-up, reading every agent diff in those crates; agents own tests, fixtures, bots, CI, tooling and the County service. Without this rule the 3,000 hours produce a shipped game and a shallow Rust education.
4. **Bevy client or Godot client?** Default: Bevy pinned to 0.20 for the year, engine-free sim crate, postcard protocol, and a hard fallback to Godot via gdext if two players cannot walk a ruin together by month 4.
5. **Hours and runway.** Default: cap at 45-50 hours per week, log actual hours, and write down the cash runway and the month it runs out before setting any gate. Is any of this on top of a day job? That answer changes every date in Section 7.
6. **Publisher or self-publish?** Default: self-publish the Prologue and the page, but open conversations with Hooded Horse / Modern Wolf / Fireshine-class publishers at 5k wishlists for marketing and porting only, never for design control or a wipe-based live service. Every solo comp above $100k in year one had one.

## 9. Sources

Lane and judge inputs (deduplicated; figures marked unverified in the text were taken from these pages via search snippets or memory because the pages were egress-blocked):

- https://howtomarketagame.com/2026/01/27/what-the-hell-happened-in-2025/
- https://howtomarketagame.com/2025/01/15/what-the-hell-happened-in-2024/
- https://howtomarketagame.com/2026/02/12/only-28-games-recovered-from-a-bad-launch-in-2024-what-do-they-have-in-common/
- https://howtomarketagame.com/2025/08/04/2025-q2-games-that-are-selling/
- https://howtomarketagame.com/2023/01/18/how-a-pixel-art-shooter-achieved-a-million-dollar-launch-the-zero-sievert-story/
- https://howtomarketagame.com/?p=4587
- https://howtomarketagame.com/2026/06/25/archive-how-many-wishlists-should-i-have-when-i-launch-my-game/
- https://www.gamesradar.com/games/a-terrifying-20-282-games-were-released-on-steam-in-2025-and-just-608-managed-to-get-1-000-reviews-expert-finds-we-might-be-in-a-bit-of-an-indie-golden-age/
- https://newsletter.gamediscover.co/p/which-genre-should-your-next-pc-game
- https://newsletter.gamediscover.co/p/the-state-of-steam-early-access-graduates
- https://newsletter.gamediscover.co/p/how-abiotic-factor-sold-600k-copies
- https://newsletter.gamediscover.co/p/analysis-multiplayer-game-discovery
- https://newsletter.gamediscover.co/p/the-state-of-steam-wishlist-conversions
- https://newsletter.gamediscover.co/p/what-long-tail-should-you-expect
- https://newsletter.gamediscover.co/p/what-happened-to-early-access-graduates
- https://gamedevreports.substack.com/p/gamalytic-67-of-games-on-steam-earned
- https://gamedevreports.substack.com/p/gamediscoverco-games-long-tail-revenue
- https://gameworldobserver.com/2025/12/10/according-to-gamediscoverco-most-games-on-steam-earn-less-after-leaving-early-access-than-they-do-at-launch
- https://gameworldobserver.com/2024/02/07/paradox-interactive-record-revenue-cities-skylines-2-sales
- https://gameworldobserver.com/2023/01/23/zero-sievert-1-million-revenue-cabo-studio-extraction-shooter-steam
- https://gameworldobserver.com/tag/cabo-studio
- https://www.gamespress.com/Manor-Lords-celebrates-three-million-sales
- https://www.gamespress.com/ko-KR/against-the-storm-2-million-copies-sold-milestone
- https://www.gamespress.com/Medieval-Survival-RPG-Bellwright-Surpasses-1-Million-Units-Sold-on-Ste
- https://www.gamespress.com/Free-to-Play-MMOFPS-STALCRAFT-X-Celebrates-10th-Anniversary-with-Huge-
- https://www.unrealengine.com/en-US/developer-interviews/inside-kingdoms-reborn-s-game-dev-s-journey-of-discovery-and-city-building
- https://steamcommunity.com/app/1307890/discussions/0/3823048293511738403/
- https://steambase.io/games/the-settlers-new-allies/steam-charts
- https://news.ubisoft.com/en-us/article/7FK06qQE7cN6hvjbVJrI0i/anno-117-pax-romana-celebrates-117-million-players
- https://www.gamedeveloper.com/business/study-finds-co-op-games-keep-growing-in-numbers-and-sales-on-steam
- https://www.gamedeveloper.com/business/escape-from-duckov-has-sold-2-million-copies-in-two-weeks
- https://www.gamedeveloper.com/business/yager-sunsetting-the-cycle-frontier-after-cheaters-cause-irreparable-damage
- https://www.gamedeveloper.com/business/curve-games-boasts-strong-sales-for-i-the-ascent-i-and-i-human-fall-flat-i-
- https://www.gamedeveloper.com/design/valuable-but-not-profitable-garden-wars-postmortem
- https://wnhub.io/news/stores-and-publishing/item-48428
- https://wnhub.io/news/stores-and-publishing/item-51221
- https://wnhub.io/news/other/item-1533
- https://store.steampowered.com/app/3337140/After_Inc_Revival/
- https://store.steampowered.com/app/3527290
- https://store.steampowered.com/app/3164500
- https://store.steampowered.com/app/892970
- https://store.steampowered.com/app/378360/Ymir/
- https://store.steampowered.com/app/2080690
- https://pokeindie.com/blog/game-genre-study-2026-which-genres-are-worth-building
- https://voxbooster.com/blog/indie-game-statistics-2026/
- https://steamdata.ai/game/892970/valheim
- https://games.gg/news/abiotic-factor-million-sales/
- https://massivelyop.com/2025/11/09/sandbox-mmo-lite-necesse-has-sold-an-eye-popping-2-million-copies-so-far/
- https://massivelyop.com/2015/06/23/excessive-duping-leads-to-ark-survival-evolved-wipe/
- https://massivelyop.com/?p=311532
- https://levvvel.com/sunkenland-statistics/
- https://www.gamesradar.com/palworld-dev-ceo-jokes-server-fees-could-bankrupt-the-studio-after-it-spends-dollar478000-to-never-let-the-service-go-down-no-matter-what/
- https://steamcharts.com/app/1928980
- https://www.pcgamer.com/games/survival-crafting/despite-making-significant-changes-to-survival-game-nightingale-developer-inflexion-games-says-it-hasnt-been-commercially-successful-enough-to-avoid-dozens-of-layoffs/
- https://www.pcgamer.com/games/action/escape-from-duckov-quacks-past-300-000-concurrent-players-only-10-days-after-release/
- https://www.pcgamer.com/games/third-person-shooter/the-forever-winter-my-favourite-extraction-shooter-just-overhauled-its-most-contentious-feature-for-the-second-time-it-was-a-hell-of-a-rollercoaster-to-make-the-adjustment/
- https://pindrop.gg/bellwright/guides/multiplayer
- https://app.sensortower.com/vgi/game/vein
- https://app.sensortower.com/vgi/game/enshrouded
- https://underdog.ltd/presskit.html
- https://en.wikipedia.org/wiki/Escape_from_Duckov
- https://en.wikipedia.org/wiki/Abiotic_Factor
- https://en.wikipedia.org/wiki/SimCity_(2013_video_game)
- https://heroiclabs.com/blog/marauders-case-study/
- https://www.gameshub.com/news/article/marathon-sales-estimate-1-2-million-copies-steam-ps5-2026-2862962/
- https://gamingbolt.com/arc-raiders-has-sold-more-than-12-4-million-copies
- https://www.shanethegamer.com/pc-mac/the-division-resurgence-lands-on-steam-splits-players-with-46-positive-score/
- https://gamerant.com/steam-charts-co-op-games-top-5-dominating-2025/
- https://vortexgaming.io/en/postdetail/498074
- https://github.com/clockworklabs/SpacetimeDB/releases/tag/v1.0.0
- https://github.com/clockworklabs/BitCraftPublic
- https://raw.githubusercontent.com/clockworklabs/SpacetimeDB/master/LICENSE.txt
- https://github.com/Julian-adv/OpenMMO
- https://github.com/andreapavoni/parabellum
- https://github.com/lanedirt/OGameX
- https://github.com/Dylan-Zheng/Rise-of-Kingdoms-Bot
- https://github.com/screeps/docs/blob/master/source/control.md
- https://github.com/screeps/docs/blob/master/source/tos.md
- https://github.com/screeps/screeps
- https://docs.screeps.com/start-areas.html
- https://github.com/clapfoot/warapi
- https://github.com/longturn/freeciv21/blob/master/docs/Contributing/game-admin.rst
- https://github.com/simulogics/prosperousuniverse-docs
- https://sensortower.com/blog
- https://github.com/VIV-T/BUT3_Developpement_web/tree/64d0a457f8f9c5a159c424cd2d9974d0764d4e9b/1_Preparation_Donnees/donneesBrutes/scrapping_gamesData_Gamalytics
- https://github.com/kgs112233/dataproject5team/blob/189aaa24dfbfe79b634e29af488beb78f7232d99/output/output_after.csv
- https://github.com/rwmt/Multiplayer/releases
- https://github.com/RimWorld-Together/Rimworld-Together
- https://github.com/thomaswp/BeaverBuddies/wiki
- https://github.com/timbermods/TimberTogether
- https://github.com/onimp/oni_multiplayer
- https://github.com/CitiesSkylinesMultiplayer/CSM/issues?q=is%3Aissue+desync
- https://github.com/NebulaModTeam/nebula
- https://github.com/dklassic/GDC-transcript/blob/master/static/src/transcript/YoOCUpdYYm4.txt
- https://github.com/dklassic/GDC-transcript/blob/master/static/src/transcript/EOJ1Y3xRpwc.txt
- https://github.com/dklassic/GDC-transcript/blob/master/static/src/transcript/bcXxyKqgV0c.txt
- https://github.com/dklassic/GDC-transcript/blob/master/static/src/transcript/0M0xBMEuWdU.txt
- https://github.com/dklassic/GDC-transcript/blob/master/static/src/transcript/W52Lm505300.txt
- https://github.com/dklassic/GDC-transcript/blob/master/static/src/transcript/zto7ZP2-ooM.txt
- https://github.com/dklassic/GDC-transcript/blob/master/static/src/transcript/jdc9-N48Cl4.txt
- https://github.com/escapingnetwork/core-keeper-dedicated
- https://github.com/ValveSoftware/GameNetworkingSockets
- https://github.com/aws-solutions-library-samples/guidance-for-persistent-world-game-hosting-on-aws
- https://github.com/lloesche/valheim-server-docker
- https://github.com/thijsvanloef/palworld-server-docker
- https://github.com/thijsvanloef/palworld-server-docker/issues/947
- https://github.com/xNul/palworld-host-save-fix
- https://github.com/cheahjs/palworld-save-tools
- https://github.com/Kakoen/valheim-save-tools
- https://github.com/Impostor/Impostor
- https://blog.playstation.com/2024/09/03/an-important-update-on-concord/
- https://crates.io/api/v1/crates/bevy
- https://crates.io/api/v1/crates/lightyear
- https://crates.io/api/v1/crates/lightyear/versions
- https://crates.io/api/v1/crates/bevy_replicon
- https://crates.io/api/v1/crates/avian3d
- https://crates.io/api/v1/crates/fyrox
- https://github.com/bevyengine/bevy_editor_prototypes
- https://github.com/bevyengine/bevy
- https://github.com/bevyengine/bevy/releases
- https://raw.githubusercontent.com/bevyengine/bevy-website/main/content/news/2026-08-10-bevys-sixth-birthday/index.md
- https://raw.githubusercontent.com/bevyengine/bevy-website/main/content/news/2024-08-10-bevys-fourth-birthday/index.md
- https://bevyengine.org/news/bevys-sixth-birthday/
- https://github.com/cBournhonesque/lightyear
- https://github.com/cBournhonesque/lightyear/releases
- https://github.com/cBournhonesque/lightyear/issues/1156
- https://github.com/cBournhonesque/lightyear/issues/1693
- https://github.com/projectharmonia/bevy_replicon
- https://github.com/aecsocket/aeronet
- https://github.com/aecsocket/aeronet/blob/main/crates/aeronet_steam/src/config.rs
- https://github.com/Jondolf/avian
- https://github.com/Noxime/steamworks-rs
- https://github.com/Noxime/steamworks-rs/issues?q=voice
- https://github.com/Noxime/steamworks-rs/issues/169
- https://raw.githubusercontent.com/Noxime/steamworks-rs/master/src/user.rs
- https://raw.githubusercontent.com/lucaspoffo/renet/master/renet_steam/Cargo.toml
- https://raw.githubusercontent.com/HouraiTeahouse/bevy_steamworks/main/Cargo.toml
- https://github.com/bevy-cheatbook/bevy-cheatbook/blob/main/src/programming/system-order.md
- https://github.com/veloren/veloren
- https://gitlab.com/veloren/veloren
- https://developer.apple.com/documentation/safari-release-notes/safari-26_4-release-notes
- https://developer.apple.com/app-store/small-business-program/
- https://raw.githubusercontent.com/godot-rust/book/master/src/toolchain/export-web.md
- https://www.hetzner.com/cloud/
- https://github.com/elsbrock/hetzner-radar
- https://github.com/leinstay/steamdb
- https://github.com/openfrontio/OpenFrontIO
- https://github.com/openfrontio/OpenFrontIO/blob/main/resources/public/press/index.html
- https://github.com/openfrontio/OpenFrontIO/blob/main/src/client/Payments.ts
- https://github.com/Anuken/Mindustry
- https://www.businessofapps.com/marketplace/user-acquisition/research/cost-per-install/
- https://steamdb.info/
- https://vginsights.com/
- https://vginsights.com/game/3241660
- https://vginsights.com/game/526870
- https://vginsights.com/game/1966720
- https://vginsights.com/game/1363080
- https://vginsights.com/game/960090
- https://metr.org/blog/2025-07-10-early-2025-ai-experienced-os-dev-study/
- https://simonwillison.net/2025/Jul/12/ai-open-source-productivity/
- https://birchtree.me/blog/an-update-from-the-study-that-said-devs-were-actually-slower-with-coding-agents/
- https://docs.iza.org/dp8129.pdf
- https://www.who.int/news/item/17-05-2021-long-working-hours-increasing-deaths-from-heart-disease-and-stroke-who-ilo
- https://www.shortform.com/blog/how-was-stardew-valley-made/
- https://www.mcvuk.com/development-news/how-mike-bithells-never-ending-crunch-on-volume-sent-him-to-hospital/
- https://respawn.outlookindia.com/amp/story/gaming/gaming-originals/2026-gdc-survey-exposes-workforce-strain-and-technological-shifts
- https://www.steampageanalyzer.com/tools/regional-pricing-calculator
- https://www.steampageanalyzer.com/blog/how-many-wishlists-before-launch
- https://www.steampageanalyzer.com/blog/steam-revenue-share-explained
- https://gamedeveloper.com/business/valve-says-5-836-titles-earned-over-100-000-on-steam-in-2025
- https://www.uberstrategist.com/press-releases/the-last-plague-blight-by-solo-developer-original-studios-out-now-on-steam-in-early-access
- https://nodal.gg/game/tinkerlands-2617700
- https://gamesbeat.com/valheim-finally-hits-1-0-after-five-years-in-early-access-interview/
- https://80.lv/articles/relaxing-castle-builder-tiny-glade-has-finally-arrived-on-steam/
- https://majic-jungle.prezly.com/sapiens-press-kit
- https://raijin.gg/app/2553040/Nested_Lands
- https://raijin.gg/app/1805320/Romestead
- https://raijin.gg/app/464350/Screeps_World
- https://gamalytic.com/game/1766060
- https://diablo2.diablowiki.net/B.net
- https://primagames.com/news/duped-items-deleted-new-fallout-76-patch
- https://devtrackers.gg/eco/p/3eeb00d5-player-retention-ideas
- https://www.ofcom.org.uk/online-safety/the-online-safety-act-and-gaming-know-the-risks-know-the-rules-know-how-to-comply
- https://www.techdirt.com/company/microcosm/
- https://github.com/Erol444/TravianBotSharp
- https://github.com/ogame-tbot/TBot
