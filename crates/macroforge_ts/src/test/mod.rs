//! # Test Suite for Macroforge TypeScript Macro Engine
//!
//! This module contains comprehensive tests for the macro expansion system,
//! covering all built-in macros and their behavior across different TypeScript
//! type constructs (classes, interfaces, enums, type aliases).
//!
//! ## Test Categories
//!
//! ### Derive Macro Tests
//!
//! Tests for each built-in derive macro:
//!
//! - **Debug** - Generates `toString()` methods
//! - **Clone** - Generates `clone()` methods for deep copying
//! - **PartialEq** - Generates `equals()` methods for equality comparison
//! - **Hash** - Generates `hashCode()` methods for hash-based collections
//! - **Ord/PartialOrd** - Generates `compare()` and `partialCompare()` methods for ordering
//! - **Default** - Generates `defaultValue()` factory methods
//! - **Encode** - Generates JSON encoding methods
//! - **Decode** - Generates JSON decoding methods with validation
//!
//! ### Type Construct Tests
//!
//! Each macro is tested against:
//!
//! - Classes (with constructors, methods, visibility modifiers)
//! - Interfaces (generates namespace with static functions)
//! - Enums (numeric and string enums)
//! - Type Aliases (object types and union types)
//!
//! ### DTS Output Tests
//!
//! Tests verifying correct `.d.ts` type declaration generation:
//!
//! - Method signatures are properly typed
//! - Constructor bodies are stripped
//! - Visibility modifiers are preserved
//! - Generic type parameters are preserved
//!
//! ### Source Mapping Tests
//!
//! Tests for bidirectional position mapping between original and expanded code.
//!
//! ### Early Bailout Tests
//!
//! Tests verifying that files without `@derive` are returned unchanged
//! (important for Svelte runes and other non-macro TypeScript code).
//!
//! ## Running Tests
//!
//! ```bash
//! cargo test -p macroforge_ts
//! ```

#[cfg(feature = "buildtime-boa")]
mod buildtime_integration;
mod class_features;
mod decorator_stripping;
mod derive_basic;
mod early_bailout;
mod endec_tests;
mod enum_tests;
mod foreign_types;
mod interface_tests;
mod jsdoc_tests;
mod resident_registries;
mod source_mapping;
mod type_alias_tests;

use crate::host::MacroExpander;
use crate::host::config::ForeignTypeConfig;
use crate::host::expand::MacroExpansion;

/// Expands macros in `source` as `test.ts`.
fn expand_test(source: &str) -> MacroExpansion {
    expand_test_file(source, "test.ts")
}

/// Expands macros in `source` as `file_name`.
fn expand_test_file(source: &str, file_name: &str) -> MacroExpansion {
    let host = MacroExpander::new().unwrap();
    host.expand_source(source, file_name).unwrap()
}

/// Helper to create a ForeignTypeConfig for testing.
fn make_foreign_type(
    name: &str,
    from: Vec<&str>,
    encode_expr: Option<&str>,
    decode_expr: Option<&str>,
    default_expr: Option<&str>,
    has_shape_expr: Option<&str>,
) -> ForeignTypeConfig {
    ForeignTypeConfig {
        name: name.to_string(),
        from: from.into_iter().map(|s| s.to_string()).collect(),
        encode_expr: encode_expr.map(|s| s.to_string()),
        decode_expr: decode_expr.map(|s| s.to_string()),
        default_expr: default_expr.map(|s| s.to_string()),
        has_shape_expr: has_shape_expr.map(|s| s.to_string()),
        aliases: vec![],
        builtin: false,
        handler_sites: Vec::new(),
    }
}
