//! Walk an OXC program, rewrite calls to registered declarative macros,
//! and emit the patches that strip out the original macroRules definitions.

use std::collections::HashSet;

use oxc::ast::ast::{
    BindingPattern, Expression, ObjectPropertyKind, Program, Statement, VariableDeclarationKind,
};
use oxc::span::GetSpan;

use crate::ts_syn::abi::{Diagnostic, DiagnosticLevel, Patch, PatchCode, SpanIR};
use crate::ts_syn::declarative::{MacroArm, MacroDef, MacroMode};

use super::BuildMode;
use super::discovery::{DiscoveredMacro, MACRO_RULES_IDENT, find_macro_rules_import_span};
use super::expander::{ExpansionContext, expand_body_with_registry};
use super::matcher::{MatchError, match_invocation_against_arms};
use super::megamorph::{
    self, MegamorphReport, Recommendation, ResolvedCallSite, extract_type_shape,
};
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
///
/// `build_mode` controls reverse-monomorphization: in `Dev`, all modes
/// behave like `ExpandOnly`; in `Prod`, `ShareOnly` / `ShareAnyway`
/// emit a top-of-file runtime helper and each call site becomes a call
/// to it. `Auto` in `Prod` is a stub for Phase 9c — it currently falls
/// back to `ExpandOnly` behavior until the megamorphism analyzer lands.
pub fn rewrite(
    program: &Program<'_>,
    source: &str,
    registry: &DeclarativeMacroRegistry,
    discovered: &[DiscoveredMacro],
    build_mode: BuildMode,
    type_registry: Option<&crate::ts_syn::abi::ir::type_registry::TypeRegistry>,
) -> RewriteOutput {
    let mut out = RewriteOutput::default();

    // Strip each discovered macro's declaration.
    for dm in discovered {
        out.patches.push(Patch::Delete { span: dm.def_span });
    }

    // Strip the `import { macroRules } from "macroforge/rules"` statement
    // too — after all declarations are deleted the import is dead code and
    // `noUnusedLocals` would flag it. This only runs when the import is
    // present; files that only consume macros via `/** import macro */`
    // JSDoc (and never import `macroRules` as a value) are untouched.
    if let Some(import_span) = find_macro_rules_import_span(program) {
        out.patches.push(Patch::Delete { span: import_span });
    }

    // Phase 9c: if any `Auto`-mode macros are registered AND we're in
    // prod, run the megamorphism analyzer over a first-pass walk of the
    // program. The resulting report is consulted in `try_rewrite_call`
    // to pick share vs. cluster vs. expand on a per-macro basis.
    //
    // Phase 14: when the project-wide type registry is available, pass
    // it through so each recorded call site captures a structural
    // fingerprint (sorted field names) for the Jaccard-similarity
    // clusterer.
    //
    // In dev, or when no Auto macros are registered, we skip the first
    // pass entirely — the second pass (the existing rewrite walk) does
    // all the work in one go.
    let has_auto = registry.iter().any(|(_, def)| def.mode == MacroMode::Auto);
    let megamorph_report = if has_auto && build_mode == BuildMode::Prod {
        let sites = collect_auto_call_sites(program, registry, type_registry);
        Some(megamorph::analyze(registry, &sites, 4))
    } else {
        None
    };

    // Emit non-fatal diagnostics for any Auto macro the analyzer
    // flagged as megamorphic.
    if let Some(report) = &megamorph_report {
        for (name, info) in &report.per_macro {
            match &info.recommendation {
                Recommendation::Cluster(clusters) => {
                    out.diagnostics.push(Diagnostic {
                        level: DiagnosticLevel::Warning,
                        message: format!(
                            "macro `${}` is called with {} distinct argument shapes; shared runtime would be megamorphic. Partitioned into {} clusters. Use `mode: \"share-anyway\"` to silence.",
                            name,
                            info.distinct_shapes,
                            clusters.len()
                        ),
                        span: None,
                        notes: vec![],
                        help: None,
                    });
                }
                Recommendation::ForceExpand => {
                    out.diagnostics.push(Diagnostic {
                        level: DiagnosticLevel::Warning,
                        message: format!(
                            "macro `${}` has {} distinct argument shapes, clustered too coarsely to share. Falling back to inline expansion at every call site.",
                            name, info.distinct_shapes
                        ),
                        span: None,
                        notes: vec![],
                        help: None,
                    });
                }
                Recommendation::Share => {}
            }
        }
    }

    // Walk statements looking for macro call sites. The expansion counter
    // is per-call (not global) so snapshot output is deterministic — each
    // `rewrite()` call starts numbering expansions from 1.
    let mut ctx = WalkCtx {
        registry,
        source,
        output: &mut out,
        counter: 0,
        build_mode,
        emitted_runtimes: HashSet::new(),
        megamorph_report: megamorph_report.as_ref(),
    };
    for stmt in &program.body {
        walk_statement(stmt, &mut ctx);
    }

    // Phase 13: second pass — type-position macros. Runs after the
    // value-position walk because both contribute to the same patch
    // list, and type-position patches need to land in the same order
    // as value-position patches before the applicator sorts by span.
    super::type_walker::walk_type_positions(program, source, registry, &mut out);

    out
}

/// First-pass walk for the megamorphism analyzer: collect a
/// [`ResolvedCallSite`] for every call expression that resolves to an
/// `Auto`-mode macro. No patches, no diagnostics — just recording.
fn collect_auto_call_sites(
    program: &Program<'_>,
    registry: &DeclarativeMacroRegistry,
    type_registry: Option<&crate::ts_syn::abi::ir::type_registry::TypeRegistry>,
) -> Vec<ResolvedCallSite> {
    let mut out: Vec<ResolvedCallSite> = Vec::new();
    for stmt in &program.body {
        collect_stmt(stmt, registry, type_registry, &mut out);
    }
    out
}

fn collect_stmt(
    stmt: &Statement<'_>,
    registry: &DeclarativeMacroRegistry,
    type_registry: Option<&crate::ts_syn::abi::ir::type_registry::TypeRegistry>,
    out: &mut Vec<ResolvedCallSite>,
) {
    match stmt {
        Statement::BlockStatement(block) => {
            for s in &block.body {
                collect_stmt(s, registry, type_registry, out);
            }
        }
        Statement::ExpressionStatement(es) => {
            collect_expr(&es.expression, registry, type_registry, out)
        }
        Statement::IfStatement(i) => {
            collect_expr(&i.test, registry, type_registry, out);
            collect_stmt(&i.consequent, registry, type_registry, out);
            if let Some(alt) = &i.alternate {
                collect_stmt(alt, registry, type_registry, out);
            }
        }
        Statement::ReturnStatement(r) => {
            if let Some(arg) = &r.argument {
                collect_expr(arg, registry, type_registry, out);
            }
        }
        Statement::VariableDeclaration(v) => {
            if is_macro_definition_declaration(v) {
                return;
            }
            for decl in &v.declarations {
                if let Some(init) = &decl.init {
                    collect_expr(init, registry, type_registry, out);
                }
            }
        }
        Statement::ExportNamedDeclaration(export) => {
            if let Some(decl) = &export.declaration
                && let oxc::ast::ast::Declaration::VariableDeclaration(v) = decl
            {
                if is_macro_definition_declaration(v) {
                    return;
                }
                for d in &v.declarations {
                    if let Some(init) = &d.init {
                        collect_expr(init, registry, type_registry, out);
                    }
                }
            }
        }
        Statement::ExportDefaultDeclaration(export) => {
            if let Some(expr) = export.declaration.as_expression() {
                collect_expr(expr, registry, type_registry, out);
            }
        }
        Statement::ForStatement(f) => {
            collect_stmt(&f.body, registry, type_registry, out);
        }
        Statement::WhileStatement(w) => collect_stmt(&w.body, registry, type_registry, out),
        Statement::FunctionDeclaration(f) => {
            if let Some(body) = &f.body {
                for s in &body.statements {
                    collect_stmt(s, registry, type_registry, out);
                }
            }
        }
        _ => {}
    }
}

fn collect_expr(
    expr: &Expression<'_>,
    registry: &DeclarativeMacroRegistry,
    type_registry: Option<&crate::ts_syn::abi::ir::type_registry::TypeRegistry>,
    out: &mut Vec<ResolvedCallSite>,
) {
    if let Expression::CallExpression(call) = expr
        && let Expression::Identifier(callee) = &call.callee
        && let Some(name) = callee.name.as_str().strip_prefix('$')
        && let Some(def) = registry.lookup(name)
        && def.mode == MacroMode::Auto
    {
        // Record the site. For multi-arg macros we use the first arg's
        // shape as the representative; a more precise analyzer would
        // combine shapes across args.
        let shape = if let Some(arg) = call.arguments.first() {
            extract_type_shape(arg, type_registry)
        } else {
            super::megamorph::TypeShape::Opaque
        };
        out.push(ResolvedCallSite {
            macro_name: name.to_string(),
            call_span: SpanIR::new(call.span.start + 1, call.span.end + 1),
            arg_shape: shape,
        });
    }
    // Recurse into sub-expressions so we catch nested calls.
    match expr {
        Expression::CallExpression(call) => {
            collect_expr(&call.callee, registry, type_registry, out);
            for arg in &call.arguments {
                if let Some(e) = arg.as_expression() {
                    collect_expr(e, registry, type_registry, out);
                }
            }
        }
        Expression::BinaryExpression(b) => {
            collect_expr(&b.left, registry, type_registry, out);
            collect_expr(&b.right, registry, type_registry, out);
        }
        Expression::LogicalExpression(b) => {
            collect_expr(&b.left, registry, type_registry, out);
            collect_expr(&b.right, registry, type_registry, out);
        }
        Expression::ConditionalExpression(c) => {
            collect_expr(&c.test, registry, type_registry, out);
            collect_expr(&c.consequent, registry, type_registry, out);
            collect_expr(&c.alternate, registry, type_registry, out);
        }
        Expression::ArrayExpression(arr) => {
            for elem in &arr.elements {
                if let Some(e) = elem.as_expression() {
                    collect_expr(e, registry, type_registry, out);
                }
            }
        }
        Expression::ParenthesizedExpression(p) => {
            collect_expr(&p.expression, registry, type_registry, out)
        }
        _ => {}
    }
}

struct WalkCtx<'a> {
    registry: &'a DeclarativeMacroRegistry,
    source: &'a str,
    output: &'a mut RewriteOutput,
    counter: u32,
    build_mode: BuildMode,
    /// Set of macro names whose shared runtime has already been emitted
    /// in this file. Used by `ShareOnly` / `ShareAnyway` / `Auto` to
    /// deduplicate the top-of-file `Patch::Insert` for the helper.
    emitted_runtimes: HashSet<String>,
    /// Optional megamorphism report from the first-pass walk. Populated
    /// only when at least one `Auto`-mode macro is registered and we're
    /// building for prod — see [`rewrite`].
    megamorph_report: Option<&'a MegamorphReport>,
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
        Statement::ExportNamedDeclaration(export) => {
            // `export const x = $macro(...)` wraps the variable declaration
            // in an ExportNamedDeclaration; recurse into the inner decl so
            // the call site gets rewritten.
            if let Some(declaration) = &export.declaration {
                match declaration {
                    oxc::ast::ast::Declaration::VariableDeclaration(v) => {
                        // Re-use the same skip for declarative macro defs
                        // in case someone writes `export const $name = macroRules\`...\``.
                        if is_macro_definition_declaration(v) {
                            return;
                        }
                        for decl in &v.declarations {
                            if let Some(init) = &decl.init {
                                walk_expression(init, ctx, ExpansionContext::Expression);
                            }
                        }
                    }
                    oxc::ast::ast::Declaration::FunctionDeclaration(f) => {
                        if let Some(body) = &f.body {
                            for s in &body.statements {
                                walk_statement(s, ctx);
                            }
                        }
                    }
                    // Class/type/interface/enum exports don't contain
                    // value-position macro calls at the outer level.
                    _ => {}
                }
            }
        }
        Statement::ExportDefaultDeclaration(export) => {
            // `export default $macro(...)` — the declaration is the
            // expression itself.
            if let Some(expr) = export.declaration.as_expression() {
                walk_expression(expr, ctx, ExpansionContext::Expression);
            }
        }
        // Import, interface, type-alias, enum, class declarations don't
        // contain value-position macro calls at the outer level in typical code.
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

/// Decide which arms (`arms` vs `call_arms`) the rewriter should expand
/// for a given macro, based on its mode and the current build mode.
///
/// Returns `(arms_to_use, should_emit_runtime)`. `should_emit_runtime` is
/// `true` when the rewriter should also emit the shared runtime helper
/// once per file on the first call site.
fn resolve_emission_strategy<'a>(
    def: &'a MacroDef,
    build_mode: BuildMode,
    report: Option<&MegamorphReport>,
) -> (&'a [MacroArm], bool) {
    match def.mode {
        MacroMode::ExpandOnly => (def.arms.as_slice(), false),
        MacroMode::ShareOnly | MacroMode::ShareAnyway => {
            // Always share regardless of build mode. Fall back to
            // expand if `call_arms`/`runtime` are missing — validation in
            // discovery prevents that, but we stay defensive.
            if let Some(call_arms) = def.call_arms.as_deref() {
                (call_arms, def.runtime.is_some())
            } else {
                (def.arms.as_slice(), false)
            }
        }
        MacroMode::Auto => match build_mode {
            // Dev: always expand inline so the type checker sees real
            // per-call code, not opaque calls to a helper.
            BuildMode::Dev => (def.arms.as_slice(), false),
            // Prod: consult the megamorphism report.
            //   Share       → emit the shared runtime + call_arms
            //   Cluster     → emit shared runtime + call_arms (cluster
            //                  partitioning is a follow-up; for now
            //                  we emit ONE runtime and let V8 handle it,
            //                  with the warning already logged to the
            //                  user so they know it's not ideal)
            //   ForceExpand → inline expand at every call site
            //   (no report  → fall back to sharing, matching Phase 9b)
            BuildMode::Prod => {
                let decision = report
                    .and_then(|r| r.lookup(&def.name))
                    .map(|info| info.recommendation.clone());
                match decision {
                    Some(Recommendation::Share) | Some(Recommendation::Cluster(_)) | None => {
                        if let Some(call_arms) = def.call_arms.as_deref() {
                            (call_arms, def.runtime.is_some())
                        } else {
                            (def.arms.as_slice(), false)
                        }
                    }
                    Some(Recommendation::ForceExpand) => (def.arms.as_slice(), false),
                }
            }
        },
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

    // Pick which arms to expand based on the macro's mode, the current
    // build mode, and (for Auto in Prod) the megamorphism report. May
    // also signal that we should emit the shared runtime helper once
    // per file.
    let (arms, should_emit_runtime) =
        resolve_emission_strategy(def, ctx.build_mode, ctx.megamorph_report);

    // First call site of a sharing-mode macro → emit the runtime helper.
    if should_emit_runtime
        && !ctx.emitted_runtimes.contains(name)
        && let Some(runtime_src) = def.runtime.as_deref()
    {
        ctx.output.patches.push(Patch::Insert {
            at: SpanIR::new(1, 1),
            code: PatchCode::Text(format!("{}\n", runtime_src.trim())),
            source_macro: Some(format!("${}", name)),
        });
        ctx.emitted_runtimes.insert(name.to_string());
    }

    // Match call args against the selected arm set.
    match match_invocation_against_arms(arms, &call.arguments, ctx.source) {
        Ok((arm_index, bindings)) => {
            let arm = &arms[arm_index];
            match expand_body_with_registry(
                &arm.body,
                &bindings,
                ctx.next_id(),
                context,
                0,
                Some(ctx.registry),
            ) {
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
