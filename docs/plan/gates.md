# Gate tracker

Producer. Gates fire when their evidence exists on main, never on founder time (D10, D31): there
are no hour marks, bands, or forecast dates. Each gate has six fields: its Status row holds the
event that fires it and the day it was reached; its Rules entry holds the decision IDs, the
evidence required, the pass rule, and the fail consequence. Reached on is the day the gate's
evidence landed, with the founder's verdict beside it once given; the producer fills it from main
and filed sheets, and Rules change only with a decision. The producer presents the evidence
against the pass rule; the founder gives the verdict.

## Status

No gate is reached yet: every Reached on cell is empty.

| Gate | Fires when | Reached on |
|---|---|---|
| Architecture review 1 | the weeks 0-6 spikes end | |
| Gate 0: the ant farm is fun alone | the settlement viewer on main runs the designer's merged Gate 0 script | |
| Bake-off result | both engines finish the slice, or the founder rules that one cannot reach a rung | |
| Camera lock | the two-camera prototype merges in the winner with video | |
| Two-client proof | `headless-session`'s `proof.txt` is on main | |
| Two-player loop and engine confirmation | the loop's video is on main in the winner | |
| Architecture review 2 | the two-player loop is reached | |
| Gate 1: a 30-minute run is fun alone | a solo 30-minute run from the hearth to a ruin and back plays end to end from main, with the designer's Gate 1 script merged | |
| 90-day post-page gate | 90 days after the page goes live | |
| Gate 2: the Thursday group plays unasked | day 7 of the day-0 build the founder names | |
| Review-saturation counter | the eighth counted week in a row | |

## Rules

Fun-gate pass lines are the game designer's playtest scripts, run as written with at least four
testers and filed (D10); Gate 0's 3-of-4 line and Gate 2's cohort follow Q26's default. A failed
fun gate goes to the founder as a decision with written options, never an automatic cut (D28).

**Architecture reviews 1 and 2.** Decisions: D12. Fixed fee, $500-1,500 each, or a $0 review swap.
- Evidence: the reviewer's written report. Review 1 covers the deterministic-sim harness, the wire
  protocol and versioning, persistence and save migration, and the sidecar contract (HS 5).
  Review 2 comes at the two-player loop (HS 9.7). The founder books and pays.
- Pass: the report is filed, and each finding is queued by the producer or answered by the founder.
- Fail: none to scope. A review not yet booked is reported in each weekly summary.

**Gate 0: the ant farm is fun alone.** Decisions: D10, D27, D28, D31.
- Evidence: the designer's Gate 0 script: one tester alone with the settlement viewer for 20
  scripted minutes; sheets filed.
- Pass: 3 of 4 testers keep watching past minute 20 unprompted, and can say what each settler is
  doing and what next hour's ledger will show.
- Fail: a founder decision with written options (D28).

**Bake-off result.** Decisions: D4, D31.
- Evidence: both engines' rung PRs merged with their evidence, set against the scorecard the
  founder signed in `client/BAKEOFF.md` before either engine project existed. An engine cannot
  reach a rung only when the founder rules so in writing on the client engineer's evidence report;
  that engine loses, unless the founder changes the rung for both engines.
- Pass: the founder's written verdict on merit, with reasons, names the winner; if the founder
  cannot separate them, Unity 6 wins (D4). Client work then continues in the winner only.
- Fail: none to scope. An unfinished slice runs on; nothing is cut to finish it (D28).

**Camera lock.** Decisions: D13, D31. The two-camera prototype, in the bake-off winner, is the
first client item queued after the result.
- Evidence: a prototype of both cameras, the third-person follow camera and the top-down diorama,
  with video in its PR.
- Pass: the founder locks the camera.
- Fail: none. D13's default stands until the founder locks it; the producer reports it weekly.

**Two-client proof.** Decisions: D8, D9, D10, D28, D31. Its items lead the queue once Gate 0's
items have merged.
- Evidence: `headless-session`'s `proof.txt` from main: two bots connect, move, loot, and see each
  other's state replicated, with per-tick hashes equal to a standalone replay.
- Pass: that proof exists, and the founder accepts it.
- Fail: none to scope (D28). Its items keep the head of the queue until the proof is on main.

**Two-player loop and engine confirmation.** Decisions: D4, D10, D24, D31.
- Evidence: video from main in the bake-off winner: two players on the sidecar listen server
  finish one out-and-back run from the hearth and bank its units.
- Pass: the founder confirms the winner and writes D4's outcome (D24); the producer queues the
  loser's removal. Architecture review 2 and page-era preparation follow (`producer.md`).
- Fail: none to scope (D28). Until the founder confirms a winner in writing, D4 stays Pending.

**Gate 1: a 30-minute run is fun alone.** Decisions: D10, D28, D31.
- Evidence: the designer's Gate 1 script: a solo 30-minute run from the hearth to a ruin and
  back; sheets filed.
- Pass: 3 of 4 testers finish, bank units, answer yes to "another run right now?", and can
  explain one gear number from its tooltip (MR 7).
- Fail: a founder decision with written options (D28).

**90-day post-page gate.** Decisions: D10, D12, D19, D28, D31. Day 0 is the day the founder
publishes the page.
- Evidence: marketing's day-90 reading (`docs/plan/marketing/gate-90-day.md`): net organic
  wishlists from the tracker, or a live publisher conversation (a named publisher sent terms or
  scheduled a call inside the window, logged with its date).
- Pass: ~1,500-2,000 organic wishlists, or a live publisher conversation, unlocks the client/feel
  contractor (D12); the founder spends. From 500 to 1,500: no hire, no cut, and a re-read on the
  demo-page day (Q36 default).
- Fail: under ~500 means zero hires and no cut (D28).
- Every outcome: the founder records one D19 option (below) by day 97; from day 98 until it is
  recorded, each weekly summary reports it missed.

**Gate 2: the Thursday group plays unasked.** Decisions: D10, D28, D31. Day 0 is the build the
founder names; the producer asks for it at the Gate 1 verdict.
- Evidence: the session log. The cohort is every Thursday tester who launched the build on day 0.
- Pass: more than 30% launch it on day 7, with no founder message to the group from day 1 to day 7.
- Fail: a founder decision with written options (D28).

**Review-saturation counter.** Decisions: D6, D12, D31.
- Evidence: the reviewer's weekly report, with ages: each line-by-line PR that waited for the
  founder, ready or held, more than five days, and each nightly invariant failure untriaged more
  than seven days. A week counts when it has either (HS 7); any other week resets the counter to 0.
  Deload days (weeks 12-13, 26-27, and 40-41 from week 0, then a fortnight every 14 weeks) and
  founder-pause days count toward no wait; a week spent wholly under either neither counts nor
  resets.
- Pass: under 8; nothing changes.
- Fail, at 8 in a row: reported to the founder with the eight weeks' numbers. The producer
  proposes no hire (D12).

## D19: the employment options, written at week 0

At the 90-day gate the founder chooses one of these in writing, on that gate's evidence (HS 9.14).
Not deciding is not an option.
1. A four-day week or a sabbatical, funded from a savings runway the founder names.
2. A publisher advance, benchmarked against Hooded Horse's 65/35 no-recoup terms; a 100%-recoup
   deal is refused (HS 9.9).
3. Hobby pace on the cut scope, with an undated early access accepted in writing (D31).
