//! Host-side support for declarative (pattern-matching) macros.
//!
//! This module coordinates the discovery, matching, and rewriting of
//! user-defined declarative macros of the form
//! `` const $name = macro`...` ``. It runs as a pre-pass before the
//! existing derive macro pipeline, producing a set of [`Patch`]es that
//! rewrite call sites and strip the original macro definitions.
//!
//! Unless noted, everything here only compiles under the `oxc` feature —
//! the SWC pipeline does not support declarative macros in the MVP.

/// Build mode that controls reverse-monomorphization behavior.
///
/// Propagated from `ExpandOptions.build_mode` (a user-facing string) into
/// the rewriter, which uses it to decide whether [`crate::ts_syn::declarative::MacroMode::Auto`]
/// macros expand inline (dev) or run through the share-mode pipeline
/// (prod).
///
/// PR 14 turned `Dev` into a struct variant so developers can opt
/// into analyzer telemetry diagnostics without affecting the default
/// flow. PR 17 added the `force_share` flag so users can test the
/// share-mode emission path in dev without rebuilding for prod.
/// Both flags default to `false` and are constructed via
/// [`BuildMode::dev`] for backward compatibility with existing
/// call sites; the struct-variant form is used at the few places
/// that need to read the flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildMode {
    /// Dev mode: all macros (including `Auto`) expand inline for precise
    /// diagnostics and type-checking. Share modes still emit the runtime
    /// helper so per-call code shapes match prod.
    ///
    /// `analyzer_telemetry: true` emits an `Info`-level diagnostic
    /// for every `Auto` macro describing the megamorph analyzer's
    /// decision (Share / Cluster / ForceExpand) and the distinct
    /// shape count, so users can see why a particular emission
    /// strategy was picked. Defaults to `false`.
    ///
    /// `force_share: true` is PR 17's opt-in — when set, `Auto`
    /// macros expand to the shared-runtime form in dev, matching
    /// the prod pipeline, so share-mode bugs surface at dev time
    /// instead of only in prod builds. Defaults to `false`.
    Dev {
        analyzer_telemetry: bool,
        force_share: bool,
    },
    /// Prod mode: share modes emit the runtime helper once and replace
    /// call sites with calls to it; `Auto` consults the megamorphism
    /// analyzer to pick share vs. cluster vs. expand.
    Prod,
}

impl Default for BuildMode {
    fn default() -> Self {
        Self::dev()
    }
}

impl BuildMode {
    /// The default dev build mode: analyzer telemetry off, force-share off.
    /// Use this at call sites that don't care about the flags (test
    /// helpers, the CLI default).
    pub const fn dev() -> Self {
        BuildMode::Dev {
            analyzer_telemetry: false,
            force_share: false,
        }
    }

    /// The default prod build mode.
    pub const fn prod() -> Self {
        BuildMode::Prod
    }

    /// Parse a `BuildMode` from the JS string option. Unknown values
    /// (including `None`) default to [`BuildMode::dev`].
    pub fn from_option(s: Option<&str>) -> Self {
        match s {
            Some("prod") | Some("production") | Some("build") => BuildMode::prod(),
            _ => BuildMode::dev(),
        }
    }

    /// Returns `true` if this is any `Dev` variant (ignoring flag
    /// differences). Used by pipeline gates that care only about
    /// "dev vs prod" granularity.
    pub fn is_dev(&self) -> bool {
        matches!(self, BuildMode::Dev { .. })
    }

    /// Returns `true` if analyzer telemetry diagnostics should be
    /// emitted. False in `Prod` (prod builds shouldn't spam diags).
    pub fn analyzer_telemetry(&self) -> bool {
        matches!(
            self,
            BuildMode::Dev {
                analyzer_telemetry: true,
                ..
            }
        )
    }

    /// Returns `true` if `Auto` macros should use the share-mode
    /// emission path even in dev (PR 17).
    pub fn force_share(&self) -> bool {
        matches!(
            self,
            BuildMode::Dev {
                force_share: true,
                ..
            }
        )
    }
}

#[cfg(feature = "oxc")]
pub mod discovery;
#[cfg(feature = "oxc")]
pub mod expander;
#[cfg(feature = "oxc")]
mod hygiene;
#[cfg(feature = "oxc")]
pub mod matcher;
#[cfg(feature = "oxc")]
pub mod megamorph;
pub mod project_registry;
#[cfg(feature = "oxc")]
pub mod registry;
#[cfg(feature = "oxc")]
pub mod rewriter;
#[cfg(feature = "oxc")]
pub mod type_walker;

#[cfg(all(test, feature = "oxc"))]
mod tests;

#[cfg(feature = "oxc")]
pub use discovery::{
    DiscoveredMacro, ImportedMacro, ResolvedImports, collect_dollar_imports, discover,
    resolve_cross_file_imports,
};
#[cfg(feature = "oxc")]
pub use megamorph::{
    MacroPolymorphism, MegamorphReport, Recommendation, ResolvedCallSite, TypeCluster, TypeShape,
    analyze as analyze_megamorphism, extract_type_shape,
};
pub use project_registry::ProjectDeclarativeRegistry;
#[cfg(feature = "oxc")]
pub use registry::{DeclarativeMacroRegistry, RegistryError};
#[cfg(all(not(feature = "swc"), feature = "oxc"))]
pub use rewriter::ProcMacroFallback;
pub use rewriter::RewriteOutput;
#[cfg(all(not(feature = "swc"), feature = "oxc"))]
pub use rewriter::rewrite;

/// Parse `source` with OXC purely to confirm it is syntactically valid
/// TypeScript and surface any parse errors as structured [`Diagnostic`]s.
///
/// Used by the patch applicator's post-validation hook (Phase H of the
/// production-hardening plan) to re-parse the fully-patched source after
/// declarative macro expansion. If a macro's expansion produced invalid
/// TypeScript, this helper turns OXC's parse errors into diagnostics the
/// caller can blame on the originating macro via the patch source-map.
///
/// `jsx` should match whatever `SourceType` the caller used for the
/// original parse — i.e., `true` iff the file's extension is `.tsx`.
///
/// Returns `Ok(())` when the source parses cleanly, or `Err(diagnostics)`
/// with one `DiagnosticLevel::Error` entry per OXC error. Each entry's
/// `message` is the OXC diagnostic text; `span` is left unset because
/// the caller is responsible for mapping offsets back to a source
/// macro.
#[cfg(feature = "oxc")]
pub fn reparse_for_validation(
    source: &str,
    jsx: bool,
) -> Result<(), Vec<crate::ts_syn::abi::Diagnostic>> {
    use crate::ts_syn::abi::{Diagnostic, DiagnosticLevel};
    use oxc::allocator::Allocator;
    use oxc::parser::Parser;
    use oxc::span::SourceType;

    let allocator = Allocator::default();
    let source_type = SourceType::ts().with_jsx(jsx);
    let parsed = Parser::new(&allocator, source, source_type).parse();

    if parsed.errors.is_empty() {
        return Ok(());
    }

    let diagnostics = parsed
        .errors
        .into_iter()
        .map(|err| Diagnostic {
            level: DiagnosticLevel::Error,
            message: err.to_string(),
            span: None,
            notes: Vec::new(),
            help: None,
        })
        .collect();
    Err(diagnostics)
}

/// Validate a post-patch source and attribute parse errors to the
/// originating declarative macro via a [`SourceMapping`].
///
/// Unlike [`reparse_for_validation`], this variant does offset
/// blame-tracing: for every OXC parse error with a labeled span, it
/// looks the byte offset up in `mapping` (built by the patch
/// applicator's `apply_with_mapping`) and, if the offset falls inside
/// a generated region, prefixes the diagnostic with the originating
/// macro name. This turns "Parse error after declarative macro
/// expansion: unexpected token" into "macro `$foo` produced invalid
/// TypeScript: unexpected token (at offset N)".
///
/// Returns an empty vector when the source parses cleanly. Otherwise
/// returns one `Error`-level `Diagnostic` per OXC parse error.
#[cfg(feature = "oxc")]
pub fn validate_expanded_source(
    source: &str,
    mapping: &crate::ts_syn::abi::SourceMapping,
    jsx: bool,
) -> Vec<crate::ts_syn::abi::Diagnostic> {
    use crate::ts_syn::abi::{Diagnostic, DiagnosticLevel};
    use oxc::allocator::Allocator;
    use oxc::parser::Parser;
    use oxc::span::SourceType;

    let allocator = Allocator::default();
    let source_type = SourceType::ts().with_jsx(jsx);
    let parsed = Parser::new(&allocator, source, source_type).parse();

    if parsed.errors.is_empty() {
        return Vec::new();
    }

    parsed
        .errors
        .into_iter()
        .map(|err| {
            // OXC diagnostics carry a `Vec<LabeledSpan>` (from
            // oxc-miette) via a `Deref` on `OxcDiagnostic`. The
            // first label is the "this is where it broke" marker;
            // its byte offset is what we feed to `generated_by`.
            let offset: Option<u32> = err
                .labels
                .as_ref()
                .and_then(|v| v.first())
                .map(|ls| ls.offset() as u32);

            let attribution = offset.and_then(|o| mapping.generated_by(o).map(|s| s.to_string()));

            let message = match attribution {
                Some(macro_name) => format!(
                    "macro `{}` produced invalid TypeScript: {}",
                    macro_name, err
                ),
                None => format!(
                    "declarative macro expansion produced invalid TypeScript: {}",
                    err
                ),
            };

            Diagnostic {
                level: DiagnosticLevel::Error,
                message,
                span: None,
                notes: Vec::new(),
                help: None,
            }
        })
        .collect()
}

#[cfg(all(test, feature = "oxc"))]
mod reparse_validation_tests {
    use super::reparse_for_validation;
    use crate::ts_syn::abi::DiagnosticLevel;

    #[test]
    fn valid_typescript_parses_cleanly() {
        let source = "const x: number = 1 + 2;";
        assert!(reparse_for_validation(source, false).is_ok());
    }

    #[test]
    fn invalid_typescript_surfaces_error_diagnostic() {
        // `const x = ;` — an expression is missing after `=`.
        let source = "const x = ;";
        let diagnostics = reparse_for_validation(source, false).unwrap_err();
        assert!(
            !diagnostics.is_empty(),
            "expected at least one parse-error diagnostic"
        );
        assert!(
            diagnostics
                .iter()
                .all(|d| matches!(d.level, DiagnosticLevel::Error)),
            "all returned diagnostics should be errors, got: {:?}",
            diagnostics.iter().map(|d| &d.level).collect::<Vec<_>>()
        );
        assert!(
            diagnostics.iter().all(|d| !d.message.is_empty()),
            "error diagnostics must carry a non-empty message"
        );
    }

    #[test]
    fn jsx_source_respects_jsx_flag() {
        // A JSX fragment parses only when `jsx = true`.
        let source = "const el = <div>hello</div>;";
        assert!(
            reparse_for_validation(source, true).is_ok(),
            "JSX should parse with jsx=true"
        );
        assert!(
            reparse_for_validation(source, false).is_err(),
            "JSX should fail to parse with jsx=false"
        );
    }
}
