# Engine bake-off plan

Owned by the client engineer (`.claude/agents/client-engineer.md`, first task 3). This is week-0
item 4 in `docs/plan/decisions.md`: the identical vertical slice and the founder's scorecard
(D31), signed and merged before `client/godot/` or `client/unity/` exists. The candidates are
Godot 4.7 C# (.NET build) and Unity 6 LTS with URP, each on the minor line its own pin-only PR
sets (D17, rule 8). The founder's written verdict picks the engine; the founder confirms it and
writes D4's outcome at the engine confirmation gate (`docs/plan/gates.md`, D10).

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

- Neither engine's pin-only PR nor any rung PR opens until this plan, with the founder's signature
  below, is on main (D31).
- Both engines' PRs for a rung build against the same `client/lib` commit, named by SHA in each.
- The same kit, palette, and animation set in both. No engine-specific feature the other cannot
  match; no shader is written during the bake-off (the art director's rule: it is built once, in
  the winner).
- The founder builds both engines' exports on their own machine (Q20 default, D21); nothing logs
  the time (D31). Neither engine exports in CI during the bake-off.
- A rung is done for an engine when its PR has merged with the evidence above.
- If an engine cannot reach a rung as written, the client engineer reports it to the producer with
  the evidence: what failed, and the PRs and return rounds tried. The slice never changes for one
  engine alone. The founder rules in writing: either that engine cannot reach the rung, which ends
  the bake-off and goes to the verdict, or the rung changes for both engines.
- The bake-off runs in weeks 0-6 (D4). If rung 6 is not done in both engines by then, it runs on:
  dates slip, scope does not (D28).
- After merge, a rung, a rule, or the scorecard changes only in a PR the founder signs, and the
  change applies to both engines.

## Scorecard (pre-registered, D4, D31)

What the founder weighs, for each engine. No row records anyone's time, and no row is a formula.

1. **Rung results and video.** Each rung: done, or ruled unreachable, with its merged PR and the
   evidence above, linked.
2. **Return rounds.** Each rung: how many times that engine's PRs for it were returned
   (`returned:`, by the reviewer or the founder) before merge, split and fix PRs included.
3. **Exports and runs on the targets.** Rung 6's Windows and Linux artifacts and the Deck or
   1280x800 Proton session: whether each launched, reached rung 5, and played on a gamepad, and
   what failed.
4. **Play notes.** The founder's own written notes on each rung in each engine, a few lines from
   running that rung's build.
5. **Stability under the pinned minor.** Crashes, editor or export breakage, and patch releases on
   the pinned minor line met during the bake-off, each with the PR or log that shows it.
6. **Counted lines, information only.** Changed lines per engine across its pin-only and rung PRs,
   counted as D22 counts them, with the listed generated files beside them. They never decide.

The client engineer fills rows 1-3, 5, and 6 under Result, each with a link. The founder writes
row 4 in the table below and the verdict; no agent writes, edits, or proposes either.

| Rung | Engine | Build (PR) | Founder's play note |
|---|---|---|---|

## Verdict rule (pre-registered, D4, D31)

- The founder writes the verdict under Result once rung 6 is done in both engines, or once the
  founder rules that one engine cannot reach a rung.
- The verdict names the winner and gives the founder's reasons, each citing the scorecard rows it
  rests on. It is the founder's judgment on merit, toward the target state: the cut scope at
  full quality (D2, D28).
- An engine the founder has ruled unable to reach a rung the other reached does not win.
- A tie goes to Unity 6: if the founder cannot separate the engines on the scorecard, Unity 6 wins.

Founder signature on this plan, its scorecard, and its verdict rule. Only the founder fills it;
agents never do.

Signed: ______________________ Date: ____________

## Proposed disposal of the loser (for the producer to queue)

1. When the founder writes the verdict, the client engineer records the losing project's last
   commit SHA under Result. Client work goes on in the winner only; the loser's project is left
   as it is, with no rung, fix, upgrade, or export.
2. At the engine confirmation gate (`docs/plan/gates.md`, D10), the founder confirms the winner
   and writes D4's outcome (D24); the client engineer fills the D4 ADR (client-engineer task 6).
3. The producer then queues two PRs: one removes `client/<loser>/` and its `.gitignore` lines; one
   pin-only PR (rule 8) removes the loser's engine pin and marks its pin ADR superseded by the D4
   ADR. The loser's project stays recoverable from git history at the recorded SHA.

## Result

Empty until rung 6 is done in both engines or the founder rules that one cannot reach a rung.
Client-engineer task 6 fills scorecard rows 1-3, 5, and 6 with their evidence links and what each
engine could not reach and why; the founder then writes the verdict here, with reasons.
