# Cause and name: a sickness that is over, and "Reclaimed"

2026-10-09. A proposal, not a decision, revised after a skeptic review. It changes only the cause, run clock and name in [the setting review](setting-review.md). The decision and every founder-owned edit stay yours under [D24](../plan/decisions.md).

## The answer

**The skeptic refuted the draft, and the recommendation changed.** Drain samples from an empty city always test clean, so a health board could never close a sector without the sickness being live, and it read as COVID. Now the sickness is backstory only, and building inspectors lift the sectors. No mechanic changes.

**Cause: a sickness that is over, and a city left unsafe.** A sickness emptied the city. It is over, unnamed and never shown. The cordon stays because twelve years of neglect left the buildings unsafe, and a re-entry survey certifies them sector by sector. The world tells what people did while it lasted, not where it came from.

**Pitch:** *[Twelve years](setting-review.md#the-answer) after the city was sealed, the all-clear comes one sector a week. Build a hearth at the edge of the wild, go in with a friend for what the city left behind, and be back through the gates by dusk.*

**Title: only provisionally, as "Reclaimed: Fenceline".** Never bare: near-twins exist, and it is the genre's obvious default. It holds until D20 counsel clears it.

**Biggest risk:** the capsule and title read as The Division, which is live on Steam and owns co-op reclaiming of a pandemic-emptied city. A capsule test gates the spend.

## The causes compared

Shelf figures are paid online co-op survival games from 2022-26 in the [Steam dump](https://github.com/leinstay/steamdb/releases/latest/download/steamdb.min.json.gz): 1,280 titles, 12.0% reaching 1,000 reviews.

| Cause | Shelf | Verdict |
|---|---|---|
| P2 sickness, over | 23 disease titles without zombies, none with this premise | **Pick, as backstory only** |
| U2 unknown cause | 4 titles, 0 hits | **Its structural survey runs the week**; fallback cause |
| P4 plague-cordon language | "cordon" in 1 description | Texture only: coin stones, a memorial |

Rejected: P1 infected (182 titles, 9.9% hits; a bite is per-player status), U1 anomalies (STALKER and Pacific Drive own them), P3 Pluribus-like. U3, overgrowth as cause, is look only ([Area X](https://southernreach.fandom.com/wiki/Area_X)).

**Why P2.** Non-zombie, non-nuclear post-apocalyptic co-op survival hit 1,000 reviews 28.1% of the time in 2022-24 (n=32), against 17.2%. It matches your taste for The Division's emptied city, which is also the risk.

**The shelf is less open than descriptions say.** Steam shelves by tags: of 53 paid online co-op Post-apocalyptic + Survival titles since 2022, 34 (64%) are tagged Zombies. [The Division Resurgence](https://www.gamedeveloper.com/press-release/tom-clancy-s-the-division-resurgence-is-now-available-on-pc-via-ubisoft-connect-and-steam) launched free on Steam on 2026-08-05 with 4-player co-op, and [The Division 3](https://mp1st.com/news/the-division-3-big-impact-first-division-shaping-up-monster) is in production.

**Keep the word off the store.** Paid titles saying "pandemic" reached 1,000 reviews 1.4% of the time, against 3.4%.

**Why not P3.** Premise checked: an alien signal joins nearly everyone into one content hive mind, leaving [13](https://www.techradar.com/streaming/apple-tv-plus/pluribus-the-joining-explained-what-is-it-why-is-carol-spoiler-and-more-on-the-apple-tv-shows-big-mystery) out. Take one lesson: the people are not monsters. Here they are simply gone.

## How the cause runs the game

### Weekly clock: the re-entry survey

`cordon_stage(season_seed, week)` and its seeded table stay as the setting review defines them. Only nouns and art change.

Each week building inspectors survey the sectors. A sector certified structurally safe lifts. Others are posted "closed for works", and the works are real: shoring, demolition, rubble hauled out. The sim never models inspections.

Precedent: Christchurch's cordon came down [in stages](https://www.odt.co.nz/node/125860) ([June 2013](https://www.dpmc.govt.nz/sites/default/files/2024-04/first-week-of-junes-rolling-cordon-reduction-underway-4-june-2013.pdf)). Your COVID reference survives as the shape, phased reopening by zone, not the noun.

### Run clock: lock-up

One shared `dusk` value per run replaces the exposure bar. It rises only in-session and freezes with the session. `catch_up` never runs it.

Gates lock at printed `dusk` values, deep wicket gates first, the main gate last. Leaving early is safe; pushing on means a long walk out in failing light. The world shows it: lengthening shadows, floodlights, one bell per gate that locks, gate lamps turning red.

Missing the last gate ends the run under [open question 27](../plan/open-questions.md)'s default: gear kept, carried units left where you stood. Lethal Company's midnight departure, which loses the scrap if nobody is aboard, sold [about 10M](https://www.gamedeveloper.com/business/lethal-company-sold-an-estimated-10-million-copies).

Hard rule check: one shared value; no per-player meter, persistence, cure, mask or infection. Cost: dusk shadows on the Deck.

Risks: "reach safety before dark" is [Dying Light's loop](https://dyinglightgame.com/news/beauty-by-day-horror-by-night), so night stays a fail state, never a play space. A fenced zone, runs from a base and a closing exit is [Pacific Drive's](https://www.destructoid.com/how-to-use-gateways-in-pacific-drive/), so the capsule test asks.

### Enemy family: the boar sounder, bolder at dusk

Keep it. Boldness is a content curve keyed to `dusk`, adding no state. Haifa's boar walked the emptied city [by day](https://www.algemeiner.com/2020/04/17/beware-the-boar-wild-pigs-patrol-israeli-city-under-coronavirus-closure/) in 2020. No rats.

### Stakes mode: the Hospital Quarter (placeholder, deferred)

The old isolation hospital's sector never lifts; its placards stay red. Entry by licence, a [90-second](setting-review.md#how-it-replaces-the-thaw) inspection gate, carried units dropped on death: the Meltline one to one. Deferred under D2; [open question 28](../plan/open-questions.md)'s default (none in v1) holds. Never a "Dark Zone", The Division's term. The old placeholder collides with Steam's [Lazaretto](https://store.steampowered.com/app/628350) and [Lazaret](https://store.steampowered.com/app/2088280).

### What the world shows

Generated from sector state, none authored:
- Door placards on the [ATC-20 model](https://firesafety.vermont.gov/sites/firesafety/files/documents/disaster%20placards.pdf): green INSPECTED, yellow RESTRICTED USE, red UNSAFE, going red to green when a sector lifts. Rectangles only, never a cross or a white ground. Words and stripes also separate red from green.
- Coin stones at sector edges, after Eyam's [vinegar-filled holes](https://www.amusingplanet.com/2016/12/the-plague-village-of-eyam-and-its.html).
- Survey posts, and the lift board at the hearth.
- Plague pits only as a fenced memorial, never as loot.

Out: door crosses, since a red cross on white is the emblem the British Red Cross had [Prison Architect remove](https://techraptor.net/gaming/news/prison-architect-changes-ambulances-to-comply-with-geneva-conventions); and FEMA X-codes, whose bottom quadrant [counts the dead](https://louisianastatemuseum.org/sites/default/files/2024-08/Living-with-Hurricanes-Gallery-Guide-Grades-5-12-Final2_0.pdf).

Banned on the capsule and in copy: masks, swabs, biohazard marks, rats, bodies, placards, plague-doctor masks, and the words pandemic, COVID and curfew.

## The title

**Can you use "Reclaimed"? Only provisionally, with a subtitle, never bare.** A pre-screen: trademark registers were blocked, so D20 counsel decides.

For it: 0 exact "Reclaimed" in the 190,806-app dump; nine letters suit a small capsule the logo should ["nearly fill"](https://partner.steamgames.com/doc/store/assets/standard); and it says both halves: nature took the city, people take it back.

Against it:
- Near-twins: [Reclaimers](https://store.steampowered.com/app/2177010) (released 2025-04-13, $9.99), two unreleased Reclaim apps ([4661670](https://store.steampowered.com/app/4661670), [2910940](https://store.steampowered.com/app/2910940)) and [Reclaimer](https://store.steampowered.com/app/4303180). 30 names contain "reclaim", 8 more "reclam".
- Bossa used "Reclaimed" in 2021 for [an uplifting survival builder concept](https://www.altered.ai/blog/bossa-presents-uses-altered-studio-to-voice-the-protohype-video-trailer-for-concept-game-reclaimed/) (snippet only). The genre's default name is a weak mark.
- Over ruins with a sickness behind them, it leans toward The Division, fought ["to reclaim the city"](https://sirusgaming.com/the-division-2-review/).
- Noise: a jam entry, a RECLAIMED BY YOU mark, reclaimed wood.

**Recommended form: "Reclaimed: Fenceline", provisional.**
- The capsule shows RECLAIMED alone; the subtitle carries search and trademark strength.
- "Fenceline" has 0 Steam name matches and names the ring fence. But it is [EPA](https://www.epa.gov/sites/production/files/2016-06/documents/2010-0682_factsheet_communities_09292015.pdf) language for refinery pollution, a contamination cause next door to the dropped nuclear one. The capsule test asks for a pollution reading; if one shows, use "Reclaimed: All Clear" (0 exact, 2 containing).
- Half the capsule testers see "Unfenced" on the same art. If "Reclaimed" draws more Division reads, switch.
- Domains are unknown: NXDOMAIN does not prove a name free ([held names leave DNS](https://www.eurodns.com/blog/what-are-epp-status-codes)). Before any spend, a human checks reclaimedgame.com at a registrar or in RDAP. Do not buy reclaimed.game ([about $300-530 a year](https://www.gandi.net/en-GB/domain/tld/game), more if premium).
- Keep out: Day, Vault, Reclamation, Zone, Quarantine, Curfew, Hearth, Cordon, Joining, Others.

If D20 blocks the word: Unfenced, then Reclaimant (0 Steam name matches each), since a mark blocking "Reclaimed" may block a near-twin (my reasoning, not counsel's). Drop "The Reopening"; it says COVID.

## What changes from the setting review

| Setting review | Becomes |
|---|---|
| Halfwild (alternate Growback) | Reclaimed: Fenceline, provisional (pending D20; second All Clear; fallbacks Unfenced, Reclaimant) |
| contamination as the cause | the sickness: over, unnamed, never shown |
| the survey; survey posts | the re-entry survey by building inspectors; survey posts and door placards |
| exposure clock and level | lock-up clock, `dusk` level |
| past the limit, low ground costs health | gates lock at printed `dusk` values, deepest first |
| the Inner Cordon; decon gate | the Hospital Quarter (placeholder); inspection gate |
| fallback cause: chemical works | U2: drop the sickness, keep the survey |
| capsule test: under 25% name a nuclear franchise | under 25% name a zombie game, The Division, COVID or Pacific Drive; few read pollution |
| palette roles | add placard red, yellow and green, kept off the capsule (it bans hazard yellow) |

Unchanged: "closed for works" (now real works), `cordon_stage()`, the boar, the jobs, the art plan.

Also: the Thursday test adds [3 of 4](../plan/open-questions.md) testers naming the open gate without the HUD, and the Deck gate adds dusk captures.

## Skeptic review

**Verdict: refuted, medium confidence.** High on the weekly fiction and door marks, medium on the shelf, medium-low on the title; the run clock survived.

**Strongest argument.** Wastewater testing detects what living people shed, and CDC guidance warns non-detection alone should not justify relaxing measures ([snippet](https://stacks.cdc.gov/view/cdc/96226/cdc_96226_DS1.pdf), [study](https://www.medrxiv.org/content/10.1101/2021.06.11.21258797.full.pdf)). An empty city always tests clean, so a closed sector means the sickness is live, and certified zones and curfews read as COVID whatever the store says. The door crosses made a red cross on white; X-codes put a body count on every door.

**What changed.**
- Inspectors lift sectors; drain sampling and the COVID precedents are gone.
- Placards replace crosses and X-codes; limewash-white is dropped.
- "Curfew" became lock-up, and the disputed origin, which pulled toward Pacific Drive, is gone.
- The title is provisional: a Division arm and pollution check in the capsule test, no domain purchase.
- The shelf claim carries the zombie tag share. Lazaretto was renamed.

I re-ran the 2026-10-08 dump for the zombie share, near-twins, Lazaret and zero matches; the rest is the skeptic's sourcing.

## Gaps

Wikipedia, the Steam store, trademark registers, RDAP, the CDC library and altered.ai were blocked. Their figures come from search snippets.
