//! `@nonExhaustive` — intersect a type alias's RHS with a brand sentinel so
//! external matches are forced to carry a default arm.
//!
//! Given:
//! ```ts
//! /** @nonExhaustive */
//! export type Kind = 'a' | 'b' | 'c';
//! ```
//!
//! …the pass rewrites the RHS to
//! `('a' | 'b' | 'c') & { readonly __nonExhaustive: unique symbol }`
//! and strips the annotation JSDoc line.
//!
//! The brand property name is configurable via
//! [`NonExhaustiveConfig::brand`](macroforge_ts_syn::config::NonExhaustiveConfig).

use macroforge_ts_syn::config::NonExhaustiveConfig;

use super::discovery::{AttributeAnnotation, AttributeKind, DeclKind};
use crate::ts_syn::abi::{Diagnostic, DiagnosticLevel, Patch, PatchCode};

pub fn apply(
    annotations: &[&AttributeAnnotation],
    source: &str,
    config: &NonExhaustiveConfig,
) -> (Vec<Patch>, Vec<Diagnostic>) {
    let mut patches = Vec::new();
    let mut diagnostics = Vec::new();

    for ann in annotations {
        if ann.kind != AttributeKind::NonExhaustive {
            continue;
        }
        if ann.decl_kind != DeclKind::TypeAlias {
            diagnostics.push(Diagnostic {
                level: DiagnosticLevel::Error,
                message: "[@nonExhaustive] can only be applied to type aliases".into(),
                span: Some(ann.jsdoc_span),
                notes: Vec::new(),
                help: Some(
                    "Only `type X = ...` declarations support @nonExhaustive; apply it there."
                        .into(),
                ),
            });
            continue;
        }
        let Some(rhs_span) = ann.type_alias_rhs_span else {
            continue;
        };

        // Fetch the original RHS text by translating 1-based SpanIR back to 0-based slice bounds.
        let start = rhs_span.start.saturating_sub(1) as usize;
        let end = rhs_span.end.saturating_sub(1) as usize;
        let original_rhs = &source[start..end];
        let replacement = format!(
            "({original}) & {{ readonly {brand}: unique symbol }}",
            original = original_rhs,
            brand = config.brand,
        );

        // Replace just the RHS span, then strip the annotation JSDoc.
        patches.push(Patch::Replace {
            span: rhs_span,
            code: PatchCode::Text(replacement),
            source_macro: Some("nonExhaustive".into()),
        });
        patches.push(Patch::Delete {
            span: ann.jsdoc_span,
        });
    }

    (patches, diagnostics)
}
