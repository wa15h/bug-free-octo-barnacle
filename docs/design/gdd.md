# The Reclaimers: game design document

Game designer. The cut scope (D1, D2) is the whole design. This half holds the pillars, loop, and hearth.
The runs half (runs and the lock-up clock, combat, the cordon calendar) goes before Deferred. Every setting
word comes from D26's glossary. The doc holds no tuning number. Every number the player ever sees is an
integer cell in a `content/` table (`content/README.md`); the sim owns its own constants (Q8 default).

**Scope (D2).** One cordon season of twelve real weeks, one region (Alder, the overgrown city), one kit,
one enemy family (a wild boar sounder), three settler jobs, and a 2-player listen server (D8).

## Pillars (MR 2; D1, D26)

MR 2 reads the founder's taste as one loop: build a home base, go out into a systemic world to gather, come
back and compound. It names seven pillars. Each holds in v1 as follows.

1. **The hearth is the anchor.** (MR 2: the base as anchor of identity.) The hearth is what a player owns
   across every session and every season. It is the first view, read from the top-down diorama (D13), and
   nothing in it is ever wiped (D14).
2. **Every system is legible.** (MR 2: legible systems, with GlassBox agents, junk-to-components, explicit
   gear math.) Settlers are visible agents, the Salvager turns junk into components, and the ledger prints
   every unit each hour. Every number on screen is a table cell, and no roll is hidden (`legibility.md`).
   The founder can explain every rule after one read (D18, rule 9).
3. **Progress compounds over months.** (MR 2: months-long compounding progression.) A season is twelve real
   weeks of the cordon calendar (D1, D26). Settlers turn what a run banks into components and buildings,
   and the hearth carries all of it into the next season (D14).
4. **The run goes out and comes back.** (MR 2: the out-and-back expedition.) Runs last 30-90 minutes, in
   the third-person follow camera (D2, D13). The runs half designs them.
5. **Solo works; a friend multiplies it.** (MR 2: solo-viable play with a social multiplier.) Solo is the
   same server on localhost (D8; MR 7). A second player joins the host's hearth (D2); none is ever needed.
6. **Opt-in stakes: none in v1.** (MR 2: bounded opt-in stakes.) v1 carries no opt-in stake (Q28 default,
   D21); the deferred list says why.
7. **The world is told by its environment.** (MR 2, restated in D26's terms: a world told through its
   environment.) The world is Alder, an overgrown city whose ring fence lifts sector by sector each week
   (D26). The kit, the lifting, and the ledger tell it, with no heavy authored narrative (an MR 2 dislike).
   The look runs from "wild green and rust beyond a small lit hearth in week one" to "cleared streets, beds,
   and lamplight across the lifted sectors by week twelve" (D26).

**Session shapes (MR 2).** 2-10 minute async check-ins, 30-90 minute runs, and occasional 2-4 hour deep
sessions; every system serves at least one. MR 2's dislikes are written as rules in `prohibitions.md`.

## The loop (D1, D2, D8, D26)

One loop of three steps: check-in, run, return.

1. **Check-in (a 2-10 minute session).** The host starts the game, and the hearth catches up on the hours
   since the last session, printed as ledger lines (see The ledger). The player reads the ledger and the
   cordon calendar, assigns settlers, and places blueprints. A check-in alone is a whole session.
2. **Run (a 30-90 minute session).** Solo or with one friend, the players go out through the cordon gates
   into the lifted sectors and come back before the lock-up clock locks the gates (D26).
3. **Return.** What the players carry back through the gates is banked: each carried unit moves from a
   player's pack into the hearth's store, as a ledger line. Settlers then turn it into components and
   buildings, whether or not anyone is watching.

Between sessions the hearth keeps working: catch-up is one deterministic function, identical on every
server (MR 7), so a player comes back to the hearth a live session would have made. Each real week the
cordon calendar lifts some sectors and closes others for works (D26), so the next run has new ground.

Two players share one hearth: the host's (D2, D8). A guest's character lives in the host's save (Q12
default, D21).

## The hearth (D10, D13, D27)

The hearth is the players' settlement, read from the top-down diorama (D13). It must be fun to watch alone
before any run exists: Gate 0 is one tester alone with the settlement viewer and no run (D10, D27). So the
hearth runs from its starting rows alone. A watcher reads each settler's job, task, carried units, and
destination from the diorama without a click (`legibility.md`), and can say what next hour's ledger will show
(`docs/plan/gates.md`, Gate 0).

### Settlers and jobs (D2, D14, D26)

- The hearth's settlers, buildings, and balances start as the accounts in `content/ledger.csv`. A settler's
  row there names its starting job, so the hearth works before anyone assigns one.
- There are three jobs, one row each in `content/jobs.csv`: inputs, outputs, work hours, and the building
  the job works at. A fourth job row is a load error (D2).
  - **Salvager: junk to components.** It breaks the junk that runs bring back into components.
  - **Grower: planted beds to food.** A planted bed is its workplace; tending the bed grows food.
  - **Builder: components to blueprint buildings.** Its `content/jobs.csv` row names the site as its
    building and holds zero inputs, outputs, and work hours: its free workplace is a placed site no Builder
    holds, and that site's blueprint row gives the cycle (see Blueprints and buildings).
- Each settler holds one job at a time, so the diorama shows it without a click; the player assigns it there.
- A job runs in cycles. A cycle starts when the store holds the row's inputs and a workplace of the row's
  building is free. The inputs move from the store to the settler, who carries them to the workplace. When
  the row's work hours are done, the inputs are used up and the outputs go to the store.
- The row's work hours are the whole cycle, walk included. Where a building stands changes no number: there
  is no adjacency bonus and no walking cost.
- A settler that cannot start a cycle is idle and shows why: the store lacks the inputs, or no workplace is
  free. Settlers start cycles in the order `content/ledger.csv` lists them, so the same hearth always prints
  the same ledger.
- A new assignment takes effect when the settler's current cycle ends; an idle settler changes at once. No
  carried unit and no work hour is lost to a reassignment.
- Junk enters the hearth only from the starting balances and from a run's return. A hearth no one runs for
  slows down and waits; nothing in it is lost while it waits.
- Work hours are production, never a countdown: nothing to skip, nothing to buy (D14).

### The ledger (D2, D18, D21)

- The hearth holds three resources: junk, components, and food (D26). Every unit is an integer balance in a
  named account: a settler, a building (the store and sites included), or a player's pack. All economy math
  is integer and fixed-point (CLAUDE.md, Stack), and every number shown is its cell at its declared unit.
  The hearth has no coin: blueprints cost components and food, and nothing in it is bought or sold.
- Every change is a transfer between two accounts. A unit enters only from a named source and leaves only to
  a named sink, so none appears or vanishes off the ledger. The hearth's one source is a cycle's outputs and
  its one sink a cycle's inputs used up, a blueprint's cost included; runs add their own.
- The ledger prints once per in-game hour: every unit produced, consumed, moved, or lost, with the job, the
  settler, and the building that did it. Each resource's lines sum to zero that hour, sources and sinks
  included (`legibility.md`).
- Catch-up is printed, never silent: a returning player sees the lines a live session would have printed,
  and the first line states the elapsed hours and any clamp (one cordon season, Q9 default).
- Time in the hearth is in-game hours. The sim owns the constants that turn hours into ticks (Q8 default: one
  `TICK_HZ` and one `HEARTH_EVERY`; their values wait on the founder's answer to Q8).

### Blueprints and buildings (D2, D26; MR 7)

- A blueprint is a row in `content/blueprints.csv`: its cost in components and food, its work hours, and the
  building it makes (a row in `content/buildings.csv`). Components go into the building and food feeds the
  hands that raise it, so all three jobs feed the one thing the player places.
- The player picks a blueprint and places it on the hearth's free ground in the diorama: one pick and one
  placement, with no nested menu (MR 7: building that does not live in nested menus).
- A placed blueprint is a site, and a site is a Builder's workplace. Builders take sites in the order they
  were placed, so the same hearth always prints the same ledger.
- The Builder's cycle is the site's blueprint row: its cost moves from the store to the Builder at the start
  and is used up when the work hours are done. Its output is the building, not units in the store. Every
  blueprint cost is a ledger transfer.
- The store is one starting building: it holds every unit no settler, site, or pack holds, with no capacity
  limit, so no absence wastes a unit and nothing pushes a player to check in on time (D14). Every other
  building is a workplace for its row's job; more workplaces let more settlers work that job at once.

### What persists (D14)

A season reseeds the city after week twelve. The hearth, its settlers, its buildings, its ledger, and every
player's gear persist across the reseed; nothing a player owns is deleted by a season, an update, or a save
migration.

## Deferred (D2, D31)

D2's cut scope is the whole design, and nothing below is designed here, even as later. Both lists are the
deferred list: an item enters only when the founder adds a decision row naming it (D31). The second list
holds the four items CLAUDE.md's cut scope marks never in v1.

Deferred until the founder adds it back:
- 4-player drop-in
- the free headless dedicated binary
- the Hospital Quarter mode (D26): the opt-in mode MR 7 defers, so v1 has no opt-in stake (Q28 default)
- a second kit
- the Boroughs meta-layer (D26)
- localization

Never in v1:
- client prediction or rollback (D9)
- strangers
- free text between players
- in-game voice promises
