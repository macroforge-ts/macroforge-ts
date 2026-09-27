//! CLI argument definitions using clap

use clap::{Parser, Subcommand};

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

    /// Tag main at the release version and push the tag, which CI publishes
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

    /// Check that the generated docs match their sources instead of
    /// regenerating them, so the tree is left untouched. For CI.
    #[arg(long, conflicts_with = "skip_docs")]
    pub check: bool,
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
    /// Extract Rust documentation to JSON and the builtin macro pages
    ExtractRust,

    /// Extract TypeScript documentation to JSON
    ExtractTs,

    /// Generate README.md files from the extracted JSON
    GenerateReadmes,

    /// Extract the built website's pages into the MCP server's docs
    ExtractMcp,

    /// Check that every generated doc matches its sources, writing nothing
    CheckFreshness,

    /// Regenerate every doc: API data, READMEs, and the MCP docs from a
    /// fresh website build
    All,
}

#[derive(clap::Args)]
pub struct TestArgs {
    /// Test suite to run: 'rust', 'packages', 'playground', or 'all'
    #[arg(default_value = "all")]
    pub suite: String,
}

/// A registry `publish-local` publishes to.
#[derive(Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Registry {
    Crates,
    Npm,
    Jsr,
}

#[derive(clap::Args)]
pub struct PublishLocalArgs {
    /// Publish to this registry only; repeat for several. All of them when
    /// omitted.
    #[arg(long = "registry", value_enum)]
    pub registries: Vec<Registry>,

    /// Build what the selected registries' unpublished packages need and
    /// publish nothing, so another machine can publish them with
    /// `--skip-build`.
    #[arg(long, conflicts_with = "skip_build")]
    pub build_only: bool,

    /// Skip the WASM and npm package builds (already built)
    #[arg(long)]
    pub skip_build: bool,

    /// Print what would be done without actually publishing
    #[arg(long)]
    pub dry_run: bool,

    /// Skip confirmation prompts
    #[arg(short = 'y', long)]
    pub yes: bool,

    /// Never prompt or log in: credentials come from the environment (npm's
    /// config or a trusted publishing `NPM_ID_TOKEN`, `CARGO_REGISTRY_TOKEN`
    /// or a trusted publishing `CRATES_IO_ID_TOKEN`, `JSR_TOKEN`) and an auth
    /// failure is an error.
    /// Implies `--yes`. For CI.
    #[arg(long)]
    pub non_interactive: bool,
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
