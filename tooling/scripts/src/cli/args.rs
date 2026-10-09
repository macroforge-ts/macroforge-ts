//! CLI argument definitions using clap

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mf")]
#[command(about = "Macroforge tooling: build, verify, document and release the monorepo")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Verify a release: build, check, test and regenerate the docs
    Verify(VerifyArgs),

    /// Set the release version every package carries
    Bump(BumpArgs),

    /// Run comprehensive multi-tool diagnostics
    Diagnostics(DiagnosticsArgs),

    /// Documentation generation and management
    Docs(DocsArgs),

    /// Run tests for packages
    Test(TestArgs),

    /// Publish all packages from local machine
    #[command(name = "publish-local")]
    PublishLocal(PublishLocalArgs),

    /// Tag main at the release version and push the tag, which CI publishes
    Tag(TagArgs),
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
    /// Version to set (e.g. 0.4.0); increments the patch version if omitted
    #[arg(long)]
    pub version: Option<String>,
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

    /// Report only: skip formatting (deno fmt, cargo fmt) and lint fixes
    /// (deno lint --fix, cargo clippy --fix)
    #[arg(long)]
    pub no_fix: bool,
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
    /// Test suite to run: 'rust', 'packages', 'testground', or 'all'
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
pub struct TagArgs {
    /// Skip confirmation prompts
    #[arg(short = 'y', long)]
    pub yes: bool,

    /// Dry run - show what would be done
    #[arg(long)]
    pub dry_run: bool,

    /// Move an existing tag to HEAD, on origin too, which reruns its release
    /// pipeline
    #[arg(long)]
    pub retag: bool,
}
