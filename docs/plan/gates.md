# Gate tracker

Producer. Gates are cumulative build-hour triggers (D10, D23), never calendar dates. Each gate has
seven fields: its Status row holds the trigger, the cumulative build-hours now, and the projected
bands; its Rules entry holds the decision IDs, the evidence required, the pass rule, and the fail
consequence. The producer re-derives Status weekly from `hours.md`; Rules change only with a
decision. A gate fires at the first `hours.md` row whose Cum build reaches its trigger, or at its
event; the producer presents the evidence against the pass rule and the founder gives the verdict.

## How the bands are made

- Cum build comes from `hours.md`: Build = max(0, (P - 2.5) x 0.8 x 0.92), D23 floored at 0.
- Pace, in build-hours per project week. Rolling: the mean Build of the last 4 window weeks in
  `hours.md` (all of them while fewer than 4; none before the first row). Fixed:
  (h - 2.5) x 0.8 x 0.92, so 20.24 at 30 h/week, 12.88 at 20, and 9.2 at 15.
- Calendar weeks to go = (trigger - Cum build) / pace x 52/42. The 52/42 turns 42 project weeks
  a year (D5) into calendar weeks; deloads and other zero weeks sit inside it.
- Point = the Monday after the latest logged week, plus the calendar weeks to go. Until week 0's
  row sets its dates, the origin is the first day of week 0's planned dates in `week-0.md`.
- Band = the point plus or minus 2 weeks, printed as the month or months it touches. The point is
  never printed. Week numbers are calendar weeks from week 0; deload weeks count (`hours.md`).
- 30 h/week is the plan (D5). The 20 and 15 h/week columns are the re-baseline fallbacks and
  reproduce HS 6's gate list within one month. The weekly summary reads the rolling column.
- D11 check: at 30 h/week this method puts the two-player loop (~735 likely build-hours) at
  Aug-Sep 2027 and the page (~875) at Oct-Nov 2027, D11's "around Aug 2027" and "around Oct 2027".

## Status at week 0

No `hours.md` row is logged yet: every Cum build is 0 and the rolling column is empty ("-").

| Gate | Trigger (build-hours) | Now | Rolling | 30 h/week (plan) | 20 h/week | 15 h/week |
|---|---|---|---|---|---|---|
| Architecture review 1 | event: weeks 0-6 spikes end (141.7 at 30 h) | week 0 | week 6: Nov-Dec 2026 | same | same | same |
| Gate 0: the ant farm is fun alone | 250 | 0 | - | Jan-Feb 2027 | Mar-Apr 2027 | May-Jun 2027 |
| Velocity gate | 400 | 0 | - | Mar-Apr 2027 | Jun-Jul 2027 | Oct-Nov 2027 |
| Camera lock | 400 | 0 | - | Mar-Apr 2027 | Jun-Jul 2027 | Oct-Nov 2027 |
| Hard two-client trigger | 500 | 0 | - | Apr-May 2027 | Aug-Sep 2027 | Jan-Feb 2028 |
| Engine confirmation | 600 | 0 | - | Jun-Jul 2027 | Nov-Dec 2027 | Apr-May 2028 |
| Architecture review 2 | event: two-player loop (~735 likely) | 0 | - | Aug-Sep 2027 | Feb-Mar 2028 | Aug-Sep 2028 |
| Gate 1: a 30-minute run is fun alone | 900 | 0 | - | Oct-Nov 2027 | May-Jun 2028 | Jan-Feb 2029 |
| 90-day post-page gate | event: page live (~875 likely) + 90 days | page not live | - | Jan-Feb 2028 | Aug-Sep 2028 | Mar-Apr 2029 |
| Gate 2: the Thursday group plays unasked | 1,100 | 0 | - | Jan-Feb 2028 | Oct-Nov 2028 | Jul-Aug 2029 |
| D2 add-back trigger | 26 weeks at 18+ logged hours (not build-hours) | 0 weeks | - | week 25: Mar-Apr 2027 | same | never (under 18) |
| D5 re-baseline | Avg 8 logged hours under 20 | no Avg 8 | - | does not fire | does not fire | week 7's row: Nov-Dec 2026 |
| Quarterly fatigue check | event: the week before each deload | next: week 11 | week 11: Dec 2026-Jan 2027 | same | same | same |
| Review-saturation counter | 8 saturated weeks in a row | 0 of 8 | measured only | not projected | not projected | not projected |

## Rules

Fun-gate pass lines are the game designer's playtest scripts, run as written with at least four
testers and filed (D10); Gate 0's 3-of-4 line and Gate 2's cohort follow Q26's default. A low
rolling average, a V under 0.6, or a fatigue flag re-dates the bands and nothing else; a failed
fun gate goes to the founder as a decision with written options, never an automatic cut (D28).

**Architecture reviews 1 and 2.** Decisions: D12. Fixed fee, $500-1,500 each, or a $0 review swap.
- Evidence: the reviewer's written report. Review 1 covers the deterministic-sim harness, the wire
  protocol and versioning, persistence and save migration, and the sidecar contract (HS 5).
  Review 2 comes at the two-player loop (HS 9.7). The founder books and pays.
- Pass: the report is filed, and each finding is queued by the producer or answered by the founder.
- Fail: none to scope. A review not yet booked is reported in each weekly summary.

**Gate 0: the ant farm is fun alone.** Decisions: D10, D27, D28.
- Evidence: the designer's Gate 0 script: one tester alone with the settlement viewer for 20
  scripted minutes; sheets filed.
- Pass: 3 of 4 testers keep watching past minute 20 unprompted, and can say what each settler is
  doing and what next hour's ledger will show.
- Fail: a founder decision with written options (D28).

**Velocity gate.** Decisions: D2, D10, D11, D28.
- Evidence: V4 on the first `hours.md` row whose Cum build reaches 400. Its Done cells need
  `milestones.md`'s item table.
- Pass: V4 of 0.8 or more keeps the current bands. From 0.6 to under 0.8 keeps the cut scope and
  bars add-backs.
- Fail: V4 under 0.6 re-dates every band from measured V and nothing else (D28).

**Camera lock.** Decisions: D13, D23. D13's ~400 logged hours count as build-hours, as HS 9.11's do.
- Evidence: a prototype of both cameras, the third-person follow camera and the top-down diorama,
  with video in its PR.
- Pass: the founder locks the camera.
- Fail: none. D13's default stands until the founder locks it; the producer reports it weekly.

**Hard two-client trigger.** Decisions: D8, D9, D10, D28.
- Evidence: `headless-session`'s `proof.txt` from main: two bots connect, move, loot, and see each
  other's state replicated, with per-tick hashes equal to a standalone replay.
- Pass: that proof exists by ~500.
- Fail: no such session by ~500 re-dates every band from measured V and nothing else (D28).

**Engine confirmation.** Decisions: D4, D10, D24. The bake-off itself runs in weeks 0-6.
- Evidence: Bake G and Bake U in `hours.md`, summed from `client/BAKEOFF.md` (Q19 default), for
  the identical slice in both engines.
- Pass: fewer logged hours wins. A gap within 15% goes to Unity 6, by `BAKEOFF.md`'s tie formula
  (D4). The founder writes D4's outcome (D24).
- Fail: a slice unfinished at the trigger is reported to the founder, who decides.

**Gate 1: a 30-minute run is fun alone.** Decisions: D10, D28.
- Evidence: the designer's Gate 1 script: a solo 30-minute run from the hearth to a ruin and
  back; sheets filed.
- Pass: 3 of 4 testers finish, bank units, answer yes to "another run right now?", and can
  explain one gear number from its tooltip (MR 7).
- Fail: a founder decision with written options (D28).

**90-day post-page gate.** Decisions: D10, D12, D19, D28. Its band follows the page's likely
hours, as HS 6 and D11 print it.
- Evidence: marketing's day-90 reading (`docs/plan/marketing/gate-90-day.md`): net organic
  wishlists from the tracker, or a live publisher conversation (a named publisher sent terms or
  scheduled a call inside the window, logged with its date).
- Pass: ~1,500-2,000 organic wishlists, or a live publisher conversation, unlocks the client/feel
  contractor (D12); the founder spends. From 500 to 1,500: no hire, no cut, and a re-read on the
  demo-page day (Q36 default).
- Fail: under ~500 means zero hires and no cut (D28).
- Every outcome: the founder records one D19 option (below) by the first `hours.md` row after day
  90, whose Cum build is this gate's hour mark; from that row until it is recorded, each weekly
  summary reports it missed.

**Gate 2: the Thursday group plays unasked.** Decisions: D10, D28.
- Evidence: the session log. The cohort is every Thursday tester who launched the build on day 0.
- Pass: more than 30% launch it on day 7, with no founder message to the group from day 1 to day 7.
- Fail: a founder decision with written options (D28).

**D2 add-back trigger.** Decisions: D2, D28.
- Evidence: P in `hours.md`. The count runs in calendar weeks from the first week of the first
  rolling 4-week window (window weeks, as `hours.md` defines them) with a mean P of 18 or more.
  A window under 18, or a missing row, restarts it.
- Pass, at 26 weeks (six months): D2's deferred items become eligible, not queued. Nothing is
  queued until the founder adds a decision-log row naming the item and the producer re-derives the
  bands with it. A velocity-gate V4 from 0.6 to under 0.8 bars add-backs.
- Fail: none. The cut scope ships (D2).

**D5 re-baseline.** Decisions: D5, D11, D28.
- Evidence: Avg 8 in `hours.md`, read once 8 window weeks exist.
- Pass: Avg 8 of 20 or more; the bands stand.
- Fail: under 20, the producer re-derives every band at the measured pace (HS 6's 15 h/week column
  when it sits near 15), brought to the founder as a re-plan, not a failure. Dates move; scope and
  quality do not (D28). A milestone that moves more than one quarter goes to the founder first.

**Quarterly fatigue check.** Decisions: D5, D28. A standing queue item in weeks 11, 25, and 39;
the deloads are weeks 12-13, 26-27, and 40-41.
- Evidence: the quarter's `hours.md` rows: P, Total, Fatigue, and Avg 8.
- Pass: no Fatigue yes, and Avg 8 of 20 or more; the bands stand.
- Fail: a Fatigue yes heads the next weekly summary. It, or Avg 8 under 20, re-dates the bands and
  never cuts or adds scope (D28).

**Review-saturation counter, at the D6 revised cap.** Decisions: D6, D12.
- Evidence: the reviewer's weekly report must give the count of line-by-line PRs waiting for the
  founder, set against two weeks of that week's D6 PR count (about 20-25 at 30 h, 8-15 at 15 h).
  A week counts when that count is over two weeks' worth, or when nightly invariant failures
  exceed what the 40% review share can triage (HS 7). Any other week resets the counter to 0.
- Pass: under 8; nothing changes.
- Fail, at 8 in a row: reported to the founder with the eight weeks' numbers. The producer
  proposes no hire (D12).

## D19: the employment options, written at week 0

At the 90-day gate the founder chooses one of these in writing, on that gate's evidence (HS 9.14).
Not deciding is not an option.
1. A four-day week or a sabbatical, funded from a savings runway the founder names.
2. A publisher advance, benchmarked against Hooded Horse's 65/35 no-recoup terms; a 100%-recoup
   deal is refused (HS 9.9).
3. Hobby pace on the cut scope, with early access in 2031-33 accepted in writing (HS 6's 15 h/week
   early-access band: ~1,850 build-hours at P20 to ~2,590 at P50).
