# Cause and name: a sickness that is over, and "Reclaimed"

2026-10-09. A proposal, not a decision. It changes only the cause, the run clock and the name in [the setting review](setting-review.md). The decision and every edit to founder-owned files stay yours under [D24](../plan/decisions.md).

## The answer

**Cause: P2 dressed in P4's look.** A sickness emptied the city, and it is over. Nobody names it or shows it. Where it came from stays disputed, told only by what was left behind.

**Pitch:** *[Twelve years](setting-review.md#the-answer) after the city was sealed, the all-clear comes one sector a week. Build a hearth at the edge of the wild, go in with a friend for what the city left behind, and be back through the gates by dusk.*

**Title: yes, as "Reclaimed: Fenceline".** The bare word has no exact Steam match, but it is too generic to find or protect alone.

**Biggest risk:** the capsule reads as a COVID game or as The Division. A capsule test gates the spend, and U2 is the fallback with identical mechanics.

## The causes compared

Shelf figures are paid online co-op survival games from 2022-26 in the [Steam dump](https://github.com/leinstay/steamdb/releases/latest/download/steamdb.min.json.gz): [1,280 titles, 12.0% reaching 1,000 reviews](https://github.com/leinstay/steamdb/releases/latest/download/steamdb.min.json.gz).

| Cause | Shelf | Design fit | Tone | Verdict |
|---|---|---|---|---|
| P2 sickness over, ring lifting | [23 disease titles without zombies](https://github.com/leinstay/steamdb/releases/latest/download/steamdb.min.json.gz), none with this premise | Real reopenings give the weekly reason | Hopeful | **Pick** |
| P4 plague-cordon language | "cordon" in [1 description](https://github.com/leinstay/steamdb/releases/latest/download/steamdb.min.json.gz) | Door marks make sector states readable | Humane | **Adopt as the look** |
| U2 unknown, no anomalies | [4 titles, 0 hits](https://github.com/leinstay/steamdb/releases/latest/download/steamdb.min.json.gz) | No reason for weekly lifts; invites a reveal | Melancholy | Runner-up |
| U3 overgrowth is the cause | [Area X](https://southernreach.fandom.com/wiki/Area_X) and The Last of Us own it | Pulls toward a growth sim | Wonder | Look only |
| P3 Pluribus-like | The show owns it | Needs a crowd of NPCs | Uncanny | Reject |
| P1 infected | [182 titles, 9.9% hits](https://github.com/leinstay/steamdb/releases/latest/download/steamdb.min.json.gz) | A bite becomes per-player status | Grim | Reject |
| U1 anomalies | STALKER, [Pacific Drive](https://www.techradar.com/gaming/pacific-drive-review) | Every anomaly is a new sim rule | Eerie | Reject |

**Why P2.** Non-zombie, non-nuclear post-apocalyptic co-op survival hit 1,000 reviews [28.1% of the time in 2022-24 (n=32), against 17.2%](https://github.com/leinstay/steamdb/releases/latest/download/steamdb.min.json.gz). It matches your taste: The Division put [40M players](https://www.kitguru.net/?p=506849) into a city a pandemic emptied.

**Keep the word off the store.** Paid titles saying "pandemic" reached 1,000 reviews [1.4% of the time, against 3.4%](https://github.com/leinstay/steamdb/releases/latest/download/steamdb.min.json.gz). The COVID film Songbird [scored 9%](https://www.slashfilm.com/578274/first-songbird-reviews-early-buzz/). Station Eleven shows the other road: a post-pandemic story called ["ultimately uplifting"](https://www.rollingstone.com/tv/tv-reviews/station-eleven-review-1265799/).

**Why not P3.** Premise checked: an alien signal carries RNA that joins nearly everyone into one content hive mind, and Carol is one of [13](https://www.techradar.com/streaming/apple-tv-plus/pluribus-the-joining-explained-what-is-it-why-is-carol-spoiler-and-more-on-the-apple-tv-shows-big-mystery) left out. It is [Apple's most-watched show](https://www.thewrap.com/media-platforms/tv/pluribus-finale-viewership-nielsen-streaming-high-apple-tv/). Take one lesson only: the people are not monsters. Here they are simply gone, and your settlers are the ones coming back.

## How the cause runs the game

### Weekly clock: the all-clear

`cordon_stage(season_seed, week)` stays as the setting review defines it. Only the fiction changes.

Each week the health board reads drain samples from every sector. A sector that tests clean long enough is certified clear and lifts. A sector that flags is posted "closed for works": drains flushed, topsoil dug out. Your fences and diggers stay.

The sim never models samples or infection. The schedule is the seeded table.

Precedents:
- Italy re-colored its regions [every week](https://www.thelocal.it/20210519/map-which-parts-of-italy-will-be-covid-19-white-zones-in-june).
- New York reopened in [4 phases, at least 2 weeks apart, against 7 metrics](https://www.duanemorris.com/alerts/new_york_forward_plan_reopens_construction_phase_one_0520.html).
- Programs sampled sewersheds of about [1,471 people](https://www.medrxiv.org/content/10.1101/2022.06.07.22276055v1) at manholes.
- Anthrax kept Gruinard shut [almost 50 years](https://www.bitesizedbritain.co.uk/gruinard-island---deadly-for-nearly-50-years111/), until the topsoil came off.

### Run clock: curfew

One shared `dusk` value per run replaces the exposure bar. It rises only in-session and freezes with the session, as meltwater did. `catch_up` never runs it.

The cordon's gates lock at printed `dusk` values, deep wicket gates first and the main gate last. Leaving early is safe. Pushing on means the long walk out in failing light.

Players read it from the world: shadows lengthening, floodlights coming on, one bell per gate that locks, gate lamps turning red.

Missing the last gate ends the run under [open question 27](../plan/open-questions.md)'s default: gear kept, carried units left where you stood. The exposure rule that cost health is gone.

Precedents: France's COVID curfew began at [6pm on 16 January 2021](https://www.connexionfrance.com/news/curfew-at-1800-for-whole-of-france-from-saturday/404537). Lethal Company's midnight departure, which loses the scrap if nobody is aboard, sold [an estimated 10M copies](https://www.gamedeveloper.com/business/lethal-company-sold-an-estimated-10-million-copies).

Hard rule check: one shared value, no per-player meter, nothing persistent, no cure, mask or infection.

Cost: the lighting ramp is one global value, but long dusk shadows cost frame time on the Deck.

Risk: "reach safety before dark" is [Dying Light's loop](https://dyinglightgame.com/news/beauty-by-day-horror-by-night). It stays distinct because night is a fail state, never a play space, and the enemy is boar.

### Enemy family: the boar sounder, bolder at dusk

Keep it. Its boldness is a content curve keyed to `dusk`, so it adds no state.

The fiction now has a pandemic precedent. In the 2020 lockdown, Haifa's boar walked the city [by day in family groups](https://www.algemeiner.com/2020/04/17/beware-the-boar-wild-pigs-patrol-israeli-city-under-coronavirus-closure/). Rats stay out; they pull toward disease.

### Stakes mode: the Lazaretto (deferred)

The old isolation hospital's sector is the one never lifted. Entry by licence, one inspection gate with a [90-second](setting-review.md#how-it-replaces-the-thaw) wait, carried units dropped on death. It is the Meltline one to one.

It stays deferred under D2, and [open question 28](../plan/open-questions.md)'s default (none in v1) holds. Never call it a Dark Zone: The Division walled [a Midtown quarantine](https://gamespace.com/all-articles/news/the-divisions-story-so-far/) into one.

### What the world shows

All of it is generated from sector state, none authored:
- Door stencils on the [FEMA X-code](https://www.iavoad.org/wp-content/uploads/2022/07/SAR-Marking-a-Building.pdf) pattern.
- Marks that turn red to white when a sector lifts, as London's 1666 order went from a [red cross for 40 days to a white cross for 20](https://www.nationalarchives.gov.uk/education/resources/great-fire-of-london-examine-the-evidence/preventing-the-plague-in-may-1666/).
- Coin stones at sector edges, after Eyam's [vinegar-filled holes](https://www.amusingplanet.com/2016/12/the-plague-village-of-eyam-and-its.html).
- Sampling posts on manholes, and the lift board at the hearth.
- Plague pits only as a fenced memorial, never as loot.

Banned on the capsule and in copy: masks, swabs, biohazard marks, rats, bodies, and the words pandemic and COVID. Ban the plague-doctor mask too; the co-op game Unplagued sits at [33% of 4,797 reviews](https://github.com/leinstay/steamdb/releases/latest/download/steamdb.min.json.gz).

## The title

**Can you use "Reclaimed"? Yes, with a subtitle.** This is a pre-screen. Every trademark register was blocked, so D20's human search still applies.

For it:
- [0 exact](https://github.com/leinstay/steamdb/releases/latest/download/steamdb.min.json.gz) "Reclaimed" in the 190,806-app dump. Only [Makai Kingdom: Reclaimed and Rebound](https://store.steampowered.com/app/1732060) contains it.
- Snippets showed no RECLAIMED word mark for games (low confidence).
- Nine letters suit a small capsule the logo should ["nearly fill"](https://partner.steamgames.com/doc/store/assets/standard).
- It says both halves of the game: nature took the city, people take it back.

Against it:
- The root is crowded: [38 Steam titles](https://github.com/leinstay/steamdb/releases/latest/download/steamdb.min.json.gz), including unreleased genre neighbours [Reclaim Earth](https://store.steampowered.com/app/2868550) and [Reclaim The Wasteland](https://store.steampowered.com/app/3400290).
- An itch.io jam entry is called [Reclaimed](https://itch.io/jam/puls-game-jam/rate/3401252).
- [RECLAIMED BY YOU](https://trademark.justia.com/879/53/reclaimed-by-87953180.html) is a class 41 mark, for a painting studio.
- reclaimed.com and reclaimed.gg are taken ([DNS only](https://rdap.org/domain/reclaimed.game)).
- Search noise: reclaimed wood, [ASOS's Reclaimed Vintage](https://www.asos.com/us/men/a-to-z-of-brands/reclaimed-vintage/cat/), and [817 Steam descriptions](https://github.com/leinstay/steamdb/releases/latest/download/steamdb.min.json.gz) using reclaim*.
- Fallout 76's ["Reclamation Day"](https://wvpublic.org/story/wvpb-news/reclamation-day-fallout-76-released-to-the-public/) sits close.

**Recommended form: "Reclaimed: Fenceline".**
- The capsule shows RECLAIMED alone. The subtitle carries search and trademark strength.
- "Fenceline" has [0 Steam name matches](https://github.com/leinstay/steamdb/releases/latest/download/steamdb.min.json.gz) and names the ring fence.
- reclaimedgame.com and reclaimed.game returned NXDOMAIN, so they are likely free.
- Second choice: "Reclaimed: All Clear" ([0 exact, 2 containing](https://github.com/leinstay/steamdb/releases/latest/download/steamdb.min.json.gz)). Use "all clear" in-game for the lift either way.
- Keep out: Day, Vault, Reclamation, Zone, Quarantine, Hearth ([18 Steam names](https://github.com/leinstay/steamdb/releases/latest/download/steamdb.min.json.gz)), Cordon ([an unreleased CORDON](https://store.steampowered.com/app/5275610)), Joining, Others.

If D20 blocks the word: Unfenced first, then Reclaimant. My reasoning, not counsel's: a mark that blocks "Reclaimed" may also block a near-twin. Drop "The Reopening"; it says COVID out loud.

## What changes from the setting review

Founder-owned under D24. Agents propose; you edit.

| Setting review | Becomes |
|---|---|
| Halfwild (alternate Growback) | Reclaimed: Fenceline (pending D20; fallbacks Unfenced, Reclaimant) |
| contamination as the cause | the sickness: over, unnamed, never shown |
| the survey; survey posts | the sampling round; sampling posts on manholes |
| exposure clock and level | curfew clock, `dusk` level |
| past the limit, low ground costs health | gates lock at printed `dusk` values, deepest first |
| the Inner Cordon; decon gate | the Lazaretto (placeholder); inspection gate |
| fallback cause: chemical works | U2, a structural survey, as [Christchurch lifted area by area](https://www.dpmc.govt.nz/sites/default/files/2024-04/first-week-of-junes-rolling-cordon-reduction-underway-4-june-2013.pdf) |
| capsule test: few name a nuclear franchise | few name a zombie game, The Division or COVID |
| palette roles | add mark-red and limewash-white for doors |

Unchanged: "closed for works", `cordon_stage()`, the boar, the three jobs, the art plan.

Also:
- The Thursday test adds: [3 of 4](../plan/open-questions.md) testers name the open gate without the HUD.
- The Deck capture gate adds dusk captures.
- The capsule test asks about The Division. A survey-orange cordon may read as its [orange SHD display](https://gameranx.com/updates/id/21916/article/the-division-agents-wear-a-holographic-smart-watch-here-s-how-it-looks/) (untested).
- Recheck Atomfall's "3.7M players". Rebellion reported [1.5M in the first week](https://steamdeckhq.com/news/atomfall-1-5-million-players/).

## Gaps

Wikipedia, Rotten Tomatoes, the Steam store, trademark registers and RDAP were blocked. Their figures come from search snippets.
