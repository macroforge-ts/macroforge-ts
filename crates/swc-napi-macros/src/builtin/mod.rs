//! Built-in derive macros for the ts-macros framework.
//!
//! This crate provides the standard derive macros:
//! - `/** @derive(Debug) */` - Generates a `toString()` method for debugging
//! - `/** @derive(Clone) */` - Generates a `clone()` method for deep cloning
//! - `/** @derive(Eq) */` - Generates `equals()` and `hashCode()` methods

mod derive_clone;
mod derive_debug;
mod derive_eq;
