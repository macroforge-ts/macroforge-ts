//! Extract TypeScript documentation to JSON
//!
//! Parses TypeScript source files using OXC for proper AST-based extraction of
//! JSDoc comments, exported declarations, and type signatures.

use crate::cli::commands::docs::generated::GeneratedFiles;
use crate::utils::format;
use anyhow::{Context, Result};
use oxc::allocator::Allocator;
use oxc::ast::Comment;
use oxc::ast::ast::{
    BindingPattern, Declaration, ExportDefaultDeclarationKind, FormalParameter, Function, Program,
    Statement, TSTypeAnnotation, VariableDeclarationKind,
};
use oxc::parser::Parser;
use oxc::span::{GetSpan, SourceType, Span};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamDoc {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
    pub description: String,
}

/// Parsed JSDoc block
#[derive(Debug, Clone, Default)]
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

/// TypeScript packages to document. `@macroforge/core` is absent: its API
/// is the wasm bindings, documented from the Rust source.
const TS_PACKAGES: &[(&str, &str)] = &[
    ("shared", "packages/shared"),
    ("vite-plugin", "packages/vite-plugin"),
    ("typescript-plugin", "packages/typescript-plugin"),
    ("svelte-language-server", "packages/svelte-language-server"),
    ("svelte-preprocessor", "packages/svelte-preprocessor"),
    ("mcp-server", "packages/mcp-server"),
    ("deno-plugin", "packages/deno-plugin"),
];

/// Where the TypeScript API JSON lives, relative to the repository root.
const OUTPUT_DIR: &str = "website/static/api-data/typescript";

/// Extracts TypeScript JSDoc and type information into the API JSON.
pub fn generate(root: &Path) -> Result<GeneratedFiles> {
    let output_path = Path::new(OUTPUT_DIR);

    format::header("Extracting TypeScript Documentation");

    let mut files: Vec<(PathBuf, String)> = Vec::new();
    let mut all_docs = Vec::new();
    let mut total_exports = 0;

    for (pkg_name, pkg_path) in TS_PACKAGES {
        let pkg_dir = root.join(pkg_path);
        if !pkg_dir.exists() {
            anyhow::bail!("Package not found: {}", pkg_dir.display());
        }

        print!("Processing {}... ", pkg_name);

        let docs = extract_package_docs(&pkg_dir, pkg_name)
            .with_context(|| format!("failed to extract docs for {pkg_name}"))?;

        let export_count = docs.exports.len();
        total_exports += export_count;

        files.push((
            output_path.join(format!("{}.json", pkg_name)),
            crate::utils::json::to_string_pretty(&docs)?,
        ));

        println!("{} exports", export_count);
        all_docs.push(docs);
    }

    let index = serde_json::json!({
        "packages": all_docs.iter().map(|doc| serde_json::json!({
            "name": doc.name,
            "version": doc.version,
            "exportCount": doc.exports.len(),
        })).collect::<Vec<_>>(),
    });
    files.push((
        output_path.join("index.json"),
        crate::utils::json::to_string_pretty(&index)?,
    ));

    format::success(&format!(
        "Extracted {} exports from {} packages",
        total_exports,
        all_docs.len()
    ));

    GeneratedFiles::build(root, vec![output_path.to_path_buf()], files)
}

/// The package.json fields the API docs record.
#[derive(Deserialize)]
struct PackageManifest {
    name: Option<String>,
    version: String,
    #[serde(default)]
    description: String,
    main: Option<String>,
    module: Option<String>,
}

fn extract_package_docs(pkg_dir: &Path, pkg_name: &str) -> Result<PackageDoc> {
    let pkg_json_path = pkg_dir.join("package.json");
    let pkg_json_content = fs::read_to_string(&pkg_json_path)
        .with_context(|| format!("failed to read {}", pkg_json_path.display()))?;
    let manifest: PackageManifest = serde_json::from_str(&pkg_json_content)
        .with_context(|| format!("{} is not a valid package.json", pkg_json_path.display()))?;

    let main_file = manifest
        .main
        .as_deref()
        .or(manifest.module.as_deref())
        .unwrap_or("src/index.ts");

    let source_paths = [
        pkg_dir.join("src/index.ts"),
        pkg_dir.join("src/lib.ts"),
        pkg_dir.join(main_file.replace(".js", ".ts")),
        pkg_dir.join("src/index.d.ts"),
    ];
    let entry_path = source_paths
        .iter()
        .find(|path| path.exists())
        .with_context(|| format!("{} has no TypeScript entry point", pkg_dir.display()))?;
    let source = read_source(entry_path)?;
    let allocator = Allocator::default();
    let program = parse(&allocator, &source, entry_path)?;
    let mut exports = extract_exports(&program, &source);

    // A barrel entry re-exports other files, which hold the declarations.
    for re_path in relative_module_files(&program, entry_path) {
        let re_source = read_source(&re_path)?;
        let re_allocator = Allocator::default();
        let re_program = parse(&re_allocator, &re_source, &re_path)?;
        for export in extract_exports(&re_program, &re_source) {
            if !exports.iter().any(|existing| existing.name == export.name) {
                exports.push(export);
            }
        }
    }

    Ok(PackageDoc {
        name: manifest.name.unwrap_or_else(|| pkg_name.to_string()),
        version: manifest.version,
        description: manifest.description,
        exports,
    })
}

fn read_source(path: &Path) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))
}

fn parse<'a>(allocator: &'a Allocator, source: &'a str, path: &Path) -> Result<Program<'a>> {
    let source_type = SourceType::from_path(path)
        .map_err(|error| anyhow::anyhow!("{}: {error}", path.display()))?;
    let parsed = Parser::new(allocator, source, source_type).parse();
    if !parsed.diagnostics.is_empty() {
        anyhow::bail!(
            "parse errors in {}: {}",
            path.display(),
            parsed
                .diagnostics
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("; ")
        );
    }
    Ok(parsed.program)
}

/// Files the entry point imports or re-exports through a `./` specifier.
fn relative_module_files(program: &Program<'_>, entry_path: &Path) -> Vec<PathBuf> {
    let parent = entry_path.parent().unwrap_or(Path::new("."));
    program
        .body
        .iter()
        .filter_map(|statement| match statement {
            Statement::ImportDeclaration(import) => Some(import.source.value.as_str()),
            Statement::ExportFromDeclaration(export) => Some(export.source.value.as_str()),
            Statement::ExportAllDeclaration(export) => Some(export.source.value.as_str()),
            _ => None,
        })
        .filter(|specifier| specifier.starts_with("./"))
        .filter_map(|specifier| {
            let base = specifier
                .trim_end_matches(".js")
                .trim_end_matches(".d.ts")
                .trim_end_matches(".ts");
            [".ts", ".d.ts", "/index.ts"]
                .iter()
                .map(|extension| parent.join(format!("{base}{extension}")))
                .find(|candidate| candidate.exists() && candidate != entry_path)
        })
        .collect()
}

/// The documented exports declared in `program`.
fn extract_exports(program: &Program<'_>, source: &str) -> Vec<ExportDoc> {
    let mut exports = Vec::new();

    for statement in &program.body {
        match statement {
            // export function foo() {}, export const x = 1, export interface Foo {}, ...
            Statement::ExportDeclaration(export) => {
                let jsdoc = leading_jsdoc(export.span, &program.comments, source);
                if !jsdoc.is_internal {
                    exports.extend(declaration_docs(&export.declaration, source, jsdoc));
                }
            }

            // export default function foo() {}, export default class Foo {}, export default expr
            Statement::ExportDefaultDeclaration(export) => {
                let jsdoc = leading_jsdoc(export.span, &program.comments, source);
                if !jsdoc.is_internal {
                    exports.push(default_export_doc(&export.declaration, source, jsdoc));
                }
            }

            // Re-exports are documented from the files they point at.
            _ => {}
        }
    }

    exports
}

fn declaration_docs(declaration: &Declaration<'_>, source: &str, jsdoc: JsDoc) -> Vec<ExportDoc> {
    match declaration {
        Declaration::FunctionDeclaration(function) => {
            let name = function
                .id
                .as_ref()
                .map_or_else(|| "default".to_string(), |id| id.name.to_string());
            vec![function_doc(name, function, source, jsdoc)]
        }
        Declaration::ClassDeclaration(class) => {
            let name = class
                .id
                .as_ref()
                .map_or_else(|| "default".to_string(), |id| id.name.to_string());
            vec![export_doc(name, "class", None, jsdoc)]
        }
        Declaration::TSInterfaceDeclaration(interface) => {
            vec![export_doc(
                interface.id.name.to_string(),
                "interface",
                None,
                jsdoc,
            )]
        }
        Declaration::TSTypeAliasDeclaration(alias) => {
            let type_text = span_text(alias.type_annotation.span(), source);
            vec![export_doc(
                alias.id.name.to_string(),
                "type",
                Some(type_text),
                jsdoc,
            )]
        }
        Declaration::TSEnumDeclaration(ts_enum) => {
            vec![export_doc(ts_enum.id.name.to_string(), "enum", None, jsdoc)]
        }
        Declaration::VariableDeclaration(variable) => {
            let kind = match variable.kind {
                VariableDeclarationKind::Const => "const",
                VariableDeclarationKind::Let => "let",
                VariableDeclarationKind::Var => "var",
                VariableDeclarationKind::Using => "using",
                VariableDeclarationKind::AwaitUsing => "await using",
            };
            // Every declarator in the statement shares its JSDoc.
            variable
                .declarations
                .iter()
                .filter_map(|declarator| match &declarator.id {
                    BindingPattern::BindingIdentifier(ident) => Some(export_doc(
                        ident.name.to_string(),
                        kind,
                        annotation_text(declarator.type_annotation.as_deref(), source),
                        jsdoc.clone(),
                    )),
                    _ => None,
                })
                .collect()
        }
        Declaration::TSExternalModuleDeclaration(_)
        | Declaration::TSNamespaceDeclaration(_)
        | Declaration::TSGlobalDeclaration(_)
        | Declaration::TSImportEqualsDeclaration(_) => Vec::new(),
    }
}

fn default_export_doc(
    declaration: &ExportDefaultDeclarationKind<'_>,
    source: &str,
    jsdoc: JsDoc,
) -> ExportDoc {
    match declaration {
        ExportDefaultDeclarationKind::FunctionDeclaration(function) => {
            let name = function
                .id
                .as_ref()
                .map_or_else(|| "default".to_string(), |id| id.name.to_string());
            function_doc(name, function, source, jsdoc)
        }
        ExportDefaultDeclarationKind::ClassDeclaration(class) => {
            let name = class
                .id
                .as_ref()
                .map_or_else(|| "default".to_string(), |id| id.name.to_string());
            export_doc(name, "class", None, jsdoc)
        }
        ExportDefaultDeclarationKind::TSInterfaceDeclaration(interface) => {
            export_doc(interface.id.name.to_string(), "interface", None, jsdoc)
        }
        // Every remaining variant is an expression: `export default <expr>`.
        _ => export_doc("default".to_string(), "const", None, jsdoc),
    }
}

fn export_doc(name: String, kind: &str, export_type: Option<String>, jsdoc: JsDoc) -> ExportDoc {
    ExportDoc {
        name,
        kind: kind.to_string(),
        export_type,
        description: jsdoc.description,
        params: None,
        returns: None,
        examples: nonempty_vec(jsdoc.examples),
        remarks: jsdoc.remarks,
        see: nonempty_vec(jsdoc.see),
        deprecated: jsdoc.deprecated,
    }
}

fn function_doc(name: String, function: &Function<'_>, source: &str, jsdoc: JsDoc) -> ExportDoc {
    let params = function_params(function, source, &jsdoc);
    // Prefer the JSDoc @returns text, fall back to the annotated return type.
    let returns = jsdoc
        .returns
        .clone()
        .or_else(|| annotation_text(function.return_type.as_deref(), source));
    let export_type = format_fn_type(function, source);
    ExportDoc {
        params: nonempty_vec(params),
        returns,
        ..export_doc(name, "function", Some(export_type), jsdoc)
    }
}

// ---------------------------------------------------------------------------
// Function signature extraction
// ---------------------------------------------------------------------------

/// One parameter of a function signature.
struct Param {
    name: String,
    type_text: Option<String>,
    optional: bool,
}

fn signature_params(function: &Function<'_>, source: &str) -> Vec<Param> {
    let mut params: Vec<Param> = function
        .params
        .items
        .iter()
        .map(|param| formal_param(param, source))
        .collect();
    if let Some(rest) = &function.params.rest {
        let name = match &rest.rest.argument {
            BindingPattern::BindingIdentifier(ident) => format!("...{}", ident.name),
            _ => "...args".to_string(),
        };
        params.push(Param {
            name,
            type_text: annotation_text(rest.type_annotation.as_deref(), source),
            optional: false,
        });
    }
    params
}

fn formal_param(param: &FormalParameter<'_>, source: &str) -> Param {
    Param {
        name: pattern_name(&param.pattern),
        type_text: annotation_text(param.type_annotation.as_deref(), source),
        optional: param.optional || param.initializer.is_some(),
    }
}

/// The documented name of a parameter: its identifier, or a stand-in for a
/// destructuring pattern.
fn pattern_name(pattern: &BindingPattern<'_>) -> String {
    match pattern {
        BindingPattern::BindingIdentifier(ident) => ident.name.to_string(),
        BindingPattern::ObjectPattern(_) => "options".to_string(),
        BindingPattern::ArrayPattern(_) => "items".to_string(),
        BindingPattern::AssignmentPattern(assignment) => pattern_name(&assignment.left),
    }
}

/// Parameter docs, each described by the matching JSDoc `@param`.
fn function_params(function: &Function<'_>, source: &str, jsdoc: &JsDoc) -> Vec<ParamDoc> {
    signature_params(function, source)
        .into_iter()
        .map(|param| {
            let description = jsdoc
                .params
                .iter()
                .find(|tag| tag.name == param.name || param.name.ends_with(&tag.name))
                .map(|tag| tag.description.clone())
                .unwrap_or_default();
            ParamDoc {
                name: param.name,
                param_type: param.type_text.unwrap_or_default(),
                description,
            }
        })
        .collect()
}

/// Format a function's type signature as a string like `(param: Type) => ReturnType`
fn format_fn_type(function: &Function<'_>, source: &str) -> String {
    let params: Vec<String> = signature_params(function, source)
        .into_iter()
        .map(|param| {
            let optional = if param.optional { "?" } else { "" };
            match param.type_text {
                Some(type_text) => format!("{}{optional}: {type_text}", param.name),
                None => format!("{}{optional}", param.name),
            }
        })
        .collect();

    let ret = annotation_text(function.return_type.as_deref(), source)
        .unwrap_or_else(|| "void".to_string());

    if function.r#async {
        format!("async ({}) => {}", params.join(", "), ret)
    } else {
        format!("({}) => {}", params.join(", "), ret)
    }
}

// ---------------------------------------------------------------------------
// JSDoc parsing
// ---------------------------------------------------------------------------

/// The JSDoc block directly before the node starting at `span.start`.
fn leading_jsdoc(span: Span, comments: &[Comment], source: &str) -> JsDoc {
    comments
        .iter()
        .rev()
        .filter(|comment| comment.is_block() && comment.attached_to == span.start)
        .map(|comment| comment.content_span())
        .map(|content| &source[content.start as usize..content.end as usize])
        .find(|text| text.starts_with('*'))
        .map(parse_jsdoc)
        .unwrap_or_default()
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
                && example_lines
                    .iter()
                    .any(|example_line: &String| example_line.contains("```"))
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

            let tag_rest = |prefixes: &[&str]| {
                prefixes
                    .iter()
                    .find_map(|prefix| trimmed.strip_prefix(prefix))
                    .map(str::trim)
            };

            if let Some(rest) = tag_rest(&["@param"]) {
                current_tag = Some("param".to_string());
                current_tag_content.push(rest.to_string());
            } else if let Some(rest) = tag_rest(&["@returns", "@return"]) {
                current_tag = Some("returns".to_string());
                current_tag_content.push(rest.to_string());
            } else if let Some(rest) = tag_rest(&["@example"]) {
                current_tag = Some("example".to_string());
                in_example = true;
                if !rest.is_empty() {
                    example_lines.push(rest.to_string());
                }
            } else if let Some(rest) = tag_rest(&["@remarks"]) {
                current_tag = Some("remarks".to_string());
                current_tag_content.push(rest.to_string());
            } else if let Some(rest) = tag_rest(&["@see"]) {
                current_tag = Some("see".to_string());
                current_tag_content.push(rest.to_string());
            } else if let Some(rest) = tag_rest(&["@deprecated"]) {
                current_tag = Some("deprecated".to_string());
                current_tag_content.push(rest.to_string());
            } else if tag_rest(&["@internal"]).is_some() {
                doc.is_internal = true;
                current_tag = None;
            } else if let Some(rest) = tag_rest(&["@module", "@fileoverview"]) {
                doc.is_module = true;
                current_tag = Some("module".to_string());
                if !rest.is_empty() {
                    current_tag_content.push(rest.to_string());
                }
            } else {
                // Other tags (@template, @default, @throws, ...) are not description.
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
    let fence_count = lines
        .iter()
        .filter(|line| line.trim().starts_with("```"))
        .count();
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
                    .map(String::as_str)
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

/// The trimmed source text `span` covers.
fn span_text(span: Span, source: &str) -> String {
    source[span.start as usize..span.end as usize]
        .trim()
        .to_string()
}

/// The type text of an annotation, without its leading `:`.
fn annotation_text(annotation: Option<&TSTypeAnnotation<'_>>, source: &str) -> Option<String> {
    annotation.map(|annotation| span_text(annotation.type_annotation.span(), source))
}

/// Convert a vec to Option<Vec>, returning None if empty
fn nonempty_vec<T>(items: Vec<T>) -> Option<Vec<T>> {
    if items.is_empty() { None } else { Some(items) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exports_of(source: &str) -> Vec<ExportDoc> {
        let allocator = Allocator::default();
        let program =
            parse(&allocator, source, Path::new("fixture.ts")).expect("fixture should parse");
        extract_exports(&program, source)
    }

    fn export<'a>(exports: &'a [ExportDoc], name: &str) -> &'a ExportDoc {
        exports
            .iter()
            .find(|export| export.name == name)
            .unwrap_or_else(|| panic!("{name} should be exported"))
    }

    #[test]
    fn function_signature_keeps_exact_type_text() {
        let exports = exports_of(
            r#"
/**
 * Expands macros.
 * @param code - The source text
 * @param options - How to expand
 */
export async function expand(code: string, options?: ExpandOptions, ...rest: Array<string>): Promise<Result> {
    return run(code);
}
"#,
        );

        let expand = export(&exports, "expand");
        assert_eq!(expand.kind, "function");
        assert_eq!(expand.description, "Expands macros.");
        assert_eq!(
            expand.export_type.as_deref(),
            Some(
                "async (code: string, options?: ExpandOptions, ...rest: Array<string>) => Promise<Result>"
            )
        );
        assert_eq!(expand.returns.as_deref(), Some("Promise<Result>"));

        let params = expand.params.as_ref().expect("function has params");
        assert_eq!(params.len(), 3);
        assert_eq!(params[0].name, "code");
        assert_eq!(params[0].param_type, "string");
        assert_eq!(params[0].description, "The source text");
        assert_eq!(params[1].description, "How to expand");
        assert_eq!(params[2].name, "...rest");
        assert_eq!(params[2].param_type, "Array<string>");
    }

    #[test]
    fn defaulted_and_destructured_params() {
        let exports = exports_of(
            "export function build({ root }: Config, [first]: string[], retries: number = 3) {}
",
        );

        let build = export(&exports, "build");
        assert_eq!(
            build.export_type.as_deref(),
            Some("(options: Config, items: string[], retries?: number) => void")
        );
        let params = build.params.as_ref().expect("function has params");
        assert_eq!(params[0].name, "options");
        assert_eq!(params[1].name, "items");
        assert_eq!(params[2].param_type, "number");
    }

    #[test]
    fn declaration_kinds_and_jsdoc_tags() {
        let exports = exports_of(
            r#"
/** A plugin. */
export class Plugin {}
/**
 * Options.
 * @see https://macroforge.dev
 */
export interface Options {}
/**
 * A result.
 * @deprecated Use Outcome
 */
export type Result = { ok: boolean } | null;
export enum Level { Info }
/** Both share this doc. */
export const first: number = 1, second = 2;
/** @internal */
export const hidden = 3;
export default function () {}
"#,
        );

        assert_eq!(export(&exports, "Plugin").kind, "class");
        assert_eq!(export(&exports, "Plugin").description, "A plugin.");
        assert_eq!(export(&exports, "Options").kind, "interface");
        assert_eq!(
            export(&exports, "Options").see.as_deref(),
            Some(&["https://macroforge.dev".to_string()][..])
        );
        let result = export(&exports, "Result");
        assert_eq!(
            result.export_type.as_deref(),
            Some("{ ok: boolean } | null")
        );
        assert_eq!(result.deprecated.as_deref(), Some("Use Outcome"));
        assert_eq!(export(&exports, "Level").kind, "enum");
        let first = export(&exports, "first");
        assert_eq!(first.kind, "const");
        assert_eq!(first.export_type.as_deref(), Some("number"));
        assert_eq!(
            export(&exports, "second").description,
            "Both share this doc."
        );
        assert!(exports.iter().all(|export| export.name != "hidden"));
        assert_eq!(export(&exports, "default").kind, "function");
    }

    #[test]
    fn examples_keep_fenced_code() {
        let exports = exports_of(
            r#"
/**
 * Runs.
 * @example
 * ```ts
 * run();
 * ```
 */
export function run() {}
"#,
        );

        assert_eq!(
            export(&exports, "run").examples.as_deref(),
            Some(&["```ts\nrun();\n```".to_string()][..])
        );
    }

    #[test]
    fn a_non_jsdoc_block_comment_is_not_documentation() {
        let exports = exports_of(
            "/* plain */
export const value = 1;
",
        );
        assert_eq!(export(&exports, "value").description, "");
    }
}
