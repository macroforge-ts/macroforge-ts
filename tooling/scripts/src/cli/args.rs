//! CLI argument definitions using clap

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "mf")]
#[command(about = "Macroforge Tooling - unified CLI for build, release, and diagnostics")]
#[command(version)]
pub struct Cli {
    /// Enable TUI mode (dashboard interface)
    #[arg(long, global = true)]
    pub tui: bool,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Enable debug logging
    #[arg(long, global = true)]
    pub debug: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Interactive TUI dashboard
    Tui,

    /// Prepare a release (bump versions, build, test, docs)
    Prep(PrepArgs),

    /// Commit queue for releasing packages in dependency order
    Commit(CommitArgs),

    /// Manifest manipulation (versions, dependencies)
    Manifest(ManifestArgs),

    /// Fetch latest versions from npm/crates.io
    Versions(VersionsArgs),

    /// Run comprehensive multi-tool diagnostics
    Diagnostics(DiagnosticsArgs),

    /// Clean build packages
    Build(BuildArgs),

    /// Documentation generation and management
    Docs(DocsArgs),

    /// Expand macros in playground files
    Expand(ExpandArgs),

    /// Get diagnostics for a single file
    Check(CheckArgs),
}

#[derive(clap::Args)]
pub struct PrepArgs {
    /// Repos to update (comma-separated, or 'all', 'rust', 'ts')
    #[arg(default_value = "all")]
    pub repos: String,

    /// Version string (e.g., 0.1.4). Auto-increments if not provided
    #[arg(long)]
    pub version: Option<String>,

    /// Dry run - show what would be done
    #[arg(long)]
    pub dry_run: bool,

    /// Skip build and test steps
    #[arg(long)]
    pub skip_build: bool,

    /// Skip documentation steps
    #[arg(long)]
    pub skip_docs: bool,

    /// Only bump versions (skip build, test, docs)
    #[arg(long)]
    pub bump_only: bool,

    /// Sync all packages to their latest versions
    #[arg(long)]
    pub sync_versions: bool,

    /// Don't cascade version bumps to dependents
    #[arg(long)]
    pub no_cascade: bool,
}

#[derive(clap::Args)]
pub struct CommitArgs {
    /// Repos to commit (comma-separated, or 'all', 'rust', 'ts')
    #[arg(short, long, default_value = "all")]
    pub repos: String,

    /// Skip confirmation prompts
    #[arg(short = 'y', long)]
    pub yes: bool,

    /// Dry run - show what would be done
    #[arg(long)]
    pub dry_run: bool,

    /// Custom commit message
    #[arg(short, long)]
    pub message: Option<String>,

    /// Don't cascade to dependent packages
    #[arg(long)]
    pub no_cascade: bool,
}

#[derive(clap::Args)]
pub struct ManifestArgs {
    #[command(subcommand)]
    pub command: ManifestCommands,
}

#[derive(Subcommand)]
pub enum ManifestCommands {
    /// List all repositories as JSON
    List,

    /// Get version for a repo
    GetVersion {
        repo: String,
        #[arg(long)]
        registry: bool,
    },

    /// Set version for a repo
    SetVersion {
        repo: String,
        version: String,
        #[arg(long)]
        registry: bool,
    },

    /// Apply versions from cache to all files
    ApplyVersions {
        #[arg(long)]
        local: bool,
    },

    /// Swap dependencies to local paths
    SwapLocal,

    /// Swap dependencies to registry versions
    SwapRegistry,

    /// Dump all versions as JSON
    DumpVersions,

    /// Update Zed extension files
    UpdateZed,

    /// Link local dependencies
    LinkLocal { repo: String, deps: Vec<String> },

    /// Restore repo dependencies
    RestoreRepo { repo: String },
}

#[derive(clap::Args)]
pub struct VersionsArgs {
    /// Only check versions, don't update
    #[arg(long)]
    pub check_only: bool,
}

#[derive(clap::Args)]
pub struct DiagnosticsArgs {
    /// Write logs to .tmp/diagnostics/ directory
    #[arg(long)]
    pub log: bool,

    /// Only run specific tools (comma-separated: biome,clippy,tsc,svelte)
    #[arg(long)]
    pub tools: Option<String>,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

#[derive(clap::Args)]
pub struct BuildArgs {
    /// Repos to build (comma-separated, or 'all', 'rust', 'ts')
    #[arg(short, long, default_value = "all")]
    pub repos: String,
}

#[derive(clap::Args)]
pub struct DocsArgs {
    #[command(subcommand)]
    pub command: DocsCommands,
}

#[derive(Subcommand)]
pub enum DocsCommands {
    /// Extract Rust documentation to JSON
    ExtractRust {
        /// Output directory for JSON files
        #[arg(long, default_value = "website/static/api-data/rust")]
        output_dir: PathBuf,
    },

    /// Extract TypeScript documentation to JSON
    ExtractTs {
        /// Output directory for JSON files
        #[arg(long, default_value = "website/static/api-data/typescript")]
        output_dir: PathBuf,
    },

    /// Build markdown documentation book
    BuildBook {
        /// Output file path
        #[arg(long, default_value = "docs/BOOK.md")]
        output_path: PathBuf,
    },

    /// Generate README.md files
    GenerateReadmes,

    /// Check if documentation is up to date (CI)
    CheckFreshness,

    /// Run all documentation generation steps
    All,
}

#[derive(clap::Args)]
pub struct ExpandArgs {
    /// Use CLI binary instead of Node.js API
    #[arg(long)]
    pub use_cli: bool,

    /// Specific directory to expand
    pub path: Option<PathBuf>,
}

#[derive(clap::Args)]
pub struct CheckArgs {
    /// File to check for diagnostics
    pub file: PathBuf,
}
