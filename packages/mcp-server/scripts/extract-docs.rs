#!/usr/bin/env rust-script
//! Extract documentation from website prerendered HTML into markdown files for MCP server.
//! Reads from build output (website/build/prerendered/) to include all dynamic content.
//! Auto-discovers all pages from the website's navigation.ts config.
//! Large documents are automatically chunked at H2 headers for better AI consumption.
//!
//! Usage: rust-script scripts/extract-docs.rs
//!
//! ```cargo
//! [dependencies]
//! htmd = "0.5"
//! scraper = "0.18"
//! regex = "1"
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! ```

use htmd::HtmlToMarkdown;
use regex::Regex;
use scraper::{Html, Selector};
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

// ============================================================================
// Constants
// ============================================================================

const CHUNK_SIZE_THRESHOLD: usize = 6000;
const MIN_CHUNK_SIZE: usize = 500;

// ============================================================================
// Types
// ============================================================================

#[derive(Debug)]
struct NavItem {
    title: String,
    href: String,
}

#[derive(Debug)]
struct NavSection {
    title: String,
    items: Vec<NavItem>,
}

#[derive(Debug, Serialize)]
struct DocSection {
    id: String,
    title: String,
    category: String,
    category_title: String,
    path: String,
    use_cases: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_chunked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chunk_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<String>,
}

#[derive(Debug)]
struct Chunk {
    slug: String,
    title: String,
    content: String,
}

// ============================================================================
// Use Cases Map
// ============================================================================

fn get_use_cases_map() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();

    // Getting Started
    m.insert("/docs/getting-started", "setup, install, npm, getting started, quick start, init");
    m.insert("/docs/getting-started/first-macro", "tutorial, example, hello world, beginner, learn");

    // Core Concepts
    m.insert("/docs/concepts", "architecture, overview, understanding, basics, fundamentals");
    m.insert("/docs/concepts/derive-system", "@derive, decorator, annotation, derive macro");
    m.insert("/docs/concepts/architecture", "internals, rust, swc, napi, how it works");

    // Built-in Macros
    m.insert("/docs/builtin-macros", "all macros, list, available macros, macro list");
    m.insert("/docs/builtin-macros/debug", "toString, debugging, logging, output, print");
    m.insert("/docs/builtin-macros/clone", "copy, clone, duplicate, shallow copy, immutable");
    m.insert("/docs/builtin-macros/default", "default values, factory, initialization, constructor");
    m.insert("/docs/builtin-macros/hash", "hashCode, hashing, hash map, equality, hash function");
    m.insert("/docs/builtin-macros/ord", "compareTo, ordering, sorting, comparison, total order");
    m.insert("/docs/builtin-macros/partial-eq", "equals, equality, comparison, value equality");
    m.insert("/docs/builtin-macros/partial-ord", "compareTo, partial ordering, sorting, nullable comparison");
    m.insert("/docs/builtin-macros/serialize", "toJSON, serialization, json, api, data transfer");
    m.insert("/docs/builtin-macros/deserialize", "fromJSON, deserialization, parsing, validation, json");

    // Custom Macros
    m.insert("/docs/custom-macros", "custom, extending, creating macros, own macro");
    m.insert("/docs/custom-macros/rust-setup", "rust, cargo, napi, compilation, building");
    m.insert("/docs/custom-macros/ts-macro-derive", "attribute, proc macro, derive attribute, rust macro");
    m.insert("/docs/custom-macros/ts-quote", "ts_quote, template, code generation, interpolation");

    // Integration
    m.insert("/docs/integration", "setup, integration, tools, ecosystem");
    m.insert("/docs/integration/cli", "command line, macroforge command, expand, terminal");
    m.insert("/docs/integration/typescript-plugin", "vscode, ide, language server, intellisense, autocomplete");
    m.insert("/docs/integration/vite-plugin", "vite, build, bundler, react, svelte, sveltekit");
    m.insert("/docs/integration/svelte-preprocessor", "svelte, preprocessor, svelte components, .svelte files, sveltekit");
    m.insert("/docs/integration/mcp-server", "mcp, ai, claude, llm, model context protocol, assistant");
    m.insert("/docs/integration/configuration", "macroforge.json, config, settings, options");

    // Language Servers
    m.insert("/docs/language-servers", "lsp, language server, editor support");
    m.insert("/docs/language-servers/svelte", "svelte, svelte language server, .svelte files");
    m.insert("/docs/language-servers/zed", "zed, zed editor, extension");

    // API Reference
    m.insert("/docs/api", "api, functions, exports, programmatic");
    m.insert("/docs/api/expand-sync", "expandSync, expand, transform, macro expansion");
    m.insert("/docs/api/transform-sync", "transformSync, transform, metadata, low-level");
    m.insert("/docs/api/native-plugin", "NativePlugin, caching, language server, stateful");
    m.insert("/docs/api/position-mapper", "PositionMapper, source map, diagnostics, position");

    // Roadmap
    m.insert("/docs/roadmap", "roadmap, future, planned features, upcoming");

    m
}

fn get_category_id_map() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("getting-started", "installation");
    m.insert("concepts", "how-macros-work");
    m.insert("builtin-macros", "macros-overview");
    m.insert("custom-macros", "custom-overview");
    m.insert("integration", "integration-overview");
    m.insert("language-servers", "ls-overview");
    m.insert("api", "api-overview");
    m.insert("roadmap", "roadmap");
    m
}

// ============================================================================
// Navigation Parsing
// ============================================================================

fn parse_navigation(nav_path: &Path) -> Vec<NavSection> {
    let content = fs::read_to_string(nav_path).expect("Failed to read navigation.ts");
    let mut sections = Vec::new();

    let section_re = Regex::new(r#"\{\s*title:\s*['"]([^'"]+)['"]\s*,\s*items:\s*\[([\s\S]*?)\]\s*\}"#).unwrap();
    let item_re = Regex::new(r#"\{\s*title:\s*['"]([^'"]+)['"]\s*,\s*href:\s*['"]([^'"]+)['"]\s*\}"#).unwrap();

    for section_cap in section_re.captures_iter(&content) {
        let section_title = section_cap[1].to_string();
        let items_content = &section_cap[2];

        let mut items = Vec::new();
        for item_cap in item_re.captures_iter(items_content) {
            items.push(NavItem {
                title: item_cap[1].to_string(),
                href: item_cap[2].to_string(),
            });
        }

        if !items.is_empty() {
            sections.push(NavSection {
                title: section_title,
                items,
            });
        }
    }

    sections
}

// ============================================================================
// Path Helpers
// ============================================================================

fn href_to_category(href: &str) -> String {
    let path = href.strip_prefix("/docs/").unwrap_or(href);
    path.split('/').next().unwrap_or("").to_string()
}

fn href_to_id(href: &str, category_map: &HashMap<&str, &str>) -> String {
    let path = href.strip_prefix("/docs/").unwrap_or(href);
    let parts: Vec<&str> = path.split('/').collect();

    if parts.len() == 1 {
        category_map.get(parts[0]).map(|s| s.to_string()).unwrap_or_else(|| parts[0].to_string())
    } else {
        parts.last().unwrap_or(&"").to_string()
    }
}

fn href_to_prerendered_path(href: &str, website_dir: &Path) -> PathBuf {
    let path = href.strip_prefix("/").unwrap_or(href);
    website_dir.join("build/prerendered").join(path).with_extension("html")
}

fn href_to_source_path(href: &str, website_dir: &Path) -> PathBuf {
    let path = href.strip_prefix("/").unwrap_or(href);
    website_dir.join("src/routes").join(path).join("+page.svx")
}

// ============================================================================
// Markdown Processing
// ============================================================================

fn strip_mdsvex_boilerplate(markdown: &str) -> String {
    let mut md = markdown.to_string();

    // Remove leading HTML comments
    let comment_re = Regex::new(r"^<!--[\s\S]*?-->\s*").unwrap();
    md = comment_re.replace(&md, "").to_string();

    // Remove svelte:head blocks
    let head_re = Regex::new(r"<svelte:head>[\s\S]*?</svelte:head>\s*").unwrap();
    md = head_re.replace_all(&md, "").to_string();

    md.trim().to_string() + "\n"
}

fn read_markdown_source(href: &str, website_dir: &Path) -> Option<String> {
    let source_path = href_to_source_path(href, website_dir);
    if source_path.exists() {
        let content = fs::read_to_string(&source_path).ok()?;
        Some(strip_mdsvex_boilerplate(&content))
    } else {
        None
    }
}

// ============================================================================
// HTML to Markdown Conversion
// ============================================================================

fn extract_prose_html(html: &str) -> Option<String> {
    let document = Html::parse_document(html);

    let prose_selector = Selector::parse("div.prose").ok()?;
    if let Some(prose) = document.select(&prose_selector).next() {
        return Some(prose.html());
    }

    let article_selector = Selector::parse("article").ok()?;
    if let Some(article) = document.select(&article_selector).next() {
        return Some(article.html());
    }

    None
}

fn html_to_markdown(html: &str) -> String {
    let prose_html = match extract_prose_html(html) {
        Some(h) => h,
        None => return String::new(),
    };

    let converter = HtmlToMarkdown::builder()
        .skip_tags(vec!["script", "style", "svg", "button", "nav"])
        .build();

    let mut md = converter.convert(&prose_html).unwrap_or_default();
    md = cleanup_markdown(&md);
    md
}

fn cleanup_markdown(md: &str) -> String {
    let mut result = md.to_string();

    // Fix excessive newlines
    let newline_re = Regex::new(r"\n{3,}").unwrap();
    result = newline_re.replace_all(&result, "\n\n").to_string();

    // Fix HTML entities
    result = result
        .replace("&nbsp;", " ")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&#x27;", "'")
        .replace("&#123;", "{")
        .replace("&#125;", "}");

    result.trim().to_string()
}

// ============================================================================
// Chunking
// ============================================================================

fn header_to_slug(header: &str) -> String {
    let backtick_re = Regex::new(r"`([^`]+)`").unwrap();
    let mut slug = backtick_re.replace_all(header, "$1").to_string();

    // Remove HTML entities
    let entity_re = Regex::new(r"&#\d+;").unwrap();
    slug = entity_re.replace_all(&slug, "").to_string();
    let named_entity_re = Regex::new(r"&[a-z]+;").unwrap();
    slug = named_entity_re.replace_all(&slug, "").to_string();

    // Keep only alphanumeric and spaces/hyphens
    slug = slug.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-')
        .collect();

    // Replace spaces with hyphens, collapse multiple hyphens
    let space_re = Regex::new(r"\s+").unwrap();
    slug = space_re.replace_all(&slug, "-").to_string();
    let hyphen_re = Regex::new(r"-+").unwrap();
    slug = hyphen_re.replace_all(&slug, "-").to_string();

    slug.trim_matches('-').to_string()
}

fn extract_chunk_use_cases(content: &str, parent_use_cases: &str) -> String {
    let mut keywords = Vec::new();

    // Get code-related terms
    let code_re = Regex::new(r"`([^`]+)`").unwrap();
    for cap in code_re.captures_iter(content).take(5) {
        let term = cap[1].to_lowercase();
        if term.len() > 2 && term.len() < 30 && !term.contains(' ') {
            keywords.push(term);
        }
    }

    // Include parent use cases
    let parent_keywords: Vec<&str> = parent_use_cases.split(',').map(|k| k.trim()).take(2).collect();

    let mut result: Vec<String> = parent_keywords.iter().map(|s| s.to_string()).collect();
    result.extend(keywords);
    result.dedup();
    result.truncate(6);
    result.join(", ")
}

fn chunk_markdown(markdown: &str, parent_title: &str) -> Vec<Chunk> {
    let mut chunks = Vec::new();

    let h2_re = Regex::new(r"(?m)^## (.+)$").unwrap();
    let parts: Vec<&str> = h2_re.split(markdown).collect();
    let headers: Vec<String> = h2_re.captures_iter(markdown).map(|c| c[1].to_string()).collect();

    // First part is content before any H2
    if !parts.is_empty() && parts[0].trim().len() >= MIN_CHUNK_SIZE {
        chunks.push(Chunk {
            slug: "overview".to_string(),
            title: format!("{}: Overview", parent_title),
            content: parts[0].trim().to_string(),
        });
    } else if !parts.is_empty() && !parts[0].trim().is_empty() {
        chunks.push(Chunk {
            slug: "_intro".to_string(),
            title: String::new(),
            content: parts[0].trim().to_string(),
        });
    }

    // Process header/content pairs
    for (i, header) in headers.iter().enumerate() {
        let content = parts.get(i + 1).map(|s| s.trim()).unwrap_or("");
        let slug = header_to_slug(header);
        let full_content = format!("## {}\n\n{}", header, content);

        // Merge small chunks with previous
        if full_content.len() < MIN_CHUNK_SIZE && !chunks.is_empty() {
            let last_idx = chunks.len() - 1;
            if chunks[last_idx].slug != "_intro" {
                chunks[last_idx].content.push_str("\n\n");
                chunks[last_idx].content.push_str(&full_content);
                continue;
            }
        }

        chunks.push(Chunk {
            slug,
            title: format!("{}: {}", parent_title, header),
            content: full_content,
        });
    }

    // Handle intro content
    if chunks.len() > 1 && chunks[0].slug == "_intro" {
        let intro = chunks.remove(0);
        chunks[0].content = format!("{}\n\n{}", intro.content, chunks[0].content);
        chunks[0].slug = "overview".to_string();
        chunks[0].title = format!("{}: Overview", parent_title);
    } else if chunks.len() == 1 && chunks[0].slug == "_intro" {
        chunks[0].slug = "overview".to_string();
        chunks[0].title = format!("{}: Overview", parent_title);
    }

    chunks
}

fn should_chunk(content: &str) -> bool {
    content.len() > CHUNK_SIZE_THRESHOLD
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    // Paths - script runs from repo root or from packages/mcp-server
    let cwd = std::env::current_dir().unwrap();

    // Determine repo root - check for pixi.toml to confirm we're at root
    let repo_root = if cwd.join("pixi.toml").exists() {
        cwd.clone()
    } else {
        // Try to find repo root by looking for pixi.toml
        let mut candidate = cwd.clone();
        loop {
            if candidate.join("pixi.toml").exists() {
                break candidate;
            }
            match candidate.parent() {
                Some(parent) => candidate = parent.to_path_buf(),
                None => break cwd.clone(), // Give up and use cwd
            }
        }
    };

    let website_dir = repo_root.join("website");
    let prerendered_dir = website_dir.join("build/prerendered");
    let navigation_path = website_dir.join("src/lib/config/navigation.ts");
    let output_dir = repo_root.join("packages/mcp-server/docs");

    // Check prerendered exists
    let has_prerendered = prerendered_dir.exists();
    if !has_prerendered {
        eprintln!("Warning: Prerendered directory not found: {:?}", prerendered_dir);
        eprintln!("Falling back to mdsvex source pages when available.\n");
    }

    println!("Auto-discovering pages from navigation.ts...\n");

    // Parse navigation
    let navigation = parse_navigation(&navigation_path);
    println!("Found {} sections in navigation.ts\n", navigation.len());

    // Ensure output directory exists
    if let Err(e) = fs::create_dir_all(&output_dir) {
        eprintln!("Failed to create output directory: {}", e);
        std::process::exit(1);
    }

    let use_cases_map = get_use_cases_map();
    let category_id_map = get_category_id_map();
    let mut sections: Vec<DocSection> = Vec::new();

    for section in &navigation {
        let category = href_to_category(section.items.first().map(|i| i.href.as_str()).unwrap_or(""));

        // Create category directory
        let category_dir = output_dir.join(&category);
        fs::create_dir_all(&category_dir).ok();

        for item in &section.items {
            let item_id = href_to_id(&item.href, &category_id_map);
            println!("Processing: {} ({})", item.title, item.href);

            // Try source markdown first
            let markdown_content = match read_markdown_source(&item.href, &website_dir) {
                Some(md) => md,
                None => {
                    if !has_prerendered {
                        eprintln!("Warning: No source markdown for {} and no prerendered HTML", item.href);
                        continue;
                    }

                    let html_path = href_to_prerendered_path(&item.href, &website_dir);
                    if !html_path.exists() {
                        eprintln!("Warning: File not found: {:?}", html_path);
                        continue;
                    }

                    let raw_html = fs::read_to_string(&html_path).unwrap_or_default();
                    html_to_markdown(&raw_html)
                }
            };

            let use_cases = use_cases_map.get(item.href.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| item.title.to_lowercase());

            // Check if we need to chunk
            if should_chunk(&markdown_content) {
                let chunks = chunk_markdown(&markdown_content, &item.title);

                if chunks.len() > 1 {
                    println!("  → Chunking into {} parts", chunks.len());

                    // Create chunk directory
                    let chunk_dir = category_dir.join(&item_id);
                    fs::create_dir_all(&chunk_dir).ok();

                    let mut chunk_ids = Vec::new();

                    for chunk in &chunks {
                        let chunk_id = format!("{}/{}", item_id, chunk.slug);
                        let chunk_path = chunk_dir.join(format!("{}.md", chunk.slug));
                        fs::write(&chunk_path, &chunk.content).ok();

                        chunk_ids.push(chunk_id.clone());

                        sections.push(DocSection {
                            id: chunk_id,
                            title: chunk.title.clone(),
                            category: category.clone(),
                            category_title: section.title.clone(),
                            path: format!("{}/{}/{}.md", category, item_id, chunk.slug),
                            use_cases: extract_chunk_use_cases(&chunk.content, &use_cases),
                            is_chunked: None,
                            chunk_ids: None,
                            parent_id: Some(item_id.clone()),
                        });
                    }

                    // Add parent entry
                    sections.push(DocSection {
                        id: item_id.clone(),
                        title: item.title.clone(),
                        category: category.clone(),
                        category_title: section.title.clone(),
                        path: format!("{}/{}.md", category, item_id),
                        use_cases: use_cases.clone(),
                        is_chunked: Some(true),
                        chunk_ids: Some(chunk_ids),
                        parent_id: None,
                    });

                    // Write full file for reference
                    let output_path = category_dir.join(format!("{}.md", item_id));
                    fs::write(&output_path, &markdown_content).ok();

                    continue;
                }
            }

            // Not chunked - write as single file
            let output_path = category_dir.join(format!("{}.md", item_id));
            fs::write(&output_path, &markdown_content).ok();

            sections.push(DocSection {
                id: item_id,
                title: item.title.clone(),
                category: category.clone(),
                category_title: section.title.clone(),
                path: format!("{}/{}.md", category, item.href.split('/').last().unwrap_or("")),
                use_cases,
                is_chunked: None,
                chunk_ids: None,
                parent_id: None,
            });
        }
    }

    // Write sections.json
    let sections_path = output_dir.join("sections.json");
    let json = serde_json::to_string_pretty(&sections).unwrap();
    fs::write(&sections_path, json).ok();

    println!("\nExtracted {} documentation sections", sections.len());
    println!("Output directory: {:?}", output_dir);
}
