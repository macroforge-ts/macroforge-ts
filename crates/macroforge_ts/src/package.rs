//! The npm specifiers the macro engine emits into generated code.
//!
//! Generated modules import their runtime from the published package, so these
//! strings end up in every expanded file and in the `.d.ts` beside it. They
//! live here rather than at each emission site because a rename that reaches
//! only some of them produces output that resolves in some files and not
//! others, and nothing about the generated code says which is which.

/// The published package that carries the generated runtime.
pub const PACKAGE: &str = "@macroforge/core";

/// Serialization runtime: `DeserializeContext`, `DeserializeError`, `PendingRef`.
pub const SERDE: &str = "@macroforge/core/serde";

/// Declarative macro definitions (`macroRules`) and the `import macro` form.
pub const RULES: &str = "@macroforge/core/rules";

/// Build-time evaluation helpers.
pub const BUILDTIME: &str = "@macroforge/core/buildtime";

/// Trait definitions the derive macros implement against.
pub const TRAITS: &str = "@macroforge/core/traits";
