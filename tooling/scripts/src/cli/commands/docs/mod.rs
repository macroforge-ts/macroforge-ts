//! Documentation commands

pub mod check_freshness;
pub mod extract_rust;
pub mod extract_ts;
pub mod generate_readmes;

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Regenerate the website's API documentation data.
pub fn extract_api_docs(root: &Path) -> Result<()> {
    extract_rust::run(&root.join("website/static/api-data/rust"))
        .context("Failed to extract the Rust API docs")?;
    extract_ts::run(&root.join("website/static/api-data/typescript"))
        .context("Failed to extract the TypeScript API docs")?;
    Ok(())
}

/// Strip the `"generated"` timestamp field from JSON content, so a comparison
/// sees the documentation rather than when it was extracted.
pub(crate) fn strip_generated_field(content: &str) -> String {
    let re = regex::Regex::new(r#"(?m)^\s*"generated"\s*:\s*"[^"]*",?\s*\n?"#).unwrap();
    re.replace_all(content, "").to_string()
}

/// Write generated JSON, leaving the file untouched when only its timestamp moved.
///
/// Every extraction stamps the current time, so writing unconditionally dirties
/// the whole docs tree on every run whether or not a single doc comment
/// changed. That buries real documentation changes and leaves a failed release
/// run with a tree full of edits it did not actually make.
///
/// Compared as parsed values rather than as text: these files are written here
/// with two-space indentation and then reformatted by `deno fmt` to the repo's
/// four, so a textual comparison never matches and the two keep overwriting
/// each other.
pub(crate) fn write_docs_json(path: &Path, json: &str) -> Result<()> {
    if let Ok(existing) = fs::read_to_string(path)
        && documentation_of(&existing) == documentation_of(json)
    {
        return Ok(());
    }
    // The formatter adds the trailing newline, so emitting it here keeps the
    // generator and `deno fmt` from rewriting each other's output.
    fs::write(path, format!("{json}\n"))?;
    Ok(())
}

/// A docs file's content with the generation stamp dropped, or `None` when it
/// does not parse.
fn documentation_of(text: &str) -> Option<serde_json::Value> {
    let mut value: serde_json::Value = serde_json::from_str(text).ok()?;
    if let Some(object) = value.as_object_mut() {
        object.remove("generated");
    }
    Some(value)
}
