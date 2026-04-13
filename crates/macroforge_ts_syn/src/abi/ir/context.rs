//! Macro execution context types.
//!
//! This module provides the context that is passed to macro functions during
//! execution. The [`MacroContextIR`] contains all the information a macro needs
//! to process its input and generate output.
//!
//! ## Context Flow
//!
//! ```text
//! TypeScript Source
//!        │
//!        ▼
//! ┌─────────────────┐
//! │  Parse & Lower  │  (SWC parser → IR types)
//! └────────┬────────┘
//!          │
//!          ▼
//! ┌─────────────────┐
//! │ MacroContextIR  │  (Serialized to macro)
//! └────────┬────────┘
//!          │
//!          ▼
//! ┌─────────────────┐
//! │  Macro Function │  (Your code!)
//! └────────┬────────┘
//!          │
//!          ▼
//! ┌─────────────────┐
//! │   MacroResult   │  (Patches to apply)
//! └─────────────────┘
//! ```
//!
//! ## Example Usage
//!
//! ```rust
//! use macroforge_ts_syn::{MacroContextIR, MacroResult};
//!
//! // This shows the typical signature of a derive macro
//! pub fn my_derive_macro(ctx: MacroContextIR) -> MacroResult {
//!     // Access macro metadata
//!     println!("Macro: {} from {}", ctx.macro_name, ctx.module_path);
//!     println!("File: {}", ctx.file_name);
//!
//!     // Work with the target
//!     if let Some(class) = ctx.as_class() {
//!         println!("Processing class: {}", class.name);
//!     }
//!
//!     MacroResult::default()
//! }
//! ```

use std::collections::HashMap;

use crate::abi::type_registry::{ResolvedTypeRef, TypeRegistry};
use crate::abi::{ClassIR, EnumIR, FunctionIR, InterfaceIR, SpanIR, TypeAliasIR};
use crate::import_registry::ImportRegistry;
use serde::{Deserialize, Serialize};

/// The kind of macro being executed.
///
/// Different macro kinds have different invocation syntax and capabilities:
///
/// - **Derive**: Attached to types, generates additional code alongside the type
/// - **Attribute**: Attached to declarations, can transform the declaration
/// - **Call**: Invoked inline, generates code at the call site
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MacroKind {
    /// A derive macro attached via JSDoc.
    ///
    /// Example: `/** @derive(Debug, Clone) */`
    ///
    /// Derive macros receive the type definition and generate additional
    /// code (methods, traits, etc.) without modifying the original type.
    Derive,

    /// An attribute macro attached to a declaration.
    ///
    /// Example: `@log`, `@sqlTable("users")`
    ///
    /// Attribute macros can transform or augment the target declaration.
    Attribute,

    /// A call macro invoked inline.
    ///
    /// Example: `sql!("SELECT * FROM users")`, `html!(<div>...</div>)`
    ///
    /// Call macros generate code at the invocation site.
    Call,
}

/// The target declaration of a macro application.
///
/// Wraps the IR type of the declaration that the macro is applied to,
/// allowing macros to handle different target types uniformly.
///
/// # Example
///
/// ```rust,no_run
/// use macroforge_ts_syn::TargetIR;
///
/// fn get_target_name(target: &TargetIR) -> &str {
///     match target {
///         TargetIR::Class(c) => &c.name,
///         TargetIR::Interface(i) => &i.name,
///         TargetIR::Enum(e) => &e.name,
///         TargetIR::TypeAlias(t) => &t.name,
///         TargetIR::Function(f) => &f.name,
///         TargetIR::Other => "<unknown>",
///     }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetIR {
    /// Macro applied to a class declaration.
    Class(ClassIR),

    /// Macro applied to an enum declaration.
    Enum(EnumIR),

    /// Macro applied to an interface declaration.
    Interface(InterfaceIR),

    /// Macro applied to a type alias declaration.
    TypeAlias(TypeAliasIR),

    /// Macro applied to a function declaration.
    Function(FunctionIR),

    /// Macro applied to an unsupported construct.
    Other,
}

/// Context provided to macros during execution.
///
/// This is the primary input to all macro functions. It contains:
/// - Metadata about the macro invocation (name, kind, source location)
/// - The target declaration ([`TargetIR`]) the macro is applied to
/// - Source spans for accurate error reporting
/// - The raw source code of the target for custom parsing
///
/// # ABI Stability
///
/// The `abi_version` field allows for backwards compatibility checking.
/// Macros can verify they're compatible with the runtime version.
///
/// # Error Reporting
///
/// Use [`error_span()`](Self::error_span) to get the best span for error
/// messages. It prefers `macro_name_span` (pointing to the specific macro)
/// over `decorator_span` (pointing to the entire decorator).
///
/// # Example
///
/// ```rust
/// use macroforge_ts_syn::{MacroContextIR, MacroResult, insert_into_class};
///
/// pub fn debug_derive(ctx: MacroContextIR) -> MacroResult {
///     let class = ctx.as_class().expect("Debug requires a class");
///
///     // Generate a debug method
///     let method_code = format!(
///         r#"debug(): string {{
///             return "{}({{}})";
///         }}"#,
///         class.name
///     );
///
///     // Insert at end of class body
///     MacroResult {
///         runtime_patches: vec![insert_into_class(class.body_span, method_code)],
///         ..Default::default()
///     }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroContextIR {
    /// ABI version for compatibility checking
    pub abi_version: u32,

    /// The kind of macro being executed
    pub macro_kind: MacroKind,

    /// The name of the macro (e.g., "Debug")
    pub macro_name: String,

    /// The module path the macro comes from (e.g., "@macro/derive")
    pub module_path: String,

    /// Span of the decorator/macro invocation (entire @derive(...))
    pub decorator_span: SpanIR,

    /// Span of just the macro name within the decorator (e.g., "Debug" in @derive(Debug))
    /// Used for error reporting to point to the specific macro that caused the error
    #[serde(default)]
    pub macro_name_span: Option<SpanIR>,

    /// Span of the target (class, enum, etc.)
    pub target_span: SpanIR,

    /// The file being processed
    pub file_name: String,

    /// The target of the macro application
    pub target: TargetIR,

    /// The source code of the target (class, enum, etc.)
    /// This enables macros to parse the source themselves using TsStream
    pub target_source: String,

    /// The full import registry from the file being processed.
    /// Populated by the host before serializing the context for external macros.
    /// Includes source imports, config imports, and previously generated imports,
    /// giving external macros full parity with builtins.
    #[serde(default)]
    pub import_registry: ImportRegistry,

    /// Macroforge configuration from macroforge.config.ts.
    /// Populated by the host before serializing the context for external macros.
    /// Gives external macros access to foreign type configs, etc.
    #[serde(default)]
    pub config: Option<crate::config::MacroforgeConfig>,

    /// Project-wide type registry, populated during pre-expansion scan.
    /// Gives macros access to all types defined in the project for
    /// Zig-style compile-time type awareness.
    ///
    /// This is `None` when no pre-scan was performed (backward compatible).
    #[serde(default)]
    pub type_registry: Option<TypeRegistry>,

    /// Resolved type references for the target's fields.
    /// Maps field name -> ResolvedTypeRef for each field in the target.
    /// Only populated when type_registry is available.
    #[serde(default)]
    pub resolved_fields: Option<HashMap<String, ResolvedTypeRef>>,
}

impl MacroContextIR {
    /// Create a new macro context for a derive macro on a class
    pub fn new_derive_class(
        macro_name: String,
        module_path: String,
        decorator_span: SpanIR,
        target_span: SpanIR,
        file_name: String,
        class: ClassIR,
        target_source: String,
    ) -> Self {
        Self {
            abi_version: 1,
            macro_kind: MacroKind::Derive,
            macro_name,
            module_path,
            decorator_span,
            macro_name_span: None,
            target_span,
            file_name,
            target: TargetIR::Class(class),
            target_source,
            import_registry: ImportRegistry::new(),
            config: None,
            type_registry: None,
            resolved_fields: None,
        }
    }

    /// Set the macro name span (builder pattern)
    pub fn with_macro_name_span(mut self, span: SpanIR) -> Self {
        self.macro_name_span = Some(span);
        self
    }

    /// Get the best span for error reporting - prefers macro_name_span if available
    pub fn error_span(&self) -> SpanIR {
        self.macro_name_span.unwrap_or(self.decorator_span)
    }

    /// Get the class IR if the target is a class
    pub fn as_class(&self) -> Option<&ClassIR> {
        match &self.target {
            TargetIR::Class(class) => Some(class),
            _ => None,
        }
    }

    /// Get the enum IR if the target is an enum
    pub fn as_enum(&self) -> Option<&EnumIR> {
        match &self.target {
            TargetIR::Enum(enum_ir) => Some(enum_ir),
            _ => None,
        }
    }

    /// Get the interface IR if the target is an interface
    pub fn as_interface(&self) -> Option<&InterfaceIR> {
        match &self.target {
            TargetIR::Interface(interface_ir) => Some(interface_ir),
            _ => None,
        }
    }

    /// Get the type alias IR if the target is a type alias
    pub fn as_type_alias(&self) -> Option<&TypeAliasIR> {
        match &self.target {
            TargetIR::TypeAlias(type_alias_ir) => Some(type_alias_ir),
            _ => None,
        }
    }

    /// Get the function IR if the target is a function
    pub fn as_function(&self) -> Option<&FunctionIR> {
        match &self.target {
            TargetIR::Function(function_ir) => Some(function_ir),
            _ => None,
        }
    }

    /// Create a new macro context for a derive macro on an interface
    pub fn new_derive_interface(
        macro_name: String,
        module_path: String,
        decorator_span: SpanIR,
        target_span: SpanIR,
        file_name: String,
        interface: InterfaceIR,
        target_source: String,
    ) -> Self {
        Self {
            abi_version: 1,
            macro_kind: MacroKind::Derive,
            macro_name,
            module_path,
            decorator_span,
            macro_name_span: None,
            target_span,
            file_name,
            target: TargetIR::Interface(interface),
            target_source,
            import_registry: ImportRegistry::new(),
            config: None,
            type_registry: None,
            resolved_fields: None,
        }
    }

    /// Create a new macro context for a derive macro on a type alias
    pub fn new_derive_type_alias(
        macro_name: String,
        module_path: String,
        decorator_span: SpanIR,
        target_span: SpanIR,
        file_name: String,
        type_alias: TypeAliasIR,
        target_source: String,
    ) -> Self {
        Self {
            abi_version: 1,
            macro_kind: MacroKind::Derive,
            macro_name,
            module_path,
            decorator_span,
            macro_name_span: None,
            target_span,
            file_name,
            target: TargetIR::TypeAlias(type_alias),
            target_source,
            import_registry: ImportRegistry::new(),
            config: None,
            type_registry: None,
            resolved_fields: None,
        }
    }

    /// Create a new macro context for a derive macro on an enum
    pub fn new_derive_enum(
        macro_name: String,
        module_path: String,
        decorator_span: SpanIR,
        target_span: SpanIR,
        file_name: String,
        enum_ir: EnumIR,
        target_source: String,
    ) -> Self {
        Self {
            abi_version: 1,
            macro_kind: MacroKind::Derive,
            macro_name,
            module_path,
            decorator_span,
            macro_name_span: None,
            target_span,
            file_name,
            target: TargetIR::Enum(enum_ir),
            target_source,
            import_registry: ImportRegistry::new(),
            config: None,
            type_registry: None,
            resolved_fields: None,
        }
    }

    /// Create a new macro context for an attribute macro on a function
    pub fn new_attribute_function(
        macro_name: String,
        module_path: String,
        decorator_span: SpanIR,
        target_span: SpanIR,
        file_name: String,
        function_ir: FunctionIR,
        target_source: String,
    ) -> Self {
        Self {
            abi_version: 1,
            macro_kind: MacroKind::Attribute,
            macro_name,
            module_path,
            decorator_span,
            macro_name_span: None,
            target_span,
            file_name,
            target: TargetIR::Function(function_ir),
            target_source,
            import_registry: ImportRegistry::new(),
            config: None,
            type_registry: None,
            resolved_fields: None,
        }
    }

    /// Create a new macro context for an attribute macro on a class
    pub fn new_attribute_class(
        macro_name: String,
        module_path: String,
        decorator_span: SpanIR,
        target_span: SpanIR,
        file_name: String,
        class: ClassIR,
        target_source: String,
    ) -> Self {
        Self {
            abi_version: 1,
            macro_kind: MacroKind::Attribute,
            macro_name,
            module_path,
            decorator_span,
            macro_name_span: None,
            target_span,
            file_name,
            target: TargetIR::Class(class),
            target_source,
            import_registry: ImportRegistry::new(),
            config: None,
            type_registry: None,
            resolved_fields: None,
        }
    }

    /// Create a new macro context for an attribute macro on an interface
    pub fn new_attribute_interface(
        macro_name: String,
        module_path: String,
        decorator_span: SpanIR,
        target_span: SpanIR,
        file_name: String,
        interface: InterfaceIR,
        target_source: String,
    ) -> Self {
        Self {
            abi_version: 1,
            macro_kind: MacroKind::Attribute,
            macro_name,
            module_path,
            decorator_span,
            macro_name_span: None,
            target_span,
            file_name,
            target: TargetIR::Interface(interface),
            target_source,
            import_registry: ImportRegistry::new(),
            config: None,
            type_registry: None,
            resolved_fields: None,
        }
    }

    /// Create a new macro context for an attribute macro on an enum
    pub fn new_attribute_enum(
        macro_name: String,
        module_path: String,
        decorator_span: SpanIR,
        target_span: SpanIR,
        file_name: String,
        enum_ir: EnumIR,
        target_source: String,
    ) -> Self {
        Self {
            abi_version: 1,
            macro_kind: MacroKind::Attribute,
            macro_name,
            module_path,
            decorator_span,
            macro_name_span: None,
            target_span,
            file_name,
            target: TargetIR::Enum(enum_ir),
            target_source,
            import_registry: ImportRegistry::new(),
            config: None,
            type_registry: None,
            resolved_fields: None,
        }
    }

    /// Create a new macro context for an attribute macro on a type alias
    pub fn new_attribute_type_alias(
        macro_name: String,
        module_path: String,
        decorator_span: SpanIR,
        target_span: SpanIR,
        file_name: String,
        type_alias: TypeAliasIR,
        target_source: String,
    ) -> Self {
        Self {
            abi_version: 1,
            macro_kind: MacroKind::Attribute,
            macro_name,
            module_path,
            decorator_span,
            macro_name_span: None,
            target_span,
            file_name,
            target: TargetIR::TypeAlias(type_alias),
            target_source,
            import_registry: ImportRegistry::new(),
            config: None,
            type_registry: None,
            resolved_fields: None,
        }
    }

    /// Returns the module specifier the current file should use to `import`
    /// `type_name`. Returns `None` when the type is co-located, unknown, or
    /// the path arithmetic fails.
    ///
    /// Resolution order:
    /// 1. If the current file already imports `type_name`, return its existing
    ///    specifier (preserves aliases and the user's chosen path style).
    /// 2. Otherwise look up `type_name` in `self.type_registry`.
    /// 3. If the type lives in `self.file_name`, return `None` (no import needed).
    /// 4. Compute the relative path from the current file's directory to
    ///    `entry.file_path`, strip TypeScript file extensions, and prepend
    ///    `./` if needed.
    pub fn import_specifier_for(&self, type_name: &str) -> Option<String> {
        if let Some(existing) = self.import_registry.get_source(type_name)
            && !existing.is_empty()
        {
            return Some(existing.to_string());
        }
        let registry = self.type_registry.as_ref()?;
        let entry = registry.types.get(type_name)?;
        if entry.file_path == self.file_name {
            return None;
        }
        relative_module_specifier(&self.file_name, &entry.file_path)
    }
}

/// Computes a TypeScript module specifier for `target_file` relative to the
/// directory containing `current_file`. Strips `.svelte.ts` → `.svelte`,
/// `.tsx` → empty, `.ts` → empty, and prepends `./` for sibling/descendant
/// paths.
fn relative_module_specifier(current_file: &str, target_file: &str) -> Option<String> {
    use std::path::Path;
    let from = Path::new(current_file).parent()?;
    let to = Path::new(target_file);
    let rel = pathdiff::diff_paths(to, from)?;
    let mut s = rel.to_string_lossy().replace('\\', "/");

    if let Some(stripped) = s.strip_suffix(".svelte.ts") {
        s = format!("{stripped}.svelte");
    } else if let Some(stripped) = s.strip_suffix(".tsx") {
        s = stripped.to_string();
    } else if let Some(stripped) = s.strip_suffix(".ts") {
        s = stripped.to_string();
    }

    if !s.starts_with('.') && !s.starts_with('/') {
        s = format!("./{s}");
    }
    Some(s)
}

#[cfg(test)]
mod import_specifier_tests {
    use super::*;
    use crate::SpanIR;
    use crate::abi::ir::interface::InterfaceIR;
    use crate::abi::ir::type_registry::{TypeDefinitionIR, TypeRegistry, TypeRegistryEntry};

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

    fn make_ctx(file_name: &str) -> MacroContextIR {
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
            type_registry: None,
            resolved_fields: None,
        }
    }

    fn registry_with(name: &str, file_path: &str) -> TypeRegistry {
        let mut reg = TypeRegistry {
            types: HashMap::new(),
            qualified_types: HashMap::new(),
            ambiguous_names: vec![],
        };
        reg.types.insert(
            name.to_string(),
            TypeRegistryEntry {
                name: name.to_string(),
                file_path: file_path.to_string(),
                is_exported: true,
                definition: TypeDefinitionIR::Interface(empty_interface(name)),
                file_imports: vec![],
            },
        );
        reg
    }

    #[test]
    fn existing_import_takes_priority() {
        let mut ctx = make_ctx("/foo/order.svelte.ts");
        ctx.import_registry.install_source_imports(vec![
            crate::import_registry::SourceImportEntry {
                local_name: "Customer".to_string(),
                source_module: "./customer.svelte".to_string(),
                original_name: None,
                is_type_only: true,
            },
        ]);
        ctx.type_registry = Some(registry_with(
            "Customer",
            "/totally/different/path.svelte.ts",
        ));
        assert_eq!(
            ctx.import_specifier_for("Customer"),
            Some("./customer.svelte".to_string())
        );
    }

    #[test]
    fn co_located_returns_none() {
        let mut ctx = make_ctx("/foo/bar.svelte.ts");
        ctx.type_registry = Some(registry_with("Bar", "/foo/bar.svelte.ts"));
        assert_eq!(ctx.import_specifier_for("Bar"), None);
    }

    #[test]
    fn sibling_file() {
        let mut ctx = make_ctx("/foo/order.svelte.ts");
        ctx.type_registry = Some(registry_with("Customer", "/foo/customer.svelte.ts"));
        assert_eq!(
            ctx.import_specifier_for("Customer"),
            Some("./customer.svelte".to_string())
        );
    }

    #[test]
    fn nested_subdir() {
        let mut ctx = make_ctx("/foo/order.svelte.ts");
        ctx.type_registry = Some(registry_with("Customer", "/foo/sub/customer.svelte.ts"));
        assert_eq!(
            ctx.import_specifier_for("Customer"),
            Some("./sub/customer.svelte".to_string())
        );
    }

    #[test]
    fn parent_dir() {
        let mut ctx = make_ctx("/foo/sub/order.svelte.ts");
        ctx.type_registry = Some(registry_with("Customer", "/foo/customer.svelte.ts"));
        assert_eq!(
            ctx.import_specifier_for("Customer"),
            Some("../customer.svelte".to_string())
        );
    }

    #[test]
    fn unknown_type() {
        let ctx = make_ctx("/foo/order.svelte.ts");
        assert_eq!(ctx.import_specifier_for("Customer"), None);
    }

    #[test]
    fn plain_ts_file() {
        let mut ctx = make_ctx("/foo/order.svelte.ts");
        ctx.type_registry = Some(registry_with("Util", "/foo/util.ts"));
        assert_eq!(ctx.import_specifier_for("Util"), Some("./util".to_string()));
    }

    #[test]
    fn tsx_file() {
        let mut ctx = make_ctx("/foo/order.svelte.ts");
        ctx.type_registry = Some(registry_with("Component", "/foo/component.tsx"));
        assert_eq!(
            ctx.import_specifier_for("Component"),
            Some("./component".to_string())
        );
    }
}
