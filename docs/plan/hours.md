# Hours log

The founder types one row a week at week start, for the week just ended (D5). This file is the
source of truth (Q3 default): the producer fills derived cells (D23); a later `crates/tools` hours
tool only checks and recomputes them. No agent writes a founder cell, estimates an hour, or
back-fills a week. A missing row is flagged at the top of `queue.md`; a second freezes the queue.

## Columns

Founder cells, under five minutes a week:
- **Wk**: the week number, deload weeks included. **Dates**: that week's Monday to Sunday.
- **P**: project hours logged: everything on the project, review and the D23 overhead included.
- **Total**: all working hours that week, the day job included.
- **Rev**: review hours, a part of P.
- **Deload**: yes in a D5 deload week (the first fortnight is weeks 12-13), else no.
- **Fatigue**: yes or no, self-reported. A yes heads the next weekly summary and re-dates the
  bands only (D28). Total and Fatigue feed the quarterly fatigue check (D5).

Derived cells, written by the producer once the founder's cells land (a row with P blank gets
none): Cum P, Avg 8, Build, Cum build, Done, V4, and Bake G and Bake U, that week's Godot and
Unity hours summed from the `client/BAKEOFF.md` table (Q19 default; a part of P, not extra).

## Formulas (written once; every derived cell follows them)

A project week is a row with Deload no. Rolling windows skip deload rows, because D11's ~850
build-hours a year is 20.24 x 42 project weeks (D5).
- Build = max(0, (P - 2.5) x 0.8 x 0.92) (D23). At 30 h: 27.5 x 0.736 = 20.24.
- Cum P and Cum build: sums over every row to date, deload rows included.
- Avg 8 = mean P over the last 8 project weeks (over all of them while fewer than 8 exist). Once
  8 exist, under 20 triggers the D5 re-baseline: the bands re-date; scope and quality hold (D28).
- Done = the likely hours, from the `milestones.md` item table, of items that merged that week
  and pass their acceptance criteria; a partly done item counts 0. Blank until that table exists.
- V4 = Done over the last 4 project weeks / (P over the same 4 weeks x 0.85). Blank while fewer
  than 4 exist; never acted on for one week; read at the ~400 build-hour velocity gate.
- Derived cells show one decimal (V4 two), rounded half up from exact values; sums use exact values.

## Log

| Wk | Dates | P | Total | Rev | Deload | Fatigue | Cum P | Avg 8 | Build | Cum build | Done | V4 | Bake G | Bake U |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0 | | | | | | | | | | | | | | |

## Worked example: an illustration, not a logged week, and in no sum

It assumes three earlier project weeks logged P of 26, 30, and 32, with 50.0 Done between them.

| Wk | Dates | P | Total | Rev | Deload | Fatigue | Cum P | Avg 8 | Build | Cum build | Done | V4 | Bake G | Bake U |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| example | illustration | 30 | 70 | 12 | no | no | 118.0 | 29.5 | 20.2 | 79.5 | 19.0 | 0.69 | 3.0 | 2.5 |

- Build: (30 - 2.5) x 0.8 x 0.92 = 27.5 x 0.736 = 20.24, shown 20.2.
- Cum P: 26 + 30 + 32 + 30 = 118. Avg 8: 118 / 4 = 29.5; with 4 project weeks, D5 is not read yet.
- Cum build: (23.5 + 27.5 + 29.5 + 27.5) x 0.736 = 108 x 0.736 = 79.488, shown 79.5.
- V4: (50 + 19) / (118 x 0.85) = 69 / 100.3 = 0.688, shown 0.69.
- Rev 12 is 30 x 0.4, the HS 7 review share. Bake G and U are that week's `BAKEOFF.md` sums.
