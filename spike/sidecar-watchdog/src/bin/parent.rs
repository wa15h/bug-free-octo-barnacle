//! The parent: the launcher that `client/lib`'s `Sidecar/` copies, and the harness for W0-09's cases.
//! `parent <case>` or `parent all` runs cases and prints `RESULT <case> PASS|FAIL <detail>`, exiting 0
//! only if all passed. `parent hold <dir>` launches and heartbeats until killed; `parent sleep` is a decoy.

use sidecar_watchdog_spike::*;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use std::{env, fs, io, process};

/// Spike only: how long the parent waits from spawn to READY.
const READY_LIMIT: Duration = Duration::from_secs(CONTROL_SILENCE_SECS);

type Case = fn(&Path) -> Result<String, String>;
const CASES: [(&str, Case); 8] = [
    ("spawn_ready", spawn_ready),
    ("shutdown", shutdown_exits_0),
    ("silent_control", silent_control),
    ("ignored_shutdown", ignored_shutdown),
    ("reused_pid", reused_pid),
    ("killed_parent", killed_parent),
    ("lock_live", lock_live),
    ("lock_dead", lock_dead),
];

/// A running sidecar as the client holds it: the process, its recorded PID and start time, the socket.
struct Sidecar {
    child: Child,
    pid: u32,
    start: u64,
    out: TcpStream,
    rx: Receiver<String>,
    ready_ms: u128,
}

enum Launch {
    Ready(Sidecar),
    Exited(i32),
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("hold") => hold(Path::new(args.get(1).expect("hold <dir>"))),
        Some("sleep") => thread::sleep(Duration::from_secs(120)),
        Some(name) => process::exit(run(name)),
        None => {
            let names: Vec<&str> = CASES.iter().map(|c| c.0).collect();
            eprintln!("usage: parent all|hold <dir>|sleep|<case>; cases: {names:?}");
            process::exit(64);
        }
    }
}

fn run(name: &str) -> i32 {
    let os = format!("{} {}", env::consts::OS, env::consts::ARCH);
    let q16 =
        format!("HEARTBEAT_SECS {HEARTBEAT_SECS}, CONTROL_SILENCE_SECS {CONTROL_SILENCE_SECS}");
    log(
        "harness",
        &format!("{os}; {q16}; shutdown grace {SHUTDOWN_GRACE_SECS} s"),
    );
    let (mut ran, mut failed) = (0, 0);
    for (case, f) in CASES {
        if name != "all" && name != case {
            continue;
        }
        ran += 1;
        let dir = env::temp_dir().join(format!("sidecar-spike-{case}-{}", process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("save dir");
        log(
            "harness",
            &format!("case {case}, save dir {}", dir.display()),
        );
        match f(&dir) {
            Ok(detail) => println!("RESULT {case} PASS {detail}"),
            Err(detail) => {
                failed += 1;
                println!("RESULT {case} FAIL {detail}");
            }
        }
    }
    if ran == 0 {
        eprintln!("unknown case {name}");
        return 64;
    }
    i32::from(failed > 0)
}

fn child_exe() -> PathBuf {
    let me = env::current_exe().expect("current_exe");
    me.with_file_name(format!("child{}", env::consts::EXE_SUFFIX))
}

fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}

/// Spawn and wait for READY. The parent binds 127.0.0.1:0 first, so the OS picks a free port that
/// nothing can take before the child connects, and only the child it spawned can reach it.
fn launch(dir: &Path, extra: &[&str]) -> Result<Launch, String> {
    let listener = TcpListener::bind("127.0.0.1:0").map_err(err)?;
    let control = listener.local_addr().map_err(err)?.to_string();
    let t0 = Instant::now();
    let mut child = Command::new(child_exe())
        .args(["--control", &control, "--save-dir"])
        .arg(dir)
        .args(extra)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .map_err(|e| format!("spawn: {e}"))?;
    let pid = child.id();
    // Recorded before Hello, in memory and in sidecar.pid; readable even if the child already exited.
    let start = os::Proc::open(pid)
        .and_then(|p| p.start_time())
        .ok_or("no start time")?;
    write_record(&dir.join("sidecar.pid"), pid, start).map_err(err)?;
    let msg = format!("spawned child pid {pid} --control {control}; sidecar.pid = {pid} {start}");
    log("parent", &msg);

    listener.set_nonblocking(true).map_err(err)?;
    let stream = loop {
        match listener.accept() {
            Ok((s, _)) => break s,
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => return Err(format!("accept: {e}")),
        }
        if let Some(st) = child.try_wait().map_err(err)? {
            let code = exit_code(st);
            log(
                "parent",
                &format!("child pid {pid} exited {code} before connecting"),
            );
            return Ok(Launch::Exited(code));
        }
        if t0.elapsed() > READY_LIMIT {
            let _ = child.kill();
            let _ = child.wait();
            return Err("no control connection".into());
        }
        thread::sleep(Duration::from_millis(10));
    };
    stream.set_nonblocking(false).map_err(err)?;
    let (tx, rx) = mpsc::channel();
    let reader = BufReader::new(stream.try_clone().map_err(err)?);
    thread::spawn(move || {
        for line in reader.lines() {
            let Ok(l) = line else { break };
            if tx.send(l).is_err() {
                break;
            }
        }
    });
    let mut out = stream;
    writeln!(out, "HELLO {}", process::id()).map_err(err)?;
    match rx.recv_timeout(READY_LIMIT) {
        Ok(l) if l == format!("READY {pid}") => {}
        other => {
            let _ = child.kill();
            let _ = child.wait();
            return Err(format!("expected READY {pid}, got {other:?}"));
        }
    }
    let ready_ms = t0.elapsed().as_millis();
    log(
        "parent",
        &format!("READY {pid} after {ready_ms} ms from spawn"),
    );
    Ok(Launch::Ready(Sidecar {
        child,
        pid,
        start,
        out,
        rx,
        ready_ms,
    }))
}

fn ready(launched: Result<Launch, String>) -> Result<Sidecar, String> {
    match launched? {
        Launch::Ready(sc) => Ok(sc),
        Launch::Exited(code) => Err(format!("child exited {code} before READY")),
    }
}

/// Heartbeats from their own thread every HEARTBEAT_SECS, so the caller's work cannot delay them.
struct Beats {
    stop: Sender<()>,
    thread: JoinHandle<()>,
}

fn heartbeats(out: &TcpStream) -> Beats {
    let mut out = out.try_clone().expect("clone control socket");
    let (stop, rx) = mpsc::channel();
    let thread = thread::spawn(move || {
        let every = Duration::from_secs(HEARTBEAT_SECS);
        while let Err(RecvTimeoutError::Timeout) = rx.recv_timeout(every) {
            if writeln!(out, "HEARTBEAT").is_err() {
                break;
            }
        }
    });
    Beats { stop, thread }
}

impl Beats {
    fn stop(self) {
        let _ = self.stop.send(());
        let _ = self.thread.join();
    }
}

/// Lines the child sent, until its socket closes (200 ms at most per line).
fn drain(rx: &Receiver<String>) -> Vec<String> {
    let mut said = Vec::new();
    while let Ok(l) = rx.recv_timeout(Duration::from_millis(200)) {
        said.push(l);
    }
    said
}

/// The clean exit, heartbeats already stopped: send SHUTDOWN, wait SHUTDOWN_GRACE_SECS for the
/// child to exit, then kill by the recorded PID and start time. Returns (path, exit code, lines).
fn shutdown(mut sc: Sidecar) -> (String, i32, Vec<String>) {
    let _ = writeln!(sc.out, "SHUTDOWN");
    log("parent", &format!("sent SHUTDOWN to pid {}", sc.pid));
    let t = Instant::now();
    let path = loop {
        if let Ok(Some(_)) = sc.child.try_wait() {
            break format!(
                "exited by itself {} ms after SHUTDOWN",
                t.elapsed().as_millis()
            );
        }
        if t.elapsed() >= Duration::from_secs(SHUTDOWN_GRACE_SECS) {
            let killed =
                kill_recorded(sc.pid, sc.start).unwrap_or_else(|e| format!("kill refused: {e}"));
            break format!("no exit {SHUTDOWN_GRACE_SECS} s after SHUTDOWN, {killed}");
        }
        thread::sleep(Duration::from_millis(20));
    };
    let code = sc.child.wait().map(exit_code).unwrap_or(-1);
    let said = drain(&sc.rx);
    log(
        "parent",
        &format!("pid {}: {path}; exit code {code}; said {said:?}", sc.pid),
    );
    (path, code, said)
}

fn save(dir: &Path) -> String {
    fs::read_to_string(dir.join("save.txt"))
        .unwrap_or_default()
        .trim()
        .to_string()
}

fn verdict(ok: bool, detail: String) -> Result<String, String> {
    if ok {
        Ok(detail)
    } else {
        Err(detail)
    }
}

/// `parent hold <dir>`: launch, heartbeat, print `ready <pid>` on stdout, and wait to be killed.
fn hold(dir: &Path) -> ! {
    let sc = match ready(launch(dir, &[])) {
        Ok(sc) => sc,
        Err(e) => {
            log("parent", &e);
            process::exit(1);
        }
    };
    let _beats = heartbeats(&sc.out);
    println!("ready {}", sc.pid);
    let _ = io::stdout().flush();
    log("parent", "holding with heartbeats until killed");
    loop {
        thread::sleep(Duration::from_secs(60));
    }
}

/// Spawn, READY over localhost TCP, then heartbeats keep the child alive past CONTROL_SILENCE_SECS.
fn spawn_ready(dir: &Path) -> Result<String, String> {
    let mut sc = ready(launch(dir, &[]))?;
    let beats = heartbeats(&sc.out);
    let hold = CONTROL_SILENCE_SECS + 2;
    thread::sleep(Duration::from_secs(hold));
    let alive = matches!(sc.child.try_wait(), Ok(None));
    beats.stop();
    let (pid, ms) = (sc.pid, sc.ready_ms);
    let (_, code, _) = shutdown(sc);
    let detail = format!("pid {pid} READY {ms} ms after spawn; alive after {hold} s of heartbeats: {alive}; then exit {code}");
    verdict(alive, detail)
}

/// SHUTDOWN: the child saves, sends EXITING host_exit, releases the lock, and exits 0 unkilled.
fn shutdown_exits_0(dir: &Path) -> Result<String, String> {
    let sc = ready(launch(dir, &[]))?;
    let beats = heartbeats(&sc.out);
    thread::sleep(Duration::from_secs(HEARTBEAT_SECS + 1));
    beats.stop();
    let (path, code, said) = shutdown(sc);
    let lock_gone = !dir.join("server.lock").exists();
    let ok = code == 0
        && path.starts_with("exited by itself")
        && said.iter().any(|l| l == "EXITING host_exit")
        && save(dir).contains("reason=host_exit")
        && lock_gone;
    let detail = format!(
        "exit {code}, {path}, said {said:?}, save `{}`, lock released: {lock_gone}",
        save(dir)
    );
    verdict(ok, detail)
}

/// A silent control socket (left open, nothing sent): exit 2 within CONTROL_SILENCE_SECS + 1.
fn silent_control(dir: &Path) -> Result<String, String> {
    let mut sc = ready(launch(dir, &[]))?;
    for _ in 0..2 {
        thread::sleep(Duration::from_secs(HEARTBEAT_SECS));
        writeln!(sc.out, "HEARTBEAT").map_err(err)?;
    }
    let last = Instant::now();
    log("parent", "sent 2 heartbeats; now silent, socket left open");
    let limit = Duration::from_secs(CONTROL_SILENCE_SECS + 1);
    let code = loop {
        if let Some(st) = sc.child.try_wait().map_err(err)? {
            break exit_code(st);
        }
        if last.elapsed() > limit + Duration::from_secs(4) {
            let _ = sc.child.kill();
            return Err(format!(
                "still running {:.2} s into silence",
                last.elapsed().as_secs_f64()
            ));
        }
        thread::sleep(Duration::from_millis(10));
    };
    let silent = last.elapsed();
    let said = drain(&sc.rx);
    let ok = code == 2
        && silent >= Duration::from_secs(CONTROL_SILENCE_SECS)
        && silent <= limit
        && said.iter().any(|l| l == "EXITING watchdog")
        && save(dir).contains("reason=watchdog");
    let s = silent.as_secs_f64();
    let detail = format!(
        "exit {code} after {s:.2} s of silence (limit {} s), said {said:?}, save `{}`",
        limit.as_secs(),
        save(dir)
    );
    verdict(ok, detail)
}

/// A child that ignores SHUTDOWN (and its own watchdog) is killed after the grace by recorded PID and start time.
fn ignored_shutdown(dir: &Path) -> Result<String, String> {
    let sc = ready(launch(dir, &["--hang-on-shutdown"]))?;
    let (pid, start) = (sc.pid, sc.start);
    let (path, code, _) = shutdown(sc);
    let gone = running_start_time(pid) != Some(start);
    let detail = format!("{path}; exit code {code}; pid {pid} gone: {gone}");
    verdict(path.contains("killed pid") && gone, detail)
}

/// A sidecar.pid whose PID now names another running process (its start time differs) is never killed.
fn reused_pid(dir: &Path) -> Result<String, String> {
    let me = env::current_exe().map_err(err)?;
    let mut decoy = Command::new(me).arg("sleep").spawn().map_err(err)?;
    let pid = decoy.id();
    let real = running_start_time(pid).ok_or("decoy start time")?;
    // What a dead sidecar left behind, if the OS then gave its PID to the decoy: an earlier start.
    write_record(&dir.join("sidecar.pid"), pid, real - 1).map_err(err)?;
    let (rpid, rstart) = read_record(&dir.join("sidecar.pid")).ok_or("read sidecar.pid")?;
    let r = kill_recorded(rpid, rstart);
    log(
        "parent",
        &format!("kill by sidecar.pid ({rpid} {rstart}): {r:?}"),
    );
    thread::sleep(Duration::from_millis(500));
    let alive = matches!(decoy.try_wait(), Ok(None));
    let _ = decoy.kill();
    let _ = decoy.wait();
    let detail = format!(
        "record {rpid} {rstart}, running pid {pid} started {real}: {r:?}; still alive: {alive}"
    );
    verdict(r.is_err() && alive, detail)
}

/// The parent is killed hard; its orphaned child sees the control socket close and exits 2 by watchdog.
fn killed_parent(dir: &Path) -> Result<String, String> {
    os::become_subreaper();
    let me = env::current_exe().map_err(err)?;
    let mut parent = Command::new(me)
        .arg("hold")
        .arg(dir)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(err)?;
    let mut line = String::new();
    let stdout = parent.stdout.take().ok_or("no stdout")?;
    BufReader::new(stdout).read_line(&mut line).map_err(err)?;
    let (pid, start) = read_record(&dir.join("sidecar.pid")).ok_or("no sidecar.pid")?;
    let child = os::Proc::open(pid);
    if !line.starts_with("ready") || child.is_none() {
        let _ = parent.kill();
        return Err(format!(
            "hold parent said {line:?}; child open: {}",
            child.is_some()
        ));
    }
    let t = Instant::now();
    parent.kill().map_err(err)?;
    let pcode = parent.wait().map(exit_code).unwrap_or(-1);
    let ppid = parent.id();
    log(
        "harness",
        &format!("killed parent pid {ppid} (exit {pcode}); waiting on orphan pid {pid}"),
    );
    let limit = Duration::from_secs(CONTROL_SILENCE_SECS + 1);
    let code = child.and_then(|c| c.wait_exit(limit + Duration::from_secs(4)));
    let after = t.elapsed().as_secs_f64();
    let Some(code) = code else {
        let _ = kill_recorded(pid, start);
        return Err(format!(
            "orphan pid {pid} not seen to exit in 15 s; save `{}`",
            save(dir)
        ));
    };
    let ok = code == 2 && after <= limit.as_secs_f64() && save(dir).contains("reason=watchdog");
    let detail = format!("orphan pid {pid} exit {code} {after:.2} s after its parent was killed (limit {} s), save `{}`", limit.as_secs(), save(dir));
    verdict(ok, detail)
}

/// A second server on a save dir whose lock a live server holds exits 3 and leaves the lock alone.
fn lock_live(dir: &Path) -> Result<String, String> {
    let a = ready(launch(dir, &[]))?;
    let b = match launch(dir, &[])? {
        Launch::Exited(code) => Some(code),
        Launch::Ready(sc) => {
            shutdown(sc);
            None
        }
    };
    let lock = read_record(&dir.join("server.lock"));
    let kept = lock == Some((a.pid, a.start));
    let a_pid = a.pid;
    let (_, a_code, _) = shutdown(a);
    let detail = format!("second child exit {b:?} while pid {a_pid} ran; lock still {lock:?}: {kept}; first child then exit {a_code}");
    verdict(b == Some(3) && kept && a_code == 0, detail)
}

/// A lock left by a killed server is cleared on restart; so is one whose PID names another process.
fn lock_dead(dir: &Path) -> Result<String, String> {
    let lock = dir.join("server.lock");
    let mut a = ready(launch(dir, &[]))?;
    let killed = kill_recorded(a.pid, a.start)?;
    let a_code = a.child.wait().map(exit_code).unwrap_or(-1);
    log(
        "parent",
        &format!("{killed}, exit {a_code}: its server.lock stays behind"),
    );
    let left = read_record(&lock);
    let dead_left = left == Some((a.pid, a.start));
    let b = ready(launch(dir, &[]))?;
    let (_, b_code, _) = shutdown(b);
    // A lock naming a running process (this harness) with another start time: a reused PID.
    let me = process::id();
    let fake = running_start_time(me).ok_or("own start time")? - 1;
    write_record(&lock, me, fake).map_err(err)?;
    let c = ready(launch(dir, &[]))?;
    let (_, c_code, _) = shutdown(c);
    let detail = format!("{killed} (exit {a_code}), lock left {left:?}; restart READY then exit {b_code}; lock ({me} {fake}) naming a running pid: READY then exit {c_code}");
    verdict(dead_left && b_code == 0 && c_code == 0, detail)
}
