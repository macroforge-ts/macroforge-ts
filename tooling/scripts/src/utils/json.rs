//! Serializing JSON the way the repository formats it.
//!
//! `serde_json::to_string_pretty` indents with two spaces, and `deno fmt`
//! reindents this tree to four. A file written one way and formatted the other
//! is rewritten by whichever ran last, so every generator fought the formatter
//! and both kept "fixing" the same files. Writing the formatter's shape in the
//! first place leaves it nothing to do.

use anyhow::{Context, Result};
use serde::Serialize;

/// Indentation `deno fmt` produces for this repository.
const INDENT: &[u8] = b"    ";

/// Serialize `value` as pretty JSON indented the way the formatter wants it.
///
/// No trailing newline: callers that write to a file add one, and callers that
/// print to stdout do not want one baked in.
pub fn to_string_pretty<T: Serialize>(value: &T) -> Result<String> {
    let mut out = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(INDENT);
    let mut serializer = serde_json::Serializer::with_formatter(&mut out, formatter);
    value
        .serialize(&mut serializer)
        .context("failed to serialize JSON")?;
    String::from_utf8(out).context("serialized JSON was not UTF-8")
}
