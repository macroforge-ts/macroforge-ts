//! # Macroforge TypeScript Macro Engine
//!
//! This crate provides a TypeScript macro expansion engine that brings Rust-like derive macros
//! to TypeScript. It supports multiple output targets via feature flags:
//! - `node`: (Default) Native Node.js bindings via NAPI-RS for maximum performance.
//! - `wasm`: Universal WebAssembly module via wasm-bindgen for browser and edge environments.
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
//! - **Unified API** (`api` module): An output-agnostic trait-based interface (`MacroforgeApi`)
//!   that defines all macro operations.
//! - **Target Bindings**:
//!   - `bindings_napi`: Node.js specific entry points using NAPI-RS.
//!   - `bindings_wasm`: Universal entry points using `wasm-bindgen`.
//! - **Position Mapping** (`api_types::SourceMappingResult`): Bidirectional source mapping
//!   for IDE integration.
//! - **Macro Host** (`host` module): Core expansion engine with registry and dispatcher.
//! - **Built-in Macros** (`builtin` module): Standard derive macros (Debug, Clone, Serialize, etc.).
//!
//! ## Usage
//!
//! ### From Node.js (Default)
//!
//! ```javascript
//! const { expandSync } = require('macroforge');
//! const result = expandSync(code, filepath, { keep_decorators: false });
//! ```
//!
//! ### From WASM
//!
//! ```javascript
//! import init, { expand_sync } from './pkg/macroforge_ts.js';
//! await init();
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
#[cfg(feature = "node")]
pub extern crate napi;
#[cfg(feature = "node")]
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
pub mod api;
pub mod api_types;
mod expand_core;
mod manifest;

#[cfg(feature = "node")]
pub mod bindings_napi;
#[cfg(feature = "wasm")]
pub mod bindings_wasm;

#[cfg(feature = "node")]
mod plugin;
#[cfg(feature = "node")]
mod position_mapper;

// ============================================================================
// Public re-exports (preserving the original public API)
// ============================================================================
pub use api_types::{
    ExpandOptions, ExpandResult, GeneratedRegionResult, ImportSourceResult, JsDiagnostic,
    LoadConfigResult, MacroDiagnostic, MappingSegmentResult, ProcessFileOptions, ScanOptions,
    ScanResult, SourceMappingResult, SpanResult, SyntaxCheckResult, TransformResult,
};

#[cfg(feature = "node")]
pub use position_mapper::{NativeMapper, NativePositionMapper};

#[cfg(feature = "node")]
pub use plugin::NativePlugin;

#[cfg(feature = "node")]
pub use bindings_napi::{
    check_syntax, clear_config_cache, derive_decorator, expand_sync, load_config,
    parse_import_sources, scan_project_sync, transform_sync,
};

#[cfg(feature = "wasm")]
pub use bindings_wasm::{
    check_syntax as wasm_check_syntax, clear_config_cache as wasm_clear_config_cache,
    derive_decorator as wasm_derive_decorator, expand_sync as wasm_expand_sync,
    load_config as wasm_load_config, parse_import_sources as wasm_parse_import_sources,
    scan_project_sync as wasm_scan_project_sync, transform_sync as wasm_transform_sync,
};

pub use manifest::{
    DecoratorManifestEntry, MacroManifest, MacroManifestEntry, debug_descriptors,
    debug_get_modules, debug_lookup, get_macro_manifest, get_macro_names, is_macro_package,
};

// Re-export internal items used by tests
#[cfg(test)]
pub(crate) use expand_core::{expand_inner, has_macro_annotations};

#[cfg(test)]
mod test;
