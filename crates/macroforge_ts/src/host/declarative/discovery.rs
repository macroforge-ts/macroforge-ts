//! Find `` const $name = macroRules`...` `` declarations in an OXC program.
//!
//! Discovery has two jobs:
//!
//! 1. Confirm the file actually imports `macroRules` from
//!    `"macroforge/rules"`. This is the signal that the file opted in to
//!    the declarative macro system, and it avoids false positives from
//!    user code that happens to use a local `macroRules` identifier.
//!
//! 2. Walk the top-level statements for `VariableDeclaration`s matching
//!    the sentinel shape `const $ident = macroRules\`...\`` and hand each
//!    template body off to the syn crate's parser.
//!
//! The returned [`DiscoveredMacro`] carries both the parsed [`MacroDef`]
//! and the full declaration span so the rewriter can delete the original
//! source later.

use std::path::{Path, PathBuf};

use oxc::ast::ast::{
    BindingPattern, Declaration, Expression, ImportDeclarationSpecifier, Program, Statement,
    TemplateLiteral, VariableDeclarationKind,
};
use oxc::span::GetSpan;

use crate::ts_syn::abi::{Diagnostic, DiagnosticLevel, SpanIR};
use crate::ts_syn::declarative::{
    DeclarativeError, MacroArm, MacroDef, MacroKind, MacroMode, parse_macro_def,
};
use crate::ts_syn::import_registry::collect_macro_import_comments_pub;

use super::project_registry::ProjectDeclarativeRegistry;

/// A single declarative macro discovered in a file, paired with the span
/// that the rewriter should delete to strip the declaration from the output.
#[derive(Debug, Clone)]
pub struct DiscoveredMacro {
    /// The parsed macro definition with `name` populated.
    pub def: MacroDef,
    /// Span covering the full `const $name = macroRules\`...\`;` declaration,
    /// in the **1-based SpanIR convention** used by the patch applicator.
    pub def_span: SpanIR,
}

/// Module specifier that must be imported for declarative macros to activate.
pub const RULES_MODULE: &str = "macroforge/rules";

/// The local identifier name that the `macroRules` tag function must be
/// imported under. Users may not alias it.
pub const MACRO_RULES_IDENT: &str = "macroRules";

/// Walk the OXC `Program` and return every declarative macro definition
/// found, or the first parse error encountered.
///
/// If the file does not import `macroRules` from `"macroforge/rules"`,
/// returns an empty vector immediately — this is the fast-path for files
/// that don't use declarative macros, which is the common case.
pub fn discover(
    program: &Program<'_>,
    source: &str,
) -> Result<Vec<DiscoveredMacro>, DeclarativeError> {
    if !has_macro_import(program) {
        return Ok(Vec::new());
    }

    let mut out = Vec::new();
    for stmt in &program.body {
        // Support both `const $x = macroRules\`...\`` at the top level and
        // `export const $x = macroRules\`...\``.
        let var_decl = match stmt {
            Statement::VariableDeclaration(decl) => Some(decl.as_ref()),
            Statement::ExportNamedDeclaration(export) => {
                export.declaration.as_ref().and_then(|decl| match decl {
                    Declaration::VariableDeclaration(var) => Some(var.as_ref()),
                    _ => None,
                })
            }
            _ => None,
        };
        let Some(var_decl) = var_decl else {
            continue;
        };
        if var_decl.kind != VariableDeclarationKind::Const {
            continue;
        }
        if var_decl.declarations.len() != 1 {
            continue;
        }
        let declarator = &var_decl.declarations[0];

        // Extract binding identifier.
        let BindingPattern::BindingIdentifier(binding) = &declarator.id else {
            continue;
        };
        let binding_name = binding.name.as_str();
        if !binding_name.starts_with('$') {
            continue;
        }

        let Some(init) = &declarator.init else {
            continue;
        };

        // Compute spans. Patch spans are 1-based; OXC is 0-based.
        let decl_span = oxc_span_to_ir(stmt.span());
        // Account for the trailing semicolon if present in the source.
        let def_span = extend_to_semicolon(decl_span, source);

        // Two supported shapes:
        //   1. Tag form:    `macroRules\`...\``
        //   2. Object form: `macroRules({ expand, runtime, call, mode })`
        let mut def = match init {
            Expression::TaggedTemplateExpression(tagged) => {
                let Expression::Identifier(tag_ident) = &tagged.tag else {
                    continue;
                };
                if tag_ident.name.as_str() != MACRO_RULES_IDENT {
                    continue;
                }
                parse_tag_form_macro(tagged, source)?
            }
            Expression::CallExpression(call) => {
                let Expression::Identifier(callee) = &call.callee else {
                    continue;
                };
                if callee.name.as_str() != MACRO_RULES_IDENT {
                    continue;
                }
                parse_object_form_macro(call, source)?
            }
            _ => continue,
        };

        // Populate the name from the surrounding binding (sans the `$`).
        def.name = binding_name[1..].to_string();

        out.push(DiscoveredMacro { def, def_span });
    }

    Ok(out)
}

/// Parse a tag-form declaration: `` macroRules`...` ``. Extracts the
/// template body's static text and hands it to the syn crate parser.
fn parse_tag_form_macro(
    tagged: &oxc::ast::ast::TaggedTemplateExpression<'_>,
    source: &str,
) -> Result<MacroDef, DeclarativeError> {
    let quasi_text = extract_static_quasi(&tagged.quasi, source)?;

    // The quasi span in OXC covers the backtick-delimited text including
    // the backticks. We want the text between them: trim one off each end.
    let quasi_span_raw = oxc_span_to_ir(tagged.quasi.span);
    let inner_start = quasi_span_raw.start.saturating_add(1);
    let inner_end = quasi_span_raw.end.saturating_sub(1);
    let template_span = SpanIR::new(inner_start.min(inner_end), inner_end);

    parse_macro_def(quasi_text, template_span)
}

/// Parse an object-form declaration:
/// `macroRules({ expand: macroRules\`...\`, runtime: "...", call: macroRules\`...\`, mode: "auto" })`.
///
/// Walks the `ObjectExpression` argument, extracts each known property,
/// validates the mode/runtime/call invariants, and produces a fully
/// populated [`MacroDef`].
fn parse_object_form_macro(
    call: &oxc::ast::ast::CallExpression<'_>,
    source: &str,
) -> Result<MacroDef, DeclarativeError> {
    // Exactly one argument: the options object.
    if call.arguments.len() != 1 {
        return Err(DeclarativeError::new(
            oxc_span_to_ir(call.span),
            "macroRules(...) expects exactly one object argument",
        ));
    }
    let Some(arg_expr) = call.arguments[0].as_expression() else {
        return Err(DeclarativeError::new(
            oxc_span_to_ir(call.span),
            "macroRules(...) argument must be an object expression",
        ));
    };
    let Expression::ObjectExpression(obj) = arg_expr else {
        return Err(DeclarativeError::new(
            oxc_span_to_ir(arg_expr.span()),
            "macroRules(...) argument must be an object literal",
        ));
    };

    let mut mode: Option<MacroMode> = None;
    let mut kind: Option<MacroKind> = None;
    let mut expand_arms: Option<Vec<MacroArm>> = None;
    let mut expand_span: SpanIR = oxc_span_to_ir(obj.span);
    let mut runtime: Option<String> = None;
    let mut call_arms: Option<Vec<MacroArm>> = None;
    let mut megamorphism_threshold: Option<u8> = None;

    for prop in &obj.properties {
        use oxc::ast::ast::{ObjectPropertyKind, PropertyKey};
        let ObjectPropertyKind::ObjectProperty(p) = prop else {
            // Spread (`...other`) isn't supported — it would defeat the
            // static analysis the expander relies on.
            return Err(DeclarativeError::new(
                oxc_span_to_ir(prop.span()),
                "spread properties are not allowed in the macroRules object form",
            ));
        };
        let key_name = match &p.key {
            PropertyKey::StaticIdentifier(id) => id.name.as_str().to_string(),
            PropertyKey::StringLiteral(s) => s.value.as_str().to_string(),
            _ => {
                return Err(DeclarativeError::new(
                    oxc_span_to_ir(p.key.span()),
                    "macroRules object form only supports static string / identifier keys",
                ));
            }
        };

        match key_name.as_str() {
            "mode" => {
                let Expression::StringLiteral(lit) = &p.value else {
                    return Err(DeclarativeError::new(
                        oxc_span_to_ir(p.value.span()),
                        format!(
                            "`mode` must be a string literal; expected one of {}",
                            MacroMode::known_values()
                        ),
                    ));
                };
                let parsed = MacroMode::from_str_value(lit.value.as_str()).ok_or_else(|| {
                    DeclarativeError::new(
                        oxc_span_to_ir(lit.span),
                        format!(
                            "unknown mode `{}`; expected one of {}",
                            lit.value,
                            MacroMode::known_values()
                        ),
                    )
                })?;
                mode = Some(parsed);
            }
            "expand" => {
                // `expand: macroRules\`...\`` — a nested tag-form template.
                let Expression::TaggedTemplateExpression(tagged) = &p.value else {
                    return Err(DeclarativeError::new(
                        oxc_span_to_ir(p.value.span()),
                        "`expand` must be a `macroRules`...`` tag-form template",
                    ));
                };
                let Expression::Identifier(tag_ident) = &tagged.tag else {
                    return Err(DeclarativeError::new(
                        oxc_span_to_ir(tagged.tag.span()),
                        "`expand` tag must be `macroRules`",
                    ));
                };
                if tag_ident.name.as_str() != MACRO_RULES_IDENT {
                    return Err(DeclarativeError::new(
                        oxc_span_to_ir(tagged.tag.span()),
                        "`expand` tag must be `macroRules`",
                    ));
                }
                let nested = parse_tag_form_macro(tagged, source)?;
                expand_arms = Some(nested.arms);
                expand_span = nested.span;
            }
            "runtime" => {
                let text = extract_plain_string(&p.value)?;
                runtime = Some(text);
            }
            "call" => {
                let Expression::TaggedTemplateExpression(tagged) = &p.value else {
                    return Err(DeclarativeError::new(
                        oxc_span_to_ir(p.value.span()),
                        "`call` must be a `macroRules`...`` tag-form template",
                    ));
                };
                let Expression::Identifier(tag_ident) = &tagged.tag else {
                    return Err(DeclarativeError::new(
                        oxc_span_to_ir(tagged.tag.span()),
                        "`call` tag must be `macroRules`",
                    ));
                };
                if tag_ident.name.as_str() != MACRO_RULES_IDENT {
                    return Err(DeclarativeError::new(
                        oxc_span_to_ir(tagged.tag.span()),
                        "`call` tag must be `macroRules`",
                    ));
                }
                let nested = parse_tag_form_macro(tagged, source)?;
                call_arms = Some(nested.arms);
            }
            "kind" => {
                let Expression::StringLiteral(lit) = &p.value else {
                    return Err(DeclarativeError::new(
                        oxc_span_to_ir(p.value.span()),
                        "`kind` must be a string literal; expected `\"value\"` or `\"type\"`",
                    ));
                };
                kind = Some(match lit.value.as_str() {
                    "value" => MacroKind::Value,
                    "type" => MacroKind::Type,
                    other => {
                        return Err(DeclarativeError::new(
                            oxc_span_to_ir(lit.span),
                            format!(
                                "unknown kind `{}`; expected `\"value\"` or `\"type\"`",
                                other
                            ),
                        ));
                    }
                });
            }
            "megamorphismThreshold" | "megamorphism_threshold" => {
                let Expression::NumericLiteral(n) = &p.value else {
                    return Err(DeclarativeError::new(
                        oxc_span_to_ir(p.value.span()),
                        "`megamorphismThreshold` must be a numeric literal",
                    ));
                };
                let v = n.value as i64;
                if !(0..=255).contains(&v) {
                    return Err(DeclarativeError::new(
                        oxc_span_to_ir(n.span),
                        "`megamorphismThreshold` must fit in a u8 (0..=255)",
                    ));
                }
                megamorphism_threshold = Some(v as u8);
            }
            other => {
                return Err(DeclarativeError::new(
                    oxc_span_to_ir(p.key.span()),
                    format!(
                        "unknown macroRules option `{}`; expected one of: mode, kind, expand, runtime, call, megamorphismThreshold",
                        other
                    ),
                ));
            }
        }
    }

    let effective_kind = kind.unwrap_or(MacroKind::Value);

    // Determine effective mode: explicit `mode` wins; otherwise infer
    // from which fields are present.
    let effective_mode = match mode {
        Some(m) => m,
        None => {
            // If the user provided runtime + call, default to Auto; otherwise
            // ExpandOnly is the safe fallback.
            if runtime.is_some() && call_arms.is_some() {
                MacroMode::Auto
            } else {
                MacroMode::ExpandOnly
            }
        }
    };

    // Validate that the object form is coherent.
    let Some(expand_arms) = expand_arms else {
        return Err(DeclarativeError::new(
            oxc_span_to_ir(obj.span),
            "macroRules object form requires an `expand` template",
        ));
    };
    if effective_mode.is_sharing() && (runtime.is_none() || call_arms.is_none()) {
        return Err(DeclarativeError::new(
            oxc_span_to_ir(obj.span),
            format!(
                "mode `{:?}` requires both `runtime` and `call` to be set",
                effective_mode
            ),
        ));
    }

    // Type-position macros can't participate in reverse-monomorphization
    // because there's no runtime at the type level — `runtime` / `call`
    // only make sense in value space. Reject the combination early so
    // users get a clear diagnostic instead of silently-wrong behavior.
    if effective_kind == MacroKind::Type {
        if effective_mode.is_sharing() {
            return Err(DeclarativeError::new(
                oxc_span_to_ir(obj.span),
                "type-position macros cannot use sharing modes (share-only / share-anyway / auto); type expansions have no runtime",
            ));
        }
        if runtime.is_some() || call_arms.is_some() {
            return Err(DeclarativeError::new(
                oxc_span_to_ir(obj.span),
                "type-position macros cannot declare `runtime` or `call` — those fields only apply in value position",
            ));
        }
    }

    let mut def = MacroDef::from_arms(String::new(), expand_arms, effective_mode, expand_span);
    def.kind = effective_kind;
    def.runtime = runtime;
    def.call_arms = call_arms;
    if let Some(t) = megamorphism_threshold {
        def.megamorphism_threshold = t;
    }
    Ok(def)
}

/// Extract the text of a `StringLiteral` or interpolation-free
/// `TemplateLiteral`. Used for the `runtime` field.
fn extract_plain_string(expr: &Expression<'_>) -> Result<String, DeclarativeError> {
    match expr {
        Expression::StringLiteral(s) => Ok(s.value.as_str().to_string()),
        Expression::TemplateLiteral(tpl) => {
            if !tpl.expressions.is_empty() {
                return Err(DeclarativeError::new(
                    oxc_span_to_ir(tpl.expressions[0].span()),
                    "`runtime` template literals may not contain `${...}` interpolations",
                ));
            }
            if tpl.quasis.len() != 1 {
                return Err(DeclarativeError::new(
                    oxc_span_to_ir(tpl.span),
                    "malformed template literal in `runtime`",
                ));
            }
            Ok(tpl.quasis[0].value.raw.as_str().to_string())
        }
        _ => Err(DeclarativeError::new(
            oxc_span_to_ir(expr.span()),
            "`runtime` must be a string literal or a template literal without interpolations",
        )),
    }
}

/// Return `true` if the program imports `{ macroRules }` from the rules
/// module. The local name must match exactly (no aliasing), so discovery
/// can rely on AST-level identifier matches instead of tracking renames.
///
/// This is the broad "does this file opt into declarative macros?" check.
/// It fires regardless of what else the import statement contains — a file
/// with `import { macroRules, type MacroInvocation } from "macroforge/rules"`
/// still counts. Use [`find_macro_rules_import_span`] instead when you
/// need to know whether the whole statement is safe to delete.
fn has_macro_import(program: &Program<'_>) -> bool {
    for stmt in &program.body {
        let Statement::ImportDeclaration(import) = stmt else {
            continue;
        };
        if import.source.value.as_str() != RULES_MODULE {
            continue;
        }
        let Some(specifiers) = &import.specifiers else {
            continue;
        };
        for spec in specifiers {
            match spec {
                ImportDeclarationSpecifier::ImportSpecifier(named) => {
                    if named.local.name.as_str() == MACRO_RULES_IDENT {
                        return true;
                    }
                }
                ImportDeclarationSpecifier::ImportDefaultSpecifier(default) => {
                    if default.local.name.as_str() == MACRO_RULES_IDENT {
                        return true;
                    }
                }
                ImportDeclarationSpecifier::ImportNamespaceSpecifier(_) => {
                    continue;
                }
            }
        }
    }
    false
}

/// Find a `import { macroRules } from "macroforge/rules";` statement with
/// *exactly* `macroRules` as its single specifier, and return the span of
/// the whole statement (in the 1-based SpanIR convention).
///
/// After the declarative pre-pass strips all
/// `const $name = macroRules\`...\`` declarations, the `macroRules` value
/// binding has no remaining uses. Under `noUnusedLocals`, TypeScript
/// flags the unused import. Stripping the whole import statement fixes
/// this, but only when `macroRules` is the *only* thing imported from
/// the rules module — if the user also imports (say) a type like
/// `MacroInvocation`, the statement stays and they live with one
/// `noUnusedLocals` warning they can silence manually.
///
/// Returns `None` if no matching import exists, or if the import carries
/// additional specifiers.
pub(super) fn find_macro_rules_import_span(program: &Program<'_>) -> Option<SpanIR> {
    for stmt in &program.body {
        let Statement::ImportDeclaration(import) = stmt else {
            continue;
        };
        if import.source.value.as_str() != RULES_MODULE {
            continue;
        }
        let Some(specifiers) = &import.specifiers else {
            continue;
        };
        // Must be exactly one specifier, and it must be the value import
        // of `macroRules`. Anything more and we leave the statement alone.
        if specifiers.len() != 1 {
            continue;
        }
        let spec = &specifiers[0];
        let is_macro_rules = match spec {
            ImportDeclarationSpecifier::ImportSpecifier(named) => {
                named.local.name.as_str() == MACRO_RULES_IDENT && named.import_kind.is_value()
            }
            ImportDeclarationSpecifier::ImportDefaultSpecifier(default) => {
                default.local.name.as_str() == MACRO_RULES_IDENT
            }
            ImportDeclarationSpecifier::ImportNamespaceSpecifier(_) => false,
        };
        if is_macro_rules {
            return Some(oxc_span_to_ir(import.span));
        }
    }
    None
}

/// Pull the static text out of a TaggedTemplateExpression's quasi.
///
/// MVP rejects templates with `${...}` interpolations — the macro body
/// must be pure text. If interpolations are present we emit an error
/// pointing at the first one.
fn extract_static_quasi<'a>(
    quasi: &TemplateLiteral<'a>,
    source: &'a str,
) -> Result<&'a str, DeclarativeError> {
    if !quasi.expressions.is_empty() {
        let first_interp = quasi.expressions[0].span();
        return Err(DeclarativeError::new(
            oxc_span_to_ir(first_interp),
            "macro template body cannot contain `${...}` interpolations",
        ));
    }
    if quasi.quasis.len() != 1 {
        // No interpolations implies exactly one quasi; any other count is
        // a parser weirdness we treat as an error.
        return Err(DeclarativeError::new(
            oxc_span_to_ir(quasi.span),
            "malformed macro template literal",
        ));
    }
    // The quasi's span covers the backticks; slice the source between them.
    // oxc spans are 0-based byte offsets.
    let span = quasi.span;
    let start = span.start as usize;
    let end = span.end as usize;
    if end <= start + 2 {
        // Empty backticks `` `` ``.
        return Ok("");
    }
    Ok(&source[start + 1..end - 1])
}

/// Convert a 0-based OXC span to a 1-based [`SpanIR`] (the patch
/// applicator's convention).
fn oxc_span_to_ir(span: oxc::span::Span) -> SpanIR {
    SpanIR::new(span.start + 1, span.end + 1)
}

/// If the source immediately following `span.end` (1-based) is a semicolon,
/// extend the span to cover it so the declaration gets cleanly deleted.
fn extend_to_semicolon(span: SpanIR, source: &str) -> SpanIR {
    // span.end is 1-based exclusive. Scan forward over ASCII whitespace
    // (stay on the same line) then optionally consume a single `;`.
    let mut idx = (span.end as usize).saturating_sub(1);
    let bytes = source.as_bytes();
    while idx < bytes.len() && matches!(bytes[idx], b' ' | b'\t') {
        idx += 1;
    }
    if idx < bytes.len() && bytes[idx] == b';' {
        SpanIR::new(span.start, (idx as u32) + 2)
    } else {
        span
    }
}

/// A macro resolved from a cross-file `/** import macro */` comment.
#[derive(Debug, Clone)]
pub struct ImportedMacro {
    /// The parsed definition (cloned from the project registry).
    pub def: MacroDef,
    /// Absolute path of the file that originally declared this macro.
    pub source_file: PathBuf,
}

/// Result of running cross-file import resolution alongside local discovery.
#[derive(Debug, Default)]
pub struct ResolvedImports {
    /// Macros imported via `/** import macro { ... } from "..." */` that
    /// the project registry resolved successfully.
    pub imported: Vec<ImportedMacro>,
    /// Non-fatal diagnostics (unresolved specifiers, missing definitions).
    pub diagnostics: Vec<Diagnostic>,
}

/// Scan `source` for `/** import macro { $name1, $name2 } from "./spec" */`
/// comments and resolve each `$`-prefixed name against the project-wide
/// declarative registry. Non-prefixed names are derive-macro imports and
/// are ignored here — the derive pipeline already handles them.
///
/// `importer_path` must be the absolute path of the file whose source is
/// being scanned; it's used as the anchor for relative specifier
/// resolution.
pub fn resolve_cross_file_imports(
    source: &str,
    importer_path: &Path,
    project_registry: &ProjectDeclarativeRegistry,
) -> ResolvedImports {
    let mut out = ResolvedImports::default();
    let imports = collect_macro_import_comments_pub(source);
    for (name, module) in imports {
        // Only `$`-prefixed names are declarative macros. Derive imports
        // (bare names like `Serialize`) fall through to the derive path.
        if !name.starts_with('$') {
            continue;
        }

        // Resolve the module specifier to an absolute file path by trying
        // the usual `.ts` / `.tsx` / `index.ts` variants relative to the
        // importing file's directory.
        let Some(resolved_path) = project_registry.resolve_specifier(importer_path, &module) else {
            out.diagnostics.push(Diagnostic {
                level: DiagnosticLevel::Error,
                message: format!(
                    "cannot resolve declarative macro import `{}` from module `{}`: file not found in project registry",
                    name, module
                ),
                span: None,
                notes: vec![],
                help: Some(format!(
                    "ensure `{}` is a file under the project root and contains declarative macros",
                    module
                )),
            });
            continue;
        };

        // `name` in the comment is `$vec`; the registry key is `vec`
        // (declarative registry stores names sans the `$`).
        let bare_name = &name[1..];
        let Some(file_macros) = project_registry.file_macros(&resolved_path) else {
            // resolve_specifier only returns paths known to the registry,
            // so file_macros should always succeed — but guard anyway.
            out.diagnostics.push(Diagnostic {
                level: DiagnosticLevel::Error,
                message: format!(
                    "declarative macro `{}` imported from `{}` but no macros found in that file",
                    name, module
                ),
                span: None,
                notes: vec![],
                help: None,
            });
            continue;
        };
        let Some(def) = file_macros.get(bare_name) else {
            out.diagnostics.push(Diagnostic {
                level: DiagnosticLevel::Error,
                message: format!("declarative macro `{}` not defined in `{}`", name, module),
                span: None,
                notes: vec![],
                help: Some(format!(
                    "`{}` declares these macros: {}",
                    module,
                    file_macros
                        .keys()
                        .map(|k| format!("${}", k))
                        .collect::<Vec<_>>()
                        .join(", ")
                )),
            });
            continue;
        };

        out.imported.push(ImportedMacro {
            def: def.clone(),
            source_file: resolved_path,
        });
    }
    out
}
