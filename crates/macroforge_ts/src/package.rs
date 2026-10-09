//! The npm specifiers the macro engine emits into generated code.
//!
//! Generated modules import their runtime from the published package, so these
//! strings end up in every expanded file and in the `.d.ts` beside it. They
//! live here rather than at each emission site because a rename that reaches
//! only some of them produces output that resolves in some files and not
//! others, and nothing about the generated code says which is which.

/// The published package that carries the generated runtime.
pub const PACKAGE: &str = "@macroforge/core";

/// Encoding runtime: `DecodeContext`, `DecodeError`, `PendingRef`.
pub const ENDEC: &str = "@macroforge/core/endec";

/// Runtime fallbacks for `PartialEq`, `Hash` and `Clone` on values whose
/// declared type does not say how to compare, hash or copy them.
pub const STRUCTURAL: &str = "@macroforge/core/structural";

/// Declarative macro definitions (`macroRules`) and the `import macro` form.
pub const RULES: &str = "@macroforge/core/rules";
