//! Integration tests for the macroforge CLI.

use std::process::Command;
use tempfile::TempDir;

fn macroforge_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_macroforge"))
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
        .arg("--builtin-only")
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
        .arg("--builtin-only")
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
        .arg("--builtin-only")
        .output()
        .expect("failed to run macroforge");

    assert_eq!(
        output.status.code(),
        Some(0),
        "should exit with code 0 when macros are expanded. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
