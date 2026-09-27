//! Check documentation freshness
//!
//! Runs every docs generator without writing and compares its output with the
//! committed files. Each generator reads committed inputs, so a stale file is
//! reported at the first generator whose output differs.

use crate::cli::commands::docs::generated::Staleness;
use crate::cli::commands::docs::{all_generated, build_website};
use crate::core::config::Config;
use crate::utils::format;
use anyhow::Result;
use std::path::Path;

/// Entry point for `mf docs check-freshness`: builds the website the MCP docs
/// are extracted from, then checks every generated file.
pub fn run() -> Result<()> {
    let config = Config::load()?;
    build_website(&config.root)?;
    check(&config.root)
}

/// Fails, naming each file, when a committed generated file is missing,
/// differs from its generator's output, or is no longer generated. Expects the
/// website to be built.
pub fn check(root: &Path) -> Result<()> {
    let stale = all_generated(root)?.stale(root)?;

    println!();
    if stale.is_empty() {
        format::success("The generated documentation is current");
        return Ok(());
    }
    for file in &stale {
        let reason = match file.staleness {
            Staleness::Missing => "missing",
            Staleness::Outdated => "outdated",
            Staleness::Orphaned => "no longer generated",
        };
        format::error(&format!("{} ({reason})", file.path.display()));
    }
    let count = match stale.len() {
        1 => "1 generated documentation file is".to_string(),
        count => format!("{count} generated documentation files are"),
    };
    anyhow::bail!("{count} stale; run `pixi run docs:all` and commit the result")
}
