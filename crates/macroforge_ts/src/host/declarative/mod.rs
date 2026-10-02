//! Host-side support for declarative (pattern-matching) macros.
//!
//! This module coordinates the discovery, matching, and rewriting of
//! user-defined declarative macros, declared either as a tagged template
//! (`` const $name = macroRules`...` ``) or as an object literal
//! (`const $name = macroRules({...})`). It runs as a pre-pass before the
//! existing derive macro pipeline, producing a set of [`Patch`]es that
//! rewrite call sites and strip the original macro definitions.

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
    /// Dev mode: `ExpandOnly` and `Auto` macros expand inline for precise
    /// diagnostics and type-checking (`Auto` switches to the share path
    /// when `force_share` is set). `ShareOnly` / `ShareAnyway` emit the
    /// runtime helper and rewrite call sites in dev too, same as prod.
    ///
    /// `analyzer_telemetry: true` emits an `Info`-level diagnostic
    /// for every `Auto` macro describing the megamorph analyzer's
    /// decision (Share / Cluster / ForceExpand) and the distinct
    /// shape count, so users can see why a particular emission
    /// strategy was picked. Defaults to `false`.
    ///
    /// `force_share: true` is PR 17's opt-in: when set, `Auto`
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

pub mod discovery;
pub mod expander;
mod hygiene;
pub(crate) mod macro_imports;
pub mod matcher;
pub mod megamorph;
pub mod project_registry;
pub mod registry;
pub mod rewriter;
pub mod type_walker;

#[cfg(test)]
mod tests;

pub use discovery::{
    DiscoveredMacro, ImportedMacro, ResolvedImports, collect_dollar_imports, discover,
    resolve_cross_file_imports,
};
pub use megamorph::{
    MacroPolymorphism, MegamorphReport, Recommendation, ResolvedCallSite, TypeCluster, TypeShape,
    analyze as analyze_megamorphism, extract_type_shape,
};
pub use project_registry::ProjectDeclarativeRegistry;
pub use registry::{DeclarativeMacroRegistry, RegistryError};
pub use rewriter::ProcMacroFallback;
pub use rewriter::RewriteOutput;
pub use rewriter::rewrite;

/// Validate a post-patch source and attribute parse errors to the
/// originating declarative macro via a [`SourceMapping`].
///
/// This does offset blame-tracing: for every OXC parse error with a
/// labeled span, it
/// looks the byte offset up in `mapping` (built by the patch
/// applicator's `apply_with_mapping`) and, if the offset falls inside
/// a generated region, prefixes the diagnostic with the originating
/// macro name. This turns "Parse error after declarative macro
/// expansion: unexpected token" into "macro `$foo` produced invalid
/// TypeScript: unexpected token (at offset N)".
///
/// Returns an empty vector when the source parses cleanly. Otherwise
/// returns one `Error`-level `Diagnostic` per OXC parse error.
pub fn validate_expanded_source(
    source: &str,
    mapping: &crate::ts_syn::abi::SourceMapping,
    jsx: bool,
) -> Vec<crate::ts_syn::abi::Diagnostic> {
    use oxc::allocator::Allocator;
    use oxc::parser::Parser;

    let allocator = Allocator::default();
    let parsed = Parser::new(&allocator, source, validation_source_type(jsx)).parse();
    attribute_parse_errors(parsed.diagnostics, mapping)
}

/// The source type post-validation parses expanded output with.
pub fn validation_source_type(jsx: bool) -> oxc::span::SourceType {
    oxc::span::SourceType::ts().with_jsx(jsx)
}

/// `errors`, from parsing expanded output, as diagnostics that name the
/// macro whose generated code each one falls in, where `mapping` knows.
pub fn attribute_parse_errors(
    errors: oxc::diagnostics::Diagnostics,
    mapping: &crate::ts_syn::abi::SourceMapping,
) -> Vec<crate::ts_syn::abi::Diagnostic> {
    use crate::ts_syn::abi::{Diagnostic, DiagnosticLevel};

    errors
        .into_iter()
        .map(|err| {
            // OXC diagnostics carry their `LabeledSpan`s via a `Deref`
            // on `OxcDiagnostic`. The first label is the "this is where
            // it broke" marker; its byte offset is what we feed to
            // `generated_by`.
            let offset: Option<u32> = err.labels.first().map(|ls| ls.offset());

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

#[cfg(test)]
mod validation_tests {
    use super::validate_expanded_source;
    use crate::ts_syn::abi::{DiagnosticLevel, SourceMapping};

    fn validate(source: &str, jsx: bool) -> Vec<crate::ts_syn::abi::Diagnostic> {
        validate_expanded_source(source, &SourceMapping::new(), jsx)
    }

    #[test]
    fn valid_typescript_parses_cleanly() {
        assert!(validate("const x: number = 1 + 2;", false).is_empty());
    }

    #[test]
    fn invalid_typescript_surfaces_error_diagnostic() {
        // `const x = ;`: an expression is missing after `=`.
        let diagnostics = validate("const x = ;", false);
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
            validate(source, true).is_empty(),
            "JSX should parse with jsx=true"
        );
        assert!(
            !validate(source, false).is_empty(),
            "JSX should fail to parse with jsx=false"
        );
    }
}
