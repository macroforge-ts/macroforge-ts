//! # Macroforge TypeScript Macro Engine
//!
//! This crate provides a TypeScript macro expansion engine that brings Rust-like derive macros
//! to TypeScript. It is designed to be used via NAPI bindings from Node.js, enabling compile-time
//! code generation for TypeScript projects.
//!
//! ## Overview
//!
//! Macroforge processes TypeScript source files containing `@derive` decorators and expands them
//! into concrete implementations. For example, a class decorated with `@derive(Debug, Clone)`
//! will have `toString()` and `clone()` methods automatically generated.
//!
//! ## Architecture
//!
//! The crate is organized into several key components:
//!
//! - **NAPI Bindings** (`NativePlugin`, `expand_sync`, `transform_sync`): Entry points for Node.js
//! - **Position Mapping** (`NativePositionMapper`, `NativeMapper`): Bidirectional source mapping
//!   for IDE integration
//! - **Macro Host** (`host` module): Core expansion engine with registry and dispatcher
//! - **Built-in Macros** (`builtin` module): Standard derive macros (Debug, Clone, Serialize, etc.)
//!
//! ## Performance Considerations
//!
//! - Uses a 32MB thread stack to prevent stack overflow during deep SWC AST recursion
//! - Implements early bailout for files without `@derive` decorators
//! - Caches expansion results keyed by filepath and version
//! - Uses binary search for O(log n) position mapping lookups
//!
//! ## Usage from Node.js
//!
//! ```javascript
//! const { NativePlugin, expand_sync } = require('macroforge-ts');
//!
//! // Create a plugin instance with caching
//! const plugin = new NativePlugin();
//!
//! // Process a file (uses cache if version matches)
//! const result = plugin.process_file(filepath, code, { version: '1.0' });
//!
//! // Or use the sync function directly
//! const result = expand_sync(code, filepath, { keep_decorators: false });
//! ```
//!
//! ## Re-exports for Macro Authors
//!
//! This crate re-exports several dependencies for convenience when writing custom macros:
//! - `ts_syn`: TypeScript syntax types for AST manipulation
//! - `macros`: Macro attributes and quote templates
//! - `swc_core`, `swc_common`, `swc_ecma_ast`: SWC compiler infrastructure

// Allow the crate to reference itself as `macroforge_ts`.
// This self-reference is required for the macroforge_ts_macros generated code
// to correctly resolve paths when the macro expansion happens within this crate.
extern crate self as macroforge_ts;

// ============================================================================
// Re-exports for Macro Authors
// ============================================================================
// These re-exports allow users to only depend on `macroforge_ts` in their
// Cargo.toml instead of needing to add multiple dependencies.

// Re-export internal crates (needed for generated code)
pub extern crate inventory;
pub extern crate macroforge_ts_macros;
pub extern crate macroforge_ts_quote;
pub extern crate macroforge_ts_syn;
pub extern crate napi;
pub extern crate napi_derive;
pub extern crate serde_json;

/// Debug logging for external macros.
/// Writes to `.macroforge/debug.log` relative to the project root.
/// Use: `macroforge_ts::debug::log("tag", "message")` or `macroforge_ts::debug_log!("tag", "...")`
pub mod debug;

/// TypeScript syntax types for macro development
/// Use: `use macroforge_ts::ts_syn::*;`
pub use macroforge_ts_syn as ts_syn;

/// Macro attributes and quote templates
/// Use: `use macroforge_ts::macros::*;`
pub mod macros {
    // Re-export the ts_macro_derive attribute
    pub use macroforge_ts_macros::ts_macro_derive;

    // Re-export all quote macros
    pub use macroforge_ts_quote::{ts_quote, ts_template};
}

// ============================================================================
// Wrapper macros for proper $crate resolution
// ============================================================================
// These wrapper macros ensure that $crate resolves to macroforge_ts (this crate)
// rather than macroforge_ts_syn, allowing users to only depend on macroforge_ts.

/// Creates an SWC [`Ident`](swc_core::ecma::ast::Ident) from a string or format expression.
///
/// This is a re-export wrapper that ensures `$crate` resolves correctly when
/// used from crates that only depend on `macroforge_ts`.
///
/// # Examples
///
/// ```rust,ignore
/// use macroforge_ts::ident;
///
/// let simple = ident!("foo");
/// let formatted = ident!("{}Bar", "foo");
/// ```
#[macro_export]
macro_rules! ident {
    ($name:expr) => {
        $crate::swc_core::ecma::ast::Ident::new_no_ctxt(
            AsRef::<str>::as_ref(&$name).into(),
            $crate::swc_core::common::DUMMY_SP,
        )
    };
    ($fmt:expr, $($args:expr),+ $(,)?) => {
        $crate::swc_core::ecma::ast::Ident::new_no_ctxt(
            format!($fmt, $($args),+).into(),
            $crate::swc_core::common::DUMMY_SP,
        )
    };
}

/// Creates a private (marked) SWC [`Ident`](swc_core::ecma::ast::Ident).
///
/// Unlike [`ident!`], this macro creates an identifier with a fresh hygiene mark,
/// making it unique and preventing name collisions with user code.
#[macro_export]
macro_rules! private_ident {
    ($name:expr) => {{
        let mark = $crate::swc_core::common::Mark::fresh($crate::swc_core::common::Mark::root());
        $crate::swc_core::ecma::ast::Ident::new(
            $name.into(),
            $crate::swc_core::common::DUMMY_SP,
            $crate::swc_core::common::SyntaxContext::empty().apply_mark(mark),
        )
    }};
}

// Re-export swc_core and common modules (via ts_syn for version consistency)
pub use macroforge_ts_syn::swc_common;
pub use macroforge_ts_syn::swc_core;
pub use macroforge_ts_syn::swc_ecma_ast;

// ============================================================================
// Internal modules
// ============================================================================
pub mod host;

// Build script utilities (enabled with "build" feature)
#[cfg(feature = "build")]
pub mod build;

// Re-export abi types from ts_syn
pub use ts_syn::abi;

pub mod builtin;

// ============================================================================
// Extracted submodules
// ============================================================================
mod napi_types;
mod position_mapper;
mod plugin;
mod expand_core;
mod napi_functions;
mod manifest;

// ============================================================================
// Public re-exports (preserving the original public API)
// ============================================================================
pub use napi_types::{
    TransformResult, MacroDiagnostic, MappingSegmentResult, GeneratedRegionResult,
    SourceMappingResult, ExpandResult, ImportSourceResult, SyntaxCheckResult, SpanResult,
    JsDiagnostic, ExpandOptions, ProcessFileOptions, ScanOptions, ScanResult, LoadConfigResult,
};

pub use position_mapper::{NativePositionMapper, NativeMapper};

pub use plugin::NativePlugin;

pub use napi_functions::{
    expand_sync, transform_sync, check_syntax, parse_import_sources, derive_decorator,
    load_config, clear_config_cache, scan_project_sync,
};

pub use manifest::{
    MacroManifestEntry, DecoratorManifestEntry, MacroManifest,
    get_macro_manifest, is_macro_package, get_macro_names,
    debug_get_modules, debug_lookup, debug_descriptors,
};

// Re-export internal items used by tests
#[cfg(test)]
pub(crate) use expand_core::{expand_inner, has_macro_annotations};

#[cfg(test)]
mod test;
