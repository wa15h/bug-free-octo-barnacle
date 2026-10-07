# Market Research Report: An Online Multiplayer Rust Game Built by One Founder

Prepared 2026-10-07 for the founder, a senior backend and data engineer new to game development and to Rust. Numbers first; every figure carries a link; estimator outputs and unverified figures are flagged.

## 1. Executive summary

Build **Longthaw**, the top-ranked concept (second-pass average 8.83 of 10): a player-hosted, 1-4 player, top-down scavenging game with a settlement that keeps working between sessions, set in a county that thaws week by week, sold on Steam at $19.99, with the Rust authoritative server shipped as a listen server inside the client plus a free dedicated binary. Build it on the skeptics' terms, not the pitch's. Three adversarial reviewers attacked the three load-bearing claims of each top-four concept (market, ship, multiplayer model) and refuted all twelve; nothing survived. The refutations do not kill Longthaw, but they change the plan: the twelve-month deliverable becomes a live Steam page, a free Prologue app and a wishlist pile rather than a paid Early Access launch; EA ships only at 25-30k wishlists or with a publisher, realistically Q1 2028; the County meta-layer is redesigned so a lying player-hosted server cannot harm it and carries no free text; in-game Steam voice and client prediction leave the critical path; the stack is frozen on one Bevy version; the week is capped at 45-50 hours. The market evidence that drove the choice is unchanged: online co-op survival-craft is the highest hit-rate lane on Steam ([20.8% of 2025 releases reached 1,000 reviews](https://howtomarketagame.com/2026/01/27/what-the-hell-happened-in-2025/) vs [2.99% Steam-wide](https://voxbooster.com/blog/indie-game-statistics-2026/)), the two nearest solo-scale comps ([ZERO Sievert, 600k+ copies](https://underdog.ltd/presskit.html); [Escape from Duckov, 3M in 23 days](https://en.wikipedia.org/wiki/Escape_from_Duckov)) shipped no multiplayer, and the player-hosted model has no hosting bill, no population floor and no anti-cheat burden. What the skeptics changed is year-one arithmetic: at the pitch's 15k-wishlist gate, published conversion rates yield roughly [$50k gross](https://newsletter.gamediscover.co/p/the-state-of-steam-wishlist-conversions), not $100k.

| Rank | Concept | Lens | Avg (commercial / feasibility / founder-fit) | Skeptic claims survived |
|---|---|---|---|---|
| 1 | Longthaw | merge | 8.83 (9 / 9 / 8.5) | 0 of 3 |
| 2 | Weirhold | merge | 8.00 (8.5 / 8 / 7.5) | 0 of 3 |
| 3 | Ashward | merge | 7.67 (5.5 / 8.5 / 9) | 0 of 3 |

The fourth finalist, DEADWEIGHT (6.33), also survived 0 of 3.

## 2. Founder profile, taste DNA, and the budget correction

**Profile.** Solo developer plus AI coding agents; 25 years of backend and data engineering; new to Rust and to game development; 12-month horizon. Hard constraints: online multiplayer from day one with a Rust authoritative server; buildable by one person plus agents; no art team; commercial viability drives design; learn Rust deeply.

**Taste DNA.** Fallout, Rise of Kingdoms, The Division and SimCity are four wrappers around one loop: build a home base, go out into a systemic world to gather, come back and compound. Pillars: the base as anchor of identity (FO4 settlements, FO76 CAMPs, RoK city); legible systems (GlassBox agents, junk-to-components, explicit gear math); months-long compounding progression; the out-and-back expedition; solo-viable play with a social multiplier; bounded opt-in stakes (Dark Zone, KvK); post-collapse worlds told through environment. Session shape: 2-10 minute async check-ins, 30-90 minute runs, occasional 2-4 hour deep sessions. Dislikes: timer-gating and pay-to-skip, aim-skill PvP as the core, full-loot open PvP, heavy authored narrative, always-online shipped before the server is ready, wipes, content-treadmill live service. Quantic Foundry placement: Mastery-Achievement and Immersion-Creativity bridged by Discovery; Action-Social is weakest.

**Budget correction.** The profile said 20 hours per week (~1,000 hours per year); the founder has clarified ~3,000 hours per year, about 58 per week. Consequences: (a) at 1,000 hours the honest answer was "pick one half of the taste"; at 3,000 both halves (persistent shared world plus embodied co-op run) fit one staged game inside 15-18 months, so the merged pitches rose to the top; (b) agents multiply throughput on server, persistence, protocol, tests and tooling, and barely help with game feel, combat tuning, shaders, asset integration and Bevy churn, so the schedule is bounded by roughly 1,200-1,500 founder-only hours; (c) the only randomized trial of experienced developers using AI tools found them [19% slower while believing they were 20% faster](https://metr.org/blog/2025-07-10-early-2025-ai-experienced-os-dev-study/), and the [2026 follow-up's +18% carries a -9% to +38% interval](https://birchtree.me/blog/an-update-from-the-study-that-said-devs-were-actually-slower-with-coding-agents/), so the pitches' 2-3x multiplier is an assumption; (d) 58 hours sits above the [~50-hour point where output per hour falls](https://docs.iza.org/dp8129.pdf) (not fetched, medium confidence) and inside the [55-hour band WHO/ILO associate with +35% stroke risk](https://www.who.int/news/item/17-05-2021-long-working-hours-increasing-deaths-from-heart-disease-and-stroke-who-ilo); (e) Steam overhead still costs 4-6 weeks.

**Ranking before and after** (average of three judges; merged pitches scored only in the second pass):

| Concept | 1,000 h | 3,000 h | Move |
|---|---|---|---|
| Longthaw* | n/a | 8.83 | new #1 |
| Weirhold* | n/a | 8.00 | new #2 |
| Ashward* | n/a | 7.67 | new #3 |
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

Pitches written as minimum-viable cuts for 1,000 hours (Haulback, Fallow, Downriver) fell because they left two-thirds of the new capacity unspent; developer-hosted combat pitches (Cordon, Graymarket) did not benefit because their deciding hours are gunfeel; the merges won as the co-op scavenger with the first-pass misses fixed.

## 3. Market landscape

Caveat for every lane: Steam, SteamDB, Gamalytic, VG Insights, Sensor Tower, GameDiscoverCo and most press sites were egress-blocked, so many figures come from search snippets, GitHub-hosted estimator CSVs and a nightly Steam catalog dump ([leinstay/steamdb](https://github.com/leinstay/steamdb), ~190k rows, 2026-10-04/06). Developer-announced milestones are high confidence; estimator revenues are medium at best and diverge up to 2x between tools (Enshrouded [$119.5M on Sensor Tower](https://app.sensortower.com/vgi/game/enshrouded) vs ~$53M elsewhere).

### 3.1 Steam city and colony builders

Winner-take-most and worsening; the top is reachable by tiny teams but almost always single-player after long EA, and buyers punish simulation bugs harder than they reward scope.

| Metric | Value | Source |
|---|---|---|
| City Builder 1,000-review rate | 10/370 (2.7%) 2024; 2/397 (0.5%) 2025; Colony Sim 1.5% then 1.0% | [howtomarketagame](https://howtomarketagame.com/2026/01/27/what-the-hell-happened-in-2025/) |
| Median lifetime net | Colony Sim $44k, City Builder $22k (GDCo); Colony Sim $8k (Gamalytic) | [GDCo](https://newsletter.gamediscover.co/p/which-genre-should-your-next-pc-game), [Gamalytic](https://gamedevreports.substack.com/p/gamalytic-67-of-games-on-steam-earned) |
| Tiny-team hits | Manor Lords 3M (solo + contractors); Against the Storm 2M (6 people) | [gamespress](https://www.gamespress.com/Manor-Lords-celebrates-three-million-sales), [gamespress](https://www.gamespress.com/ko-KR/against-the-storm-2-million-copies-sold-milestone) |
| EA graduates 2025 | 225; median 437 days; 20% earned more at 1.0; 1.0 month = 40% of EA month | [gameworldobserver](https://gameworldobserver.com/2025/12/10/according-to-gamediscoverco-most-games-on-steam-earn-less-after-leaving-early-access-than-they-do-at-launch) |
| Multiplayer builders | Kingdoms Reborn 300k, desync threads 2023-26; Settlers: New Allies 48%; Anno 117 65% of 5,501 | [unrealengine](https://www.unrealengine.com/en-US/developer-interviews/inside-kingdoms-reborn-s-game-dev-s-journey-of-discovery-and-city-building), [steambase](https://steambase.io/games/the-settlers-new-allies/steam-charts), [ubisoft](https://news.ubisoft.com/en-us/article/7FK06qQE7cN6hvjbVJrI0i/anno-117-pax-romana-celebrates-117-million-players) |
| Co-op overall | 6% of 2023 releases, 36% of units; median ~40k vs ~5k units; $4.1B H1 2025 | [gamedeveloper](https://www.gamedeveloper.com/business/study-finds-co-op-games-keep-growing-in-numbers-and-sales-on-steam), [wnhub](https://wnhub.io/news/stores-and-publishing/item-48428) |

Verdict: multiplayer is neutral for a classic city builder and a liability with weak netcode; for a base-building/survival hybrid, 2-4 player co-op is a differentiator. The only current post-apocalyptic settlement reference, [After Inc: Revival (78%, $17.99)](https://store.steampowered.com/app/3337140/After_Inc_Revival/), is single-player.

### 3.2 Online co-op survival-craft (OWSC)

The highest hit-rate genre on Steam, dominated by player-hosted servers, with a settlement layer as the 2024-26 differentiator.

| Metric | Value | Source |
|---|---|---|
| OWSC 1,000-review rate | 15/72 (20.8%) 2025; 23/94 (24.5%) 2024; median gross ~$102k | [2025](https://howtomarketagame.com/2026/01/27/what-the-hell-happened-in-2025/), [2024](https://howtomarketagame.com/2025/01/15/what-the-hell-happened-in-2024/), [pokeindie](https://pokeindie.com/blog/game-genre-study-2026-which-genres-are-worth-building) |
| Tiny-team breakouts | Valheim est. 17.0M/$219.9M (5 at launch); Abiotic Factor 1.4M (~12); Necesse 2M (solo start); Sunkenland ~545k (7) | [steamdata](https://steamdata.ai/game/892970/valheim), [games.gg](https://games.gg/news/abiotic-factor-million-sales/), [massivelyop](https://massivelyop.com/2025/11/09/sandbox-mmo-lite-necesse-has-sold-an-eye-popping-2-million-copies-so-far/), [levvvel](https://levvvel.com/sunkenland-statistics/) |
| Official-server cost | Palworld ~$478k in one month | [gamesradar](https://www.gamesradar.com/palworld-dev-ceo-jokes-server-fees-could-bankrupt-the-studio-after-it-spends-dollar478000-to-never-let-the-service-go-down-no-matter-what/) |
| Always-online backlash | Nightingale 47,569 peak CCU to 2,493; 65 layoffs | [steamcharts](https://steamcharts.com/app/1928980) |
| Launch playbook | Abiotic Factor demo drew ~250k wishlists in a week; only 28 games (0.156%) recovered from a weak 2024 launch | [GDCo](https://newsletter.gamediscover.co/p/how-abiotic-factor-sold-600k-copies), [howtomarketagame](https://howtomarketagame.com/2026/02/12/only-28-games-recovered-from-a-bad-launch-in-2024-what-do-they-have-in-common/) |

Host-only co-op is a recurring complaint ([Bellwright shipped no dedicated servers for 2+ years](https://pindrop.gg/bellwright/guides/multiplayer) while reaching [1M copies](https://www.gamespress.com/Medieval-Survival-RPG-Bellwright-Surpasses-1-Million-Units-Sold-on-Ste)). Post-apocalyptic is a proven sub-lane ([VEIN est. 614k/$9.1M in under a year](https://app.sensortower.com/vgi/game/vein)) but not the hook. Every solo breakout took far longer than 12 months (Necesse 13 years to 1.0, Dinkum 8).

### 3.3 Looter and extraction co-op

PvP-first extraction from small teams is a graveyard; PvE-first top-down extraction-lite is where solo-scale hits live; the Division fantasy is unsupplied on PC.

| Metric | Value | Source |
|---|---|---|
| ZERO Sievert | solo; $1.3M first week; 600k+ copies; no co-op | [wnhub](https://wnhub.io/news/other/item-1533), [presskit](https://underdog.ltd/presskit.html) |
| Escape from Duckov | 5 devs; 2M in two weeks, 3M by 2025-11-08; 301,322 peak CCU; single-player | [gamedeveloper](https://www.gamedeveloper.com/business/escape-from-duckov-has-sold-2-million-copies-in-two-weeks), [pcgamer](https://www.pcgamer.com/games/action/escape-from-duckov-quacks-past-300-000-concurrent-players-only-10-days-after-release/) |
| PvP extraction deaths | The Cycle (cheaters); Marauders 500k sold vs 80k expected then CCU collapse; Marathon ~1.2M/$55M, under 1,000 CCU in 6 months | [gamedeveloper](https://www.gamedeveloper.com/business/yager-sunsetting-the-cycle-frontier-after-cheaters-cause-irreparable-damage), [heroiclabs](https://heroiclabs.com/blog/marauders-case-study/), [gameshub](https://www.gameshub.com/news/article/marathon-sales-estimate-1-2-million-copies-steam-ps5-2026-2862962/) |
| Division on PC | Heartland cancelled; Resurgence port 46% positive, 4,117 peak CCU | [shanethegamer](https://www.shanethegamer.com/pc-mac/the-division-resurgence-lands-on-steam-splits-players-with-46-positive-score/) |
| Top-down co-op comp | The Ascent: 12 devs, 1M copies, $5M week one | [gamedeveloper](https://www.gamedeveloper.com/business/curve-games-boasts-strong-sales-for-i-the-ascent-i-and-i-human-fall-flat-i-) |
| Indie MP shooter survival | 1 of 7 that reached 1,000 reviews still alive (Q2 2025) | [howtomarketagame](https://howtomarketagame.com/2025/08/04/2025-q2-games-that-are-selling/) |

[The Forever Winter removed its offline water timer twice](https://www.pcgamer.com/games/third-person-shooter/the-forever-winter-my-favourite-extraction-shooter-just-overhauled-its-most-contentious-feature-for-the-second-time-it-was-a-hell-of-a-rollercoaster-to-make-the-adjustment/); [STALCRAFT: X (95,600+ reviews, 10-14k CCU)](https://www.gamespress.com/Free-to-Play-MMOFPS-STALCRAFT-X-Celebrates-10th-Anniversary-with-Huge-) shows small persistent shards can hold population.

### 3.4 Persistent-world strategy

Enormous mobile appetite monetized by pay-to-win, a browser lineage still alive, and almost no small-team paid entry on PC. Revenue figures here were unverifiable in-session.

| Metric | Value | Source |
|---|---|---|
| Rust server tech | SpacetimeDB 1.0 on 2025-03-03, 25.3k stars; BitCraft server open-sourced (597 stars); OpenMMO solo Rust MMO, 1.8k stars in ~14 months | [releases](https://github.com/clockworklabs/SpacetimeDB/releases/tag/v1.0.0), [BitCraftPublic](https://github.com/clockworklabs/BitCraftPublic), [OpenMMO](https://github.com/Julian-adv/OpenMMO) |
| RoK automation | 196 repos, top ones grind bots (148 stars) and OCR trackers | [RoK bot](https://github.com/Dylan-Zheng/Rise-of-Kingdoms-Bot) |
| Non-P2W models | Screeps buy-once + CPU Unlock, server open-sourced; Foxhole 3 shards with a 3-60 s War API | [screeps docs](https://github.com/screeps/docs/blob/master/source/control.md), [warapi](https://github.com/clapfoot/warapi) |
| Mobile 4X scale | RoK >$1B lifetime; Whiteout Survival >$1B in 2024 (UNVERIFIED, low) | [sensortower](https://sensortower.com/blog) |

A GitHub search for a persistent strategy server in Rust returned zero results; the slow-tick lineage ([Longturn at 23-hour turns](https://github.com/longturn/freeciv21/blob/master/docs/Contributing/game-admin.rst), Prosperous Universe, Neptune's Pride) is alive but tiny.

### 3.5 The multiplayer-builder gap

Every major single-player builder has a community multiplayer retrofit fighting desync; a server-authoritative design removes that bug class. Sales figures are Gamalytic estimates from a [Nov 2024-Feb 2025 snapshot](https://github.com/VIV-T/BUT3_Developpement_web/tree/64d0a457f8f9c5a159c424cd2d9974d0764d4e9b/1_Preparation_Donnees/donneesBrutes/scrapping_gamesData_Gamalytics) and a [Dec 2025 snapshot](https://github.com/kgs112233/dataproject5team/blob/189aaa24dfbfe79b634e29af488beb78f7232d99/output/output_after.csv), medium confidence.

| Title | Est. copies / revenue / avg playtime | Model |
|---|---|---|
| Eco | 682,636 / $17.8M / 74.9h (highest non-factory builder) | persistent server, 7+ years EA |
| Kingdoms Reborn | 572,472 / $9.3M / 22.6h; 91% | solo origin, up to 8 on a shared map |
| Colony Survival | 321,849 / $5.6M / 37.1h | 2 people, player-hosted |
| Sapiens | 64,097 / $1.3M; 85% | solo, multiplayer from EA |
| Cities: Skylines / RimWorld | 10.7M / $107M; 4.0M / $112.7M / 200h | single-player |
| Factorio / Satisfactory | 4.1M / $111.5M / 139h; 5.5M / $119.6M (Dec 2025: $170.8M) | co-op factory |

Demand proof: [RimWorld Multiplayer, 112 open issues after 7 years](https://github.com/rwmt/Multiplayer/releases); [RimWorld Together, 2,086 commits since Feb 2024](https://github.com/RimWorld-Together/Rimworld-Together); [BeaverBuddies, nine desync issues in a year](https://github.com/thomaswp/BeaverBuddies/wiki); [TimberTogether (2026-09-21): "a colony each on a shared map"](https://github.com/timbermods/TimberTogether); [14 of 20 CSM issues searched are desyncs](https://github.com/CitiesSkylinesMultiplayer/CSM/issues?q=is%3Aissue+desync). Multiplayer-native city builders cap near $5-18M lifetime; [SimCity 2013](https://en.wikipedia.org/wiki/SimCity_(2013_video_game)) (not fetched, low confidence) is the always-online warning.

### 3.6 Multiplayer postmortems

Every tiny-team multiplayer hit runs the simulation on a player's machine; every shutdown in the record was centrally hosted and population-dependent.

| Finding | Value | Source |
|---|---|---|
| Valheim model | 3 founders, hoped for 10-20k copies, "you play it on your own server"; Core Keeper defaults to Steam Datagram Relay | [GDC](https://github.com/dklassic/GDC-transcript/blob/master/static/src/transcript/YoOCUpdYYm4.txt), [core-keeper](https://github.com/escapingnetwork/core-keeper-dedicated) |
| Mesh P2P trap | For Honor: ~8 months of a full AAA team to move to dedicated servers while live | [GDC](https://github.com/dklassic/GDC-transcript/blob/master/static/src/transcript/EOJ1Y3xRpwc.txt) |
| Persistent-world cost | AWS reference $1.80/CCU-month at 1,500 CCU, 64% egress; Division 2: ~1,000 players per 40-core box, daily 1,000-bot tests | [AWS](https://github.com/aws-solutions-library-samples/guidance-for-persistent-world-game-hosting-on-aws), [GDC](https://github.com/dklassic/GDC-transcript/blob/master/static/src/transcript/bcXxyKqgV0c.txt) |
| Anti-cheat burden | DayZ bans 1.39% of licenses; ~400 cheat products; cheat sites grossing up to 1.25M/yr | [GDC](https://github.com/dklassic/GDC-transcript/blob/master/static/src/transcript/0M0xBMEuWdU.txt) |
| Steam-only services | SDR, Steam auth and signaling are not in open-source GameNetworkingSockets | [GNS](https://github.com/ValveSoftware/GameNetworkingSockets) |

Decision rule: ship the Rust server as a library embedded in the host's client and as a dedicated binary over Steam Datagram Relay with friends-list lobbies; host no simulation; a persistent official layer is viable only as a low-tick meta-server; never build anything whose fun requires strangers online simultaneously ([The Maestros: monolith, not microservices](https://github.com/dklassic/GDC-transcript/blob/master/static/src/transcript/zto7ZP2-ooM.txt)).

### 3.7 Rust engine and netcode stack

Bevy is production-usable but churns; the netcode stack is consolidated but unproven in shipped persistent games.

| Component | State (2026-10-05) | Source |
|---|---|---|
| Bevy | 0.19.1 stable (2026-08-13); 0.20.0-rc.2 (2026-09-28); breaking releases "approximately once every 3 months" | [crates.io](https://crates.io/api/v1/crates/bevy) |
| Editor and funding | prototypes archived 2026-04-16; Foundation "drastically underfunded"; editor MVP undated | [archive](https://github.com/bevyengine/bevy_editor_prototypes), [6th birthday](https://raw.githubusercontent.com/bevyengine/bevy-website/main/content/news/2026-08-10-bevys-sixth-birthday/index.md) |
| lightyear / bevy_replicon / aeronet | 0.30.1 with prediction, 1.2k stars, no shipped commercial title; 0.44.2 replication, no prediction; 0.21 transports incl. Steam | [lightyear](https://github.com/cBournhonesque/lightyear), [replicon](https://github.com/projectharmonia/bevy_replicon), [aeronet](https://github.com/aecsocket/aeronet) |
| Lag after Bevy 0.19 | avian 1 day, aeronet 5, lightyear 7, bevy_rapier 23; naia/matchbox none after 100+ days | [versions](https://crates.io/api/v1/crates/lightyear/versions) |
| Physics | avian3d 111k downloads/90d vs bevy_rapier3d 62k; bevy_rapier folded into rapier 2026-09-24 | [avian](https://github.com/Jondolf/avian) |
| Web | Safari 26.4 added WebTransport; wtransport "not considered completely production-ready" | [Apple](https://developer.apple.com/documentation/safari-release-notes/safari-26_4-release-notes) |
| Alternatives | Godot 4.7.2 + gdext 0.5.5 usable, web export experimental; Fyrox 1.0 with 5.6k downloads/90d; SpacetimeDB BSL 1.1, one production instance until 2031 | [gdext web](https://raw.githubusercontent.com/godot-rust/book/master/src/toolchain/export-web.md), [fyrox](https://crates.io/api/v1/crates/fyrox), [LICENSE](https://raw.githubusercontent.com/clockworklabs/SpacetimeDB/master/LICENSE.txt) |
| Shipped Bevy titles | Tiny Glade (Bevy ECS + custom renderer, 1M+ wishlists pre-launch); no persistent-world game found | [4th birthday](https://raw.githubusercontent.com/bevyengine/bevy-website/main/content/news/2024-08-10-bevys-fourth-birthday/index.md) |

### 3.8 Platforms and monetization

Steam is the only platform where a solo Rust online game can be paid in 12 months. Figures from the [catalog dump](https://github.com/leinstay/steamdb); the revenue proxy (reviews x 35 x list price) is a Boxleiter-style estimator, medium confidence.

| Metric | Value |
|---|---|
| Releases per year | 12,801 (2023); 17,157 (2024); 19,942 (2025); 19,077 (2026 to Oct 4) |
| All 2024 releases | median 13 reviews; 3.8% at 1,000; paid median gross proxy $1,679; 9.7% ≥ $100k; 2.5% ≥ $1M |
| Paid online-MP 2024 (n=1,303) | median 18 reviews; 21.9% ≥ $100k; 10.4% ≥ $1M |
| Co-op vs PvP-only, paid 2024 | co-op median 27 reviews (p90 2,539); PvP-only median 9 (p90 449) |
| Hit rate by price, paid online-MP 2024-25 | <$5: 1.7%; $15-19.99: 18.6%; $20-29.99: 41.2% |
| 298 paid online-MP successes | median price $24.99; 81.9% Online Co-op; 29.9% EA; 5.7% native Linux |
| Subscription persistent indies | Screeps 1,982 reviews; Project Gorgon 2,456; Embers Adrift 108; one-time premium Foxhole 44,068 |

[OpenFront reached 200-250k daily browser players and 190,000 Steam wishlists with no marketing budget](https://github.com/openfrontio/OpenFrontIO/blob/main/resources/public/press/index.html), with [Steam microtransactions on Steam and Stripe only on web](https://github.com/openfrontio/OpenFrontIO/blob/main/src/client/Payments.ts). Valve as merchant of record removes VAT, refunds and chargebacks ([Screeps uses Xsolla for the same reason](https://github.com/screeps/docs/blob/master/source/tos.md)). Mobile needs paid UA at [~$2-5 CPI](https://www.businessofapps.com/marketplace/user-acquisition/research/cost-per-install/) (low confidence); [Bevy has no console targets](https://github.com/bevyengine/bevy). Derived hosting (prices not fetched, low confidence): ~$0.05-0.50 per CCU-month on a [Hetzner-class included-traffic box](https://www.hetzner.com/cloud/) vs $1-6 on a hyperscaler, where egress at 20 KB/s per player alone is ~$5.

### 3.9 2025-26 trends

Cheap 2-4 player co-op is the most repeatable breakout; building genres have the best hit rates on tiny supply; idle, cozy, survivors-likes, deckbuilders and tower defense are saturated. Computed from the catalog snapshot ([steamdb](https://steamdb.info/) as nominal source; tags sparse for 2025-26).

| Signal | Value | Source |
|---|---|---|
| Friendslop hits | PEAK 374,876 reviews at $7.99; R.E.P.O. 185,642 at $9.99; Schedule I (solo) 204,041 at $19.99 with 4-player co-op | [PEAK](https://store.steampowered.com/app/3527290), [R.E.P.O.](https://vginsights.com/game/3241660), [Schedule I](https://store.steampowered.com/app/3164500) |
| Co-op multiplier, 2025 releases | ≥500 reviews: 12.5% co-op vs 3.5% single-player; ≥5,000: 3.32% vs 0.36% | [steamdb](https://steamdb.info/) |
| OWSC tag hit rate | 31.9% / 23.4% / 34.2% of 47 / 64 / 38 releases (2022-24) | [Valheim](https://store.steampowered.com/app/892970) |
| Saturated lanes | idle 127 to 1,357 releases (8.7% to 4.3%); cozy 112 to 870 (16.1% to 3.4%); TD 2.0% in 2025 | [TD](https://vginsights.com/game/960090) |
| Supply shock | 22,441 releases in 2026 through Oct 4; single-player reaches 500 reviews 2-3% of the time | [steamdb](https://steamdb.info/) |

[Valheim's lead engineer](https://store.steampowered.com/app/892970) credits a $20 price and small download, and describes client-owned-area networking, not a server-authoritative model: the Rust server must buy something players feel or it is pure cost.

## 4. The gaps we found (ranked)

1. **Online co-op top-down PvE extraction-lite with a settlement layer.** [ZERO Sievert](https://underdog.ltd/presskit.html) and [Duckov](https://en.wikipedia.org/wiki/Escape_from_Duckov) lack multiplayer while forums ask for it; online co-op lifts the 5,000-review rate [~9x](https://steamdb.info/); OWSC is the [best hit-rate genre](https://howtomarketagame.com/2026/01/27/what-the-hell-happened-in-2025/). The winner fills this. Caveat: paid $15-19.99 self-published EA co-op launches of 2024 had a [median of 139 reviews with 12% at 1,000](https://github.com/leinstay/steamdb), so the median outcome is modest.
2. **"The world persists while the host is offline."** [Bellwright](https://pindrop.gg/bellwright/guides/multiplayer), Forever Skies and Len's Island are host-only; a free dedicated binary delivers this at zero developer cost, and deterministic catch-up on load covers the listen-server majority.
3. **Settlement/NPC colony layer on survival.** [Bellwright 1M](https://www.gamespress.com/Medieval-Survival-RPG-Bellwright-Surpasses-1-Million-Units-Sold-on-Ste), [Necesse 2M](https://massivelyop.com/2025/11/09/sandbox-mmo-lite-necesse-has-sold-an-eye-popping-2-million-copies-so-far/), Soulmask 1M, Dinkum 1.4M; it is the founder's strongest pillar.
4. **The Division fantasy on PC.** [Heartland cancelled, Resurgence at 46%](https://www.shanethegamer.com/pc-mac/the-division-resurgence-lands-on-steam-splits-players-with-46-positive-score/); served inside the winner by the Meltline zone, not as the headline.
5. **Persistent server with many cities on one world, polished.** Only [Eco (~683k, 74.9h, 7+ years EA)](https://github.com/VIV-T/BUT3_Developpement_web/tree/64d0a457f8f9c5a159c424cd2d9974d0764d4e9b/1_Preparation_Donnees/donneesBrutes/scrapping_gamesData_Gamalytics) supplies it; communities build it themselves ([RimWorld Together](https://github.com/RimWorld-Together/Rimworld-Together), [TimberTogether](https://github.com/timbermods/TimberTogether)). Demoted by the skeptics: the one solo paid comp of the async shape, [Ymir, has 1,079 reviews at 76% after 7.5 years in EA](https://store.steampowered.com/app/378360/Ymir/). Served as a post-launch meta-layer.
6. **Rise of Kingdoms without pay-to-win on PC.** [196 RoK repos, mostly grind bots](https://github.com/Dylan-Zheng/Rise-of-Kingdoms-Bot), are a design brief; but the audience lives on mobile and no paid PC comp exists.
7. **"No desync" as a selling point.** Every shipped multiplayer builder was punished for netcode ([Kingdoms Reborn](https://steamcommunity.com/app/1307890/discussions/0/3823048293511738403/)); it is a review-risk mitigator, not a purchase driver.

## 5. All concepts considered

Second-pass scores (commercial / feasibility / founder-fit). Asterisk marks merged pitches.

| Concept | Lens | One-line hook | C | F | FF | Avg |
|---|---|---|---|---|---|---|
| Longthaw* | merge | Scavenge the ruins each week's thaw uncovers with 3 friends; settlement works while you sleep; pin it to a shared County | 9 | 9 | 8.5 | 8.83 |
| Weirhold* | merge | Settlement on a dam whose gates decide which drowned ruins are reachable; County Board post-launch | 8.5 | 8 | 7.5 | 8.00 |
| Ashward* | merge | Persistent burned-over valley regreening as towns grow, phone check-ins; embodied salvage runs in phase two | 5.5 | 8.5 | 9 | 7.67 |
| DEADWEIGHT | trend-first | Duckov meets Valheim: tide clock, deadweight meter, proximity voice, settlement on timers | 7.5 | 6.5 | 5 | 6.33 |
| Wintergrid | taste-first | Watchable settlers plus 45-minute frozen-city expeditions, player-hosted | 6.5 | 6 | 6 | 6.17 |
| Haulback | scope-first | Fallout 4 settlements meets Duckov, friends-only, EA at month 9 | 7 | 7 | 4.5 | 6.17 |
| Last County | backend-first | A 2,000-settlement ruined county on a 15-second tick; settlers work while offline | 4.5 | 5 | 7 | 5.50 |
| Fallow | scope-first | Slow-tick valley, 10-minute check-ins, RoK map with SimCity sim, no P2W | 3 | 7.5 | 5.5 | 5.33 |
| Watershed | taste-first | Persistent river valley from your phone plus 45-minute PC salvage runs; free browser tier | 5 | 2.5 | 8 | 5.17 |
| Downriver | gap-first | Shared river-valley shard of up to 200 towns, no desync, no wipes | 4 | 4.5 | 6.5 | 5.00 |
| RUSTBELT | trend-first | RoK without pay-to-win on PC, Factorio-lite automation, shared regional economy | 4 | 5 | 6 | 5.00 |
| Graymarket | backend-first | 30-minute raids into a quarantined city that never stops simulating; safehouse shop sells while you sleep | 6 | 3.5 | 4 | 4.50 |
| Cordon | gap-first | Top-down co-op Division: 30-minute ops, Holds other players visit, cheat-proof shards | 5.5 | 3 | 3 | 3.83 |

## 6. Finalists in depth

All twelve adversarial verdicts came back "refuted". Where a mitigation is sound we adopt it; where it changes rank we say so.

### 6.1 Longthaw (avg 8.83)

**Hook and loop.** Hallam County ten years after a volcanic winter; each real week the thaw exposes new ruins and floods others on a reseeded twelve-week season. Out: 30-60 minute runs into kit-built ruins, typed junk under a deadweight meter, slow telegraphed top-down combat, a meltwater clock that runs only in-session. Home: settlers as visible agents with a printed ledger; progress persists 24/7 on a dedicated server or catches up deterministically on a listen server. The Meltline (90-second pump extraction, drop-on-death) is the opt-in Dark Zone; the County pins hearths to a shared map for caravans, compacts of 20, a seasonal Great Work, visitable snapshots and phone check-ins.

**Multiplayer model.** One sim crate, three deployments: embedded listen server (1-4), free dedicated binary (up to 8), a County meta-service syncing kilobytes per hearth per hour; Steam Datagram Relay and lobbies, friends-list only, no matchmaking.

**Comparables and market evidence.** [Duckov 3M in 23 days](https://en.wikipedia.org/wiki/Escape_from_Duckov); [ZERO Sievert $1.3M first week](https://wnhub.io/news/other/item-1533); [Abiotic Factor 1.4M with ~12 people](https://games.gg/news/abiotic-factor-million-sales/); [Necesse 2M](https://massivelyop.com/2025/11/09/sandbox-mmo-lite-necesse-has-sold-an-eye-popping-2-million-copies-so-far/); [Valheim est. 17M](https://steamdata.ai/game/892970/valheim); [Schedule I 204,041 reviews, solo](https://store.steampowered.com/app/3164500); [Foxhole](https://github.com/clapfoot/warapi) for the meta-layer. OWSC [20.8%](https://howtomarketagame.com/2026/01/27/what-the-hell-happened-in-2025/); co-op [~9x at 5,000 reviews](https://steamdb.info/); the [$20-29.99 band at 41.2%](https://github.com/leinstay/steamdb); [Abiotic Factor's ~250k-wishlist demo week](https://newsletter.gamediscover.co/p/how-abiotic-factor-sold-600k-copies).

**12-month scope as pitched.** Q1 sim crate, Bevy client, one kit, 2-player replicon over local UDP, Steam page mid-December, Gate 1 solo-fun test; Q2 SDR lobbies, 4-player drop-in, voice, dedicated binary, three kits, five settler jobs, Deck pass, alpha of 50; Q3 Meltline, two kits, prediction, June Next Fest demo, County v1 as a dark feature, Gate 3 at 15k wishlists; Q4 freeze, Deck Verified, EA late September 2027 at $19.99 with six kits and 20-25 hours; ~2,560 productive hours.

**Market: refuted (medium).** At the [median 0.10x wishlist-to-week-1 conversion above $10](https://newsletter.gamediscover.co/p/the-state-of-steam-wishlist-conversions), a [2.7x year-one multiplier](https://gamedevreports.substack.com/p/gamediscoverco-games-long-tail-revenue) and [~69% of US list realized](https://www.steampageanalyzer.com/tools/regional-pricing-calculator), the 15k gate yields ~1,500 week-one units and ~$50k gross (~$30k net); $100k gross needs ~30k wishlists, while a new page gets [100-1,200 in two weeks](https://www.steampageanalyzer.com/blog/how-many-wishlists-before-launch) and the [median June 2026 Next Fest demo gained ~200](https://wnhub.io/news/stores-and-publishing/item-51221). Every solo comp that cleared $100k in year one had years of pre-launch work plus a publisher ([ZERO Sievert with Modern Wolf, 277,966 wishlists at launch](https://gameworldobserver.com/tag/cabo-studio)). *Mitigation adopted:* 30k-wishlist or publisher gate; a free Prologue through two Next Fests ([Tinkerlands' prologue held ~1,000 daily CCU before an est. 266k-unit EA](https://nodal.gg/game/tinkerlands-2617700)); EA in Q1-Q2 2028; publisher talks at Gate 2.

**Ship: refuted (high for the 9-12 month EA; the 3-month prototype survives).** No solo or tiny-team comp reached EA with less scope in under two years ([ZERO Sievert ~2 years solo, single-player](https://gameworldobserver.com/2023/01/23/zero-sievert-1-million-revenue-cabo-studio-extraction-shooter-steam); [Valheim ~4 years, 5 people](https://gamesbeat.com/valheim-finally-hits-1-0-after-five-years-in-early-access-interview/); [Tiny Glade ~2.5 years, 2 people, Bevy, no multiplayer](https://80.lv/articles/relaxing-castle-builder-tiny-glade-has-finally-arrived-on-steam/)); the agent multiplier is contradicted by [METR](https://metr.org/blog/2025-07-10-early-2025-ai-experienced-os-dev-study/); and [renet_steam pins steamworks 0.12.2](https://raw.githubusercontent.com/lucaspoffo/renet/master/renet_steam/Cargo.toml) while [bevy_steamworks 0.17 pins 0.13.1](https://raw.githubusercontent.com/HouraiTeahouse/bevy_steamworks/main/Cargo.toml), two incompatible Client types. *Mitigation adopted:* EA build = hearth + 3 kits + one enemy family + 2-4 player listen server, ~10-12 hours; dedicated binary at EA+60 days, County at EA+6 months; 18 months to EA as base plan; 45-50 hour cap; measured agent multiplier; one transport path with a CI check on duplicate steamworks versions; week-6 Gate 0 on the ant farm.

**MP model: refuted (high on anti-cheat; medium on population).** County inputs are HMAC-signed receipts from a server the player owns and can patch, so a receipt proves nothing; every client-held economy has been flooded ([Diablo II open Battle.net](https://diablo2.diablowiki.net/B.net), [Fallout 76 dupes](https://primagames.com/news/duped-items-deleted-new-fallout-76-patch), [ARK's wipe three weeks after launch](https://massivelyop.com/2015/06/23/excessive-duping-leads-to-ark-survival-evolved-wipe/)); free-text mail makes a solo developer a user-to-user service under the [UK Online Safety Act](https://www.ofcom.org.uk/online-safety/the-online-safety-act-and-gaming-know-the-risks-know-the-rules-know-how-to-comply); [steamworks-rs has no voice module](https://github.com/Noxime/steamworks-rs/issues?q=voice). *Mitigation adopted:* the County mints its own credit on a fixed per-hearth schedule and only debits what it minted; Great Works score participation, never quantity; compacts of 8-10; no free text; opens above ~300 hearths; voice dropped; version handshake for dedicated servers.

**Net assessment.** Longthaw stays the recommendation: its Phase 1 is the one multiplayer model the evidence supports, and every mitigation is a cut or a gate rather than a new system. The calendar does not survive; the concept does.

### 6.2 Weirhold (avg 8.00)

**Hook, loop and model.** Carrow Weir: a concrete dam, lock and powerhouse as the settlement; the reservoir level drains or floods a drowned mill town. The same hearth loop with ice and a boat, the Intake as the PvE Dark Zone, a weekly freeze/thaw; the County Board (escrowed caravans, compacts of 20, a seasonal Great Work, visitable snapshots) in the first or second EA update. Multiplayer identical to Longthaw.

**Comparables and market evidence.** The same comp set ([Duckov](https://en.wikipedia.org/wiki/Escape_from_Duckov), [ZERO Sievert](https://underdog.ltd/presskit.html), [Valheim](https://steamdata.ai/game/892970/valheim), [Abiotic Factor](https://games.gg/news/abiotic-factor-million-sales/)); the [82% co-op share of paid online-MP hits](https://github.com/leinstay/steamdb); [For Honor: over half of live-period players were friend-invited](https://github.com/dklassic/GDC-transcript/blob/master/static/src/transcript/EOJ1Y3xRpwc.txt).

**12-month scope.** Q1 sim crate, Bevy client, one kit, page mid-December; Q2 SDR, 4-player, dedicated binary, three kits, Deck pass, alpha of 50; Q3 Intake, two kits, prediction, Next Fest, Board as dark feature, 15k gate; Q4 EA late September 2027 at $19.99 with six kits.

**Market: refuted (medium).** Developer net per $19.99 unit is [~$8-9](https://www.steampageanalyzer.com/blog/steam-revenue-share-explained), so $100k net needs ~11,500-12,500 units and the 7-15k gate yields $10-60k net at [0.07-0.13x EA conversion and 2.46-3.6x year one](https://newsletter.gamediscover.co/p/what-long-tail-should-you-expect); in the catalog, self-published entrants of this shape (N=20) have a median of 311 reviews and first-time self-published ones (N=5) a median of 13; [Nested Lands (veteran team plus publisher) sits at an est. $105.9k gross after 7 months](https://raijin.gg/app/2553040/Nested_Lands); [Romestead had 250k wishlists and a publisher before an est. $1.1M in 8 weeks](https://raijin.gg/app/1805320/Romestead). *Mitigation:* 25k gate, a free Prologue in the October 2027 Next Fest with the dam-drain as the trailer shot, EA Q1 2028, publisher talks at 5k.

**Ship: refuted (high).** ~2,560 hours is below [ZERO Sievert's ~4,000 for the single-player half alone](https://howtomarketagame.com/2023/01/18/how-a-pixel-art-shooter-achieved-a-million-dollar-launch-the-zero-sievert-story/) and 15-20x below [Abiotic Factor's ~2.5 years with 10-12 people](https://en.wikipedia.org/wiki/Abiotic_Factor); [Sapiens, the nearest settler-sim-plus-multiplayer analogue, took a decade solo](https://majic-jungle.prezly.com/sapiens-press-kit); [Bithell was hospitalized a year into full-time indie work](https://www.mcvuk.com/development-news/how-mike-bithells-never-ending-crunch-on-volume-sent-him-to-hospital/). *Mitigation:* EA at month 15-18 gated on a month-9 demo with 7k+ wishlists; 2-3 kits; interpolation-only combat; freeze Bevy; Board in year 2.

**MP model: refuted (medium-high).** Save-editing tools appear within weeks for every player-hosted game ([palworld-save-tools, 869 stars](https://github.com/cheahjs/palworld-save-tools); [valheim-save-tools](https://github.com/Kakoen/valheim-save-tools)), so Board inputs are attacker-controlled and "never wiped" removes the only remedy; the dual build is the shape of [Palworld's host-save GUID bug (945 stars, 68 open issues)](https://github.com/xNul/palworld-host-save-fix); [LFGSS shut down rather than carry OSA liability](https://www.techdirt.com/company/microcosm/). *Mitigation:* Board-minted scrip with daily caps, cooperative thresholds, preset-phrase mail, Steam ID as sole identity, 2-3 h/week of named ops.

**Net assessment.** Longthaw with a stronger single image and weaker gates; the reservoir lever is worth stealing as a Season-2 Great Work.

### 6.3 Ashward (avg 7.67)

**Hook, loop and model.** A burned-over timber valley regreening tile by tile as 48 player towns plus NPC towns grow; a town is an agent-simulated diorama with a ledger and shortfall forecast; salvage parties resolve on printed odds; one valley day per 10 real minutes; fire season and winter as shared pressure; great works need several towns; towns can never be lost. Phase 2 (H1 2028): the adjacent ruin hex becomes a 20 Hz 1-4 player instance, the Burn its contested zone. Developer-hosted shards from day one (one Rust process per 48-town valley, WebSocket over axum, event-sourced Postgres) plus a free private-valley binary with a configurable tick.

**Comparables and market evidence.** [Eco ~683k / 74.9h](https://github.com/VIV-T/BUT3_Developpement_web/tree/64d0a457f8f9c5a159c424cd2d9974d0764d4e9b/1_Preparation_Donnees/donneesBrutes/scrapping_gamesData_Gamalytics), Kingdoms Reborn ~572k, [Foxhole 44,068 reviews](https://github.com/leinstay/steamdb), [OpenFront's 190k wishlists](https://github.com/openfrontio/OpenFrontIO/blob/main/resources/public/press/index.html), the [4X tag's 29.7-36.8% hit rate on 19-49 releases](https://steamdb.info/), [Massively Multiplayer's $6.8M top-100 median](https://vginsights.com/).

**12-month scope.** Q1 plain-Rust valley sim plus tokio/axum/sqlx server, egui viewer, Bevy 2D client, five friends on a 60x dev valley, page mid-December; Q2 NPC towns, market, caravans, great work, private-valley binary, phone web client, alpha of 50-100; Q3 Steam auth, Deck pass, Valley API, bot load tests, Next Fest demo valley; Q4 EA late September 2027 at $19.99 with two official valleys, Phase 2 spike. One new stack per quarter.

**Market: refuted (medium).** The shipped product is a slow-tick 2D async town game whose one solo precedent, [Ymir (solo, $34.99, EA since 2019-03-16), has 1,079 reviews at 76% and is still in EA after 7.5 years](https://store.steampowered.com/app/378360/Ymir/) at an est. [$594k lifetime](https://github.com/leinstay/steamdb); [Screeps is est. $570k-$1.45M over ten years](https://raijin.gg/app/464350/Screeps_World); [simulogics called the Prosperous Universe subscription a "necessity"](https://massivelyop.com/?p=311532); paid "Asynchronous Multiplayer" releases have a median of 28 reviews. *Mitigation:* make the EA SKU a co-op town builder for 1-4 friends on private valleys with official valleys second; put a 10-15 minute embodied salvage slice in the Next Fest demo; gate at 25k wishlists or 2k sustained browser DAU; budget $40-80k net.

**Ship: refuted (medium-high; the 3-month slice survives).** The niche has [82 persistent-server strategy/builder titles in the catalog, 37 paid, 7 ever at 1,000 reviews](https://github.com/leinstay/steamdb); [parabellum is 22 months into its Rust rewrite and incomplete](https://github.com/andreapavoni/parabellum); [OpenFront took 25 months and 279 commit authors](https://github.com/openfrontio/OpenFrontIO); [OpenMMO's 2,622 commits include 1,619 co-authored by Claude](https://github.com/Julian-adv/OpenMMO), which proves an agent-assisted solo server is possible but not that fun in a 10-minutes-a-day valley is found faster, since that takes real-speed valley years with humans. *Mitigation:* ship the private valley in year one; replace the wishlist gate with a fun gate (three friend groups complete a valley year and start another); official valleys in year two.

**MP model: refuted (medium-high).** Fixed 48 permanent slots, no wipes and multi-town great works cannot all hold once Steam decay arrives: [Eco shows 26 reviews in the last 30 days against 13,212 lifetime, Kingdoms Reborn 33 of 9,113, BitCraft 51 of 3,757](https://github.com/leinstay/steamdb), so 48 humans become 5-10 plus 40 frozen ghost towns; every long-lived slow-tick game broke one of these promises ([Screeps' novice and respawn areas](https://docs.screeps.com/start-areas.html)); the genre is the most-botted on GitHub ([214 "travian bot" repos](https://github.com/Erol444/TravianBotSharp), [140 "ogame bot"](https://github.com/ogame-tbot/TBot)); [Hetzner dedicated datacenters are Germany and Finland only](https://github.com/elsbrock/hetzner-radar). *Mitigation:* 12-week valley seasons with a persistent hearth carried forward; dormancy after 14 days with the town held in trust by NPCs; elastic valleys opening at 12-16 humans; an attention budget identical for every town; Steam-ownership-only official valleys; a moderation and OSA/DSA floor; a US-region box.

**Net assessment.** The best plan if the founder decides not to ship combat, and the deepest Rust curriculum; but its commercial claim fails on the one solo comp that exists and its shard design fails on decay. Demoted to third; its dormancy-and-seasons design and one-stack-per-quarter sequencing are adopted.

### 6.4 DEADWEIGHT (avg 6.33)

**Hook, loop and model.** "Escape from Duckov meets Valheim" in a drowned container port: every run is a bet on how much you can carry before the tide comes in; sealed cargo is winched at a noisy crane; range-limited proximity voice; a settlement on timers. Player-hosted 1-4 co-op at $17.99, EA at month 12. Comparables: [Duckov](https://en.wikipedia.org/wiki/Escape_from_Duckov), [ZERO Sievert](https://underdog.ltd/presskit.html), [Lethal Company 487,160 reviews, solo](https://vginsights.com/game/1966720); [co-op median 27 vs PvP-only 9 reviews](https://github.com/leinstay/steamdb). Scope: month 3 sim crate, lightyear over SDR, one district, deadweight, tide, boat; month 6 four players, three districts, settlers on a dedicated server, crane, voice; month 9 Next Fest demo, refinery, Docker; month 12 EA with five districts.

**Market: refuted (medium).** Paid + Online Co-op + EA + $15-19.99 + self-published 2024 launches (n=33) have a median of 139 reviews and 12% at 1,000; at [HumanitZ's ~$396 gross per review](https://gamalytic.com/game/1766060), $100k gross needs 180-250 reviews in 12 months, a 70th-80th percentile outcome, and HumanitZ itself (5,693 reviews, est. $2.25M) is a two-person team plus publisher after ~3 years; drowned-city comps are weak ([Floodland 1,001 reviews at 56%](https://store.steampowered.com/app/2080690)); [Bevy's sixth-birthday post lists nine shipped titles with 2-60 reviews each](https://bevyengine.org/news/bevys-sixth-birthday/). *Mitigation:* page by month 3, demo in two Next Fests, EA only at 20-25k wishlists, plan on 5-10k copies.

**Ship: refuted (high).** [lightyear shipped ten breaking releases between July 2025 and September 2026](https://github.com/cBournhonesque/lightyear/releases); [Steam headless mode was broken from 2025-08-12 to 2026-08-03](https://github.com/cBournhonesque/lightyear/issues/1156) and [Steam IO beside UDP IO silently disables server inputs](https://github.com/cBournhonesque/lightyear/issues/1693); [no Rust voice bindings exist](https://raw.githubusercontent.com/Noxime/steamworks-rs/master/src/user.rs); [Bevy system order is nondeterministic by default](https://github.com/bevy-cheatbook/bevy-cheatbook/blob/main/src/programming/system-order.md); [Veloren is at 0.18 after eight years](https://github.com/veloren/veloren). *Mitigation:* pin Bevy 0.20, replicon with interpolation only, drop lightyear, voice and walking settlers from EA, EA Q1 2028.

**MP model: refuted (medium).** The 24/7 promise exists only on the dedicated binary, a second product ([Palworld's Docker image still logs join failures after updates](https://github.com/thijsvanloef/palworld-server-docker/issues/947)); [aeronet_steam ships its session config marked "CURRENTLY BROKEN"](https://github.com/aecsocket/aeronet/blob/main/crates/aeronet_steam/src/config.rs); most friend groups run the listen server. *Mitigation:* offline progression as a deterministic event-timer ledger identical on any load; world save in Steam Cloud with host handoff; dedicated binary as a post-EA milestone.

**Net assessment.** The most streamable design and cleanest hosting model, but it spends founder hours on tide shaders, voice FFI and density and drops the shared-world pillars. Its tide clock and "progress between sessions, not 24/7" framing are folded into the recommendation; the pitch stays fourth.

## 7. Recommendation

**Concept: Longthaw, mitigated.** A $19.99 top-down 1-4 player scavenging game with a settlement that progresses between sessions, in a county that thaws week by week on a reseeded 12-week season. Phase 1 (the hearth) is the product; the County is a free Season-1 update redesigned as a participation layer. Changes versus the pitch, all from the skeptic mitigations: EA scope is hearth + three ruin kits + one or two enemy families + 1-4 player listen server, 10-12 hours; the dedicated binary ships at EA+60 days with a version handshake and SteamCMD script; the Meltline is the first EA update; the County arrives at EA+6 months, mints its own credit on a fixed per-hearth schedule, scores Great Works by participation with no leaderboard, caps compacts at 8-10, carries no free text and opens only above ~300 pinned hearths; in-game Steam voice is not promised; offline progression is a deterministic catch-up function identical on listen and dedicated servers, marketed as "your base progresses between sessions"; open hearths for strangers are deferred indefinitely. The name needs a Steam, trademark and domain search before the page goes live.

**Engine.** Bevy client plus Bevy headless server sharing one engine-free simulation crate, with three rules for the churn. (1) Pin one Bevy version for the year: adopt 0.20 once it stabilizes (rc.2 shipped [2026-09-28](https://crates.io/api/v1/crates/bevy)), upgrade once before EA at ~40 hours, skip later releases until after launch. (2) Keep the simulation crate free of Bevy types and the wire protocol engine-agnostic (postcard over SDR/QUIC), so the Rust learning curve lives in the sim, server and persistence layer rather than the client, and a Godot 4.7 client via gdext remains a fallback if two players cannot walk a ruin together by month 4; this gives the thick-client plan the same protection the founder-fit judge credited to thin clients. (3) Netcode: bevy_replicon with server-authoritative replication and client interpolation only, combat tuned for 80-150 ms, no lightyear and no prediction in year one; one Steam transport path chosen in week 1 with a single steamworks version pinned workspace-wide and a CI job failing on duplicates (the [renet_steam 0.12.2 vs bevy_steamworks 0.13.1 conflict](https://raw.githubusercontent.com/lucaspoffo/renet/master/renet_steam/Cargo.toml) is real); Avian over rapier; no wtransport, WebTransport or WASM on the critical path. Determinism must be specified: fixed-point economy math, explicitly ordered systems, no float crossing a platform boundary. Learning rule: the founder hand-writes the first version of every novel subsystem (sim core, replication, persistence, catch-up) and reads every agent diff there; agents own tests, fixtures, bots, CI, tooling and the County service; measure hours per merged feature for eight weeks and re-plan from the observed multiplier, budgeting 1.2x until measured.

**Platforms.** Steam first: Windows, Steam Deck "Playable" at EA via Proton (gamepad UI from Q2, Vulkan from wgpu) with Verified when Valve's queue allows, native Linux as a near-free bonus. No web client, mobile app or console in the first 12 months; a plain-HTML County page for phone check-ins when the County opens.

**Art direction.** Top-down three-quarter diorama with a fixed camera; one or two low-poly modular kits (Synty/Kenney class) recolored to one palette, varied by procedural placement, snowpack depth, meltwater level and a weathering shader; the thaw is the look (white and slate in week one, ochre and green by week twelve); one commissioned settler rig so the ant farm has charm; one commissioned capsule and key art set. Mod surface stops at data tables because pack licenses forbid redistributing meshes. Year-one cash under $8k.

**Monetization.** Premium only, Valve as merchant of record: $19.99 at EA (never lower; [sub-$15 online games reach 1,000 reviews under 8% of the time](https://github.com/leinstay/steamdb)), $24.99 at 1.0, a supporter edition, cosmetics after 1.0; no F2P, subscription, timers or wipes. Honest per-unit model: ~$12.40 gross and ~$7.40 net, so $100k gross is ~8,000 year-one units and needs ~30k launch wishlists at median conversion.

**Multiplayer model.** Listen server embedded in the host's client (solo is the same server on localhost), free headless dedicated binary, Steam Datagram Relay and lobbies, friends-list invites only, no matchmaking, nothing that needs anyone else online. The County is a low-tick meta-service syncing kilobytes per hearth per hour and never running simulation.

**Hosting cost.** Simulation: $0 (player-hosted; SDR is free to Steam partners, medium confidence since partner docs were blocked). County: one Hetzner cloud or AX-class box at roughly EUR 15-50 per month with Postgres on-box, offsite backups ~$5 per month, domain $15 per year, under $700 for the year (provider prices unverified, low confidence). For scale, the [AWS persistent-world reference costs $1.80 per CCU-month at 1,500 CCU with 64% egress](https://github.com/aws-solutions-library-samples/guidance-for-persistent-world-game-hosting-on-aws); the derived Hetzner-class figure is $0.05-0.50. Founder ops: ~3 hours per week after the County opens, one restore drill per quarter.

**Twelve-month milestones (Oct 2026 to Sep 2027, ~2,400-2,560 productive hours).**

- *Q1, Oct-Dec 2026.* Rust ramp (expect 200-300 hours of slower-than-planned work). Plain-Rust sim crate with no engine and no async: settlers, jobs, typed components, buildings, ledger, thaw-stage model, fixed step, proptest conservation, versioned serde saves. egui viewer by week 5; Gate 0 at week 6: is the ant farm fun alone? If not, the settlement is the product and the expedition half is cut. Bevy 0.20 client with one ruin kit, movement, deadweight and a first combat loop; two-player replicon over local UDP by December. Thursday test group from week 2. Art direction locked by week 8; Steam page live by week 10 with capsule art, six screenshots and a 45-second trailer; weekly devlog; name clearance; two days re-verifying the key comps from primary sources. Gate 1 at month 3: a 30-minute run is fun alone for 3 of 4 testers, or pivot to valley-first on the same crate.
- *Q2, Jan-Mar 2027.* One Steam transport path (SDR, lobbies, auth tickets), 4-player drop-in/drop-out, two more kits, five settler jobs, gear and perks with printed math, the weekly thaw stage, save-migration harness in CI, deterministic catch-up, Deck gamepad pass (80 h, tested monthly on hardware), closed alpha of ~50, late-March City Builder and Colony Sim Fest. Gate 2 at month 6: the Thursday group plays unasked and alpha D7 return exceeds 30%; publisher outreach at 5k wishlists.
- *Q3, Apr-Jun 2027.* Free standalone Prologue (own app ID: the hearth, one kit, 2-player co-op, 2-4 hours) in the June Next Fest with a co-op streamer key drop; performance pass; Meltline prototype; County v1 built by agents as a dark feature in its mitigated form. Gate 3 at end of June: 25-30k wishlists or a signed publisher means EA in September; 10-25k means EA in Q1 2028 after the October Next Fest; under 10k means keep iterating the Prologue and treat 2028 as a sub-$50k year.
- *Q4, Jul-Sep 2027.* Content freeze three weeks before any launch, refund-window polish, second trailer. If Gate 3 is green: EA in late September at $19.99 with three kits and 10-12 hours, dedicated binary at EA+60 days. Otherwise the Prologue stays live as the permanent front door, the Meltline and a fourth kit land in the demo, and EA ships in Q1 2028. Minimum shippable build at ~2,000 hours if the pace fails: hearth + two kits + 2-player co-op + Prologue.

**Sustainable pace at 58 hours a week.** Do not plan 58. Plan 45-50 with one full day off and every sixth week a deload; that is the ~2,400-2,560 productive hours the pitch already assumed, so nothing is lost on paper and the plan stops depending on the band where [per-hour output falls](https://docs.iza.org/dp8129.pdf) and [health risk rises](https://www.who.int/news/item/17-05-2021-long-working-hours-increasing-deaths-from-heart-disease-and-stroke-who-ilo). [GDC's 2026 survey found 94% of developers reporting a burnout symptom while only 11% work over 50 hours](https://respawn.outlookindia.com/amp/story/gaming/gaming-originals/2026-gdc-survey-exposes-workforce-strain-and-technological-shifts); [Barone burned out after shipping Stardew alone](https://www.shortform.com/blog/how-was-stardew-valley-made/). Month one, the steepest part of the Rust curve, should be a short week. Gates slip dates and never add hours. If any of the 3,000 hours sits on top of a day job, every date above slips by at least a quarter. Name the cash runway now: there is no revenue before EA, and the June gate is where financial pressure will try to override the data.

## 8. Decisions the founder must make now

1. **Scavenger first (Longthaw) or valley first (Ashward)?** Default: Longthaw. Its Phase 1 is the only multiplayer model the evidence supports and it ships the loop he plays most. Choose Ashward only if he would rather run a living valley than play Thursday-night runs, accepting the embodied layer is 15-18 months out.
2. **Hold September 2027 for paid EA, or gate it?** Default: gate it. Twelve months buys a page, a free Prologue, two Next Fests and a wishlist pile; EA ships at 25-30k wishlists or with a publisher, realistically Q1 2028, because a thin EA cannot be fixed later ([20% of graduates do better at 1.0](https://gameworldobserver.com/2025/12/10/according-to-gamediscoverco-most-games-on-steam-earn-less-after-leaving-early-access-than-they-do-at-launch); [0.156% recovered from a weak launch](https://howtomarketagame.com/2026/02/12/only-28-games-recovered-from-a-bad-launch-in-2024-what-do-they-have-in-common/)).
3. **Which code does the founder write by hand?** Default: the first version of the sim core, replication, persistence and catch-up, reading every agent diff in those crates; agents own tests, fixtures, bots, CI, tooling and the County service. Without this rule the 3,000 hours produce a shipped game and a shallow Rust education.
4. **Bevy client or Godot client?** Default: Bevy pinned to 0.20 for the year, engine-free sim crate, postcard protocol, hard fallback to Godot via gdext if two players cannot walk a ruin together by month 4.
5. **Hours and runway.** Default: cap at 45-50 hours per week, log actual hours, and write down the cash runway and the month it ends before setting any gate. Is any of this on top of a day job? That answer moves every date in Section 7.
6. **Publisher or self-publish?** Default: self-publish the Prologue and the page, but open conversations with Hooded Horse / Modern Wolf / Fireshine-class publishers at 5k wishlists for marketing and porting only, never for design control or a wipe-based live service; every solo comp above $100k in year one had one.

## 9. Sources

Every URL cited inline above, deduplicated, in order of first use; figures marked unverified in the text were taken from these pages via search snippets or memory because the pages were egress-blocked during research.

https://howtomarketagame.com/2026/01/27/what-the-hell-happened-in-2025/
https://voxbooster.com/blog/indie-game-statistics-2026/
https://underdog.ltd/presskit.html
https://en.wikipedia.org/wiki/Escape_from_Duckov
https://newsletter.gamediscover.co/p/the-state-of-steam-wishlist-conversions
https://metr.org/blog/2025-07-10-early-2025-ai-experienced-os-dev-study/
https://birchtree.me/blog/an-update-from-the-study-that-said-devs-were-actually-slower-with-coding-agents/
https://docs.iza.org/dp8129.pdf
https://www.who.int/news/item/17-05-2021-long-working-hours-increasing-deaths-from-heart-disease-and-stroke-who-ilo
https://github.com/leinstay/steamdb
https://app.sensortower.com/vgi/game/enshrouded
https://newsletter.gamediscover.co/p/which-genre-should-your-next-pc-game
https://gamedevreports.substack.com/p/gamalytic-67-of-games-on-steam-earned
https://www.gamespress.com/Manor-Lords-celebrates-three-million-sales
https://www.gamespress.com/ko-KR/against-the-storm-2-million-copies-sold-milestone
https://gameworldobserver.com/2025/12/10/according-to-gamediscoverco-most-games-on-steam-earn-less-after-leaving-early-access-than-they-do-at-launch
https://www.unrealengine.com/en-US/developer-interviews/inside-kingdoms-reborn-s-game-dev-s-journey-of-discovery-and-city-building
https://steambase.io/games/the-settlers-new-allies/steam-charts
https://news.ubisoft.com/en-us/article/7FK06qQE7cN6hvjbVJrI0i/anno-117-pax-romana-celebrates-117-million-players
https://www.gamedeveloper.com/business/study-finds-co-op-games-keep-growing-in-numbers-and-sales-on-steam
https://wnhub.io/news/stores-and-publishing/item-48428
https://store.steampowered.com/app/3337140/After_Inc_Revival/
https://howtomarketagame.com/2025/01/15/what-the-hell-happened-in-2024/
https://pokeindie.com/blog/game-genre-study-2026-which-genres-are-worth-building
https://steamdata.ai/game/892970/valheim
https://games.gg/news/abiotic-factor-million-sales/
https://massivelyop.com/2025/11/09/sandbox-mmo-lite-necesse-has-sold-an-eye-popping-2-million-copies-so-far/
https://levvvel.com/sunkenland-statistics/
https://www.gamesradar.com/palworld-dev-ceo-jokes-server-fees-could-bankrupt-the-studio-after-it-spends-dollar478000-to-never-let-the-service-go-down-no-matter-what/
https://steamcharts.com/app/1928980
https://newsletter.gamediscover.co/p/how-abiotic-factor-sold-600k-copies
https://howtomarketagame.com/2026/02/12/only-28-games-recovered-from-a-bad-launch-in-2024-what-do-they-have-in-common/
https://pindrop.gg/bellwright/guides/multiplayer
https://www.gamespress.com/Medieval-Survival-RPG-Bellwright-Surpasses-1-Million-Units-Sold-on-Ste
https://app.sensortower.com/vgi/game/vein
https://wnhub.io/news/other/item-1533
https://www.gamedeveloper.com/business/escape-from-duckov-has-sold-2-million-copies-in-two-weeks
https://www.pcgamer.com/games/action/escape-from-duckov-quacks-past-300-000-concurrent-players-only-10-days-after-release/
https://www.gamedeveloper.com/business/yager-sunsetting-the-cycle-frontier-after-cheaters-cause-irreparable-damage
https://heroiclabs.com/blog/marauders-case-study/
https://www.gameshub.com/news/article/marathon-sales-estimate-1-2-million-copies-steam-ps5-2026-2862962/
https://www.shanethegamer.com/pc-mac/the-division-resurgence-lands-on-steam-splits-players-with-46-positive-score/
https://www.gamedeveloper.com/business/curve-games-boasts-strong-sales-for-i-the-ascent-i-and-i-human-fall-flat-i-
https://howtomarketagame.com/2025/08/04/2025-q2-games-that-are-selling/
https://www.pcgamer.com/games/third-person-shooter/the-forever-winter-my-favourite-extraction-shooter-just-overhauled-its-most-contentious-feature-for-the-second-time-it-was-a-hell-of-a-rollercoaster-to-make-the-adjustment/
https://www.gamespress.com/Free-to-Play-MMOFPS-STALCRAFT-X-Celebrates-10th-Anniversary-with-Huge-
https://github.com/clockworklabs/SpacetimeDB/releases/tag/v1.0.0
https://github.com/clockworklabs/BitCraftPublic
https://github.com/Julian-adv/OpenMMO
https://github.com/Dylan-Zheng/Rise-of-Kingdoms-Bot
https://github.com/screeps/docs/blob/master/source/control.md
https://github.com/clapfoot/warapi
https://sensortower.com/blog
https://github.com/longturn/freeciv21/blob/master/docs/Contributing/game-admin.rst
https://github.com/VIV-T/BUT3_Developpement_web/tree/64d0a457f8f9c5a159c424cd2d9974d0764d4e9b/1_Preparation_Donnees/donneesBrutes/scrapping_gamesData_Gamalytics
https://github.com/kgs112233/dataproject5team/blob/189aaa24dfbfe79b634e29af488beb78f7232d99/output/output_after.csv
https://github.com/rwmt/Multiplayer/releases
https://github.com/RimWorld-Together/Rimworld-Together
https://github.com/thomaswp/BeaverBuddies/wiki
https://github.com/timbermods/TimberTogether
https://github.com/CitiesSkylinesMultiplayer/CSM/issues?q=is%3Aissue+desync
https://en.wikipedia.org/wiki/SimCity_(2013_video_game
https://github.com/dklassic/GDC-transcript/blob/master/static/src/transcript/YoOCUpdYYm4.txt
https://github.com/escapingnetwork/core-keeper-dedicated
https://github.com/dklassic/GDC-transcript/blob/master/static/src/transcript/EOJ1Y3xRpwc.txt
https://github.com/aws-solutions-library-samples/guidance-for-persistent-world-game-hosting-on-aws
https://github.com/dklassic/GDC-transcript/blob/master/static/src/transcript/bcXxyKqgV0c.txt
https://github.com/dklassic/GDC-transcript/blob/master/static/src/transcript/0M0xBMEuWdU.txt
https://github.com/ValveSoftware/GameNetworkingSockets
https://github.com/dklassic/GDC-transcript/blob/master/static/src/transcript/zto7ZP2-ooM.txt
https://crates.io/api/v1/crates/bevy
https://github.com/bevyengine/bevy_editor_prototypes
https://raw.githubusercontent.com/bevyengine/bevy-website/main/content/news/2026-08-10-bevys-sixth-birthday/index.md
https://github.com/cBournhonesque/lightyear
https://github.com/projectharmonia/bevy_replicon
https://github.com/aecsocket/aeronet
https://crates.io/api/v1/crates/lightyear/versions
https://github.com/Jondolf/avian
https://developer.apple.com/documentation/safari-release-notes/safari-26_4-release-notes
https://raw.githubusercontent.com/godot-rust/book/master/src/toolchain/export-web.md
https://crates.io/api/v1/crates/fyrox
https://raw.githubusercontent.com/clockworklabs/SpacetimeDB/master/LICENSE.txt
https://raw.githubusercontent.com/bevyengine/bevy-website/main/content/news/2024-08-10-bevys-fourth-birthday/index.md
https://github.com/openfrontio/OpenFrontIO/blob/main/resources/public/press/index.html
https://github.com/openfrontio/OpenFrontIO/blob/main/src/client/Payments.ts
https://github.com/screeps/docs/blob/master/source/tos.md
https://www.businessofapps.com/marketplace/user-acquisition/research/cost-per-install/
https://github.com/bevyengine/bevy
https://www.hetzner.com/cloud/
https://steamdb.info/
https://store.steampowered.com/app/3527290
https://vginsights.com/game/3241660
https://store.steampowered.com/app/3164500
https://store.steampowered.com/app/892970
https://vginsights.com/game/960090
https://store.steampowered.com/app/378360/Ymir/
https://steamcommunity.com/app/1307890/discussions/0/3823048293511738403/
https://gamedevreports.substack.com/p/gamediscoverco-games-long-tail-revenue
https://www.steampageanalyzer.com/tools/regional-pricing-calculator
https://www.steampageanalyzer.com/blog/how-many-wishlists-before-launch
https://wnhub.io/news/stores-and-publishing/item-51221
https://gameworldobserver.com/tag/cabo-studio
https://nodal.gg/game/tinkerlands-2617700
https://gameworldobserver.com/2023/01/23/zero-sievert-1-million-revenue-cabo-studio-extraction-shooter-steam
https://gamesbeat.com/valheim-finally-hits-1-0-after-five-years-in-early-access-interview/
https://80.lv/articles/relaxing-castle-builder-tiny-glade-has-finally-arrived-on-steam/
https://raw.githubusercontent.com/lucaspoffo/renet/master/renet_steam/Cargo.toml
https://raw.githubusercontent.com/HouraiTeahouse/bevy_steamworks/main/Cargo.toml
https://diablo2.diablowiki.net/B.net
https://primagames.com/news/duped-items-deleted-new-fallout-76-patch
https://massivelyop.com/2015/06/23/excessive-duping-leads-to-ark-survival-evolved-wipe/
https://www.ofcom.org.uk/online-safety/the-online-safety-act-and-gaming-know-the-risks-know-the-rules-know-how-to-comply
https://github.com/Noxime/steamworks-rs/issues?q=voice
https://www.steampageanalyzer.com/blog/steam-revenue-share-explained
https://newsletter.gamediscover.co/p/what-long-tail-should-you-expect
https://raijin.gg/app/2553040/Nested_Lands
https://raijin.gg/app/1805320/Romestead
https://howtomarketagame.com/2023/01/18/how-a-pixel-art-shooter-achieved-a-million-dollar-launch-the-zero-sievert-story/
https://en.wikipedia.org/wiki/Abiotic_Factor
https://majic-jungle.prezly.com/sapiens-press-kit
https://www.mcvuk.com/development-news/how-mike-bithells-never-ending-crunch-on-volume-sent-him-to-hospital/
https://github.com/cheahjs/palworld-save-tools
https://github.com/Kakoen/valheim-save-tools
https://github.com/xNul/palworld-host-save-fix
https://www.techdirt.com/company/microcosm/
https://vginsights.com/
https://raijin.gg/app/464350/Screeps_World
https://massivelyop.com/?p=311532
https://github.com/andreapavoni/parabellum
https://github.com/openfrontio/OpenFrontIO
https://docs.screeps.com/start-areas.html
https://github.com/Erol444/TravianBotSharp
https://github.com/ogame-tbot/TBot
https://github.com/elsbrock/hetzner-radar
https://vginsights.com/game/1966720
https://gamalytic.com/game/1766060
https://store.steampowered.com/app/2080690
https://bevyengine.org/news/bevys-sixth-birthday/
https://github.com/cBournhonesque/lightyear/releases
https://github.com/cBournhonesque/lightyear/issues/1156
https://github.com/cBournhonesque/lightyear/issues/1693
https://raw.githubusercontent.com/Noxime/steamworks-rs/master/src/user.rs
https://github.com/bevy-cheatbook/bevy-cheatbook/blob/main/src/programming/system-order.md
https://github.com/veloren/veloren
https://github.com/thijsvanloef/palworld-server-docker/issues/947
https://github.com/aecsocket/aeronet/blob/main/crates/aeronet_steam/src/config.rs
https://respawn.outlookindia.com/amp/story/gaming/gaming-originals/2026-gdc-survey-exposes-workforce-strain-and-technological-shifts
https://www.shortform.com/blog/how-was-stardew-valley-made/
