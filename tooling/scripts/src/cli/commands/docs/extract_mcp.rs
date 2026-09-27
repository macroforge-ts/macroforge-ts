//! Extract the website's documentation into markdown for the MCP server
//!
//! Discovers every page from the website's `navigation.ts`, reads each one's
//! mdsvex source where it has one and the prerendered HTML (`website/build`)
//! otherwise, so the website must be built first. Documents over
//! [`CHUNK_SIZE_THRESHOLD`] are also split at their H2 headers, which serves
//! an assistant better than one long page. `sections.json` indexes them all.

use crate::cli::commands::docs::generated::GeneratedFiles;
use crate::utils::format;
use anyhow::{Context, Result};
use htmd::HtmlToMarkdown;
use regex::Regex;
use scraper::{Html, Selector};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

/// Where the MCP server's docs live, relative to the repository root.
const OUTPUT_DIR: &str = "packages/mcp-server/docs";

const CHUNK_SIZE_THRESHOLD: usize = 6000;
const MIN_CHUNK_SIZE: usize = 500;

/// Search keywords per page, beyond its title.
const USE_CASES: &[(&str, &str)] = &[
    // Getting Started
    (
        "/docs/getting-started",
        "setup, install, npm, getting started, quick start, init",
    ),
    (
        "/docs/getting-started/first-macro",
        "tutorial, example, hello world, beginner, learn",
    ),
    // Core Concepts
    (
        "/docs/concepts",
        "architecture, overview, understanding, basics, fundamentals",
    ),
    (
        "/docs/concepts/derive-system",
        "@derive, decorator, annotation, derive macro",
    ),
    (
        "/docs/concepts/architecture",
        "internals, rust, swc, napi, how it works",
    ),
    // Built-in Macros
    (
        "/docs/builtin-macros",
        "all macros, list, available macros, macro list",
    ),
    (
        "/docs/builtin-macros/debug",
        "toString, debugging, logging, output, print",
    ),
    (
        "/docs/builtin-macros/clone",
        "copy, clone, duplicate, shallow copy, immutable",
    ),
    (
        "/docs/builtin-macros/default",
        "default values, factory, initialization, constructor",
    ),
    (
        "/docs/builtin-macros/hash",
        "hashCode, hashing, hash map, equality, hash function",
    ),
    (
        "/docs/builtin-macros/ord",
        "compareTo, ordering, sorting, comparison, total order",
    ),
    (
        "/docs/builtin-macros/partial-eq",
        "equals, equality, comparison, value equality",
    ),
    (
        "/docs/builtin-macros/partial-ord",
        "compareTo, partial ordering, sorting, nullable comparison",
    ),
    (
        "/docs/builtin-macros/serialize",
        "toJSON, serialization, json, api, data transfer",
    ),
    (
        "/docs/builtin-macros/deserialize",
        "fromJSON, deserialization, parsing, validation, json",
    ),
    // Custom Macros
    (
        "/docs/custom-macros",
        "custom, extending, creating macros, own macro",
    ),
    (
        "/docs/custom-macros/rust-setup",
        "rust, cargo, napi, compilation, building",
    ),
    (
        "/docs/custom-macros/ts-macro-derive",
        "attribute, proc macro, derive attribute, rust macro",
    ),
    (
        "/docs/custom-macros/ts-quote",
        "ts_quote, template, code generation, interpolation",
    ),
    // Integration
    ("/docs/integration", "setup, integration, tools, ecosystem"),
    (
        "/docs/integration/cli",
        "command line, macroforge command, expand, terminal",
    ),
    (
        "/docs/integration/typescript-plugin",
        "vscode, ide, language server, intellisense, autocomplete",
    ),
    (
        "/docs/integration/vite-plugin",
        "vite, build, bundler, react, svelte, sveltekit",
    ),
    (
        "/docs/integration/svelte-preprocessor",
        "svelte, preprocessor, svelte components, .svelte files, sveltekit",
    ),
    (
        "/docs/integration/mcp-server",
        "mcp, ai, claude, llm, model context protocol, assistant",
    ),
    (
        "/docs/integration/configuration",
        "macroforge.config.ts, config, settings, options",
    ),
    // Language Servers
    (
        "/docs/language-servers",
        "lsp, language server, editor support",
    ),
    (
        "/docs/language-servers/svelte",
        "svelte, svelte language server, .svelte files",
    ),
    ("/docs/language-servers/zed", "zed, zed editor, extension"),
    // API Reference
    ("/docs/api", "api, functions, exports, programmatic"),
    (
        "/docs/api/expand-sync",
        "expandSync, expand, transform, macro expansion",
    ),
    (
        "/docs/api/transform-sync",
        "transformSync, transform, metadata, low-level",
    ),
    (
        "/docs/api/native-plugin",
        "NativePlugin, caching, language server, stateful",
    ),
    (
        "/docs/api/position-mapper",
        "PositionMapper, source map, diagnostics, position",
    ),
    // Roadmap
    (
        "/docs/roadmap",
        "roadmap, future, planned features, upcoming",
    ),
];

/// The id of each category's index page, which has no path segment of its own.
const CATEGORY_IDS: &[(&str, &str)] = &[
    ("getting-started", "installation"),
    ("concepts", "how-macros-work"),
    ("builtin-macros", "macros-overview"),
    ("custom-macros", "custom-overview"),
    ("integration", "integration-overview"),
    ("language-servers", "ls-overview"),
    ("api", "api-overview"),
    ("roadmap", "roadmap"),
];

struct NavItem {
    title: String,
    href: String,
}

struct NavSection {
    title: String,
    items: Vec<NavItem>,
}

/// One entry of `sections.json`, which the MCP server loads.
#[derive(Serialize)]
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

struct Chunk {
    slug: String,
    title: String,
    content: String,
}

/// The regular expressions the extraction uses, compiled once per run.
struct Patterns {
    nav_section: Regex,
    nav_item: Regex,
    leading_comment: Regex,
    svelte_head: Regex,
    blank_lines: Regex,
    inline_code: Regex,
    numeric_entity: Regex,
    named_entity: Regex,
    whitespace: Regex,
    hyphens: Regex,
    h2: Regex,
}

impl Patterns {
    fn compile() -> Result<Self> {
        let compile = |pattern: &str| {
            Regex::new(pattern).with_context(|| format!("invalid pattern {pattern}"))
        };
        Ok(Self {
            nav_section: compile(
                r#"\{\s*title:\s*['"]([^'"]+)['"]\s*,\s*items:\s*\[([\s\S]*?)\]\s*\}"#,
            )?,
            nav_item: compile(
                r#"\{\s*title:\s*['"]([^'"]+)['"]\s*,\s*href:\s*['"]([^'"]+)['"]\s*\}"#,
            )?,
            leading_comment: compile(r"^<!--[\s\S]*?-->\s*")?,
            svelte_head: compile(r"<svelte:head>[\s\S]*?</svelte:head>\s*")?,
            blank_lines: compile(r"\n{3,}")?,
            inline_code: compile(r"`([^`]+)`")?,
            numeric_entity: compile(r"&#\d+;")?,
            named_entity: compile(r"&[a-z]+;")?,
            whitespace: compile(r"\s+")?,
            hyphens: compile(r"-+")?,
            h2: compile(r"(?m)^## (.+)$")?,
        })
    }
}

/// Extracts every page the website's navigation lists into markdown, with
/// `sections.json` indexing them.
pub fn generate(root: &Path) -> Result<GeneratedFiles> {
    let website_dir = root.join("website");
    let prerendered_dir = website_dir.join("build");
    let navigation_path = website_dir.join("src/lib/config/navigation.ts");
    let output_dir = Path::new(OUTPUT_DIR);

    if !prerendered_dir.exists() {
        anyhow::bail!(
            "{} not found: build the website first (pixi run build:website)",
            prerendered_dir.display()
        );
    }

    format::header("Extracting MCP Server Docs");

    let patterns = Patterns::compile()?;
    let navigation_source = fs::read_to_string(&navigation_path)
        .with_context(|| format!("failed to read {}", navigation_path.display()))?;
    let navigation = parse_navigation(&navigation_source, &patterns);
    println!("Found {} sections in navigation.ts", navigation.len());

    let mut files: Vec<(PathBuf, String)> = Vec::new();
    let mut sections: Vec<DocSection> = Vec::new();

    for section in &navigation {
        let category = section
            .items
            .first()
            .map(|item| href_to_category(&item.href))
            .unwrap_or_default();

        for item in &section.items {
            let item_id = href_to_id(&item.href);
            println!("Processing: {} ({})", item.title, item.href);

            let markdown = match read_markdown_source(&item.href, &website_dir, &patterns)? {
                Some(markdown) => markdown,
                None => {
                    let html_path = href_to_prerendered_path(&item.href, &prerendered_dir);
                    let html = fs::read_to_string(&html_path).with_context(|| {
                        format!("no page for {} at {}", item.href, html_path.display())
                    })?;
                    html_to_markdown(&html, &patterns)
                        .with_context(|| format!("failed to convert {}", html_path.display()))?
                }
            };

            let use_cases = USE_CASES
                .iter()
                .find(|(href, _)| *href == item.href)
                .map(|(_, use_cases)| use_cases.to_string())
                .unwrap_or_else(|| item.title.to_lowercase());

            let chunks = if markdown.len() > CHUNK_SIZE_THRESHOLD {
                chunk_markdown(&markdown, &item.title, &patterns)
            } else {
                Vec::new()
            };

            let page_path = format!("{category}/{item_id}.md");
            files.push((output_dir.join(&page_path), markdown));

            if chunks.len() > 1 {
                println!("  → Chunking into {} parts", chunks.len());
                let mut chunk_ids = Vec::new();
                for chunk in chunks {
                    let chunk_id = format!("{item_id}/{}", chunk.slug);
                    let chunk_path = format!("{category}/{item_id}/{}.md", chunk.slug);
                    sections.push(DocSection {
                        id: chunk_id.clone(),
                        title: chunk.title,
                        category: category.clone(),
                        category_title: section.title.clone(),
                        path: chunk_path.clone(),
                        use_cases: chunk_use_cases(&chunk.content, &use_cases, &patterns),
                        is_chunked: None,
                        chunk_ids: None,
                        parent_id: Some(item_id.clone()),
                    });
                    files.push((output_dir.join(chunk_path), chunk.content));
                    chunk_ids.push(chunk_id);
                }
                sections.push(DocSection {
                    id: item_id,
                    title: item.title.clone(),
                    category: category.clone(),
                    category_title: section.title.clone(),
                    path: page_path,
                    use_cases,
                    is_chunked: Some(true),
                    chunk_ids: Some(chunk_ids),
                    parent_id: None,
                });
            } else {
                sections.push(DocSection {
                    id: item_id,
                    title: item.title.clone(),
                    category: category.clone(),
                    category_title: section.title.clone(),
                    path: page_path,
                    use_cases,
                    is_chunked: None,
                    chunk_ids: None,
                    parent_id: None,
                });
            }
        }
    }

    files.push((
        output_dir.join("sections.json"),
        crate::utils::json::to_string_pretty(&sections)?,
    ));

    format::success(&format!(
        "Extracted {} documentation sections",
        sections.len()
    ));

    GeneratedFiles::build(root, vec![output_dir.to_path_buf()], files)
}

fn parse_navigation(source: &str, patterns: &Patterns) -> Vec<NavSection> {
    patterns
        .nav_section
        .captures_iter(source)
        .filter_map(|section| {
            let items: Vec<NavItem> = patterns
                .nav_item
                .captures_iter(&section[2])
                .map(|item| NavItem {
                    title: item[1].to_string(),
                    href: item[2].to_string(),
                })
                .collect();
            (!items.is_empty()).then(|| NavSection {
                title: section[1].to_string(),
                items,
            })
        })
        .collect()
}

fn href_to_category(href: &str) -> String {
    let path = href.strip_prefix("/docs/").unwrap_or(href);
    path.split('/').next().unwrap_or_default().to_string()
}

fn href_to_id(href: &str) -> String {
    let path = href.strip_prefix("/docs/").unwrap_or(href);
    match path.rsplit_once('/') {
        Some((_, last)) => last.to_string(),
        None => CATEGORY_IDS
            .iter()
            .find(|(category, _)| *category == path)
            .map(|(_, id)| id.to_string())
            .unwrap_or_else(|| path.to_string()),
    }
}

fn href_to_prerendered_path(href: &str, prerendered_dir: &Path) -> PathBuf {
    let path = href.strip_prefix('/').unwrap_or(href);
    prerendered_dir.join(path).with_extension("html")
}

/// A page's mdsvex source without its boilerplate, or `None` for a page
/// written in Svelte.
fn read_markdown_source(
    href: &str,
    website_dir: &Path,
    patterns: &Patterns,
) -> Result<Option<String>> {
    let path = href.strip_prefix('/').unwrap_or(href);
    let source_path = website_dir.join("src/routes").join(path).join("+page.svx");
    if !source_path.exists() {
        return Ok(None);
    }
    let source = fs::read_to_string(&source_path)
        .with_context(|| format!("failed to read {}", source_path.display()))?;
    let without_comment = patterns.leading_comment.replace(&source, "");
    let without_head = patterns.svelte_head.replace_all(&without_comment, "");
    Ok(Some(format!("{}\n", without_head.trim())))
}

/// The page's prose, converted to markdown.
fn html_to_markdown(html: &str, patterns: &Patterns) -> Result<String> {
    let document = Html::parse_document(html);
    let prose = ["div.prose", "article"]
        .iter()
        .map(|selector| {
            Selector::parse(selector)
                .map_err(|error| anyhow::anyhow!("invalid selector {selector}: {error}"))
        })
        .collect::<Result<Vec<_>>>()?
        .iter()
        .find_map(|selector| {
            document
                .select(selector)
                .next()
                .map(|element| element.html())
        })
        .context("the page has no div.prose or article content")?;

    let converter = HtmlToMarkdown::builder()
        .skip_tags(vec!["script", "style", "svg", "button", "nav"])
        .build();
    let markdown = converter
        .convert(&prose)
        .context("htmd failed to convert the page")?;

    let collapsed = patterns.blank_lines.replace_all(&markdown, "\n\n");
    let decoded = collapsed
        .replace("&nbsp;", " ")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&#x27;", "'")
        .replace("&#123;", "{")
        .replace("&#125;", "}");
    let trimmed = decoded.trim();
    if trimmed.is_empty() {
        anyhow::bail!("the page's prose converted to empty markdown");
    }
    Ok(trimmed.to_string())
}

fn header_to_slug(header: &str, patterns: &Patterns) -> String {
    let unquoted = patterns.inline_code.replace_all(header, "$1");
    let without_numeric = patterns.numeric_entity.replace_all(&unquoted, "");
    let without_entities = patterns.named_entity.replace_all(&without_numeric, "");
    let kept: String = without_entities
        .to_lowercase()
        .chars()
        .filter(|character| character.is_alphanumeric() || *character == ' ' || *character == '-')
        .collect();
    let hyphenated = patterns.whitespace.replace_all(&kept, "-");
    patterns
        .hyphens
        .replace_all(&hyphenated, "-")
        .trim_matches('-')
        .to_string()
}

fn chunk_use_cases(content: &str, parent_use_cases: &str, patterns: &Patterns) -> String {
    let mut keywords: Vec<String> = parent_use_cases
        .split(',')
        .map(|keyword| keyword.trim().to_string())
        .take(2)
        .collect();
    keywords.extend(
        patterns
            .inline_code
            .captures_iter(content)
            .take(5)
            .map(|code| code[1].to_lowercase())
            .filter(|term| term.len() > 2 && term.len() < 30 && !term.contains(' ')),
    );
    keywords.dedup();
    keywords.truncate(6);
    keywords.join(", ")
}

fn chunk_markdown(markdown: &str, parent_title: &str, patterns: &Patterns) -> Vec<Chunk> {
    let parts: Vec<&str> = patterns.h2.split(markdown).collect();
    let headers: Vec<String> = patterns
        .h2
        .captures_iter(markdown)
        .map(|header| header[1].to_string())
        .collect();

    let mut chunks = Vec::new();
    let intro = parts.first().map(|part| part.trim()).unwrap_or_default();
    if intro.len() >= MIN_CHUNK_SIZE {
        chunks.push(Chunk {
            slug: "overview".to_string(),
            title: format!("{parent_title}: Overview"),
            content: intro.to_string(),
        });
    } else if !intro.is_empty() {
        chunks.push(Chunk {
            slug: "_intro".to_string(),
            title: String::new(),
            content: intro.to_string(),
        });
    }

    for (index, header) in headers.iter().enumerate() {
        let content = parts
            .get(index + 1)
            .map(|part| part.trim())
            .unwrap_or_default();
        let full_content = format!("## {header}\n\n{content}");

        // A small section joins the one before it.
        if full_content.len() < MIN_CHUNK_SIZE
            && let Some(previous) = chunks.last_mut()
            && previous.slug != "_intro"
        {
            previous.content.push_str("\n\n");
            previous.content.push_str(&full_content);
            continue;
        }

        chunks.push(Chunk {
            slug: header_to_slug(header, patterns),
            title: format!("{parent_title}: {header}"),
            content: full_content,
        });
    }

    // A short intro opens the first real chunk, which becomes the overview.
    if chunks.first().is_some_and(|first| first.slug == "_intro") {
        if chunks.len() > 1 {
            let intro = chunks.remove(0);
            chunks[0].content = format!("{}\n\n{}", intro.content, chunks[0].content);
        }
        chunks[0].slug = "overview".to_string();
        chunks[0].title = format!("{parent_title}: Overview");
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    fn patterns() -> Patterns {
        Patterns::compile().expect("the extraction patterns compile")
    }

    #[test]
    fn parses_navigation_sections_and_items() {
        let source = r#"
            export const navigation = [
                { title: 'Getting Started', items: [
                    { title: 'Installation', href: '/docs/getting-started' },
                    { title: 'First Macro', href: '/docs/getting-started/first-macro' }
                ] },
                { title: 'Empty', items: [] }
            ];
        "#;
        let sections = parse_navigation(source, &patterns());
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].title, "Getting Started");
        assert_eq!(sections[0].items.len(), 2);
        assert_eq!(
            sections[0].items[1].href,
            "/docs/getting-started/first-macro"
        );
    }

    #[test]
    fn category_index_pages_take_their_category_id() {
        assert_eq!(href_to_id("/docs/getting-started"), "installation");
        assert_eq!(href_to_id("/docs/builtin-macros/debug"), "debug");
        assert_eq!(href_to_id("/docs/unlisted"), "unlisted");
        assert_eq!(
            href_to_category("/docs/builtin-macros/debug"),
            "builtin-macros"
        );
    }

    #[test]
    fn slugs_drop_code_marks_entities_and_punctuation() {
        let patterns = patterns();
        assert_eq!(
            header_to_slug("The `toJSON()` Method", &patterns),
            "the-tojson-method"
        );
        assert_eq!(
            header_to_slug("A &amp; B &#39;quoted&#39;", &patterns),
            "a-b-quoted"
        );
    }

    #[test]
    fn short_sections_merge_and_a_short_intro_opens_the_overview() {
        let long_body = "word ".repeat(150);
        let markdown = format!(
            "Intro text.\n\n## First\n\n{long_body}\n\n## Tiny\n\nshort\n\n## Second\n\n{long_body}"
        );
        let chunks = chunk_markdown(&markdown, "Page", &patterns());
        let slugs: Vec<&str> = chunks.iter().map(|chunk| chunk.slug.as_str()).collect();
        assert_eq!(slugs, ["overview", "second"]);
        assert!(chunks[0].content.starts_with("Intro text.\n\n## First"));
        assert!(chunks[0].content.contains("## Tiny"));
        assert_eq!(chunks[0].title, "Page: Overview");
    }

    #[test]
    fn mdsvex_boilerplate_is_stripped() {
        let dir = tempfile::tempdir().expect("a temp dir");
        let page = dir.path().join("src/routes/docs/page");
        fs::create_dir_all(&page).expect("the page dir");
        fs::write(
            page.join("+page.svx"),
            "<!-- generated -->\n<svelte:head><title>T</title></svelte:head>\n# Title\n\nBody\n",
        )
        .expect("the page");
        let markdown = read_markdown_source("/docs/page", dir.path(), &patterns())
            .expect("the source reads")
            .expect("the page has a source");
        assert_eq!(markdown, "# Title\n\nBody\n");
    }

    #[test]
    fn html_prose_converts_to_markdown() {
        let html = "<html><body><nav>skip</nav><div class=\"prose\"><h2>Use &lt;T&gt;</h2><p>Text</p></div></body></html>";
        let markdown = html_to_markdown(html, &patterns()).expect("the page converts");
        assert_eq!(markdown, "## Use \\<T>\n\nText");
    }
}
