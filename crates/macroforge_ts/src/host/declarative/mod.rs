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
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BuildMode {
    /// Dev mode: all macros (including `Auto`) expand inline for precise
    /// diagnostics and type-checking. Share modes still emit the runtime
    /// helper so per-call code shapes match prod.
    #[default]
    Dev,
    /// Prod mode: share modes emit the runtime helper once and replace
    /// call sites with calls to it; `Auto` consults the megamorphism
    /// analyzer to pick share vs. cluster vs. expand.
    Prod,
}

impl BuildMode {
    /// Parse a `BuildMode` from the JS string option. Unknown values
    /// (including `None`) default to [`BuildMode::Dev`].
    pub fn from_option(s: Option<&str>) -> Self {
        match s {
            Some("prod") | Some("production") | Some("build") => BuildMode::Prod,
            _ => BuildMode::Dev,
        }
    }
}

#[cfg(feature = "oxc")]
pub mod discovery;
#[cfg(feature = "oxc")]
pub mod expander;
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
    DiscoveredMacro, ImportedMacro, ResolvedImports, discover, resolve_cross_file_imports,
};
#[cfg(feature = "oxc")]
pub use megamorph::{
    MacroPolymorphism, MegamorphReport, Recommendation, ResolvedCallSite, TypeCluster, TypeShape,
    analyze as analyze_megamorphism, extract_type_shape,
};
pub use project_registry::ProjectDeclarativeRegistry;
#[cfg(feature = "oxc")]
pub use registry::{DeclarativeMacroRegistry, RegistryError};
#[cfg(feature = "oxc")]
pub use rewriter::{RewriteOutput, rewrite};
