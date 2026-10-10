//! The stub server. Spawned as `child --control 127.0.0.1:<port> --save-dir <dir> [--hang-on-shutdown]`
//! (`lock_race` adds `--take-lock-at <unix ms>`).
//! Order: take the save-dir lock (exit 3 if a live server holds it), bind the control port the client
//! chose, accept one connection and stop listening, answer `HELLO` with `READY <pid>`, then tick.
//! Every exit path saves between ticks, sends `EXITING <reason>`, and releases the lock: `SHUTDOWN`
//! exits 0; a closed control socket, one silent for `CONTROL_SILENCE_SECS`, or no connection within
//! it, exits 2. `--hang-on-shutdown` plays a hung server that the parent must kill.

use sidecar_watchdog_spike::{log, read_record, running_start_time, unix_ms, CONTROL_SILENCE_SECS};
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, TryRecvError};
use std::time::{Duration, Instant};
use std::{env, fs, process, thread};

const TICK: Duration = Duration::from_millis(100);

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = |name: &str| {
        let i = args.iter().position(|a| a == name)?;
        args.get(i + 1).cloned()
    };
    let control = arg("--control").expect("--control 127.0.0.1:<port>");
    let dir = PathBuf::from(arg("--save-dir").expect("--save-dir <dir>"));
    let hang = args.iter().any(|a| a == "--hang-on-shutdown");
    let who = format!("child {}", process::id());
    log(
        &who,
        &format!("started: --control {control} --save-dir {}", dir.display()),
    );

    // lock_race only: wait for an instant shared with a second server, so both take the lock at once.
    if let Some(at) = arg("--take-lock-at").and_then(|a| a.parse::<u128>().ok()) {
        while unix_ms() < at {
            std::hint::spin_loop();
        }
    }
    if let Err(holder) = take_lock(&dir, &who) {
        log(
            &who,
            &format!("save dir locked by live pid {holder}: exit 3"),
        );
        process::exit(3);
    }
    // Bound only once the lock is held, so a server that exits 3 never listens.
    let listener = match TcpListener::bind(&control) {
        Ok(l) => l,
        Err(e) => {
            log(&who, &format!("control bind {control} failed: {e}"));
            exit_path(&who, &dir, None, 0, 2, "watchdog", "control port unusable");
        }
    };
    log(&who, &format!("listening on {control}"));
    let Some(mut out) = accept_one(listener) else {
        let why = "no control connection";
        exit_path(&who, &dir, None, 0, 2, "watchdog", why);
    };
    log(&who, "accepted the control connection; no longer listening");

    // The socket task forwards each line; None means the socket closed or failed.
    let (tx, rx) = mpsc::channel::<Option<String>>();
    let reader = BufReader::new(out.try_clone().expect("clone control socket"));
    thread::spawn(move || {
        for line in reader.lines() {
            let Ok(l) = line else { break };
            if tx.send(Some(l)).is_err() {
                return;
            }
        }
        let _ = tx.send(None);
    });

    // The tick task: drain the control socket, check the watchdog, run one tick.
    let mut tick: u64 = 0;
    let mut heard = Instant::now();
    loop {
        loop {
            match rx.try_recv() {
                Ok(Some(msg)) => {
                    heard = Instant::now();
                    if msg.starts_with("HELLO") {
                        log(&who, &format!("got {msg}; sending READY {}", process::id()));
                        let _ = writeln!(out, "READY {}", process::id());
                    } else if msg == "SHUTDOWN" && hang {
                        log(
                            &who,
                            "got SHUTDOWN; --hang-on-shutdown: ignoring it and the watchdog",
                        );
                        loop {
                            thread::sleep(Duration::from_secs(60));
                        }
                    } else if msg == "SHUTDOWN" {
                        exit_path(
                            &who,
                            &dir,
                            Some(&mut out),
                            tick,
                            0,
                            "host_exit",
                            "got SHUTDOWN",
                        );
                    }
                }
                Ok(None) | Err(TryRecvError::Disconnected) => {
                    let why = "control socket closed";
                    exit_path(&who, &dir, Some(&mut out), tick, 2, "watchdog", why);
                }
                Err(TryRecvError::Empty) => break,
            }
        }
        let silent = heard.elapsed();
        if silent >= Duration::from_secs(CONTROL_SILENCE_SECS) {
            let why = format!("control socket silent {:.2} s", silent.as_secs_f64());
            exit_path(&who, &dir, Some(&mut out), tick, 2, "watchdog", &why);
        }
        tick += 1; // the simulation tick would run here
        thread::sleep(TICK);
    }
}

/// Accept one connection, then drop the listener, so only the first connection can send `SHUTDOWN`.
/// No connection within `CONTROL_SILENCE_SECS` counts as a silent control socket.
fn accept_one(listener: TcpListener) -> Option<TcpStream> {
    listener.set_nonblocking(true).ok()?;
    let t = Instant::now();
    while t.elapsed() < Duration::from_secs(CONTROL_SILENCE_SECS) {
        match listener.accept() {
            Ok((s, _)) => {
                s.set_nonblocking(false).ok()?;
                return Some(s);
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(10))
            }
            Err(_) => return None,
        }
    }
    None
}

/// Create `server.lock` holding our PID and start time. A lock whose PID is running with the
/// recorded start time belongs to a live server: return its PID. Otherwise it is stale (the holder
/// died, or its PID now names another process): clear it and take it.
fn take_lock(dir: &Path, who: &str) -> Result<(), u32> {
    let path = dir.join("server.lock");
    let me = process::id();
    let start = running_start_time(me).expect("own start time");
    for _ in 0..2 {
        match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(mut f) => {
                writeln!(f, "{me} {start}").expect("write lock");
                log(who, &format!("took save-dir lock ({me} {start})"));
                return Ok(());
            }
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => match read_record(&path) {
                Some((pid, s)) if running_start_time(pid) == Some(s) => return Err(pid),
                held => {
                    let now = held.and_then(|(pid, _)| running_start_time(pid));
                    let msg = format!(
                        "stale lock {held:?} (that pid now: {}): cleared",
                        now.map_or("not running".into(), |s| format!("running, start {s}"))
                    );
                    log(who, &msg);
                    let _ = fs::remove_file(&path);
                }
            },
            Err(e) => panic!("lock {}: {e}", path.display()),
        }
    }
    Err(0)
}

/// Every exit path: the save (a stand-in record, written between ticks), `EXITING`, the lock, the code.
fn exit_path(
    who: &str,
    dir: &Path,
    out: Option<&mut TcpStream>,
    tick: u64,
    code: i32,
    reason: &str,
    why: &str,
) -> ! {
    let tmp = dir.join("save.tmp");
    fs::write(&tmp, format!("tick={tick} reason={reason} exit={code}\n")).expect("write save");
    fs::rename(&tmp, dir.join("save.txt")).expect("rename save");
    if let Some(out) = out {
        let _ = writeln!(out, "EXITING {reason}");
    }
    let _ = fs::remove_file(dir.join("server.lock"));
    log(
        who,
        &format!("{why}: saved at tick {tick}, sent EXITING {reason}, exit {code}"),
    );
    process::exit(code);
}
