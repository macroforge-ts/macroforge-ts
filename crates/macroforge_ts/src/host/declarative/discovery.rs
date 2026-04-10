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

use oxc::ast::ast::{
    BindingPattern, Declaration, Expression, ImportDeclarationSpecifier, Program, Statement,
    TemplateLiteral, VariableDeclarationKind,
};
use oxc::span::GetSpan;

use crate::ts_syn::abi::SpanIR;
use crate::ts_syn::declarative::{DeclarativeError, MacroDef, parse_macro_def};

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

        // Extract tagged template where the tag is `macroRules`.
        let Some(init) = &declarator.init else {
            continue;
        };
        let Expression::TaggedTemplateExpression(tagged) = init else {
            continue;
        };
        let Expression::Identifier(tag_ident) = &tagged.tag else {
            continue;
        };
        if tag_ident.name.as_str() != MACRO_RULES_IDENT {
            continue;
        }

        // Extract the quasi (template static text).
        let quasi_text = extract_static_quasi(&tagged.quasi, source)?;

        // Compute spans. Patch spans are 1-based; OXC is 0-based.
        let decl_span = oxc_span_to_ir(stmt.span());
        // Account for the trailing semicolon if present in the source.
        let def_span = extend_to_semicolon(decl_span, source);

        // The quasi span in OXC covers the backtick-delimited text including
        // the backticks. We want the text between them: trim one off each end.
        let quasi_span_raw = oxc_span_to_ir(tagged.quasi.span);
        let inner_start = quasi_span_raw.start.saturating_add(1);
        let inner_end = quasi_span_raw.end.saturating_sub(1);
        let template_span = SpanIR::new(inner_start.min(inner_end), inner_end);

        let mut def = parse_macro_def(quasi_text, template_span)?;
        // Populate the name from the surrounding binding (sans the `$`).
        def.name = binding_name[1..].to_string();

        out.push(DiscoveredMacro { def, def_span });
    }

    Ok(out)
}

/// Return `true` if the program imports `{ macroRules }` from the rules
/// module. The local name must match exactly (no aliasing), so discovery
/// can rely on AST-level identifier matches instead of tracking renames.
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
                    let local = named.local.name.as_str();
                    if local == MACRO_RULES_IDENT {
                        return true;
                    }
                }
                ImportDeclarationSpecifier::ImportDefaultSpecifier(default) => {
                    if default.local.name.as_str() == MACRO_RULES_IDENT {
                        return true;
                    }
                }
                ImportDeclarationSpecifier::ImportNamespaceSpecifier(_) => {
                    // Namespace import like `import * as rules from "macroforge/rules"`.
                    // MVP doesn't support accessing `macroRules` through a
                    // namespace; treat it as not-imported.
                    continue;
                }
            }
        }
    }
    false
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
