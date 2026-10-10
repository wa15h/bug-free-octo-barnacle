# Engine bake-off plan

Owned by the client engineer (`.claude/agents/client-engineer.md`, first task 3). This is week-0
item 4 in `docs/plan/decisions.md`: the identical vertical slice, the hours log, and the tie rule,
merged before `client/godot/` or `client/unity/` exists. The candidates are Godot 4.7 C# (.NET
build) and Unity 6 LTS with URP, each on the minor line its own pin-only PR sets (D17, rule 8).
The result writes D4's outcome at the ~600 h engine confirmation gate (D10).

## The slice: six rungs, identical in both engines

Each engine reaches the rungs in this order, one PR per rung per engine, never two engines in one
PR. Each rung PR names its rung and quotes that rung's Accept and Evidence lines word for word.
Where the engines name a part differently, the line names both.

1. **Controller and camera.** Accept: a third-person controller and over-the-shoulder follow
   camera driven by `client/lib` input events; keyboard, mouse, and gamepad; a 1280x800 window.
   Evidence: a 30-second video on keyboard and mouse, and a 30-second video on a gamepad.
2. **Kit and palette.** Accept: one low-poly kit imported under `content/`'s rules, recolored to
   the one palette, one ruin assembled. Evidence: screenshots at the week-one and week-twelve
   palettes (D26: wild green and rust beyond a small lit hearth in week one; cleared streets,
   beds, and lamplight across the lifted sectors by week twelve).
3. **Animation.** Accept: root motion (AnimationTree in Godot, Mecanim in Unity): idle, walk,
   run, and one melee swing whose active frames start no earlier than `MIN_TELEGRAPH_TICKS` after
   the swing begins. Evidence: a video with a tick overlay.
4. **Steam overlay.** Accept: the Steam overlay opened inside a Vulkan (Godot) or URP (Unity)
   build through the binding in `client/lib`; HS 5 calls this a spike, not a given. Evidence: a
   screenshot of the overlay over the running scene.
5. **Sidecar connect.** Accept: a sidecar connect through `client/lib`: spawn, `Hello`/`Welcome`,
   snapshots applied, one server-driven entity interpolated at 150 ms simulated latency, and a
   clean exit with the PID gone. Evidence: a video, plus the `Starved` count and the
   spawn-to-`Ready` milliseconds printed by the test.
6. **Exports.** Accept: Windows and Linux exports that launch and reach rung 5. Evidence: both
   artifacts' names and sizes, a launch log from each, and a gamepad session on Steam Deck
   hardware or a 1280x800 Proton run (D15).

## Rules of the bake-off

- Both engines' PRs for a rung build against the same `client/lib` commit, named by SHA in each.
- The same kit, palette, and animation set in both. No engine-specific feature the other cannot
  match; no shader is written during the bake-off (the art director's rule: it is built once, in
  the winner).
- The founder builds both engines' exports on their own machine and logs the time as founder
  hours under export (Q20 default, D21). Neither engine exports in CI during the bake-off.
- A rung is done for an engine when its PR has merged with the evidence above and the founder has
  logged at least one row for that engine and rung below.
- If an engine cannot reach a rung as written, the client engineer reports it to the producer with
  the evidence. The slice never changes for one engine alone; the founder decides in writing.
- The bake-off runs in weeks 0-6 (D4). If rung 6 is not done in both engines by then, it runs on:
  dates slip, scope does not (D28).
- After merge, a rung, a rule, or the formula changes only in a PR the founder signs, and the
  change applies to both engines.

## Hours log (Q19 default, D21; feeds `docs/plan/hours.md`)

Every founder hour spent on one engine alone is a row. Only the founder writes rows; no agent
writes, estimates, or edits one, and agent hours are never counted (HS 5). The hours start with
the Godot and Unity pin-only PRs (`docs/plan/week-0.md`, "Pushed to week 1").

- **Date**: the day worked, YYYY-MM-DD. **Engine**: Godot or Unity. **Rung**: 1-6, or `pin` for
  that engine's pin-only PR. **Hours**: decimal, in quarter hours (1.25 is 1 h 15 min).
- **Activity**, one of four: review (reading that engine's PR and its evidence), play (running its
  build, a Deck or Proton session included), install (its editor, export templates, SDK, and
  sign-in), and export (building its Windows and Linux exports on the founder's machine).
- **Note**: what the time went on, in a few words.
- Work that serves both engines (`client/lib`, this file, buying the kit) is not a row here; it is
  ordinary project hours.
- Every row is also inside that week's P in `hours.md`, never added on top. The producer sums each
  week's rows (Monday to Sunday, by Date) per engine into that week's Bake G and Bake U cells.

| Date | Engine | Rung | Activity | Hours | Note |
|---|---|---|---|---|---|

## Tie rule (pre-registered, D4)

G is the sum of Hours over every Godot row; U is the same over every Unity row. Both cover every
rung, the `pin` rows, and all four activities, and use exact values, rounded nowhere before the
comparison. Once rung 6 is done in both engines:

- Godot 4.7 wins if G <= 0.85 x U, that is, if Godot's total is at least 15% lower than Unity's.
- Otherwise Unity 6 wins: a Godot total less than 15% lower, an equal total, or a lower Unity total.

Worked example: an illustration, not logged hours, and in no sum. Godot lower by = (U - G) / U.

| Case | U | G | 0.85 x U | G <= 0.85 x U | Godot lower by | Winner |
|---|---|---|---|---|---|---|
| A | 40.0 | 35.0 | 34.0 | no | 5.0 h, 12.5% | Unity 6 |
| B | 40.0 | 33.5 | 34.0 | yes | 6.5 h, 16.25% | Godot 4.7 |
| C | 40.0 | 34.0 | 34.0 | yes, at the bar | 6.0 h, 15% | Godot 4.7 |
| D | 36.0 | 38.0 | 30.6 | no | -2.0 h, -5.56% | Unity 6 |

- A: 0.85 x 40.0 = 34.0, and 35.0 > 34.0. Godot logged 5.0 fewer hours and still loses: 12.5% is
  inside the 15% that goes to Unity 6.
- B: 33.5 <= 34.0, so Godot wins. C: exactly 15% lower meets the bar, so Godot wins.
- D: 0.85 x 36.0 = 30.6, and 38.0 > 30.6. Unity logged fewer hours, so Unity 6 wins.

Founder signature on the tie rule and this plan. The founder fills it in this PR; agents never do.

Signed: ______________________ Date: ____________

## Proposed disposal of the loser (for the producer to queue)

1. When the formula gives a result, the client engineer records it under Result with the losing
   project's last commit SHA. Client work goes on in the winner only; the loser's project is left
   as it is, with no rung, fix, upgrade, or export.
2. At the ~600 h engine confirmation gate (D10), the founder confirms the winner and writes D4's
   outcome (D24); the client engineer fills the D4 ADR (client-engineer task 6).
3. The producer then queues two PRs: one removes `client/<loser>/` and its `.gitignore` lines; one
   pin-only PR (rule 8) removes the loser's engine pin and marks its pin ADR superseded by the D4
   ADR. The loser's project stays recoverable from git history at the recorded SHA.

## Result

Empty until rung 6 is done in both engines. Client-engineer task 6 fills it: the totals, the tie
rule applied, per-rung evidence links, what each engine could not reach and why, and the D17
upgrade cost observed.
