#!/usr/bin/env rust-script
//! Extract documentation from Rust source files.
//!
//! Parses //! (module docs) and /// (item docs) comments from Rust crates
//! and outputs structured JSON for website/README generation.
//!
//! ```cargo
//! [dependencies]
//! clap = { version = "4", features = ["derive"] }
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! pulldown-cmark = "0.9"
//! regex = "1"
//! toml = "0.8"
//! walkdir = "2"
//! syn = { version = "2", features = ["full", "extra-traits"] }
//! quote = "1"
//! proc-macro2 = "1"
//! ```

use clap::Parser;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser as MdParser, Tag};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use syn::{Attribute, Item, Lit, Meta};

// ============================================================================
// CLI
// ============================================================================

#[derive(Parser, Debug)]
#[command(name = "extract-rust-docs")]
#[command(about = "Extract documentation from Rust source files into JSON")]
struct Args {
    /// Output directory for JSON files
    #[arg(default_value = "website/static/api-data/rust")]
    output_dir: PathBuf,

    /// Path to crates directory
    #[arg(long, default_value = "crates")]
    crates_dir: PathBuf,

    /// Skip syncing expanded output to Rust docs
    #[arg(long)]
    skip_sync: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
struct CrateDoc {
    name: String,
    kind: String,
    version: String,
    description: String,
    overview: String,
    items: Vec<ItemDoc>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ItemDoc {
    name: String,
    kind: String,
    signature: String,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Vec<ParamDoc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    returns: Option<ReturnDoc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    examples: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    panics: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ParamDoc {
    name: String,
    #[serde(rename = "type")]
    param_type: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReturnDoc {
    #[serde(rename = "type")]
    return_type: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BuiltinMacro {
    name: String,
    slug: String,
    description: String,
    #[serde(rename = "generatedOutput")]
    generated_output: String,
    #[serde(rename = "fieldOptions")]
    field_options: String,
    example: String,
    #[serde(rename = "exampleCode")]
    example_code: Vec<CodeBlock>,
    title: String,
    raw: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CodeBlock {
    lang: String,
    code: String,
    #[serde(rename = "rawCode")]
    raw_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CliDoc {
    name: String,
    version: String,
    title: String,
    description: String,
    commands: HashMap<String, String>,
    #[serde(rename = "outputFileNaming")]
    output_file_naming: String,
    #[serde(rename = "exitCodes")]
    exit_codes: String,
    #[serde(rename = "nodeIntegration")]
    node_integration: String,
    functions: Vec<ItemDoc>,
    raw: String,
}

#[derive(Debug, Serialize)]
struct IndexDoc {
    generated: String,
    crates: Vec<CrateSummary>,
    #[serde(rename = "builtinMacros")]
    builtin_macros: Vec<String>,
    cli: Option<CliSummary>,
}

#[derive(Debug, Serialize)]
struct CrateSummary {
    name: String,
    version: String,
    #[serde(rename = "itemCount")]
    item_count: usize,
}

#[derive(Debug, Serialize)]
struct CliSummary {
    version: String,
    #[serde(rename = "commandCount")]
    command_count: usize,
}

// ============================================================================
// Crate Configuration
// ============================================================================

struct CrateConfig {
    name: &'static str,
    display_name: &'static str,
    entry_file: &'static str,
}

const CRATES: &[CrateConfig] = &[
    CrateConfig { name: "macroforge_ts", display_name: "macroforge_ts", entry_file: "src/lib.rs" },
    CrateConfig { name: "macroforge_ts_syn", display_name: "macroforge_ts_syn", entry_file: "src/lib.rs" },
    CrateConfig { name: "macroforge_ts_quote", display_name: "macroforge_ts_quote", entry_file: "src/lib.rs" },
    CrateConfig { name: "macroforge_ts_macros", display_name: "macroforge_ts_macros", entry_file: "src/lib.rs" },
];

struct MacroConfig {
    name: &'static str,
    file: &'static str,
    display_name: &'static str,
}

const BUILTIN_MACROS: &[MacroConfig] = &[
    MacroConfig { name: "debug", file: "src/builtin/derive_debug.rs", display_name: "Debug" },
    MacroConfig { name: "clone", file: "src/builtin/derive_clone.rs", display_name: "Clone" },
    MacroConfig { name: "default", file: "src/builtin/derive_default.rs", display_name: "Default" },
    MacroConfig { name: "hash", file: "src/builtin/derive_hash.rs", display_name: "Hash" },
    MacroConfig { name: "ord", file: "src/builtin/derive_ord.rs", display_name: "Ord" },
    MacroConfig { name: "partial_eq", file: "src/builtin/derive_partial_eq.rs", display_name: "PartialEq" },
    MacroConfig { name: "partial_ord", file: "src/builtin/derive_partial_ord.rs", display_name: "PartialOrd" },
    MacroConfig { name: "serialize", file: "src/builtin/serde/derive_serialize.rs", display_name: "Serialize" },
    MacroConfig { name: "deserialize", file: "src/builtin/serde/derive_deserialize.rs", display_name: "Deserialize" },
];

// ============================================================================
// Cargo.toml Parsing
// ============================================================================

#[derive(Debug, Deserialize)]
struct CargoToml {
    package: Option<CargoPackage>,
    workspace: Option<CargoWorkspace>,
}

#[derive(Debug, Deserialize)]
struct CargoPackage {
    name: Option<String>,
    version: Option<toml::Value>,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CargoWorkspace {
    package: Option<WorkspacePackage>,
}

#[derive(Debug, Deserialize)]
struct WorkspacePackage {
    version: Option<String>,
}

fn parse_cargo_toml(cargo_path: &Path, crates_root: &Path) -> (String, String, String) {
    let content = match fs::read_to_string(cargo_path) {
        Ok(c) => c,
        Err(_) => return (String::new(), String::new(), String::new()),
    };

    let cargo: CargoToml = match toml::from_str(&content) {
        Ok(c) => c,
        Err(_) => return (String::new(), String::new(), String::new()),
    };

    let name = cargo.package.as_ref()
        .and_then(|p| p.name.clone())
        .unwrap_or_default();

    let description = cargo.package.as_ref()
        .and_then(|p| p.description.clone())
        .unwrap_or_default();

    // Handle workspace version inheritance
    let version = if let Some(pkg) = &cargo.package {
        match &pkg.version {
            Some(toml::Value::String(v)) => v.clone(),
            Some(toml::Value::Table(t)) if t.get("workspace").map(|v| v.as_bool()) == Some(Some(true)) => {
                // Read from workspace Cargo.toml
                let workspace_path = crates_root.join("Cargo.toml");
                if let Ok(ws_content) = fs::read_to_string(&workspace_path) {
                    if let Ok(ws_cargo) = toml::from_str::<CargoToml>(&ws_content) {
                        ws_cargo.workspace
                            .and_then(|w| w.package)
                            .and_then(|p| p.version)
                            .unwrap_or_default()
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                }
            }
            _ => String::new(),
        }
    } else {
        String::new()
    };

    (name, version, description)
}

// ============================================================================
// Rust Doc Comment Extraction (using syn)
// ============================================================================

/// Extract doc comments from attributes (both `///` and `#[doc = "..."]`)
fn extract_doc_from_attrs(attrs: &[Attribute]) -> String {
    let mut doc_lines = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let Meta::NameValue(meta) = &attr.meta {
                if let syn::Expr::Lit(expr_lit) = &meta.value {
                    if let Lit::Str(lit_str) = &expr_lit.lit {
                        let content = lit_str.value();
                        // Remove leading space that rustdoc adds
                        let content = content.strip_prefix(' ').unwrap_or(&content);
                        doc_lines.push(content.to_string());
                    }
                }
            }
        }
    }

    doc_lines.join("\n")
}

/// Extract module-level documentation (//! comments) from source
fn extract_module_docs(source: &str) -> String {
    // For module docs, we need to extract //! comments which syn parses as inner attributes
    // But syn only sees them if they're attached to a module. For top-level files,
    // we need to parse manually or use a different approach.

    // First try to parse as a file and get inner attributes
    if let Ok(file) = syn::parse_file(source) {
        let doc = extract_doc_from_attrs(&file.attrs);
        if !doc.is_empty() {
            return doc;
        }
    }

    // Fallback: manual extraction for //! comments at the top of the file
    let mut doc_lines = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("//!") {
            let content = &trimmed[3..];
            let content = content.strip_prefix(' ').unwrap_or(content);
            doc_lines.push(content.to_string());
        } else if trimmed.is_empty() && !doc_lines.is_empty() {
            doc_lines.push(String::new());
        } else if !trimmed.starts_with("//") && !trimmed.is_empty() {
            break;
        }
    }

    // Trim trailing empty lines
    while doc_lines.last().map(|s| s.is_empty()).unwrap_or(false) {
        doc_lines.pop();
    }

    doc_lines.join("\n")
}

/// Extract documented items from Rust source using syn
fn extract_item_docs(source: &str) -> Vec<ItemDoc> {
    let file = match syn::parse_file(source) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };

    let mut items = Vec::new();

    for item in &file.items {
        if let Some(doc_item) = extract_item_doc(item) {
            items.push(doc_item);
        }
    }

    items
}

/// Extract documentation from a single syn Item
fn extract_item_doc(item: &Item) -> Option<ItemDoc> {
    match item {
        Item::Fn(item_fn) => {
            let doc = extract_doc_from_attrs(&item_fn.attrs);
            if doc.is_empty() && !is_public(&item_fn.vis) {
                return None;
            }
            if !is_public(&item_fn.vis) {
                return None;
            }

            let name = item_fn.sig.ident.to_string();
            let signature = format_fn_signature(&item_fn.sig, &item_fn.vis);
            let (description, sections) = parse_doc_sections_from_str(&doc);

            Some(ItemDoc {
                name,
                kind: "function".to_string(),
                signature,
                description,
                params: parse_params(sections.get("arguments")),
                returns: parse_returns(sections.get("returns")),
                examples: parse_examples(sections.get("example").or(sections.get("examples"))),
                errors: sections.get("errors").cloned(),
                panics: sections.get("panics").cloned(),
                notes: sections.get("note").or(sections.get("notes")).cloned(),
            })
        }
        Item::Struct(item_struct) => {
            let doc = extract_doc_from_attrs(&item_struct.attrs);
            if !is_public(&item_struct.vis) {
                return None;
            }

            let name = item_struct.ident.to_string();
            let generics = if item_struct.generics.params.is_empty() {
                String::new()
            } else {
                format!("<{}>", item_struct.generics.params.iter()
                    .map(|p| quote::quote!(#p).to_string())
                    .collect::<Vec<_>>()
                    .join(", "))
            };
            let signature = format!("pub struct {}{}", name, generics);
            let (description, sections) = parse_doc_sections_from_str(&doc);

            Some(ItemDoc {
                name,
                kind: "struct".to_string(),
                signature,
                description,
                params: None,
                returns: None,
                examples: parse_examples(sections.get("example").or(sections.get("examples"))),
                errors: None,
                panics: None,
                notes: sections.get("note").or(sections.get("notes")).cloned(),
            })
        }
        Item::Enum(item_enum) => {
            let doc = extract_doc_from_attrs(&item_enum.attrs);
            if !is_public(&item_enum.vis) {
                return None;
            }

            let name = item_enum.ident.to_string();
            let signature = format!("pub enum {}", name);
            let (description, sections) = parse_doc_sections_from_str(&doc);

            Some(ItemDoc {
                name,
                kind: "enum".to_string(),
                signature,
                description,
                params: None,
                returns: None,
                examples: parse_examples(sections.get("example").or(sections.get("examples"))),
                errors: None,
                panics: None,
                notes: sections.get("note").or(sections.get("notes")).cloned(),
            })
        }
        Item::Type(item_type) => {
            let doc = extract_doc_from_attrs(&item_type.attrs);
            if !is_public(&item_type.vis) {
                return None;
            }

            let name = item_type.ident.to_string();
            let signature = format!("pub type {} = ...", name);
            let (description, sections) = parse_doc_sections_from_str(&doc);

            Some(ItemDoc {
                name,
                kind: "type".to_string(),
                signature,
                description,
                params: None,
                returns: None,
                examples: parse_examples(sections.get("example").or(sections.get("examples"))),
                errors: None,
                panics: None,
                notes: sections.get("note").or(sections.get("notes")).cloned(),
            })
        }
        Item::Trait(item_trait) => {
            let doc = extract_doc_from_attrs(&item_trait.attrs);
            if !is_public(&item_trait.vis) {
                return None;
            }

            let name = item_trait.ident.to_string();
            let signature = format!("pub trait {}", name);
            let (description, sections) = parse_doc_sections_from_str(&doc);

            Some(ItemDoc {
                name,
                kind: "trait".to_string(),
                signature,
                description,
                params: None,
                returns: None,
                examples: parse_examples(sections.get("example").or(sections.get("examples"))),
                errors: None,
                panics: None,
                notes: sections.get("note").or(sections.get("notes")).cloned(),
            })
        }
        Item::Mod(item_mod) => {
            let doc = extract_doc_from_attrs(&item_mod.attrs);
            if !is_public(&item_mod.vis) {
                return None;
            }

            let name = item_mod.ident.to_string();
            let signature = format!("pub mod {}", name);
            let (description, _) = parse_doc_sections_from_str(&doc);

            Some(ItemDoc {
                name,
                kind: "module".to_string(),
                signature,
                description,
                params: None,
                returns: None,
                examples: None,
                errors: None,
                panics: None,
                notes: None,
            })
        }
        _ => None,
    }
}

/// Check if an item has public visibility
fn is_public(vis: &syn::Visibility) -> bool {
    matches!(vis, syn::Visibility::Public(_))
}

/// Format a function signature for display
fn format_fn_signature(sig: &syn::Signature, vis: &syn::Visibility) -> String {
    let vis_str = if is_public(vis) { "pub " } else { "" };
    let async_str = if sig.asyncness.is_some() { "async " } else { "" };
    let fn_name = &sig.ident;

    let generics = if sig.generics.params.is_empty() {
        String::new()
    } else {
        format!("<{}>", sig.generics.params.iter()
            .map(|p| quote::quote!(#p).to_string())
            .collect::<Vec<_>>()
            .join(", "))
    };

    let inputs: Vec<String> = sig.inputs.iter()
        .map(|arg| quote::quote!(#arg).to_string())
        .collect();

    let output = match &sig.output {
        syn::ReturnType::Default => String::new(),
        syn::ReturnType::Type(_, ty) => format!(" -> {}", quote::quote!(#ty)),
    };

    format!("{}{}fn {}{}({}){}", vis_str, async_str, fn_name, generics, inputs.join(", "), output)
}

/// Wrapper to parse doc sections from a string
fn parse_doc_sections_from_str(doc: &str) -> (String, HashMap<String, String>) {
    let lines: Vec<String> = doc.lines().map(|s| s.to_string()).collect();
    parse_doc_sections(&lines)
}

fn parse_doc_sections(doc_lines: &[String]) -> (String, HashMap<String, String>) {
    let mut sections: HashMap<String, String> = HashMap::new();
    let mut current_section: Option<String> = None;
    let mut current_content: Vec<String> = Vec::new();
    let mut description: Vec<String> = Vec::new();

    let section_re = Regex::new(r"^#\s+(.+)$").unwrap();

    for line in doc_lines {
        if let Some(caps) = section_re.captures(line) {
            // Save previous section
            if let Some(section) = current_section.take() {
                sections.insert(section, current_content.join("\n").trim().to_string());
            }
            current_section = Some(
                caps.get(1).unwrap().as_str()
                    .to_lowercase()
                    .replace(' ', "_")
            );
            current_content.clear();
        } else if current_section.is_some() {
            current_content.push(line.clone());
        } else {
            description.push(line.clone());
        }
    }

    // Save last section
    if let Some(section) = current_section {
        sections.insert(section, current_content.join("\n").trim().to_string());
    }

    (description.join("\n").trim().to_string(), sections)
}

fn parse_params(content: Option<&String>) -> Option<Vec<ParamDoc>> {
    let content = content?;
    let param_re = Regex::new(r"\*\s*`(\w+)`\s*-\s*(.+)").unwrap();
    let params: Vec<ParamDoc> = param_re.captures_iter(content)
        .map(|caps| ParamDoc {
            name: caps.get(1).unwrap().as_str().to_string(),
            param_type: String::new(),
            description: caps.get(2).unwrap().as_str().trim().to_string(),
        })
        .collect();

    if params.is_empty() { None } else { Some(params) }
}

fn parse_returns(content: Option<&String>) -> Option<ReturnDoc> {
    content.map(|c| ReturnDoc {
        return_type: String::new(),
        description: c.trim().to_string(),
    })
}

fn parse_examples(content: Option<&String>) -> Option<Vec<String>> {
    let content = content?;
    let code_re = Regex::new(r"```(?:\w+)?\n([\s\S]*?)```").unwrap();
    let examples: Vec<String> = code_re.captures_iter(content)
        .map(|caps| caps.get(1).unwrap().as_str().trim().to_string())
        .collect();

    if examples.is_empty() { None } else { Some(examples) }
}

// ============================================================================
// Markdown Code Block Transformation
// ============================================================================

/// Check if code contains @derive decorator
fn contains_derive(code: &str) -> bool {
    let derive_re = Regex::new(r"@derive\s*\(").unwrap();
    let jsdoc_re = Regex::new(r"/\*\*[\s\S]*?@derive").unwrap();
    derive_re.is_match(code) || jsdoc_re.is_match(code)
}

/// Check if a language is TypeScript/JavaScript
fn is_ts_or_js(lang: &str) -> bool {
    let lower = lang.to_lowercase();
    matches!(lower.as_str(), "typescript" | "ts" | "javascript" | "js" | "")
}

/// Represents a parsed code block
#[derive(Debug, Clone)]
struct ParsedCodeBlock {
    lang: String,
    code: String,
    start_offset: usize,
    end_offset: usize,
}

/// Transform markdown to convert "code + Generated output: + code" patterns into before/after pairs
fn transform_macro_blocks(input: &str) -> String {
    // Collect all code blocks with their positions using pulldown-cmark
    let mut code_blocks: Vec<ParsedCodeBlock> = Vec::new();
    let mut current_block: Option<(String, String, usize)> = None;

    let options = Options::empty();
    let parser = MdParser::new_ext(input, options);

    for (event, range) in parser.into_offset_iter() {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                let lang = match kind {
                    CodeBlockKind::Fenced(lang) => lang.to_string(),
                    CodeBlockKind::Indented => String::new(),
                };
                current_block = Some((lang, String::new(), range.start));
            }
            Event::Text(text) => {
                if let Some((_, ref mut code, _)) = current_block {
                    code.push_str(&text);
                }
            }
            Event::End(Tag::CodeBlock(_)) => {
                if let Some((lang, code, start)) = current_block.take() {
                    code_blocks.push(ParsedCodeBlock {
                        lang,
                        code,
                        start_offset: start,
                        end_offset: range.end,
                    });
                }
            }
            _ => {}
        }
    }

    // Scan for the pattern: code block → "Generated output:" → code block
    let mut replacements: Vec<(usize, usize, String)> = Vec::new();
    let gen_output_re = Regex::new(r"(?i)\n\s*Generated output:\s*\n").unwrap();

    let mut i = 0;
    while i < code_blocks.len() {
        let block = &code_blocks[i];

        // Check if this block has @derive and is TypeScript/JavaScript
        if !contains_derive(&block.code) || !is_ts_or_js(&block.lang) {
            i += 1;
            continue;
        }

        // Look for "Generated output:" between this block and the next
        if i + 1 < code_blocks.len() {
            let next_block = &code_blocks[i + 1];
            let between = &input[block.end_offset..next_block.start_offset];

            if gen_output_re.is_match(between) {
                // Found the pattern! Create the replacement
                let lang = if block.lang.is_empty() { "typescript" } else { &block.lang };

                let replacement = format!(
                    "```{} before\n{}\n```\n\n```{} after\n{}\n```",
                    lang,
                    block.code.trim(),
                    lang,
                    next_block.code.trim()
                );

                replacements.push((block.start_offset, next_block.end_offset, replacement));

                // Skip the next block since we've consumed it
                i += 2;
                continue;
            }
        }

        i += 1;
    }

    // Apply replacements in reverse order to preserve offsets
    let mut result = input.to_string();
    for (start, end, replacement) in replacements.into_iter().rev() {
        result.replace_range(start..end, &replacement);
    }

    result
}

// ============================================================================
// Builtin Macro Processing
// ============================================================================

fn parse_module_doc_sections(module_docs: &str) -> BuiltinMacro {
    let mut title = String::new();
    let mut description = String::new();
    let mut generated_output = String::new();
    let mut field_options = String::new();
    let mut example = String::new();

    let lines: Vec<&str> = module_docs.lines().collect();
    let mut current_section = "description";
    let mut current_content: Vec<String> = Vec::new();

    let h1_re = Regex::new(r"^#\s+(.+)$").unwrap();
    let h2_re = Regex::new(r"^##\s+(.+)$").unwrap();

    for line in lines {
        if let Some(caps) = h1_re.captures(line) {
            title = caps.get(1).unwrap().as_str().to_string();
            continue;
        }

        if let Some(caps) = h2_re.captures(line) {
            // Save previous section
            match current_section {
                "description" => description = current_content.join("\n").trim().to_string(),
                "generatedOutput" => generated_output = current_content.join("\n").trim().to_string(),
                "fieldOptions" => field_options = current_content.join("\n").trim().to_string(),
                "example" => example = current_content.join("\n").trim().to_string(),
                _ => {}
            }

            let header = caps.get(1).unwrap().as_str().to_lowercase();
            current_section = if header.contains("generated") || header.contains("output") {
                "generatedOutput"
            } else if header.contains("field") || header.contains("option") || header.contains("decorator") {
                "fieldOptions"
            } else if header.contains("example") {
                "example"
            } else {
                "other"
            };
            current_content.clear();
            continue;
        }

        current_content.push(line.to_string());
    }

    // Save last section
    match current_section {
        "description" => description = current_content.join("\n").trim().to_string(),
        "generatedOutput" => generated_output = current_content.join("\n").trim().to_string(),
        "fieldOptions" => field_options = current_content.join("\n").trim().to_string(),
        "example" => example = current_content.join("\n").trim().to_string(),
        _ => {}
    }

    // Extract code blocks from example
    let example_code = extract_code_blocks(&example);

    BuiltinMacro {
        name: title.clone(),
        slug: String::new(), // Will be set later
        description,
        generated_output,
        field_options,
        example,
        example_code,
        title,
        raw: module_docs.to_string(),
    }
}

fn extract_code_blocks(content: &str) -> Vec<CodeBlock> {
    let re = Regex::new(r"```(\w+)?\n([\s\S]*?)```").unwrap();
    re.captures_iter(content)
        .map(|caps| {
            let lang = caps.get(1).map(|m| m.as_str()).unwrap_or("typescript").to_string();
            let code = caps.get(2).unwrap().as_str().trim().to_string();
            CodeBlock {
                lang: lang.clone(),
                code: code.clone(),
                raw_code: code,
            }
        })
        .collect()
}

// ============================================================================
// Output Generation
// ============================================================================

fn escape_comparisons_in_tables(md: &str) -> String {
    let lines: Vec<&str> = md.lines().collect();
    let mut in_code_block = false;
    let mut result = Vec::new();

    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            result.push(line.to_string());
            continue;
        }

        if in_code_block || !trimmed.starts_with('|') {
            result.push(line.to_string());
            continue;
        }

        // Escape comparisons in table cells
        let escaped = line
            .replace(" < ", " &lt; ")
            .replace(" > ", " &gt; ")
            .replace(" <= ", " &lt;= ")
            .replace(" >= ", " &gt;= ");
        result.push(escaped);
    }

    result.join("\n")
}

fn generate_svx_page(macro_info: &BuiltinMacro, expanded_md: &str) -> String {
    let description = if macro_info.description.is_empty() {
        format!("{} derive macro documentation.", macro_info.name)
    } else {
        macro_info.description.replace('"', "&quot;")
    };

    format!(
        r#"<!--
  Generated by rust-script scripts/extract-rust-docs.rs
  Do not edit manually — edit the Rust module docs instead.
-->

<svelte:head>
	<title>{} Macro - Macroforge Documentation</title>
	<meta name="description" content="{}" />
</svelte:head>

{}
"#,
        macro_info.name,
        description,
        expanded_md
    )
}

// ============================================================================
// Main Processing
// ============================================================================

fn process_crate(config: &CrateConfig, crates_root: &Path) -> Option<CrateDoc> {
    let crate_path = crates_root.join(config.name);
    let cargo_path = crate_path.join("Cargo.toml");
    let entry_path = crate_path.join(config.entry_file);

    if !entry_path.exists() {
        eprintln!("Warning: Entry file not found: {:?}", entry_path);
        return None;
    }

    let (_name, version, description) = parse_cargo_toml(&cargo_path, crates_root);
    let source = fs::read_to_string(&entry_path).ok()?;
    let overview = extract_module_docs(&source);
    let items = extract_item_docs(&source);

    // Filter to only documented items
    let public_items: Vec<ItemDoc> = items
        .into_iter()
        .filter(|item| !item.description.is_empty() || item.params.is_some())
        .collect();

    Some(CrateDoc {
        name: config.display_name.to_string(),
        kind: "rust_crate".to_string(),
        version: if version.is_empty() { "unknown".to_string() } else { version },
        description,
        overview,
        items: public_items,
    })
}

fn process_builtin_macros(crates_root: &Path) -> HashMap<String, BuiltinMacro> {
    let mut macros = HashMap::new();

    for config in BUILTIN_MACROS {
        let file_path = crates_root.join("macroforge_ts").join(config.file);

        if !file_path.exists() {
            eprintln!("Warning: Builtin macro file not found: {:?}", file_path);
            continue;
        }

        let source = match fs::read_to_string(&file_path) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let module_docs = extract_module_docs(&source);
        let mut macro_info = parse_module_doc_sections(&module_docs);
        macro_info.slug = config.name.replace('_', "-");
        macro_info.name = config.display_name.to_string();

        macros.insert(config.name.to_string(), macro_info);
    }

    macros
}

fn write_builtin_macro_pages(
    macros: &HashMap<String, BuiltinMacro>,
    repo_root: &Path,
) {
    let website_docs_root = repo_root.join("website/src/routes/docs/builtin-macros");
    let mcp_docs_root = repo_root.join("packages/mcp-server/docs/builtin-macros");

    // Regex to match H1 headers (must be at start of line)
    let h1_re = Regex::new(r"(?m)^#\s+[^\n]+\n*").unwrap();

    for (_key, macro_info) in macros {
        let slug = &macro_info.slug;

        // Normalize the markdown: escape comparisons in tables
        let raw = escape_comparisons_in_tables(&macro_info.raw);

        // Remove all H1 headers from the content and add our own
        let body = h1_re.replace_all(&raw, "").trim().to_string();
        let normalized = format!("# {}\n\n{}\n", macro_info.name, body);

        // Transform macro blocks (before/after pairs)
        let expanded = transform_macro_blocks(&normalized);

        // Write website SVX page
        let website_dir = website_docs_root.join(slug);
        fs::create_dir_all(&website_dir).ok();
        let svx_content = generate_svx_page(macro_info, &expanded);
        fs::write(website_dir.join("+page.svx"), svx_content).ok();

        // Write MCP markdown doc (plain, no transformation)
        fs::create_dir_all(&mcp_docs_root).ok();
        fs::write(mcp_docs_root.join(format!("{}.md", slug)), &normalized).ok();
    }
}

fn main() {
    let args = Args::parse();

    // Determine paths - use current directory as repo root
    let repo_root = std::env::current_dir().expect("Failed to get current directory");
    let crates_root = repo_root.join(&args.crates_dir);
    let output_dir = repo_root.join(&args.output_dir);

    if args.verbose {
        eprintln!("Repo root: {:?}", repo_root);
        eprintln!("Crates dir: {:?}", crates_root);
        eprintln!("Output dir: {:?}", output_dir);
    }

    // Create output directory
    fs::create_dir_all(&output_dir).expect("Failed to create output directory");

    println!("Extracting Rust documentation...\n");

    let mut all_docs = Vec::new();

    // Process crates
    for config in CRATES {
        println!("Processing {}...", config.name);
        if let Some(docs) = process_crate(config, &crates_root) {
            let out_path = output_dir.join(format!("{}.json", config.name));
            let json = serde_json::to_string_pretty(&docs).unwrap();
            fs::write(&out_path, json).expect("Failed to write JSON");
            println!("  -> {:?}", out_path);
            println!("     {} documented items", docs.items.len());
            all_docs.push(docs);
        }
    }

    // Process builtin macros
    println!("\nProcessing builtin macros...");
    let builtin_macros = process_builtin_macros(&crates_root);
    let builtin_path = output_dir.join("builtin-macros.json");
    let builtin_json = serde_json::to_string_pretty(&builtin_macros).unwrap();
    fs::write(&builtin_path, builtin_json).expect("Failed to write builtin macros JSON");
    println!("  -> {:?}", builtin_path);
    println!("     {} macros documented", builtin_macros.len());

    // Write SVX pages
    write_builtin_macro_pages(&builtin_macros, &repo_root);

    // Write index file
    let index = IndexDoc {
        generated: chrono_lite_now(),
        crates: all_docs.iter().map(|d| CrateSummary {
            name: d.name.clone(),
            version: d.version.clone(),
            item_count: d.items.len(),
        }).collect(),
        builtin_macros: builtin_macros.keys().cloned().collect(),
        cli: None, // TODO: Add CLI docs processing
    };

    let index_path = output_dir.join("index.json");
    let index_json = serde_json::to_string_pretty(&index).unwrap();
    fs::write(&index_path, index_json).expect("Failed to write index JSON");

    println!("\nWrote index: {:?}", index_path);
    println!("\nTotal: {} documented items", all_docs.iter().map(|d| d.items.len()).sum::<usize>());
    println!("Total: {} builtin macros", builtin_macros.len());
}

/// Simple ISO timestamp without external dependency
fn chrono_lite_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let secs = duration.as_secs();
    // Approximate ISO format
    format!("{}Z", secs)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_basic() {
        let input = r#"## Example

```typescript
/** @derive(Debug) */
class User {
    id: number;
}
```

Generated output:

```typescript
class User {
    id: number;
    static toString() {}
}
```
"#;

        let result = transform_macro_blocks(input);
        assert!(result.contains("```typescript before"));
        assert!(result.contains("```typescript after"));
        assert!(!result.contains("Generated output:"));
    }

    #[test]
    fn test_preserves_unrelated_blocks() {
        let input = r#"## JSON Format

```json
{"key": "value"}
```

Some text here.

## Example

```typescript
/** @derive(Debug) */
class User {}
```

Generated output:

```typescript
class User { static toString() {} }
```
"#;

        let result = transform_macro_blocks(input);
        // JSON block preserved
        assert!(result.contains("```json"));
        // Macro example transformed
        assert!(result.contains("```typescript before"));
        assert!(result.contains("```typescript after"));
        assert!(!result.contains("Generated output:"));
    }

    #[test]
    fn test_extract_module_docs() {
        let source = r#"//! # My Module
//!
//! This is the description.
//!
//! ## Example
//!
//! Some example.

use std::io;

fn main() {}
"#;

        let docs = extract_module_docs(source);
        assert!(docs.contains("# My Module"));
        assert!(docs.contains("This is the description"));
        assert!(docs.contains("## Example"));
    }

    #[test]
    fn test_contains_derive() {
        assert!(contains_derive("@derive(Debug)"));
        assert!(contains_derive("/** @derive(Serialize) */"));
        assert!(!contains_derive("class User {}"));
        assert!(!contains_derive("@serde({ rename: 'x' })"));
    }
}
