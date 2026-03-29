//! Generate README.md files from extracted API documentation JSON
//!
//! Creates rich README files for both Rust crates and TypeScript packages by
//! reading the extracted JSON docs and including badges, overview text, key
//! exports grouped by kind, first code example, and API reference links.

use crate::core::config::Config;
use crate::utils::format;
use anyhow::Result;
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
        let readme = generate_rust_readme(&config.root, crate_name)?;
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
        let readme = generate_ts_readme(&config.root, json_name, npm_name, pkg_path)?;
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
        let readme = generate_rust_readme(root, crate_name)?;
        fs::write(dest.join("README.md"), readme)?;
    }

    for (json_name, pkg_path, npm_name) in TS_PACKAGES {
        let pkg_dir = root.join(pkg_path);
        if !pkg_dir.exists() {
            continue;
        }
        let dest = out_root.join(pkg_path);
        fs::create_dir_all(&dest)?;
        let readme = generate_ts_readme(root, json_name, npm_name, pkg_path)?;
        fs::write(dest.join("README.md"), readme)?;
    }

    Ok(())
}

// ── Rust README generation ──────────────────────────────────────────────────

fn generate_rust_readme(root: &Path, crate_name: &str) -> Result<String> {
    let json_path = root.join(format!(
        "website/static/api-data/rust/{}.json",
        crate_name
    ));

    let mut out = String::new();

    // Title
    out.push_str(&format!("# {}\n\n", crate_name));

    // Try to load extracted JSON docs
    let doc: Option<serde_json::Value> = if json_path.exists() {
        let raw = fs::read_to_string(&json_path)?;
        serde_json::from_str(&raw).ok()
    } else {
        None
    };

    let description = doc
        .as_ref()
        .and_then(|d| d.get("description"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let version = doc
        .as_ref()
        .and_then(|d| d.get("version"))
        .and_then(|v| v.as_str())
        .unwrap_or("0.0.0");

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

    // Installation
    out.push_str("## Installation\n\n");
    out.push_str(&format!(
        "Add this to your `Cargo.toml`:\n\n```toml\n[dependencies]\n{} = \"{}\"\n```\n\n",
        crate_name, version
    ));

    // Key Exports grouped by kind
    if let Some(items) = doc.as_ref().and_then(|d| d.get("items")).and_then(|v| v.as_array()) {
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
        "See the [full API documentation](https://macroforge.dev/docs/api/reference/rust/{}) on\nthe Macroforge website.\n\n",
        crate_name
    ));

    // License
    out.push_str("## License\n\nMIT\n");

    Ok(out)
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
        let raw = fs::read_to_string(&json_path)?;
        serde_json::from_str(&raw).ok()
    } else {
        None
    };

    // Fall back to package.json for description if JSON doc is empty
    let pkg_json_path = root.join(pkg_path).join("package.json");
    let pkg_json: Option<serde_json::Value> = if pkg_json_path.exists() {
        let raw = fs::read_to_string(&pkg_json_path)?;
        serde_json::from_str(&raw).ok()
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

    // Badge
    let encoded = npm_name.replace('/', "%2F").replace('@', "%40");
    out.push_str(&format!(
        "[![npm version](https://badge.fury.io/js/{encoded}.svg)](https://www.npmjs.com/package/{npm_name})\n\n"
    ));

    // Overview
    if !description.is_empty() {
        out.push_str("## Overview\n\n");
        out.push_str(description);
        out.push_str("\n\n");
    }

    // Installation
    out.push_str("## Installation\n\n");
    out.push_str(&format!(
        "```bash\nnpm install {}\n```\n\n",
        npm_name
    ));

    // API section — exports grouped by kind
    if let Some(exports) = doc
        .as_ref()
        .and_then(|d| d.get("exports"))
        .and_then(|v| v.as_array())
    {
        if !exports.is_empty() {
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
    }

    // Documentation link
    let slug = json_name;
    out.push_str("## Documentation\n\n");
    out.push_str(&format!(
        "See the [full documentation](https://macroforge.dev/docs/api/reference/typescript/{}) on\nthe Macroforge website.\n\n",
        slug
    ));

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
        let kind = item
            .get("kind")
            .and_then(|v| v.as_str())
            .unwrap_or("other");
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
        if let Some(entries) = buckets.remove(*kind) {
            if !entries.is_empty() {
                let label = kind_labels.get(kind).unwrap_or(kind);
                result.push((label.to_string(), entries));
            }
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
        let kind = exp
            .get("kind")
            .and_then(|v| v.as_str())
            .unwrap_or("other");
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
        if let Some(entries) = buckets.remove(*kind) {
            if !entries.is_empty() {
                let label = kind_labels.get(kind).unwrap_or(kind);
                result.push((label.to_string(), entries));
            }
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

/// Extract the first sentence from a doc string (up to the first period
/// followed by whitespace or end-of-string, or up to the first newline).
fn first_sentence(text: &str) -> String {
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
    if let Some(first_line) = trimmed.lines().next() {
        if first_line.trim() == line_to_strip.trim() {
            let after = &trimmed[first_line.len()..];
            return after.trim_start_matches('\n');
        }
    }
    text
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => format!("{}{}", c.to_uppercase(), chars.as_str()),
    }
}
