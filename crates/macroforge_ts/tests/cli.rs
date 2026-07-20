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
