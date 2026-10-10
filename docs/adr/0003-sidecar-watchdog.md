# 0003: Sidecar watchdog and process lifecycle

## Status
Proposed

## Decision log ID
D8 (the listen server is a sidecar process the client spawns, with a watchdog: silent-control-socket
exit, PID kill on clean exit, tick-boundary saves; no in-process server). Required as a design choice
inside D8 (`README.md`), not by rule 6: the spike is std-only. Relies on Q16's default (D21):
`CONTROL_SILENCE_SECS` 10, `HEARTBEAT_SECS` 2, localhost TCP on a client-chosen port.

## Context
HS 5 names the sidecar's lifecycle a top risk: a process from Godot's `OS.create_process` "will not
terminate when Godot terminates", and .NET's `Process.Start` behaves the same, so a client crash
orphans a listen server holding the save. HS 5 and 9.3 answer with D8's watchdog and drop the
in-process server, even for solo play, which is the same sidecar on localhost. Week-0 item 2
(`decisions.md`, one day) and W0-09 prove it on `claude/spike-sidecar-watchdog`, never merged: a
std-only Rust parent and child, run on ubuntu and windows by that branch's workflow, check spawn and
`Ready` over localhost TCP; exit 2 within `CONTROL_SILENCE_SECS + 1` of a silent control socket;
exit 0 on `Shutdown`, else a kill by recorded PID and start time, since an OS reuses a dead PID; a
killed parent's child exiting by watchdog; and a dead PID's save-dir lock cleared on restart, a live
one's exit 3. Options considered: an in-process server (D8 removes it); stdin/stdout pipes instead
of TCP (Q16); a kill by PID alone. Tick-boundary saves need a tick loop, so the server's tick-loop
work proves them, not this spike. The founder writes the outcome as D8's line (W0-19, D24);
`SIDECAR.md`, the server skeleton, and the client's sidecar launcher wait on it.

## Decision
The choice in one paragraph. For a dependency: the package name exactly as the manifest writes it,
the exact version, the maintainer, the license, and which crates or projects may use it.

## Consequences
What becomes easier, what becomes harder or forbidden, and any follow-up item for the queue.

## Evidence
Links to the runs, logs, benchmarks, or upstream pages the decision rests on, with the commit SHA or
version each one used. Evidence described but not linked is missing.

## Pinned versions touched
Each pin this ADR sets (D17) and its current value, or "none". An upgrade PR (rule 8) edits this ADR
in place: the new value and a dated line saying why. A changed choice is a new, superseding ADR.
