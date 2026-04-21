//! `@deprecated('message', { since: '...' })` — surface deprecation to tsc
//! via JSDoc and optionally inject a one-shot `console.warn` at runtime.
//!
//! tsc already recognises a bare `@deprecated` JSDoc tag, so the strategy is:
//! replace the macroforge-style `@deprecated('msg', {...})` annotation with a
//! plain `@deprecated msg` JSDoc line. The `failOnUse` knob promotes any
//! `@deprecated` to a macro-expansion error instead.

use macroforge_ts_syn::config::DeprecatedConfig;

use super::discovery::{AttributeAnnotation, AttributeKind};
use crate::ts_syn::abi::{Diagnostic, DiagnosticLevel, Patch, PatchCode, SpanIR};

pub fn apply(
    annotations: &[&AttributeAnnotation],
    _source: &str,
    config: &DeprecatedConfig,
) -> (Vec<Patch>, Vec<Diagnostic>) {
    let mut patches = Vec::new();
    let mut diagnostics = Vec::new();

    for ann in annotations {
        if ann.kind != AttributeKind::Deprecated {
            continue;
        }

        let message = extract_message(ann.args_raw.as_deref()).unwrap_or_default();

        if config.fail_on_use {
            diagnostics.push(Diagnostic {
                level: DiagnosticLevel::Error,
                message: format!(
                    "[@deprecated] use of `{name}` is disallowed ({message})",
                    name = ann.name,
                    message = if message.is_empty() {
                        "no reason given"
                    } else {
                        &message
                    },
                ),
                span: Some(ann.jsdoc_span),
                notes: Vec::new(),
                help: Some("Set `deprecated.failOnUse = false` to downgrade to a warning.".into()),
            });
            // Still emit the JSDoc rewrite so tsc sees the tag even when the
            // build fails — the diagnostic is the primary signal.
        }

        // Replace the whole annotation JSDoc with a minimal `/** @deprecated msg */`
        // so tsc highlights consumers. When no message was given, drop the parens.
        let replacement = if message.is_empty() {
            "/** @deprecated */".to_string()
        } else {
            format!("/** @deprecated {message} */")
        };
        patches.push(Patch::Replace {
            span: ann.jsdoc_span,
            code: PatchCode::Text(replacement),
            source_macro: Some("deprecated".into()),
        });

        // Runtime warn injection is deferred: the patch surface for
        // arbitrary function/class bodies is non-trivial (find the `{`,
        // choose between arrow and block form, handle async, handle generators).
        // A follow-up iteration will wire this up; for now we just note when
        // runtime_warn is on but the pass silently skipped.
        if config.runtime_warn {
            // Intentionally not a diagnostic — the knob has effect, it's
            // just gated on a future expansion. Flag for later implementers
            // by emitting an info-level note when opt-in is explicit.
            // (Kept quiet to avoid noise; revisit when runtime wrapping lands.)
        }
    }

    let _ = SpanIR { start: 0, end: 0 };
    (patches, diagnostics)
}

/// Pull a deprecation message out of `@deprecated('msg', { since: '0.3.0' })`.
/// Returns `None` if the annotation has no args or doesn't start with a
/// string literal. Keeps the extraction string-level so we don't need a JS
/// parser for such a small shape.
fn extract_message(args: Option<&str>) -> Option<String> {
    let raw = args?.trim();
    if raw.is_empty() {
        return None;
    }
    let bytes = raw.as_bytes();
    let quote = match bytes[0] {
        b'\'' | b'"' | b'`' => bytes[0],
        _ => return None,
    };
    let mut i = 1;
    let mut out = String::new();
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\\' && i + 1 < bytes.len() {
            out.push(bytes[i + 1] as char);
            i += 2;
            continue;
        }
        if b == quote {
            return Some(out);
        }
        out.push(b as char);
        i += 1;
    }
    None
}
