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
//! # Expand a single file (writes src/User.expanded.ts by default)
//! macroforge expand src/User.ts
//!
//! # Expand to specific output file
//! macroforge expand src/User.ts --out dist/User.js
//!
//! # Check pass: scan a directory, expand in memory, write nothing
//! macroforge expand --scan src/
//!
//! # Emit a packager-ready mirrored tree of the expanded sources into <dir>
//! macroforge expand --scan src/ --out dist/expanded
//!
//! # Emit the generated .d.ts type surfaces into <dir> (mirrored layout)
//! macroforge expand --scan src/ --types-out dist/types
//!
//! # Legacy: write <name>.expanded.<ext> siblings next to each source
//! macroforge expand --scan src/ --emit-expanded
//!
//! # Print expanded output to stdout
//! macroforge expand src/User.ts --print
//! ```
//!
//! ### Scan output routing
//!
//! In `--scan` (or directory-input) mode, `--out` and `--types-out` name
//! **directories**, and the scanned tree is mirrored into them:
//!
//! - `--out <dir>` writes each macro file expanded at `<dir>/<relative-path>`
//!   (same filename) and copies every other file (`.svelte`, `.css`, `.d.ts`,
//!   macro-free `.ts`, assets) verbatim — a complete, packager-ready staging
//!   tree.
//! - `--types-out <dir>` writes the generated `.d.ts` type surfaces mirrored
//!   under `<dir>` (`foo.ts` → `foo.d.ts`, `foo.svelte.ts` → `foo.svelte.d.ts`).
//!
//! Files whose name contains `.expanded.` are never scanned or mirrored. An
//! output directory nested inside the scanned root (e.g. `--scan . --types-out
//! dist/types`) is pruned from the walk, so a re-run never re-ingests its own
//! output; only an output directory that *is* the scan root or an ancestor of
//! it is rejected. A scan fails with a non-zero exit if any file cannot be
//! expanded, so a partial staging tree never silently feeds a downstream
//! packager.
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
//! ### `macroforge svelte-package`
//!
//! Run `@sveltejs/package` over macro-expanded sources, so a published library
//! ships the generated derive runtime (and correct `.d.ts`) for its
//! `.ts`/`.svelte.ts` type modules — no separate expand step or staging tree.
//! Drop-in replacement for `svelte-package` in a library build:
//!
//! ```bash
//! # Package src/lib into dist with macros expanded
//! macroforge svelte-package --input src/lib --output dist
//!
//! # Explicit tsconfig; skip .d.ts emission
//! macroforge svelte-package --tsconfig tsconfig.json --no-types
//!
//! # Ignore the previous build and repackage from nothing
//! macroforge svelte-package --full-rebuild
//! ```
//!
//! Builds are incremental. Each run records what it consumed and produced in
//! `.macroforge/svelte-package/`, and a run whose inputs all match the previous
//! one exits without repackaging. When a rebuild is needed, only the files that
//! changed are re-expanded; the rest keep the expanded output from last time.
//!
//! A file that differs only in formatting — trailing whitespace, blank-line runs
//! — does not count as changed. `.ts` is transpiled on the way into the package
//! so its formatting is discarded anyway, but `.svelte` and `.js` are copied
//! through verbatim, which means a formatting-only edit to those will not reach
//! the package until the next real change or a `--full-rebuild`.
//!
//! Any of these forces a full rebuild on its own: a changed macroforge version,
//! `macroforge.config.*`, or external macro binary; a changed `svelte.config.*`,
//! `package.json`, or tsconfig; a changed `@sveltejs/package`, `@macroforge/core`, or
//! `@macroforge/svelte-preprocessor` version; different command-line options; a
//! changed project source outside the input directory; or an output directory
//! that was deleted or modified behind the CLI's back.
//!
//! Expansion failures fail the build. A module that cannot be expanded has no
//! correct packaged form, and shipping its unexpanded source publishes a library
//! whose generated runtime is silently missing.
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
//! In single-file mode (and with `--scan --emit-expanded`), expanded files are
//! written with `.expanded` inserted before the extension:
//!
//! - `foo.ts` → `foo.expanded.ts`
//! - `foo.svelte.ts` → `foo.expanded.svelte.ts`
//!
//! In `--scan` mode with `--out`/`--types-out`, the original filenames are
//! preserved and mirrored under the target directory (see *Scan output
//! routing* above).
//!
//! ## Exit Codes
//!
//! - `0` - Success
//! - `1` - Error during expansion
//! - `2` - No macros found in the input file (with `--quiet` suppresses output)
//!
//! ## Configuration
//!
//! The CLI loads and respects `macroforge.config.ts/js` for foreign type configuration.
//! The config is parsed natively without requiring Node.js. External macros are
//! supported via FFI (compiled `.node`/`.dylib`/`.so` packages loaded with dlopen).

mod atomic_fs;
mod build;
mod cache;
mod expand;
mod lock;
mod package_expand;
mod package_state;
mod watch;
mod wrappers;

#[cfg(test)]
mod tests;

use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use build::run_build;
use cache::{run_cache, run_refresh};
use expand::{ScanOptions, expand_file, scan_and_expand};
use lock::{ProjectLock, resolve_project_root};
use watch::run_watch;
use wrappers::{run_svelte_check_wrapper, run_svelte_package_wrapper, run_tsc_wrapper};

/// Command-line interface for Macroforge TypeScript macro utilities.
///
/// Provides eight commands:
/// - `expand` - Expand macros in TypeScript files or directories
/// - `tsc` - Run TypeScript type checking with macro expansion
/// - `svelte-check` - Run svelte-check with macro expansion
/// - `svelte-package` - Run @sveltejs/package with macro expansion
/// - `watch` - Watch files and maintain the .macroforge/cache
/// - `cache` - Build the .macroforge/cache once and exit
/// - `refresh` - Delete and rebuild the .macroforge/cache
/// - `build` - Build a macro crate to WASM with $ aliases
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
        /// Suppress output when no macros are found (exit silently with code 2)
        #[arg(long, short = 'q')]
        quiet: bool,
        /// Scan directory for TypeScript files with macros (uses input as root, or cwd if not specified)
        #[arg(long)]
        scan: bool,
        /// Include files ignored by .gitignore when scanning
        #[arg(long)]
        include_ignored: bool,
        /// With --scan: also write `<name>.expanded.<ext>` debug siblings next to
        /// each expanded source file (legacy behavior; default off)
        #[arg(long)]
        emit_expanded: bool,
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
    /// Run @sveltejs/package with macro expansion baked into file reads.
    ///
    /// Emits a published library whose `.ts`/`.svelte.ts` type modules ship the
    /// generated derive runtime and correct `.d.ts` — no separate expand step.
    SveltePackage {
        /// Input directory (defaults to svelte-package's own default, src/lib)
        #[arg(long, short = 'i')]
        input: Option<PathBuf>,
        /// Output directory (defaults to svelte-package's own default, dist)
        #[arg(long, short = 'o')]
        output: Option<PathBuf>,
        /// Path to a tsconfig/jsconfig file
        #[arg(long)]
        tsconfig: Option<PathBuf>,
        /// Do not emit type declarations (.d.ts)
        #[arg(long)]
        no_types: bool,
        /// Re-expand and repackage everything, ignoring the previous build
        #[arg(long)]
        full_rebuild: bool,
    },
    /// Watch source files and maintain a .macroforge/cache for fast Vite dev mode.
    ///
    /// Expands macros in all TypeScript files and writes results to .macroforge/cache/.
    /// When a file changes, only that file is re-expanded. When the config changes,
    /// all files are re-expanded. Use with `vite dev` for instant macro expansion.
    Watch {
        /// Root directory to watch (defaults to cwd)
        root: Option<PathBuf>,
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
    },
    /// Delete the .macroforge/cache directory and rebuild from scratch.
    ///
    /// Equivalent to manually deleting the cache and running `cache`.
    /// Useful when the cache is corrupted or you want a guaranteed clean state.
    Refresh {
        /// Root directory (defaults to cwd)
        root: Option<PathBuf>,
    },
    /// Build a macro crate to WASM with wasm-bindgen and add $ aliases for Call macros.
    ///
    /// Compiles the crate at the given path (or cwd) to `wasm32-unknown-unknown`,
    /// runs `wasm-bindgen`, then post-processes the output to add `$`-prefixed
    /// re-exports for function-like (Call) macros (e.g. `$state`, `$derived`).
    Build {
        /// Path to the macro crate directory (defaults to cwd)
        crate_dir: Option<PathBuf>,
        /// Output directory for the WASM package (defaults to <crate_dir>/pkg)
        #[arg(long, short = 'o')]
        out: Option<PathBuf>,
    },
}

impl Command {
    /// The subcommand's name, recorded in the project lock so a waiting process
    /// can report what it is waiting for.
    fn label(&self) -> &'static str {
        match self {
            Command::Expand { .. } => "expand",
            Command::Tsc { .. } => "tsc",
            Command::SvelteCheck { .. } => "svelte-check",
            Command::SveltePackage { .. } => "svelte-package",
            Command::Watch { .. } => "watch",
            Command::Cache { .. } => "cache",
            Command::Refresh { .. } => "refresh",
            Command::Build { .. } => "build",
        }
    }

    /// The directory this subcommand operates on, if it names one.
    ///
    /// The cache-oriented subcommands take a project root. `build` takes a
    /// crate directory and writes its WASM package there, so keying on that
    /// rather than the current directory makes two builds of the same crate
    /// serialize no matter where they were invoked from. Everything else works
    /// from the current directory.
    fn explicit_root(&self) -> Option<&PathBuf> {
        match self {
            Command::Watch { root, .. } | Command::Cache { root } | Command::Refresh { root } => {
                root.as_ref()
            }
            Command::Build { crate_dir, .. } => crate_dir.as_ref(),
            _ => None,
        }
    }

    /// Whether this invocation promises a clean stderr.
    ///
    /// `expand --quiet` exists so callers can parse stderr; a message about
    /// waiting on a lock this process happened to contend on would break that.
    fn is_quiet(&self) -> bool {
        matches!(self, Command::Expand { quiet: true, .. })
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // One lock per project, taken here so every subcommand is covered by the
    // same rule. `.macroforge/` — the type registry, the declarative registry,
    // the expansion cache and its manifest — is shared mutable state, and
    // running two macroforge processes against one project is routine
    // (`watch` beside `vite dev`, `svelte-package` from another terminal, an
    // editor invoking `svelte-check`).
    let root = resolve_project_root(cli.command.explicit_root().map(PathBuf::as_path));

    // `watch` is the exception: it is a daemon, so holding the lock for its
    // lifetime would block every other command until the watcher is killed. It
    // locks around each cache-mutation burst instead and runs unlocked while
    // idle — the granularity cargo-watch gets for free by re-invoking a fresh
    // `cargo` per change.
    let _lock = match cli.command {
        Command::Watch { .. } => None,
        _ => Some(ProjectLock::acquire(
            &root,
            cli.command.label(),
            cli.command.is_quiet(),
        )?),
    };

    match cli.command {
        Command::Expand {
            input,
            out,
            types_out,
            print,
            quiet,
            scan,
            include_ignored,
            emit_expanded,
        } => {
            let scan_options = || ScanOptions {
                include_ignored,
                out_dir: out.clone(),
                types_out_dir: types_out.clone(),
                emit_expanded,
            };

            if scan {
                let scan_root = input.unwrap_or_else(|| PathBuf::from("."));
                scan_and_expand(&root, scan_root, scan_options())
            } else {
                let input = input.ok_or_else(|| {
                    anyhow!("input file required (use --scan to scan a directory)")
                })?;

                // If input is a directory, treat it as --scan
                if input.is_dir() {
                    scan_and_expand(&root, input, scan_options())
                } else {
                    if emit_expanded {
                        return Err(anyhow!(
                            "--emit-expanded is only valid with --scan; single-file mode already writes a sibling .expanded file (use --out to redirect)"
                        ));
                    }
                    expand_file(&root, input, out, types_out, print, quiet)
                }
            }
        }
        Command::Tsc { project } => run_tsc_wrapper(&root, project),
        Command::SvelteCheck {
            workspace,
            tsconfig,
            output,
            fail_on_warnings,
        } => run_svelte_check_wrapper(&root, workspace, tsconfig, output, fail_on_warnings),
        Command::SveltePackage {
            input,
            output,
            tsconfig,
            no_types,
            full_rebuild,
        } => run_svelte_package_wrapper(&root, input, output, tsconfig, no_types, full_rebuild),
        Command::Watch { debounce_ms, .. } => run_watch(&root, debounce_ms),
        Command::Cache { .. } => run_cache(&root),
        Command::Refresh { .. } => run_refresh(&root),
        Command::Build { crate_dir, out } => run_build(crate_dir, out),
    }
}
