# 0003: Sidecar watchdog and process lifecycle

## Status
Accepted

## Decision log ID
D8 (the listen server is a sidecar process the client spawns, with a watchdog: silent-control-socket
exit, PID kill on clean exit, tick-boundary saves; no in-process server). Required as a design choice
inside D8 (`README.md`), not by rule 6: the spike is std-only. Relies on Q16's default (D21):
`CONTROL_SILENCE_SECS` 10, `HEARTBEAT_SECS` 2, localhost TCP on a client-chosen port.

## Context
HS 5 names the sidecar's lifecycle a top risk: a process from Godot's `OS.create_process` "will not
terminate when Godot terminates", and .NET's `Process.Start` behaves the same, so a client crash
orphans a listen server holding the save. HS 5 and 9.3 answer with D8's watchdog and drop the
in-process server, even for solo play, which is the same sidecar on localhost. Week-0 item 2 (W0-09)
proves it with a std-only Rust parent and child on `claude/spike-sidecar-watchdog`, never merged, run
on ubuntu and windows by that branch's workflow. Options considered: an in-process server (D8 removes
it); stdin/stdout pipes (Q16); the server listening on a port the client picked; a kill by PID alone.
Tick-boundary saves need a tick loop, so server task 4 proves them, not this spike.

## Decision
The server is its own process on every path and ends itself. The client binds `127.0.0.1:0` and
passes the OS-given port as `--control`; the server takes the save-dir lock, then connects. No process
can take the port first, and the server listens on nothing, so nothing else can send it `Shutdown`.
`Hello` gets `Ready` with the server's PID. The server saves, sends `Exiting`, frees the lock, and
exits 2 when the socket closes or is silent for `CONTROL_SILENCE_SECS`, or 0 on `Shutdown`. It exits
3 if `server.lock` names a running PID with its recorded start time, else it clears the lock. The C#
launcher (`client/lib` `Sidecar/`, client-engineer task 2) copies [`parent.rs`](https://github.com/wa15h/bug-free-octo-barnacle/blob/54ef4ca6f35047f9258c736ed179a8915cd89dd3/spike/sidecar-watchdog/src/bin/parent.rs):
1. Bind `127.0.0.1:0` before spawning; accept one connection; check `Ready` names the spawned PID.
2. Before `Hello`, record PID and start time in memory and `sidecar.pid` (none if it already exited 3).
3. Heartbeat every `HEARTBEAT_SECS` from a dedicated thread; stop and join it before `Shutdown`.
4. After `Shutdown` wait the grace (here `CONTROL_SILENCE_SECS + 1`), then kill only if the start time
   still matches: on Windows read it and kill through one process handle; on Linux read it
   (`/proc/<pid>/stat` field 22) immediately before the kill.
5. On exit 3 another server holds the save: tell the player and kill nothing.

> D8 outcome, for the founder to write (W0-19, D24): the sidecar watchdog spike passed on ubuntu and
> windows (ADR 0003): `Ready` over localhost TCP; exit 2 within `CONTROL_SILENCE_SECS + 1` of a silent
> or closed control socket, a killed client's orphan included; exit 0 on `Shutdown`, else a kill by
> recorded PID and start time, never a reused PID; a live save-dir lock exits 3, a dead one is cleared.

## Consequences
`SIDECAR.md` takes this lifecycle, exit codes 0, 2, 3, and a grace constant no decision names yet; the
C# launcher passes these eight cases. Not proven here: tick-boundary saves, `crates/proto` messages.

## Evidence
[Run 38035133893](https://github.com/wa15h/bug-free-octo-barnacle/actions/runs/38035133893) at spike head `54ef4ca`, both jobs green; cells summarize each case's `RESULT` line (Ready: from spawn, then 12 s alive on heartbeats; seconds: from the last heartbeat or the kill).

| [ubuntu-latest](https://github.com/wa15h/bug-free-octo-barnacle/actions/runs/38035133893/job/114163889675), rustc 1.99.0 | Ready | `Shutdown` | silent socket | ignored `Shutdown` | reused PID | killed parent | live lock | dead lock |
|---|---|---|---|---|---|---|---|---|
| image ubuntu24 20261004.327.1 | PASS, 101 ms | PASS, exit 0 | PASS, exit 2 at 10.02 s | PASS, killed at 11 s (137) | PASS, not killed | PASS, exit 2 at 0.10 s | PASS, exit 3 | PASS, cleared |

| [windows-latest](https://github.com/wa15h/bug-free-octo-barnacle/actions/runs/38035133893/job/114163889792), rustc 1.98.1 | Ready | `Shutdown` | silent socket | ignored `Shutdown` | reused PID | killed parent | live lock | dead lock |
|---|---|---|---|---|---|---|---|---|
| image win25-vs2026 20260925.250.1 | PASS, 117 ms | PASS, exit 0 | PASS, exit 2 at 10.05 s | PASS, killed at 11 s (1) | PASS, not killed | PASS, exit 2 at 0.10 s | PASS, exit 3 | PASS, cleared |

## Pinned versions touched
None: std only. The spike workflow used `actions/checkout` at ADR 0001's SHA `3d3c42e5` (v7.0.1).
