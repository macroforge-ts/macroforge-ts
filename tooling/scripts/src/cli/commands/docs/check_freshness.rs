//! Check documentation freshness (CI)
//!
//! Verifies that generated documentation is up-to-date with source files by
//! regenerating docs into a temp directory and comparing content hashes.
//! Timestamps ("generated" fields in JSON) are stripped before comparison.

use crate::cli::commands::docs::{extract_rust, extract_ts, generate_readmes};
use crate::core::config::Config;
use crate::utils::format;
use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// The 9 builtin macro slugs (matching extract_rust.rs BUILTIN_MACROS)
const BUILTIN_MACRO_SLUGS: &[&str] = &[
    "debug",
    "clone",
    "default",
    "hash",
    "ord",
    "partial-eq",
    "partial-ord",
    "serialize",
    "deserialize",
];

/// Rust JSON data files
const RUST_JSON_FILES: &[&str] = &[
    "website/static/api-data/rust/index.json",
    "website/static/api-data/rust/macroforge_ts.json",
    "website/static/api-data/rust/macroforge_ts_syn.json",
    "website/static/api-data/rust/macroforge_ts_quote.json",
    "website/static/api-data/rust/macroforge_ts_macros.json",
    "website/static/api-data/rust/builtin-macros.json",
];

/// TypeScript JSON data files
const TS_JSON_FILES: &[&str] = &[
    "website/static/api-data/typescript/index.json",
    "website/static/api-data/typescript/core.json",
    "website/static/api-data/typescript/shared.json",
    "website/static/api-data/typescript/vite-plugin.json",
    "website/static/api-data/typescript/typescript-plugin.json",
    "website/static/api-data/typescript/svelte-language-server.json",
    "website/static/api-data/typescript/svelte-preprocessor.json",
    "website/static/api-data/typescript/mcp-server.json",
];

/// Rust crate README files
const RUST_README_FILES: &[&str] = &[
    "crates/macroforge_ts/README.md",
    "crates/macroforge_ts_syn/README.md",
    "crates/macroforge_ts_quote/README.md",
    "crates/macroforge_ts_macros/README.md",
];

/// TypeScript package README files
const TS_README_FILES: &[&str] = &[
    "packages/shared/README.md",
    "packages/vite-plugin/README.md",
    "packages/typescript-plugin/README.md",
    "packages/svelte-language-server/README.md",
    "packages/svelte-preprocessor/README.md",
    "packages/mcp-server/README.md",
];

/// Build the list of all generated file paths (relative to root)
fn all_generated_files() -> Vec<String> {
    let mut files = Vec::new();

    // JSON API data
    for f in RUST_JSON_FILES {
        files.push(f.to_string());
    }
    for f in TS_JSON_FILES {
        files.push(f.to_string());
    }

    // .svx pages for builtin macros
    for slug in BUILTIN_MACRO_SLUGS {
        files.push(format!(
            "website/src/routes/docs/builtin-macros/{}/+page.svx",
            slug
        ));
    }

    // MCP markdown docs for builtin macros
    for slug in BUILTIN_MACRO_SLUGS {
        files.push(format!(
            "packages/mcp-server/docs/builtin-macros/{}.md",
            slug
        ));
    }

    // README files
    for f in RUST_README_FILES {
        files.push(f.to_string());
    }
    for f in TS_README_FILES {
        files.push(f.to_string());
    }

    files
}

/// Strip the "generated" timestamp field from JSON content so that
/// timestamps do not cause false-positive staleness.
fn strip_generated_field(content: &str) -> String {
    // Remove lines matching `"generated": "..."` (with optional trailing comma)
    let re = regex::Regex::new(r#"(?m)^\s*"generated"\s*:\s*"[^"]*",?\s*\n?"#).unwrap();
    re.replace_all(content, "").to_string()
}

/// Compute a normalized content string for comparison.
/// For JSON files, strip the "generated" timestamp; for everything else, use raw content.
fn normalize_content(path: &Path) -> Result<String> {
    let content = fs::read_to_string(path)?;
    if path.extension().is_some_and(|e| e == "json") {
        Ok(strip_generated_field(&content))
    } else {
        Ok(content)
    }
}

pub fn run() -> Result<()> {
    let config = Config::load()?;

    format::header("Checking Documentation Freshness");

    let generated_files = all_generated_files();
    let total = generated_files.len();

    // ── Step 1: snapshot current file contents ──────────────────────────
    println!("Snapshotting {} generated files...", total);

    let mut before: HashMap<String, String> = HashMap::new();
    let mut missing_before: Vec<String> = Vec::new();

    for rel_path in &generated_files {
        let abs = config.root.join(rel_path);
        if abs.exists() {
            match normalize_content(&abs) {
                Ok(c) => {
                    before.insert(rel_path.clone(), c);
                }
                Err(e) => {
                    format::warning(&format!("Could not read {}: {}", rel_path, e));
                }
            }
        } else {
            missing_before.push(rel_path.clone());
        }
    }

    // ── Step 2: regenerate docs into the real locations ─────────────────
    // (extract_rust and extract_ts write to a given output dir;
    //  generate_readmes writes READMEs in-place)
    //
    // To avoid modifying the working tree we regenerate into a temp dir
    // for the JSON files, and capture README content via the public helper.

    println!("\nRegenerating documentation for comparison...\n");

    let tmp = tempfile::tempdir()?;
    let tmp_rust = tmp.path().join("rust");
    let tmp_ts = tmp.path().join("typescript");
    fs::create_dir_all(&tmp_rust)?;
    fs::create_dir_all(&tmp_ts)?;

    // Regenerate Rust JSON docs into temp
    extract_rust::run(&tmp_rust)?;

    // Regenerate TypeScript JSON docs into temp
    extract_ts::run(&tmp_ts)?;

    // Regenerate READMEs into temp
    let tmp_readmes = tmp.path().join("readmes");
    fs::create_dir_all(&tmp_readmes)?;
    generate_readmes::generate_all_to(&config.root, &tmp_readmes)?;

    // ── Step 3: build "after" map from regenerated files ────────────────
    println!();
    format::header("Comparing Files");

    let mut stale: Vec<String> = Vec::new();
    let mut checked: usize = 0;

    for rel_path in &generated_files {
        // Determine where the freshly-generated version lives
        let fresh_path = freshly_generated_path(rel_path, tmp.path(), &tmp_readmes);

        let Some(fresh) = fresh_path else {
            // For svx / MCP files we cannot regenerate independently here,
            // so just check that the file exists.
            let abs = config.root.join(rel_path);
            print!("Checking {}... ", rel_path);
            if abs.exists() {
                println!("ok (existence only)");
                checked += 1;
            } else {
                println!("MISSING");
                stale.push(rel_path.clone());
            }
            continue;
        };

        print!("Checking {}... ", rel_path);

        if !fresh.exists() {
            // Generator did not produce this file (e.g. package dir missing)
            println!("skipped (not generated)");
            continue;
        }

        let fresh_content = normalize_content(&fresh)?;

        if let Some(old_content) = before.get(rel_path) {
            if *old_content == fresh_content {
                println!("ok");
            } else {
                println!("STALE");
                stale.push(rel_path.clone());
            }
        } else {
            // File was missing before regeneration
            println!("MISSING");
            stale.push(rel_path.clone());
        }

        checked += 1;
    }

    // ── Step 4: report ──────────────────────────────────────────────────
    println!();

    if stale.is_empty() {
        format::success(&format!("All {} documentation files are fresh", checked));
        Ok(())
    } else {
        format::error(&format!(
            "{} of {} documentation files are stale:",
            stale.len(),
            checked
        ));
        for path in &stale {
            println!("  - {}", path);
        }
        println!();
        format::info("Run `mf docs all` to regenerate documentation");
        std::process::exit(1);
    }
}

/// Map a relative generated-file path to its freshly-regenerated counterpart
/// inside the temp directory. Returns `None` for file types we do not
/// regenerate here (svx pages, MCP markdown).
fn freshly_generated_path(rel: &str, tmp_root: &Path, tmp_readmes: &Path) -> Option<PathBuf> {
    // JSON API data — rust
    if let Some(name) = rel.strip_prefix("website/static/api-data/rust/") {
        return Some(tmp_root.join("rust").join(name));
    }
    // JSON API data — typescript
    if let Some(name) = rel.strip_prefix("website/static/api-data/typescript/") {
        return Some(tmp_root.join("typescript").join(name));
    }
    // README files — both crates and packages
    if rel.starts_with("crates/") || rel.starts_with("packages/") {
        // e.g. "crates/macroforge_ts/README.md" -> tmp_readmes/crates/macroforge_ts/README.md
        return Some(tmp_readmes.join(rel));
    }
    // svx pages and MCP markdown are not regenerated in this check
    None
}
