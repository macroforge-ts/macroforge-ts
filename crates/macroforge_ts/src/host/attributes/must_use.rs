//! `@mustUse` — emit a diagnostic at any call site that discards a return
//! value from an annotated function.
//!
//! Only walks call sites in the same source file; cross-file enforcement
//! requires the project-wide declarative registry and is deferred to a
//! follow-up iteration. That's consistent with Rust's `#[must_use]`, which
//! is per-function-definition but only enforced where the compiler sees the
//! call site.

use std::collections::HashSet;

use oxc::ast::ast::{Expression, Program, Statement};
use oxc::span::GetSpan;

use macroforge_ts_syn::config::MustUseConfig;

use super::discovery::{AttributeAnnotation, AttributeKind};
use crate::ts_syn::abi::{Diagnostic, DiagnosticLevel, Patch, SpanIR};

pub fn apply(
    program: &Program<'_>,
    annotations: &[&AttributeAnnotation],
    _source: &str,
    _config: &MustUseConfig,
) -> (Vec<Patch>, Vec<Diagnostic>) {
    let mut patches = Vec::new();
    let mut diagnostics = Vec::new();

    // Collect every @mustUse-annotated function name plus its reason (if any).
    let names: HashSet<&str> = annotations
        .iter()
        .filter(|ann| ann.kind == AttributeKind::MustUse)
        .map(|ann| ann.name.as_str())
        .filter(|s| !s.is_empty())
        .collect();
    if names.is_empty() {
        return (patches, diagnostics);
    }

    // Strip the `@mustUse` JSDoc lines so the annotation doesn't survive into
    // the generated output. Matches what `@cfg` and `@nonExhaustive` do — the
    // pre-pass owns the tag, downstream tools don't need to see it.
    for ann in annotations {
        if ann.kind == AttributeKind::MustUse {
            patches.push(Patch::Delete {
                span: ann.jsdoc_span,
            });
        }
    }

    // Walk top-level statements for discarded calls. A "discarded call" is a
    // call expression that sits directly inside an ExpressionStatement, whose
    // callee is one of the annotated names.
    for stmt in &program.body {
        check_statement(stmt, &names, &mut diagnostics);
    }

    (patches, diagnostics)
}

fn check_statement(stmt: &Statement<'_>, names: &HashSet<&str>, diagnostics: &mut Vec<Diagnostic>) {
    // Note: we don't descend into nested scopes. That's intentional for this
    // iteration — top-level statements cover the common case and keep the
    // walker small. A future pass can use oxc_ast_visit to scan bodies.
    let Statement::ExpressionStatement(expr_stmt) = stmt else {
        return;
    };
    let Expression::CallExpression(call) = &expr_stmt.expression else {
        return;
    };
    // Method calls (`foo.bar()`) are not caught — would require resolving
    // `bar` against the type registry to know if it's the annotated
    // function. Left for a follow-up.
    let Expression::Identifier(id) = &call.callee else {
        return;
    };
    let name = id.name.as_str();
    if !names.contains(name) {
        return;
    }
    let span = call.span();
    diagnostics.push(Diagnostic {
        level: DiagnosticLevel::Error,
        message: format!("[@mustUse] return value of `{name}` is being discarded"),
        span: Some(SpanIR {
            start: span.start + 1,
            end: span.end + 1,
        }),
        notes: Vec::new(),
        help: Some("Assign the result, return it, or otherwise consume it.".into()),
    });
}
