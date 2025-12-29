//! Integration tests for the mf CLI binary
//!
//! These tests verify the CLI interface works correctly by running the actual binary.
//! Some commands may fail gracefully when run outside the project root - this is expected.

use assert_cmd::Command;
use predicates::prelude::*;

/// Helper to create a command for the mf binary
fn mf_cmd() -> Command {
    Command::new(env!("CARGO_BIN_EXE_mf"))
}

#[test]
fn test_help_flag() {
    mf_cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Macroforge Tooling"))
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("Commands:"));
}

#[test]
fn test_version_flag() {
    mf_cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("mf"))
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn test_no_args_shows_help() {
    // When no command is provided, the CLI should show help
    mf_cmd()
        .assert()
        .success()
        .stdout(predicate::str::contains("Macroforge Tooling"))
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_unknown_command() {
    mf_cmd()
        .arg("nonexistent-command")
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:").or(predicate::str::contains("unrecognized")));
}

#[test]
fn test_diagnostics_help() {
    mf_cmd()
        .arg("diagnostics")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("diagnostics"))
        .stdout(predicate::str::contains("--log"))
        .stdout(predicate::str::contains("--tools"))
        .stdout(predicate::str::contains("--json"));
}

#[test]
fn test_manifest_help() {
    mf_cmd()
        .arg("manifest")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("manifest"))
        .stdout(predicate::str::contains("Commands:"))
        .stdout(predicate::str::contains("list"));
}

#[test]
fn test_manifest_list_outside_project() {
    // When run outside the project root, manifest list should fail gracefully
    // We don't assert success here because it may legitimately fail
    let result = mf_cmd()
        .arg("manifest")
        .arg("list")
        .assert()
        .code(predicate::in_iter([0, 1])); // Allow success (0) or error (1)

    // If it fails, it should have a reasonable error message
    if !result.get_output().status.success() {
        assert!(
            !result.get_output().stderr.is_empty() || !result.get_output().stdout.is_empty(),
            "Failed command should produce some output"
        );
    }
}

#[test]
fn test_docs_help() {
    mf_cmd()
        .arg("docs")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("docs"))
        .stdout(predicate::str::contains("Commands:"))
        .stdout(predicate::str::contains("extract-rust"))
        .stdout(predicate::str::contains("extract-ts"))
        .stdout(predicate::str::contains("build-book"))
        .stdout(predicate::str::contains("generate-readmes"));
}

#[test]
fn test_prep_help() {
    mf_cmd()
        .arg("prep")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("prep"))
        .stdout(predicate::str::contains("--dry-run"))
        .stdout(predicate::str::contains("--skip-build"));
}

#[test]
fn test_commit_help() {
    mf_cmd()
        .arg("commit")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("commit"))
        .stdout(predicate::str::contains("--yes"))
        .stdout(predicate::str::contains("--message"));
}

#[test]
fn test_build_help() {
    mf_cmd()
        .arg("build")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("build"))
        .stdout(predicate::str::contains("--repos"));
}

#[test]
fn test_versions_help() {
    mf_cmd()
        .arg("versions")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("versions"))
        .stdout(predicate::str::contains("--check-only"));
}

#[test]
fn test_expand_help() {
    mf_cmd()
        .arg("expand")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("expand"))
        .stdout(predicate::str::contains("--use-cli"));
}

#[test]
fn test_check_help() {
    mf_cmd()
        .arg("check")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("check"))
        .stdout(predicate::str::contains("<FILE>"));
}

#[test]
fn test_tui_help() {
    mf_cmd()
        .arg("tui")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("tui"))
        .stdout(predicate::str::contains("Interactive TUI dashboard"));
}

#[test]
fn test_global_flags() {
    // Test that global flags are recognized
    mf_cmd()
        .arg("--verbose")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--verbose"));

    mf_cmd()
        .arg("--debug")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--debug"));

    mf_cmd()
        .arg("--tui")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--tui"));
}

#[test]
fn test_manifest_subcommands() {
    // Test various manifest subcommands show up in help
    let output = mf_cmd()
        .arg("manifest")
        .arg("--help")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("list"));
    assert!(stdout.contains("get-version"));
    assert!(stdout.contains("set-version"));
    assert!(stdout.contains("apply-versions"));
    assert!(stdout.contains("swap-local"));
    assert!(stdout.contains("swap-registry"));
    assert!(stdout.contains("dump-versions"));
}

#[test]
fn test_docs_subcommands() {
    // Test various docs subcommands show up in help
    let output = mf_cmd()
        .arg("docs")
        .arg("--help")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("extract-rust"));
    assert!(stdout.contains("extract-ts"));
    assert!(stdout.contains("build-book"));
    assert!(stdout.contains("generate-readmes"));
    assert!(stdout.contains("check-freshness"));
    assert!(stdout.contains("all"));
}

#[test]
fn test_invalid_flag() {
    mf_cmd()
        .arg("--invalid-flag-that-does-not-exist")
        .assert()
        .failure()
        .stderr(predicate::str::contains("unexpected argument"));
}

#[test]
fn test_check_missing_file_argument() {
    // check command requires a file argument
    mf_cmd()
        .arg("check")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required").or(predicate::str::contains("<FILE>")));
}

#[test]
fn test_manifest_get_version_missing_args() {
    // get-version requires a repo argument
    mf_cmd()
        .arg("manifest")
        .arg("get-version")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required").or(predicate::str::contains("<REPO>")));
}

#[test]
fn test_manifest_set_version_missing_args() {
    // set-version requires both repo and version arguments
    mf_cmd()
        .arg("manifest")
        .arg("set-version")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}
