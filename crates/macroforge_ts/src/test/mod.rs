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
//! - **Ord/PartialOrd** - Generates `compareTo()` methods for ordering
//! - **Default** - Generates `defaultValue()` factory methods
//! - **Serialize** - Generates JSON serialization methods
//! - **Deserialize** - Generates JSON deserialization methods with validation
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

#[cfg(all(feature = "oxc", feature = "buildtime-boa"))]
// Buildtime is OXC-only — the OxcBackend's `@buildtime` pre-pass is gated
// behind `not(feature = "swc")`, so these tests only succeed on the
// OXC-default profile.
#[cfg(not(feature = "swc"))]
mod buildtime_integration;
mod class_features;
mod decorator_stripping;
mod derive_basic;
mod early_bailout;
mod enum_tests;
#[cfg(feature = "swc")]
mod expansion_context;
mod foreign_types;
mod interface_tests;
mod jsdoc_tests;
mod serde_tests;
#[cfg(feature = "swc")]
mod source_mapping;
mod type_alias_tests;

// Some imports are used only by submodules via `use super::*;`
#[allow(unused_imports)]
use crate::host::PatchCollector;
use crate::host::config::ForeignTypeConfig;
use crate::host::expand::MacroExpansion;
#[allow(unused_imports)]
use crate::host::import_registry::{clear_foreign_types, set_foreign_types};
#[allow(unused_imports)]
use crate::ts_syn::abi::{
    ClassIR, DiagnosticLevel, MacroContextIR, MacroResult, Patch, PatchCode, SpanIR,
};
#[allow(unused_imports)]
use crate::{
    GeneratedRegionResult, MappingSegmentResult, SourceMappingResult, host::MacroExpander,
};
#[cfg(feature = "node")]
#[allow(unused_imports)]
use crate::{NativePositionMapper, parse_import_sources};

// SWC-specific imports for tests that need low-level SWC APIs (expansion_context, source_mapping)
#[cfg(feature = "swc")]
#[allow(unused_imports)]
use swc_core::ecma::ast::{ClassMember, Program};
#[cfg(feature = "swc")]
#[allow(unused_imports)]
use swc_core::{
    common::{FileName, GLOBALS, SourceMap, sync::Lrc},
    ecma::parser::{Lexer, Parser, StringInput, Syntax, TsSyntax},
};

#[cfg(feature = "swc")]
const DERIVE_MODULE_PATH: &str = "@macro/derive";

/// Backend-agnostic test helper: expands macros in the given source code.
fn expand_test(source: &str) -> MacroExpansion {
    expand_test_file(source, "test.ts")
}

/// Backend-agnostic test helper with custom filename.
fn expand_test_file(source: &str, file_name: &str) -> MacroExpansion {
    let host = MacroExpander::new().unwrap();
    host.expand_source(source, file_name).unwrap()
}

/// Parses TypeScript source code into an SWC AST `Program`.
#[cfg(feature = "swc")]
fn parse_module(source: &str) -> Program {
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(
        FileName::Custom("test.ts".into()).into(),
        source.to_string(),
    );

    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax {
            decorators: true,
            ..Default::default()
        }),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().expect("should parse");
    Program::Module(module)
}

#[cfg(feature = "swc")]
/// Creates a minimal `ClassIR` for testing macro expansion.
///
/// Returns a class IR with the given name and default spans,
/// used as input for testing macro output processing.
fn base_class(name: &str) -> ClassIR {
    ClassIR {
        name: name.into(),
        span: SpanIR::new(0, 200),
        body_span: SpanIR::new(10, 190),
        is_abstract: false,
        type_params: vec![],
        heritage: vec![],
        decorators: vec![],
        decorators_ast: vec![],
        fields: vec![],
        methods: vec![],
        members: vec![],
    }
}

/// Helper to create a ForeignTypeConfig for testing.
fn make_foreign_type(
    name: &str,
    from: Vec<&str>,
    serialize_expr: Option<&str>,
    deserialize_expr: Option<&str>,
    default_expr: Option<&str>,
    has_shape_expr: Option<&str>,
    expression_namespaces: Vec<&str>,
) -> ForeignTypeConfig {
    ForeignTypeConfig {
        name: name.to_string(),
        namespace: if name.contains('.') {
            Some(name.rsplit_once('.').unwrap().0.to_string())
        } else {
            None
        },
        from: from.into_iter().map(|s| s.to_string()).collect(),
        serialize_expr: serialize_expr.map(|s| s.to_string()),
        serialize_import: None,
        deserialize_expr: deserialize_expr.map(|s| s.to_string()),
        deserialize_import: None,
        default_expr: default_expr.map(|s| s.to_string()),
        default_import: None,
        has_shape_expr: has_shape_expr.map(|s| s.to_string()),
        has_shape_import: None,
        aliases: vec![],
        expression_namespaces: expression_namespaces
            .into_iter()
            .map(|s| s.to_string())
            .collect(),
    }
}
