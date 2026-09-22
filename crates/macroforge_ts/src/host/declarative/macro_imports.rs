//! Removal of call-macro imports that expansion left without a use.
//!
//! A call macro (`$name(...)`) exists only at build time. Once every call is
//! expanded, its import would make the output load the macro package at
//! runtime for nothing, and `noUnusedLocals` rejects it.

use oxc::allocator::Allocator;
use oxc::ast::ast::{ImportDeclaration, ImportDeclarationSpecifier, Statement};
use oxc::parser::Parser;
use oxc::semantic::{Scoping, SemanticBuilder};
use oxc::span::GetSpan;

use crate::host::patch_applicator::PatchApplicator;
use crate::ts_syn::abi::{Patch, PatchCode, SpanIR};

/// Removes every `$name` import binding that nothing in `source` references,
/// and each import statement left with no bindings. Returns `None` when there
/// is nothing to remove.
pub(crate) fn strip_consumed_macro_imports(
    source: &str,
    file_name: &str,
) -> Result<Option<String>, String> {
    let allocator = Allocator::default();
    let parsed = Parser::new(&allocator, source, crate::source_type::for_path(file_name)).parse();
    if !parsed.diagnostics.is_empty() {
        return Err(format!(
            "parse error after macro expansion: {}",
            parsed
                .diagnostics
                .iter()
                .map(|diagnostic| diagnostic.to_string())
                .collect::<Vec<_>>()
                .join("; ")
        ));
    }
    let semantic = SemanticBuilder::new().build(&parsed.program);
    let scoping = semantic.semantic.scoping();

    let mut patches = Vec::new();
    for statement in &parsed.program.body {
        let Statement::ImportDeclaration(import) = statement else {
            continue;
        };
        if let Some(patch) = import_patch(source, import, scoping) {
            patches.push(patch);
        }
    }
    if patches.is_empty() {
        return Ok(None);
    }
    PatchApplicator::new(source, patches)
        .apply()
        .map(Some)
        .map_err(|err| format!("failed to remove consumed macro imports: {err}"))
}

/// The patch removing `import`'s consumed call-macro bindings, if it has any.
fn import_patch(source: &str, import: &ImportDeclaration<'_>, scoping: &Scoping) -> Option<Patch> {
    if import.import_kind.is_type() {
        return None;
    }
    let specifiers = import.specifiers.as_ref()?;
    let is_consumed = |specifier: &ImportDeclarationSpecifier<'_>| {
        let local = specifier.local();
        is_call_macro_name(&local.name) && scoping.symbol_is_unused(local.symbol_id())
    };
    let kept: Vec<&ImportDeclarationSpecifier<'_>> = specifiers
        .iter()
        .filter(|specifier| !is_consumed(specifier))
        .collect();
    if kept.len() == specifiers.len() {
        return None;
    }

    if kept.is_empty() {
        return Some(Patch::Delete {
            span: through_line_end(source, import.span),
        });
    }

    // Some bindings survive: rebuild the import from them, keeping each one's
    // original text and the module specifier and attributes as written.
    let text = |span: oxc::span::Span| &source[span.start as usize..span.end as usize];
    let mut clauses = Vec::new();
    let mut named = Vec::new();
    for specifier in kept {
        match specifier {
            ImportDeclarationSpecifier::ImportSpecifier(_) => named.push(text(specifier.span())),
            ImportDeclarationSpecifier::ImportDefaultSpecifier(_)
            | ImportDeclarationSpecifier::ImportNamespaceSpecifier(_) => {
                clauses.push(text(specifier.span()).to_string());
            }
        }
    }
    if !named.is_empty() {
        clauses.push(format!("{{ {} }}", named.join(", ")));
    }
    let tail = &source[import.source.span.start as usize..import.span.end as usize];
    Some(Patch::Replace {
        span: to_span_ir(import.span),
        code: PatchCode::Text(format!("import {} from {tail}", clauses.join(", "))),
        source_macro: None,
    })
}

/// A `$` followed by a letter: the shape reserved for call macros.
fn is_call_macro_name(name: &str) -> bool {
    let mut chars = name.chars();
    chars.next() == Some('$') && chars.next().is_some_and(|next| next.is_ascii_alphabetic())
}

/// `span` extended over the newline that ends its line, so a deleted
/// statement doesn't leave a blank line behind.
fn through_line_end(source: &str, span: oxc::span::Span) -> SpanIR {
    let end = span.end as usize;
    let end = if source[end..].starts_with('\n') {
        end + 1
    } else {
        end
    };
    SpanIR::new(span.start + 1, end as u32 + 1)
}

fn to_span_ir(span: oxc::span::Span) -> SpanIR {
    SpanIR::new(span.start + 1, span.end + 1)
}

#[cfg(test)]
mod tests {
    use super::strip_consumed_macro_imports;

    #[test]
    fn removes_an_import_whose_macros_were_all_expanded() {
        let source = "import { $state } from 'macros';\nconst count = createSignal(0);\n";
        let stripped = strip_consumed_macro_imports(source, "a.ts").unwrap();
        assert_eq!(
            stripped.as_deref(),
            Some("const count = createSignal(0);\n")
        );
    }

    #[test]
    fn keeps_runtime_bindings_and_macros_still_in_use() {
        let source = "import { $state, $derived, batch } from 'macros';\nbatch($derived);\n";
        let stripped = strip_consumed_macro_imports(source, "a.ts").unwrap();
        assert_eq!(
            stripped.as_deref(),
            Some("import { $derived, batch } from 'macros';\nbatch($derived);\n")
        );
    }

    #[test]
    fn leaves_ordinary_imports_alone() {
        let source = "import { unused } from 'lib';\nimport type { $T } from 'types';\n";
        assert_eq!(strip_consumed_macro_imports(source, "a.ts").unwrap(), None);
    }
}
