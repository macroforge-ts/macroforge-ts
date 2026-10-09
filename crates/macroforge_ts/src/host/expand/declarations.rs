//! The `.d.ts` surface of an expanded file: its declarations with the
//! type-level patches applied, stripped of every body and initializer, which
//! an ambient context does not allow.

use oxc::allocator::Allocator;
use oxc::ast::ast::{ClassBody, ClassElement, MethodDefinition};
use oxc::ast_visit::{VisitMut, walk_mut};
use oxc::codegen::{Codegen, CodegenOptions, CommentOptions};
use oxc::isolated_declarations::{IsolatedDeclarations, IsolatedDeclarationsOptions};
use oxc::parser::Parser;

use crate::host::error::{MacroError, Result};
use crate::host::{Diagnostic, DiagnosticLevel};

/// A file's declaration surface and what kept a declaration from being
/// typed exactly.
pub(super) struct DeclarationSurface {
    pub code: String,
    pub diagnostics: Vec<Diagnostic>,
}

/// The declarations of `typed_source`, the file `file_name` with its type
/// patches applied. A declaration whose type can only be inferred (a
/// function without a return type, say) is reported as a warning naming it.
pub(super) fn declaration_surface(
    typed_source: &str,
    file_name: &str,
) -> Result<DeclarationSurface> {
    let allocator = Allocator::default();
    let source_type = crate::source_type::for_path(file_name);
    let parsed = Parser::new(&allocator, typed_source, source_type).parse();
    // A type patch on a declaration file puts implementations in its ambient
    // declarations, which the parser reports but reads in full; the
    // declarations below strip them. Anything else is a broken patch.
    let tolerated = source_type.is_typescript_definition() && !parsed.fatal_error;
    if let Some(error) = parsed.diagnostics.first()
        && !tolerated
    {
        let at = error
            .labels
            .first()
            .and_then(|label| usize::try_from(label.offset()).ok())
            .and_then(|offset| source_line(typed_source, offset))
            .map_or_else(String::new, |(line, text)| {
                format!(" at line {line}: `{}`", text.trim())
            });
        return Err(MacroError::Patch(format!(
            "the type patches of {file_name} do not parse{at}: {}",
            error.message
        )));
    }
    let mut program = parsed.program;
    UnpairedSignatures.visit_program(&mut program);
    let declared = IsolatedDeclarations::new(
        &allocator,
        IsolatedDeclarationsOptions {
            strip_internal: false,
        },
    )
    .build(&program);
    let code = Codegen::new()
        .with_options(CodegenOptions {
            comments: CommentOptions {
                jsdoc: true,
                ..CommentOptions::disabled()
            },
            ..CodegenOptions::default()
        })
        .build(&declared.program)
        .code;
    let diagnostics = declared
        .diagnostics
        .iter()
        .map(|error| Diagnostic {
            level: DiagnosticLevel::Warning,
            message: format!("{file_name}: declaration surface: {}", error.message),
            span: None,
            notes: error
                .labels
                .iter()
                .filter_map(|label| {
                    let start = usize::try_from(label.offset()).ok()?;
                    let end = start.checked_add(usize::try_from(label.len()).ok()?)?;
                    typed_source.get(start..end)
                })
                .map(|declared| format!("at `{declared}`"))
                .collect(),
            help: error.help.as_ref().map(ToString::to_string),
        })
        .collect();
    Ok(DeclarationSurface { code, diagnostics })
}

/// The 1-based number and the text of the line holding byte `offset`.
fn source_line(source: &str, offset: usize) -> Option<(usize, &str)> {
    let before = source.get(..offset)?;
    let start = before.rfind('\n').map_or(0, |newline| newline + 1);
    let end = source
        .get(offset..)?
        .find('\n')
        .map_or(source.len(), |newline| offset + newline);
    Some((before.matches('\n').count() + 1, source.get(start..end)?))
}

/// Isolated declarations reads a class method without a body as an overload
/// signature and skips the next method with one as its implementation, by
/// position alone. A type patch adds members after signatures of other
/// names (a declaration file's, or a method a macro declares), so each such
/// member keeps only its signature instead of being skipped.
struct UnpairedSignatures;

impl<'a> VisitMut<'a> for UnpairedSignatures {
    fn visit_class_body(&mut self, body: &mut ClassBody<'a>) {
        let mut open_signature: Option<(Option<String>, bool)> = None;
        for element in body.body.iter_mut() {
            let ClassElement::MethodDefinition(method) = element else {
                continue;
            };
            let identity = (
                method.key.static_name().map(|name| name.into_owned()),
                method.r#static,
            );
            let is_signature = method.value.body.is_none() && !is_bodiless_by_kind(method);
            match &open_signature {
                _ if is_signature => open_signature = Some(identity),
                Some(signature) if signature.0.is_some() && *signature == identity => {
                    open_signature = None;
                }
                Some(_) if method.value.body.is_some() => {
                    method.value.body = None;
                    open_signature = Some(identity);
                }
                Some(_) | None => {}
            }
        }
        walk_mut::walk_class_body(self, body);
    }
}

/// An abstract or optional method has no body without being an overload.
fn is_bodiless_by_kind(method: &MethodDefinition<'_>) -> bool {
    method.r#type.is_abstract() || method.optional
}

#[cfg(test)]
#[path = "declarations_tests.rs"]
mod tests;
