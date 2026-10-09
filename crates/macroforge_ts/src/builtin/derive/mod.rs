//! The built-in derive macros, one module each, with the helpers they share
//! in [`common`].

/// Clone macro implementation (deep copy).
pub mod clone;

/// Helpers the derive macros share: field options, type utilities and
/// registry lookups.
pub mod common;

/// Debug macro implementation (toString).
mod debug;

/// Default macro implementation (factory method).
mod default;

/// Eq macro implementation (a marker over PartialEq).
mod eq;

/// Hash macro implementation (hashCode).
pub mod hash;

/// Ord macro implementation (total ordering).
mod ord;

/// PartialEq macro implementation (equals).
pub mod partial_eq;

/// PartialOrd macro implementation (partial ordering).
mod partial_ord;

/// Encoding macros (Encode, Decode).
pub mod endec;
