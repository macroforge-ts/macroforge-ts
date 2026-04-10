//! Walk an OXC program, rewrite calls to registered declarative macros,
//! and emit the patches that strip out the original macroRules definitions.

use oxc::ast::ast::{
    BindingPattern, Expression, ObjectPropertyKind, Program, Statement, VariableDeclarationKind,
};
use oxc::span::GetSpan;

use crate::ts_syn::abi::{Diagnostic, DiagnosticLevel, Patch, PatchCode, SpanIR};

use super::discovery::{DiscoveredMacro, MACRO_RULES_IDENT};
use super::expander::{ExpansionContext, expand_body};
use super::matcher::{MatchError, match_invocation};
use super::registry::DeclarativeMacroRegistry;

/// Collected output of the rewriter pass.
#[derive(Debug, Default, Clone)]
pub struct RewriteOutput {
    /// Patches to apply: `Patch::Replace` for each call site, and
    /// `Patch::Delete` for each declaration to strip.
    pub patches: Vec<Patch>,
    /// Diagnostics for failed matches (non-fatal; the build continues).
    pub diagnostics: Vec<Diagnostic>,
}

/// Walk `program` for `$name(...)` call sites of macros in `registry`,
/// match them, expand them, and emit patches. Also emits `Patch::Delete`
/// patches for every discovered macro's declaration span so the original
/// `` const $name = macroRules`...` `` is stripped from the output.
pub fn rewrite(
    program: &Program<'_>,
    source: &str,
    registry: &DeclarativeMacroRegistry,
    discovered: &[DiscoveredMacro],
) -> RewriteOutput {
    let mut out = RewriteOutput::default();

    // Strip each discovered macro's declaration.
    for dm in discovered {
        out.patches.push(Patch::Delete { span: dm.def_span });
    }

    // Walk statements looking for macro call sites. The expansion counter
    // is per-call (not global) so snapshot output is deterministic — each
    // `rewrite()` call starts numbering expansions from 1.
    let mut ctx = WalkCtx {
        registry,
        source,
        output: &mut out,
        counter: 0,
    };
    for stmt in &program.body {
        walk_statement(stmt, &mut ctx);
    }
    out
}

struct WalkCtx<'a> {
    registry: &'a DeclarativeMacroRegistry,
    source: &'a str,
    output: &'a mut RewriteOutput,
    counter: u32,
}

impl WalkCtx<'_> {
    fn next_id(&mut self) -> u32 {
        self.counter += 1;
        self.counter
    }
}

// ---------------------------------------------------------------------------
// AST walking
// ---------------------------------------------------------------------------
//
// The MVP walker descends through the most common expression-position and
// statement-position nodes where a user would invoke a macro. It is not a
// full OXC visitor — it's intentionally small and only covers the nodes
// where `$name(...)` invocations can plausibly appear. Less-common nodes
// (class bodies, decorators, type expressions, destructuring defaults)
// are skipped; macros in those positions are out of scope for MVP.

fn walk_statement(stmt: &Statement<'_>, ctx: &mut WalkCtx<'_>) {
    match stmt {
        Statement::BlockStatement(block) => {
            for s in &block.body {
                walk_statement(s, ctx);
            }
        }
        Statement::ExpressionStatement(expr_stmt) => {
            walk_expression(&expr_stmt.expression, ctx, ExpansionContext::Statement);
        }
        Statement::IfStatement(if_stmt) => {
            walk_expression(&if_stmt.test, ctx, ExpansionContext::Expression);
            walk_statement(&if_stmt.consequent, ctx);
            if let Some(alt) = &if_stmt.alternate {
                walk_statement(alt, ctx);
            }
        }
        Statement::WhileStatement(w) => {
            walk_expression(&w.test, ctx, ExpansionContext::Expression);
            walk_statement(&w.body, ctx);
        }
        Statement::DoWhileStatement(d) => {
            walk_expression(&d.test, ctx, ExpansionContext::Expression);
            walk_statement(&d.body, ctx);
        }
        Statement::ForStatement(f) => {
            if let Some(init) = &f.init
                && let Some(expr) = init.as_expression()
            {
                walk_expression(expr, ctx, ExpansionContext::Expression);
                // VariableDeclaration init is walked through the declarator path
                // below when we hit `Statement::VariableDeclaration` at the top
                // level; inside a for-init it's rare and we skip descending.
            }
            if let Some(test) = &f.test {
                walk_expression(test, ctx, ExpansionContext::Expression);
            }
            if let Some(update) = &f.update {
                walk_expression(update, ctx, ExpansionContext::Expression);
            }
            walk_statement(&f.body, ctx);
        }
        Statement::ForInStatement(f) => {
            walk_expression(&f.right, ctx, ExpansionContext::Expression);
            walk_statement(&f.body, ctx);
        }
        Statement::ForOfStatement(f) => {
            walk_expression(&f.right, ctx, ExpansionContext::Expression);
            walk_statement(&f.body, ctx);
        }
        Statement::ReturnStatement(r) => {
            if let Some(expr) = &r.argument {
                walk_expression(expr, ctx, ExpansionContext::Expression);
            }
        }
        Statement::SwitchStatement(s) => {
            walk_expression(&s.discriminant, ctx, ExpansionContext::Expression);
            for case in &s.cases {
                if let Some(test) = &case.test {
                    walk_expression(test, ctx, ExpansionContext::Expression);
                }
                for stmt in &case.consequent {
                    walk_statement(stmt, ctx);
                }
            }
        }
        Statement::ThrowStatement(t) => {
            walk_expression(&t.argument, ctx, ExpansionContext::Expression);
        }
        Statement::TryStatement(t) => {
            for stmt in &t.block.body {
                walk_statement(stmt, ctx);
            }
            if let Some(handler) = &t.handler {
                for stmt in &handler.body.body {
                    walk_statement(stmt, ctx);
                }
            }
            if let Some(finalizer) = &t.finalizer {
                for stmt in &finalizer.body {
                    walk_statement(stmt, ctx);
                }
            }
        }
        Statement::VariableDeclaration(v) => {
            // Skip the declarations of the declarative macros themselves;
            // the rewriter already emitted Delete patches for those.
            if is_macro_definition_declaration(v) {
                return;
            }
            for decl in &v.declarations {
                if let Some(init) = &decl.init {
                    walk_expression(init, ctx, ExpansionContext::Expression);
                }
            }
        }
        Statement::LabeledStatement(l) => {
            walk_statement(&l.body, ctx);
        }
        Statement::WithStatement(w) => {
            walk_expression(&w.object, ctx, ExpansionContext::Expression);
            walk_statement(&w.body, ctx);
        }
        Statement::FunctionDeclaration(f) => {
            if let Some(body) = &f.body {
                for s in &body.statements {
                    walk_statement(s, ctx);
                }
            }
        }
        // Import, export, interface, type-alias, enum, class declarations don't
        // contain value-position macro calls at the outer level in typical code.
        // We skip them for MVP; if users need macros in those positions we can
        // extend the walker.
        _ => {}
    }
}

fn is_macro_definition_declaration(v: &oxc::ast::ast::VariableDeclaration<'_>) -> bool {
    if v.kind != VariableDeclarationKind::Const || v.declarations.len() != 1 {
        return false;
    }
    let d = &v.declarations[0];
    let BindingPattern::BindingIdentifier(bi) = &d.id else {
        return false;
    };
    if !bi.name.as_str().starts_with('$') {
        return false;
    }
    let Some(init) = &d.init else {
        return false;
    };
    let Expression::TaggedTemplateExpression(tagged) = init else {
        return false;
    };
    let Expression::Identifier(id) = &tagged.tag else {
        return false;
    };
    id.name.as_str() == MACRO_RULES_IDENT
}

fn walk_expression(expr: &Expression<'_>, ctx: &mut WalkCtx<'_>, context: ExpansionContext) {
    // Try to rewrite this call expression first. If it becomes a patch,
    // don't descend into its arguments — the arguments are now part of
    // the replacement text and walking them would double-patch.
    if let Expression::CallExpression(call) = expr
        && try_rewrite_call(call, ctx, context)
    {
        return;
    }

    match expr {
        Expression::ArrayExpression(arr) => {
            for elem in &arr.elements {
                if let Some(expr) = elem.as_expression() {
                    walk_expression(expr, ctx, ExpansionContext::Expression);
                }
            }
        }
        Expression::ObjectExpression(obj) => {
            for prop in &obj.properties {
                if let ObjectPropertyKind::ObjectProperty(p) = prop {
                    walk_expression(&p.value, ctx, ExpansionContext::Expression);
                }
            }
        }
        Expression::CallExpression(call) => {
            walk_expression(&call.callee, ctx, ExpansionContext::Expression);
            for arg in &call.arguments {
                if let Some(expr) = arg.as_expression() {
                    walk_expression(expr, ctx, ExpansionContext::Expression);
                }
            }
        }
        Expression::NewExpression(new_expr) => {
            walk_expression(&new_expr.callee, ctx, ExpansionContext::Expression);
            for arg in &new_expr.arguments {
                if let Some(expr) = arg.as_expression() {
                    walk_expression(expr, ctx, ExpansionContext::Expression);
                }
            }
        }
        Expression::BinaryExpression(b) => {
            walk_expression(&b.left, ctx, ExpansionContext::Expression);
            walk_expression(&b.right, ctx, ExpansionContext::Expression);
        }
        Expression::LogicalExpression(b) => {
            walk_expression(&b.left, ctx, ExpansionContext::Expression);
            walk_expression(&b.right, ctx, ExpansionContext::Expression);
        }
        Expression::UnaryExpression(u) => {
            walk_expression(&u.argument, ctx, ExpansionContext::Expression);
        }
        Expression::AssignmentExpression(a) => {
            walk_expression(&a.right, ctx, ExpansionContext::Expression);
        }
        Expression::ConditionalExpression(c) => {
            walk_expression(&c.test, ctx, ExpansionContext::Expression);
            walk_expression(&c.consequent, ctx, ExpansionContext::Expression);
            walk_expression(&c.alternate, ctx, ExpansionContext::Expression);
        }
        Expression::SequenceExpression(s) => {
            for expr in &s.expressions {
                walk_expression(expr, ctx, ExpansionContext::Expression);
            }
        }
        Expression::ParenthesizedExpression(p) => {
            walk_expression(&p.expression, ctx, context);
        }
        Expression::StaticMemberExpression(m) => {
            walk_expression(&m.object, ctx, ExpansionContext::Expression);
        }
        Expression::ComputedMemberExpression(m) => {
            walk_expression(&m.object, ctx, ExpansionContext::Expression);
            walk_expression(&m.expression, ctx, ExpansionContext::Expression);
        }
        Expression::ArrowFunctionExpression(f) => {
            for stmt in &f.body.statements {
                walk_statement(stmt, ctx);
            }
        }
        Expression::FunctionExpression(f) => {
            if let Some(body) = &f.body {
                for stmt in &body.statements {
                    walk_statement(stmt, ctx);
                }
            }
        }
        Expression::TemplateLiteral(t) => {
            for expr in &t.expressions {
                walk_expression(expr, ctx, ExpansionContext::Expression);
            }
        }
        Expression::AwaitExpression(a) => {
            walk_expression(&a.argument, ctx, ExpansionContext::Expression);
        }
        Expression::YieldExpression(y) => {
            if let Some(arg) = &y.argument {
                walk_expression(arg, ctx, ExpansionContext::Expression);
            }
        }
        Expression::TSAsExpression(t) => {
            walk_expression(&t.expression, ctx, context);
        }
        Expression::TSSatisfiesExpression(t) => {
            walk_expression(&t.expression, ctx, context);
        }
        Expression::TSNonNullExpression(t) => {
            walk_expression(&t.expression, ctx, context);
        }
        Expression::TSTypeAssertion(t) => {
            walk_expression(&t.expression, ctx, context);
        }
        _ => {}
    }
}

/// If `call` is a `$name(...)` invocation of a registered macro, emit a
/// `Patch::Replace` for the call site and return `true`. Returns `false`
/// if the callee isn't a registered macro; the caller will then descend
/// into the call's arguments normally.
fn try_rewrite_call(
    call: &oxc::ast::ast::CallExpression<'_>,
    ctx: &mut WalkCtx<'_>,
    context: ExpansionContext,
) -> bool {
    let Expression::Identifier(callee) = &call.callee else {
        return false;
    };
    let callee_name = callee.name.as_str();
    if !callee_name.starts_with('$') {
        return false;
    }
    let name = &callee_name[1..];
    let Some(def_arc) = ctx.registry.lookup(name) else {
        return false;
    };
    let def = def_arc.as_ref();

    // Match call args against arms.
    match match_invocation(def, &call.arguments, ctx.source) {
        Ok(result) => {
            let arm = &def.arms[result.arm_index];
            match expand_body(&arm.body, &result.bindings, ctx.next_id(), context) {
                Ok(expanded) => {
                    let span = call.span;
                    let span_ir = SpanIR::new(span.start + 1, span.end + 1);
                    ctx.output.patches.push(Patch::Replace {
                        span: span_ir,
                        code: PatchCode::Text(expanded),
                        source_macro: Some(format!("${}", name)),
                    });
                    true
                }
                Err(e) => {
                    let span = call.span();
                    ctx.output.diagnostics.push(Diagnostic {
                        level: DiagnosticLevel::Error,
                        message: format!("error expanding macro `${}`: {}", name, e),
                        span: Some(SpanIR::new(span.start + 1, span.end + 1)),
                        notes: vec![],
                        help: None,
                    });
                    true
                }
            }
        }
        Err(match_err) => {
            let span = call.span();
            let help = match &match_err {
                MatchError::NoArmMatched { tried } => {
                    if tried.is_empty() {
                        None
                    } else {
                        Some(format!("tried patterns: {}", tried.join(" | ")))
                    }
                }
                _ => None,
            };
            ctx.output.diagnostics.push(Diagnostic {
                level: DiagnosticLevel::Error,
                message: format!(
                    "macro `${}` invocation did not match any arm: {}",
                    name, match_err
                ),
                span: Some(SpanIR::new(span.start + 1, span.end + 1)),
                notes: vec![],
                help,
            });
            true
        }
    }
}
