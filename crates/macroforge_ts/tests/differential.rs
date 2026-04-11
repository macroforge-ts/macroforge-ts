//! Differential test harness for declarative macros.
//!
//! For each fixture under `tests/fixtures/differential/**/*.ts`, this
//! test:
//!
//! 1. Expands the fixture once with `BuildMode::Dev` and once with
//!    `BuildMode::Prod`.
//! 2. Feeds each expansion to Deno as a standalone TypeScript script.
//! 3. Captures `stdout` from both runs and asserts they match.
//!
//! Dev and prod produce different SOURCE for reverse-monomorphization
//! macros (inline expansion vs shared runtime helper), but their
//! observable BEHAVIOR must be identical. Any divergence is either
//! a bug in the expander or a bug in the share-mode emission — the
//! harness catches both.
//!
//! The harness skips with a warning if `deno` isn't on the `PATH`,
//! rather than failing hard. CI always has Deno (the playground tests
//! depend on it).

use std::path::Path;
use std::process::Command;

use macroforge_ts::host::MacroExpander;
use macroforge_ts::host::declarative::BuildMode;

fn deno_available() -> bool {
    Command::new("deno")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn run_in_deno(source: &str) -> Result<String, String> {
    // Write the script to a tempfile and run it via `deno run`.
    // `deno eval` takes code on the command line, which gets clumsy for
    // multi-line TypeScript; the tempfile path avoids quoting issues.
    let tmp_file = tempfile::Builder::new()
        .prefix("macroforge-differential-")
        .suffix(".ts")
        .tempfile()
        .map_err(|e| format!("tempfile: {}", e))?;
    std::fs::write(tmp_file.path(), source.as_bytes())
        .map_err(|e| format!("write tempfile: {}", e))?;

    let output = Command::new("deno")
        .arg("run")
        .arg("--no-check")
        .arg("--allow-all")
        .arg(tmp_file.path())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()
        .map_err(|e| format!("failed to spawn deno: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "deno run exited with status {}: stderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn expand_with_mode(source: &str, file_name: &str, mode: BuildMode) -> Result<String, String> {
    let mut expander = MacroExpander::new().map_err(|e| format!("expander: {}", e))?;
    expander.set_build_mode(mode);
    let result = expander
        .expand_source(source, file_name)
        .map_err(|e| format!("expand ({:?}): {}", mode, e))?;

    // Surface any error-level diagnostics.
    let errors: Vec<String> = result
        .diagnostics
        .iter()
        .filter(|d| matches!(d.level, macroforge_ts::ts_syn::abi::DiagnosticLevel::Error))
        .map(|d| d.message.clone())
        .collect();
    if !errors.is_empty() {
        return Err(format!("{:?} expansion errors: {:?}", mode, errors));
    }

    Ok(result.code)
}

fn run_differential(path: &Path) {
    let input = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read fixture {:?}: {}", path, e));
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .expect("fixture path")
        .to_string();

    let dev_code = expand_with_mode(&input, &file_name, BuildMode::Dev)
        .unwrap_or_else(|e| panic!("{:?}: {}", path, e));
    let prod_code = expand_with_mode(&input, &file_name, BuildMode::Prod)
        .unwrap_or_else(|e| panic!("{:?}: {}", path, e));

    let dev_out = run_in_deno(&dev_code)
        .unwrap_or_else(|e| panic!("{:?} dev run failed: {}\ncode:\n{}", path, e, dev_code));
    let prod_out = run_in_deno(&prod_code)
        .unwrap_or_else(|e| panic!("{:?} prod run failed: {}\ncode:\n{}", path, e, prod_code));

    assert_eq!(
        dev_out, prod_out,
        "differential output mismatch for {:?}\n--- dev stdout ---\n{}\n--- prod stdout ---\n{}\n--- dev code ---\n{}\n--- prod code ---\n{}",
        path, dev_out, prod_out, dev_code, prod_code
    );
}

#[test]
fn differential_declarative_macros() {
    if !deno_available() {
        eprintln!(
            "[differential] deno not found on PATH — skipping. Install Deno to run this test."
        );
        return;
    }

    let fixtures_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("differential");
    assert!(
        fixtures_dir.exists(),
        "fixtures dir not found: {:?}",
        fixtures_dir
    );

    let entries: Vec<_> = std::fs::read_dir(&fixtures_dir)
        .expect("read_dir")
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|x| x.to_str())
                .is_some_and(|x| x == "ts")
        })
        .collect();

    assert!(
        !entries.is_empty(),
        "no differential fixtures found in {:?}",
        fixtures_dir
    );

    for entry in entries {
        let path = entry.path();
        eprintln!("[differential] {:?}", path.file_name().unwrap());
        run_differential(&path);
    }
}
