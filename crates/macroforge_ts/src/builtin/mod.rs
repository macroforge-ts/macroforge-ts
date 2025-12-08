//! Built-in derive macros for the macroforge framework.
//!
//! This crate provides the standard derive macros:
//!
//! ## Equality & Hashing
//! - `/** @derive(PartialEq) */` - Generates an `equals()` method for field-by-field comparison
//! - `/** @derive(Hash) */` - Generates a `hashCode()` method for hashing
//!
//! ## Ordering
//! - `/** @derive(PartialOrd) */` - Generates a `compareTo()` method (returns -1, 0, 1, or null)
//! - `/** @derive(Ord) */` - Generates a `compareTo()` method for total ordering (never null)
//!
//! ## Cloning & Debugging
//! - `/** @derive(Clone) */` - Generates a `clone()` method for deep cloning
//! - `/** @derive(Debug) */` - Generates a `toString()` method for debugging
//!
//! ## Initialization
//! - `/** @derive(Default) */` - Generates a static `default()` factory method
//!
//! ## Serialization
//! - `/** @derive(Serialize) */` - Generates a `toJSON()` method for JSON serialization
//! - `/** @derive(Deserialize) */` - Generates a static `fromJSON()` method for JSON deserialization

mod derive_clone;
mod derive_common;
mod derive_debug;
mod derive_default;
mod derive_hash;
mod derive_ord;
mod derive_partial_eq;
mod derive_partial_ord;
mod serde;
