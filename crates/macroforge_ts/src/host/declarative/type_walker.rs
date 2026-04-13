//! Phase 13 — type-position macro helper.
//!
//! Helper function that rewrites a single `TSTypeReference` node when
//! it resolves to a declarative macro with `kind: "type"`. After the
//! B-phase walker migration, the traversal itself is owned by
//! [`super::rewriter::RewriteVisitor`] — it calls
//! [`try_rewrite_type_ref`] from its `visit_ts_type_reference`
//! override. This module keeps the type-specific matcher invocation
//! and diagnostic shaping so the rewriter stays focused on its value-
//! position logic.

use oxc::ast::ast::TSTypeName;

use crate::ts_syn::abi::{Diagnostic, DiagnosticLevel, Patch, PatchCode, SpanIR};
use crate::ts_syn::declarative::MacroKind;

use super::expander::{ExpansionContext, expand_body_with_registry};
use super::matcher::{MatchError, match_type_invocation_against_arms};
use super::rewriter::RewriteVisitor;

/// Attempt to rewrite a `TSTypeReference` node as a type-position
/// macro invocation. Returns `true` if the reference was rewritten
/// (the caller should then skip its children) — i.e. the caller is
/// [`super::rewriter::RewriteVisitor::visit_ts_type_reference`], which
/// on `false` falls through to the default walker to descend into
/// type arguments and pick up nested type macros.
///
/// All shared mutable state lives on the visitor: the patch list, the
/// diagnostic list, the expansion counter, and the dedup set of
/// already-rewritten spans.
pub(super) fn try_rewrite_type_ref(
    tr: &oxc::ast::ast::TSTypeReference<'_>,
    visitor: &mut RewriteVisitor<'_>,
) -> bool {
    // Resolve `type_name` to a bare `$identifier`. Qualified names
    // (`a.b.c`) can't declare macros in MVP.
    let TSTypeName::IdentifierReference(ident) = &tr.type_name else {
        return false;
    };
    let Some(macro_name) = ident.name.as_str().strip_prefix('$') else {
        return false;
    };
    // PR 11: scoped lookup so nested type-macro declarations
    // shadow outer ones at their use sites.
    let Some(def) = visitor.registry().lookup_at(macro_name, tr.span.start + 1) else {
        // Not a declarative macro — try proc macro dispatch for type position.
        return try_dispatch_proc_type_ref(tr, macro_name, visitor);
    };
    if def.kind != MacroKind::Type {
        // The macro exists but it's value-position only. Emit a hard
        // error so the user knows they're using it wrong.
        visitor.output_mut().diagnostics.push(Diagnostic {
            level: DiagnosticLevel::Error,
            message: format!(
                "macro `${}` is value-only; cannot use it in type position",
                macro_name
            ),
            span: Some(SpanIR::new(tr.span.start + 1, tr.span.end + 1)),
            notes: vec![],
            help: None,
        });
        return false;
    }

    // Dedupe: if we've already rewritten this exact span, skip.
    if !visitor.record_type_rewrite(tr.span.start, tr.span.end) {
        return false;
    }

    // Clone the Arc so we don't hold the registry borrow while we
    // later take a `&mut` borrow on the visitor to push patches.
    let def = def.clone();
    let source = visitor.source();

    // Extract the type parameters. `$Foo<A, B>` → the OXC Vec of A, B;
    // `$Foo` with no params uses the empty-pattern fast path since we
    // can't easily construct an OXC Vec outside its allocator.
    let result = match tr.type_arguments.as_ref() {
        Some(tp) => match_type_invocation_against_arms(&def.arms, &tp.params, source),
        None => match_type_invocation_empty(&def.arms),
    };

    match result {
        Ok((arm_index, bindings)) => {
            let arm = &def.arms[arm_index];
            // Type-position expansions use the dedicated `Type`
            // context so the expander doesn't apply the JS-level
            // IIFE wrap that expressions need for block bodies.
            let expansion_id = visitor.next_expansion_id();
            let registry = visitor.registry();
            match expand_body_with_registry(
                &arm.body,
                &bindings,
                expansion_id,
                ExpansionContext::Type,
                0,
                Some(registry),
                // Type-position macros don't participate in
                // runtime-sharing, so there's no cluster to thread.
                None,
            ) {
                Ok(expanded) => {
                    visitor.output_mut().patches.push(Patch::Replace {
                        span: SpanIR::new(tr.span.start + 1, tr.span.end + 1),
                        code: PatchCode::Text(expanded),
                        // Type-position macros never cluster (sharing
                        // modes are rejected for them), so the cluster
                        // component is always empty. Use the helper
                        // anyway to keep attribution formatting in
                        // a single place.
                        source_macro: Some(super::rewriter::format_attribution(macro_name, "")),
                    });
                    true
                }
                Err(e) => {
                    visitor.output_mut().diagnostics.push(Diagnostic {
                        level: DiagnosticLevel::Error,
                        message: format!(
                            "error expanding type-position macro `${}`: {}",
                            macro_name, e
                        ),
                        span: Some(SpanIR::new(tr.span.start + 1, tr.span.end + 1)),
                        notes: vec![],
                        help: None,
                    });
                    false
                }
            }
        }
        Err(MatchError::NoArmMatched { tried }) => {
            visitor.output_mut().diagnostics.push(Diagnostic {
                level: DiagnosticLevel::Error,
                message: format!(
                    "no arm of type-position macro `${}` matched its invocation; tried {} arm(s): {}",
                    macro_name,
                    tried.len(),
                    tried.join(" | ")
                ),
                span: Some(SpanIR::new(tr.span.start + 1, tr.span.end + 1)),
                notes: vec![],
                help: None,
            });
            false
        }
        Err(err) => {
            visitor.output_mut().diagnostics.push(Diagnostic {
                level: DiagnosticLevel::Error,
                message: format!(
                    "type-position macro `${}` match failed: {}",
                    macro_name, err
                ),
                span: Some(SpanIR::new(tr.span.start + 1, tr.span.end + 1)),
                notes: vec![],
                help: None,
            });
            false
        }
    }
}

/// Handle the zero-type-parameters case without constructing a fake
/// OXC `Vec`. Only arms whose pattern is `Empty` can match.
fn match_type_invocation_empty(
    arms: &[crate::ts_syn::declarative::MacroArm],
) -> Result<
    (
        usize,
        std::collections::HashMap<String, super::matcher::Binding>,
    ),
    MatchError,
> {
    use crate::ts_syn::declarative::Pattern;
    for (arm_index, arm) in arms.iter().enumerate() {
        if matches!(arm.pattern, Pattern::Empty) {
            return Ok((arm_index, std::collections::HashMap::new()));
        }
    }
    Err(MatchError::NoArmMatched {
        tried: vec!["()".to_string()],
    })
}

/// Attempt to dispatch a `$Name<T1, T2>` type reference as a function-like
/// proc macro in type position. Called when the declarative registry lookup
/// fails.
fn try_dispatch_proc_type_ref(
    tr: &oxc::ast::ast::TSTypeReference<'_>,
    macro_name: &str,
    visitor: &mut super::rewriter::RewriteVisitor<'_>,
) -> bool {
    use oxc::span::GetSpan;

    let dispatcher = match visitor.proc_dispatcher {
        Some(d) => d,
        None => return false,
    };

    let callee_with_dollar = format!("${}", macro_name);

    // Resolve module path.
    let module_path = if let Some(sources) = visitor.import_sources
        && let Some(mp) = sources.get(&callee_with_dollar)
    {
        mp.clone()
    } else if let Some(desc) = crate::host::derived::lookup_by_name(macro_name)
        && desc.kind == crate::ts_syn::abi::MacroKind::Call
    {
        "@macro/derive".to_string()
    } else {
        return false;
    };

    // Extract the type arguments as source text.
    let args_source = tr
        .type_arguments
        .as_ref()
        .map(|tp| {
            let sp = tp.span();
            let raw = &visitor.source()[sp.start as usize..sp.end as usize];
            // Strip < and >
            raw.strip_prefix('<')
                .and_then(|s| s.strip_suffix('>'))
                .unwrap_or(raw)
                .to_string()
        })
        .unwrap_or_default();

    let type_span = SpanIR::new(tr.span.start + 1, tr.span.end + 1);

    let ctx = crate::ts_syn::abi::MacroContextIR {
        abi_version: 1,
        macro_kind: crate::ts_syn::abi::MacroKind::Call,
        macro_name: macro_name.to_string(),
        module_path,
        decorator_span: type_span,
        macro_name_span: None,
        target_span: type_span,
        file_name: String::new(),
        target: crate::ts_syn::abi::TargetIR::Other,
        target_source: args_source,
        import_registry: crate::ts_syn::ImportRegistry::new(),
        config: None,
        type_registry: None,
        resolved_fields: None,
    };

    let mut result = dispatcher.dispatch(ctx.clone());

    let is_not_found = result.diagnostics.iter().any(|d| {
        d.message.contains("Macro")
            && (d.message.contains("not found") || d.message.contains("is not a Macroforge"))
    });
    if is_not_found {
        if let Some(loader) = visitor.external_loader {
            match loader.run_macro(&ctx) {
                Ok(external_result) => result = external_result,
                Err(_) => return false,
            }
        } else {
            return false;
        }
    }

    if let Some(tokens) = &result.tokens {
        visitor.output_mut().patches.push(Patch::Replace {
            span: type_span,
            code: PatchCode::Text(tokens.clone()),
            source_macro: Some(format!("${}", macro_name)),
        });
    }

    for patch in result.runtime_patches {
        visitor.output_mut().patches.push(patch);
    }
    for diag in result.diagnostics {
        visitor.output_mut().diagnostics.push(diag);
    }

    true
}
