//! A `syn`-like stream for TypeScript macro input and output.
//!
//! This module provides a [`TsStream`] type and [`ParseTs`] trait. This is the
//! low-level API; most macro authors will prefer using
//! [`DeriveInput`](crate::DeriveInput) from the [`derive`](crate::derive) module.
//!
//! ## Overview
//!
//! - [`TsStream`] - Source text plus the macro context, imports and patches
//! - [`ParseTs`] - A trait for types that can be parsed from a [`TsStream`]
//!
//! ## Basic Usage
//!
//! Parse snippets into an OXC arena with the crate-level `parse_*` helpers:
//!
//! ```rust
//! use macroforge_ts_syn::{expr_to_string, parse_expr, parse_program, parse_statement, stmt_to_string};
//!
//! let allocator = macroforge_ts_syn::oxc::allocator::Allocator::default();
//! let expr = parse_expr(&allocator, "x + y")?;
//! let stmt = parse_statement(&allocator, "const x = 5;")?;
//! let module = parse_program(&allocator, "export class Foo {}")?;
//!
//! assert_eq!(expr_to_string(&expr), "x + y");
//! assert_eq!(stmt_to_string(&stmt).trim(), "const x = 5;");
//! assert_eq!(module.body.len(), 1);
//! # Ok::<(), macroforge_ts_syn::TsSynError>(())
//! ```
//!
//! ## Macro Context
//!
//! When used within the macro system, [`TsStream`] carries context information
//! about the macro invocation:
//!
//! ```rust,no_run
//! use macroforge_ts_syn::TsStream;
//!
//! fn example(stream: TsStream) {
//!     // In a macro implementation
//!     let ctx = stream.context().expect("macro context");
//!     let decorator_span = ctx.decorator_span;
//!     let _target = &ctx.target;
//! }
//! ```
//!
//! ## Adding Imports
//!
//! [`TsStream`] provides helpers for adding imports that will be inserted
//! at the top of the file:
//!
//! ```rust,no_run
//! use macroforge_ts_syn::{TsStream, TsSynError};
//!
//! fn main() -> Result<(), TsSynError> {
//!     let source = "class Foo {}";
//!     let mut stream = TsStream::new(source, "input.ts")?;
//!
//!     // Add a runtime import
//!     stream.add_import("deserialize", "./runtime");
//!
//!     // Add a type-only import
//!     stream.add_type_import("Options", "./types");
//!
//!     // Convert to result
//!     let _result = stream.into_result();
//!     Ok(())
//! }
//! ```

use crate::TsSynError;

/// Configuration for a single import to be added to a file.
///
/// Used with [`TsStream::add_imports`] to batch-add multiple imports.
///
/// # Example
///
/// ```rust,ignore
/// use macroforge_ts_syn::ImportConfig;
///
/// const MY_IMPORTS: &[ImportConfig] = &[
///     ImportConfig::value("ok", "__mf_ok", "@macroforge/core/serde"),
///     ImportConfig::type_only("Result", "__mf_Result", "@macroforge/core/serde"),
/// ];
///
/// stream.add_imports(MY_IMPORTS);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ImportConfig {
    /// The original export name from the source module
    pub name: &'static str,
    /// The alias to use in the generated import (typically `__mf_` prefixed)
    pub alias: &'static str,
    /// The module path to import from
    pub module: &'static str,
    /// Whether this is a type-only import (`import type { ... }`)
    pub is_type: bool,
}

impl ImportConfig {
    /// Create a value import config (runtime import).
    pub const fn value(name: &'static str, alias: &'static str, module: &'static str) -> Self {
        Self {
            name,
            alias,
            module,
            is_type: false,
        }
    }

    /// Create a type-only import config.
    pub const fn type_only(name: &'static str, alias: &'static str, module: &'static str) -> Self {
        Self {
            name,
            alias,
            module,
            is_type: true,
        }
    }
}

/// TypeScript source carrying its macro context, analogous to `syn::parse::ParseBuffer`.
///
/// `TsStream` manages the parsing context for TypeScript source code. It holds:
/// - The source code being parsed
/// - An optional macro context with span information
/// - Accumulated runtime patches (imports, etc.)
///
/// # Creating a TsStream
///
/// ```rust
/// use macroforge_ts_syn::{TsStream, TsSynError};
///
/// fn main() -> Result<(), TsSynError> {
///     // From source code
///     let _stream = TsStream::new("const x = 5;", "input.ts")?;
///
///     // From an owned string
///     let generated_code = "const y = 10;".to_string();
///     let _stream = TsStream::from_string(generated_code);
///     Ok(())
/// }
/// ```
///
/// With macro context (typically used by the host system):
///
/// ```rust,ignore
/// use macroforge_ts_syn::{TsStream, TsSynError, MacroContextIR, SpanIR, ClassIR};
///
/// // MacroContextIR requires a target type - this is typically provided by the host
/// let source = "class Foo {}";
/// let ctx = MacroContextIR::new_derive_class(/* ... */);
/// let stream = TsStream::with_context(source, "input.ts", ctx)?;
/// ```
///
/// # Parsing
///
/// ```rust
/// use macroforge_ts_syn::{TsStream, TsSynError};
///
/// fn main() -> Result<(), TsSynError> {
///     let stream = TsStream::new("const x = 5;", "input.ts")?;
///     let allocator = macroforge_ts_syn::oxc::allocator::Allocator::default();
///     let _stmt = stream.parse_stmt(&allocator)?;
///     Ok(())
/// }
/// ```
///
/// # Working with Imports
///
/// ```rust,ignore
/// use macroforge_ts_syn::{TsStream, TsSynError};
///
/// fn main() -> Result<(), TsSynError> {
///     let source = "class Foo {}";
///     let mut stream = TsStream::new(source, "file.ts")?;
///
///     // These imports will be added to the file
///     stream.add_import("deserialize", "./runtime");
///     stream.add_type_import("Options", "./types");
///
///     // Get the result with accumulated patches
///     let _result = stream.into_result();
///     Ok(())
/// }
/// ```
pub struct TsStream {
    source: String,
    file_name: String,
    /// Macro context data (decorator span, target span, etc.)
    /// This is populated when TsStream is created by the macro host
    pub ctx: Option<crate::abi::MacroContextIR>,
    /// Runtime patches to apply (e.g., imports at file level)
    pub runtime_patches: Vec<crate::abi::Patch>,
    /// Where this stream's code should be inserted relative to the target.
    /// Defaults to `Below` (after the target declaration).
    pub insert_pos: crate::abi::InsertPos,
    /// Cross-module function suffixes for auto-import resolution.
    /// External macros register suffixes here so the framework can resolve
    /// cross-module references following the `{camelCaseTypeName}{Suffix}` pattern.
    pub cross_module_suffixes: Vec<String>,
    /// Cross-module type suffixes for auto-import resolution.
    /// These resolve `{PascalCaseTypeName}{Suffix}` type references and generate
    /// `import type` statements. Used for types like `ColorsErrors`, `ColorsTainted`.
    pub cross_module_type_suffixes: Vec<String>,
}

impl TsStream {
    /// Parse the first statement of the stream into `allocator`.
    pub fn parse_stmt<'a>(
        &'a self,
        allocator: &'a oxc::allocator::Allocator,
    ) -> Result<oxc::ast::ast::Statement<'a>, TsSynError> {
        let source_type = oxc::span::SourceType::ts()
            .with_typescript(true)
            .with_jsx(self.file_name.ends_with(".tsx"));
        let ret = oxc::parser::Parser::new(allocator, &self.source, source_type).parse();
        if !ret.diagnostics.is_empty() {
            return Err(TsSynError::Parse(format!(
                "parse errors: {:?}",
                ret.diagnostics
            )));
        }

        ret.program
            .body
            .into_iter()
            .next()
            .ok_or_else(|| TsSynError::Parse("No statement found".to_string()))
    }
}

impl TsStream {
    /// Create a new parsing stream from source code.
    pub fn new(source: &str, file_name: &str) -> Result<Self, TsSynError> {
        Ok(TsStream {
            source: source.to_string(),
            file_name: file_name.to_string(),
            ctx: None,
            runtime_patches: vec![],
            insert_pos: crate::abi::InsertPos::default(),
            cross_module_suffixes: vec![],
            cross_module_type_suffixes: vec![],
        })
    }

    /// Create a new parsing stream from an owned string.
    pub fn from_string(source: String) -> Self {
        TsStream {
            source,
            file_name: "macro_output.ts".to_string(),
            ctx: None,
            runtime_patches: vec![],
            insert_pos: crate::abi::InsertPos::default(),
            cross_module_suffixes: vec![],
            cross_module_type_suffixes: vec![],
        }
    }

    /// Create a new parsing stream with a specific insert position.
    pub fn with_insert_pos(source: String, insert_pos: crate::abi::InsertPos) -> Self {
        TsStream {
            source,
            file_name: "macro_output.ts".to_string(),
            ctx: None,
            runtime_patches: vec![],
            insert_pos,
            cross_module_suffixes: vec![],
            cross_module_type_suffixes: vec![],
        }
    }

    /// Create a new parsing stream with a specific insert position and runtime patches.
    /// Used by `ts_template!` to collect patches from embedded TsStreams.
    pub fn with_insert_pos_and_patches(
        source: String,
        insert_pos: crate::abi::InsertPos,
        runtime_patches: Vec<crate::abi::Patch>,
    ) -> Self {
        TsStream {
            source,
            file_name: "macro_output.ts".to_string(),
            ctx: None,
            runtime_patches,
            insert_pos,
            cross_module_suffixes: vec![],
            cross_module_type_suffixes: vec![],
        }
    }

    /// Get the source code of the stream.
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Create a new parsing stream with macro context attached.
    /// This is used by the macro host to provide context to macros.
    pub fn with_context(
        source: &str,
        file_name: &str,
        ctx: crate::abi::MacroContextIR,
    ) -> Result<Self, TsSynError> {
        Ok(TsStream {
            source: source.to_string(),
            file_name: file_name.to_string(),
            ctx: Some(ctx),
            runtime_patches: vec![],
            insert_pos: crate::abi::InsertPos::default(),
            cross_module_suffixes: vec![],
            cross_module_type_suffixes: vec![],
        })
    }

    /// Get the macro context if available
    pub fn context(&self) -> Option<&crate::abi::MacroContextIR> {
        self.ctx.as_ref()
    }

    /// Convert the stream into a MacroResult.
    /// Captures any imports registered via `add_import()` etc. from the thread-local
    /// registry so they survive serialization across process boundaries.
    pub fn into_result(self) -> crate::abi::MacroResult {
        let imports = crate::import_registry::with_registry_mut(|r| r.take_generated_imports());
        crate::abi::MacroResult {
            runtime_patches: self.runtime_patches,
            type_patches: vec![],
            diagnostics: vec![],
            tokens: Some(self.source),
            insert_pos: self.insert_pos,
            debug: None,
            cross_module_suffixes: self.cross_module_suffixes,
            cross_module_type_suffixes: self.cross_module_type_suffixes,
            imports,
            // The entry point a macro is loaded through fills this in.
            registry_reads: None,
        }
    }

    /// Add an import statement. Registers in the [`ImportRegistry`](crate::ImportRegistry)
    /// for idempotent deduplication — no patches are emitted; the registry emits all
    /// generated imports at the end of expansion.
    pub fn add_import(&mut self, specifier: &str, module: &str) {
        let (original, local) = parse_import_specifier(specifier);
        crate::import_registry::with_registry_mut(|r| {
            r.request_import(&local, original.as_deref(), module, false);
        });
    }

    /// Add a type-only import statement. Registers in the [`ImportRegistry`](crate::ImportRegistry).
    pub fn add_type_import(&mut self, specifier: &str, module: &str) {
        let (original, local) = parse_import_specifier(specifier);
        crate::import_registry::with_registry_mut(|r| {
            r.request_import(&local, original.as_deref(), module, true);
        });
    }

    /// Returns the module specifier this stream would use to import `type_name`,
    /// without emitting an import. Resolution comes from the attached or
    /// thread-local [`MacroContextIR`](crate::abi::ir::context::MacroContextIR).
    /// Returns `None` if the type is co-located, unknown, or no context is available.
    pub fn module_specifier_for(&self, type_name: &str) -> Option<String> {
        if let Some(ctx) = self.ctx.as_ref() {
            return ctx.import_specifier_for(type_name);
        }
        crate::context_registry::with_context(|ctx| {
            ctx.and_then(|c| c.import_specifier_for(type_name))
        })
    }

    /// Resolves the module for `type_name` and emits a value import for
    /// `local_name` from that module. Returns `true` when an import was queued.
    /// No-op when the type can't be resolved (e.g. co-located or unknown).
    pub fn add_import_for(&mut self, local_name: &str, type_name: &str) -> bool {
        let Some(module) = self.module_specifier_for(type_name) else {
            return false;
        };
        self.add_import(local_name, &module);
        true
    }

    /// Type-only variant of [`Self::add_import_for`].
    pub fn add_type_import_for(&mut self, local_name: &str, type_name: &str) -> bool {
        let Some(module) = self.module_specifier_for(type_name) else {
            return false;
        };
        self.add_type_import(local_name, &module);
        true
    }

    /// Batch helper: emits N imports for the same `type_name`, resolving the
    /// module exactly once. Each entry is `(local_name, is_type_only)`.
    /// Returns `true` when imports were queued.
    pub fn add_helpers_for(&mut self, type_name: &str, helpers: &[(&str, bool)]) -> bool {
        let Some(module) = self.module_specifier_for(type_name) else {
            return false;
        };
        for (name, is_type) in helpers {
            if *is_type {
                self.add_type_import(name, &module);
            } else {
                self.add_import(name, &module);
            }
        }
        true
    }

    /// Add an import with automatic `__mf_` alias to avoid collisions with user imports.
    ///
    /// # Example
    /// ```ignore
    /// stream.add_aliased_import("DeserializeContext", "@macroforge/core/serde");
    /// // Generates: import { DeserializeContext as __mf_DeserializeContext } from "@macroforge/core/serde";
    /// ```
    pub fn add_aliased_import(&mut self, name: &str, module: &str) {
        let alias = format!("__mf_{name}");
        crate::import_registry::with_registry_mut(|r| {
            r.request_import(&alias, Some(name), module, false);
        });
    }

    /// Add a type-only import with automatic `__mf_` alias.
    ///
    /// # Example
    /// ```ignore
    /// stream.add_aliased_type_import("DeserializeOptions", "@macroforge/core/serde");
    /// // Generates: import type { DeserializeOptions as __mf_DeserializeOptions } from "@macroforge/core/serde";
    /// ```
    pub fn add_aliased_type_import(&mut self, name: &str, module: &str) {
        let alias = format!("__mf_{name}");
        crate::import_registry::with_registry_mut(|r| {
            r.request_import(&alias, Some(name), module, true);
        });
    }

    /// Add an import with a custom alias.
    ///
    /// # Example
    /// ```ignore
    /// stream.add_import_as("resultOk", "__mf_resultOk", "@macroforge/core/serde");
    /// // Generates: import { resultOk as __mf_resultOk } from "@macroforge/core/serde";
    /// ```
    pub fn add_import_as(&mut self, name: &str, alias: &str, module: &str) {
        crate::import_registry::with_registry_mut(|r| {
            r.request_import(alias, Some(name), module, false);
        });
    }

    /// Add a type-only import with a custom alias.
    ///
    /// # Example
    /// ```ignore
    /// stream.add_type_import_as("Result", "__mf_Result", "@macroforge/core/serde");
    /// // Generates: import type { Result as __mf_Result } from "@macroforge/core/serde";
    /// ```
    pub fn add_type_import_as(&mut self, name: &str, alias: &str, module: &str) {
        crate::import_registry::with_registry_mut(|r| {
            r.request_import(alias, Some(name), module, true);
        });
    }

    /// Add multiple imports from a slice of [`ImportConfig`].
    ///
    /// This is the preferred way to add macro-related imports, as it handles
    /// both value and type imports with proper aliasing.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use macroforge_ts_syn::ImportConfig;
    ///
    /// const SERDE_IMPORTS: &[ImportConfig] = &[
    ///     ImportConfig::value("DeserializeContext", "__mf_DeserializeContext", "@macroforge/core/serde"),
    ///     ImportConfig::type_only("DeserializeOptions", "__mf_DeserializeOptions", "@macroforge/core/serde"),
    /// ];
    ///
    /// stream.add_imports(SERDE_IMPORTS);
    /// ```
    pub fn add_imports(&mut self, imports: &[ImportConfig]) {
        crate::import_registry::with_registry_mut(|r| {
            for import in imports {
                r.request_import(
                    import.alias,
                    Some(import.name),
                    import.module,
                    import.is_type,
                );
            }
        });
    }

    /// Register a cross-module function suffix for auto-import resolution.
    ///
    /// When the generated code references a function like `companyNameGetFields()`,
    /// the framework needs to know that `GetFields` is a valid suffix to resolve
    /// against imported types. Built-in macros (Default, Serialize, etc.) have
    /// their suffixes hardcoded; external macros use this method to register theirs.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // In your macro's generate function:
    /// output.add_cross_module_suffix("GetFields");
    /// // Now if the file imports `CompanyName` from `./account.svelte`,
    /// // and the generated code references `companyNameGetFields()`,
    /// // the framework will auto-add: import { companyNameGetFields } from "./account.svelte";
    /// ```
    pub fn add_cross_module_suffix(&mut self, suffix: &str) {
        self.cross_module_suffixes.push(suffix.to_string());
    }

    /// Register a cross-module type suffix for PascalCase type reference auto-import.
    ///
    /// Unlike `add_cross_module_suffix` (which resolves `{camelCase}{Suffix}` function calls),
    /// this resolves `{PascalCase}{Suffix}` type references and generates `import type` statements.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // In your macro's generate function:
    /// output.add_cross_module_type_suffix("Errors");
    /// // Now if the file imports `Colors` from `./shared.svelte`,
    /// // and the generated code references `ColorsErrors` (type position),
    /// // the framework will auto-add: import type { ColorsErrors } from "./shared.svelte";
    /// ```
    pub fn add_cross_module_type_suffix(&mut self, suffix: &str) {
        self.cross_module_type_suffixes.push(suffix.to_string());
    }

    /// Merge another TsStream into this one.
    ///
    /// Combines the source code and runtime patches from both streams.
    /// The insert position of `self` is preserved.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let standalone = ts_template! { export function foo() {} };
    /// let class_body = ts_template!(Within { static bar() {} });
    ///
    /// // Instead of: ts_template! { {$typescript standalone} {$typescript class_body} }
    /// let combined = standalone.merge(class_body);
    /// ```
    pub fn merge(mut self, other: Self) -> Self {
        // Combine source code with a separator only when needed.
        if !self.source.is_empty() && !other.source.is_empty() {
            let left_ends_ws = self
                .source
                .chars()
                .last()
                .map(|c| c.is_whitespace())
                .unwrap_or(false);
            let right_starts_ws = other
                .source
                .chars()
                .next()
                .map(|c| c.is_whitespace())
                .unwrap_or(false);
            if !(left_ends_ws || right_starts_ws) {
                self.source.push('\n');
            }
        }
        self.source.push_str(&other.source);

        // Merge runtime patches
        self.runtime_patches.extend(other.runtime_patches);

        // Merge cross-module suffixes
        self.cross_module_suffixes
            .extend(other.cross_module_suffixes);
        self.cross_module_type_suffixes
            .extend(other.cross_module_type_suffixes);

        self
    }

    /// Merge multiple TsStreams into one.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let combined = TsStream::merge_all([stream1, stream2, stream3]);
    /// ```
    pub fn merge_all(streams: impl IntoIterator<Item = Self>) -> Self {
        let mut iter = streams.into_iter();
        match iter.next() {
            Some(first) => iter.fold(first, |acc, stream| acc.merge(stream)),
            None => Self::from_string(String::new()),
        }
    }
}

/// A trait for types that can be parsed from a [`TsStream`], analogous to `syn::parse::Parse`.
///
/// [`DeriveInput`](crate::DeriveInput) implements `ParseTs`, which is what the
/// `parse_ts_macro_input!` macro relies on. Implement it for a custom input type
/// to use that type with the macro.
pub trait ParseTs: Sized {
    /// Parse a value of this type from a parsing stream.
    ///
    /// # Errors
    ///
    /// Returns [`TsSynError::Parse`] if the source code doesn't match the
    /// expected syntax for this type.
    fn parse(input: &mut TsStream) -> Result<Self, TsSynError>;
}

/// Parse an import specifier like `"Foo as Bar"` into `(Some("Foo"), "Bar")`,
/// or `"Foo"` into `(None, "Foo")`.
fn parse_import_specifier(specifier: &str) -> (Option<String>, String) {
    if let Some(idx) = specifier.find(" as ") {
        let original = specifier[..idx].trim().to_string();
        let local = specifier[idx + 4..].trim().to_string();
        (Some(original), local)
    } else {
        (None, specifier.trim().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_cross_module_suffix() {
        let mut stream = TsStream::from_string("export function foo() {}".to_string());
        assert!(stream.cross_module_suffixes.is_empty());

        stream.add_cross_module_suffix("GetFields");
        assert_eq!(stream.cross_module_suffixes, vec!["GetFields"]);

        stream.add_cross_module_suffix("CustomSuffix");
        assert_eq!(
            stream.cross_module_suffixes,
            vec!["GetFields", "CustomSuffix"]
        );
    }

    #[test]
    fn test_cross_module_suffixes_propagate_to_result() {
        let mut stream = TsStream::from_string("export function foo() {}".to_string());
        stream.add_cross_module_suffix("GetFields");
        stream.add_cross_module_suffix("OtherSuffix");

        let result = stream.into_result();
        assert_eq!(
            result.cross_module_suffixes,
            vec!["GetFields", "OtherSuffix"]
        );
    }

    #[test]
    fn test_merge_combines_cross_module_suffixes() {
        let mut a = TsStream::from_string("export function a() {}".to_string());
        a.add_cross_module_suffix("GetFields");

        let mut b = TsStream::from_string("export function b() {}".to_string());
        b.add_cross_module_suffix("CustomSuffix");

        let merged = a.merge(b);
        assert_eq!(
            merged.cross_module_suffixes,
            vec!["GetFields", "CustomSuffix"]
        );
    }

    #[test]
    fn test_empty_cross_module_suffixes_by_default() {
        let stream = TsStream::from_string("const x = 1;".to_string());
        assert!(stream.cross_module_suffixes.is_empty());

        let result = stream.into_result();
        assert!(result.cross_module_suffixes.is_empty());
    }
}

#[cfg(test)]
mod import_for_tests {
    use super::*;
    use crate::abi::ir::context::{MacroContextIR, MacroKind, TargetIR};
    use crate::abi::ir::interface::InterfaceIR;
    use crate::abi::ir::type_registry::{TypeDefinitionIR, TypeRegistry, TypeRegistryEntry};
    use crate::import_registry::{ImportRegistry, take_registry};
    use crate::{SpanIR, context_registry};

    fn empty_interface(name: &str) -> InterfaceIR {
        InterfaceIR {
            name: name.to_string(),
            span: SpanIR::new(0, 0),
            body_span: SpanIR::new(0, 0),
            type_params: vec![],
            heritage: vec![],
            fields: vec![],
            methods: vec![],
            decorators: vec![],
        }
    }

    fn make_ctx_with_registry(file_name: &str, type_name: &str, type_path: &str) -> MacroContextIR {
        let mut registry = TypeRegistry::new();
        registry.types.insert(
            type_name.to_string(),
            TypeRegistryEntry {
                name: type_name.to_string(),
                file_path: type_path.to_string(),
                is_exported: true,
                definition: TypeDefinitionIR::Interface(empty_interface(type_name)),
                file_imports: vec![],
            },
        );
        MacroContextIR {
            abi_version: 1,
            macro_kind: MacroKind::Derive,
            macro_name: "Test".to_string(),
            module_path: "@test".to_string(),
            decorator_span: SpanIR::new(0, 0),
            macro_name_span: None,
            target_span: SpanIR::new(0, 0),
            file_name: file_name.to_string(),
            target: TargetIR::Interface(empty_interface("Probe")),
            target_source: String::new(),
            import_registry: ImportRegistry::new(),
            config: None,
            type_registry: registry,
            resolved_fields: None,
        }
    }

    /// Helper that resets the thread-local registries before each test runs
    /// (the tests in this module share thread-local state).
    fn reset_thread_locals() {
        context_registry::clear_context();
        let _ = take_registry();
        crate::import_registry::install_registry(ImportRegistry::new());
    }

    #[test]
    fn add_import_for_resolves_via_thread_local_context() {
        reset_thread_locals();
        let ctx = make_ctx_with_registry(
            "/proj/src/order.svelte.ts",
            "Customer",
            "/proj/src/customer.svelte.ts",
        );
        context_registry::install_context(ctx);

        let mut stream = TsStream::from_string(String::new());
        let added = stream.add_import_for("customerDefaultValue", "Customer");
        assert!(added);

        let result = stream.into_result();
        let customer = result
            .imports
            .iter()
            .find(|i| i.local_name == "customerDefaultValue")
            .expect("expected customerDefaultValue import to be queued");
        assert_eq!(customer.source_module, "./customer.svelte");
        assert!(!customer.is_type_only);

        context_registry::clear_context();
    }

    #[test]
    fn add_helpers_for_batches_correctly() {
        reset_thread_locals();
        let ctx = make_ctx_with_registry(
            "/proj/src/order.svelte.ts",
            "Customer",
            "/proj/src/customer.svelte.ts",
        );
        context_registry::install_context(ctx);

        let mut stream = TsStream::from_string(String::new());
        let added = stream.add_helpers_for(
            "Customer",
            &[
                ("customerDefaultValue", false),
                ("CustomerErrors", true),
                ("customerHasShape", false),
            ],
        );
        assert!(added);

        let result = stream.into_result();

        let value = result
            .imports
            .iter()
            .find(|i| i.local_name == "customerDefaultValue")
            .expect("value import missing");
        assert_eq!(value.source_module, "./customer.svelte");
        assert!(!value.is_type_only);

        let type_import = result
            .imports
            .iter()
            .find(|i| i.local_name == "CustomerErrors")
            .expect("type import missing");
        assert_eq!(type_import.source_module, "./customer.svelte");
        assert!(type_import.is_type_only);

        let has_shape = result
            .imports
            .iter()
            .find(|i| i.local_name == "customerHasShape")
            .expect("has_shape import missing");
        assert_eq!(has_shape.source_module, "./customer.svelte");
        assert!(!has_shape.is_type_only);

        context_registry::clear_context();
    }

    #[test]
    fn add_import_for_returns_false_without_context() {
        reset_thread_locals();
        let mut stream = TsStream::from_string(String::new());
        let added = stream.add_import_for("customerDefaultValue", "Customer");
        assert!(!added);
        let result = stream.into_result();
        assert!(result.imports.is_empty());
    }

    #[test]
    fn add_import_for_returns_false_for_co_located_type() {
        reset_thread_locals();
        let ctx = make_ctx_with_registry(
            "/proj/src/customer.svelte.ts",
            "Customer",
            "/proj/src/customer.svelte.ts",
        );
        context_registry::install_context(ctx);

        let mut stream = TsStream::from_string(String::new());
        let added = stream.add_import_for("customerDefaultValue", "Customer");
        assert!(!added);
        let result = stream.into_result();
        assert!(result.imports.is_empty());

        context_registry::clear_context();
    }

    /// Pre-registering a variant's source module through
    /// `install_source_imports` shadows any later `add_type_import` for the
    /// same name: `request_import` skips it because `source_imports` already
    /// holds the name, so the bare variant type is referenced in generated code
    /// and never imported. This pins that interaction, so a macro that
    /// pre-registers a variant is known to lose the import.
    #[test]
    fn add_helpers_for_emits_bare_variant_type_import() {
        reset_thread_locals();
        let ctx = make_ctx_with_registry(
            "/proj/src/employee.svelte.ts",
            "LinkedUser",
            "/proj/src/linked-user.svelte.ts",
        );
        context_registry::install_context(ctx);

        // Simulate the buggy pre-registration: pretend something put
        // `LinkedUser` into `source_imports` before the helper batch ran.
        crate::import_registry::with_registry_mut(|r| {
            r.install_source_imports(vec![crate::import_registry::SourceImportEntry {
                local_name: "LinkedUser".to_string(),
                source_module: "./linked-user.svelte".to_string(),
                original_name: None,
                is_type_only: true,
            }]);
        });

        let mut stream = TsStream::from_string(String::new());
        let added = stream.add_helpers_for(
            "LinkedUser",
            &[
                ("LinkedUserFieldControllers", true),
                ("LinkedUser", true),
                ("linkedUserGetControllers", false),
                ("linkedUserDefaultErrors", false),
                ("linkedUserDefaultTainted", false),
                ("linkedUserDefaultValue", false),
                ("linkedUserIs", false),
            ],
        );
        assert!(added, "add_helpers_for should have resolved LinkedUser");

        let result = stream.into_result();

        // The derived `FieldControllers` helper should be present.
        let field_controllers = result
            .imports
            .iter()
            .find(|i| i.local_name == "LinkedUserFieldControllers")
            .expect("LinkedUserFieldControllers type import missing");
        assert_eq!(field_controllers.source_module, "./linked-user.svelte");
        assert!(field_controllers.is_type_only);

        // The dedup correctly skips re-emitting `LinkedUser` because it's
        // already in `source_imports`. This is the *intended* behavior for
        // request_import — the bug was using `install_source_imports` to
        // shortcut module resolution for variants. Document the dedup
        // behavior here so a future regression in either direction is caught.
        let bare = result.imports.iter().find(|i| i.local_name == "LinkedUser");
        assert!(
            bare.is_none(),
            "bare LinkedUser should be dedup'd against source_imports — \
             callers must NOT pre-register variant types in source_imports \
             if they also want a generated `import type {{ LinkedUser }}` line"
        );

        // All five value helpers should still be present (they have distinct
        // local names that aren't dedup'd against the source-imports entry).
        for name in [
            "linkedUserGetControllers",
            "linkedUserDefaultErrors",
            "linkedUserDefaultTainted",
            "linkedUserDefaultValue",
            "linkedUserIs",
        ] {
            let entry = result
                .imports
                .iter()
                .find(|i| i.local_name == name)
                .unwrap_or_else(|| panic!("{name} value import missing"));
            assert_eq!(entry.source_module, "./linked-user.svelte");
            assert!(!entry.is_type_only);
        }

        context_registry::clear_context();
    }

    /// Companion to `add_helpers_for_emits_bare_variant_type_import`: without
    /// the pre-registration, the bare type import is emitted normally.
    #[test]
    fn add_helpers_for_emits_bare_type_when_source_imports_clean() {
        reset_thread_locals();
        let ctx = make_ctx_with_registry(
            "/proj/src/entry.svelte.ts",
            "CustomerReferral",
            "/proj/src/customer-referral.svelte.ts",
        );
        context_registry::install_context(ctx);

        let mut stream = TsStream::from_string(String::new());
        let added = stream.add_helpers_for(
            "CustomerReferral",
            &[
                ("CustomerReferralFieldControllers", true),
                ("CustomerReferral", true),
                ("customerReferralIs", false),
            ],
        );
        assert!(added);

        let result = stream.into_result();
        let bare = result
            .imports
            .iter()
            .find(|i| i.local_name == "CustomerReferral")
            .expect("bare CustomerReferral type import missing");
        assert_eq!(bare.source_module, "./customer-referral.svelte");
        assert!(bare.is_type_only);

        context_registry::clear_context();
    }
}
