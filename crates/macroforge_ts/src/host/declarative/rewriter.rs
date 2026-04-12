//! Walk an OXC program, rewrite calls to registered declarative macros,
//! and emit the patches that strip out the original macroRules definitions.
//!
//! The walker is implemented as two `oxc::ast_visit::Visit` implementors:
//!
//! - [`CollectVisitor`] — first pass (only when at least one `Auto`-mode
//!   macro is registered AND we're building for prod). Records a
//!   [`ResolvedCallSite`] for every `$name(...)` whose callee resolves to
//!   an `Auto` macro, so the megamorph analyzer has real shapes to work
//!   with.
//! - [`RewriteVisitor`] — main pass. A single traversal that handles
//!   BOTH value-position calls (`$macro(x)`) and type-position
//!   references (`$Macro<T>`), because the default OXC walker already
//!   descends into both. This avoids the two-walker duplication that
//!   the hand-rolled MVP had.
//!
//! The expansion context for a rewritten call (Statement vs Expression)
//! is determined at exactly one point: inside [`RewriteVisitor::visit_expression_statement`],
//! where the top-level expression of an `ExpressionStatement` is
//! unwrapped through parens and TS casts to check if it's a direct
//! macro call. Every other macro call is in Expression position by
//! construction.

use std::collections::HashSet;

use oxc::ast::ast::{
    BindingPattern, CallExpression, Decorator, Expression, JSXExpressionContainer, Program,
    PropertyDefinition, TSTypeReference, VariableDeclarationKind, VariableDeclarator,
};
use oxc::ast_visit::{Visit, walk};
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
    // pass entirely — the rewrite pass does all the work in one go.
    let has_auto = registry.iter().any(|(_, def)| def.mode == MacroMode::Auto);
    // Run the megamorph analyzer when we're in prod (the original
    // condition) OR when dev-mode `force_share` is set (PR 17).
    let run_analyzer =
        has_auto && (matches!(build_mode, BuildMode::Prod) || build_mode.force_share());
    let megamorph_report = if run_analyzer {
        // PR 14: emit a one-time `Info` diagnostic when the analyzer
        // runs without a type registry. Structural clustering is
        // downgraded to first-letter bucketing in that case, which
        // a user running in prod may not realize without an
        // explicit notice.
        if type_registry.is_none() {
            out.diagnostics.push(Diagnostic {
                level: DiagnosticLevel::Info,
                message: "declarative macro analyzer running without a type registry; structural clustering disabled, falling back to the name-prefix heuristic. Pass `type_registry_json` in ExpandOptions to enable field-level fingerprinting.".to_string(),
                span: None,
                notes: vec![],
                help: None,
            });
        }
        let mut collector = CollectVisitor {
            registry,
            type_registry,
            sites: Vec::new(),
        };
        collector.visit_program(program);
        Some(megamorph::analyze(registry, &collector.sites, 4))
    } else {
        None
    };

    // Emit non-fatal diagnostics for any Auto macro the analyzer
    // examined. Warnings for megamorphic cases (Cluster / ForceExpand),
    // and — when `analyzer_telemetry` is enabled (PR 14) — an
    // additional `Info` diagnostic for every decision so users can
    // see the full analyzer trace, not just the problem cases.
    if let Some(report) = &megamorph_report {
        let emit_telemetry = build_mode.analyzer_telemetry();
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
            if emit_telemetry {
                // PR 14: always-on telemetry when the user opts in.
                // Summarizes what the analyzer decided regardless of
                // severity.
                let decision = match &info.recommendation {
                    Recommendation::Share => "Share (single helper)".to_string(),
                    Recommendation::Cluster(clusters) => {
                        format!("Cluster (into {} partitions)", clusters.len())
                    }
                    Recommendation::ForceExpand => {
                        "ForceExpand (inline at every call site)".to_string()
                    }
                };
                out.diagnostics.push(Diagnostic {
                    level: DiagnosticLevel::Info,
                    message: format!(
                        "analyzer decision for macro `${}`: {} distinct argument shapes → {}",
                        name, info.distinct_shapes, decision
                    ),
                    span: None,
                    notes: vec![],
                    help: None,
                });
            }
        }
    }

    // Main rewrite pass. The expansion counter is per-call (not global)
    // so snapshot output is deterministic — each `rewrite()` call starts
    // numbering expansions from 1. A single visitor handles both value-
    // and type-position rewriting in one traversal; the default OXC
    // walker descends into TS type annotations, so `visit_ts_type_reference`
    // is reached without a second pass.
    let mut visitor = RewriteVisitor {
        registry,
        source,
        output: &mut out,
        counter: 0,
        build_mode,
        emitted_runtimes: HashSet::new(),
        megamorph_report: megamorph_report.as_ref(),
        type_rewritten: HashSet::new(),
        type_registry,
    };
    visitor.visit_program(program);

    out
}

/// First-pass visitor for the megamorphism analyzer. Runs only when at
/// least one `Auto`-mode macro is registered and `BuildMode::Prod` is
/// active. Records a [`ResolvedCallSite`] for every `$name(...)` whose
/// callee resolves to an `Auto` macro. No patches, no diagnostics.
///
/// Uses OXC's default walker for every node type except
/// [`VariableDeclarator`], which we skip when it's a macro definition
/// (we don't want to record the `$name` call inside `macroRules(...)`
/// as a real call site).
struct CollectVisitor<'a> {
    registry: &'a DeclarativeMacroRegistry,
    type_registry: Option<&'a crate::ts_syn::abi::ir::type_registry::TypeRegistry>,
    sites: Vec<ResolvedCallSite>,
}

impl<'a> Visit<'a> for CollectVisitor<'_> {
    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        if let Expression::Identifier(callee) = &call.callee
            && let Some(name) = callee.name.as_str().strip_prefix('$')
            && let Some(def) = self.registry.lookup_at(name, call.span.start + 1)
            && def.mode == MacroMode::Auto
        {
            // Record the site with one shape entry per positional
            // argument (PR 7 / fix D). Spread arguments fall back to
            // `Opaque`. Two call sites with the same full shape
            // tuple count as one polymorphism class; two call sites
            // that differ in ANY position count as distinct.
            let arg_shapes: Vec<super::megamorph::TypeShape> = call
                .arguments
                .iter()
                .map(|arg| extract_type_shape(arg, self.type_registry))
                .collect();
            self.sites.push(ResolvedCallSite {
                macro_name: name.to_string(),
                call_span: SpanIR::new(call.span.start + 1, call.span.end + 1),
                arg_shapes,
            });
        }
        // Descend into callee + arguments so we catch nested calls.
        walk::walk_call_expression(self, call);
    }

    fn visit_variable_declarator(&mut self, decl: &VariableDeclarator<'a>) {
        if is_macro_definition_declarator(decl) {
            // Don't descend into the macro-definition template literal.
            return;
        }
        walk::walk_variable_declarator(self, decl);
    }
}

/// Main rewrite visitor — one traversal that handles both value-position
/// and type-position macro rewrites. Implements `oxc::ast_visit::Visit`
/// and relies on the generated default walkers for every node it doesn't
/// override explicitly, so new OXC AST nodes are covered automatically
/// as long as the generated walker knows how to descend through them.
///
/// The value-position expansion context (Statement vs Expression) is
/// determined at exactly one entry point:
/// [`RewriteVisitor::visit_expression_statement`]. Every other macro
/// call is in Expression position by construction, because OXC's
/// default walker reaches call expressions only through sub-expression
/// traversal after we've unwrapped the enclosing statement.
pub(super) struct RewriteVisitor<'a> {
    registry: &'a DeclarativeMacroRegistry,
    source: &'a str,
    output: &'a mut RewriteOutput,
    counter: u32,
    build_mode: BuildMode,
    /// Set of (macro-name, cluster-id) pairs whose shared runtime has
    /// already been emitted in this file. Used by `ShareOnly` /
    /// `ShareAnyway` / `Auto` to deduplicate the top-of-file
    /// `Patch::Insert` for the helper. Non-clustered emissions use
    /// an empty string for the cluster component so the key is
    /// `("foo", "")`.
    emitted_runtimes: HashSet<(String, String)>,
    /// Optional megamorphism report from the first-pass walk. Populated
    /// only when at least one `Auto`-mode macro is registered and we're
    /// building for prod — see [`rewrite`].
    megamorph_report: Option<&'a MegamorphReport>,
    /// Deduplication set for type-position rewrites. A `TSTypeReference`
    /// whose span has already been rewritten is skipped so the same
    /// node can't produce two overlapping patches if the traversal
    /// reaches it twice for any reason.
    type_rewritten: HashSet<(u32, u32)>,
    /// Project-wide type registry, when available. Threaded through
    /// so the rewrite-time cluster resolution produces the same
    /// [`super::megamorph::TypeShape`] fingerprints as the
    /// first-pass collector did — without this, fingerprinted
    /// shapes from the collector would never match the rewriter's
    /// `None`-fingerprint lookups and `resolve_cluster_id` would
    /// silently fall through to the defensive single-helper path.
    type_registry: Option<&'a crate::ts_syn::abi::ir::type_registry::TypeRegistry>,
}

impl RewriteVisitor<'_> {
    fn next_id(&mut self) -> u32 {
        self.counter += 1;
        self.counter
    }
}

/// Peel off parenthesized / TS-cast wrappers to reach the "real"
/// expression underneath. Used by [`RewriteVisitor::visit_expression_statement`]
/// so constructs like `($macro(x));` or `$macro(x) as unknown;` still
/// expand in Statement context — the wrappers preserve statement-ness,
/// matching the hand-rolled walker's previous behavior.
fn unwrap_paren_and_casts<'b, 'a>(expr: &'b Expression<'a>) -> &'b Expression<'a> {
    match expr {
        Expression::ParenthesizedExpression(p) => unwrap_paren_and_casts(&p.expression),
        Expression::TSAsExpression(t) => unwrap_paren_and_casts(&t.expression),
        Expression::TSSatisfiesExpression(t) => unwrap_paren_and_casts(&t.expression),
        Expression::TSNonNullExpression(t) => unwrap_paren_and_casts(&t.expression),
        Expression::TSTypeAssertion(t) => unwrap_paren_and_casts(&t.expression),
        other => other,
    }
}

impl<'a> Visit<'a> for RewriteVisitor<'_> {
    fn visit_variable_declarator(&mut self, decl: &VariableDeclarator<'a>) {
        // Skip the declarations of the declarative macros themselves —
        // `rewrite()` already queued `Patch::Delete` for their full span.
        // Descending would re-parse the template-literal body as user
        // code, which is wrong.
        if is_macro_definition_declarator(decl) {
            return;
        }
        walk::walk_variable_declarator(self, decl);
    }

    fn visit_expression_statement(&mut self, es: &oxc::ast::ast::ExpressionStatement<'a>) {
        // If the top-level expression of this statement is (after
        // unwrapping parens / TS casts) a direct macro call, rewrite
        // it in Statement context and DO NOT descend — the
        // replacement text owns the whole span. Any nested macro calls
        // are part of the expanded output now.
        if let Expression::CallExpression(call) = unwrap_paren_and_casts(&es.expression)
            && try_rewrite_call(call, self, ExpansionContext::Statement)
        {
            return;
        }
        // Otherwise let the default walker descend into the expression
        // — any nested macro calls will reach `visit_call_expression`
        // in Expression context.
        walk::walk_expression_statement(self, es);
    }

    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        // Any call reaching this method is not the direct expression of
        // an `ExpressionStatement` (that case is handled above before
        // descent), so the context is always Expression here.
        if try_rewrite_call(call, self, ExpansionContext::Expression) {
            return;
        }
        walk::walk_call_expression(self, call);
    }

    fn visit_ts_type_reference(&mut self, tr: &TSTypeReference<'a>) {
        // Phase 13: type-position rewrite. Delegates to the helper in
        // `type_walker.rs` so the dispatch logic lives beside the
        // type-specific matcher helpers.
        if super::type_walker::try_rewrite_type_ref(tr, self) {
            return;
        }
        walk::walk_ts_type_reference(self, tr);
    }

    // The overrides below are not strictly necessary — the generated
    // default walkers already descend into JSX expression containers,
    // decorators, and class property definitions — but spelling them
    // out makes the "yes, we cover these positions" guarantee explicit
    // to future readers and gives us a hook if we ever need per-node
    // state tracking.

    fn visit_jsx_expression_container(&mut self, node: &JSXExpressionContainer<'a>) {
        walk::walk_jsx_expression_container(self, node);
    }

    fn visit_decorator(&mut self, d: &Decorator<'a>) {
        walk::walk_decorator(self, d);
    }

    fn visit_property_definition(&mut self, p: &PropertyDefinition<'a>) {
        walk::walk_property_definition(self, p);
    }
}

/// Predicate used by both visitors to recognize a `VariableDeclarator`
/// that is the declaration of a declarative macro, i.e. a `const`-kind
/// binding of the form `` const $name = macroRules`...` ``.
fn is_macro_definition_declarator(d: &VariableDeclarator<'_>) -> bool {
    if d.kind != VariableDeclarationKind::Const {
        return false;
    }
    let BindingPattern::BindingIdentifier(bi) = &d.id else {
        return false;
    };
    if !bi.name.as_str().starts_with('$') {
        return false;
    }
    let Some(init) = &d.init else {
        return false;
    };
    match init {
        Expression::TaggedTemplateExpression(tagged) => {
            let Expression::Identifier(id) = &tagged.tag else {
                return false;
            };
            id.name.as_str() == MACRO_RULES_IDENT
        }
        Expression::CallExpression(call) => {
            let Expression::Identifier(id) = &call.callee else {
                return false;
            };
            id.name.as_str() == MACRO_RULES_IDENT
        }
        _ => false,
    }
}

/// Describes how the rewriter should emit code for a single macro
/// definition, based on its mode, the current build mode, and (for
/// `Auto` in prod) the megamorphism report.
enum EmissionPlan<'a> {
    /// Inline expand at every call site using the dev-form arms.
    /// No runtime helper to emit, no cluster discriminator.
    InlineExpand { arms: &'a [MacroArm] },
    /// Emit a single shared runtime helper per file and replace each
    /// call site with the `call_arms` expansion. Used by
    /// `ShareOnly` / `ShareAnyway` (unconditionally) and by `Auto`
    /// in prod when the analyzer recommends `Share`.
    ShareSingle { arms: &'a [MacroArm] },
    /// Emit one shared runtime helper per *cluster*, each with a
    /// cluster-specialized name derived from `runtime_name_template`
    /// (or an auto-suffix fallback). Each call site dispatches to its
    /// own cluster's helper via a `$__cluster__` substitution in the
    /// `call_arms` body and, for the runtime body itself, via a
    /// plain text replace of `$__cluster__`.
    ShareClustered {
        arms: &'a [MacroArm],
        clusters: &'a [super::megamorph::TypeCluster],
    },
}

/// Decide which arms (`arms` vs `call_arms`) the rewriter should expand
/// for a given macro, based on its mode and the current build mode.
fn resolve_emission_strategy<'a>(
    def: &'a MacroDef,
    build_mode: BuildMode,
    report: Option<&'a MegamorphReport>,
) -> EmissionPlan<'a> {
    match def.mode {
        MacroMode::ExpandOnly => EmissionPlan::InlineExpand {
            arms: def.arms.as_slice(),
        },
        MacroMode::ShareOnly | MacroMode::ShareAnyway => {
            // Always share regardless of build mode. Fall back to
            // inline expand if `call_arms`/`runtime` are missing —
            // validation in discovery prevents that, but we stay
            // defensive.
            match def.call_arms.as_deref() {
                Some(call_arms) if def.runtime.is_some() => {
                    EmissionPlan::ShareSingle { arms: call_arms }
                }
                _ => EmissionPlan::InlineExpand {
                    arms: def.arms.as_slice(),
                },
            }
        }
        MacroMode::Auto => {
            // PR 17: dev builds can opt in to the share-mode path
            // via `BuildMode::Dev { force_share: true }`. In that
            // case we treat Auto like Prod for emission-strategy
            // purposes so share-mode bugs surface at dev time
            // rather than only in prod. Without the flag, dev
            // keeps the old "always inline" behaviour.
            let is_share_path = matches!(build_mode, BuildMode::Prod) || build_mode.force_share();
            if !is_share_path {
                return EmissionPlan::InlineExpand {
                    arms: def.arms.as_slice(),
                };
            }
            // Share path — consult the megamorphism report.
            //   Share       → single shared runtime + call_arms.
            //   Cluster     → one shared runtime per cluster +
            //                  call_arms, each carrying the cluster
            //                  id via the `$__cluster__` substitution.
            //   ForceExpand → inline expand at every call site.
            //   (no report) → fall back to single sharing, matching
            //                  the Phase 9b behaviour.
            let info = report.and_then(|r| r.lookup(&def.name));
            match info.map(|i| &i.recommendation) {
                Some(Recommendation::Cluster(clusters)) => match def.call_arms.as_deref() {
                    Some(call_arms) if def.runtime.is_some() => EmissionPlan::ShareClustered {
                        arms: call_arms,
                        clusters: clusters.as_slice(),
                    },
                    _ => EmissionPlan::InlineExpand {
                        arms: def.arms.as_slice(),
                    },
                },
                Some(Recommendation::Share) | None => match def.call_arms.as_deref() {
                    Some(call_arms) if def.runtime.is_some() => {
                        EmissionPlan::ShareSingle { arms: call_arms }
                    }
                    _ => EmissionPlan::InlineExpand {
                        arms: def.arms.as_slice(),
                    },
                },
                Some(Recommendation::ForceExpand) => EmissionPlan::InlineExpand {
                    arms: def.arms.as_slice(),
                },
            }
        }
    }
}

/// Find the cluster whose member shape tuples contain the given
/// call site's `arg_shapes`, and return the cluster's id. If no
/// cluster matches (e.g. the analyzer's report is out-of-date
/// because the first pass and the rewrite pass walked slightly
/// different fragment sets), return `None` — the caller falls back
/// to single-helper behavior with a defensive diagnostic.
fn resolve_cluster_id<'a>(
    clusters: &'a [super::megamorph::TypeCluster],
    arg_shapes: &[super::megamorph::TypeShape],
) -> Option<&'a str> {
    for cluster in clusters {
        if cluster
            .shapes
            .iter()
            .any(|tuple| tuple.as_slice() == arg_shapes)
        {
            return Some(&cluster.id);
        }
    }
    None
}

/// Compute the cluster-specialized helper name for a macro's runtime
/// given the user-supplied `runtime_name_template` (if any) and the
/// cluster id. When no template is provided, append `__{id}` to the
/// bare helper name parsed out of `runtime_src` (the first `function
/// NAME(` occurrence). When even that parse fails, return `None` so
/// the caller can bail gracefully.
fn specialize_helper_name(
    runtime_name_template: Option<&str>,
    runtime_src: &str,
    cluster_id: &str,
) -> Option<String> {
    if let Some(template) = runtime_name_template {
        return Some(template.replace("$__cluster__", cluster_id));
    }
    // Fallback: scan `runtime_src` for `function NAME(` and append
    // `__{id}` to NAME.
    let bytes = runtime_src.as_bytes();
    let needle = b"function ";
    let mut i = 0;
    while i + needle.len() < bytes.len() {
        if &bytes[i..i + needle.len()] == needle {
            let mut j = i + needle.len();
            while j < bytes.len() && matches!(bytes[j], b' ' | b'\t') {
                j += 1;
            }
            let start = j;
            while j < bytes.len()
                && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_' || bytes[j] == b'$')
            {
                j += 1;
            }
            if j > start {
                let base = std::str::from_utf8(&bytes[start..j]).ok()?;
                return Some(format!("{}__{}", base, cluster_id));
            }
        }
        i += 1;
    }
    None
}

/// If `call` is a `$name(...)` invocation of a registered macro, emit a
/// `Patch::Replace` for the call site and return `true`. Returns `false`
/// if the callee isn't a registered macro; the caller will then descend
/// into the call's arguments normally.
///
/// Lives at the module level (not as a method on [`RewriteVisitor`]) so
/// it's accessible from the type-position walker helper module too — the
/// same shared state carries both value-side and type-side rewrite
/// bookkeeping.
pub(super) fn try_rewrite_call(
    call: &oxc::ast::ast::CallExpression<'_>,
    visitor: &mut RewriteVisitor<'_>,
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
    // PR 11: look up the macro at the call site's 1-based byte
    // position so nested declarations shadow outer ones. A top-
    // level `const $foo` is visible from anywhere, but a nested
    // `const $foo` inside a function body shadows the outer one
    // at call sites within the same function.
    let call_pos = call.span.start + 1;
    let Some(def_arc) = visitor.registry.lookup_at(name, call_pos) else {
        return false;
    };
    let def = def_arc.as_ref();

    // Pick which arms to expand based on the macro's mode, the current
    // build mode, and (for Auto in Prod) the megamorphism report.
    let plan = resolve_emission_strategy(def, visitor.build_mode, visitor.megamorph_report);

    // Resolve the per-call cluster id (if any) and the arms to use.
    // `cluster_id == ""` means "no clustering in play"; `Some(id)`
    // marks the call for per-cluster runtime specialization via the
    // `$__cluster__` substitution.
    let (arms, cluster_id): (&[MacroArm], Option<String>) = match &plan {
        EmissionPlan::InlineExpand { arms } => (*arms, None),
        EmissionPlan::ShareSingle { arms } => (*arms, Some(String::new())),
        EmissionPlan::ShareClustered { arms, clusters } => {
            // Compute this call site's full shape tuple the same way
            // the analyzer's first pass did — one shape per positional
            // argument, in left-to-right order (PR 7 / fix D), AND
            // threading the project-wide type registry through so
            // fingerprinted shapes compare equal to what the collector
            // built. Without the registry the rewriter would produce
            // `Named { fields: None }` shapes that never match the
            // collector's `Named { fields: Some(_) }` cluster members.
            let arg_shapes: Vec<super::megamorph::TypeShape> = call
                .arguments
                .iter()
                .map(|arg| extract_type_shape(arg, visitor.type_registry))
                .collect();
            let resolved = resolve_cluster_id(clusters, &arg_shapes).map(|s| s.to_string());
            if resolved.is_none() {
                // The analyzer didn't place this call's shape in any
                // cluster — defensive fallback to the single-helper
                // path. This can happen if the first-pass walker
                // and the rewrite pass see different fragment sets,
                // or if the user's code changed between analysis and
                // rewrite (which is rare but possible with fast
                // incremental rebuilds).
                visitor.output.diagnostics.push(Diagnostic {
                    level: DiagnosticLevel::Warning,
                    message: format!(
                        "macro `${}` call site's argument shape did not match any cluster; falling back to a single shared helper. This is usually a sign of stale analysis data.",
                        name
                    ),
                    span: Some(SpanIR::new(call.span.start + 1, call.span.end + 1)),
                    notes: vec![],
                    help: None,
                });
                (*arms, Some(String::new()))
            } else {
                (*arms, resolved)
            }
        }
    };

    // First call site of a sharing-mode macro for this (name,
    // cluster) pair → emit the runtime helper. Non-clustered
    // emissions use an empty cluster-id key so a `ShareSingle` macro
    // emits exactly one helper per file.
    if let Some(cluster_id_str) = cluster_id.as_deref()
        && let Some(runtime_src) = def.runtime.as_deref()
        && !matches!(plan, EmissionPlan::InlineExpand { .. })
    {
        let dedup_key = (name.to_string(), cluster_id_str.to_string());
        if !visitor.emitted_runtimes.contains(&dedup_key) {
            // Substitute `$__cluster__` in the runtime source text.
            // When we're in the non-clustered path (cluster_id_str ==
            // ""), the substitution is a no-op unless the user wrote
            // `$__cluster__` themselves — which is allowed and
            // produces empty-string substitution (harmless but odd).
            //
            // For the clustered path we also compute a specialized
            // helper name via `runtime_name_template` (or the auto-
            // suffix fallback) so the emitted `function __helper_a(`
            // declaration actually differs per cluster.
            let runtime_emit = if cluster_id_str.is_empty() {
                runtime_src.to_string()
            } else {
                let specialized = specialize_helper_name(
                    def.runtime_name_template.as_deref(),
                    runtime_src,
                    cluster_id_str,
                );
                if let (Some(template), Some(specialized_name)) =
                    (def.runtime_name_template.as_deref(), specialized.as_deref())
                {
                    // User gave us a name template. Replace `$__cluster__`
                    // in the runtime body so the `function NAME(`
                    // declaration matches the template's output.
                    let base = template.replace("$__cluster__", "");
                    let base_trimmed = base.trim_matches('_');
                    if !base_trimmed.is_empty() {
                        runtime_src
                            .replace(base_trimmed, specialized_name)
                            .replace("$__cluster__", cluster_id_str)
                    } else {
                        runtime_src.replace("$__cluster__", cluster_id_str)
                    }
                } else {
                    runtime_src.replace("$__cluster__", cluster_id_str)
                }
            };
            visitor.output.patches.push(Patch::Insert {
                at: SpanIR::new(1, 1),
                code: PatchCode::Text(format!("{}\n", runtime_emit.trim())),
                // PR 14: attribution includes the cluster id when
                // the emission is clustered, so error blame and
                // source-map consumers can distinguish between
                // variants. Format is `$name` for non-clustered
                // emissions and `$name@cluster_id` for clustered
                // ones.
                source_macro: Some(format_attribution(name, cluster_id_str)),
            });
            visitor.emitted_runtimes.insert(dedup_key);
        }
    }

    // The expander wants an `Option<&str>` for its cluster parameter.
    // `Some("")` means "clustered path but no discriminator" which
    // would incorrectly inject an empty-string substitution. Convert
    // an empty cluster id to `None` so the expander skips the
    // synthetic binding.
    let expander_cluster_id: Option<&str> = cluster_id
        .as_deref()
        .and_then(|s| if s.is_empty() { None } else { Some(s) });

    // Match call args against the selected arm set.
    match match_invocation_against_arms(arms, &call.arguments, visitor.source) {
        Ok((arm_index, bindings)) => {
            let arm = &arms[arm_index];
            let expansion_id = visitor.next_id();
            match expand_body_with_registry(
                &arm.body,
                &bindings,
                expansion_id,
                context,
                0,
                Some(visitor.registry),
                expander_cluster_id,
            ) {
                Ok(expanded) => {
                    let span = call.span;
                    let span_ir = SpanIR::new(span.start + 1, span.end + 1);
                    let cluster_attr = cluster_id.as_deref().unwrap_or("");
                    visitor.output.patches.push(Patch::Replace {
                        span: span_ir,
                        code: PatchCode::Text(expanded),
                        // PR 14: per-call-site attribution carries
                        // the cluster id so error blame
                        // disambiguates between variants of the
                        // same macro.
                        source_macro: Some(format_attribution(name, cluster_attr)),
                    });
                    true
                }
                Err(e) => {
                    let span = call.span();
                    visitor.output.diagnostics.push(Diagnostic {
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
            visitor.output.diagnostics.push(Diagnostic {
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

/// Format the `source_macro` attribution string embedded in every
/// patch that PR 14 emits. Non-clustered emissions use the plain
/// `$name` form; clustered emissions append `@cluster_id` so
/// diagnostic blame distinguishes between per-cluster helper
/// variants.
///
/// Used by both [`try_rewrite_call`] (in this module) and
/// [`super::type_walker::try_rewrite_type_ref`] via
/// [`RewriteVisitor::format_attribution`] — the latter delegates
/// here so the type-walker's attribution format stays in sync with
/// the value-walker's.
pub(super) fn format_attribution(name: &str, cluster_id: &str) -> String {
    if cluster_id.is_empty() {
        format!("${}", name)
    } else {
        format!("${}@{}", name, cluster_id)
    }
}

// ---------------------------------------------------------------------------
// Cross-module accessors for the type-position walker.
// ---------------------------------------------------------------------------
//
// `type_walker::try_rewrite_type_ref` needs to append patches and
// diagnostics to the visitor's shared output, dedupe against
// `type_rewritten`, mint fresh expansion ids, and read `source` and
// `registry`. Exposing a handful of focused accessors keeps the
// visitor's fields private to this module while still giving the
// sibling module everything it needs — no blanket `pub(super)` on
// every field.

impl<'a> RewriteVisitor<'a> {
    pub(super) fn registry(&self) -> &'a DeclarativeMacroRegistry {
        self.registry
    }

    pub(super) fn source(&self) -> &'a str {
        self.source
    }

    pub(super) fn output_mut(&mut self) -> &mut RewriteOutput {
        self.output
    }

    pub(super) fn next_expansion_id(&mut self) -> u32 {
        self.next_id()
    }

    /// Returns `true` if the given `(start, end)` span has not been
    /// rewritten yet and records it as rewritten. Returns `false` if
    /// the span was already recorded — the caller should skip it.
    pub(super) fn record_type_rewrite(&mut self, start: u32, end: u32) -> bool {
        self.type_rewritten.insert((start, end))
    }
}
