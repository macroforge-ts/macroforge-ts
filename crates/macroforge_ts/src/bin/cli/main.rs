//! # Macroforge CLI Binary
//!
//! This binary provides command-line utilities for working with Macroforge TypeScript macros.
//! It is designed for development workflows, enabling macro expansion and type checking
//! without requiring Node.js integration.
//!
//! ## Commands
//!
//! ### `macroforge expand`
//!
//! Expands macros in TypeScript/TSX files:
//!
//! ```bash
//! # Expand a single file
//! macroforge expand src/User.ts
//!
//! # Expand to specific output file
//! macroforge expand src/User.ts --out dist/User.js
//!
//! # Scan and expand all files in a directory
//! macroforge expand --scan src/
//!
//! # Use only built-in Rust macros (faster, no external dependencies)
//! macroforge expand src/User.ts --builtin-only
//!
//! # Print expanded output to stdout
//! macroforge expand src/User.ts --print
//! ```
//!
//! ### `macroforge tsc`
//!
//! Run TypeScript type checking with macro expansion baked in:
//!
//! ```bash
//! # Type check with default tsconfig.json
//! macroforge tsc
//!
//! # Type check with custom tsconfig
//! macroforge tsc -p tsconfig.build.json
//! ```
//!
//! ### `macroforge svelte-check`
//!
//! Run svelte-check with macro expansion baked into file reads:
//!
//! ```bash
//! # Type check a SvelteKit project
//! macroforge svelte-check
//!
//! # Explicit tsconfig
//! macroforge svelte-check --tsconfig tsconfig.json
//!
//! # Fail on warnings
//! macroforge svelte-check --fail-on-warnings
//!
//! # Machine-readable output
//! macroforge svelte-check --output machine
//! ```
//!
//! ## Configuration
//!
//! The CLI automatically searches for a configuration file starting from the input file's
//! directory, walking up to the nearest `package.json` (project root). Configuration files
//! are searched in this order:
//!
//! 1. `macroforge.config.ts`
//! 2. `macroforge.config.mts`
//! 3. `macroforge.config.js`
//! 4. `macroforge.config.mjs`
//! 5. `macroforge.config.cjs`
//!
//! ### Foreign Types
//!
//! Configuration files can define foreign type handlers for external types like Effect's
//! `DateTime`. When a matching type is found during expansion, the configured handlers
//! are used automatically:
//!
//! ```javascript
//! // macroforge.config.ts
//! import { DateTime } from "effect";
//!
//! export default {
//!   foreignTypes: {
//!     "DateTime.DateTime": {
//!       from: ["effect"],
//!       serialize: (v) => DateTime.formatIso(v),
//!       deserialize: (raw) => DateTime.unsafeFromDate(new Date(raw)),
//!       default: () => DateTime.unsafeNow()
//!     }
//!   }
//! }
//! ```
//!
//! See the [Configuration](crate::host::config) module for full documentation.
//!
//! ## Output File Naming
//!
//! By default, expanded files are written with `.expanded` inserted before the extension:
//!
//! - `foo.ts` → `foo.expanded.ts`
//! - `foo.svelte.ts` → `foo.expanded.svelte.ts`
//!
//! ## Exit Codes
//!
//! - `0` - Success
//! - `1` - Error during expansion
//! - `2` - No macros found in the input file (with `--quiet` suppresses output)
//!
//! ## Node.js Integration
//!
//! By default, the CLI uses Node.js to support external/custom macros from npm packages.
//! Use `--builtin-only` for fast expansion with only the built-in Rust macros (Debug, Clone,
//! PartialEq, Hash, Ord, PartialOrd, Default, Serialize, Deserialize).
//!
//! Both modes load and respect `macroforge.config.ts/js` for foreign type configuration.
//! The config is parsed natively using SWC, so foreign types work without Node.js.

mod cache;
mod expand;
mod watch;
mod wrappers;

#[cfg(test)]
mod tests;

use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use cache::{run_cache, run_refresh};
use expand::{expand_file, scan_and_expand};
use watch::run_watch;
use wrappers::{run_svelte_check_wrapper, run_tsc_wrapper};

/// Command-line interface for Macroforge TypeScript macro utilities.
///
/// Provides three main commands:
/// - `expand` - Expand macros in TypeScript files
/// - `tsc` - Run TypeScript type checking with macro expansion
/// - `svelte-check` - Run svelte-check with macro expansion
#[derive(Parser)]
#[command(name = "macroforge", about = "TypeScript macro development utilities")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// Available CLI subcommands.
#[derive(Subcommand)]
enum Command {
    /// Expand macros in a TypeScript file or directory.
    ///
    /// By default, uses Node.js for full macro support including external macros.
    /// Use `--builtin-only` for faster expansion with only built-in Rust macros.
    Expand {
        /// Path to the TypeScript/TSX file or directory to expand
        input: Option<PathBuf>,
        /// Optional path to write the transformed JS/TS output
        #[arg(long)]
        out: Option<PathBuf>,
        /// Optional path to write the generated .d.ts surface
        #[arg(long = "types-out")]
        types_out: Option<PathBuf>,
        /// Print expansion result to stdout even if --out is specified
        #[arg(long)]
        print: bool,
        /// Use only built-in Rust macros (faster, but no external macro support)
        #[arg(long)]
        builtin_only: bool,
        /// Suppress output when no macros are found (exit silently with code 2)
        #[arg(long, short = 'q')]
        quiet: bool,
        /// Scan directory for TypeScript files with macros (uses input as root, or cwd if not specified)
        #[arg(long)]
        scan: bool,
        /// Include files ignored by .gitignore when scanning
        #[arg(long)]
        include_ignored: bool,
    },
    /// Run tsc with macro expansion baked into file reads (tsc --noEmit semantics)
    Tsc {
        /// Path to tsconfig.json (defaults to tsconfig.json in cwd)
        #[arg(long, short = 'p')]
        project: Option<PathBuf>,
    },
    /// Run svelte-check with macro expansion baked into file reads
    SvelteCheck {
        /// Path to the workspace directory (defaults to cwd)
        #[arg(long)]
        workspace: Option<PathBuf>,
        /// Path to tsconfig.json (defaults to tsconfig.json in cwd)
        #[arg(long)]
        tsconfig: Option<PathBuf>,
        /// Output format: human, human-verbose, machine, machine-verbose
        #[arg(long)]
        output: Option<String>,
        /// Fail on warnings in addition to errors
        #[arg(long)]
        fail_on_warnings: bool,
    },
    /// Watch source files and maintain a .macroforge/cache for fast Vite dev mode.
    ///
    /// Expands macros in all TypeScript files and writes results to .macroforge/cache/.
    /// When a file changes, only that file is re-expanded. When the config changes,
    /// all files are re-expanded. Use with `vite dev` for instant macro expansion.
    Watch {
        /// Root directory to watch (defaults to cwd)
        root: Option<PathBuf>,
        /// Use only built-in Rust macros (faster, no Node.js)
        #[arg(long)]
        builtin_only: bool,
        /// Debounce interval in milliseconds
        #[arg(long, default_value = "100")]
        debounce_ms: u64,
    },
    /// Build the .macroforge/cache once and exit.
    ///
    /// Same as `watch` but without the file-watching loop — expands all TypeScript
    /// files, writes the cache, then exits. Useful in CI or as a pre-build step.
    Cache {
        /// Root directory to cache (defaults to cwd)
        root: Option<PathBuf>,
        /// Use only built-in Rust macros (faster, no Node.js)
        #[arg(long)]
        builtin_only: bool,
    },
    /// Delete the .macroforge/cache directory and rebuild from scratch.
    ///
    /// Equivalent to manually deleting the cache and running `cache`.
    /// Useful when the cache is corrupted or you want a guaranteed clean state.
    Refresh {
        /// Root directory (defaults to cwd)
        root: Option<PathBuf>,
        /// Use only built-in Rust macros (faster, no Node.js)
        #[arg(long)]
        builtin_only: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Expand {
            input,
            out,
            types_out,
            print,
            builtin_only,
            quiet,
            scan,
            include_ignored,
        } => {
            if scan {
                let root = input.unwrap_or_else(|| PathBuf::from("."));
                scan_and_expand(root, builtin_only, include_ignored)
            } else {
                let input = input.ok_or_else(|| {
                    anyhow!("input file required (use --scan to scan a directory)")
                })?;

                // If input is a directory, treat it as --scan
                if input.is_dir() {
                    scan_and_expand(input, builtin_only, include_ignored)
                } else {
                    expand_file(input, out, types_out, print, builtin_only, quiet)
                }
            }
        }
        Command::Tsc { project } => run_tsc_wrapper(project),
        Command::SvelteCheck {
            workspace,
            tsconfig,
            output,
            fail_on_warnings,
        } => run_svelte_check_wrapper(workspace, tsconfig, output, fail_on_warnings),
        Command::Watch {
            root,
            builtin_only,
            debounce_ms,
        } => run_watch(root, builtin_only, debounce_ms),
        Command::Cache { root, builtin_only } => run_cache(root, builtin_only),
        Command::Refresh { root, builtin_only } => run_refresh(root, builtin_only),
    }
}
