//! Documentation commands

pub mod check_freshness;
pub mod extract_mcp;
pub mod extract_rust;
pub mod extract_ts;
pub mod generate_readmes;
pub mod generated;

use crate::core::shell;
use anyhow::{Context, Result};
use generated::GeneratedFiles;
use std::path::Path;

/// Regenerate the website's API documentation data.
pub fn extract_api_docs(root: &Path) -> Result<()> {
    let mut files = extract_rust::generate(root)?;
    files.extend(extract_ts::generate(root)?);
    files.write(root)?;
    Ok(())
}

/// Regenerate everything derived from the API data, which must be current:
/// the READMEs, then the website build and the MCP docs extracted from it.
pub fn extract_derived_docs(root: &Path) -> Result<()> {
    generate_readmes::generate(root)?.write(root)?;
    build_website(root)?;
    extract_mcp::generate(root)?.write(root)?;
    Ok(())
}

/// Every generated documentation file. The MCP docs are extracted from the
/// website build, which must be current.
pub fn all_generated(root: &Path) -> Result<GeneratedFiles> {
    let mut files = extract_rust::generate(root)?;
    files.extend(extract_ts::generate(root)?);
    files.extend(generate_readmes::generate(root)?);
    files.extend(extract_mcp::generate(root)?);
    Ok(files)
}

/// Build the website, whose prerendered pages the MCP docs are extracted from.
pub fn build_website(root: &Path) -> Result<()> {
    shell::deno::task_inherit(&root.join("website"), "build")
        .context("the website build failed")?;
    Ok(())
}
