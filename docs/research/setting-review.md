# Setting review: leaving the frozen county

2026-10-09. A proposal for the founder, not a decision. The setting becomes a decision only when the founder records it as D26 in [the decision log](../plan/decisions.md).

## The answer

**Recommended: C-lite. A city that nature took back, behind a cordon that lifts week by week.**

Pitch: *Twelve years after the evacuation, the cordon around the city lifts one sector a week. Build a hearth at the edge of the wild, and go in with a friend for what the city left behind.*

Why. The overgrown city is the open shelf: no online co-op survival game described it in any year from 2022 to 2026 ([Steam dump](https://github.com/leinstay/steamdb/releases/latest/download/steamdb.min.json.gz)). The cordon is the cleanest clock, the same shape as the thaw. Contamination explains the cordon and keeps the Fallout and Division threads.

Both skeptics refuted the leader, C. Runner-up A fails both worse, so it is not the pick. C survives only cut down and sold with B's face.

Biggest risk: green ruins behind a fence still read as Atomfall, STALKER or Pacific Drive. A capsule test (the skeptic's estimate: $0-200) runs before any capsule spend.

## The options compared

Scores are out of 10, from this review's three judges (no external source).

| Option | Commercial | Design | Cost | Avg | Verdict |
|---|---|---|---|---|---|
| C rewilded zone | 5 | 9 | 5 | 6.3 | Best clock, borrowed look; leads only as C-lite |
| A radioactive | 3 | 6 | 8 | 5.7 | Cheapest, but crowded, invisible and legally watched |
| B nature city | 7 | 4 | 3 | 4.7 | Best shelf and tone, weakest clock, costliest art |

A is proven but crowded. Radioactive survival titles from 2022-24 reached 1,000 reviews 15.4% of the time, against 6.8% for all paid survival ([dump](https://github.com/leinstay/steamdb/releases/latest/download/steamdb.min.json.gz)). STALKER 2 sold [1M copies in 2 days](https://www.gematsu.com/2024/11/stalker-2-heart-of-chornobyl-sales-top-one-million). GSC got MISERY [pulled from Steam in November 2025](https://www.gamespark.jp/article/2025/11/14/159483.html). Radiation is invisible, so the week shows only through HUD or green haze.

B has the room. It sits between grim and cozy, below Solarpunk's [1M+ wishlists](https://gamegeeker.com/games/solarpunk-1805110-steam-pc?market=US) and The Last Caretaker's [400k+ copies](https://games.gg/news/last-caretaker-xbox-game-pass-launch/). But growth only closes ground, so reopening it needs a second system. Dense foliage is also the costliest thing to draw on a Steam Deck.

C has the best clock. Its look is already sold by Atomfall, [Pacific Drive (1M+ copies)](https://pushsquare.com/news/2025/02/ironwood-studios-pacific-drive-cruises-past-1-million-copies-sold) and STALKER.

## How it replaces the thaw

Mechanics stay as the docs define them. Names and fiction change.

**World clock: the lifting.** Each real week the survey lifts some sectors, which opens new ruins. It posts others "closed for works", which closes that ground. `cordon_stage(season_seed, week)` replaces `thaw_stage` with the same shape: two disjoint slot sets per stage, and overgrowth and exposure levels as `Fixed` in place of snowpack and meltwater. Week 13 reseeds a new survey over the same city. The close is administrative, shown with fences, diggers and bag piles. Fukushima lifted its orders on a few dates [from 2017 to 2022](https://www.pref.fukushima.lg.jp/site/portal-english/en-1-3-3.html). Decay cannot drive a weekly change, because cesium-137's half-life is [30 years](https://19january2025snapshot.epa.gov/radiation/radionuclide-basics-cesium-137).

**Run clock: crew exposure.** One shared bar per run replaces the meltwater level. It rises only in-session and pauses with the session, as meltwater does. Open streets and drains fill it fastest. Interiors and upper floors fill it slowest. That matches the physics: a basement shields [5-30 times, a ground floor 2-5](https://digital.library.unt.edu/ark:/67531/metadc13021/m2/1/high_res_d/CEX-59.13.pdf). Past the limit, low ground costs health at a printed rate. The streets home close the way meltwater closed them. There is no max-HP penalty, no cure item, and nothing carried between runs.

**Stakes mode: the Inner Cordon (deferred).** An unlifted sector holds a cache. You leave through a 90-second decontamination gate, and carried units drop on death. It is the Meltline one to one. It stays deferred under [D2](../plan/decisions.md), and [open question 28](../plan/open-questions.md)'s default (none in v1) holds.

**Enemy family: the boar sounder.** Piglets harass. Sows bluff-charge. The tusker huffs, clacks its tusks, then charges in a straight line and recovers slowly. Pillars, cars and stairs break charges. The sounder is driven off at a morale break, never exterminated. Boar really did [overrun Fukushima's empty towns](https://www.nbcnews.com/news/world/wild-boars-take-over-towns-near-japan-s-fukushima-nuclear-n731661). Chernobyl's boar [match clean reserves in number](https://www.sci.news/biology/science-wildlife-chernobyl-exclusion-zone-03308.html). No mutants.

**Three settler jobs: Salvager, Grower, Builder.** The Salvager breaks typed junk into components. The Grower turns planted beds into food. The Builder turns components into blueprint buildings. They replace the hauler, sorter and builder placeholders. None carries a hazard rule, so a later setting change costs nothing.

**First screenshot.** The diorama at dusk shows the lit hearth, two players and settlers at planted beds. A survey-orange cordon line of stakes, tape and temporary fence crosses the street. Beyond it the city is wild: moss, vines, rust and grey. The HUD reads "Week 5 of 12" and names the next sector to lift. The capsule uses the same picture with no text. Valve allows only artwork, title and subtitle on capsules [since 2022](https://www.pcgamesn.com/steam/game-art-new-rules-valve), and the small capsule is [231x87 pixels](https://www.steampageanalyzer.com/blog/steam-capsule-sizes).

## What the skeptics found and what changed

**Shelf: refuted (medium).** Atomfall sells ["rolling green hills, lush valleys"](https://www.tomsguide.com/gaming/i-played-atomfall-early-and-it-could-be-one-of-2025s-most-pleasant-surprises) in a quarantine zone. It reached [3.7M players and a TV deal](https://www.pushsquare.com/news/2026/04/atomfall-joins-the-ranks-of-the-last-of-us-and-fallout-with-its-own-tv-series-adaptation). Fallout 76 owns co-op rebuilding in a green post-nuclear forest, with a [31,002 Steam peak](https://steambase.io/games/fallout-76) in January 2026. STALKER 2 even has boar you dodge by [climbing onto cars](https://windowscentral.com/gaming/stalker-2-mutant-types-list-of-all-mutants-and-how-to-kill-them).

What changed:
- The region is a city, not a northern county. Atomfall's zone is rural northern England.
- The capsule bans trefoils, gas masks, hazmat suits, Geiger counters, cooling towers, hazard yellow, green haze, and any animal as the hero.
- The pitch claims the supply gap, not a new look.
- A gate before the capsule artist is paid (the skeptic's proposed numbers): a greybox capsule, 5 seconds in a mock Steam grid, 30+ survival players. It passes if under 25% name a nuclear franchise and over 50% say "settlement" or "co-op".
- If it fails, the cause becomes a chemical works, on the model of Seveso's fenced Zone A, now a [43-hectare oak park](https://www.9colonne.it/620937/seveso-50-years-on-honoring-the-memory-of-a-tragedy-that-changed-the-world). No mechanic changes. If that fails too, ship B's look with the same cordon clock.

**Clock: refuted (medium-high).** The design lane's per-player dosimeter added a dose stat, a max-HP rule, persistent crew dose, a decontamination economy and a ledger column. All of it lands in the crates you read line by line. Personal exposure timers draw complaints: Dying Light 2 raised its immunity timer [by 50%](https://www.dsogaming.com/patches/dying-light-2-title-update-1-23-released-full-patch-notes/). The lane's "basements are hotter" rule was physically backwards.

What changed:
- The run clock is the shared crew bar above.
- Closes are administrative, not runoff physics.
- Settler jobs carry no hazard rules.
- Greening follows sector state, not the week number. Real cleanup strips vegetation; grass removal alone cut dose [70-90%](https://jdarchive.org/api/item/1698363). So lifted sectors look tended (cleared streets, beds, lit windows) and unlifted sectors look wild.
- A gate: 1-2 days of captures at weeks 1, 6 and 12, in both cameras, on a real Deck. [3 of 4](../plan/open-questions.md) Thursday testers must say which sectors opened or closed without the HUD. The sim must hold no more state fields than the thaw.

**Runner-up A fails both worse.** Its capsule lands on the STALKER, Fallout and Atomfall shelf with nothing to set it apart. Its week is invisible without a HUD.

## Art approach and cost

Sparse spring rewilding, with one heavy layer:
- Ruin kit: Synty POLYGON Apocalypse Pack, [$349.99 list, seen at $175, Godot 4.6.2 project](https://syntystore.com/products/polygon-apocalypse-pack). Kenney's CC0 city kits fill gaps.
- Foliage: Quaternius's [CC0 Stylized Nature MegaKit](https://quaternius.itch.io/stylized-nature-megakit) and Kenney's [CC0 Nature Kit](https://opengameart.org/content/nature-kit), both engine-agnostic. Synty's biomes cost [$54.99 each and ship for Unity and Unreal only](https://syntystore.com/products/polygon-meadow-forest-nature-biome).
- Ivy: a [$35](https://www.turbosquid.com/3d-models/3d-ivy-plants-games-1586550) or [$25](https://offaxisstudios.itch.io/polyworks-ruins-pack) pack, cut to 10-20 facade pieces on the kit grid.
- Skip the Woodland Apocalypse Map. It requires the Alpine biome, which [brings snow and ice back](https://syntystore.com/products/polygon-alpine-mountain-nature-biomes).
- One "growth" shader value per sector drives moss and vines. Setting a Godot global uniform [costs nothing](https://raw.githubusercontent.com/godotengine/godot-docs/master/tutorials/shaders/shader_reference/shading_language.rst). Foliage is placed at full density and revealed with [`visible_instance_count`](https://raw.githubusercontent.com/godotengine/godot-docs/master/tutorials/performance/using_multimesh.rst).
- Knee-high ground cover with no shadows, opaque trees, a camera-to-player dither, no volumetric fog.
- Foliage is cosmetic. It never blocks sight lines in the sim.

Why the restraint: in community reports, Valheim on a Deck runs about [60, 50 and 40 fps](https://cal1.lr.ggtyler.dev/r/SteamDeckGameSetting) at low, medium and high vegetation. Enshrouded dips into the [high 20s in forests](https://steamdeckhq.com/game-reviews/enshrouded-early-access/), and its players ask for [foliage that stops hiding the player](https://enshrouded.featureupvote.com/suggestions/720007/make-foliage-between-the-camera-and-the-player-character-transparent).

Cost: packs come to about $300-600 at the prices above, far under the [$8k cap](market-research.md). The boar is the risk. A commissioned quadruped costs [$2-5k plus $500-1k for the rig and $200-1,000 per cycle](https://rocketbrush.com/blog/3d-character-art-prices-guide). Price a pack or CC0 animal first; the three variants share one rig.

Bake-off note: Synty's biomes would favor Unity 6. CC0 foliage keeps the bake-off neutral. Add a 1-2 day Deck foliage test to each engine's slice.

## Name shortlist

This is a pre-screen, not legal clearance. It checked the 2026-10-08 [Steam dump](https://github.com/leinstay/steamdb/releases/latest/download/steamdb.min.json.gz) (190,806 apps), web searches and DNS. Trademark databases were blocked. D20's human search still applies.

| Name | Steam | Notes |
|---|---|---|
| **Halfwild** (pick) | 0 exact, 0 containing | Half lifted, half wild. Sally Green's 2015 novel [*Half Wild*](https://en.wikipedia.org/wiki/Half_Wild); a book title once drove an [opposition against ZeniMax](https://techraptor.net/gaming/news/redfall-trademark-dispute-settled-between-zenimax-and-author-jay-falconer). "Half" searches surface Half-Life. The .com is parked at Afternic; the .gg is free. |
| **Growback** (alternate) | 0 exact; nearest [Throwback](https://store.steampowered.com/app/2468030) | Clean on the web. Hair-regrowth search noise. The .gg is free. |
| Afterbloom | 0 exact | An itch.io jam entry; a [WITHERBLOOM mark](https://trademark.justia.com/owners/ever-curious-entertainment-inc-5785502) for a survival game; [132 Steam titles](https://github.com/leinstay/steamdb/releases/latest/download/steamdb.min.json.gz) contain "bloom". |
| Fenceline | 0 exact | Industrial-pollution jargon; fenceline.com is an active site. |
| Second Spring | 0 exact | Clean, but two common words. |
| Longbloom | 0 exact | A [Dutch flower company](https://www.cbinsights.com/company/longbloom/alternatives-competitors) runs longbloom.com. |

Keep "Zone", "Exclusion", "Atom" and "Fallout" out of any title.

## What changes in the docs

Every file below is founder-owned under D24. Agents propose; you edit.
- A new decision, D26, records the setting and replaces D1's "thaw-calendar hook".
- D20 names the new working title, still pending the search.
- CLAUDE.md: the title, the "northern county that thaws" line, the scope list and the deferred list.
- Agent files: art-director, game-designer and sim-engineer carry most terms. Net, server, client, devex, steam-release and marketing carry a few.
- Open questions 27 and 28 change wording only.
- The capsule rule changes: the cordon line carries the calendar on the capsule, and "Week N of 12" moves to the first screenshot.
- The research reports stay as written. They are history, and D26 overrides them.
- Code names (`Longthaw.sln`, `Longthaw.Client.csproj`) wait for the name search, so they change once.
- `FROZEN` in the queue and `schema_is_frozen` are not setting terms. Leave them.

### Term map

| Today | Becomes |
|---|---|
| Longthaw | Halfwild (pending D20; alternate Growback) |
| thaw / thaws week by week | the lifting / lifts week by week |
| thaw calendar, thaw-calendar hook | cordon calendar, cordon-calendar hook |
| thaw line, snowline | cordon line |
| thaw stage, thaw week, thaw season | cordon stage, cordon week, cordon season |
| `content/thaw.csv`, `thaw_stage()` | `content/cordon.csv`, `cordon_stage()` |
| exposed / flooded slots; floods low ground | lifted / closed slots; closes ground for works |
| `floodable`, `snow_capable` (kit columns) | `closable`, `growth_capable` |
| meltwater clock, meltwater level | exposure clock, exposure level |
| Meltline; 90-second pump extraction | the Inner Cordon; 90-second decon-gate extraction |
| volcanic winter, Long Winter, winter | the evacuation |
| northern county; Hallam County | an evacuated city; Alder (placeholder, unscreened) |
| the County (meta-layer) | the Boroughs |
| frozen (setting sense) | overgrown |
| snow, snowpack, ice | overgrowth |
| the twelve-stage thaw ramp | the twelve-stage cordon ramp |
| "white and slate in week one, ochre and green by week twelve" | "wild green and rust beyond a small lit hearth in week one; cleared streets, beds and lamplight across the lifted sectors by week twelve" |
| palette roles snow, slate, meltwater, ochre, green | wild, concrete, cordon-orange, lamplight, bed-green |
