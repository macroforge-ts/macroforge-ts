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

    /// Verify a release: build, check, test and regenerate the docs
    Verify(VerifyArgs),

    /// Bump release versions and everything stamped with them
    Bump(BumpArgs),

    /// Manifest manipulation (versions, dependencies)
    Manifest(ManifestArgs),

    /// Fetch latest versions from npm/crates.io and sync them to disk
    ///
    /// Rewrites versions.json, package.json/Cargo.toml manifests, and the Zed
    /// extension version constants unless --check-only is passed.
    Versions(VersionsArgs),

    /// Run comprehensive multi-tool diagnostics
    Diagnostics(DiagnosticsArgs),

    /// Clean build packages
    Build(BuildArgs),

    /// Documentation generation and management
    Docs(DocsArgs),

    /// Run tests for packages
    Test(TestArgs),

    /// Publish all packages from local machine
    #[command(name = "publish-local")]
    PublishLocal(PublishLocalArgs),

    /// Tag and push the monorepo
    Push(PushArgs),
}

#[derive(clap::Args)]
pub struct VerifyArgs {
    /// Skip the install, build, test and extension steps
    #[arg(long)]
    pub skip_build: bool,

    /// Skip the documentation steps
    #[arg(long)]
    pub skip_docs: bool,
}

#[derive(clap::Args)]
pub struct BumpArgs {
    /// Repos to bump (comma-separated, or 'all', 'rust', 'ts')
    #[arg(default_value = "all")]
    pub repos: String,

    /// Version to set (e.g. 0.2.1); increments the patch version if omitted
    #[arg(long)]
    pub version: Option<String>,

    /// Move every selected package to one shared version
    #[arg(long)]
    pub sync_versions: bool,

    /// Don't cascade the bump to dependents
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

    /// Dump all versions as JSON
    DumpVersions,

    /// Update Zed extension files
    UpdateZed,
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

    /// Only run specific tools (comma-separated: deno-lint,clippy,tsc,svelte)
    #[arg(long)]
    pub tools: Option<String>,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,

    /// Skip formatting (deno fmt for JS/TS, cargo fmt for Rust)
    #[arg(long)]
    pub no_format: bool,
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

    /// Generate README.md files
    GenerateReadmes,

    /// Check if documentation is up to date
    CheckFreshness,

    /// Run API extraction (Rust + TypeScript) and README generation
    All,
}

#[derive(clap::Args)]
pub struct TestArgs {
    /// Test suite to run: 'rust', 'packages', 'playground', or 'all'
    #[arg(default_value = "all")]
    pub suite: String,
}

#[derive(clap::Args)]
pub struct PublishLocalArgs {
    /// Skip the WASM build step (`deno task build:wasm`; package already built)
    #[arg(long)]
    pub skip_build: bool,

    /// Print what would be done without actually publishing
    #[arg(long)]
    pub dry_run: bool,

    /// Skip confirmation prompts
    #[arg(short = 'y', long)]
    pub yes: bool,
}

#[derive(clap::Args)]
pub struct PushArgs {
    /// Skip confirmation prompts
    #[arg(short = 'y', long)]
    pub yes: bool,

    /// Dry run - show what would be done
    #[arg(long)]
    pub dry_run: bool,
}
