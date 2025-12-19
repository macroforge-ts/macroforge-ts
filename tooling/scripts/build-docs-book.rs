#!/usr/bin/env rust-script
//! Build a markdown book from website documentation pages.
//!
//! Reads from prerendered HTML or source .svx files and converts to markdown.
//! Uses htmd for proper HTML to Markdown conversion.
//!
//! ```cargo
//! [dependencies]
//! clap = { version = "4", features = ["derive"] }
//! htmd = "0.5"
//! scraper = "0.18"
//! regex = "1"
//! ```

use clap::Parser;
use htmd::HtmlToMarkdown;
use regex::Regex;
use scraper::{Html, Selector};
use std::fs;
use std::path::{Path, PathBuf};

// ============================================================================
// CLI
// ============================================================================

#[derive(Parser, Debug)]
#[command(name = "build-docs-book")]
#[command(about = "Build a markdown book from website documentation pages")]
struct Args {
    /// Output file path
    #[arg(default_value = "docs/BOOK.md")]
    output_path: PathBuf,

    /// Website root directory
    #[arg(long, default_value = "website")]
    website_dir: PathBuf,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

// ============================================================================
// Navigation Configuration
// ============================================================================

struct NavItem {
    title: &'static str,
    href: &'static str,
}

struct NavSection {
    title: &'static str,
    items: &'static [NavItem],
}

const NAVIGATION: &[NavSection] = &[
    NavSection {
        title: "Getting Started",
        items: &[
            NavItem { title: "Installation", href: "/docs/getting-started" },
            NavItem { title: "First Macro", href: "/docs/getting-started/first-macro" },
        ],
    },
    NavSection {
        title: "Core Concepts",
        items: &[
            NavItem { title: "How Macros Work", href: "/docs/concepts" },
            NavItem { title: "The Derive System", href: "/docs/concepts/derive-system" },
            NavItem { title: "Architecture", href: "/docs/concepts/architecture" },
        ],
    },
    NavSection {
        title: "Built-in Macros",
        items: &[
            NavItem { title: "Overview", href: "/docs/builtin-macros" },
            NavItem { title: "Debug", href: "/docs/builtin-macros/debug" },
            NavItem { title: "Clone", href: "/docs/builtin-macros/clone" },
            NavItem { title: "Default", href: "/docs/builtin-macros/default" },
            NavItem { title: "Hash", href: "/docs/builtin-macros/hash" },
            NavItem { title: "Ord", href: "/docs/builtin-macros/ord" },
            NavItem { title: "PartialEq", href: "/docs/builtin-macros/partial-eq" },
            NavItem { title: "PartialOrd", href: "/docs/builtin-macros/partial-ord" },
            NavItem { title: "Serialize", href: "/docs/builtin-macros/serialize" },
            NavItem { title: "Deserialize", href: "/docs/builtin-macros/deserialize" },
        ],
    },
    NavSection {
        title: "Custom Macros",
        items: &[
            NavItem { title: "Overview", href: "/docs/custom-macros" },
            NavItem { title: "Rust Setup", href: "/docs/custom-macros/rust-setup" },
            NavItem { title: "ts_macro_derive", href: "/docs/custom-macros/ts-macro-derive" },
            NavItem { title: "Template Syntax", href: "/docs/custom-macros/ts-quote" },
        ],
    },
    NavSection {
        title: "Integration",
        items: &[
            NavItem { title: "Overview", href: "/docs/integration" },
            NavItem { title: "CLI", href: "/docs/integration/cli" },
            NavItem { title: "TypeScript Plugin", href: "/docs/integration/typescript-plugin" },
            NavItem { title: "Vite Plugin", href: "/docs/integration/vite-plugin" },
            NavItem { title: "Configuration", href: "/docs/integration/configuration" },
        ],
    },
    NavSection {
        title: "Language Servers",
        items: &[
            NavItem { title: "Overview", href: "/docs/language-servers" },
            NavItem { title: "Svelte", href: "/docs/language-servers/svelte" },
            NavItem { title: "Zed Extensions", href: "/docs/language-servers/zed" },
        ],
    },
    NavSection {
        title: "API Reference",
        items: &[
            NavItem { title: "Overview", href: "/docs/api" },
            NavItem { title: "expandSync()", href: "/docs/api/expand-sync" },
            NavItem { title: "transformSync()", href: "/docs/api/transform-sync" },
            NavItem { title: "NativePlugin", href: "/docs/api/native-plugin" },
            NavItem { title: "PositionMapper", href: "/docs/api/position-mapper" },
        ],
    },
];

// ============================================================================
// Path Helpers
// ============================================================================

fn href_to_prerendered_path(href: &str, website_dir: &Path) -> PathBuf {
    website_dir.join("build/prerendered").join(&href[1..]).with_extension("html")
}

fn href_to_source_path(href: &str, website_dir: &Path) -> PathBuf {
    website_dir.join("src/routes").join(&href[1..]).join("+page.svx")
}

// ============================================================================
// Markdown Source Processing
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
// HTML to Markdown Conversion using htmd
// ============================================================================

fn extract_prose_html(html: &str) -> Option<String> {
    let document = Html::parse_document(html);

    // Try to find the prose content area
    let prose_selector = Selector::parse("div.prose").ok()?;
    if let Some(prose) = document.select(&prose_selector).next() {
        return Some(prose.html());
    }

    // Fallback: try article
    let article_selector = Selector::parse("article").ok()?;
    if let Some(article) = document.select(&article_selector).next() {
        return Some(article.html());
    }

    None
}

fn html_to_markdown(html: &str) -> String {
    // Extract just the prose content
    let prose_html = match extract_prose_html(html) {
        Some(h) => h,
        None => return String::new(),
    };

    // Use htmd for conversion
    let converter = HtmlToMarkdown::builder()
        .skip_tags(vec!["script", "style", "svg", "button", "nav"])
        .build();

    let mut md = converter.convert(&prose_html).unwrap_or_default();

    // Post-processing cleanup
    md = cleanup_markdown(&md);

    md
}

fn cleanup_markdown(md: &str) -> String {
    let mut result = md.to_string();

    // Fix excessive newlines
    let newline_re = Regex::new(r"\n{3,}").unwrap();
    result = newline_re.replace_all(&result, "\n\n").to_string();

    // Clean up code block language hints (htmd may output weird ones)
    let code_lang_re = Regex::new(r"```\s*\n").unwrap();
    result = code_lang_re.replace_all(&result, "```\n").to_string();

    // Fix any remaining HTML entities
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
// Book Building
// ============================================================================

fn build_book(args: &Args) {
    // Determine repo root - check for pixi.toml to confirm we're at root
    let cwd = std::env::current_dir().expect("Failed to get current directory");
    let repo_root = if cwd.join("pixi.toml").exists() {
        cwd
    } else {
        // Try to find repo root by looking for pixi.toml
        let mut candidate = cwd.clone();
        loop {
            if candidate.join("pixi.toml").exists() {
                break candidate;
            }
            match candidate.parent() {
                Some(parent) => candidate = parent.to_path_buf(),
                None => break cwd, // Give up and use cwd
            }
        }
    };

    let website_dir = repo_root.join(&args.website_dir);
    let prerendered_dir = website_dir.join("build/prerendered");
    let output_path = repo_root.join(&args.output_path);

    // Check if prerendered directory exists
    let has_prerendered = prerendered_dir.exists();
    if !has_prerendered {
        eprintln!("Warning: Prerendered directory not found: {:?}", prerendered_dir);
        eprintln!("Will use source .svx files where available.\n");
    }

    // Ensure output directory exists
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).ok();
    }

    let mut book = Vec::new();

    // Title page
    book.push("# Macroforge Documentation".to_string());
    book.push(String::new());
    book.push("*TypeScript Macros - Rust-Powered Code Generation*".to_string());
    book.push(String::new());
    book.push("---".to_string());
    book.push(String::new());

    // Table of contents
    book.push("## Table of Contents".to_string());
    book.push(String::new());

    for section in NAVIGATION {
        book.push(format!("### {}", section.title));
        for item in section.items {
            let anchor = item.title.to_lowercase()
                .chars()
                .map(|c| if c.is_alphanumeric() { c } else { '-' })
                .collect::<String>();
            let anchor = Regex::new(r"-+").unwrap().replace_all(&anchor, "-").to_string();
            book.push(format!("- [{}](#{})", item.title, anchor.trim_matches('-')));
        }
        book.push(String::new());
    }

    book.push("---".to_string());
    book.push(String::new());

    // Process each section
    for section in NAVIGATION {
        book.push(format!("# {}", section.title));
        book.push(String::new());

        for item in section.items {
            // Try source markdown first
            if let Some(md) = read_markdown_source(item.href, &website_dir) {
                if args.verbose {
                    println!("Processing: {} [source]", item.title);
                }
                book.push(md);
            } else if has_prerendered {
                // Fall back to prerendered HTML
                let html_path = href_to_prerendered_path(item.href, &website_dir);
                if html_path.exists() {
                    if args.verbose {
                        println!("Processing: {} [html]", item.title);
                    }
                    let html = fs::read_to_string(&html_path).unwrap_or_default();
                    let md = html_to_markdown(&html);
                    book.push(md);
                } else {
                    eprintln!("Warning: No content found for {}", item.href);
                }
            } else {
                eprintln!("Warning: No content found for {}", item.href);
            }

            book.push(String::new());
            book.push("---".to_string());
            book.push(String::new());
        }
    }

    // Write the book
    let book_content = book.join("\n");
    fs::write(&output_path, &book_content).expect("Failed to write book");

    println!("\nBook generated: {:?}", output_path);
    println!("Total size: {:.1} KB", book_content.len() as f64 / 1024.0);
}

fn main() {
    let args = Args::parse();
    build_book(&args);
}
