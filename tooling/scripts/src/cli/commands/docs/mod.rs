//! Documentation commands

pub mod build_book;
pub mod check_freshness;
pub mod extract_rust;
pub mod extract_ts;
pub mod generate_readmes;

use anyhow::Result;
use std::fs;
use std::path::Path;

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
pub(crate) fn write_docs_json(path: &Path, json: &str) -> Result<()> {
    if let Ok(existing) = fs::read_to_string(path)
        && strip_generated_field(&existing) == strip_generated_field(json)
    {
        return Ok(());
    }
    fs::write(path, json)?;
    Ok(())
}
