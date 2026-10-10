# 0003: Sidecar watchdog and process lifecycle

## Status
Accepted

## Decision log ID
D8 (the listen server is a sidecar process the client spawns, with a watchdog: silent-control-socket exit, PID kill on
clean exit, tick-boundary saves; no in-process server). Required as a design choice inside D8 (`README.md`), not by
rule 6: the spike is std-only. Relies on Q16's default (D21): `CONTROL_SILENCE_SECS` 10, `HEARTBEAT_SECS` 2, localhost
TCP on a client-chosen port.

## Context
HS 5 names the sidecar's lifecycle a top risk: a process from Godot's `OS.create_process` "will not terminate when
Godot terminates", and .NET's `Process.Start` behaves the same, so a client crash orphans a listen server holding the
save. HS 5 and 9.3 answer with D8's watchdog and drop the in-process server, even for solo play, which is the same
sidecar on localhost. Week-0 item 2 and W0-09 prove it with a std-only Rust parent and child on
`claude/spike-sidecar-watchdog`, never merged, run on ubuntu and windows by that branch's workflow. Options
considered: an in-process server (D8 removes it); stdin/stdout pipes instead of TCP (Q16); a kill by PID alone.
Tick-boundary saves need a tick loop, so the server's tick-loop work proves them. The founder writes the outcome as
D8's line (W0-19, D24); `SIDECAR.md`, the server skeleton, and the client's sidecar launcher wait on it.

## Decision
The server is its own process on every path and ends itself. The client picks a free port (binds `127.0.0.1:0`, reads
the port, closes it) and passes it as `--control`. In `server-engineer.md`'s startup order, the server takes the
save-dir lock, then binds that port, accepts one connection, and stops listening, so only the first connection can
send `Shutdown`. `Hello` gets `Ready` with the server's PID. The server saves, sends `Exiting`, frees the lock, and
exits 2 when the socket closes or stays silent for `CONTROL_SILENCE_SECS` (from the bind if no one connects), or 0 on
`Shutdown`. It exits 3 if `server.lock` already holds a running PID's record with its start time, else clears it.
The C# launcher (`client/lib` `Sidecar/`, client-engineer task 2) copies [`parent.rs`](https://github.com/wa15h/bug-free-octo-barnacle/blob/11342c731a9c1dadcf2d335f5e269b485269dd16/spike/sidecar-watchdog/src/bin/parent.rs):
1. Read the start time at spawn (unreadable: the child already exited, so read its exit code). Connect, retrying while watching for the child's exit, up to a spawn-to-`Ready` limit; check `Ready` names the spawned PID. Exit 3 means another server holds the save: tell the player and kill nothing.
2. Write PID and start time to `sidecar.pid` after the connect, before `Hello`: an exit-3 child never connects, so it never replaces a live server's record.
3. Heartbeat every `HEARTBEAT_SECS` from a dedicated thread; stop and join it before `Shutdown`.
4. After `Shutdown` wait the grace (here `CONTROL_SILENCE_SECS + 1`), then kill only if the start time still matches: on Windows read it and kill through one process handle; on Linux read it (`/proc/<pid>/stat` field 22) immediately before the kill.

> D8 outcome, for the founder to write (W0-19, D24): the sidecar watchdog spike passed its eight cases on ubuntu and
> windows (ADR 0003): `Ready` over localhost TCP on a client-chosen port; exit 2 within `CONTROL_SILENCE_SECS + 1` of
> a silent or closed control socket, a killed client's orphan included; exit 0 on `Shutdown`, else a kill by recorded
> PID and start time, never a reused PID; a second server exits 3 on a live server's lock, and a dead server's lock is
> cleared. Two servers starting at once both took the spike's lock, so `SIDECAR.md` needs an atomic one.

## Consequences
`SIDECAR.md` takes this lifecycle and exit codes 0, 2, 3, and names what no decision does: the grace, the
spawn-to-`Ready` limit, the code for a port the server cannot bind, and an atomic save-dir lock, such as an OS file
lock (the spike reads an empty lock as stale). The C# launcher passes the eight cases. Not proven here: tick-boundary
saves, proto messages, a server no one connects to, a taken port, an engine-spawned, desktop, or Proton parent.

## Evidence
[Run 38039367778](https://github.com/wa15h/bug-free-octo-barnacle/actions/runs/38039367778) at spike head `11342c7`: both jobs fail only on `lock_race`, a case W0-09 does not list. Cells summarize each case's `RESULT` line (Ready: from spawn, then 12 s alive on heartbeats; seconds: from the last heartbeat, from `Shutdown`, or from the parent's kill; race: tries where both servers took the lock, of 100 with no lock and of 100 with a dead PID's lock).

| [ubuntu-latest](https://github.com/wa15h/bug-free-octo-barnacle/actions/runs/38039367778/job/114176370382), rustc 1.99.0 | Ready | `Shutdown` | silent socket | ignored `Shutdown` | reused PID | killed parent | live lock | dead lock | lock race |
|---|---|---|---|---|---|---|---|---|---|
| image ubuntu24 20261004.327.1 | PASS, 111 ms | PASS, exit 0 | PASS, exit 2 at 10.02 s | PASS, killed at 11 s (137) | PASS, not killed | PASS, exit 2 at 0.10 s | PASS, exit 3, lock and `sidecar.pid` kept | PASS, cleared | FAIL, 100 and 0 |

| [windows-latest](https://github.com/wa15h/bug-free-octo-barnacle/actions/runs/38039367778/job/114176370194), rustc 1.98.1 | Ready | `Shutdown` | silent socket | ignored `Shutdown` | reused PID | killed parent | live lock | dead lock | lock race |
|---|---|---|---|---|---|---|---|---|---|
| image win25-vs2026 20260925.250.1 | PASS, 236 ms | PASS, exit 0 | PASS, exit 2 at 10.05 s | PASS, killed at 11 s (1) | PASS, not killed | PASS, exit 2 at 0.10 s | PASS, exit 3, lock and `sidecar.pid` kept | PASS, cleared | FAIL, 2 and 7 |

## Pinned versions touched
None: std only. The spike workflow used `actions/checkout` `3d3c42e5aac5ba805825da76410c181273ba90b1` (tag v7.0.1), the SHA W0-04 proposes.
