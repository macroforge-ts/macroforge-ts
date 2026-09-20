//! Integration tests for the macroforge CLI.

use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

fn macroforge_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_macroforge"))
}

/// Builds a `src/` tree used by the scan-mode tests:
/// - `src/withmacro.ts`      — a `@derive(Default)` interface (expands)
/// - `src/plain.ts`          — no macros (copied verbatim into a mirrored tree)
/// - `src/nested/style.css`  — a non-TS asset (copied verbatim)
/// - `src/Comp.svelte`       — a component file (copied verbatim, not expanded)
/// - `src/withmacro.expanded.ts` — a pre-existing debug sibling (never scanned)
fn setup_scan_fixture(temp: &Path) {
    // A package.json marks the project root for config/registry resolution.
    std::fs::write(temp.join("package.json"), "{ \"name\": \"scan-fixture\" }").unwrap();

    let src = temp.join("src");
    std::fs::create_dir_all(src.join("nested")).unwrap();
    std::fs::write(
        src.join("withmacro.ts"),
        "/** @derive(Default, Serialize, Deserialize) */\nexport interface Foo {\n  name: string;\n}\n",
    )
    .unwrap();
    std::fs::write(
        src.join("plain.ts"),
        "export interface Plain {\n  x: number;\n}\n",
    )
    .unwrap();
    std::fs::write(
        src.join("nested").join("style.css"),
        "body { margin: 0; }\n",
    )
    .unwrap();
    std::fs::write(
        src.join("Comp.svelte"),
        "<script lang=\"ts\">let x = 1;</script>\n",
    )
    .unwrap();
    // A stale sibling from a previous --emit-expanded run: must be ignored.
    std::fs::write(
        src.join("withmacro.expanded.ts"),
        "// stale artifact — should never be scanned or mirrored\n",
    )
    .unwrap();
}

/// Recursively collects file paths (relative to `dir`) under `dir`.
fn list_files(dir: &Path) -> Vec<String> {
    let mut out = Vec::new();
    fn walk(base: &Path, dir: &Path, out: &mut Vec<String>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(base, &path, out);
            } else if let Ok(rel) = path.strip_prefix(base) {
                out.push(rel.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    walk(dir, dir, &mut out);
    out.sort();
    out
}

#[test]
fn expand_file_without_macros_exits_with_code_2() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("no-macros.ts");

    // A TypeScript file with no @derive decorators
    std::fs::write(
        &input_path,
        r#"
export class User {
    name: string;
    age: number;
}
"#,
    )
    .unwrap();

    // Test without --quiet: should print message to stderr
    let output = macroforge_bin()
        .arg("expand")
        .arg(&input_path)
        .current_dir(temp_dir.path())
        .output()
        .expect("failed to run macroforge");

    assert_eq!(
        output.status.code(),
        Some(2),
        "should exit with code 2 when no macros found"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("no macros found"),
        "stderr should contain 'no macros found', got: {}",
        stderr
    );

    // Test with --quiet: should exit silently
    let quiet_output = macroforge_bin()
        .arg("expand")
        .arg(&input_path)
        .arg("--quiet")
        .current_dir(temp_dir.path())
        .output()
        .expect("failed to run macroforge");

    assert_eq!(
        quiet_output.status.code(),
        Some(2),
        "should exit with code 2 when no macros found (quiet mode)"
    );

    let quiet_stderr = String::from_utf8_lossy(&quiet_output.stderr);
    assert!(
        quiet_stderr.is_empty(),
        "stderr should be empty in quiet mode, got: {}",
        quiet_stderr
    );
}

#[test]
fn expand_file_with_macros_exits_with_code_0() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("with-macros.ts");

    // A TypeScript file with a @derive decorator
    std::fs::write(
        &input_path,
        r#"
import { Derive } from "@macro/derive";

/** @derive(Debug) */
class User {
    name: string;
}
"#,
    )
    .unwrap();

    let output = macroforge_bin()
        .arg("expand")
        .arg(&input_path)
        .current_dir(temp_dir.path())
        .output()
        .expect("failed to run macroforge");

    assert_eq!(
        output.status.code(),
        Some(0),
        "should exit with code 0 when macros are expanded. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

// ============================================================================
// Scan output routing
// ============================================================================

#[test]
fn scan_with_out_dir_mirrors_expanded_tree() {
    let temp = TempDir::new().unwrap();
    setup_scan_fixture(temp.path());

    let output = macroforge_bin()
        .current_dir(temp.path())
        .args(["expand", "--scan", "src", "--out", "staging"])
        .output()
        .expect("failed to run macroforge");

    assert!(
        output.status.success(),
        "scan --out should succeed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let staging = temp.path().join("staging");

    // The macro file is written expanded, at the same relative path.
    let expanded = std::fs::read_to_string(staging.join("withmacro.ts")).unwrap();
    assert!(
        expanded.contains("fooDefaultValue"),
        "expanded output should contain generated runtime, got: {expanded}"
    );
    assert!(
        !expanded.contains("@derive"),
        "expanded output should not contain the @derive annotation"
    );

    // A macro-free file is copied verbatim.
    let plain_src = std::fs::read_to_string(temp.path().join("src").join("plain.ts")).unwrap();
    let plain_out = std::fs::read_to_string(staging.join("plain.ts")).unwrap();
    assert_eq!(
        plain_out, plain_src,
        "macro-free file should be copied verbatim"
    );

    // Non-TS assets and components are copied too.
    assert!(
        staging.join("nested").join("style.css").exists(),
        "css asset should be mirrored"
    );
    assert!(
        staging.join("Comp.svelte").exists(),
        "svelte component should be mirrored"
    );

    // No .expanded.* debug siblings leak into the mirrored tree.
    let staged = list_files(&staging);
    assert!(
        staged.iter().all(|f| !f.contains(".expanded.")),
        "staging tree must not contain .expanded.* files, got: {staged:?}"
    );

    // The scan does not write any new sibling into the source tree.
    let src_files = list_files(&temp.path().join("src"));
    let siblings: Vec<_> = src_files
        .iter()
        .filter(|f| f.contains(".expanded."))
        .collect();
    assert_eq!(
        siblings,
        vec![&"withmacro.expanded.ts".to_string()],
        "only the pre-existing sibling should be present in src, got: {src_files:?}"
    );
}

#[test]
fn scan_with_types_out_dir_writes_type_surfaces() {
    let temp = TempDir::new().unwrap();
    setup_scan_fixture(temp.path());

    let output = macroforge_bin()
        .current_dir(temp.path())
        .args(["expand", "--scan", "src", "--types-out", "types"])
        .output()
        .expect("failed to run macroforge");

    assert!(
        output.status.success(),
        "scan --types-out should succeed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let types = temp.path().join("types");

    // The macro file's type surface is written with a mirrored .d.ts path.
    let surface = std::fs::read_to_string(types.join("withmacro.d.ts"))
        .expect("type surface for withmacro.ts should exist");
    assert!(
        surface.contains("fooDefaultValue"),
        "type surface should declare generated symbols, got: {surface}"
    );

    // A macro-free file produces no type surface.
    assert!(
        !types.join("plain.d.ts").exists(),
        "macro-free file should not produce a type surface"
    );

    // No siblings written into the source tree.
    let src_files = list_files(&temp.path().join("src"));
    let new_siblings: Vec<_> = src_files
        .iter()
        .filter(|f| f.contains(".expanded.") && *f != "withmacro.expanded.ts")
        .collect();
    assert!(
        new_siblings.is_empty(),
        "no new siblings should be written to src, got: {src_files:?}"
    );
}

#[test]
fn scan_types_out_dir_nested_in_root_is_allowed() {
    // The canonical form from the bug report: run from the project root with a
    // relative output dir inside it. The output dir must be pruned from the
    // walk, not rejected.
    let temp = TempDir::new().unwrap();
    setup_scan_fixture(temp.path());

    let output = macroforge_bin()
        .current_dir(temp.path())
        .args(["expand", "--scan", ".", "--types-out", "dist/types"])
        .output()
        .expect("failed to run macroforge");

    assert!(
        output.status.success(),
        "scan . --types-out dist/types should succeed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let surface = std::fs::read_to_string(temp.path().join("dist/types/src/withmacro.d.ts"))
        .expect("type surface should be written into the nested output dir");
    assert!(surface.contains("fooDefaultValue"));
}

#[test]
fn scan_out_dir_nested_in_root_is_not_reingested_on_rerun() {
    // Running twice with an output dir inside the scanned root must be stable:
    // the second run prunes the first run's output instead of re-expanding it
    // into a deeper nested tree.
    let temp = TempDir::new().unwrap();
    setup_scan_fixture(temp.path());

    for _ in 0..2 {
        let output = macroforge_bin()
            .current_dir(temp.path())
            .args(["expand", "--scan", ".", "--out", "dist/staged"])
            .output()
            .expect("failed to run macroforge");
        assert!(
            output.status.success(),
            "repeated scan . --out dist/staged should succeed. stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // The expanded file is present once, not nested under a re-ingested copy.
    assert!(temp.path().join("dist/staged/src/withmacro.ts").exists());
    assert!(
        !temp.path().join("dist/staged/dist").exists(),
        "output dir must not be re-scanned into itself on a second run"
    );
}

#[test]
fn scan_out_dir_equal_to_root_is_rejected() {
    let temp = TempDir::new().unwrap();
    setup_scan_fixture(temp.path());

    let output = macroforge_bin()
        .current_dir(temp.path())
        .args(["expand", "--scan", "src", "--out", "src"])
        .output()
        .expect("failed to run macroforge");

    assert!(
        !output.status.success(),
        "an output dir equal to the scan root should be rejected"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("contains the scan root"),
        "stderr should explain the overwrite hazard, got: {stderr}"
    );
}

#[test]
fn scan_without_output_flags_writes_nothing() {
    let temp = TempDir::new().unwrap();
    setup_scan_fixture(temp.path());

    let before = list_files(&temp.path().join("src"));

    let output = macroforge_bin()
        .current_dir(temp.path())
        .args(["expand", "--scan", "src"])
        .output()
        .expect("failed to run macroforge");

    assert!(
        output.status.success(),
        "check-pass scan should succeed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let after = list_files(&temp.path().join("src"));
    assert_eq!(
        before, after,
        "a check-pass scan must not modify the source tree"
    );
}

#[test]
fn scan_with_emit_expanded_writes_siblings() {
    let temp = TempDir::new().unwrap();
    setup_scan_fixture(temp.path());

    let output = macroforge_bin()
        .current_dir(temp.path())
        .args(["expand", "--scan", "src", "--emit-expanded"])
        .output()
        .expect("failed to run macroforge");

    assert!(
        output.status.success(),
        "scan --emit-expanded should succeed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // The legacy sibling is (re)generated next to the source with expanded code.
    let sibling =
        std::fs::read_to_string(temp.path().join("src").join("withmacro.expanded.ts")).unwrap();
    assert!(
        sibling.contains("fooDefaultValue"),
        "sibling should contain expanded runtime, got: {sibling}"
    );

    // No staging directory is created when only --emit-expanded is given.
    assert!(
        !temp.path().join("staging").exists(),
        "no staging tree should be produced without --out"
    );
}

#[test]
fn emit_expanded_rejected_in_single_file_mode() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("solo.ts");
    std::fs::write(
        &input,
        "/** @derive(Default) */\nexport interface Solo { a: string; }\n",
    )
    .unwrap();

    let output = macroforge_bin()
        .arg("expand")
        .arg(&input)
        .arg("--emit-expanded")
        .current_dir(temp.path())
        .output()
        .expect("failed to run macroforge");

    assert!(
        !output.status.success(),
        "--emit-expanded without --scan should fail"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--emit-expanded is only valid with --scan"),
        "stderr should explain the misuse, got: {stderr}"
    );
}

/// Takes the project lock for `root` the way a second macroforge process would,
/// returning the locked handle. Dropping it releases the lock.
fn hold_project_lock(root: &Path) -> std::fs::File {
    let dir = root.join(".macroforge");
    std::fs::create_dir_all(&dir).unwrap();
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(dir.join(".lock"))
        .unwrap();
    file.lock().unwrap();
    file
}

/// Sets up a project root whose only source file has no macros, so a run over
/// it does no real work and any delay is the lock.
fn setup_lock_fixture(root: &Path, name: &str) {
    std::fs::write(
        root.join("package.json"),
        format!("{{ \"name\": \"{name}\" }}"),
    )
    .unwrap();
    std::fs::write(
        root.join("plain.ts"),
        "export interface Plain {\n  x: number;\n}\n",
    )
    .unwrap();
}

/// Reads `stderr` on a thread, signalling as soon as the lock notice appears.
///
/// Returns the receiver for that signal and a handle yielding everything read.
fn watch_for_lock_notice(
    stderr: std::process::ChildStderr,
) -> (
    std::sync::mpsc::Receiver<()>,
    std::thread::JoinHandle<String>,
) {
    use std::io::{BufRead, BufReader};

    let (tx, rx) = std::sync::mpsc::channel();
    let handle = std::thread::spawn(move || {
        let mut reader = BufReader::new(stderr);
        let mut collected = String::new();
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) | Err(_) => break,
                Ok(_) => {
                    if line.contains(LOCK_NOTICE) {
                        let _ = tx.send(());
                    }
                    collected.push_str(&line);
                }
            }
        }
        collected
    });

    (rx, handle)
}

/// What a blocked run prints, and the signal both lock tests wait on.
const LOCK_NOTICE: &str = "waiting for the project lock";

/// How long to wait for a spawned child to reach the lock.
///
/// Generous because it covers process startup, which is what makes this a wait
/// for an event rather than a sleep: the deadline only has to be longer than
/// the slowest plausible start, not tuned to the actual one.
const CONTENTION_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(60);

/// Blocks until `rx` reports the lock notice, failing if the child never
/// contends.
///
/// Waiting for the notice rather than for a fixed window is what keeps the test
/// about contention. A timer cannot tell a process blocked on the lock from one
/// still dynamically linking, so on a loaded machine the window expires while
/// the child is still starting, the lock is released before it ever reaches it,
/// and the assertions that follow pass without anything having been contended.
fn await_contention(rx: &std::sync::mpsc::Receiver<()>) {
    rx.recv_timeout(CONTENTION_TIMEOUT)
        .expect("macroforge should announce that it is waiting for the project lock");
}

#[test]
fn contended_lock_is_announced_and_then_acquired() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();
    setup_lock_fixture(root, "locked");

    let held = hold_project_lock(root);

    let mut child = macroforge_bin()
        .arg("expand")
        .arg("plain.ts")
        .current_dir(root)
        .stderr(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("failed to run macroforge");

    let (notice, reader) = watch_for_lock_notice(child.stderr.take().expect("stderr is piped"));
    await_contention(&notice);
    drop(held);

    let status = child.wait().unwrap();
    let stderr = reader.join().unwrap();
    assert!(
        stderr.contains(LOCK_NOTICE),
        "a blocked run should say what it is waiting for, got: {stderr}"
    );
    assert_eq!(
        status.code(),
        Some(2),
        "the run should complete normally once the lock frees, got: {stderr}"
    );
}

#[test]
fn contended_lock_stays_silent_in_quiet_mode() {
    // `--quiet` exists so callers can parse stderr; a lock this process
    // happened to contend on must not leak into it.
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();
    setup_lock_fixture(root, "locked-quiet");

    let held = hold_project_lock(root);

    let mut quiet = macroforge_bin()
        .arg("expand")
        .arg("plain.ts")
        .arg("--quiet")
        .current_dir(root)
        .stderr(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("failed to run macroforge");

    // A quiet run announces nothing, so it cannot signal that it reached the
    // lock. A second, speaking run does, and it was started later against the
    // same lock — so once it reports waiting, the quiet one is waiting too.
    let mut probe = macroforge_bin()
        .arg("expand")
        .arg("plain.ts")
        .current_dir(root)
        .stderr(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("failed to run the probe");

    let (notice, probe_reader) =
        watch_for_lock_notice(probe.stderr.take().expect("stderr is piped"));
    let (_, quiet_reader) = watch_for_lock_notice(quiet.stderr.take().expect("stderr is piped"));
    await_contention(&notice);
    drop(held);

    let status = quiet.wait().unwrap();
    probe.wait().unwrap();
    probe_reader.join().unwrap();

    let stderr = quiet_reader.join().unwrap();
    assert!(
        stderr.is_empty(),
        "stderr should be empty in quiet mode even when the lock was contended, got: {stderr}"
    );
    assert_eq!(status.code(), Some(2));
}

#[test]
fn cache_with_explicit_root_keeps_state_in_that_project() {
    // State and debug logs belong to the project being cached, not to the
    // directory the command happens to run from.
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("project");
    let elsewhere = temp.path().join("elsewhere");
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::create_dir_all(&elsewhere).unwrap();
    std::fs::write(root.join("macroforge.config.ts"), "export default {};\n").unwrap();
    std::fs::write(
        root.join("src").join("point.ts"),
        "/** @derive(Debug) */\nexport class Point {\n  x: number;\n}\n",
    )
    .unwrap();

    let output = macroforge_bin()
        .arg("cache")
        .arg(&root)
        .current_dir(&elsewhere)
        .output()
        .expect("failed to run macroforge");

    assert!(
        output.status.success(),
        "cache failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(root.join(".macroforge").join("debug.log").is_file());
    assert!(
        !elsewhere.join(".macroforge").exists(),
        "cache wrote state into the working directory"
    );
}

#[test]
fn svelte_package_accepts_full_rebuild() {
    // Packaging is incremental by default, so the escape hatch has to exist and
    // be discoverable — a user staring at a stale `dist` reaches for `--help`.
    let output = macroforge_bin()
        .args(["svelte-package", "--help"])
        .output()
        .expect("failed to run macroforge");

    assert!(output.status.success());
    let help = String::from_utf8_lossy(&output.stdout);
    assert!(
        help.contains("--full-rebuild"),
        "svelte-package --help should document --full-rebuild:\n{help}"
    );
}
