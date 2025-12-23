//! Check documentation freshness (CI)
//!
//! Verifies that generated documentation is up-to-date with source files.

use crate::core::config::Config;
use crate::utils::format;
use anyhow::Result;
use std::fs;
use std::path::Path;
use std::process::Command;

/// Files to check for freshness
const CHECK_FILES: &[(&str, &str)] = &[
    (
        "website/static/api-data/rust/index.json",
        "crates/macroforge_ts/src/lib.rs",
    ),
    (
        "website/static/api-data/typescript/index.json",
        "packages/shared/src/index.ts",
    ),
];

pub fn run() -> Result<()> {
    let config = Config::load()?;

    format::header("Checking Documentation Freshness");

    let mut all_fresh = true;
    let mut checked = 0;
    let mut stale = 0;

    for (doc_path, source_path) in CHECK_FILES {
        let doc_file = config.root.join(doc_path);
        let source_file = config.root.join(source_path);

        if !doc_file.exists() {
            format::warning(&format!("Doc file missing: {}", doc_path));
            stale += 1;
            all_fresh = false;
            continue;
        }

        if !source_file.exists() {
            format::warning(&format!("Source file missing: {}", source_path));
            continue;
        }

        print!("Checking {}... ", doc_path);

        let doc_mtime = get_mtime(&doc_file)?;
        let source_mtime = get_mtime(&source_file)?;

        if source_mtime > doc_mtime {
            println!("STALE");
            format::warning(&format!(
                "  {} is newer than {}",
                source_path, doc_path
            ));
            stale += 1;
            all_fresh = false;
        } else {
            println!("ok");
        }

        checked += 1;
    }

    // Also check git status for uncommitted changes in docs
    println!("\nChecking for uncommitted documentation changes...");

    let output = Command::new("git")
        .args([
            "status",
            "--porcelain",
            "website/static/api-data",
            "packages/*/README.md",
        ])
        .current_dir(&config.root)
        .output();

    if let Ok(output) = output {
        let status = String::from_utf8_lossy(&output.stdout);
        if !status.trim().is_empty() {
            println!("\nUncommitted changes:");
            for line in status.lines() {
                println!("  {}", line);
            }
            all_fresh = false;
        }
    }

    println!();
    if all_fresh {
        format::success(&format!("All {} documentation files are fresh", checked));
        Ok(())
    } else {
        format::error(&format!(
            "{} of {} documentation files are stale",
            stale, checked
        ));
        format::info("Run `mf docs all` to regenerate documentation");
        std::process::exit(1);
    }
}

fn get_mtime(path: &Path) -> Result<std::time::SystemTime> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.modified()?)
}
