//! # Macroforge Configuration Types
//!
//! Serializable configuration types shared between the host process and external macro
//! processes. These live in `macroforge_ts_syn` so they can be used in [`MacroContextIR`]
//! for cross-process transfer.
//!
//! The config parsing logic (reading `macroforge.config.ts` via SWC) remains in
//! `macroforge_ts::host::config`.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Information about an imported function.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportInfo {
    /// The imported name (or "default" for default imports).
    pub name: String,
    /// The module specifier.
    pub source: String,
}

/// An alias for a foreign type that allows matching different name-package pairs.
///
/// This is useful when a type can be imported from different paths or with different names.
///
/// ## Example
///
/// ```javascript
/// foreignTypes: {
///   "DateTime.DateTime": {
///     from: ["effect"],
///     aliases: [
///       { name: "DateTime", from: "effect/DateTime" }
///     ],
///     serialize: (v) => DateTime.formatIso(v),
///     // ...
///   }
/// }
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ForeignTypeAlias {
    /// The type name to match (e.g., "DateTime" or "DateTime.DateTime").
    pub name: String,
    /// The import source to match (e.g., "effect/DateTime").
    pub from: String,
}

/// Configuration for a single foreign type.
///
/// Foreign types allow global registration of handlers for external types
/// (like Effect's `DateTime`) so they work like primitives without per-field annotations.
///
/// ## Key Format
///
/// The key in `foreignTypes` should be the fully qualified type name as used in code:
/// - Simple type name: `"DateTime"` - matches `DateTime` in code
/// - Fully qualified: `"DateTime.DateTime"` - matches `DateTime.DateTime` (namespace.type pattern)
///
/// ## Import Source Validation
///
/// Foreign types are only matched when the type is imported from a source listed in
/// `from` or one of the `aliases`. Types with the same name from different packages
/// are ignored (fall back to generic handling).
///
/// ## Example
///
/// ```javascript
/// foreignTypes: {
///   // For Effect's DateTime where you import { DateTime } and use DateTime.DateTime
///   "DateTime.DateTime": {
///     from: ["effect"],
///     aliases: [
///       { name: "DateTime", from: "effect/DateTime" },
///       { name: "MyDateTime", from: "my-effect-wrapper" }
///     ],
///     serialize: (v) => DateTime.formatIso(v),
///     deserialize: (raw) => DateTime.unsafeFromDate(new Date(raw)),
///     default: () => DateTime.unsafeNow()
///   }
/// }
/// ```
///
/// This configuration matches:
/// - `import { DateTime } from 'effect'` with type `DateTime.DateTime`
/// - `import { DateTime } from 'effect/DateTime'` with type `DateTime`
/// - `import { MyDateTime } from 'my-effect-wrapper'` with type `MyDateTime`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ForeignTypeConfig {
    /// The full type key as specified in config (e.g., "DateTime" or "DateTime.DateTime").
    /// This is the key from the foreignTypes object.
    pub name: String,

    /// Optional namespace for the type (e.g., "DateTime" for DateTime.DateTime).
    /// If specified, the type is accessed as `namespace.typeName`.
    /// If not specified, defaults to the first segment of the name if it contains a dot.
    pub namespace: Option<String>,

    /// Import sources where this type can come from (e.g., ["effect", "effect/DateTime"]).
    /// Used to validate that the type is imported from the correct module.
    pub from: Vec<String>,

    /// Serialization function expression (e.g., "(v, ctx) => v.toJSON()").
    pub serialize_expr: Option<String>,

    /// Import info if serialize is a named function from another module.
    pub serialize_import: Option<ImportInfo>,

    /// Deserialization function expression.
    pub deserialize_expr: Option<String>,

    /// Import info if deserialize is a named function from another module.
    pub deserialize_import: Option<ImportInfo>,

    /// Default value function expression (e.g., "() => DateTime.now()").
    pub default_expr: Option<String>,

    /// Import info if default is a named function from another module.
    pub default_import: Option<ImportInfo>,

    /// Shape-check predicate expression for union variant matching.
    /// Used when this foreign type appears as a variant in a union type alias.
    /// The expression should be a function `(value: unknown) => boolean`.
    /// Example: `(v: unknown) => typeof v === "string"` for types deserialized from strings.
    pub has_shape_expr: Option<String>,

    /// Import info if hasShape is a named function from another module.
    pub has_shape_import: Option<ImportInfo>,

    /// Aliases for this foreign type, allowing different name-package pairs to use the same config.
    #[serde(default)]
    pub aliases: Vec<ForeignTypeAlias>,

    /// Namespaces referenced in expressions (serialize_expr, deserialize_expr, default_expr).
    ///
    /// This is auto-extracted during config parsing by analyzing the expression ASTs.
    /// For example, if `serialize: (v) => DateTime.formatIso(v)`, this would contain `["DateTime"]`.
    ///
    /// Used to determine which namespaces need to be imported for the generated code to work.
    #[serde(default)]
    pub expression_namespaces: Vec<String>,
}

impl ForeignTypeConfig {
    /// Returns the namespace for this type.
    /// If `namespace` is explicitly set, returns that.
    /// Otherwise, if the name contains a dot (e.g., "Deep.A.B.Type"), returns everything before the last dot.
    /// Otherwise, returns None.
    pub fn get_namespace(&self) -> Option<&str> {
        if let Some(ref ns) = self.namespace {
            return Some(ns);
        }
        // If name contains a dot, extract namespace (everything before the last dot)
        if let Some(dot_idx) = self.name.rfind('.') {
            return Some(&self.name[..dot_idx]);
        }
        None
    }

    /// Returns the simple type name (last segment after dots).
    /// For "DateTime.DateTime", returns "DateTime".
    /// For "DateTime", returns "DateTime".
    pub fn get_type_name(&self) -> &str {
        self.name.rsplit('.').next().unwrap_or(&self.name)
    }

    /// Returns the full qualified name to match against.
    /// If namespace is set: "namespace.typeName"
    /// Otherwise: the name as-is
    pub fn get_qualified_name(&self) -> String {
        if let Some(ns) = self.get_namespace() {
            let type_name = self.get_type_name();
            if ns != type_name {
                return format!("{}.{}", ns, type_name);
            }
        }
        self.name.clone()
    }
}

/// Configuration for the macro host system.
///
/// This struct represents the contents of a `macroforge.config.js` file.
/// It controls macro loading, execution, and foreign type handling.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MacroforgeConfig {
    /// Whether to preserve `@derive` decorators in the output code.
    ///
    /// When `false` (default), decorators are stripped after expansion.
    /// When `true`, decorators remain in the output (useful for debugging).
    #[serde(default)]
    pub keep_decorators: bool,

    /// Whether to generate a convenience const for non-class types.
    ///
    /// When `true` (default), generates an `export const TypeName = { ... } as const;`
    /// that groups all generated functions for a type into a single namespace-like object.
    #[serde(default = "default_generate_convenience_const")]
    pub generate_convenience_const: bool,

    /// Foreign type configurations.
    ///
    /// Maps type names to their handlers for serialization, deserialization, and defaults.
    #[serde(default)]
    pub foreign_types: Vec<ForeignTypeConfig>,

    /// Import sources from the config file itself.
    ///
    /// Maps imported names (e.g., "DateTime", "Option") to their import info
    /// (module source). This is used to determine the correct import source
    /// when generating namespace imports for foreign type expressions.
    #[serde(default, skip_serializing)]
    pub config_imports: HashMap<String, ImportInfo>,
}

/// Returns the default for generate_convenience_const (true).
pub fn default_generate_convenience_const() -> bool {
    true
}

impl Default for MacroforgeConfig {
    fn default() -> Self {
        Self {
            keep_decorators: false,
            generate_convenience_const: true,
            foreign_types: Vec::new(),
            config_imports: HashMap::new(),
        }
    }
}
