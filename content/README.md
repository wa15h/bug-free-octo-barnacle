# Content tables

The rules every data table below obeys. The game designer owns this file and these tables; the sim
engineer owns the structs they load into and the validation that rejects a bad table at load.
`content/palette/` and `content/kits/` are the art director's and follow their own READMEs. Decisions: D2
(the cut scope), D18 and CLAUDE.md rule 9 (the founder can explain every row), D26 (setting words), and
CLAUDE.md "Stack" (fixed-point economy math). No table exists yet; each lands in a later queue item (W1-09
lands `jobs.csv` and `ledger.csv`), which follows these rules.

## The tables and their row counts

The sim's load errors enforce D2's counts: a fourth job row, a second enemy family, or other than twelve
cordon stages (`sim-engineer.md`), so widening the cut scope through a table fails CI, not only review. A
row for anything D2 defers (a fourth job, a second enemy family, the Hospital Quarter, a second kit, the
Boroughs) is scope, not tuning: it goes to the producer, and it enters only through a founder decision row
(D2, D31).

| File | One row is | Row count | Its `why` also |
|---|---|---|---|
| `jobs.csv` | a settler job: inputs, outputs, work hours, building | exactly three: `salvager`, `grower`, `builder` (D2, D26) | names the session shape it serves, a check-in or a run (MR 2) |
| `ledger.csv` | a starting account and its units | one per account; D2 fixes none | - |
| `buildings.csv` | a building | one per building; D2 fixes none | - |
| `blueprints.csv` | a blueprint: its ledger costs, work hours, and the building it raises | one per blueprint; D2 fixes none | - |
| `cordon.csv` | a cordon stage | exactly twelve, `week` 1 to 12, one each (D2, D26); week 13 reseeds the season and is never a row | - |
| `enemies.csv` | a sounder member, or one point of the sounder's `dusk`-keyed boldness curve (D26: a content curve, no new state) | one family, the wild boar sounder (D2): members `piglet`, `sow`, `tusker` (D26), plus one row per curve point; a second family is a load error | - |
| `attacks.csv` | an attack: its telegraph, active, and recovery ticks (D9) | one per attack; D2 fixes none | - |
| `gear.csv` | a gear piece, or the death-loss bound (Q27 default, D21) | one per piece, plus one bound row; D2 fixes none | prints each stat's formula, every term a cell |
| `perks.csv` | a perk | one per perk; D2 fixes none | - |
| `loot.csv` | one entry in one loot pool | one per entry; D2 fixes none | gives the entry's weight out of 1000 (rule 8) |

## Rules

Each rule ends with its check: what a reviewer reads in the table's rows (for rule 4, a screenshot or a
doc; for rule 7, the PR) to see it holds.

1. **One CSV per table.** One file per table above: header row first, one row per thing, UTF-8, LF line
   ends. A cell is quoted only when it holds a comma, and no cell holds a double quote.
   Check: the first line is the header; no two rows share an `id`; every quoted cell holds a comma.
2. **Every number is an integer in a declared unit.** The header ends in the unit: `_units` (a whole
   amount: resource units, damage, carry, a weight), `_ticks` (sim ticks, rule 5), `_hours` (in-game
   hours), `_pct` (0 to 100), or `_hundredths`. No float, no `0.3`, no `30%`. The one number without a
   suffix is `cordon.csv`'s `week`, the cordon week. Any other column without a unit holds words: the
   `id`, the `why`, a cell naming another row's `id` (its header ends `_id`), or text the player reads.
   Check: every cell under a unit header is digits, with at most a leading `-`; a `_pct` cell is 0 to 100.
3. **Every row ends with a `why`.** The last column is `why`: one sentence a tester could read aloud that
   explains the number, as "`<n>` work hours: a Salvager breaks down one day of junk in one day, so the
   pile never grows unattended". A row without a `why` fails review; so does a `why` that says "feels
   right" or gives no reason (D18, CLAUDE.md rule 9).
   Check: read the last cell aloud; it names the number and the reason for it.
4. **Every number the player sees is a cell.** No constant lives in a doc, a comment, or code: a doc or
   code that needs a number names its cell (file, row `id`, column). If the client needs a number, it
   reads the same row the sim reads.
   Check: take a number from a screenshot or a doc; the cell it names holds that number, at that unit.
5. **Time is in-game hours.** A time column ends `_hours`. Two exceptions: an attack's telegraph, active,
   and recovery in `attacks.csv` are `_ticks`, because D9 and `MIN_TELEGRAPH_TICKS` count ticks; and
   `cordon.csv`'s `week` is the cordon week (rule 2). The sim owns the one constant that turns hours into
   ticks; no table holds it.
   Check: a time header ends `_hours`, or `_ticks` in `attacks.csv`, or is `cordon.csv`'s `week`.
6. **Rows are keyed by a stable `id`.** The first column is `id`: kebab-case (lowercase letters and digits,
   words joined by single hyphens), unique in its file. A rename is a new row plus a deprecated row, never
   an edit in place, so replay fixtures and saves keep resolving: the old row keeps every cell and sets
   `replaced_by_id` to the new row's `id`; a live row leaves it empty. A table gains that column, just
   before `why`, in its first rename PR, which waits on the sim loader reading it; the producer sequences
   it with the sim engineer. A rename in `jobs.csv`, `cordon.csv`, or `enemies.csv`, whose ids or counts
   D2 and D26 fix, goes to the producer first; how a deprecated row counts is the sim's validation.
   Check: the `id` is kebab-case; a deprecated row's `replaced_by_id` chain ends at a live row in its file.
7. **A table change ships with what consumes it.** The PR links the sim fixture or property test that
   loads the changed rows, green on its head; a ledger number change links the conservation property
   test, green (the reviewer's rule for `content/`). Where the queue row moves the loader or the
   conservation test to a later item, the PR names that item (W1-09: N-01 and N-02).
   Check: the PR's "Accept on evidence" links that run on the PR's head, or names the items its queue row
   moves the loader and the conservation test to.
8. **Weights sum to 1000.** Loot and placement weights are integers that sum to 1000 per pool, and each
   weight row's `why` says so.
   Check: the row's `why` gives its weight out of 1000; the pool's rows add up to 1000.
9. **Setting words come from D26's glossary.** Every setting word in a doc, a row `id`, or a `why` comes
   from the glossary under D26 in `docs/plan/decisions.md`, "the single source for every setting term",
   so text the player reads in a cell follows it too. The sickness is never named or shown.
   Check: read the row's word cells; each setting word comes from the glossary under D26, none is an old
   term the glossary replaces, and none names or describes the sickness.
