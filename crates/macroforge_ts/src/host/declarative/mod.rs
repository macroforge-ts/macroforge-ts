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

#[cfg(feature = "oxc")]
pub mod discovery;
#[cfg(feature = "oxc")]
pub mod expander;
#[cfg(feature = "oxc")]
pub mod matcher;
#[cfg(feature = "oxc")]
pub mod registry;
#[cfg(feature = "oxc")]
pub mod rewriter;

#[cfg(all(test, feature = "oxc"))]
mod tests;

#[cfg(feature = "oxc")]
pub use discovery::{DiscoveredMacro, discover};
#[cfg(feature = "oxc")]
pub use registry::{DeclarativeMacroRegistry, RegistryError};
#[cfg(feature = "oxc")]
pub use rewriter::{RewriteOutput, rewrite};
