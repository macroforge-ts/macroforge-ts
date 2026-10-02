//! # Macroforge TypeScript Macro Engine
//!
//! This crate provides a TypeScript macro expansion engine that brings Rust-like derive macros
//! to TypeScript. It ships as a WebAssembly module (the default `wasm` feature, through
//! wasm-bindgen) and as the native `macroforge` CLI.
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
//! - **Unified API** (`api` module): `CoreEngine`, the output-agnostic facade the bindings
//!   delegate to.
//! - **Bindings** (`bindings_wasm`): the JavaScript entry points, through `wasm-bindgen`.
//! - **Position Mapping** (`api_types::SourceMappingResult`): Bidirectional source mapping
//!   for IDE integration.
//! - **Macro Host** (`host` module): Core expansion engine with registry and dispatcher.
//! - **Built-in Macros** (`builtin` module): Standard derive macros (Debug, Clone, Serialize, etc.).
//!
//! ## Usage
//!
//! The package is an ES module that instantiates its WebAssembly on import.
//!
//! ```javascript
//! import { expandSync } from '@macroforge/core';
//! const result = expandSync(code, filepath, { keepDecorators: false });
//! ```
//!
//! ## Re-exports for Macro Authors
//!
//! This crate re-exports several dependencies for convenience when writing custom macros:
//! - `ts_syn`: TypeScript syntax types for AST manipulation
//! - `macros`: Macro attributes and quote templates
//! - `ast`: Source-backed expression and identifier values for templates

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
pub extern crate serde_json;
#[cfg(feature = "wasm")]
pub extern crate serde_wasm_bindgen;
#[cfg(feature = "wasm")]
pub extern crate wasm_bindgen;

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
    // Re-export proc macro attributes
    pub use macroforge_ts_macros::{ts_macro, ts_macro_attribute, ts_macro_derive};

    // Re-export all quote macros
    pub use macroforge_ts_quote::{ts_quote, ts_template};
}

// ============================================================================
// Feature Dispatch Macros
// ============================================================================
// These macros allow generated code in dependent crates to react to the features
// enabled in macroforge_ts without needing to define those same features themselves.

#[cfg(feature = "wasm")]
#[macro_export]
macro_rules! if_wasm { ($($tokens:tt)*) => { $($tokens)* } }
#[cfg(not(feature = "wasm"))]
#[macro_export]
macro_rules! if_wasm {
    ($($tokens:tt)*) => {};
}

#[cfg(feature = "wasm")]
#[macro_export]
macro_rules! if_wasm_else {
    ($item:expr, $else:expr) => {
        $item
    };
}
#[cfg(not(feature = "wasm"))]
#[macro_export]
macro_rules! if_wasm_else {
    ($item:expr, $else:expr) => {
        $else
    };
}

pub use macroforge_ts_syn::ast;

// ============================================================================
// Internal modules
// ============================================================================
pub mod host;

// Re-export abi types from ts_syn
pub use ts_syn::abi;

pub mod builtin;

#[cfg(feature = "test-macros")]
pub mod test_macros;

// ============================================================================
// Extracted submodules
// ============================================================================
pub mod api;
pub mod api_types;
mod expand_core;
pub mod line_index;
pub mod workers;
pub use expand_core::has_macro_annotations;
pub use expand_core::macro_imports;
mod manifest;
pub mod package;
mod source_type;

#[cfg(feature = "wasm")]
pub mod bindings_wasm;

mod position_mapper;

// ============================================================================
// Public re-exports (preserving the original public API)
// ============================================================================
pub use api_types::{
    ExpandOptions, ExpandResult, GeneratedRegionResult, ImportSourceResult, JsDiagnostic,
    LoadConfigResult, MacroDiagnostic, MappingSegmentResult, ProcessFileOptions, ScanOptions,
    ScanResult, SourceMappingResult, SpanResult, SyntaxCheckResult,
};

pub use position_mapper::NativePositionMapper;

// ============================================================================
// C-ABI support for external macro loading
// ============================================================================
//
// Pointers and lengths only, never JS values: the CLI instantiates a macro
// package's wasm with wasmtime and calls these as ordinary exports, reading results
// out of the module's linear memory. The per-macro `__macroforge_ffi_run_*` are
// emitted by `macroforge_ts_macros`.

/// Free a buffer allocated by an FFI function.
/// Must be called by the host after reading the output.
///
/// # Safety
///
/// `ptr` must point to a valid allocation of `len` bytes previously returned by
/// an FFI function in this crate (e.g. `__macroforge_ffi_get_manifest`).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __macroforge_ffi_free(ptr: *mut u8, len: usize) {
    if !ptr.is_null() && len > 0 {
        drop(unsafe { Box::from_raw(std::ptr::slice_from_raw_parts_mut(ptr, len)) });
    }
}

/// Installs the type registry a host sends once, so the contexts it sends
/// afterwards refer to it instead of carrying it on every call.
///
/// Input: `payload_ptr`/`payload_len`, UTF-8 JSON of a
/// [`ts_syn::abi::ir::type_registry::ResidentRegistryPayload`]. Returns 0 on
/// success, with an empty `out_ptr`/`out_len`; otherwise 1, with the error
/// message there for the host to free through `__macroforge_ffi_free`.
///
/// # Safety
///
/// `payload_ptr` must point to `payload_len` readable bytes, and `out_ptr`
/// and `out_len` must be valid, non-null pointers to writable memory.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __macroforge_ffi_set_registry(
    payload_ptr: *const u8,
    payload_len: usize,
    out_ptr: *mut *mut u8,
    out_len: *mut usize,
) -> i32 {
    let payload = unsafe { std::slice::from_raw_parts(payload_ptr, payload_len) };
    let installed = std::str::from_utf8(payload)
        .map_err(|error| format!("the registry payload is not UTF-8: {error}"))
        .and_then(ts_syn::abi::ir::type_registry::install_resident_registry);
    match installed {
        Ok(()) => {
            unsafe {
                *out_len = 0;
                *out_ptr = std::ptr::null_mut();
            }
            0
        }
        Err(message) => {
            let message = message.into_bytes().into_boxed_slice();
            unsafe {
                *out_len = message.len();
                *out_ptr = Box::into_raw(message) as *mut u8;
            }
            1
        }
    }
}

/// Returns the full MacroManifest as JSON via FFI.
/// Linked into every macro package automatically since they depend on `macroforge_ts`.
/// Uses `inventory` to collect all `#[ts_macro_derive]` registrations in the crate.
///
/// # Safety
///
/// `out_ptr` and `out_len` must be valid, non-null pointers to writable memory.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __macroforge_ffi_get_manifest(
    out_ptr: *mut *mut u8,
    out_len: *mut usize,
) -> i32 {
    let manifest = get_macro_manifest();
    match serde_json::to_string(&manifest) {
        Ok(json) => {
            let bytes = json.into_bytes().into_boxed_slice();
            unsafe {
                *out_len = bytes.len();
                *out_ptr = Box::into_raw(bytes) as *mut u8;
            }
            0
        }
        Err(e) => {
            let msg = format!("Failed to serialize manifest: {e}")
                .into_bytes()
                .into_boxed_slice();
            unsafe {
                *out_len = msg.len();
                *out_ptr = Box::into_raw(msg) as *mut u8;
            }
            1
        }
    }
}

#[cfg(feature = "wasm")]
pub use bindings_wasm::{
    check_syntax as wasm_check_syntax, clear_config_cache as wasm_clear_config_cache,
    derive_decorator as wasm_derive_decorator, expand_sync as wasm_expand_sync,
    load_config as wasm_load_config, parse_import_sources as wasm_parse_import_sources,
    scan_project_sync as wasm_scan_project_sync,
};

pub use manifest::{
    DecoratorManifestEntry, MacroManifest, MacroManifestEntry, debug_descriptors,
    debug_get_modules, debug_lookup, get_macro_manifest, get_macro_names, is_macro_package,
};

#[cfg(test)]
mod test;
