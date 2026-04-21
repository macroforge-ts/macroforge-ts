//! # Attribute macros
//!
//! A single pre-pass that implements four Rust-inspired attribute macros
//! (`@cfg`, `@deprecated`, `@mustUse`, `@nonExhaustive`) driven by keys in
//! `macroforge.config.*`. The pass runs before `@buildtime` so `@cfg`-stripped
//! code doesn't waste evaluator cycles, and before declarative/derive macros
//! so stripped code never participates in expansion.
//!
//! ## High-level flow
//!
//! ```text
//! source.ts
//!     │
//!     ▼
//! discovery::discover       ← walk OXC AST for JSDoc @cfg/@deprecated/@mustUse/@nonExhaustive
//!     │
//!     ▼
//! cfg::apply               ← evaluate predicates, Patch::Delete on mismatch
//! non_exhaustive::apply    ← intersect RHS with brand sentinel
//! deprecated::apply        ← inject tsc-readable JSDoc + optional runtime warn
//! must_use::apply          ← walk call sites, emit diagnostics on discarded returns
//!     │
//!     ▼
//! Vec<Patch> + Vec<Diagnostic>
//!     │
//!     ▼
//! PatchApplicator (host)
//! ```
//!
//! Each submodule produces zero or more patches and diagnostics. The parent
//! `run_prepass` merges them and hands the single [`AttributePrepassOutput`]
//! back to the expansion pipeline. The design mirrors
//! [`crate::host::buildtime`] intentionally so changes there translate.

pub mod cfg;
pub mod deprecated;
pub mod discovery;
pub mod must_use;
pub mod non_exhaustive;

#[cfg(test)]
mod tests;

use std::path::PathBuf;

use oxc::ast::ast::Program;

use crate::ts_syn::abi::{Diagnostic, DiagnosticLevel, Patch};
use macroforge_ts_syn::config::MacroforgeConfig;

/// Output of the attribute pre-pass.
#[derive(Debug, Clone, Default)]
pub struct AttributePrepassOutput {
    /// Rewritten source, or `None` if no annotation produced a patch.
    pub rewritten: Option<String>,
    /// Diagnostics surfaced to the user. `@mustUse` errors, config-driven
    /// `failOnUse` errors from `@deprecated`, and predicate errors from
    /// `@cfg` all land here.
    pub diagnostics: Vec<Diagnostic>,
}

impl AttributePrepassOutput {
    pub fn is_identity(&self) -> bool {
        self.rewritten.is_none() && self.diagnostics.is_empty()
    }
}

/// Run the attribute pre-pass against `source`.
///
/// * `program` — parsed OXC AST.
/// * `source` — the original text (used for span-to-text lookups and JSDoc
///   parsing).
/// * `_origin_path` — currently unused but plumbed through so diagnostics
///   can grow a path field later without breaking the signature.
/// * `config` — resolved `MacroforgeConfig`. Determines what `@cfg`
///   predicates pass, what `@deprecated` emits, and what brand
///   `@nonExhaustive` injects.
pub fn run_prepass(
    program: &Program<'_>,
    source: &str,
    _origin_path: &std::path::Path,
    config: &MacroforgeConfig,
) -> AttributePrepassOutput {
    let discovered = discovery::discover(program, source);
    if discovered.is_empty() {
        return AttributePrepassOutput::default();
    }

    let mut patches: Vec<Patch> = Vec::new();
    let mut diagnostics: Vec<Diagnostic> = Vec::new();

    // @cfg runs first: dropping declarations means later passes don't see
    // them at all. Every other attribute then runs only against survivors.
    let (cfg_patches, cfg_diagnostics, dropped_spans) = cfg::apply(&discovered, &config.cfg);
    patches.extend(cfg_patches);
    diagnostics.extend(cfg_diagnostics);

    // Remaining passes skip annotations whose owning declaration was stripped.
    let survivors: Vec<&discovery::AttributeAnnotation> = discovered
        .iter()
        .filter(|ann| !dropped_spans.contains(&ann.owner_span()))
        .collect();

    let (ne_patches, ne_diagnostics) =
        non_exhaustive::apply(&survivors, source, &config.non_exhaustive);
    patches.extend(ne_patches);
    diagnostics.extend(ne_diagnostics);

    let (dep_patches, dep_diagnostics) = deprecated::apply(&survivors, source, &config.deprecated);
    patches.extend(dep_patches);
    diagnostics.extend(dep_diagnostics);

    let (mu_patches, mu_diagnostics) =
        must_use::apply(program, &survivors, source, &config.must_use);
    patches.extend(mu_patches);
    diagnostics.extend(mu_diagnostics);

    let rewritten = if patches.is_empty() {
        None
    } else {
        let applicator = crate::host::patch_applicator::PatchApplicator::new(source, patches);
        match applicator.apply() {
            Ok(rewritten) => Some(rewritten),
            Err(error) => {
                diagnostics.push(Diagnostic {
                    level: DiagnosticLevel::Error,
                    message: format!("[attributes] patch apply failed: {error}"),
                    span: None,
                    notes: Vec::new(),
                    help: None,
                });
                None
            }
        }
    };

    let _ = PathBuf::new(); // keep PathBuf in scope for future use
    AttributePrepassOutput {
        rewritten,
        diagnostics,
    }
}
