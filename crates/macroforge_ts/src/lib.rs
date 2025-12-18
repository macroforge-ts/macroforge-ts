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

use napi::bindgen_prelude::*;
use napi_derive::napi;
use swc_core::{
    common::{FileName, GLOBALS, Globals, SourceMap, errors::Handler, sync::Lrc},
    ecma::{
        ast::{EsVersion, Program},
        codegen::{Emitter, text_writer::JsWriter},
        parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer},
    },
};

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

/// TypeScript syntax types for macro development
/// Use: `use macroforge_ts::ts_syn::*;`
pub use macroforge_ts_syn as ts_syn;

/// Macro attributes and quote templates
/// Use: `use macroforge_ts::macros::*;`
pub mod macros {
    // Re-export the ts_macro_derive attribute
    pub use macroforge_ts_macros::ts_macro_derive;

    // Re-export all quote macros
    pub use macroforge_ts_quote::{above, below, body, signature, ts_template};
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

use host::derived;
use host::CONFIG_CACHE;
use ts_syn::{Diagnostic, DiagnosticLevel};

pub mod builtin;

#[cfg(test)]
mod test;

use crate::host::MacroExpander;

// ============================================================================
// Data Structures
// ============================================================================

/// Result of transforming TypeScript code through the macro system.
///
/// This struct is returned by [`transform_sync`] and contains the transformed code
/// along with optional source maps, type declarations, and metadata about processed classes.
///
/// # Fields
///
/// * `code` - The transformed TypeScript/JavaScript code with macros expanded
/// * `map` - Optional source map for debugging (currently not implemented)
/// * `types` - Optional TypeScript type declarations for generated methods
/// * `metadata` - Optional JSON metadata about processed classes
#[napi(object)]
#[derive(Clone)]
pub struct TransformResult {
    /// The transformed TypeScript/JavaScript code with all macros expanded.
    pub code: String,
    /// Source map for mapping transformed positions back to original.
    /// Currently always `None` - source mapping is handled separately via `SourceMappingResult`.
    pub map: Option<String>,
    /// TypeScript type declarations (`.d.ts` content) for generated methods.
    /// Used by IDEs to provide type information for macro-generated code.
    pub types: Option<String>,
    /// JSON-serialized metadata about processed classes.
    /// Contains information about which classes were processed and what was generated.
    pub metadata: Option<String>,
}

/// A diagnostic message produced during macro expansion.
///
/// Diagnostics can represent errors, warnings, or informational messages
/// that occurred during the macro expansion process.
///
/// # Fields
///
/// * `level` - Severity level: "error", "warning", or "info"
/// * `message` - Human-readable description of the issue
/// * `start` - Optional byte offset where the issue starts in the source
/// * `end` - Optional byte offset where the issue ends in the source
///
/// # Example
///
/// ```rust
/// use macroforge_ts::MacroDiagnostic;
///
/// let _diag = MacroDiagnostic {
///     level: "error".to_string(),
///     message: "Unknown macro 'Foo'".to_string(),
///     start: Some(42),
///     end: Some(45),
/// };
/// ```
#[napi(object)]
#[derive(Clone)]
pub struct MacroDiagnostic {
    /// Severity level of the diagnostic.
    /// One of: "error", "warning", "info".
    pub level: String,
    /// Human-readable message describing the diagnostic.
    pub message: String,
    /// Byte offset in the original source where the issue starts.
    /// `None` if the diagnostic is not associated with a specific location.
    pub start: Option<u32>,
    /// Byte offset in the original source where the issue ends.
    /// `None` if the diagnostic is not associated with a specific location.
    pub end: Option<u32>,
}

/// A segment mapping a range in the original source to a range in the expanded source.
///
/// These segments form the core of the bidirectional source mapping system,
/// enabling IDE features like "go to definition" and error reporting to work
/// correctly with macro-expanded code.
///
/// # Invariants
///
/// - `original_start < original_end`
/// - `expanded_start < expanded_end`
/// - Segments are non-overlapping and sorted by position
#[napi(object)]
#[derive(Clone)]
pub struct MappingSegmentResult {
    /// Byte offset where this segment starts in the original source.
    pub original_start: u32,
    /// Byte offset where this segment ends in the original source.
    pub original_end: u32,
    /// Byte offset where this segment starts in the expanded source.
    pub expanded_start: u32,
    /// Byte offset where this segment ends in the expanded source.
    pub expanded_end: u32,
}

/// A region in the expanded source that was generated by a macro.
///
/// These regions identify code that has no corresponding location in the
/// original source because it was synthesized by a macro.
///
/// # Example
///
/// For a `@derive(Debug)` macro that generates a `toString()` method,
/// a `GeneratedRegionResult` would mark the entire method body as generated
/// with `source_macro = "Debug"`.
#[napi(object)]
#[derive(Clone)]
pub struct GeneratedRegionResult {
    /// Byte offset where the generated region starts in the expanded source.
    pub start: u32,
    /// Byte offset where the generated region ends in the expanded source.
    pub end: u32,
    /// Name of the macro that generated this region (e.g., "Debug", "Clone").
    pub source_macro: String,
}

/// Complete source mapping information for a macro expansion.
///
/// Contains both preserved segments (original code that wasn't modified)
/// and generated regions (new code synthesized by macros).
///
/// # Usage
///
/// This mapping enables:
/// - Converting positions from original source to expanded source and vice versa
/// - Identifying which macro generated a given piece of code
/// - Mapping IDE diagnostics from expanded code back to original source
#[napi(object)]
#[derive(Clone)]
pub struct SourceMappingResult {
    /// Segments mapping preserved regions between original and expanded source.
    /// Sorted by position for efficient binary search lookups.
    pub segments: Vec<MappingSegmentResult>,
    /// Regions in the expanded source that were generated by macros.
    /// Used to identify synthetic code with no original source location.
    pub generated_regions: Vec<GeneratedRegionResult>,
}

/// Result of expanding macros in TypeScript source code.
///
/// This is the primary return type for macro expansion operations,
/// containing the expanded code, diagnostics, and source mapping.
///
/// # Example
///
/// ```rust
/// use macroforge_ts::{ExpandResult, MacroDiagnostic};
///
/// // Create an ExpandResult programmatically
/// let result = ExpandResult {
///     code: "class User {}".to_string(),
///     types: None,
///     metadata: None,
///     diagnostics: vec![],
///     source_mapping: None,
/// };
///
/// // Check for errors
/// if result.diagnostics.iter().any(|d| d.level == "error") {
///     // Handle errors
/// }
/// ```
#[napi(object)]
#[derive(Clone)]
pub struct ExpandResult {
    /// The expanded TypeScript code with all macros processed.
    pub code: String,
    /// Optional TypeScript type declarations for generated methods.
    pub types: Option<String>,
    /// Optional JSON metadata about processed classes.
    pub metadata: Option<String>,
    /// Diagnostics (errors, warnings, info) from the expansion process.
    pub diagnostics: Vec<MacroDiagnostic>,
    /// Source mapping for position translation between original and expanded code.
    pub source_mapping: Option<SourceMappingResult>,
}

impl ExpandResult {
    /// Creates a no-op result with the original code unchanged.
    ///
    /// Used for early bailout when no macros need processing, avoiding
    /// unnecessary parsing and expansion overhead.
    ///
    /// # Arguments
    ///
    /// * `code` - The original source code to return unchanged
    ///
    /// # Returns
    ///
    /// An `ExpandResult` with:
    /// - `code` set to the input
    /// - All other fields empty/None
    /// - No source mapping (identity mapping implied)
    pub fn unchanged(code: &str) -> Self {
        Self {
            code: code.to_string(),
            types: None,
            metadata: None,
            diagnostics: vec![],
            source_mapping: None,
        }
    }
}

/// Information about an imported identifier from a TypeScript module.
///
/// Used to track where decorators and macro-related imports come from.
#[napi(object)]
#[derive(Clone)]
pub struct ImportSourceResult {
    /// Local identifier name in the import statement (e.g., `Derive` in `import { Derive }`).
    pub local: String,
    /// Module specifier this identifier was imported from (e.g., `"macroforge-ts"`).
    pub module: String,
}

/// Result of checking TypeScript syntax validity.
///
/// Returned by [`check_syntax`] to indicate whether code parses successfully.
#[napi(object)]
#[derive(Clone)]
pub struct SyntaxCheckResult {
    /// `true` if the code parsed without errors, `false` otherwise.
    pub ok: bool,
    /// Error message if parsing failed, `None` if successful.
    pub error: Option<String>,
}

/// A span (range) in source code, represented as start position and length.
///
/// Used for mapping diagnostics and other positional information.
#[napi(object)]
#[derive(Clone)]
pub struct SpanResult {
    /// Byte offset where the span starts.
    pub start: u32,
    /// Length of the span in bytes.
    pub length: u32,
}

/// A diagnostic from the TypeScript/JavaScript compiler or IDE.
///
/// This structure mirrors TypeScript's diagnostic format for interoperability
/// with language servers and IDEs.
#[napi(object)]
#[derive(Clone)]
pub struct JsDiagnostic {
    /// Byte offset where the diagnostic starts. `None` for global diagnostics.
    pub start: Option<u32>,
    /// Length of the diagnostic span in bytes.
    pub length: Option<u32>,
    /// Human-readable diagnostic message.
    pub message: Option<String>,
    /// TypeScript diagnostic code (e.g., 2304 for "Cannot find name").
    pub code: Option<u32>,
    /// Diagnostic category: "error", "warning", "suggestion", "message".
    pub category: Option<String>,
}

// ============================================================================
// Position Mapper (Optimized with Binary Search)
// ============================================================================

/// Bidirectional position mapper for translating between original and expanded source positions.
///
/// This mapper enables IDE features like error reporting, go-to-definition, and hover
/// to work correctly with macro-expanded code by translating positions between the
/// original source (what the user wrote) and the expanded source (what the compiler sees).
///
/// # Performance
///
/// Position lookups use binary search for O(log n) complexity, where n is the number
/// of mapping segments. This is critical for responsive IDE interactions.
///
/// # Example
///
/// ```javascript
/// const mapper = new PositionMapper(sourceMapping);
///
/// // Convert original position to expanded
/// const expandedPos = mapper.original_to_expanded(42);
///
/// // Convert expanded position back to original (if not in generated code)
/// const originalPos = mapper.expanded_to_original(100);
///
/// // Check if a position is in macro-generated code
/// if (mapper.is_in_generated(pos)) {
///     const macro = mapper.generated_by(pos); // e.g., "Debug"
/// }
/// ```
#[napi(js_name = "PositionMapper")]
pub struct NativePositionMapper {
    /// Mapping segments sorted by position for binary search.
    segments: Vec<MappingSegmentResult>,
    /// Regions marking code generated by macros.
    generated_regions: Vec<GeneratedRegionResult>,
}

/// Wrapper around `NativePositionMapper` for NAPI compatibility.
///
/// This provides the same functionality as `NativePositionMapper` but with a
/// different JavaScript class name. Used internally by [`NativePlugin::get_mapper`].
#[napi(js_name = "NativeMapper")]
pub struct NativeMapper {
    /// The underlying position mapper implementation.
    inner: NativePositionMapper,
}

#[napi]
impl NativePositionMapper {
    /// Creates a new position mapper from source mapping data.
    ///
    /// # Arguments
    ///
    /// * `mapping` - The source mapping result from macro expansion
    ///
    /// # Returns
    ///
    /// A new `NativePositionMapper` ready for position translation.
    #[napi(constructor)]
    pub fn new(mapping: SourceMappingResult) -> Self {
        Self {
            segments: mapping.segments,
            generated_regions: mapping.generated_regions,
        }
    }

    /// Checks if this mapper has no mapping data.
    ///
    /// An empty mapper indicates no transformations occurred, so position
    /// translation is an identity operation.
    ///
    /// # Returns
    ///
    /// `true` if there are no segments and no generated regions.
    #[napi(js_name = "isEmpty")]
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty() && self.generated_regions.is_empty()
    }

    /// Converts a position in the original source to the corresponding position in expanded source.
    ///
    /// Uses binary search for O(log n) lookup performance.
    ///
    /// # Arguments
    ///
    /// * `pos` - Byte offset in the original source
    ///
    /// # Returns
    ///
    /// The corresponding byte offset in the expanded source. If the position falls
    /// in a gap between segments, returns the position unchanged. If after the last
    /// segment, extrapolates based on the delta.
    ///
    /// # Algorithm
    ///
    /// 1. Binary search to find the segment containing or after `pos`
    /// 2. If inside a segment, compute offset within segment and translate
    /// 3. If after all segments, extrapolate from the last segment
    /// 4. Otherwise, return position unchanged (gap or before first segment)
    #[napi]
    pub fn original_to_expanded(&self, pos: u32) -> u32 {
        // Binary search to find the first segment where original_end > pos.
        // This gives us the segment that might contain pos, or the one after it.
        let idx = self.segments.partition_point(|seg| seg.original_end <= pos);

        if let Some(seg) = self.segments.get(idx) {
            // Check if pos is actually inside this segment (it might be in a gap)
            if pos >= seg.original_start && pos < seg.original_end {
                // Position is within this segment - calculate the offset and translate
                let offset = pos - seg.original_start;
                return seg.expanded_start + offset;
            }
        }

        // Handle case where position is after the last segment.
        // Extrapolate by adding the delta from the end of the last segment.
        if let Some(last) = self.segments.last()
            && pos >= last.original_end
        {
            let delta = pos - last.original_end;
            return last.expanded_end + delta;
        }

        // Fallback for positions before first segment or in gaps between segments.
        // Return unchanged as an identity mapping.
        pos
    }

    /// Converts a position in the expanded source back to the original source position.
    ///
    /// Returns `None` if the position is inside macro-generated code that has no
    /// corresponding location in the original source.
    ///
    /// # Arguments
    ///
    /// * `pos` - Byte offset in the expanded source
    ///
    /// # Returns
    ///
    /// `Some(original_pos)` if the position maps to original code,
    /// `None` if the position is in macro-generated code.
    #[napi]
    pub fn expanded_to_original(&self, pos: u32) -> Option<u32> {
        // First check if the position is in a generated region (no original mapping)
        if self.is_in_generated(pos) {
            return None;
        }

        // Binary search to find the segment containing or after this expanded position
        let idx = self.segments.partition_point(|seg| seg.expanded_end <= pos);

        if let Some(seg) = self.segments.get(idx)
            && pos >= seg.expanded_start
            && pos < seg.expanded_end
        {
            // Position is within this segment - translate back to original
            let offset = pos - seg.expanded_start;
            return Some(seg.original_start + offset);
        }

        // Handle extrapolation after the last segment
        if let Some(last) = self.segments.last()
            && pos >= last.expanded_end
        {
            let delta = pos - last.expanded_end;
            return Some(last.original_end + delta);
        }

        // Position doesn't map to any segment
        None
    }

    /// Returns the name of the macro that generated code at the given position.
    ///
    /// # Arguments
    ///
    /// * `pos` - Byte offset in the expanded source
    ///
    /// # Returns
    ///
    /// `Some(macro_name)` if the position is inside generated code (e.g., "Debug"),
    /// `None` if the position is in original (non-generated) code.
    #[napi]
    pub fn generated_by(&self, pos: u32) -> Option<String> {
        // Generated regions are typically small in number, so linear scan is acceptable.
        // If this becomes a bottleneck with many macros, could be optimized with binary search.
        self.generated_regions
            .iter()
            .find(|r| pos >= r.start && pos < r.end)
            .map(|r| r.source_macro.clone())
    }

    /// Maps a span (start + length) from expanded source to original source.
    ///
    /// # Arguments
    ///
    /// * `start` - Start byte offset in expanded source
    /// * `length` - Length of the span in bytes
    ///
    /// # Returns
    ///
    /// `Some(SpanResult)` with the mapped span in original source,
    /// `None` if either endpoint is in generated code.
    #[napi]
    pub fn map_span_to_original(&self, start: u32, length: u32) -> Option<SpanResult> {
        let end = start.saturating_add(length);
        // Both start and end must successfully map for the span to be valid
        let original_start = self.expanded_to_original(start)?;
        let original_end = self.expanded_to_original(end)?;

        Some(SpanResult {
            start: original_start,
            length: original_end.saturating_sub(original_start),
        })
    }

    /// Maps a span (start + length) from original source to expanded source.
    ///
    /// This always succeeds since every original position has an expanded equivalent.
    ///
    /// # Arguments
    ///
    /// * `start` - Start byte offset in original source
    /// * `length` - Length of the span in bytes
    ///
    /// # Returns
    ///
    /// A `SpanResult` with the mapped span in expanded source.
    #[napi]
    pub fn map_span_to_expanded(&self, start: u32, length: u32) -> SpanResult {
        let end = start.saturating_add(length);
        let expanded_start = self.original_to_expanded(start);
        let expanded_end = self.original_to_expanded(end);

        SpanResult {
            start: expanded_start,
            length: expanded_end.saturating_sub(expanded_start),
        }
    }

    /// Checks if a position is inside macro-generated code.
    ///
    /// # Arguments
    ///
    /// * `pos` - Byte offset in the expanded source
    ///
    /// # Returns
    ///
    /// `true` if the position is inside a generated region, `false` otherwise.
    #[napi]
    pub fn is_in_generated(&self, pos: u32) -> bool {
        self.generated_regions
            .iter()
            .any(|r| pos >= r.start && pos < r.end)
    }
}

#[napi]
impl NativeMapper {
    /// Creates a new mapper wrapping the given source mapping.
    ///
    /// # Arguments
    ///
    /// * `mapping` - The source mapping result from macro expansion
    #[napi(constructor)]
    pub fn new(mapping: SourceMappingResult) -> Self {
        Self {
            inner: NativePositionMapper::new(mapping),
        }
    }

    /// Checks if this mapper has no mapping data.
    #[napi(js_name = "isEmpty")]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Converts a position in the original source to expanded source.
    /// See [`NativePositionMapper::original_to_expanded`] for details.
    #[napi]
    pub fn original_to_expanded(&self, pos: u32) -> u32 {
        self.inner.original_to_expanded(pos)
    }

    /// Converts a position in the expanded source back to original.
    /// See [`NativePositionMapper::expanded_to_original`] for details.
    #[napi]
    pub fn expanded_to_original(&self, pos: u32) -> Option<u32> {
        self.inner.expanded_to_original(pos)
    }

    /// Returns the name of the macro that generated code at the given position.
    /// See [`NativePositionMapper::generated_by`] for details.
    #[napi]
    pub fn generated_by(&self, pos: u32) -> Option<String> {
        self.inner.generated_by(pos)
    }

    /// Maps a span from expanded source to original source.
    /// See [`NativePositionMapper::map_span_to_original`] for details.
    #[napi]
    pub fn map_span_to_original(&self, start: u32, length: u32) -> Option<SpanResult> {
        self.inner.map_span_to_original(start, length)
    }

    /// Maps a span from original source to expanded source.
    /// See [`NativePositionMapper::map_span_to_expanded`] for details.
    #[napi]
    pub fn map_span_to_expanded(&self, start: u32, length: u32) -> SpanResult {
        self.inner.map_span_to_expanded(start, length)
    }

    /// Checks if a position is inside macro-generated code.
    /// See [`NativePositionMapper::is_in_generated`] for details.
    #[napi]
    pub fn is_in_generated(&self, pos: u32) -> bool {
        self.inner.is_in_generated(pos)
    }
}

/// Checks if the given TypeScript code has valid syntax.
///
/// This function attempts to parse the code using SWC's TypeScript parser
/// without performing any macro expansion.
///
/// # Arguments
///
/// * `code` - The TypeScript source code to check
/// * `filepath` - The file path (used to determine if it's TSX based on extension)
///
/// # Returns
///
/// A [`SyntaxCheckResult`] indicating success or containing the parse error.
///
/// # Example
///
/// ```javascript
/// const result = check_syntax("const x: number = 42;", "test.ts");
/// if (!result.ok) {
///     console.error("Syntax error:", result.error);
/// }
/// ```
#[napi]
pub fn check_syntax(code: String, filepath: String) -> SyntaxCheckResult {
    match parse_program(&code, &filepath) {
        Ok(_) => SyntaxCheckResult {
            ok: true,
            error: None,
        },
        Err(err) => SyntaxCheckResult {
            ok: false,
            error: Some(err.to_string()),
        },
    }
}

// ============================================================================
// Core Plugin Logic
// ============================================================================

/// Options for processing a file through the macro system.
///
/// Used by [`NativePlugin::process_file`] to configure expansion behavior
/// and caching.
#[napi(object)]
pub struct ProcessFileOptions {
    /// If `true`, preserves `@derive` decorators in the output.
    /// If `false` (default), decorators are stripped after expansion.
    pub keep_decorators: Option<bool>,
    /// Version string for cache invalidation.
    /// When provided, cached results are only reused if versions match.
    pub version: Option<String>,
    /// Additional decorator module names from external macros.
    /// See [`ExpandOptions::external_decorator_modules`] for details.
    pub external_decorator_modules: Option<Vec<String>>,
    /// Path to a previously loaded config file (for foreign types lookup).
    /// See [`ExpandOptions::config_path`] for details.
    pub config_path: Option<String>,
}

/// Options for macro expansion.
///
/// Used by [`expand_sync`] to configure expansion behavior.
#[napi(object)]
pub struct ExpandOptions {
    /// If `true`, preserves `@derive` decorators in the output.
    /// If `false` (default), decorators are stripped after expansion.
    pub keep_decorators: Option<bool>,

    /// Additional decorator module names from external macros.
    ///
    /// These are used during decorator stripping to identify Macroforge-specific
    /// decorators that should be removed from the output. Built-in decorator modules
    /// (like "serde", "debug") are automatically included.
    ///
    /// External macro packages should export their decorator module names, which
    /// plugins can collect and pass here.
    ///
    /// # Example
    ///
    /// ```javascript
    /// expandSync(code, filepath, {
    ///   keepDecorators: false,
    ///   externalDecoratorModules: ["myMacro", "customValidator"]
    /// });
    /// ```
    pub external_decorator_modules: Option<Vec<String>>,

    /// Path to a previously loaded config file.
    ///
    /// When provided, the expansion will use the cached configuration
    /// (including foreign types) from this path. The config must have been
    /// previously loaded via [`load_config`].
    ///
    /// # Example
    ///
    /// ```javascript
    /// // First, load the config
    /// const configResult = loadConfig(configContent, configPath);
    ///
    /// // Then use it during expansion
    /// expandSync(code, filepath, { configPath });
    /// ```
    pub config_path: Option<String>,
}

/// The main plugin class for macro expansion with caching support.
///
/// `NativePlugin` is designed to be instantiated once and reused across multiple
/// file processing operations. It maintains a cache of expansion results keyed
/// by filepath and version, enabling efficient incremental processing.
///
/// # Thread Safety
///
/// The plugin is thread-safe through the use of `Mutex` for internal state.
/// However, macro expansion itself runs in a separate thread with a 32MB stack
/// to prevent stack overflow during deep AST recursion.
///
/// # Example
///
/// ```javascript
/// // Create a single plugin instance (typically at startup)
/// const plugin = new NativePlugin();
///
/// // Process files with caching
/// const result1 = plugin.process_file("src/foo.ts", code1, { version: "1" });
/// const result2 = plugin.process_file("src/foo.ts", code2, { version: "1" }); // Cache hit!
/// const result3 = plugin.process_file("src/foo.ts", code3, { version: "2" }); // Cache miss
///
/// // Get a mapper for position translation
/// const mapper = plugin.get_mapper("src/foo.ts");
/// ```
#[napi]
pub struct NativePlugin {
    /// Cache of expansion results, keyed by filepath.
    /// Protected by a mutex for thread-safe access.
    cache: std::sync::Mutex<std::collections::HashMap<String, CachedResult>>,
    /// Optional path to a log file for debugging.
    /// Protected by a mutex for thread-safe access.
    log_file: std::sync::Mutex<Option<std::path::PathBuf>>,
}

impl Default for NativePlugin {
    fn default() -> Self {
        Self::new()
    }
}

/// Internal structure for cached expansion results.
#[derive(Clone)]
struct CachedResult {
    /// Version string at the time of caching.
    /// Used for cache invalidation when version changes.
    version: Option<String>,
    /// The cached expansion result.
    result: ExpandResult,
}

/// Converts `ProcessFileOptions` to `ExpandOptions`.
///
/// Extracts only the options relevant for expansion, discarding cache-related
/// options like `version`.
fn option_expand_options(opts: Option<ProcessFileOptions>) -> Option<ExpandOptions> {
    opts.map(|o| ExpandOptions {
        keep_decorators: o.keep_decorators,
        external_decorator_modules: o.external_decorator_modules,
        config_path: o.config_path,
    })
}

#[napi]
impl NativePlugin {
    /// Creates a new `NativePlugin` instance.
    ///
    /// Initializes the plugin with an empty cache and sets up a default log file
    /// at `/tmp/macroforge-plugin.log` for debugging purposes.
    ///
    /// # Returns
    ///
    /// A new `NativePlugin` ready for processing files.
    ///
    /// # Side Effects
    ///
    /// Creates or clears the log file at `/tmp/macroforge-plugin.log`.
    #[napi(constructor)]
    pub fn new() -> Self {
        let plugin = Self {
            cache: std::sync::Mutex::new(std::collections::HashMap::new()),
            log_file: std::sync::Mutex::new(None),
        };

        // Initialize log file with default path for debugging.
        // This is useful for diagnosing issues in production environments
        // where console output might not be easily accessible.
        if let Ok(mut log_guard) = plugin.log_file.lock() {
            let log_path = std::path::PathBuf::from("/tmp/macroforge-plugin.log");

            // Clear/create log file to start fresh on each plugin instantiation
            if let Err(e) = std::fs::write(&log_path, "=== macroforge plugin loaded ===\n") {
                eprintln!("[macroforge] Failed to initialize log file: {}", e);
            } else {
                *log_guard = Some(log_path);
            }
        }

        plugin
    }

    /// Writes a message to the plugin's log file.
    ///
    /// Useful for debugging macro expansion issues in production environments.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log
    ///
    /// # Note
    ///
    /// Messages are appended to the log file. If the log file hasn't been
    /// configured or cannot be written to, the message is silently dropped.
    #[napi]
    pub fn log(&self, message: String) {
        if let Ok(log_guard) = self.log_file.lock()
            && let Some(log_path) = log_guard.as_ref()
        {
            use std::io::Write;
            if let Ok(mut file) = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(log_path)
            {
                let _ = writeln!(file, "{}", message);
            }
        }
    }

    /// Sets the path for the plugin's log file.
    ///
    /// # Arguments
    ///
    /// * `path` - The file path to use for logging
    ///
    /// # Note
    ///
    /// This does not create the file; it will be created when the first
    /// message is logged.
    #[napi]
    pub fn set_log_file(&self, path: String) {
        if let Ok(mut log_guard) = self.log_file.lock() {
            *log_guard = Some(std::path::PathBuf::from(path));
        }
    }

    /// Processes a TypeScript file through the macro expansion system.
    ///
    /// This is the main entry point for file processing. It handles caching,
    /// thread isolation (to prevent stack overflow), and error recovery.
    ///
    /// # Arguments
    ///
    /// * `_env` - NAPI environment (unused but required by NAPI)
    /// * `filepath` - Path to the file (used for TSX detection and caching)
    /// * `code` - The TypeScript source code to process
    /// * `options` - Optional configuration for expansion and caching
    ///
    /// # Returns
    ///
    /// An [`ExpandResult`] containing the expanded code, diagnostics, and source mapping.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Thread spawning fails
    /// - The worker thread panics (often due to stack overflow)
    /// - Macro expansion fails internally
    ///
    /// # Performance
    ///
    /// - Uses a 32MB thread stack to prevent stack overflow during deep AST recursion
    /// - Caches results by filepath and version for efficient incremental processing
    /// - Early bailout for files without `@derive` decorators
    ///
    /// # Thread Safety
    ///
    /// Macro expansion runs in a separate thread because:
    /// 1. SWC AST operations can be deeply recursive, exceeding default stack limits
    /// 2. Node.js thread stack is typically only 2MB
    /// 3. Panics in the worker thread are caught and reported gracefully
    #[napi]
    pub fn process_file(
        &self,
        _env: Env,
        filepath: String,
        code: String,
        options: Option<ProcessFileOptions>,
    ) -> Result<ExpandResult> {
        let version = options.as_ref().and_then(|o| o.version.clone());

        // Cache Check: Return cached result if version matches.
        // This enables efficient incremental processing when files haven't changed.
        if let (Some(ver), Ok(guard)) = (version.as_ref(), self.cache.lock())
            && let Some(cached) = guard.get(&filepath)
            && cached.version.as_ref() == Some(ver)
        {
            return Ok(cached.result.clone());
        }

        // Run expansion in a separate thread with a LARGE stack (32MB).
        // Standard threads (and Node threads) often have 2MB stacks, which causes
        // "Broken pipe" / SEGFAULTS when SWC recurses deeply in macros.
        let opts_clone = option_expand_options(options);
        let filepath_for_thread = filepath.clone();

        let builder = std::thread::Builder::new().stack_size(32 * 1024 * 1024);
        let handle = builder
            .spawn(move || {
                // Set up SWC globals for this thread.
                // SWC uses thread-local storage for some operations.
                let globals = Globals::default();
                GLOBALS.set(&globals, || {
                    // Catch panics to report them gracefully instead of crashing.
                    // Common cause: stack overflow from deeply nested AST.
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        // Note: NAPI Env is NOT thread safe - we cannot pass it.
                        // The expand_inner function uses pure Rust AST operations.
                        expand_inner(&code, &filepath_for_thread, opts_clone)
                    }))
                })
            })
            .map_err(|e| {
                Error::new(
                    Status::GenericFailure,
                    format!("Failed to spawn worker thread: {}", e),
                )
            })?;

        // Wait for the worker thread and unwrap nested Results.
        // Result structure: join() -> panic catch -> expand_inner -> final result
        let expand_result = handle
            .join()
            .map_err(|_| {
                Error::new(
                    Status::GenericFailure,
                    "Macro expansion worker thread panicked (Stack Overflow?)",
                )
            })?
            .map_err(|_| {
                Error::new(
                    Status::GenericFailure,
                    "Macro expansion panicked inside worker",
                )
            })??;

        // Update Cache: Store the result for future requests with the same version.
        if let Ok(mut guard) = self.cache.lock() {
            guard.insert(
                filepath.clone(),
                CachedResult {
                    version,
                    result: expand_result.clone(),
                },
            );
        }

        Ok(expand_result)
    }

    /// Retrieves a position mapper for a previously processed file.
    ///
    /// The mapper enables translation between original and expanded source positions,
    /// which is essential for IDE features like error reporting and navigation.
    ///
    /// # Arguments
    ///
    /// * `filepath` - Path to the file (must have been previously processed)
    ///
    /// # Returns
    ///
    /// `Some(NativeMapper)` if the file has been processed and has source mapping data,
    /// `None` if the file hasn't been processed or has no mapping (no macros expanded).
    #[napi]
    pub fn get_mapper(&self, filepath: String) -> Option<NativeMapper> {
        let mapping = match self.cache.lock() {
            Ok(guard) => guard
                .get(&filepath)
                .cloned()
                .and_then(|c| c.result.source_mapping),
            Err(_) => None,
        };

        mapping.map(|m| NativeMapper {
            inner: NativePositionMapper::new(m),
        })
    }

    /// Maps diagnostics from expanded source positions back to original source positions.
    ///
    /// This is used by IDE integrations to show errors at the correct locations
    /// in the user's original code, rather than in the macro-expanded output.
    ///
    /// # Arguments
    ///
    /// * `filepath` - Path to the file the diagnostics are for
    /// * `diags` - Diagnostics with positions in the expanded source
    ///
    /// # Returns
    ///
    /// Diagnostics with positions mapped back to the original source.
    /// If no mapper is available for the file, returns diagnostics unchanged.
    #[napi]
    pub fn map_diagnostics(&self, filepath: String, diags: Vec<JsDiagnostic>) -> Vec<JsDiagnostic> {
        let Some(mapper) = self.get_mapper(filepath) else {
            // No mapper available - return diagnostics unchanged
            return diags;
        };

        diags
            .into_iter()
            .map(|mut d| {
                // Attempt to map the diagnostic span back to original source
                if let (Some(start), Some(length)) = (d.start, d.length)
                    && let Some(mapped) = mapper.map_span_to_original(start, length)
                {
                    d.start = Some(mapped.start);
                    d.length = Some(mapped.length);
                }
                // Note: Diagnostics in generated code cannot be mapped and keep
                // their original (expanded) positions
                d
            })
            .collect()
    }
}

// ============================================================================
// Sync Functions (Refactored for Thread Safety & Performance)
// ============================================================================

/// Parses import statements from TypeScript code and returns their sources.
///
/// This function extracts information about all import statements in the code,
/// mapping each imported identifier to its source module. Useful for analyzing
/// dependencies and understanding where decorators come from.
///
/// # Arguments
///
/// * `code` - The TypeScript source code to parse
/// * `filepath` - The file path (used for TSX detection)
///
/// # Returns
///
/// A vector of [`ImportSourceResult`] entries, one for each imported identifier.
///
/// # Errors
///
/// Returns an error if the code cannot be parsed.
///
/// # Example
///
/// ```javascript
/// // For code: import { Derive, Clone } from "macroforge-ts";
/// const imports = parse_import_sources(code, "test.ts");
/// // Returns: [
/// //   { local: "Derive", module: "macroforge-ts" },
/// //   { local: "Clone", module: "macroforge-ts" }
/// // ]
/// ```
#[napi]
pub fn parse_import_sources(code: String, filepath: String) -> Result<Vec<ImportSourceResult>> {
    let (program, _cm) = parse_program(&code, &filepath)?;
    let module = match program {
        Program::Module(module) => module,
        // Scripts don't have import statements
        Program::Script(_) => return Ok(vec![]),
    };

    let import_result = crate::host::collect_import_sources(&module, &code);
    let mut imports = Vec::with_capacity(import_result.sources.len());
    for (local, module) in import_result.sources {
        imports.push(ImportSourceResult { local, module });
    }
    Ok(imports)
}

/// The `@Derive` decorator function exported to JavaScript/TypeScript.
///
/// This is a no-op function that exists purely for TypeScript type checking.
/// The actual decorator processing happens during macro expansion, where
/// `@derive(...)` decorators are recognized and transformed.
///
/// # TypeScript Usage
///
/// ```typescript
/// import { Derive } from "macroforge-ts";
///
/// @Derive(Debug, Clone, Serialize)
/// class User {
///     name: string;
///     email: string;
/// }
/// ```
#[napi(
    js_name = "Derive",
    ts_return_type = "ClassDecorator",
    ts_args_type = "...features: any[]"
)]
pub fn derive_decorator() {}

/// Result of loading a macroforge configuration file.
///
/// Returned by [`load_config`] after parsing a `macroforge.config.js/ts` file.
#[napi(object)]
pub struct LoadConfigResult {
    /// Whether to preserve `@derive` decorators in the output code.
    pub keep_decorators: bool,
    /// Whether to generate a convenience const for non-class types.
    pub generate_convenience_const: bool,
    /// Whether the config has any foreign type handlers defined.
    pub has_foreign_types: bool,
    /// Number of foreign types configured.
    pub foreign_type_count: u32,
}

/// Load and parse a macroforge configuration file.
///
/// Parses a `macroforge.config.js/ts` file and caches the result for use
/// during macro expansion. The configuration includes both simple settings
/// (like `keepDecorators`) and foreign type handlers.
///
/// # Arguments
///
/// * `content` - The raw content of the configuration file
/// * `filepath` - Path to the configuration file (used to determine syntax and as cache key)
///
/// # Returns
///
/// A [`LoadConfigResult`] containing the parsed configuration summary.
///
/// # Example
///
/// ```javascript
/// import { loadConfig, expandSync } from 'macroforge';
/// import fs from 'fs';
///
/// const configPath = 'macroforge.config.js';
/// const configContent = fs.readFileSync(configPath, 'utf-8');
///
/// // Load and cache the configuration
/// const result = loadConfig(configContent, configPath);
/// console.log(`Loaded config with ${result.foreignTypeCount} foreign types`);
///
/// // The config is now cached and will be used by expandSync
/// const expanded = expandSync(code, filepath, { configPath });
/// ```
#[napi]
pub fn load_config(content: String, filepath: String) -> Result<LoadConfigResult> {
    use crate::host::MacroforgeConfig;

    let config = MacroforgeConfig::load_and_cache(&content, &filepath).map_err(|e| {
        Error::new(
            Status::GenericFailure,
            format!("Failed to parse config: {}", e),
        )
    })?;

    Ok(LoadConfigResult {
        keep_decorators: config.keep_decorators,
        generate_convenience_const: config.generate_convenience_const,
        has_foreign_types: !config.foreign_types.is_empty(),
        foreign_type_count: config.foreign_types.len() as u32,
    })
}

/// Clears the configuration cache.
///
/// This is useful for testing to ensure each test starts with a clean state.
/// In production, clearing the cache will force configs to be re-parsed on next access.
///
/// # Example
///
/// ```javascript
/// const { clearConfigCache, loadConfig } = require('macroforge-ts');
///
/// // Clear cache before each test
/// clearConfigCache();
///
/// // Now load a fresh config
/// const result = loadConfig(configContent, configPath);
/// ```
#[napi]
pub fn clear_config_cache() {
    crate::host::clear_config_cache();
}

/// Synchronously transforms TypeScript code through the macro expansion system.
///
/// This is similar to [`expand_sync`] but returns a [`TransformResult`] which
/// includes source map information (when available).
///
/// # Arguments
///
/// * `_env` - NAPI environment (unused but required by NAPI)
/// * `code` - The TypeScript source code to transform
/// * `filepath` - The file path (used for TSX detection)
///
/// # Returns
///
/// A [`TransformResult`] containing the transformed code and metadata.
///
/// # Errors
///
/// Returns an error if:
/// - Thread spawning fails
/// - The worker thread panics
/// - Macro expansion fails
///
/// # Thread Safety
///
/// Uses a 32MB thread stack to prevent stack overflow during deep AST recursion.
#[napi]
pub fn transform_sync(_env: Env, code: String, filepath: String) -> Result<TransformResult> {
    // Run in a separate thread with large stack for deep AST recursion
    let builder = std::thread::Builder::new().stack_size(32 * 1024 * 1024);
    let handle = builder
        .spawn(move || {
            let globals = Globals::default();
            GLOBALS.set(&globals, || {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    transform_inner(&code, &filepath)
                }))
            })
        })
        .map_err(|e| {
            Error::new(
                Status::GenericFailure,
                format!("Failed to spawn transform thread: {}", e),
            )
        })?;

    handle
        .join()
        .map_err(|_| Error::new(Status::GenericFailure, "Transform worker crashed"))?
        .map_err(|_| Error::new(Status::GenericFailure, "Transform panicked"))?
}

/// Synchronously expands macros in TypeScript code.
///
/// This is the standalone macro expansion function that doesn't use caching.
/// For cached expansion, use [`NativePlugin::process_file`] instead.
///
/// # Arguments
///
/// * `_env` - NAPI environment (unused but required by NAPI)
/// * `code` - The TypeScript source code to expand
/// * `filepath` - The file path (used for TSX detection)
/// * `options` - Optional configuration (e.g., `keep_decorators`)
///
/// # Returns
///
/// An [`ExpandResult`] containing the expanded code, diagnostics, and source mapping.
///
/// # Errors
///
/// Returns an error if:
/// - Thread spawning fails
/// - The worker thread panics
/// - Macro host initialization fails
///
/// # Performance
///
/// - Uses a 32MB thread stack to prevent stack overflow
/// - Performs early bailout for files without `@derive` decorators
///
/// # Example
///
/// ```javascript
/// const result = expand_sync(env, code, "user.ts", { keep_decorators: false });
/// console.log(result.code); // Expanded TypeScript code
/// console.log(result.diagnostics); // Any warnings or errors
/// ```
#[napi]
pub fn expand_sync(
    _env: Env,
    code: String,
    filepath: String,
    options: Option<ExpandOptions>,
) -> Result<ExpandResult> {
    // Run in a separate thread with large stack for deep AST recursion
    let builder = std::thread::Builder::new().stack_size(32 * 1024 * 1024);
    let handle = builder
        .spawn(move || {
            let globals = Globals::default();
            GLOBALS.set(&globals, || {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    expand_inner(&code, &filepath, options)
                }))
            })
        })
        .map_err(|e| {
            Error::new(
                Status::GenericFailure,
                format!("Failed to spawn expand thread: {}", e),
            )
        })?;

    handle
        .join()
        .map_err(|_| Error::new(Status::GenericFailure, "Expand worker crashed"))?
        .map_err(|_| Error::new(Status::GenericFailure, "Expand panicked"))?
}

// ============================================================================
// Inner Logic (Optimized)
// ============================================================================

/// Core macro expansion logic, decoupled from NAPI Env to allow threading.
///
/// This function contains the actual expansion implementation and is called
/// from a separate thread with a large stack size to prevent stack overflow.
///
/// # Arguments
///
/// * `code` - The TypeScript source code to expand
/// * `filepath` - The file path (used for TSX detection and error reporting)
/// * `options` - Optional expansion configuration
///
/// # Returns
///
/// An [`ExpandResult`] containing the expanded code and metadata.
///
/// # Errors
///
/// Returns an error if:
/// - The macro host fails to initialize
/// - Macro expansion fails internally
///
/// # Algorithm
///
/// 1. **Early bailout**: If code doesn't contain `@derive`, return unchanged
/// 2. **Parse**: Convert code to SWC AST
/// 3. **Expand**: Run all registered macros on decorated classes
/// 4. **Collect**: Gather diagnostics and source mapping
/// 5. **Post-process**: Inject type declarations for generated methods
fn expand_inner(
    code: &str,
    filepath: &str,
    options: Option<ExpandOptions>,
) -> Result<ExpandResult> {
    // Early bailout: Skip files without @derive decorator.
    // This optimization avoids expensive parsing for files that don't use macros
    // and prevents issues with Svelte runes ($state, $derived, etc.) that use
    // similar syntax but aren't macroforge decorators.
    if !code.contains("@derive") {
        return Ok(ExpandResult::unchanged(code));
    }

    // Create a new macro host for this thread.
    // Each thread needs its own MacroExpander because the expansion process
    // is stateful and cannot be safely shared across threads.
    let mut macro_host = MacroExpander::new().map_err(|err| {
        Error::new(
            Status::GenericFailure,
            format!("Failed to initialize macro host: {err:?}"),
        )
    })?;

    // Apply options if provided
    if let Some(ref opts) = options {
        if let Some(keep) = opts.keep_decorators {
            macro_host.set_keep_decorators(keep);
        }
        if let Some(ref modules) = opts.external_decorator_modules {
            macro_host.set_external_decorator_modules(modules.clone());
        }
    }

    // Set up foreign types and config imports from config if available
    let config_path = options.as_ref().and_then(|o| o.config_path.as_ref());
    if let Some(path) = config_path
        && let Some(config) = CONFIG_CACHE.get(path)
    {
        crate::builtin::serde::set_foreign_types(config.foreign_types.clone());
        // Convert ImportInfo to just module source strings for the serde module
        let config_imports: std::collections::HashMap<String, String> = config
            .config_imports
            .iter()
            .map(|(name, info)| (name.clone(), info.source.clone()))
            .collect();
        crate::builtin::serde::set_config_imports(config_imports);
    }

    // Parse the code into an AST.
    // On parse errors, we return a graceful "no-op" result instead of failing,
    // because parse errors can happen frequently during typing in an IDE.
    let (program, _) = match parse_program(code, filepath) {
        Ok(p) => p,
        Err(e) => {
            let error_msg = e.to_string();

            // Clean up foreign types and config imports before returning
            crate::builtin::serde::clear_foreign_types();
            crate::builtin::serde::clear_import_sources();
            crate::builtin::serde::clear_config_imports();

            // Return a "no-op" expansion result: original code unchanged,
            // with an informational diagnostic explaining why.
            // This allows the language server to continue functioning smoothly.
            return Ok(ExpandResult {
                code: code.to_string(),
                types: None,
                metadata: None,
                diagnostics: vec![MacroDiagnostic {
                    level: "info".to_string(),
                    message: format!("Macro expansion skipped due to syntax error: {}", error_msg),
                    start: None,
                    end: None,
                }],
                source_mapping: None,
            });
        }
    };

    // Extract import sources, aliases, and type-only status for foreign type validation
    let (import_sources, import_aliases, type_only_imports) = extract_import_sources(&program);
    crate::builtin::serde::set_import_sources(import_sources);
    crate::builtin::serde::set_import_aliases(import_aliases);
    crate::builtin::serde::set_type_only_imports(type_only_imports);

    // Run macro expansion on the parsed AST
    let expansion_result = macro_host.expand(code, &program, filepath);

    // Clean up all thread-local state after expansion (before error propagation)
    crate::builtin::serde::clear_foreign_types();
    crate::builtin::serde::clear_import_sources();
    crate::builtin::serde::clear_import_aliases();
    crate::builtin::serde::clear_type_only_imports();
    crate::builtin::serde::clear_required_namespace_imports();
    crate::builtin::serde::clear_config_imports();

    // Now propagate any error
    let expansion = expansion_result.map_err(|err| {
        Error::new(
            Status::GenericFailure,
            format!("Macro expansion failed: {err:?}"),
        )
    })?;

    // Convert internal diagnostics to NAPI-compatible format
    let diagnostics = expansion
        .diagnostics
        .into_iter()
        .map(|d| MacroDiagnostic {
            level: format!("{:?}", d.level).to_lowercase(),
            message: d.message,
            start: d.span.map(|s| s.start),
            end: d.span.map(|s| s.end),
        })
        .collect();

    // Convert internal source mapping to NAPI-compatible format
    let source_mapping = expansion.source_mapping.map(|mapping| SourceMappingResult {
        segments: mapping
            .segments
            .into_iter()
            .map(|seg| MappingSegmentResult {
                original_start: seg.original_start,
                original_end: seg.original_end,
                expanded_start: seg.expanded_start,
                expanded_end: seg.expanded_end,
            })
            .collect(),
        generated_regions: mapping
            .generated_regions
            .into_iter()
            .map(|region| GeneratedRegionResult {
                start: region.start,
                end: region.end,
                source_macro: region.source_macro,
            })
            .collect(),
    });

    // Post-process type declarations.
    // Heuristic fix: If the expanded code contains toJSON() but the type
    // declarations don't, inject the type signature. This ensures IDEs
    // provide proper type information for serialized objects.
    let mut types_output = expansion.type_output;
    if let Some(types) = &mut types_output
        && expansion.code.contains("toJSON(")
        && !types.contains("toJSON(")
    {
        // Find the last closing brace and insert before it.
        // This is a heuristic that works for simple cases.
        if let Some(insert_at) = types.rfind('}') {
            types.insert_str(insert_at, "  toJSON(): Record<string, unknown>;\n");
        }
    }

    Ok(ExpandResult {
        code: expansion.code,
        types: types_output,
        // Only include metadata if there were classes processed
        metadata: if expansion.classes.is_empty() {
            None
        } else {
            serde_json::to_string(&expansion.classes).ok()
        },
        diagnostics,
        source_mapping,
    })
}

/// Core transform logic, decoupled from NAPI Env to allow threading.
///
/// Similar to [`expand_inner`] but returns a [`TransformResult`] and fails
/// on any error-level diagnostics.
///
/// # Arguments
///
/// * `code` - The TypeScript source code to transform
/// * `filepath` - The file path (used for TSX detection)
///
/// # Returns
///
/// A [`TransformResult`] containing the transformed code.
///
/// # Errors
///
/// Returns an error if:
/// - The macro host fails to initialize
/// - Parsing fails
/// - Expansion fails
/// - Any error-level diagnostic is emitted
fn transform_inner(code: &str, filepath: &str) -> Result<TransformResult> {
    let macro_host = MacroExpander::new().map_err(|err| {
        Error::new(
            Status::GenericFailure,
            format!("Failed to init host: {err:?}"),
        )
    })?;

    let (program, cm) = parse_program(code, filepath)?;

    let expansion = macro_host
        .expand(code, &program, filepath)
        .map_err(|err| Error::new(Status::GenericFailure, format!("Expansion failed: {err:?}")))?;

    // Unlike expand_inner, transform_inner treats errors as fatal
    handle_macro_diagnostics(&expansion.diagnostics, filepath)?;

    // Optimization: Only re-emit if we didn't change anything.
    // If expansion.changed is true, we already have the string from the expander.
    // Otherwise, emit the original AST to string (no changes made).
    let generated = if expansion.changed {
        expansion.code
    } else {
        emit_program(&program, &cm)?
    };

    let metadata = if expansion.classes.is_empty() {
        None
    } else {
        serde_json::to_string(&expansion.classes).ok()
    };

    Ok(TransformResult {
        code: generated,
        map: None, // Source mapping handled separately via SourceMappingResult
        types: expansion.type_output,
        metadata,
    })
}

/// Parses TypeScript source code into an SWC AST.
///
/// # Arguments
///
/// * `code` - The TypeScript source code
/// * `filepath` - The file path (used to determine TSX mode from extension)
///
/// # Returns
///
/// A tuple of `(Program, SourceMap)` on success.
///
/// # Errors
///
/// Returns an error if the code contains syntax errors.
///
/// # Configuration
///
/// - TSX mode is enabled for `.tsx` files
/// - Decorators are always enabled
/// - Uses latest ES version
/// - `no_early_errors` is enabled for better error recovery
fn parse_program(code: &str, filepath: &str) -> Result<(Program, Lrc<SourceMap>)> {
    let cm: Lrc<SourceMap> = Lrc::new(SourceMap::default());
    let fm = cm.new_source_file(
        FileName::Custom(filepath.to_string()).into(),
        code.to_string(),
    );
    // Create a handler that captures errors to a buffer (we don't use its output directly)
    let handler =
        Handler::with_emitter_writer(Box::new(std::io::Cursor::new(Vec::new())), Some(cm.clone()));

    // Configure the lexer for TypeScript with decorator support
    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax {
            tsx: filepath.ends_with(".tsx"), // Enable TSX for .tsx files
            decorators: true,                // Required for @derive decorators
            dts: false,                      // Not parsing .d.ts files
            no_early_errors: true,           // Better error recovery during typing
            ..Default::default()
        }),
        EsVersion::latest(),
        StringInput::from(&*fm),
        None, // No comments collection
    );

    let mut parser = Parser::new_from(lexer);
    match parser.parse_program() {
        Ok(program) => Ok((program, cm)),
        Err(error) => {
            // Format and emit the error for debugging purposes
            let msg = format!("Failed to parse TypeScript: {:?}", error);
            error.into_diagnostic(&handler).emit();
            Err(Error::new(Status::GenericFailure, msg))
        }
    }
}

/// Emits an SWC AST back to JavaScript/TypeScript source code.
///
/// # Arguments
///
/// * `program` - The AST to emit
/// * `cm` - The source map (used for line/column tracking)
///
/// # Returns
///
/// The generated source code as a string.
///
/// # Errors
///
/// Returns an error if code generation fails (rare).
fn emit_program(program: &Program, cm: &Lrc<SourceMap>) -> Result<String> {
    let mut buf = vec![];
    let mut emitter = Emitter {
        cfg: swc_core::ecma::codegen::Config::default(),
        cm: cm.clone(),
        comments: None,
        wr: Box::new(JsWriter::new(cm.clone(), "\n", &mut buf, None)),
    };
    emitter
        .emit_program(program)
        .map_err(|e| Error::new(Status::GenericFailure, format!("{:?}", e)))?;
    Ok(String::from_utf8_lossy(&buf).to_string())
}

/// Checks diagnostics for errors and returns the first error as a Result.
///
/// This is used by [`transform_inner`] which treats errors as fatal,
/// unlike [`expand_inner`] which allows non-fatal errors in diagnostics.
///
/// # Arguments
///
/// * `diags` - The diagnostics to check
/// * `file` - The file path for error location reporting
///
/// # Returns
///
/// `Ok(())` if no errors, `Err` with the first error message otherwise.
fn handle_macro_diagnostics(diags: &[Diagnostic], file: &str) -> Result<()> {
    for diag in diags {
        if matches!(diag.level, DiagnosticLevel::Error) {
            // Format error location for helpful error messages
            let loc = diag
                .span
                .map(|s| format!("{}:{}-{}", file, s.start, s.end))
                .unwrap_or_else(|| file.to_string());
            return Err(Error::new(
                Status::GenericFailure,
                format!("Macro error at {}: {}", loc, diag.message),
            ));
        }
    }
    Ok(())
}

/// Extracts import sources from a parsed program.
///
/// Maps imported identifiers to their module sources, which is used for
/// foreign type import source validation.
///
/// # Arguments
///
/// * `program` - The parsed TypeScript/JavaScript program
///
/// # Returns
///
/// A tuple of three HashMaps:
/// - `sources`: Maps imported identifier names to their module sources
/// - `aliases`: Maps local alias names to their original imported names
/// - `type_only`: Maps identifier names to whether they are type-only imports
///
/// # Example
///
/// For `import { DateTime } from 'effect'`, this returns:
/// - sources: `{"DateTime": "effect"}`
/// - type_only: `{"DateTime": false}`
///
/// For `import type { DateTime } from 'effect'`, this returns:
/// - sources: `{"DateTime": "effect"}`
/// - type_only: `{"DateTime": true}`
///
/// For `import { Option as EffectOption }`, this returns aliases `{"EffectOption": "Option"}`.
fn extract_import_sources(
    program: &Program,
) -> (
    std::collections::HashMap<String, String>,
    std::collections::HashMap<String, String>,
    std::collections::HashMap<String, bool>,
) {
    use swc_core::ecma::ast::{ImportSpecifier, ModuleDecl, ModuleExportName, ModuleItem};

    let mut sources = std::collections::HashMap::new();
    let mut aliases = std::collections::HashMap::new();
    let mut type_only = std::collections::HashMap::new();

    let module = match program {
        Program::Module(m) => m,
        Program::Script(_) => return (sources, aliases, type_only),
    };

    for item in &module.body {
        if let ModuleItem::ModuleDecl(ModuleDecl::Import(import)) = item {
            let source = String::from_utf8_lossy(import.src.value.as_bytes()).to_string();
            // Check if the entire import statement is type-only: `import type { X } from "..."`
            let is_import_type_only = import.type_only;

            for specifier in &import.specifiers {
                match specifier {
                    ImportSpecifier::Named(named) => {
                        let local = named.local.sym.to_string();
                        sources.insert(local.clone(), source.clone());
                        // An import is type-only if either the import statement or the specifier is type-only
                        // e.g., `import type { X }` or `import { type X }`
                        type_only.insert(local.clone(), is_import_type_only || named.is_type_only);

                        // Track aliases: if there's an imported name different from local
                        if let Some(imported) = &named.imported {
                            let original_name = match imported {
                                ModuleExportName::Ident(ident) => ident.sym.to_string(),
                                ModuleExportName::Str(s) => {
                                    String::from_utf8_lossy(s.value.as_bytes()).to_string()
                                }
                            };
                            if original_name != local {
                                aliases.insert(local, original_name);
                            }
                        }
                    }
                    ImportSpecifier::Default(default) => {
                        let local = default.local.sym.to_string();
                        sources.insert(local.clone(), source.clone());
                        type_only.insert(local, is_import_type_only);
                    }
                    ImportSpecifier::Namespace(ns) => {
                        let local = ns.local.sym.to_string();
                        sources.insert(local.clone(), source.clone());
                        type_only.insert(local, is_import_type_only);
                    }
                }
            }
        }
    }

    (sources, aliases, type_only)
}

// ============================================================================
// Manifest / Debug API
// ============================================================================

/// Entry for a registered macro in the manifest.
///
/// Used by [`MacroManifest`] to describe available macros to tooling
/// such as IDE extensions and documentation generators.
#[napi(object)]
pub struct MacroManifestEntry {
    /// The macro name (e.g., "Debug", "Clone", "Serialize").
    pub name: String,
    /// The macro kind: "derive", "attribute", or "function".
    pub kind: String,
    /// Human-readable description of what the macro does.
    pub description: String,
    /// The package that provides this macro (e.g., "macroforge-ts").
    pub package: String,
}

/// Entry for a registered decorator in the manifest.
///
/// Used by [`MacroManifest`] to describe field-level decorators
/// that can be used with macros.
#[napi(object)]
pub struct DecoratorManifestEntry {
    /// The module this decorator belongs to (e.g., "serde").
    pub module: String,
    /// The exported name of the decorator (e.g., "skip", "rename").
    pub export: String,
    /// The decorator kind: "class", "property", "method", "accessor", "parameter".
    pub kind: String,
    /// Documentation string for the decorator.
    pub docs: String,
}

/// Complete manifest of all available macros and decorators.
///
/// This is returned by [`get_macro_manifest`] and is useful for:
/// - IDE autocompletion
/// - Documentation generation
/// - Tooling integration
#[napi(object)]
pub struct MacroManifest {
    /// ABI version for compatibility checking.
    pub version: u32,
    /// All registered macros (derive, attribute, function).
    pub macros: Vec<MacroManifestEntry>,
    /// All registered field/class decorators.
    pub decorators: Vec<DecoratorManifestEntry>,
}

/// Returns the complete manifest of all registered macros and decorators.
///
/// This is a debug/introspection API that allows tooling to discover
/// what macros are available at runtime.
///
/// # Returns
///
/// A [`MacroManifest`] containing all registered macros and decorators.
///
/// # Example (JavaScript)
///
/// ```javascript
/// const manifest = __macroforgeGetManifest();
/// console.log("Available macros:", manifest.macros.map(m => m.name));
/// // ["Debug", "Clone", "PartialEq", "Hash", "Serialize", "Deserialize", ...]
/// ```
#[napi(js_name = "__macroforgeGetManifest")]
pub fn get_macro_manifest() -> MacroManifest {
    let manifest = derived::get_manifest();
    MacroManifest {
        version: manifest.version,
        macros: manifest
            .macros
            .into_iter()
            .map(|m| MacroManifestEntry {
                name: m.name.to_string(),
                kind: format!("{:?}", m.kind).to_lowercase(),
                description: m.description.to_string(),
                package: m.package.to_string(),
            })
            .collect(),
        decorators: manifest
            .decorators
            .into_iter()
            .map(|d| DecoratorManifestEntry {
                module: d.module.to_string(),
                export: d.export.to_string(),
                kind: format!("{:?}", d.kind).to_lowercase(),
                docs: d.docs.to_string(),
            })
            .collect(),
    }
}

/// Checks if any macros are registered in this package.
///
/// Useful for build tools to determine if macro expansion is needed.
///
/// # Returns
///
/// `true` if at least one macro is registered, `false` otherwise.
#[napi(js_name = "__macroforgeIsMacroPackage")]
pub fn is_macro_package() -> bool {
    !derived::macro_names().is_empty()
}

/// Returns the names of all registered macros.
///
/// # Returns
///
/// A vector of macro names (e.g., `["Debug", "Clone", "Serialize"]`).
#[napi(js_name = "__macroforgeGetMacroNames")]
pub fn get_macro_names() -> Vec<String> {
    derived::macro_names()
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}

/// Returns all registered macro module names (debug API).
///
/// Modules group related macros together (e.g., "builtin", "serde").
///
/// # Returns
///
/// A vector of module names.
#[napi(js_name = "__macroforgeDebugGetModules")]
pub fn debug_get_modules() -> Vec<String> {
    crate::host::derived::modules()
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}

/// Looks up a macro by module and name (debug API).
///
/// Useful for testing macro registration and debugging lookup issues.
///
/// # Arguments
///
/// * `module` - The module name (e.g., "builtin")
/// * `name` - The macro name (e.g., "Debug")
///
/// # Returns
///
/// A string describing whether the macro was found or not.
#[napi(js_name = "__macroforgeDebugLookup")]
pub fn debug_lookup(module: String, name: String) -> String {
    match MacroExpander::new() {
        Ok(host) => match host.dispatcher.registry().lookup(&module, &name) {
            Ok(_) => format!("Found: ({}, {})", module, name),
            Err(_) => format!("Not found: ({}, {})", module, name),
        },
        Err(e) => format!("Host init failed: {}", e),
    }
}

/// Returns debug information about all registered macro descriptors (debug API).
///
/// This provides low-level access to the inventory-based macro registration
/// system for debugging purposes.
///
/// # Returns
///
/// A vector of strings describing each registered macro descriptor.
#[napi(js_name = "__macroforgeDebugDescriptors")]
pub fn debug_descriptors() -> Vec<String> {
    inventory::iter::<crate::host::derived::DerivedMacroRegistration>()
        .map(|entry| {
            format!(
                "name={}, module={}, package={}",
                entry.descriptor.name, entry.descriptor.module, entry.descriptor.package
            )
        })
        .collect()
}
