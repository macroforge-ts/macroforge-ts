//! Conservative purity analyzer for `@buildtime`-adjacent source.
//!
//! When a `@buildtime` body references functions defined elsewhere in
//! the same file, the pre-pass snapshots the file (minus its
//! `@buildtime` decls) and hands the snapshot to the sandbox as a
//! sibling module. For that to be safe, the snapshot must have no
//! top-level side effects — no `console.log("loading...")`, no
//! `fetch().then(...)`, no `writeFileSync`, no implicit state mutation.
//!
//! The analyzer walks the program and rejects any top-level statement
//! that isn't one of:
//!
//! - `import ...`
//! - `export ...` (when the exported item is itself pure)
//! - `const NAME = PURE_EXPR;`
//! - `function NAME() { ... }`
//! - `class NAME { ... }` (declaration only; body contents are runtime)
//! - `type NAME = ...` / `interface NAME { ... }`
//!
//! This is deliberately strict: false positives (pure code the analyzer
//! rejects) are better than false negatives (impure code that gets
//! loaded and surprises users).

use oxc::ast::ast::{Declaration, Expression, Program, Statement};

use crate::ts_syn::abi::SpanIR;

/// Outcome of the purity analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Purity {
    /// The source is safe to load as a sibling module.
    Pure,
    /// The source contains at least one impure top-level statement.
    /// `span` points at the first offender so the host can blame it
    /// in a diagnostic.
    Impure { reason: &'static str, span: SpanIR },
}

/// Analyze a program for top-level purity.
pub fn analyze(program: &Program<'_>) -> Purity {
    for stmt in &program.body {
        if let Some((reason, span)) = impurity_of(stmt) {
            return Purity::Impure { reason, span };
        }
    }
    Purity::Pure
}

fn impurity_of(stmt: &Statement<'_>) -> Option<(&'static str, SpanIR)> {
    use oxc::span::GetSpan;
    match stmt {
        // Pure structural declarations.
        Statement::ImportDeclaration(_) => None,
        Statement::ExportNamedDeclaration(export) => match &export.declaration {
            Some(decl) => impurity_of_declaration(decl, export.span),
            None => None, // re-export: `export { X }` has no side effects
        },
        Statement::ExportDefaultDeclaration(_) => None,
        Statement::ExportAllDeclaration(_) => None,
        Statement::VariableDeclaration(var) => {
            for declarator in &var.declarations {
                if let Some(init) = &declarator.init {
                    if let Some(reason) = expression_side_effect(init) {
                        return Some((reason, span_to_ir(declarator.span)));
                    }
                }
            }
            None
        }
        Statement::FunctionDeclaration(_) => None,
        Statement::ClassDeclaration(_) => None,
        Statement::TSTypeAliasDeclaration(_) => None,
        Statement::TSInterfaceDeclaration(_) => None,
        Statement::TSEnumDeclaration(_) => None,
        Statement::TSModuleDeclaration(_) => None,
        // Side-effectful bare expressions like `console.log(...)` or
        // `init();` are rejected — even if the function is pure, the
        // act of calling at module load is a side effect.
        Statement::ExpressionStatement(expr) => {
            Some(("top-level expression statement", span_to_ir(expr.span)))
        }
        // Any imperative block at top level is a side effect.
        Statement::IfStatement(s) => Some(("top-level `if`", span_to_ir(s.span))),
        Statement::ForStatement(s) => Some(("top-level `for`", span_to_ir(s.span))),
        Statement::WhileStatement(s) => Some(("top-level `while`", span_to_ir(s.span))),
        Statement::ThrowStatement(s) => Some(("top-level `throw`", span_to_ir(s.span))),
        Statement::TryStatement(s) => Some(("top-level `try`", span_to_ir(s.span))),
        Statement::ReturnStatement(s) => Some(("top-level `return`", span_to_ir(s.span))),
        Statement::BlockStatement(s) => Some(("top-level block", span_to_ir(s.span))),
        Statement::LabeledStatement(s) => Some(("top-level label", span_to_ir(s.span))),
        other => Some((
            "top-level statement with side effects",
            span_to_ir(other.span()),
        )),
    }
}

fn impurity_of_declaration<'a>(
    decl: &Declaration<'a>,
    fallback_span: oxc::span::Span,
) -> Option<(&'static str, SpanIR)> {
    match decl {
        Declaration::VariableDeclaration(var) => {
            for declarator in &var.declarations {
                if let Some(init) = &declarator.init {
                    if let Some(reason) = expression_side_effect(init) {
                        return Some((reason, span_to_ir(declarator.span)));
                    }
                }
            }
            None
        }
        Declaration::FunctionDeclaration(_) => None,
        Declaration::ClassDeclaration(_) => None,
        Declaration::TSTypeAliasDeclaration(_) => None,
        Declaration::TSInterfaceDeclaration(_) => None,
        Declaration::TSEnumDeclaration(_) => None,
        Declaration::TSModuleDeclaration(_) => None,
        _ => Some(("unsupported export declaration", span_to_ir(fallback_span))),
    }
}

/// Return a reason string if `expr` has visible side effects (function
/// calls, new-expressions, assignments, awaits). Returns `None` for
/// pure literals, identifiers, and simple operators over them.
pub(crate) fn expression_side_effect(expr: &Expression<'_>) -> Option<&'static str> {
    match expr {
        Expression::CallExpression(_) => Some("top-level function call"),
        Expression::NewExpression(_) => Some("top-level `new`"),
        Expression::AssignmentExpression(_) => Some("top-level assignment"),
        Expression::UpdateExpression(_) => Some("top-level update expression"),
        Expression::AwaitExpression(_) => Some("top-level await"),
        Expression::YieldExpression(_) => Some("top-level yield"),
        Expression::TaggedTemplateExpression(_) => Some("top-level tagged template"),
        // Recurse into compound expressions. Conservative: if any
        // subexpression has a side effect, the whole thing does.
        Expression::SequenceExpression(seq) => {
            for e in &seq.expressions {
                if let Some(reason) = expression_side_effect(e) {
                    return Some(reason);
                }
            }
            None
        }
        Expression::ArrayExpression(arr) => {
            for el in &arr.elements {
                if let oxc::ast::ast::ArrayExpressionElement::SpreadElement(_) = el {
                    return Some("array spread at top level");
                }
                if let Some(e) = el.as_expression() {
                    if let Some(reason) = expression_side_effect(e) {
                        return Some(reason);
                    }
                }
            }
            None
        }
        Expression::ObjectExpression(obj) => {
            for prop in &obj.properties {
                match prop {
                    oxc::ast::ast::ObjectPropertyKind::ObjectProperty(p) => {
                        if let Some(reason) = expression_side_effect(&p.value) {
                            return Some(reason);
                        }
                    }
                    oxc::ast::ast::ObjectPropertyKind::SpreadProperty(_) => {
                        return Some("object spread at top level");
                    }
                }
            }
            None
        }
        Expression::BinaryExpression(b) => {
            expression_side_effect(&b.left).or_else(|| expression_side_effect(&b.right))
        }
        Expression::LogicalExpression(l) => {
            expression_side_effect(&l.left).or_else(|| expression_side_effect(&l.right))
        }
        Expression::UnaryExpression(u) => expression_side_effect(&u.argument),
        Expression::ConditionalExpression(c) => expression_side_effect(&c.test)
            .or_else(|| expression_side_effect(&c.consequent))
            .or_else(|| expression_side_effect(&c.alternate)),
        Expression::TemplateLiteral(lit) => {
            for e in &lit.expressions {
                if let Some(reason) = expression_side_effect(e) {
                    return Some(reason);
                }
            }
            None
        }
        Expression::ParenthesizedExpression(p) => expression_side_effect(&p.expression),
        Expression::TSAsExpression(a) => expression_side_effect(&a.expression),
        Expression::TSSatisfiesExpression(s) => expression_side_effect(&s.expression),
        Expression::TSNonNullExpression(n) => expression_side_effect(&n.expression),
        Expression::TSTypeAssertion(t) => expression_side_effect(&t.expression),
        // Everything else (literals, identifiers, functions-as-values,
        // member accesses, this, super) is either purely structural or
        // accepted as a pure reference.
        _ => None,
    }
}

fn span_to_ir(span: oxc::span::Span) -> SpanIR {
    SpanIR {
        start: span.start,
        end: span.end,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxc::allocator::Allocator;
    use oxc::parser::Parser as OxcParser;
    use oxc::span::SourceType;

    fn analyze_src(src: &str) -> Purity {
        let allocator = Allocator::default();
        let ret = OxcParser::new(&allocator, src, SourceType::ts()).parse();
        assert!(ret.errors.is_empty(), "parse: {:?}", ret.errors);
        analyze(&ret.program)
    }

    #[test]
    fn pure_function_declaration() {
        assert_eq!(analyze_src("function f() { return 1; }"), Purity::Pure);
    }

    #[test]
    fn pure_const_with_literal() {
        assert_eq!(analyze_src("const X = 42;"), Purity::Pure);
    }

    #[test]
    fn pure_const_with_arithmetic() {
        assert_eq!(analyze_src("const X = 1 + 2 * 3;"), Purity::Pure);
    }

    #[test]
    fn pure_type_alias() {
        assert_eq!(analyze_src("type X = string | number;"), Purity::Pure);
    }

    #[test]
    fn pure_interface() {
        assert_eq!(analyze_src("interface X { a: number; }"), Purity::Pure);
    }

    #[test]
    fn pure_class() {
        assert_eq!(analyze_src("class X { foo() { return 1; } }"), Purity::Pure);
    }

    #[test]
    fn pure_import() {
        assert_eq!(analyze_src(r#"import { x } from "./y";"#), Purity::Pure);
    }

    #[test]
    fn pure_export_reexport() {
        assert_eq!(analyze_src(r#"export { a } from "./other";"#), Purity::Pure);
    }

    #[test]
    fn pure_export_named_decl() {
        assert_eq!(
            analyze_src("export function f() { return 1; }"),
            Purity::Pure
        );
    }

    #[test]
    fn impure_top_level_call() {
        let result = analyze_src("console.log('hi');");
        assert!(matches!(result, Purity::Impure { .. }));
    }

    #[test]
    fn impure_top_level_new() {
        let result = analyze_src("const X = new Date();");
        match result {
            Purity::Impure { reason, .. } => assert!(reason.contains("`new`"), "reason: {reason}"),
            other => panic!("expected Impure, got {:?}", other),
        }
    }

    #[test]
    fn impure_top_level_if() {
        assert!(matches!(analyze_src("if (1) { }"), Purity::Impure { .. }));
    }

    #[test]
    fn impure_const_with_call() {
        let result = analyze_src("const X = fn();");
        assert!(matches!(result, Purity::Impure { .. }));
    }

    #[test]
    fn pure_class_with_method_call_in_body_is_still_pure() {
        // Method bodies don't execute at module load; only the decl
        // matters. Class declarations are always pure at top level.
        assert_eq!(
            analyze_src("class X { run() { console.log('x'); } }"),
            Purity::Pure
        );
    }

    #[test]
    fn impure_top_level_assignment() {
        let result = analyze_src("let x = 1; x = 2;");
        assert!(matches!(result, Purity::Impure { .. }));
    }

    #[test]
    fn pure_const_referencing_other_const() {
        assert_eq!(analyze_src("const A = 1; const B = A + 1;"), Purity::Pure);
    }
}
