//! Generate README.md files from extracted API documentation JSON
//!
//! Creates rich README files for both Rust crates and TypeScript packages by
//! reading the extracted JSON docs and including badges, overview text, key
//! exports grouped by kind, first code example, and API reference links.

use crate::core::config::Config;
use crate::core::manifests;
use crate::core::shell;
use crate::utils::format;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

// ── Package / crate definitions ─────────────────────────────────────────────

/// TypeScript packages: (json_name, relative_path, npm_name)
const TS_PACKAGES: &[(&str, &str, &str)] = &[
    ("shared", "packages/shared", "@macroforge/shared"),
    (
        "vite-plugin",
        "packages/vite-plugin",
        "@macroforge/vite-plugin",
    ),
    (
        "typescript-plugin",
        "packages/typescript-plugin",
        "@macroforge/typescript-plugin",
    ),
    (
        "svelte-language-server",
        "packages/svelte-language-server",
        "@macroforge/svelte-language-server",
    ),
    (
        "svelte-preprocessor",
        "packages/svelte-preprocessor",
        "@macroforge/svelte-preprocessor",
    ),
    (
        "mcp-server",
        "packages/mcp-server",
        "@macroforge/mcp-server",
    ),
    (
        "deno-plugin",
        "packages/deno-plugin",
        "@macroforge/deno-plugin",
    ),
];

/// Rust crates: (json_name/crate_name, relative_path)
const RUST_CRATES: &[(&str, &str)] = &[
    ("macroforge_ts", "crates/macroforge_ts"),
    ("macroforge_ts_syn", "crates/macroforge_ts_syn"),
    ("macroforge_ts_quote", "crates/macroforge_ts_quote"),
    ("macroforge_ts_macros", "crates/macroforge_ts_macros"),
];

// ── Public entry points ─────────────────────────────────────────────────────

/// Generate README files in-place (used by `mf docs generate-readmes`).
pub fn run() -> Result<()> {
    let config = Config::load()?;

    format::header("Generating README Files");

    let mut generated = 0;

    // Rust crates
    for (crate_name, crate_path) in RUST_CRATES {
        let crate_dir = config.root.join(crate_path);
        if !crate_dir.exists() {
            format::warning(&format!("Crate not found: {}", crate_path));
            continue;
        }

        print!("Generating README for {}... ", crate_name);
        let readme = shell::deno::format_markdown(
            &config.root,
            &generate_rust_readme(&config.root, crate_name, crate_path)?,
        )?;
        fs::write(crate_dir.join("README.md"), readme)?;
        println!("done");
        generated += 1;
    }

    // TypeScript packages
    for (json_name, pkg_path, npm_name) in TS_PACKAGES {
        let pkg_dir = config.root.join(pkg_path);
        if !pkg_dir.exists() {
            format::warning(&format!("Package not found: {}", pkg_path));
            continue;
        }

        print!("Generating README for {}... ", json_name);
        let readme = shell::deno::format_markdown(
            &config.root,
            &generate_ts_readme(&config.root, json_name, npm_name, pkg_path)?,
        )?;
        fs::write(pkg_dir.join("README.md"), readme)?;
        println!("done");
        generated += 1;
    }

    println!();
    format::success(&format!("Generated {} README files", generated));

    Ok(())
}

/// Generate all READMEs into a given output directory tree, mirroring the
/// project layout.  Used by `check_freshness` to compare without touching the
/// real working tree.
pub fn generate_all_to(root: &Path, out_root: &Path) -> Result<()> {
    for (crate_name, crate_path) in RUST_CRATES {
        let crate_dir = root.join(crate_path);
        if !crate_dir.exists() {
            continue;
        }
        let dest = out_root.join(crate_path);
        fs::create_dir_all(&dest)?;
        let readme = shell::deno::format_markdown(
            root,
            &generate_rust_readme(root, crate_name, crate_path)?,
        )?;
        fs::write(dest.join("README.md"), readme)?;
    }

    for (json_name, pkg_path, npm_name) in TS_PACKAGES {
        let pkg_dir = root.join(pkg_path);
        if !pkg_dir.exists() {
            continue;
        }
        let dest = out_root.join(pkg_path);
        fs::create_dir_all(&dest)?;
        let readme = shell::deno::format_markdown(
            root,
            &generate_ts_readme(root, json_name, npm_name, pkg_path)?,
        )?;
        fs::write(dest.join("README.md"), readme)?;
    }

    Ok(())
}

// ── Rust README generation ──────────────────────────────────────────────────

fn generate_rust_readme(root: &Path, crate_name: &str, crate_path: &str) -> Result<String> {
    let json_path = root.join(format!("website/static/api-data/rust/{}.json", crate_name));

    let mut out = String::new();

    // Title
    out.push_str(&format!("# {}\n\n", crate_name));

    // Try to load extracted JSON docs
    let doc: Option<serde_json::Value> = if json_path.exists() {
        let raw = fs::read_to_string(&json_path)
            .with_context(|| format!("failed to read {}", json_path.display()))?;
        Some(
            serde_json::from_str(&raw)
                .with_context(|| format!("{} is not valid JSON", json_path.display()))?,
        )
    } else {
        None
    };

    let description = doc
        .as_ref()
        .and_then(|d| d.get("description"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if !description.is_empty() {
        out.push_str(&format!("{}\n\n", description));
    }

    // Badges
    out.push_str(&format!(
        "[![Crates.io](https://img.shields.io/crates/v/{crate_name}.svg)](https://crates.io/crates/{crate_name})\n"
    ));
    out.push_str(&format!(
        "[![Documentation](https://docs.rs/{crate_name}/badge.svg)](https://docs.rs/{crate_name})\n\n"
    ));

    // Overview (from module docs)
    let overview = doc
        .as_ref()
        .and_then(|d| d.get("overview"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if !overview.is_empty() {
        // The overview often starts with "# crate_name\n\n..." — strip the
        // leading heading so it doesn't duplicate the title we already wrote.
        let overview_trimmed = strip_leading_heading(overview);
        // Also strip repeated description line if it duplicates what we wrote
        let overview_clean = strip_leading_line_if_matches(&overview_trimmed, description);
        if !overview_clean.trim().is_empty() {
            out.push_str("## Overview\n\n");
            out.push_str(overview_clean.trim());
            out.push_str("\n\n");
        }
    }

    // Installation. A crate directory that also ships a JS package (the
    // engine's @macroforge/core) lists that first.
    out.push_str("## Installation\n\n");
    push_js_install(&mut out, &root.join(crate_path))?;
    out.push_str(&format!("```bash\ncargo add {crate_name}\n```\n\n"));

    // Key Exports grouped by kind
    if let Some(items) = doc
        .as_ref()
        .and_then(|d| d.get("items"))
        .and_then(|v| v.as_array())
    {
        let groups = group_items_by_kind(items);
        if !groups.is_empty() {
            out.push_str("## Key Exports\n\n");
            for (kind_label, entries) in &groups {
                out.push_str(&format!("### {}\n\n", kind_label));
                let display_limit = 10;
                let shown = entries.len().min(display_limit);
                for entry in entries.iter().take(display_limit) {
                    out.push_str(&format!(
                        "- **`{}`** - {}\n",
                        entry.0,
                        first_sentence(&entry.1)
                    ));
                }
                if entries.len() > display_limit {
                    out.push_str(&format!("- ... and {} more\n", entries.len() - shown));
                }
                out.push('\n');
            }
        }

        // First code example from items
        if let Some(example) = find_first_code_example(items) {
            out.push_str("## Example\n\n");
            out.push_str(&example);
            out.push_str("\n\n");
        }
    }

    // API Reference link
    out.push_str("## API Reference\n\n");
    out.push_str(&format!(
        "See the [full API documentation](https://docs.rs/{crate_name}) on docs.rs.\n\n"
    ));

    // License
    out.push_str("## License\n\nMIT\n");

    Ok(link_intra_doc_references(&out, crate_name))
}

// ── TypeScript README generation ────────────────────────────────────────────

fn generate_ts_readme(
    root: &Path,
    json_name: &str,
    npm_name: &str,
    pkg_path: &str,
) -> Result<String> {
    let json_path = root.join(format!(
        "website/static/api-data/typescript/{}.json",
        json_name
    ));

    let mut out = String::new();

    // Title
    out.push_str(&format!("# {}\n\n", npm_name));

    // Try to load extracted JSON docs
    let doc: Option<serde_json::Value> = if json_path.exists() {
        let raw = fs::read_to_string(&json_path)
            .with_context(|| format!("failed to read {}", json_path.display()))?;
        Some(
            serde_json::from_str(&raw)
                .with_context(|| format!("{} is not valid JSON", json_path.display()))?,
        )
    } else {
        None
    };

    // Fall back to package.json for description if JSON doc is empty
    let pkg_json_path = root.join(pkg_path).join("package.json");
    let pkg_json: Option<serde_json::Value> = if pkg_json_path.exists() {
        let raw = fs::read_to_string(&pkg_json_path)
            .with_context(|| format!("failed to read {}", pkg_json_path.display()))?;
        Some(
            serde_json::from_str(&raw)
                .with_context(|| format!("{} is not valid JSON", pkg_json_path.display()))?,
        )
    } else {
        None
    };

    let description = doc
        .as_ref()
        .and_then(|d| d.get("description"))
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .or_else(|| {
            pkg_json
                .as_ref()
                .and_then(|p| p.get("description"))
                .and_then(|v| v.as_str())
        })
        .unwrap_or("");

    // Badges for the registries the package is published to.
    let package_dir = root.join(pkg_path);
    let jsr_package = manifests::jsr_package_name(&package_dir)?;
    if let Some(npm) = manifests::npm_package_name(&package_dir)? {
        let encoded = npm.replace('/', "%2F").replace('@', "%40");
        out.push_str(&format!(
            "[![npm version](https://badge.fury.io/js/{encoded}.svg)](https://www.npmjs.com/package/{npm})\n"
        ));
    }
    if let Some(jsr) = &jsr_package {
        out.push_str(&format!(
            "[![JSR](https://jsr.io/badges/{jsr})](https://jsr.io/{jsr})\n"
        ));
    }
    out.push('\n');

    // Overview
    if !description.is_empty() {
        out.push_str("## Overview\n\n");
        out.push_str(&render_jsdoc_links(description));
        out.push_str("\n\n");
    }

    // Installation
    out.push_str("## Installation\n\n");
    push_js_install(&mut out, &package_dir)?;

    // API section — exports grouped by kind
    if let Some(exports) = doc
        .as_ref()
        .and_then(|d| d.get("exports"))
        .and_then(|v| v.as_array())
        && !exports.is_empty()
    {
        let groups = group_exports_by_kind(exports);
        if !groups.is_empty() {
            out.push_str("## API\n\n");
            for (kind_label, entries) in &groups {
                out.push_str(&format!("### {}\n\n", kind_label));
                for entry in entries {
                    let desc_short = first_sentence(&entry.1);
                    if desc_short.is_empty() {
                        out.push_str(&format!("- **`{}`**\n", entry.0));
                    } else {
                        out.push_str(&format!("- **`{}`** - {}\n", entry.0, desc_short));
                    }
                }
                out.push('\n');
            }
        }

        // First code example from exports
        if let Some(example) = find_first_ts_code_example(exports) {
            out.push_str("## Examples\n\n");
            out.push_str(&example);
            out.push_str("\n\n");
        }
    }

    // Documentation link
    out.push_str("## Documentation\n\n");
    match (&jsr_package, manifests::npm_package_name(&package_dir)?) {
        (Some(jsr), _) => out.push_str(&format!(
            "See the [full API documentation](https://jsr.io/{jsr}/doc) on JSR.\n\n"
        )),
        (None, Some(npm)) => out.push_str(&format!(
            "See the [package on npm](https://www.npmjs.com/package/{npm}).\n\n"
        )),
        (None, None) => {}
    }

    // License
    out.push_str("## License\n\nMIT\n");

    Ok(out)
}

// ── Helpers ─────────────────────────────────────────────────────────────────

/// Group Rust items by kind into (display_label, Vec<(name, description)>).
/// Order: Structs, Functions, Enums, Traits, then everything else.
fn group_items_by_kind(items: &[serde_json::Value]) -> Vec<(String, Vec<(String, String)>)> {
    let kind_order = ["struct", "function", "enum", "trait"];
    let kind_labels: std::collections::HashMap<&str, &str> = [
        ("struct", "Structs"),
        ("function", "Functions"),
        ("enum", "Enums"),
        ("trait", "Traits"),
    ]
    .into_iter()
    .collect();

    let mut buckets: std::collections::HashMap<String, Vec<(String, String)>> =
        std::collections::HashMap::new();

    for item in items {
        let kind = item.get("kind").and_then(|v| v.as_str()).unwrap_or("other");
        let name = item
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let desc = item
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        buckets
            .entry(kind.to_string())
            .or_default()
            .push((name, desc));
    }

    let mut result = Vec::new();
    for kind in &kind_order {
        if let Some(entries) = buckets.remove(*kind)
            && !entries.is_empty()
        {
            let label = kind_labels.get(kind).unwrap_or(kind);
            result.push((label.to_string(), entries));
        }
    }
    // Any remaining kinds
    let mut remaining: Vec<_> = buckets.into_iter().filter(|(_, v)| !v.is_empty()).collect();
    remaining.sort_by(|a, b| a.0.cmp(&b.0));
    for (kind, entries) in remaining {
        let label = format!("{}s", capitalize(&kind));
        result.push((label, entries));
    }

    result
}

/// Group TypeScript exports by kind into (display_label, Vec<(name, description)>).
/// Order: Functions, Classes, Types, then everything else.
fn group_exports_by_kind(exports: &[serde_json::Value]) -> Vec<(String, Vec<(String, String)>)> {
    let kind_order = ["function", "class", "interface", "type", "const"];
    let kind_labels: std::collections::HashMap<&str, &str> = [
        ("function", "Functions"),
        ("class", "Classes"),
        ("interface", "Interfaces"),
        ("type", "Types"),
        ("const", "Constants"),
    ]
    .into_iter()
    .collect();

    let mut buckets: std::collections::HashMap<String, Vec<(String, String)>> =
        std::collections::HashMap::new();

    for exp in exports {
        let kind = exp.get("kind").and_then(|v| v.as_str()).unwrap_or("other");
        let name = exp
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let desc = exp
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        buckets
            .entry(kind.to_string())
            .or_default()
            .push((name, desc));
    }

    let mut result = Vec::new();
    for kind in &kind_order {
        if let Some(entries) = buckets.remove(*kind)
            && !entries.is_empty()
        {
            let label = kind_labels.get(kind).unwrap_or(kind);
            result.push((label.to_string(), entries));
        }
    }
    let mut remaining: Vec<_> = buckets.into_iter().filter(|(_, v)| !v.is_empty()).collect();
    remaining.sort_by(|a, b| a.0.cmp(&b.0));
    for (kind, entries) in remaining {
        let label = format!("{}s", capitalize(&kind));
        result.push((label, entries));
    }

    result
}

/// Render JSDoc `{@link X}`, `{@link X|label}` and `{@linkcode X}` tags as
/// inline code: a README has no symbol to link them to.
fn render_jsdoc_links(text: &str) -> String {
    static LINK: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"\{@link(?:code|plain)?\s+([^}|\s]+)(?:\s*\|\s*([^}]+))?\}")
            .expect("the JSDoc link pattern is valid")
    });
    LINK.replace_all(text, |captures: &regex::Captures| {
        let shown = captures
            .get(2)
            .map_or(&captures[1], |label| label.as_str().trim());
        format!("`{shown}`")
    })
    .into_owned()
}

/// Extract the first sentence from a doc string (up to the first period
/// followed by whitespace or end-of-string, or up to the first newline).
fn first_sentence(text: &str) -> String {
    let text = render_jsdoc_links(text);
    let text = text.trim();
    if text.is_empty() {
        return String::new();
    }
    // Find first sentence boundary
    if let Some(pos) = text.find(".\n") {
        return text[..=pos].trim().to_string();
    }
    if let Some(pos) = text.find(". ") {
        return text[..=pos].trim().to_string();
    }
    // Fall back to first line, truncated
    let first_line = text.lines().next().unwrap_or(text);
    if first_line.len() > 120 {
        format!("{}...", &first_line[..117])
    } else {
        first_line.to_string()
    }
}

/// Find the first ```rust or ```typescript code example in item descriptions.
fn find_first_code_example(items: &[serde_json::Value]) -> Option<String> {
    let fence_re = regex::Regex::new(r"(?s)```(?:rust|rust,ignore)\n(.*?)```").ok()?;
    for item in items {
        let desc = item.get("description").and_then(|v| v.as_str())?;
        if let Some(cap) = fence_re.captures(desc) {
            let code = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            return Some(format!("```rust\n{}```", code));
        }
    }
    None
}

/// Find the first ```typescript code example in export descriptions.
fn find_first_ts_code_example(exports: &[serde_json::Value]) -> Option<String> {
    let fence_re = regex::Regex::new(r"(?s)```(?:typescript|ts)\n(.*?)```").ok()?;
    for exp in exports {
        let desc = exp
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if let Some(cap) = fence_re.captures(desc) {
            let code = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            return Some(format!("```typescript\n{}```", code));
        }
    }
    None
}

/// Strip a leading markdown heading (# ...) from text.
fn strip_leading_heading(text: &str) -> String {
    let trimmed = text.trim_start();
    if trimmed.starts_with('#') {
        // Remove the first line
        if let Some(pos) = trimmed.find('\n') {
            return trimmed[pos + 1..].trim_start_matches('\n').to_string();
        }
        return String::new();
    }
    text.to_string()
}

/// If the first non-empty line of `text` matches `line_to_strip`, remove it.
fn strip_leading_line_if_matches<'a>(text: &'a str, line_to_strip: &str) -> &'a str {
    if line_to_strip.is_empty() {
        return text;
    }
    let trimmed = text.trim_start();
    if let Some(first_line) = trimmed.lines().next()
        && first_line.trim() == line_to_strip.trim()
    {
        let after = &trimmed[first_line.len()..];
        return after.trim_start_matches('\n');
    }
    text
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
    }
}

/// Copyable install commands, one block each, for the JS package a directory
/// publishes: `deno add` from JSR first, then `npm install` when it is on npm.
fn push_js_install(out: &mut String, package_dir: &Path) -> Result<()> {
    if let Some(jsr) = manifests::jsr_package_name(package_dir)? {
        out.push_str(&format!("```bash\ndeno add jsr:{jsr}\n```\n\n"));
    }
    if let Some(npm) = manifests::npm_package_name(package_dir)? {
        out.push_str(&format!("```bash\nnpm install {npm}\n```\n\n"));
    }
    Ok(())
}

/// Rewrites rustdoc intra-doc links (`` [`Expr`](swc_core::ecma::ast::Expr) ``
/// and the shortcut `` [`ts_quote!`] ``) into docs.rs links, since a README is
/// plain markdown and would read their targets as relative file paths. Code
/// fences are left untouched.
fn link_intra_doc_references(markdown: &str, crate_name: &str) -> String {
    let mut out = String::with_capacity(markdown.len());
    let mut in_fence = false;
    for line in markdown.split_inclusive('\n') {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
        }
        if in_fence {
            out.push_str(line);
        } else {
            out.push_str(&link_intra_doc_line(line, crate_name));
        }
    }
    out
}

fn link_intra_doc_line(line: &str, crate_name: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut rest = line;
    while let Some(open) = rest.find("[`") {
        let Some(close_rel) = rest[open + 2..].find("`]") else {
            break;
        };
        let label_end = open + 2 + close_rel;
        let label = &rest[open + 2..label_end];
        let after = &rest[label_end + 2..];
        let (target, consumed) = match after.strip_prefix('(').and_then(|inner| inner.find(')')) {
            Some(target_end) => (Some(&after[1..=target_end]), target_end + 2),
            None => (None, 0),
        };
        out.push_str(&rest[..open]);
        let path = target.unwrap_or(label);
        if is_rust_path(path) {
            out.push_str(&format!(
                "[`{label}`]({})",
                docs_rs_search_url(path, crate_name)
            ));
        } else {
            out.push_str(&rest[open..label_end + 2 + consumed]);
        }
        rest = &after[consumed..];
    }
    out.push_str(rest);
    out
}

/// Whether a link target is a Rust item path rather than a URL, anchor or file.
fn is_rust_path(target: &str) -> bool {
    !target.is_empty()
        && target
            .trim_end_matches("()")
            .trim_end_matches('!')
            .split("::")
            .all(|segment| {
                !segment.is_empty()
                    && segment
                        .chars()
                        .all(|character| character.is_alphanumeric() || character == '_')
            })
}

/// A docs.rs search for the item a path names. Paths with a crate prefix
/// search that crate; bare names and `crate::`/`self::`/`super::` paths search
/// the README's own crate. The item's kind, which an exact docs.rs URL
/// encodes, is not known here.
fn docs_rs_search_url(path: &str, crate_name: &str) -> String {
    let segments: Vec<&str> = path.split("::").collect();
    let krate = match segments.as_slice() {
        [first, _, ..] if !matches!(*first, "crate" | "self" | "super") => *first,
        _ => crate_name,
    };
    let name = segments
        .last()
        .map(|segment| segment.trim_end_matches("()").trim_end_matches('!'))
        .unwrap_or(path);
    format!(
        "https://docs.rs/{krate}/latest/{}/?search={name}",
        krate.replace('-', "_")
    )
}

#[cfg(test)]
mod intra_doc_tests {
    use super::*;

    #[test]
    fn links_a_labelled_path_into_its_crate() {
        assert_eq!(
            link_intra_doc_references(
                "a [`Expr`](swc_core::ecma::ast::Expr) node\n",
                "macroforge_ts_syn"
            ),
            "a [`Expr`](https://docs.rs/swc_core/latest/swc_core/?search=Expr) node\n"
        );
    }

    #[test]
    fn links_a_bare_name_into_the_readme_crate() {
        assert_eq!(
            link_intra_doc_references("see [`ts_quote!`]\n", "macroforge_ts_syn"),
            "see [`ts_quote!`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=ts_quote)\n"
        );
    }

    #[test]
    fn leaves_urls_and_code_fences_alone() {
        let markdown = "[`site`](https://macroforge.dev)\n```rust\n/// [`Expr`]\n```\n";
        assert_eq!(
            link_intra_doc_references(markdown, "macroforge_ts"),
            markdown
        );
    }

    #[test]
    fn renders_jsdoc_links_as_code() {
        assert_eq!(
            render_jsdoc_links(
                "Options for {@link expand} and {@link expandFile | reading files}."
            ),
            "Options for `expand` and `reading files`."
        );
    }
}
