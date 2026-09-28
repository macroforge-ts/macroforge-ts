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
        .stdout(predicate::str::contains("Macroforge tooling"))
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
        .stdout(predicate::str::contains("Macroforge tooling"))
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
        .stdout(predicate::str::contains("generate-readmes"));
}

#[test]
fn test_verify_help() {
    mf_cmd()
        .arg("verify")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("verify"))
        .stdout(predicate::str::contains("--skip-build"))
        .stdout(predicate::str::contains("--skip-docs"));
}

#[test]
fn test_bump_help() {
    mf_cmd()
        .arg("bump")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("bump"))
        .stdout(predicate::str::contains("--version"));
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
