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

mod class_features;
mod decorator_stripping;
mod derive_basic;
mod early_bailout;
mod enum_tests;
mod expansion_context;
mod foreign_types;
mod interface_tests;
mod jsdoc_tests;
mod serde_tests;
mod source_mapping;
mod type_alias_tests;

// Some imports are used only by submodules via `use super::*;`
#[allow(unused_imports)]
use crate::host::PatchCollector;
use crate::host::config::ForeignTypeConfig;
#[allow(unused_imports)]
use crate::host::import_registry::{clear_foreign_types, set_foreign_types};
#[allow(unused_imports)]
use crate::ts_syn::abi::{
    ClassIR, DiagnosticLevel, MacroContextIR, MacroResult, Patch, PatchCode, SpanIR,
};
#[allow(unused_imports)]
use crate::{
    GeneratedRegionResult, MappingSegmentResult, NativePositionMapper, SourceMappingResult,
    host::MacroExpander, parse_import_sources,
};
#[allow(unused_imports)]
use swc_core::ecma::ast::{ClassMember, Program};
#[allow(unused_imports)]
use swc_core::{
    common::{FileName, GLOBALS, SourceMap, sync::Lrc},
    ecma::parser::{Lexer, Parser, StringInput, Syntax, TsSyntax},
};

/// Module path constant used for test derive macros.
const DERIVE_MODULE_PATH: &str = "@macro/derive";

/// Parses TypeScript source code into an SWC AST `Program`.
///
/// This helper function sets up the SWC parser with TypeScript syntax
/// (including decorator support) and returns the parsed module.
///
/// # Panics
///
/// Panics if the source code fails to parse.
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
