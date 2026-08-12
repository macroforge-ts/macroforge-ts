//! Per-project advisory lock, serializing every macroforge process that
//! mutates a project's `.macroforge/` state or output tree.
//!
//! Without it, `macroforge watch`, `svelte-package`, `tsc`, `svelte-check`,
//! `cache` and `refresh` all write the same `.macroforge/type-registry.json`,
//! `.macroforge/cache/manifest.json` and staging trees with no coordination —
//! two of them running at once interleave their writes.
//!
//! The primitive is `std::fs::File::{try_lock, lock}` (stable since 1.89),
//! which is `flock` on unix and `LockFileEx` on Windows. This is the same call
//! cargo makes for its own package-cache and build-directory locks. An advisory
//! whole-file lock is used rather than a create-and-delete "dot lock" because
//! the kernel releases it when the holder exits, however it exits — there is no
//! stale lock to detect and reclaim after a crash or a `kill -9`.
//!
//! ## Granularity
//!
//! Short-lived subcommands take the lock once at dispatch and hold it until
//! they exit. `watch` is a daemon: it takes the lock around each cache-mutation
//! burst and releases it while idle, so `macroforge svelte-package` in another
//! terminal is delayed by one burst rather than blocked until the watcher is
//! killed. That is the same effective granularity cargo-watch gets for free by
//! re-invoking a fresh `cargo` per change.

use anyhow::{Context, Result};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions, TryLockError},
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    sync::{Arc, LazyLock, Mutex},
};

/// Lock files, one per project root, keyed on the canonical root path.
///
/// Entries are created on first use and kept for the life of the process.
/// There is at most one per project a single invocation touches, so the map
/// never grows beyond a handful.
static LOCKS: LazyLock<Mutex<HashMap<PathBuf, Arc<LockFile>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// The lock file for one project root.
struct LockFile {
    path: PathBuf,
    root: PathBuf,
    state: Mutex<LockState>,
}

/// Whether this process currently holds the OS lock, and for how many guards.
///
/// A second `flock` on a *different* descriptor deadlocks against the first,
/// even within one process, so nested acquisitions are refcounted rather than
/// re-locked — the same approach cargo takes in its `CacheLocker`.
struct LockState {
    /// The descriptor holding the OS lock. `None` when unheld.
    file: Option<File>,
    /// Live guard count. The OS lock is released when this reaches zero.
    holders: usize,
    /// Set once this filesystem has been found unable to lock, so the failure
    /// is reported once rather than on every acquisition. `watch` acquires per
    /// change, which would otherwise repeat the warning on every keystroke.
    unsupported: bool,
}

/// A held project lock. The OS lock is released when the last guard is dropped.
pub(crate) struct ProjectLock {
    lock: Arc<LockFile>,
}

impl ProjectLock {
    /// Acquires the lock for `root`, blocking until it is available.
    ///
    /// Tries once without blocking; on contention it reports who holds the lock
    /// and then waits indefinitely, matching cargo's `Blocking waiting for file
    /// lock on …` behavior. `command` names the subcommand and is recorded in
    /// the lock file so a waiting process can say what it is waiting for.
    ///
    /// `quiet` suppresses that reporting. `expand --quiet` guarantees an empty
    /// stderr so callers can parse it, and a lock this process happened to
    /// contend on is not the caller's business.
    pub(crate) fn acquire(root: &Path, command: &str, quiet: bool) -> Result<ProjectLock> {
        let lock = lock_file_for(root)?;

        // One mutex per root, not a global one: a process blocked waiting on
        // one project must not stall acquisition for another.
        //
        // It is held across the blocking wait below, which is deliberate. Two
        // threads racing for the same root would otherwise both reach the
        // syscall, and the second `flock` — on its own descriptor — would
        // block against the first forever. Serializing them here means the
        // loser wakes up to `holders > 0` and refcounts instead.
        let mut state = lock
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        // Already held by this process: refcount instead of deadlocking on a
        // second descriptor. Likewise once the filesystem has been found unable
        // to lock — retrying would fail identically every time.
        if state.holders > 0 || state.unsupported {
            state.holders += 1;
            drop(state);
            return Ok(ProjectLock { lock });
        }

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&lock.path)
            .with_context(|| format!("failed to open the lock file {}", lock.path.display()))?;

        match file.try_lock() {
            Ok(()) => {}
            Err(TryLockError::WouldBlock) => {
                if !quiet {
                    eprintln!(
                        "[macroforge] waiting for the project lock on {} (held by {})",
                        lock.root.display(),
                        describe_holder(&lock.path)
                    );
                }
                file.lock()
                    .with_context(|| format!("failed to lock {}", lock.path.display()))?;
            }
            // The filesystem cannot lock. Some network and virtual filesystems
            // report this as `Unsupported`; others surface an errno std has no
            // `ErrorKind` for (macOS `ENOTSUP`, `ENOLCK` on an NFS mount with
            // no lock daemon), and those codes are not portable across Linux
            // architectures, so they cannot be matched by value.
            //
            // Proceed unlocked either way. This lock guards against *other*
            // macroforge processes; refusing to run at all because the
            // filesystem has no locking would turn a coordination feature into
            // a hard build failure. The unsupported case is expected and stays
            // quiet; anything else is surfaced so it is not mistaken for
            // working coordination. Cargo instead probes for NFS explicitly and
            // errors on the rest, which leaves the same gap on macOS.
            Err(TryLockError::Error(e)) => {
                if !quiet && e.kind() != std::io::ErrorKind::Unsupported {
                    eprintln!(
                        "[macroforge] warning: cannot lock {} ({e}); \
                         continuing without a project lock — concurrent \
                         macroforge processes will not be serialized",
                        lock.path.display()
                    );
                }
                state.unsupported = true;
                state.holders += 1;
                drop(state);
                return Ok(ProjectLock { lock });
            }
        }

        record_holder(&file, command);
        state.file = Some(file);
        state.holders += 1;
        drop(state);

        Ok(ProjectLock { lock })
    }
}

impl Drop for ProjectLock {
    fn drop(&mut self) {
        let mut state = self
            .lock
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        state.holders -= 1;
        if state.holders == 0 {
            // Closing the descriptor releases the OS lock; unlocking first
            // makes the release explicit and reports a failure that would
            // otherwise be silent.
            if let Some(file) = state.file.take()
                && let Err(e) = file.unlock()
            {
                eprintln!(
                    "[macroforge] warning: failed to release {}: {e}",
                    self.lock.path.display()
                );
            }
        }
    }
}

/// Returns the shared lock-file handle for `root`, creating `.macroforge/` if
/// this is the first command to touch the project.
fn lock_file_for(root: &Path) -> Result<Arc<LockFile>> {
    let dir = root.join(".macroforge");
    std::fs::create_dir_all(&dir).with_context(|| format!("failed to create {}", dir.display()))?;
    let path = dir.join(".lock");

    let mut locks = LOCKS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    Ok(locks
        .entry(path.clone())
        .or_insert_with(|| {
            Arc::new(LockFile {
                path,
                root: root.to_path_buf(),
                state: Mutex::new(LockState {
                    file: None,
                    holders: 0,
                    unsupported: false,
                }),
            })
        })
        .clone())
}

/// Stamps the lock file with who holds it, so a waiting process can name it.
///
/// This has to be written to the locked file itself — the lock lives on that
/// inode, so it cannot be replaced by a rename — which means a reader can
/// catch it mid-write. That is tolerable because the record is only ever read
/// to build a human-readable message, and [`describe_holder`] falls back when
/// it cannot parse. Failures are ignored for the same reason: an unlabelled
/// lock still locks.
fn record_holder(file: &File, command: &str) {
    let started = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let record = serde_json::json!({
        "pid": std::process::id(),
        "command": command,
        "startedAt": started,
    });

    let Ok(json) = serde_json::to_string(&record) else {
        return;
    };

    let mut file = file;
    let _ = file.set_len(0);
    let _ = file.seek(SeekFrom::Start(0));
    let _ = file.write_all(json.as_bytes());
    let _ = file.flush();
}

/// Describes the current lock holder for the waiting message.
///
/// Best-effort: the record may be absent (a holder that crashed before writing
/// it, or an older macroforge) or caught mid-write, so anything unparseable
/// degrades to a generic phrase rather than failing the wait.
fn describe_holder(path: &Path) -> String {
    let mut contents = String::new();
    let parsed = File::open(path)
        .and_then(|mut f| f.read_to_string(&mut contents))
        .ok()
        .and_then(|_| serde_json::from_str::<serde_json::Value>(&contents).ok());

    let Some(record) = parsed else {
        return "another macroforge process".to_string();
    };

    match (record["pid"].as_u64(), record["command"].as_str()) {
        (Some(pid), Some(command)) => format!("pid {pid}, `macroforge {command}`"),
        (Some(pid), None) => format!("pid {pid}"),
        _ => "another macroforge process".to_string(),
    }
}

/// Resolves the directory that owns a project's `.macroforge/` state.
///
/// This is the lock key, and it is also where the type registry, the
/// declarative registry and the expansion cache are written, so every
/// subcommand must agree on it. Commands that take an explicit root
/// (`cache`, `refresh`, `watch`) use it; the rest operate on the current
/// directory.
///
/// The path is canonicalized so two invocations naming the same project
/// through different paths — a symlink, a relative path, `.` — resolve to one
/// lock. A root that does not exist yet is returned as given: locking it will
/// create it.
pub(crate) fn resolve_project_root(explicit: Option<&Path>) -> PathBuf {
    let root = explicit
        .map(Path::to_path_buf)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    root.canonicalize().unwrap_or(root)
}
