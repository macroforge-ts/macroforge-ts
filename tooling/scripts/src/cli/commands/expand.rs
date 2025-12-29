//! Expand playground macros command
//!
//! Expands macros in playground TypeScript files.

use crate::cli::ExpandArgs;
use crate::core::config::Config;
use crate::utils::format;
use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Config file names in order of precedence
const CONFIG_FILES: &[&str] = &[
    "macroforge.config.ts",
    "macroforge.config.mts",
    "macroforge.config.js",
    "macroforge.config.mjs",
    "macroforge.config.cjs",
];

pub fn run(args: ExpandArgs) -> Result<()> {
    let config = Config::load()?;

    // Default playground roots
    let default_roots = vec![
        config.root.join("tooling/playground/svelte/src/lib/demo"),
        config.root.join("tooling/playground/svelte/src/lib/demo/types"),
        config.root.join("tooling/playground/vanilla/src"),
    ];

    let roots = if let Some(path) = &args.path {
        vec![config.root.join(path)]
    } else {
        default_roots
    };

    let playground_roots = vec![
        config.root.join("tooling/playground/svelte"),
        config.root.join("tooling/playground/vanilla"),
    ];

    format::header("Expand Macros");

    if args.use_cli {
        // Use CLI binary
        let cli_binary = find_cli_binary(&config.root);
        println!("Using CLI binary: {}", cli_binary.display());

        for root in &roots {
            if !root.exists() {
                continue;
            }

            let result = Command::new(&cli_binary)
                .args(["expand", "--scan", &root.to_string_lossy()])
                .output();

            match result {
                Ok(output) if output.status.success() => {
                    println!("  {} {}", "✓".green(), root.display());
                }
                Ok(output) => {
                    format::error(&format!(
                        "Failed to scan {}: {}",
                        root.display(),
                        String::from_utf8_lossy(&output.stderr)
                    ));
                }
                Err(e) => {
                    format::error(&format!("Failed to run CLI: {}", e));
                }
            }
        }
    } else {
        // Use Node.js API via npm script
        for root in &roots {
            if !root.exists() {
                continue;
            }

            // Find config for this root
            let config_path = find_config_file(root);
            if let Some(cfg) = &config_path {
                println!("loaded config: {}", cfg.display());
            }

            // Process TypeScript files
            for entry in fs::read_dir(root)? {
                let entry = entry?;
                let path = entry.path();

                if !is_source_file(&path) {
                    continue;
                }

                let expanded_path = get_expanded_path(&path);

                // Use node to run expansion
                let result = Command::new("node")
                    .args([
                        "-e",
                        &format!(
                            r#"
                            const {{ expandSync }} = require('{}');
                            const fs = require('fs');
                            const code = fs.readFileSync('{}', 'utf8');
                            const result = expandSync(code, '{}', {{ keepDecorators: false }});
                            fs.writeFileSync('{}', result.code);
                            "#,
                            config.root.join("crates/macroforge_ts").display(),
                            path.display(),
                            path.display(),
                            expanded_path.display()
                        ),
                    ])
                    .output();

                match result {
                    Ok(output) if output.status.success() => {
                        let rel_src = path.strip_prefix(&config.root).unwrap_or(&path);
                        let rel_dst = expanded_path.strip_prefix(&config.root).unwrap_or(&expanded_path);
                        println!("expanded {} -> {}", rel_src.display(), rel_dst.display());
                    }
                    Ok(output) => {
                        format::error(&format!(
                            "Failed to expand {}: {}",
                            path.display(),
                            String::from_utf8_lossy(&output.stderr)
                        ));
                    }
                    Err(e) => {
                        format::error(&format!("Failed to expand {}: {}", path.display(), e));
                    }
                }
            }
        }
    }

    // Format expanded files with biome
    for playground_root in &playground_roots {
        if !playground_root.exists() {
            continue;
        }

        let playground_name = playground_root.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        let result = Command::new("deno")
            .args(["run", "-A", "npm:@biomejs/biome", "format", "--write", "src"])
            .current_dir(playground_root)
            .output();

        match result {
            Ok(output) if output.status.success() => {
                println!("formatted playground/{}/src", playground_name);
            }
            Ok(output) => {
                format::warning(&format!(
                    "Formatting failed in playground/{}: {}",
                    playground_name,
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
            Err(e) => {
                format::warning(&format!("Could not format {}: {}", playground_name, e));
            }
        }
    }

    format::success("Expansion complete");
    Ok(())
}

/// Find the CLI binary (release or debug)
fn find_cli_binary(root: &Path) -> PathBuf {
    let release = root.join("crates/target/release/macroforge");
    if release.exists() {
        return release;
    }

    let debug = root.join("crates/target/debug/macroforge");
    if debug.exists() {
        return debug;
    }

    // Fall back to PATH
    PathBuf::from("macroforge")
}

/// Find config file starting from a directory
fn find_config_file(start_dir: &Path) -> Option<PathBuf> {
    let mut dir = start_dir.to_path_buf();

    loop {
        for config_name in CONFIG_FILES {
            let config_path = dir.join(config_name);
            if config_path.exists() {
                return Some(config_path);
            }
        }

        // Stop at package.json (project root)
        if dir.join("package.json").exists() {
            break;
        }

        match dir.parent() {
            Some(parent) if parent != dir => dir = parent.to_path_buf(),
            _ => break,
        }
    }

    None
}

/// Check if a file is a source file (not already expanded)
fn is_source_file(path: &Path) -> bool {
    let name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    name.ends_with(".ts") && !name.contains(".expanded.")
}

/// Get the expanded output path for a source file
fn get_expanded_path(file: &Path) -> PathBuf {
    let dir = file.parent().unwrap_or(Path::new("."));
    let basename = file.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file");

    // Find first dot to split name from extensions
    if let Some(dot_idx) = basename.find('.') {
        let name = &basename[..dot_idx];
        let extensions = &basename[dot_idx..];
        dir.join(format!("{}.expanded{}", name, extensions))
    } else {
        dir.join(format!("{}.expanded", basename))
    }
}
