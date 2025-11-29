//! Built-in derive macros for the ts-macros framework.
//!
//! This crate provides the standard derive macros:
//! - `@Derive(Debug)` - Generates a `toString()` method for debugging
//! - `@Derive(Clone)` - Generates a `clone()` method for deep cloning
//! - `@Derive(Eq)` - Generates `equals()` and `hashCode()` methods

mod derive_clone;
mod derive_debug;
mod derive_eq;

// Re-export for direct usage/testing
pub use derive_clone::derive_clone_macro;
pub use derive_debug::derive_debug_macro;
pub use derive_eq::derive_eq_macro;
