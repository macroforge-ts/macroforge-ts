#!/usr/bin/env rust-script
//! Extract documentation from TypeScript source files.
//!
//! Parses JSDoc comments from TypeScript packages using swc
//! and outputs structured JSON for website/README generation.
//!
//! ```cargo
//! [dependencies]
//! clap = { version = "4.5.53", features = ["derive"] }
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! swc_core = { version = "51.0.0", features = ["common", "ecma_parser", "ecma_ast"] }
//! regex = "1"
//! ```

use clap::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use swc_core::common::{
    comments::{CommentKind, Comments, SingleThreadedComments},
    sync::Lrc,
    BytePos, FileName, SourceMap, Span, Spanned,
};
use swc_core::ecma::ast::*;
use swc_core::ecma::parser::{lexer::Lexer, Parser as SwcParser, StringInput, Syntax, TsSyntax};

// ============================================================================
// CLI
// ============================================================================

#[derive(Parser, Debug)]
#[command(name = "extract-ts-docs")]
#[command(about = "Extract documentation from TypeScript source files into JSON")]
struct Args {
    /// Output directory for JSON files
    #[arg(default_value = "website/static/api-data/typescript")]
    output_dir: PathBuf,

    /// Path to packages directory
    #[arg(long, default_value = "packages")]
    packages_dir: PathBuf,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
struct PackageDoc {
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
    remarks: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    see: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deprecated: Option<String>,
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

#[derive(Debug, Serialize)]
struct IndexDoc {
    generated: String,
    packages: Vec<PackageSummary>,
}

#[derive(Debug, Serialize)]
struct PackageSummary {
    name: String,
    version: String,
    #[serde(rename = "itemCount")]
    item_count: usize,
    #[serde(rename = "hasOverview")]
    has_overview: bool,
}

// ============================================================================
// Package Configuration
// ============================================================================

struct PackageConfig {
    name: &'static str,
    display_name: &'static str,
    entry_file: &'static str,
}

const PACKAGES: &[PackageConfig] = &[
    PackageConfig {
        name: "shared",
        display_name: "@macroforge/shared",
        entry_file: "src/index.ts",
    },
    PackageConfig {
        name: "typescript-plugin",
        display_name: "@macroforge/typescript-plugin",
        entry_file: "src/index.ts",
    },
    PackageConfig {
        name: "vite-plugin",
        display_name: "@macroforge/vite-plugin",
        entry_file: "src/index.ts",
    },
    PackageConfig {
        name: "svelte-preprocessor",
        display_name: "@macroforge/svelte-preprocessor",
        entry_file: "src/index.ts",
    },
    PackageConfig {
        name: "mcp-server",
        display_name: "@macroforge/mcp-server",
        entry_file: "src/index.ts",
    },
];

// ============================================================================
// Package.json Parsing
// ============================================================================

#[derive(Debug, Deserialize)]
struct PackageJson {
    name: Option<String>,
    version: Option<String>,
    description: Option<String>,
}

fn parse_package_json(pkg_path: &Path) -> (String, String, String) {
    let content = match fs::read_to_string(pkg_path) {
        Ok(c) => c,
        Err(_) => return (String::new(), String::new(), String::new()),
    };

    let pkg: PackageJson = match serde_json::from_str(&content) {
        Ok(p) => p,
        Err(_) => return (String::new(), String::new(), String::new()),
    };

    (
        pkg.name.unwrap_or_default(),
        pkg.version.unwrap_or_default(),
        pkg.description.unwrap_or_default(),
    )
}

// ============================================================================
// JSDoc Parsing
// ============================================================================

#[derive(Debug, Default)]
struct ParsedJSDoc {
    description: String,
    params: Vec<ParamDoc>,
    returns: Option<ReturnDoc>,
    examples: Vec<String>,
    remarks: Option<String>,
    see: Vec<String>,
    internal: bool,
    deprecated: Option<String>,
}

fn parse_jsdoc(comment: &str) -> ParsedJSDoc {
    let mut result = ParsedJSDoc::default();

    // Remove /** and */ and normalize lines
    let content = comment
        .trim_start_matches("/**")
        .trim_end_matches("*/")
        .trim();

    let lines: Vec<&str> = content
        .lines()
        .map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with('*') {
                trimmed[1..].trim_start()
            } else {
                trimmed
            }
        })
        .collect();

    let mut current_tag: Option<&str> = None;
    let mut current_content: Vec<String> = Vec::new();
    let mut description_lines: Vec<String> = Vec::new();

    let tag_re = Regex::new(r"^@(\w+)\s*(.*)$").unwrap();

    for line in lines {
        if let Some(caps) = tag_re.captures(line) {
            // Save previous tag content
            if let Some(tag) = current_tag {
                save_tag_content(&mut result, tag, &current_content);
            }
            current_tag = Some(caps.get(1).unwrap().as_str());
            let rest = caps.get(2).map(|m| m.as_str()).unwrap_or("");
            current_content = if rest.is_empty() {
                Vec::new()
            } else {
                vec![rest.to_string()]
            };
        } else if current_tag.is_some() {
            current_content.push(line.to_string());
        } else {
            description_lines.push(line.to_string());
        }
    }

    // Save last tag
    if let Some(tag) = current_tag {
        save_tag_content(&mut result, tag, &current_content);
    }

    result.description = description_lines.join("\n").trim().to_string();
    result
}

fn save_tag_content(result: &mut ParsedJSDoc, tag: &str, content: &[String]) {
    let text = content.join("\n").trim().to_string();

    match tag {
        "param" => {
            // Parse @param {type} name - description
            let param_re = Regex::new(r"^(?:\{([^}]+)\}\s+)?(\w+)\s*-?\s*([\s\S]*)$").unwrap();
            if let Some(caps) = param_re.captures(&text) {
                result.params.push(ParamDoc {
                    name: caps.get(2).unwrap().as_str().to_string(),
                    param_type: caps.get(1).map(|m| m.as_str()).unwrap_or("").to_string(),
                    description: caps
                        .get(3)
                        .map(|m| m.as_str())
                        .unwrap_or("")
                        .trim()
                        .to_string(),
                });
            }
        }
        "returns" | "return" => {
            // Parse @returns {type} description
            let return_re = Regex::new(r"^(?:\{([^}]+)\}\s*)?([\s\S]*)$").unwrap();
            if let Some(caps) = return_re.captures(&text) {
                result.returns = Some(ReturnDoc {
                    return_type: caps.get(1).map(|m| m.as_str()).unwrap_or("").to_string(),
                    description: caps
                        .get(2)
                        .map(|m| m.as_str())
                        .unwrap_or("")
                        .trim()
                        .to_string(),
                });
            }
        }
        "example" => {
            result.examples.push(text);
        }
        "remarks" => {
            result.remarks = Some(text);
        }
        "see" => {
            result.see.push(text);
        }
        "internal" => {
            result.internal = true;
        }
        "deprecated" => {
            result.deprecated = Some(if text.is_empty() {
                "true".to_string()
            } else {
                text
            });
        }
        "fileoverview" | "module" => {
            // These are handled separately for file overview
            if result.description.is_empty() {
                result.description = text;
            }
        }
        _ => {}
    }
}

// ============================================================================
// TypeScript Parsing with swc
// ============================================================================

fn extract_file_overview(source: &str, comments: &SingleThreadedComments) -> String {
    // Get leading comments (comments at the start of the file)
    let (leading, _) = comments.borrow_all();

    for (_, comment_vec) in leading.iter() {
        for comment in comment_vec {
            if comment.kind == CommentKind::Block {
                let text = &comment.text;
                if text.contains("@fileoverview") || text.contains("@module") {
                    let parsed = parse_jsdoc(&format!("/**{}*/", text));
                    return parsed.description;
                }
            }
        }
    }

    String::new()
}

fn extract_item_docs(
    source: &str,
    comments: &SingleThreadedComments,
    module: &Module,
) -> Vec<ItemDoc> {
    let mut items = Vec::new();

    for item in &module.body {
        match item {
            ModuleItem::ModuleDecl(decl) => {
                if let Some(doc_item) = extract_from_module_decl(decl, source, comments) {
                    items.push(doc_item);
                }
            }
            ModuleItem::Stmt(stmt) => {
                if let Some(doc_item) = extract_from_stmt(stmt, source, comments) {
                    items.push(doc_item);
                }
            }
        }
    }

    items
}

fn extract_from_module_decl(
    decl: &ModuleDecl,
    source: &str,
    comments: &SingleThreadedComments,
) -> Option<ItemDoc> {
    match decl {
        ModuleDecl::ExportDecl(export_decl) => {
            extract_from_decl(&export_decl.decl, export_decl.span, source, comments)
        }
        ModuleDecl::ExportDefaultDecl(export_default) => {
            extract_from_default_decl(export_default, source, comments)
        }
        _ => None,
    }
}

fn extract_from_stmt(
    stmt: &Stmt,
    source: &str,
    comments: &SingleThreadedComments,
) -> Option<ItemDoc> {
    match stmt {
        Stmt::Decl(decl) => extract_from_decl(decl, stmt.span(), source, comments),
        _ => None,
    }
}

fn extract_from_decl(
    decl: &Decl,
    span: Span,
    source: &str,
    comments: &SingleThreadedComments,
) -> Option<ItemDoc> {
    // Find leading JSDoc comment
    let jsdoc = find_leading_jsdoc(span.lo, comments)?;
    let parsed = parse_jsdoc(&jsdoc);

    // Skip internal items
    if parsed.internal {
        return None;
    }

    match decl {
        Decl::Fn(fn_decl) => {
            let name = fn_decl.ident.sym.to_string();
            let signature = extract_fn_signature(&fn_decl.function, &name, source);
            Some(create_item_doc(name, "function", signature, parsed))
        }
        Decl::Class(class_decl) => {
            let name = class_decl.ident.sym.to_string();
            Some(create_item_doc(
                name.clone(),
                "class",
                format!("class {}", name),
                parsed,
            ))
        }
        Decl::Var(var_decl) => {
            if let Some(decl) = var_decl.decls.first() {
                if let Pat::Ident(ident) = &decl.name {
                    let name = ident.id.sym.to_string();
                    let kind = match var_decl.kind {
                        VarDeclKind::Const => "const",
                        VarDeclKind::Let => "let",
                        VarDeclKind::Var => "var",
                    };
                    return Some(create_item_doc(
                        name.clone(),
                        "variable",
                        format!("{} {}", kind, name),
                        parsed,
                    ));
                }
            }
            None
        }
        Decl::TsInterface(interface) => {
            let name = interface.id.sym.to_string();
            Some(create_item_doc(
                name.clone(),
                "interface",
                format!("interface {}", name),
                parsed,
            ))
        }
        Decl::TsTypeAlias(type_alias) => {
            let name = type_alias.id.sym.to_string();
            Some(create_item_doc(
                name.clone(),
                "type",
                format!("type {}", name),
                parsed,
            ))
        }
        Decl::TsEnum(enum_decl) => {
            let name = enum_decl.id.sym.to_string();
            Some(create_item_doc(
                name.clone(),
                "enum",
                format!("enum {}", name),
                parsed,
            ))
        }
        _ => None,
    }
}

fn extract_from_default_decl(
    export_default: &ExportDefaultDecl,
    source: &str,
    comments: &SingleThreadedComments,
) -> Option<ItemDoc> {
    let jsdoc = find_leading_jsdoc(export_default.span.lo, comments)?;
    let parsed = parse_jsdoc(&jsdoc);

    if parsed.internal {
        return None;
    }

    match &export_default.decl {
        DefaultDecl::Fn(fn_expr) => {
            let name = fn_expr
                .ident
                .as_ref()
                .map(|i| i.sym.to_string())
                .unwrap_or_else(|| "default".to_string());
            let signature = extract_fn_signature(&fn_expr.function, &name, source);
            Some(create_item_doc(name, "function", signature, parsed))
        }
        DefaultDecl::Class(class_expr) => {
            let name = class_expr
                .ident
                .as_ref()
                .map(|i| i.sym.to_string())
                .unwrap_or_else(|| "default".to_string());
            Some(create_item_doc(
                name.clone(),
                "class",
                format!("class {}", name),
                parsed,
            ))
        }
        _ => None,
    }
}

fn find_leading_jsdoc(pos: BytePos, comments: &SingleThreadedComments) -> Option<String> {
    // Get leading comments for this position
    let leading = comments.take_leading(pos);
    if let Some(comment_vec) = leading {
        let result = comment_vec.iter().rev().find_map(|comment| {
            if comment.kind == CommentKind::Block && comment.text.starts_with('*') {
                Some(format!("/**{}*/", comment.text))
            } else {
                None
            }
        });
        // Put comments back
        comments.add_leading_comments(pos, comment_vec);
        return result;
    }
    None
}

fn extract_fn_signature(func: &Function, name: &str, _source: &str) -> String {
    let async_str = if func.is_async { "async " } else { "" };

    let params: Vec<String> = func.params.iter().map(|p| format_param(&p.pat)).collect();

    let return_type = func
        .return_type
        .as_ref()
        .map(|rt| format!(": {}", format_ts_type(&rt.type_ann)))
        .unwrap_or_default();

    format!(
        "{}function {}({}){}",
        async_str,
        name,
        params.join(", "),
        return_type
    )
}

fn format_param(pat: &Pat) -> String {
    match pat {
        Pat::Ident(ident) => {
            let name = ident.id.sym.to_string();
            let type_ann = ident
                .type_ann
                .as_ref()
                .map(|ta| format!(": {}", format_ts_type(&ta.type_ann)))
                .unwrap_or_default();
            format!("{}{}", name, type_ann)
        }
        Pat::Rest(rest) => format!("...{}", format_param(&rest.arg)),
        Pat::Assign(assign) => format_param(&assign.left),
        Pat::Object(_) => "options".to_string(),
        Pat::Array(_) => "items".to_string(),
        _ => "arg".to_string(),
    }
}

fn format_ts_type(ts_type: &TsType) -> String {
    match ts_type {
        TsType::TsKeywordType(kw) => match kw.kind {
            TsKeywordTypeKind::TsStringKeyword => "string".to_string(),
            TsKeywordTypeKind::TsNumberKeyword => "number".to_string(),
            TsKeywordTypeKind::TsBooleanKeyword => "boolean".to_string(),
            TsKeywordTypeKind::TsVoidKeyword => "void".to_string(),
            TsKeywordTypeKind::TsAnyKeyword => "any".to_string(),
            TsKeywordTypeKind::TsNullKeyword => "null".to_string(),
            TsKeywordTypeKind::TsUndefinedKeyword => "undefined".to_string(),
            TsKeywordTypeKind::TsNeverKeyword => "never".to_string(),
            TsKeywordTypeKind::TsUnknownKeyword => "unknown".to_string(),
            TsKeywordTypeKind::TsObjectKeyword => "object".to_string(),
            TsKeywordTypeKind::TsBigIntKeyword => "bigint".to_string(),
            TsKeywordTypeKind::TsSymbolKeyword => "symbol".to_string(),
            TsKeywordTypeKind::TsIntrinsicKeyword => "intrinsic".to_string(),
        },
        TsType::TsTypeRef(type_ref) => {
            if let TsEntityName::Ident(ident) = &type_ref.type_name {
                ident.sym.to_string()
            } else {
                "unknown".to_string()
            }
        }
        TsType::TsArrayType(arr) => format!("{}[]", format_ts_type(&arr.elem_type)),
        TsType::TsUnionOrIntersectionType(TsUnionOrIntersectionType::TsUnionType(union)) => union
            .types
            .iter()
            .map(|t| format_ts_type(t))
            .collect::<Vec<_>>()
            .join(" | "),
        TsType::TsTypeLit(_) => "object".to_string(),
        TsType::TsFnOrConstructorType(_) => "Function".to_string(),
        _ => "unknown".to_string(),
    }
}

fn create_item_doc(name: String, kind: &str, signature: String, parsed: ParsedJSDoc) -> ItemDoc {
    ItemDoc {
        name,
        kind: kind.to_string(),
        signature,
        description: parsed.description,
        params: if parsed.params.is_empty() {
            None
        } else {
            Some(parsed.params)
        },
        returns: parsed.returns,
        examples: if parsed.examples.is_empty() {
            None
        } else {
            Some(parsed.examples)
        },
        remarks: parsed.remarks,
        see: if parsed.see.is_empty() {
            None
        } else {
            Some(parsed.see)
        },
        deprecated: parsed.deprecated,
    }
}

// ============================================================================
// Main Processing
// ============================================================================

fn parse_typescript_file(path: &Path) -> Option<(Module, SingleThreadedComments)> {
    let source = fs::read_to_string(path).ok()?;
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(FileName::Real(path.to_path_buf()).into(), source);

    let comments = SingleThreadedComments::default();

    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax {
            tsx: path.extension().map(|e| e == "tsx").unwrap_or(false),
            decorators: true,
            ..Default::default()
        }),
        Default::default(),
        StringInput::from(&*fm),
        Some(&comments),
    );

    let mut parser = SwcParser::new_from(lexer);
    let module = parser.parse_module().ok()?;

    Some((module, comments))
}

fn process_package(config: &PackageConfig, packages_root: &Path) -> Option<PackageDoc> {
    let pkg_path = packages_root.join(config.name);
    let pkg_json_path = pkg_path.join("package.json");
    let entry_path = pkg_path.join(config.entry_file);

    if !entry_path.exists() {
        eprintln!("Warning: Entry file not found: {:?}", entry_path);
        return None;
    }

    let (_, version, description) = parse_package_json(&pkg_json_path);
    let source = fs::read_to_string(&entry_path).ok()?;

    let (module, comments) = parse_typescript_file(&entry_path)?;

    let overview = extract_file_overview(&source, &comments);
    let items = extract_item_docs(&source, &comments, &module);

    Some(PackageDoc {
        name: config.display_name.to_string(),
        kind: "typescript_package".to_string(),
        version: if version.is_empty() {
            "unknown".to_string()
        } else {
            version
        },
        description,
        overview,
        items,
    })
}

fn main() {
    let args = Args::parse();

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

    let packages_root = repo_root.join(&args.packages_dir);
    let output_dir = repo_root.join(&args.output_dir);

    if args.verbose {
        eprintln!("Packages dir: {:?}", packages_root);
        eprintln!("Output dir: {:?}", output_dir);
    }

    // Create output directory
    fs::create_dir_all(&output_dir).expect("Failed to create output directory");

    println!("Extracting TypeScript documentation...\n");

    let mut all_docs = Vec::new();

    for config in PACKAGES {
        println!("Processing {}...", config.name);
        if let Some(docs) = process_package(config, &packages_root) {
            let out_path = output_dir.join(format!("{}.json", config.name));
            let json = serde_json::to_string_pretty(&docs).unwrap();
            fs::write(&out_path, json).expect("Failed to write JSON");
            println!("  -> {:?}", out_path);
            println!("     {} documented items", docs.items.len());
            if !docs.overview.is_empty() {
                println!("     Has file overview ({} chars)", docs.overview.len());
            }
            all_docs.push(docs);
        }
    }

    // Write index file
    let index = IndexDoc {
        generated: chrono_lite_now(),
        packages: all_docs
            .iter()
            .map(|d| PackageSummary {
                name: d.name.clone(),
                version: d.version.clone(),
                item_count: d.items.len(),
                has_overview: !d.overview.is_empty(),
            })
            .collect(),
    };

    let index_path = output_dir.join("index.json");
    let index_json = serde_json::to_string_pretty(&index).unwrap();
    fs::write(&index_path, index_json).expect("Failed to write index JSON");

    println!("\nWrote index: {:?}", index_path);
    println!(
        "\nTotal: {} documented items",
        all_docs.iter().map(|d| d.items.len()).sum::<usize>()
    );
}

fn chrono_lite_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{}Z", duration.as_secs())
}
