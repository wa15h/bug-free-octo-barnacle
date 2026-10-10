//! Shared by the parent (the client's launcher) and the child (the server). std only.
//! Messages are newline-terminated text here; the real ones are `crates/proto` types (D7, rule 3).

use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::ExitStatus;
use std::time::{SystemTime, UNIX_EPOCH};

/// Q16 defaults (D21).
pub const HEARTBEAT_SECS: u64 = 2;
pub const CONTROL_SILENCE_SECS: u64 = 10;
/// The parent's wait after `Shutdown` before it kills. The parent stops heartbeats when it sends
/// `Shutdown`, so a healthy child exits by its own watchdog before this; only a hung one is killed.
pub const SHUTDOWN_GRACE_SECS: u64 = CONTROL_SILENCE_SECS + 1;

/// One log line on stderr, stamped with UTC wall time so parent and child lines interleave in order.
pub fn log(who: &str, msg: &str) {
    let ms = unix_ms();
    let s = ms / 1000;
    let line = format!(
        "{:02}:{:02}:{:02}.{:03} [{who}] {msg}\n",
        (s / 3600) % 24,
        (s / 60) % 60,
        s % 60,
        ms % 1000
    );
    // One write per line, so lines from the parent and the child never split each other.
    let _ = io::stderr().write_all(line.as_bytes());
}

/// Wall time in Unix milliseconds: log stamps, and the shared instant `lock_race` starts servers at.
pub fn unix_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}

/// A process record, `<pid> <start time>`: the client's `sidecar.pid` and the server's `server.lock`.
pub fn write_record(path: &Path, pid: u32, start: u64) -> io::Result<()> {
    fs::write(path, format!("{pid} {start}\n"))
}

pub fn read_record(path: &Path) -> Option<(u32, u64)> {
    let text = fs::read_to_string(path).ok()?;
    let mut parts = text.split_whitespace();
    Some((parts.next()?.parse().ok()?, parts.next()?.parse().ok()?))
}

/// The start time of `pid` if it is running now; None if it has exited or never existed.
pub fn running_start_time(pid: u32) -> Option<u64> {
    let p = os::Proc::open(pid)?;
    if p.alive() {
        p.start_time()
    } else {
        None
    }
}

/// Kill `pid` only if it is running and still has the recorded start time. A PID the OS has given
/// to a newer process has a different start time and is never killed.
pub fn kill_recorded(pid: u32, start: u64) -> Result<String, String> {
    let p = os::Proc::open(pid).ok_or("not running")?;
    if !p.alive() {
        return Err("not running".into());
    }
    match p.start_time() {
        Some(s) if s == start => {
            if p.kill() {
                Ok(format!("killed pid {pid} (start time {s} matched)"))
            } else {
                Err(format!("kill of pid {pid} failed"))
            }
        }
        Some(s) => Err(format!(
            "pid {pid} start time {s} != recorded {start}: a reused PID, not killed"
        )),
        None => Err("no start time".into()),
    }
}

/// An exit status as one number: the exit code, or 128 + signal on Unix (the shell's convention).
pub fn exit_code(status: ExitStatus) -> i32 {
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        if let Some(sig) = status.signal() {
            return 128 + sig;
        }
    }
    status.code().unwrap_or(-1)
}

#[cfg(target_os = "linux")]
pub mod os {
    use std::os::raw::{c_int, c_ulong};
    use std::thread;
    use std::time::{Duration, Instant};

    extern "C" {
        fn kill(pid: c_int, sig: c_int) -> c_int;
        fn prctl(option: c_int, ...) -> c_int;
        fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    }

    pub struct Proc {
        pid: u32,
    }

    impl Proc {
        fn stat(&self) -> Option<Vec<String>> {
            let text = std::fs::read_to_string(format!("/proc/{}/stat", self.pid)).ok()?;
            // Field 2 is the name in parentheses and may hold spaces; split after the last ')'.
            let rest = &text[text.rfind(')')? + 1..];
            Some(rest.split_whitespace().map(String::from).collect())
        }

        pub fn open(pid: u32) -> Option<Proc> {
            let p = Proc { pid };
            p.stat().map(|_| p)
        }

        /// Field 3, the state: Z (zombie) and X (dead) have exited.
        pub fn alive(&self) -> bool {
            self.stat()
                .is_some_and(|f| f.first().is_some_and(|s| s != "Z" && s != "X"))
        }

        /// Field 22: clock ticks from boot to the process start. Readable until the PID is reaped.
        pub fn start_time(&self) -> Option<u64> {
            self.stat()?.get(19)?.parse().ok()
        }

        /// SIGKILL by PID. The start time is read just before; the gap is the only reuse window.
        pub fn kill(&self) -> bool {
            unsafe { kill(self.pid as c_int, 9) == 0 }
        }

        /// The exit code (128 + signal if killed); works for a child, or an orphan once we are its subreaper.
        pub fn wait_exit(&self, limit: Duration) -> Option<i32> {
            let t = Instant::now();
            while t.elapsed() < limit {
                let mut st: c_int = 0;
                let r = unsafe { waitpid(self.pid as c_int, &mut st, 1) }; // 1 = WNOHANG
                if r == self.pid as c_int {
                    return Some(if st & 0x7f == 0 {
                        (st >> 8) & 0xff
                    } else {
                        128 + (st & 0x7f)
                    });
                }
                thread::sleep(Duration::from_millis(20));
            }
            None
        }
    }

    /// Orphaned descendants are re-parented to this process, so it can read their exit codes.
    pub fn become_subreaper() {
        unsafe {
            prctl(36, 1 as c_ulong); // 36 = PR_SET_CHILD_SUBREAPER
        }
    }
}

#[cfg(windows)]
pub mod os {
    use std::time::Duration;

    type Handle = isize;
    const PROCESS_TERMINATE: u32 = 0x0001;
    const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 0x1000;
    const SYNCHRONIZE: u32 = 0x0010_0000;
    const WAIT_OBJECT_0: u32 = 0;
    const WAIT_TIMEOUT: u32 = 0x102;

    #[link(name = "kernel32")]
    extern "system" {
        fn OpenProcess(access: u32, inherit: i32, pid: u32) -> Handle;
        fn GetProcessTimes(h: Handle, c: *mut u64, e: *mut u64, k: *mut u64, u: *mut u64) -> i32;
        fn WaitForSingleObject(h: Handle, ms: u32) -> u32;
        fn GetExitCodeProcess(h: Handle, code: *mut u32) -> i32;
        fn TerminateProcess(h: Handle, code: u32) -> i32;
        fn CloseHandle(h: Handle) -> i32;
    }

    /// An open process handle. While it is held the OS cannot give the PID to another process, so
    /// the start-time check and the kill act on the same process.
    pub struct Proc {
        h: Handle,
    }

    impl Proc {
        pub fn open(pid: u32) -> Option<Proc> {
            let access = PROCESS_QUERY_LIMITED_INFORMATION | SYNCHRONIZE | PROCESS_TERMINATE;
            let h = unsafe { OpenProcess(access, 0, pid) };
            if h == 0 {
                None
            } else {
                Some(Proc { h })
            }
        }

        pub fn alive(&self) -> bool {
            unsafe { WaitForSingleObject(self.h, 0) == WAIT_TIMEOUT }
        }

        /// The creation time, in 100 ns units since 1601; readable after exit while a handle is open.
        pub fn start_time(&self) -> Option<u64> {
            let (mut c, mut e, mut k, mut u) = (0u64, 0u64, 0u64, 0u64);
            let ok = unsafe { GetProcessTimes(self.h, &mut c, &mut e, &mut k, &mut u) };
            if ok == 0 {
                None
            } else {
                Some(c)
            }
        }

        pub fn kill(&self) -> bool {
            unsafe { TerminateProcess(self.h, 1) != 0 }
        }

        pub fn wait_exit(&self, limit: Duration) -> Option<i32> {
            if unsafe { WaitForSingleObject(self.h, limit.as_millis() as u32) } != WAIT_OBJECT_0 {
                return None;
            }
            let mut code = 0u32;
            if unsafe { GetExitCodeProcess(self.h, &mut code) } == 0 {
                return None;
            }
            Some(code as i32)
        }
    }

    impl Drop for Proc {
        fn drop(&mut self) {
            unsafe {
                CloseHandle(self.h);
            }
        }
    }

    /// Not needed on Windows: an open handle keeps any process's exit code readable.
    pub fn become_subreaper() {}
}
