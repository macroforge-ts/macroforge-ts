//! Declarative (pattern-matching) macro grammar and parser.
//!
//! This module parses the template body of a declarative macro definition
//! of the form `` const $name = macro`...` ``. The template is its own
//! mini-language — **not** TypeScript — consisting of arms separated by
//! blank lines, where each arm is `pattern => body`.
//!
//! The output is a [`MacroDef`] that the host can use to match
//! invocations and expand bodies. The types and parser here are
//! host-agnostic (no OXC or SWC dependencies) so they can be unit
//! tested in isolation.
//!
//! See the execution plan in the repo root for a fuller description
//! of the grammar.

pub mod errors;
pub mod parser;
pub mod types;

#[cfg(test)]
mod tests;

pub use errors::DeclarativeError;
pub use parser::parse_macro_def;
pub use types::{
    Body, BodyToken, FragmentKind, MacroArm, MacroDef, MacroKind, MacroMode, Pattern,
    PatternElement, RepetitionKind,
};
