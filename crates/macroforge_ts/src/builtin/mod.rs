//! Built-in derive macros for the macroforge framework.
//!
//! This crate provides the standard derive macros:
//! - `/** @derive(Debug) */` - Generates a `toString()` method for debugging
//! - `/** @derive(Clone) */` - Generates a `clone()` method for deep cloning
//! - `/** @derive(Eq) */` - Generates `equals()` and `hashCode()` methods
//! - `/** @derive(Serialize) */` - Generates a `toJSON()` method for JSON serialization
//! - `/** @derive(Deserialize) */` - Generates a static `fromJSON()` method for JSON deserialization

mod derive_clone;
mod derive_debug;
mod derive_eq;
mod serde;
