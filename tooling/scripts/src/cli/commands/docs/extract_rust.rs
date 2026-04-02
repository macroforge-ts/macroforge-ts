//! Extract Rust documentation to JSON, .svx pages, and MCP markdown
//!
//! Parses //! (module docs) and /// (item docs) comments from Rust crates
//! and outputs structured JSON for website/README generation, .svx pages
//! for the website, and markdown docs for the MCP server.

use crate::core::config::Config;
use crate::parsers::rust_docs::{self, ItemDoc};
use crate::utils::format;
use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Crate documentation output
#[derive(Debug, Serialize, Deserialize)]
pub struct CrateDoc {
    pub name: String,
    pub kind: String,
    pub version: String,
    pub description: String,
    pub overview: String,
    pub items: Vec<ItemDoc>,
}

/// Raw builtin macro data for page generation
struct BuiltinMacroData {
    display_name: String,
    slug: String,
    raw_docs: String,
}

/// Crate configurations
const CRATES: &[(&str, &str)] = &[
    ("macroforge_ts", "src/lib.rs"),
    ("macroforge_ts_syn", "src/lib.rs"),
    ("macroforge_ts_quote", "src/lib.rs"),
    ("macroforge_ts_macros", "src/lib.rs"),
];

/// Builtin macro configurations
const BUILTIN_MACROS: &[(&str, &str, &str)] = &[
    ("debug", "src/builtin/derive_debug/mod.rs", "Debug"),
    ("clone", "src/builtin/derive_clone/mod.rs", "Clone"),
    ("default", "src/builtin/derive_default/mod.rs", "Default"),
    ("hash", "src/builtin/derive_hash/mod.rs", "Hash"),
    ("ord", "src/builtin/derive_ord/mod.rs", "Ord"),
    (
        "partial_eq",
        "src/builtin/derive_partial_eq/mod.rs",
        "PartialEq",
    ),
    (
        "partial_ord",
        "src/builtin/derive_partial_ord/mod.rs",
        "PartialOrd",
    ),
    (
        "serialize",
        "src/builtin/serde/derive_serialize/mod.rs",
        "Serialize",
    ),
    (
        "deserialize",
        "src/builtin/serde/derive_deserialize/mod.rs",
        "Deserialize",
    ),
];

pub fn run(output_dir: &Path) -> Result<()> {
    let config = Config::load()?;
    let crates_root = config.root.join("crates");
    let output_path = config.root.join(output_dir);

    fs::create_dir_all(&output_path)?;

    format::header("Extracting Rust Documentation");
    println!("Output: {}", output_path.display());

    let mut all_docs = Vec::new();
    let mut total_items = 0;

    // Process crates
    for (crate_name, entry_file) in CRATES {
        let crate_path = crates_root.join(crate_name);
        let entry_path = crate_path.join(entry_file);

        if !entry_path.exists() {
            format::warning(&format!("Entry file not found: {}", entry_path.display()));
            continue;
        }

        print!("Processing {}... ", crate_name);

        let docs = match extract_crate_docs(&crate_path, entry_file) {
            Ok(d) => d,
            Err(e) => {
                println!("failed: {}", e);
                continue;
            }
        };

        let item_count = docs.items.len();
        total_items += item_count;

        let out_path = output_path.join(format!("{}.json", crate_name));
        let json = serde_json::to_string_pretty(&docs)?;
        fs::write(&out_path, json)?;

        println!("{} items", item_count);
        all_docs.push(docs);
    }

    // Process builtin macros
    println!("\nProcessing builtin macros...");
    let mut builtin_docs: HashMap<String, serde_json::Value> = HashMap::new();
    let mut macro_data: Vec<BuiltinMacroData> = Vec::new();

    for (name, file, display_name) in BUILTIN_MACROS {
        let file_path = crates_root.join("macroforge_ts").join(file);
        if !file_path.exists() {
            format::warning(&format!("Macro file not found: {}", file_path.display()));
            continue;
        }

        if let Ok(source) = fs::read_to_string(&file_path) {
            let module_docs = rust_docs::extract_module_docs(&source);
            let slug = name.replace('_', "-");

            builtin_docs.insert(
                name.to_string(),
                serde_json::json!({
                    "name": display_name,
                    "slug": slug,
                    "description": module_docs,
                }),
            );

            macro_data.push(BuiltinMacroData {
                display_name: display_name.to_string(),
                slug,
                raw_docs: module_docs,
            });
        }
    }

    let builtin_path = output_path.join("builtin-macros.json");
    fs::write(&builtin_path, serde_json::to_string_pretty(&builtin_docs)?)?;
    println!("  {} macros documented", builtin_docs.len());

    // Write index
    let index = serde_json::json!({
        "generated": chrono::Utc::now().to_rfc3339(),
        "crates": all_docs.iter().map(|d| serde_json::json!({
            "name": d.name,
            "version": d.version,
            "itemCount": d.items.len(),
        })).collect::<Vec<_>>(),
        "builtinMacros": builtin_docs.keys().collect::<Vec<_>>(),
    });

    let index_path = output_path.join("index.json");
    fs::write(&index_path, serde_json::to_string_pretty(&index)?)?;

    // Generate .svx pages and MCP markdown docs
    println!("\nGenerating .svx pages and MCP docs...");
    write_builtin_macro_pages(&macro_data, &config.root)?;

    println!();
    format::success(&format!(
        "Extracted {} items from {} crates, {} macros",
        total_items,
        all_docs.len(),
        builtin_docs.len()
    ));

    Ok(())
}

/// Transform markdown code block pairs into before/after annotated blocks.
///
/// Scans for the pattern:
///   ```typescript (containing @derive)
///   ...
///   ```
///   Generated output:
///   ```typescript
///   ...
///   ```
///
/// Converts these into:
///   ```typescript before
///   ...
///   ```
///   ```typescript after
///   ...
///   ```
fn transform_macro_blocks(input: &str) -> String {
    // Strategy: find typescript code blocks containing @derive, followed by
    // "Generated output:" text, followed by another typescript code block.
    // Replace the pattern with before/after annotated blocks.

    let mut result = String::new();
    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];

        // Check if this is the start of a typescript code block
        if is_typescript_code_fence(line) {
            // Collect the entire code block
            let block_start = i;
            i += 1;
            let mut block_content = Vec::new();
            while i < lines.len() && !lines[i].trim_start().starts_with("```") {
                block_content.push(lines[i]);
                i += 1;
            }
            let block_end = i; // line with closing ```

            // Check if this block contains @derive
            let has_derive = block_content.iter().any(|l| l.contains("@derive"));

            if has_derive {
                // Look ahead for "Generated output:" followed by another code block
                let mut j = block_end + 1;
                // Skip blank lines
                while j < lines.len() && lines[j].trim().is_empty() {
                    j += 1;
                }

                if j < lines.len() && is_generated_output_line(lines[j]) {
                    // Skip the "Generated output:" line and any blank lines after it
                    j += 1;
                    while j < lines.len() && lines[j].trim().is_empty() {
                        j += 1;
                    }

                    // Check for the next typescript code block
                    if j < lines.len() && is_typescript_code_fence(lines[j]) {
                        j += 1;
                        let mut after_content = Vec::new();
                        while j < lines.len() && !lines[j].trim_start().starts_with("```") {
                            after_content.push(lines[j]);
                            j += 1;
                        }
                        let after_block_end = j;

                        // Write the before block
                        result.push_str("```typescript before\n");
                        for content_line in &block_content {
                            result.push_str(content_line);
                            result.push('\n');
                        }
                        result.push_str("```\n");

                        result.push('\n');

                        // Write the after block
                        result.push_str("```typescript after\n");
                        for content_line in &after_content {
                            result.push_str(content_line);
                            result.push('\n');
                        }
                        result.push_str("```\n");

                        // Skip past the after block's closing fence
                        i = after_block_end + 1;
                        continue;
                    }
                }
            }

            // Not a before/after pair - emit the block as-is
            result.push_str(lines[block_start]);
            result.push('\n');
            for content_line in &block_content {
                result.push_str(content_line);
                result.push('\n');
            }
            if block_end < lines.len() {
                result.push_str(lines[block_end]);
                result.push('\n');
            }
            i = block_end + 1;
            continue;
        }

        result.push_str(line);
        result.push('\n');
        i += 1;
    }

    // Remove trailing newline if input didn't have one
    if !input.ends_with('\n') && result.ends_with('\n') {
        result.pop();
    }

    result
}

/// Check if a line is a typescript/ts code fence opening
fn is_typescript_code_fence(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("```typescript") || trimmed.starts_with("```ts")
}

/// Check if a line is a "Generated output:" marker
fn is_generated_output_line(line: &str) -> bool {
    let trimmed = line.trim().to_lowercase();
    trimmed.starts_with("generated output")
}

/// Escape `<`, `>`, `<=`, `>=` in markdown table rows, but not inside
/// backtick-delimited code spans.
fn escape_comparisons_in_tables(md: &str) -> String {
    let mut result = String::new();
    let mut in_code_block = false;

    for line in md.lines() {
        let trimmed = line.trim();

        // Track fenced code blocks
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            result.push_str(line);
            result.push('\n');
            continue;
        }

        // Only process table rows (lines starting with |) outside code blocks
        if !in_code_block && trimmed.starts_with('|') {
            result.push_str(&escape_comparisons_in_table_row(line));
            result.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    // Remove trailing newline if input didn't have one
    if !md.ends_with('\n') && result.ends_with('\n') {
        result.pop();
    }

    result
}

/// Escape comparison operators in a single table row, preserving inline code spans.
fn escape_comparisons_in_table_row(line: &str) -> String {
    let mut result = String::new();
    let mut chars = line.chars().peekable();
    let mut in_code = false;

    while let Some(c) = chars.next() {
        if c == '`' {
            in_code = !in_code;
            result.push(c);
            continue;
        }

        if in_code {
            result.push(c);
            continue;
        }

        // Outside inline code: escape comparison operators
        // Check for <= and >= first (two-char operators)
        if c == '<' {
            if chars.peek() == Some(&'=') {
                chars.next();
                result.push_str("&lt;=");
            } else {
                result.push_str("&lt;");
            }
        } else if c == '>' {
            if chars.peek() == Some(&'=') {
                chars.next();
                result.push_str("&gt;=");
            } else {
                result.push_str("&gt;");
            }
        } else {
            result.push(c);
        }
    }

    result
}

/// Strip `{identifier}` template patterns from markdown, replacing with just the identifier.
/// E.g., `{className}ToString` -> `classNameToString`
fn strip_template_braces(md: &str) -> String {
    let re = Regex::new(r"\{(\w+)\}").unwrap();
    re.replace_all(md, "$1").to_string()
}

/// Generate a .svx page wrapping markdown content with svelte:head metadata.
fn generate_svx_page(display_name: &str, description: &str, content_md: &str) -> String {
    // Extract first paragraph as meta description (first non-empty lines until blank line)
    let meta_desc = description.replace('"', "&quot;");

    let mut page = String::new();
    page.push_str("<!--\n");
    page.push_str("  Generated by mf docs extract-rust\n");
    page.push_str("  Do not edit manually — edit the Rust module docs instead.\n");
    page.push_str("-->\n\n");
    page.push_str("<svelte:head>\n");
    page.push_str(&format!(
        "\t<title>{} Macro - Macroforge Documentation</title>\n",
        display_name
    ));
    page.push_str(&format!(
        "\t<meta name=\"description\" content=\"{}\" />\n",
        meta_desc
    ));
    page.push_str("</svelte:head>\n\n");
    page.push_str(content_md);

    page
}

/// Extract the first paragraph from markdown text (for meta description).
fn extract_first_paragraph(md: &str) -> String {
    let mut lines = Vec::new();
    let mut started = false;

    for line in md.lines() {
        let trimmed = line.trim();

        // Skip H1 headers
        if trimmed.starts_with("# ") {
            continue;
        }

        if trimmed.is_empty() {
            if started {
                break;
            }
            continue;
        }

        // Skip H2+ headers
        if trimmed.starts_with("##") {
            if started {
                break;
            }
            continue;
        }

        started = true;
        lines.push(line);
    }

    lines.join("\n")
}

/// Remove H1 headers from markdown and return the cleaned content.
fn remove_h1_headers(md: &str) -> String {
    let mut result = Vec::new();
    for line in md.lines() {
        if line.trim().starts_with("# ") && !line.trim().starts_with("## ") {
            continue;
        }
        result.push(line);
    }

    // Trim leading blank lines
    while result.first().map(|s| s.trim().is_empty()).unwrap_or(false) {
        result.remove(0);
    }

    result.join("\n")
}

/// Write .svx pages and MCP markdown docs for all builtin macros.
fn write_builtin_macro_pages(macros: &[BuiltinMacroData], root_path: &Path) -> Result<()> {
    let svx_base = root_path.join("website/src/routes/docs/builtin-macros");
    let mcp_base = root_path.join("packages/mcp-server/docs/builtin-macros");

    let mut svx_count = 0;
    let mut mcp_count = 0;

    for macro_data in macros {
        let raw_md = &macro_data.raw_docs;

        // Extract first paragraph for meta description (before any transformations)
        let first_para = extract_first_paragraph(raw_md);

        // Remove H1 headers from the raw docs
        let without_h1 = remove_h1_headers(raw_md);

        // Add our own H1 with the display name
        let with_h1 = format!("# {}\n\n{}", macro_data.display_name, without_h1);

        // Strip {identifier} template patterns
        let stripped = strip_template_braces(&with_h1);

        // Escape comparisons in table rows
        let escaped = escape_comparisons_in_tables(&stripped);

        // Transform code block pairs (before/after)
        let transformed = transform_macro_blocks(&escaped);

        // Ensure content ends with a trailing newline
        let final_md = if transformed.ends_with('\n') {
            transformed.clone()
        } else {
            format!("{}\n", transformed)
        };

        // Write .svx page
        let svx_dir = svx_base.join(&macro_data.slug);
        fs::create_dir_all(&svx_dir)?;
        let svx_path = svx_dir.join("+page.svx");
        let svx_content = generate_svx_page(&macro_data.display_name, &first_para, &final_md);
        fs::write(&svx_path, &svx_content)?;
        svx_count += 1;

        // Write MCP markdown doc
        fs::create_dir_all(&mcp_base)?;
        let mcp_path = mcp_base.join(format!("{}.md", macro_data.slug));
        fs::write(&mcp_path, &final_md)?;
        mcp_count += 1;
    }

    println!("  {} .svx pages written", svx_count);
    println!("  {} MCP markdown docs written", mcp_count);

    Ok(())
}

fn extract_crate_docs(crate_path: &Path, entry_file: &str) -> Result<CrateDoc> {
    let cargo_path = crate_path.join("Cargo.toml");
    let entry_path = crate_path.join(entry_file);

    // Parse Cargo.toml for version
    let cargo_content = fs::read_to_string(&cargo_path).unwrap_or_default();
    let cargo: toml::Value =
        toml::from_str(&cargo_content).unwrap_or(toml::Value::Table(Default::default()));

    let name = cargo
        .get("package")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or("")
        .to_string();

    let version = cargo
        .get("package")
        .and_then(|p| p.get("version"))
        .and_then(|v| v.as_str())
        .unwrap_or("0.0.0")
        .to_string();

    let description = cargo
        .get("package")
        .and_then(|p| p.get("description"))
        .and_then(|d| d.as_str())
        .unwrap_or("")
        .to_string();

    // Parse source file
    let source = fs::read_to_string(&entry_path)?;
    let overview = rust_docs::extract_module_docs(&source);
    let items = rust_docs::extract_item_docs(&source);

    Ok(CrateDoc {
        name,
        kind: "rust_crate".to_string(),
        version,
        description,
        overview,
        items,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_macro_blocks_basic() {
        let input = r#"## Example

```typescript
/** @derive(Debug) */
class User {
    name: string;
}
```

Generated output:

```typescript
class User {
    name: string;
    static toString(value: User): string {
        return userToString(value);
    }
}
```
"#;
        let result = transform_macro_blocks(input);
        assert!(result.contains("```typescript before"));
        assert!(result.contains("```typescript after"));
        assert!(!result.contains("Generated output:"));
    }

    #[test]
    fn test_transform_macro_blocks_no_derive() {
        let input = r#"```typescript
class User {
    name: string;
}
```
"#;
        let result = transform_macro_blocks(input);
        assert!(!result.contains("before"));
        assert!(!result.contains("after"));
    }

    #[test]
    fn test_escape_comparisons_in_tables() {
        let input = "| `boolean` | false < true |";
        let result = escape_comparisons_in_tables(input);
        assert_eq!(result, "| `boolean` | false &lt; true |");
    }

    #[test]
    fn test_escape_comparisons_preserves_code_spans() {
        let input = "| `Map<K,V>` | comparison |";
        let result = escape_comparisons_in_tables(input);
        assert!(result.contains("`Map<K,V>`"));
        assert!(!result.contains("&lt;K"));
    }

    #[test]
    fn test_escape_comparisons_not_in_code_blocks() {
        let input = "```\n| a < b |\n```";
        let result = escape_comparisons_in_tables(input);
        assert!(result.contains("| a < b |"));
    }

    #[test]
    fn test_escape_le_ge() {
        let input = "| a <= b >= c |";
        let result = escape_comparisons_in_tables(input);
        assert_eq!(result, "| a &lt;= b &gt;= c |");
    }

    #[test]
    fn test_strip_template_braces() {
        let input = "`{className}ToString(value)` and `{enumName}ToStr`";
        let result = strip_template_braces(input);
        assert_eq!(result, "`classNameToString(value)` and `enumNameToStr`");
    }

    #[test]
    fn test_strip_template_braces_preserves_object_literals() {
        let input = r#""ClassName { field1: value1 }""#;
        let result = strip_template_braces(input);
        // Spaces inside braces prevent matching
        assert_eq!(result, r#""ClassName { field1: value1 }""#);
    }

    #[test]
    fn test_remove_h1_headers() {
        let input = "# Debug Macro Implementation\n\nSome content\n\n## Section";
        let result = remove_h1_headers(input);
        assert!(!result.contains("# Debug Macro Implementation"));
        assert!(result.contains("Some content"));
        assert!(result.contains("## Section"));
    }

    #[test]
    fn test_generate_svx_page() {
        let content = "# Debug\n\nSome content\n";
        let result = generate_svx_page("Debug", "Some description", content);
        assert!(result.contains("Generated by mf docs extract-rust"));
        assert!(result.contains("<title>Debug Macro - Macroforge Documentation</title>"));
        assert!(result.contains("Some description"));
        assert!(result.contains("# Debug"));
    }

    #[test]
    fn test_extract_first_paragraph() {
        let input = "# Title\n\nFirst line of paragraph\ncontinued here.\n\n## Next section";
        let result = extract_first_paragraph(input);
        assert_eq!(result, "First line of paragraph\ncontinued here.");
    }

    #[test]
    fn test_extract_first_paragraph_skips_h1() {
        let input = "# Big Title\n\nThe actual description.";
        let result = extract_first_paragraph(input);
        assert_eq!(result, "The actual description.");
    }

    #[test]
    fn test_full_pipeline() {
        let raw_docs = r#"# Debug Macro Implementation

The `Debug` macro generates a `toString()` method.

## Example

```typescript
/** @derive(Debug) */
class User {
    name: string;
}
```

Generated output:

```typescript
class User {
    name: string;
    static toString(value: User): string {
        return userToString(value);
    }
}
```"#;

        let first_para = extract_first_paragraph(raw_docs);
        let without_h1 = remove_h1_headers(raw_docs);
        let with_h1 = format!("# Debug\n\n{}", without_h1);
        let stripped = strip_template_braces(&with_h1);
        let escaped = escape_comparisons_in_tables(&stripped);
        let transformed = transform_macro_blocks(&escaped);

        assert!(transformed.contains("# Debug"));
        assert!(!transformed.contains("# Debug Macro Implementation"));
        assert!(transformed.contains("```typescript before"));
        assert!(transformed.contains("```typescript after"));
        assert!(!transformed.contains("Generated output:"));
        assert_eq!(
            first_para,
            "The `Debug` macro generates a `toString()` method."
        );
    }
}
