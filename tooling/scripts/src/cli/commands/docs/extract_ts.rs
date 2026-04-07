//! Extract TypeScript documentation to JSON
//!
//! Parses TypeScript source files using SWC for proper AST-based extraction of
//! JSDoc comments, exported declarations, and type signatures.

use crate::core::config::Config;
use crate::utils::format;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use swc_core::common::comments::{CommentKind, Comments, SingleThreadedComments};
use swc_core::common::input::StringInput;
use swc_core::common::source_map::SmallPos;
use swc_core::common::{FileName, SourceMap, Span, Spanned};
use swc_core::ecma::ast::*;
use swc_core::ecma::parser::{Parser, Syntax, TsSyntax, lexer::Lexer};

/// TypeScript package documentation
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageDoc {
    pub name: String,
    pub version: String,
    pub description: String,
    pub exports: Vec<ExportDoc>,
}

/// Export documentation
#[derive(Debug, Serialize, Deserialize)]
pub struct ExportDoc {
    pub name: String,
    pub kind: String,
    #[serde(rename = "type")]
    pub export_type: Option<String>,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<ParamDoc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returns: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub see: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParamDoc {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
    pub description: String,
}

/// Parsed JSDoc block
#[derive(Debug, Default)]
struct JsDoc {
    description: String,
    params: Vec<ParamDoc>,
    returns: Option<String>,
    examples: Vec<String>,
    remarks: Option<String>,
    see: Vec<String>,
    deprecated: Option<String>,
    is_internal: bool,
    is_module: bool,
    module_description: String,
}

/// TypeScript packages to document
const TS_PACKAGES: &[(&str, &str)] = &[
    ("core", "crates/macroforge_ts"),
    ("shared", "packages/shared"),
    ("vite-plugin", "packages/vite-plugin"),
    ("typescript-plugin", "packages/typescript-plugin"),
    ("svelte-language-server", "packages/svelte-language-server"),
    ("svelte-preprocessor", "packages/svelte-preprocessor"),
    ("mcp-server", "packages/mcp-server"),
];

pub fn run(output_dir: &Path) -> Result<()> {
    let config = Config::load()?;
    let output_path = config.root.join(output_dir);

    fs::create_dir_all(&output_path)?;

    format::header("Extracting TypeScript Documentation");
    println!("Output: {}", output_path.display());

    let mut all_docs = Vec::new();
    let mut total_exports = 0;

    for (pkg_name, pkg_path) in TS_PACKAGES {
        let pkg_dir = config.root.join(pkg_path);
        if !pkg_dir.exists() {
            format::warning(&format!("Package not found: {}", pkg_dir.display()));
            continue;
        }

        print!("Processing {}... ", pkg_name);

        let docs = match extract_package_docs(&pkg_dir, pkg_name) {
            Ok(d) => d,
            Err(e) => {
                println!("failed: {}", e);
                continue;
            }
        };

        let export_count = docs.exports.len();
        total_exports += export_count;

        let out_path = output_path.join(format!("{}.json", pkg_name));
        let json = serde_json::to_string_pretty(&docs)?;
        fs::write(&out_path, json)?;

        println!("{} exports", export_count);
        all_docs.push(docs);
    }

    // Write index
    let index = serde_json::json!({
        "generated": chrono::Utc::now().to_rfc3339(),
        "packages": all_docs.iter().map(|d| serde_json::json!({
            "name": d.name,
            "version": d.version,
            "exportCount": d.exports.len(),
        })).collect::<Vec<_>>(),
    });

    let index_path = output_path.join("index.json");
    fs::write(&index_path, serde_json::to_string_pretty(&index)?)?;

    println!();
    format::success(&format!(
        "Extracted {} exports from {} packages",
        total_exports,
        all_docs.len()
    ));

    Ok(())
}

fn extract_package_docs(pkg_dir: &Path, pkg_name: &str) -> Result<PackageDoc> {
    let pkg_json_path = pkg_dir.join("package.json");

    // Parse package.json
    let pkg_json_content = fs::read_to_string(&pkg_json_path).unwrap_or_default();
    let pkg_json: serde_json::Value = serde_json::from_str(&pkg_json_content).unwrap_or_default();

    let name = pkg_json
        .get("name")
        .and_then(|n| n.as_str())
        .unwrap_or(pkg_name)
        .to_string();

    let version = pkg_json
        .get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("0.0.0")
        .to_string();

    let description = pkg_json
        .get("description")
        .and_then(|d| d.as_str())
        .unwrap_or("")
        .to_string();

    // Find entry point from package.json
    let main_file = pkg_json
        .get("main")
        .or_else(|| pkg_json.get("module"))
        .and_then(|m| m.as_str())
        .unwrap_or("src/index.ts");

    // Look for source files to parse
    let source_paths = [
        pkg_dir.join("src/index.ts"),
        pkg_dir.join("src/lib.ts"),
        pkg_dir.join(main_file.replace(".js", ".ts")),
        pkg_dir.join("src/index.d.ts"),
    ];

    let mut exports = Vec::new();

    for source_path in &source_paths {
        if source_path.exists() {
            if let Ok(source) = fs::read_to_string(source_path) {
                let mut file_exports = extract_exports_swc(&source, source_path, pkg_dir)?;
                exports.append(&mut file_exports);
            }
            break;
        }
    }

    // If the main entry is a barrel file with re-exports, also parse the referenced files
    if let Some(first_path) = source_paths.iter().find(|p| p.exists())
        && let Ok(source) = fs::read_to_string(first_path)
    {
        let re_export_files = find_reexport_sources(&source, first_path);
        for re_path in re_export_files {
            if re_path.exists()
                && let Ok(re_source) = fs::read_to_string(&re_path)
            {
                let mut re_exports =
                    extract_exports_swc(&re_source, &re_path, pkg_dir).unwrap_or_default();
                // Only add exports not already present (avoid duplicates from barrel)
                for exp in re_exports.drain(..) {
                    if !exports.iter().any(|e| e.name == exp.name) {
                        exports.push(exp);
                    }
                }
            }
        }
    }

    Ok(PackageDoc {
        name,
        version,
        description,
        exports,
    })
}

/// Find source files referenced by re-export statements (e.g., `export { ... } from './foo.js'`)
fn find_reexport_sources(source: &str, entry_path: &Path) -> Vec<PathBuf> {
    let parent = entry_path.parent().unwrap_or(Path::new("."));
    let mut paths = Vec::new();

    // Use a simple regex to find `from './...'` or `from "../..."` patterns
    let re = regex::Regex::new(r#"from\s+['"](\./[^'"]+)['"]"#).unwrap();
    for cap in re.captures_iter(source) {
        if let Some(m) = cap.get(1) {
            let rel = m.as_str();
            // Try .ts extension variants
            let base = rel
                .trim_end_matches(".js")
                .trim_end_matches(".ts")
                .trim_end_matches(".d.ts");
            for ext in &[".ts", ".d.ts", "/index.ts"] {
                let candidate = parent.join(format!("{}{}", base, ext));
                if candidate.exists() && candidate != entry_path {
                    paths.push(candidate);
                    break;
                }
            }
        }
    }

    paths
}

/// Parse a TypeScript source file with SWC and extract exported declarations
fn extract_exports_swc(source: &str, file_path: &Path, _pkg_dir: &Path) -> Result<Vec<ExportDoc>> {
    let cm = SourceMap::default();
    let comments = SingleThreadedComments::default();

    let fm = cm.new_source_file(
        FileName::Real(file_path.to_path_buf()).into(),
        source.to_string(),
    );

    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax {
            tsx: file_path.extension().is_some_and(|e| e == "tsx"),
            decorators: true,
            ..Default::default()
        }),
        EsVersion::latest(),
        StringInput::from(&*fm),
        Some(&comments),
    );

    let mut parser = Parser::new_from(lexer);
    let module = parser
        .parse_module()
        .map_err(|e| anyhow::anyhow!("SWC parse error in {}: {:?}", file_path.display(), e))?;

    let mut exports = Vec::new();

    for item in &module.body {
        match item {
            // export function foo(...) { ... }
            // export async function foo(...) { ... }
            ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export_decl)) => {
                if let Some(exp) =
                    extract_from_decl(&export_decl.decl, export_decl.span, source, &comments)
                {
                    exports.extend(exp);
                }
            }

            // export default function foo() { ... }
            // export default class Foo { ... }
            ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultDecl(default_decl)) => {
                if let Some(exp) = extract_from_default_decl(default_decl, source, &comments) {
                    exports.push(exp);
                }
            }

            // export default expression
            ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(default_expr)) => {
                let jsdoc = get_leading_jsdoc(default_expr.span, source, &comments);
                if !jsdoc.is_internal {
                    exports.push(ExportDoc {
                        name: "default".to_string(),
                        kind: "const".to_string(),
                        export_type: None,
                        description: jsdoc.description,
                        params: None,
                        returns: None,
                        examples: nonempty_vec(jsdoc.examples),
                        remarks: jsdoc.remarks,
                        see: nonempty_vec(jsdoc.see),
                        deprecated: jsdoc.deprecated,
                    });
                }
            }

            // export { Foo, Bar } from './module'  -- named re-exports
            // We skip these since we resolve re-exported files separately
            ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(_)) => {}

            // export * from './module'  -- star re-exports
            ModuleItem::ModuleDecl(ModuleDecl::ExportAll(_)) => {}

            _ => {}
        }
    }

    Ok(exports)
}

/// Extract exports from a declaration (function, class, interface, type alias, enum, var/const)
fn extract_from_decl(
    decl: &Decl,
    span: Span,
    source: &str,
    comments: &SingleThreadedComments,
) -> Option<Vec<ExportDoc>> {
    match decl {
        Decl::Fn(fn_decl) => {
            let jsdoc = get_leading_jsdoc(span, source, comments);
            if jsdoc.is_internal {
                return None;
            }

            let name = fn_decl.ident.sym.to_string();
            let (params, return_type) = extract_fn_signature(&fn_decl.function, source, &jsdoc);

            Some(vec![ExportDoc {
                name,
                kind: "function".to_string(),
                export_type: Some(format_fn_type(&fn_decl.function, source)),
                description: jsdoc.description,
                params: nonempty_vec(params),
                returns: return_type,
                examples: nonempty_vec(jsdoc.examples),
                remarks: jsdoc.remarks,
                see: nonempty_vec(jsdoc.see),
                deprecated: jsdoc.deprecated,
            }])
        }

        Decl::Class(class_decl) => {
            let jsdoc = get_leading_jsdoc(span, source, comments);
            if jsdoc.is_internal {
                return None;
            }

            let name = class_decl.ident.sym.to_string();

            Some(vec![ExportDoc {
                name,
                kind: "class".to_string(),
                export_type: None,
                description: jsdoc.description,
                params: None,
                returns: None,
                examples: nonempty_vec(jsdoc.examples),
                remarks: jsdoc.remarks,
                see: nonempty_vec(jsdoc.see),
                deprecated: jsdoc.deprecated,
            }])
        }

        Decl::TsInterface(iface) => {
            let jsdoc = get_leading_jsdoc(span, source, comments);
            if jsdoc.is_internal {
                return None;
            }

            let name = iface.id.sym.to_string();

            Some(vec![ExportDoc {
                name,
                kind: "interface".to_string(),
                export_type: None,
                description: jsdoc.description,
                params: None,
                returns: None,
                examples: nonempty_vec(jsdoc.examples),
                remarks: jsdoc.remarks,
                see: nonempty_vec(jsdoc.see),
                deprecated: jsdoc.deprecated,
            }])
        }

        Decl::TsTypeAlias(type_alias) => {
            let jsdoc = get_leading_jsdoc(span, source, comments);
            if jsdoc.is_internal {
                return None;
            }

            let name = type_alias.id.sym.to_string();
            let type_text = span_text(type_alias.type_ann.span(), source);

            Some(vec![ExportDoc {
                name,
                kind: "type".to_string(),
                export_type: Some(type_text),
                description: jsdoc.description,
                params: None,
                returns: None,
                examples: nonempty_vec(jsdoc.examples),
                remarks: jsdoc.remarks,
                see: nonempty_vec(jsdoc.see),
                deprecated: jsdoc.deprecated,
            }])
        }

        Decl::TsEnum(ts_enum) => {
            let jsdoc = get_leading_jsdoc(span, source, comments);
            if jsdoc.is_internal {
                return None;
            }

            let name = ts_enum.id.sym.to_string();

            Some(vec![ExportDoc {
                name,
                kind: "enum".to_string(),
                export_type: None,
                description: jsdoc.description,
                params: None,
                returns: None,
                examples: nonempty_vec(jsdoc.examples),
                remarks: jsdoc.remarks,
                see: nonempty_vec(jsdoc.see),
                deprecated: jsdoc.deprecated,
            }])
        }

        Decl::Var(var_decl) => {
            let mut results = Vec::new();
            for declarator in &var_decl.decls {
                let jsdoc = get_leading_jsdoc(span, source, comments);
                if jsdoc.is_internal {
                    continue;
                }

                let name = match &declarator.name {
                    Pat::Ident(ident) => ident.sym.to_string(),
                    _ => continue,
                };

                let kind = match var_decl.kind {
                    VarDeclKind::Const => "const",
                    VarDeclKind::Let => "let",
                    VarDeclKind::Var => "var",
                };

                let type_text = declarator
                    .name
                    .as_ident()
                    .and_then(|id| id.type_ann.as_ref())
                    .map(|ann| span_text(ann.type_ann.span(), source));

                results.push(ExportDoc {
                    name,
                    kind: kind.to_string(),
                    export_type: type_text,
                    description: jsdoc.description,
                    params: None,
                    returns: None,
                    examples: nonempty_vec(jsdoc.examples),
                    remarks: jsdoc.remarks,
                    see: nonempty_vec(jsdoc.see),
                    deprecated: jsdoc.deprecated,
                });
            }
            if results.is_empty() {
                None
            } else {
                Some(results)
            }
        }

        _ => None,
    }
}

/// Extract export from a default declaration
fn extract_from_default_decl(
    default_decl: &ExportDefaultDecl,
    source: &str,
    comments: &SingleThreadedComments,
) -> Option<ExportDoc> {
    let jsdoc = get_leading_jsdoc(default_decl.span, source, comments);
    if jsdoc.is_internal {
        return None;
    }

    match &default_decl.decl {
        DefaultDecl::Fn(fn_expr) => {
            let name = fn_expr
                .ident
                .as_ref()
                .map(|id| id.sym.to_string())
                .unwrap_or_else(|| "default".to_string());

            let (params, return_type) = extract_fn_signature(&fn_expr.function, source, &jsdoc);

            Some(ExportDoc {
                name,
                kind: "function".to_string(),
                export_type: Some(format_fn_type(&fn_expr.function, source)),
                description: jsdoc.description,
                params: nonempty_vec(params),
                returns: return_type,
                examples: nonempty_vec(jsdoc.examples),
                remarks: jsdoc.remarks,
                see: nonempty_vec(jsdoc.see),
                deprecated: jsdoc.deprecated,
            })
        }

        DefaultDecl::Class(class_expr) => {
            let name = class_expr
                .ident
                .as_ref()
                .map(|id| id.sym.to_string())
                .unwrap_or_else(|| "default".to_string());

            Some(ExportDoc {
                name,
                kind: "class".to_string(),
                export_type: None,
                description: jsdoc.description,
                params: None,
                returns: None,
                examples: nonempty_vec(jsdoc.examples),
                remarks: jsdoc.remarks,
                see: nonempty_vec(jsdoc.see),
                deprecated: jsdoc.deprecated,
            })
        }

        DefaultDecl::TsInterfaceDecl(iface) => {
            let name = iface.id.sym.to_string();

            Some(ExportDoc {
                name,
                kind: "interface".to_string(),
                export_type: None,
                description: jsdoc.description,
                params: None,
                returns: None,
                examples: nonempty_vec(jsdoc.examples),
                remarks: jsdoc.remarks,
                see: nonempty_vec(jsdoc.see),
                deprecated: jsdoc.deprecated,
            })
        }
    }
}

// ---------------------------------------------------------------------------
// Function signature extraction
// ---------------------------------------------------------------------------

/// Extract parameter docs and return type from a function declaration
fn extract_fn_signature(
    func: &Function,
    source: &str,
    jsdoc: &JsDoc,
) -> (Vec<ParamDoc>, Option<String>) {
    let mut params = Vec::new();

    for param in &func.params {
        let (name, type_text) = match &param.pat {
            Pat::Ident(ident) => {
                let name = ident.sym.to_string();
                let ty = ident
                    .type_ann
                    .as_ref()
                    .map(|ann| span_text(ann.type_ann.span(), source))
                    .unwrap_or_default();
                (name, ty)
            }
            Pat::Rest(rest) => {
                let name = match &*rest.arg {
                    Pat::Ident(ident) => format!("...{}", ident.sym),
                    _ => "...args".to_string(),
                };
                let ty = rest
                    .type_ann
                    .as_ref()
                    .map(|ann| span_text(ann.type_ann.span(), source))
                    .unwrap_or_default();
                (name, ty)
            }
            Pat::Assign(assign) => {
                let name = match &*assign.left {
                    Pat::Ident(ident) => ident.sym.to_string(),
                    _ => "_".to_string(),
                };
                (name, String::new())
            }
            Pat::Object(obj) => {
                let ty = obj
                    .type_ann
                    .as_ref()
                    .map(|ann| span_text(ann.type_ann.span(), source))
                    .unwrap_or_default();
                ("options".to_string(), ty)
            }
            Pat::Array(arr) => {
                let ty = arr
                    .type_ann
                    .as_ref()
                    .map(|ann| span_text(ann.type_ann.span(), source))
                    .unwrap_or_default();
                ("items".to_string(), ty)
            }
            _ => continue,
        };

        // Look up description from JSDoc @param tags
        let description = jsdoc
            .params
            .iter()
            .find(|p| p.name == name || name.ends_with(&p.name))
            .map(|p| p.description.clone())
            .unwrap_or_default();

        params.push(ParamDoc {
            name,
            param_type: type_text,
            description,
        });
    }

    // Return type: prefer JSDoc @returns, fall back to AST type annotation
    let return_type = jsdoc.returns.clone().or_else(|| {
        func.return_type
            .as_ref()
            .map(|ann| span_text(ann.type_ann.span(), source))
    });

    (params, return_type)
}

/// Format a function's type signature as a string like `(param: Type) => ReturnType`
fn format_fn_type(func: &Function, source: &str) -> String {
    let params: Vec<String> = func
        .params
        .iter()
        .map(|p| match &p.pat {
            Pat::Ident(ident) => {
                let name = ident.sym.to_string();
                let opt = if ident.optional { "?" } else { "" };
                match &ident.type_ann {
                    Some(ann) => {
                        let ty = span_text(ann.type_ann.span(), source);
                        format!("{}{}: {}", name, opt, ty)
                    }
                    None => format!("{}{}", name, opt),
                }
            }
            Pat::Rest(rest) => {
                let name = match &*rest.arg {
                    Pat::Ident(ident) => format!("...{}", ident.sym),
                    _ => "...args".to_string(),
                };
                match &rest.type_ann {
                    Some(ann) => {
                        let ty = span_text(ann.type_ann.span(), source);
                        format!("{}: {}", name, ty)
                    }
                    None => name,
                }
            }
            Pat::Assign(assign) => match &*assign.left {
                Pat::Ident(ident) => {
                    let name = ident.sym.to_string();
                    match &ident.type_ann {
                        Some(ann) => {
                            let ty = span_text(ann.type_ann.span(), source);
                            format!("{}?: {}", name, ty)
                        }
                        None => format!("{}?", name),
                    }
                }
                _ => "_".to_string(),
            },
            _ => "_".to_string(),
        })
        .collect();

    let ret = func
        .return_type
        .as_ref()
        .map(|ann| span_text(ann.type_ann.span(), source))
        .unwrap_or_else(|| "void".to_string());

    if func.is_async {
        format!("async ({}) => {}", params.join(", "), ret)
    } else {
        format!("({}) => {}", params.join(", "), ret)
    }
}

// ---------------------------------------------------------------------------
// JSDoc parsing
// ---------------------------------------------------------------------------

/// Get the leading JSDoc comment for a span and parse it
fn get_leading_jsdoc(span: Span, _source: &str, comments: &SingleThreadedComments) -> JsDoc {
    let leading = comments.get_leading(span.lo);
    if let Some(leading) = leading {
        // Find the last block comment (JSDoc) before this node
        for comment in leading.iter().rev() {
            if comment.kind == CommentKind::Block && comment.text.starts_with('*') {
                return parse_jsdoc(&comment.text);
            }
        }
    }
    JsDoc::default()
}

/// Parse a JSDoc comment body (the text between `/**` and `*/`)
fn parse_jsdoc(text: &str) -> JsDoc {
    let mut doc = JsDoc::default();
    let mut description_lines = Vec::new();
    let mut current_tag: Option<String> = None;
    let mut current_tag_content = Vec::new();
    let mut in_example = false;
    let mut example_lines = Vec::new();

    for line in text.lines() {
        // Strip leading `*` and whitespace from JSDoc comment lines
        let trimmed = line.trim().trim_start_matches('*').trim();

        // Handle example code blocks
        if in_example {
            if trimmed.starts_with("```")
                && example_lines.iter().any(|l: &String| l.contains("```"))
            {
                example_lines.push(trimmed.to_string());
                doc.examples.push(example_lines.join("\n"));
                example_lines.clear();
                in_example = false;
                current_tag = None;
            } else if trimmed.starts_with('@')
                && !trimmed.starts_with("@derive")
                && !in_code_fence(&example_lines)
            {
                // New tag encountered, close example without closing fence
                doc.examples.push(example_lines.join("\n"));
                example_lines.clear();
                in_example = false;
                // Fall through to tag processing
            } else {
                example_lines.push(trimmed.to_string());
                continue;
            }
        }

        if trimmed.starts_with('@') {
            // Flush previous tag
            flush_tag(&mut doc, &current_tag, &current_tag_content);
            current_tag_content.clear();

            if trimmed.starts_with("@param") {
                current_tag = Some("param".to_string());
                let rest = trimmed.strip_prefix("@param").unwrap().trim();
                current_tag_content.push(rest.to_string());
            } else if trimmed.starts_with("@returns") || trimmed.starts_with("@return") {
                current_tag = Some("returns".to_string());
                let rest = if trimmed.starts_with("@returns") {
                    trimmed.strip_prefix("@returns").unwrap().trim()
                } else {
                    trimmed.strip_prefix("@return").unwrap().trim()
                };
                current_tag_content.push(rest.to_string());
            } else if trimmed.starts_with("@example") {
                current_tag = Some("example".to_string());
                in_example = true;
                let rest = trimmed.strip_prefix("@example").unwrap().trim();
                if !rest.is_empty() {
                    example_lines.push(rest.to_string());
                }
            } else if trimmed.starts_with("@remarks") {
                current_tag = Some("remarks".to_string());
                let rest = trimmed.strip_prefix("@remarks").unwrap().trim();
                current_tag_content.push(rest.to_string());
            } else if trimmed.starts_with("@see") {
                current_tag = Some("see".to_string());
                let rest = trimmed.strip_prefix("@see").unwrap().trim();
                current_tag_content.push(rest.to_string());
            } else if trimmed.starts_with("@deprecated") {
                current_tag = Some("deprecated".to_string());
                let rest = trimmed.strip_prefix("@deprecated").unwrap().trim();
                current_tag_content.push(rest.to_string());
            } else if trimmed.starts_with("@internal") {
                doc.is_internal = true;
                current_tag = None;
            } else if trimmed.starts_with("@module") || trimmed.starts_with("@fileoverview") {
                doc.is_module = true;
                current_tag = Some("module".to_string());
                let rest = if trimmed.starts_with("@module") {
                    trimmed.strip_prefix("@module").unwrap().trim()
                } else {
                    trimmed.strip_prefix("@fileoverview").unwrap().trim()
                };
                if !rest.is_empty() {
                    current_tag_content.push(rest.to_string());
                }
            } else if trimmed.starts_with("@packageDocumentation")
                || trimmed.starts_with("@template")
                || trimmed.starts_with("@default")
                || trimmed.starts_with("@throws")
            {
                // Known tags we skip but don't treat as description
                current_tag = Some("skip".to_string());
            } else {
                // Unknown tag, skip
                current_tag = Some("skip".to_string());
            }
        } else if current_tag.is_some() {
            // Continuation of a tag
            if !trimmed.is_empty() {
                current_tag_content.push(trimmed.to_string());
            }
        } else {
            // Description text (before any tag)
            if !trimmed.is_empty() {
                description_lines.push(trimmed.to_string());
            }
        }
    }

    // Flush final example if still open
    if in_example && !example_lines.is_empty() {
        doc.examples.push(example_lines.join("\n"));
    }

    // Flush final tag
    flush_tag(&mut doc, &current_tag, &current_tag_content);

    doc.description = description_lines.join(" ");

    // For module-level comments, store the module description too
    if doc.is_module {
        doc.module_description = doc.description.clone();
    }

    doc
}

/// Check if we're inside a code fence in collected example lines
fn in_code_fence(lines: &[String]) -> bool {
    let fence_count = lines.iter().filter(|l| l.trim().starts_with("```")).count();
    fence_count % 2 != 0
}

/// Flush a completed JSDoc tag into the JsDoc struct
fn flush_tag(doc: &mut JsDoc, tag: &Option<String>, content: &[String]) {
    let Some(tag) = tag else { return };

    match tag.as_str() {
        "param" => {
            if let Some(first) = content.first() {
                let (name, param_type, desc) = parse_param_tag(first);
                let extra: String = content[1..]
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(" ");
                let full_desc = if extra.is_empty() {
                    desc
                } else {
                    format!("{} {}", desc, extra)
                };
                doc.params.push(ParamDoc {
                    name,
                    param_type,
                    description: full_desc,
                });
            }
        }
        "returns" => {
            let text = content.join(" ").trim().to_string();
            if !text.is_empty() {
                doc.returns = Some(text);
            }
        }
        "remarks" => {
            let text = content.join(" ").trim().to_string();
            if !text.is_empty() {
                doc.remarks = Some(text);
            }
        }
        "see" => {
            let text = content.join(" ").trim().to_string();
            if !text.is_empty() {
                doc.see.push(text);
            }
        }
        "deprecated" => {
            let text = content.join(" ").trim().to_string();
            doc.deprecated = Some(if text.is_empty() {
                "Deprecated".to_string()
            } else {
                text
            });
        }
        "module" => {
            let text = content.join(" ").trim().to_string();
            if !text.is_empty() {
                doc.module_description = text;
            }
        }
        _ => {} // skip unknown tags
    }
}

/// Parse a `@param` tag line into (name, type, description)
///
/// Supports formats:
///   `@param name - description`
///   `@param {Type} name - description`
///   `@param name description`
fn parse_param_tag(text: &str) -> (String, String, String) {
    let text = text.trim();

    // Check for `{Type} name - desc` format
    if text.starts_with('{')
        && let Some(close) = text.find('}')
    {
        let param_type = text[1..close].trim().to_string();
        let rest = text[close + 1..].trim();
        let (name, desc) = split_name_desc(rest);
        return (name, param_type, desc);
    }

    // Simple `name - desc` or `name desc` format
    let (name, desc) = split_name_desc(text);
    (name, String::new(), desc)
}

/// Split "name - description" or "name description" into parts
fn split_name_desc(text: &str) -> (String, String) {
    let text = text.trim();
    if let Some(dash_pos) = text.find(" - ") {
        let name = text[..dash_pos].trim().to_string();
        let desc = text[dash_pos + 3..].trim().to_string();
        (name, desc)
    } else {
        // Split on first whitespace
        match text.split_once(char::is_whitespace) {
            Some((name, desc)) => (name.trim().to_string(), desc.trim().to_string()),
            None => (text.to_string(), String::new()),
        }
    }
}

// ---------------------------------------------------------------------------
// Utilities
// ---------------------------------------------------------------------------

/// Extract the text of a span from source
fn span_text(span: Span, source: &str) -> String {
    let lo = span.lo.to_usize();
    let hi = span.hi.to_usize();
    if lo < source.len() && hi <= source.len() && lo < hi {
        source[lo..hi].trim().to_string()
    } else {
        String::new()
    }
}

/// Convert a vec to Option<Vec>, returning None if empty
fn nonempty_vec<T>(v: Vec<T>) -> Option<Vec<T>> {
    if v.is_empty() { None } else { Some(v) }
}
