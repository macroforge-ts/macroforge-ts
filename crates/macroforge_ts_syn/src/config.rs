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

/// Build flags consumed by the `@cfg` attribute macro.
///
/// The predicate in `/** @cfg({ feature: 'ssr' }) */` evaluates against these
/// flags: keys with single-value configs (e.g. `target`) match exactly; keys
/// whose config value is an array (e.g. `features`) match when the annotation
/// value is a member; multiple keys in one annotation combine with implicit AND.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CfgFlags {
    /// Active feature flags. `@cfg({ feature: 'ssr' })` passes when `'ssr'` is in this list.
    #[serde(default)]
    pub features: Vec<String>,

    /// Build target (e.g. `"web"`, `"node"`, `"deno"`). `@cfg({ target: 'web' })` matches exactly.
    #[serde(default)]
    pub target: Option<String>,

    /// Whether this build treats `@cfg({ debugAssertions: true })` as truthy.
    #[serde(default)]
    pub debug_assertions: bool,

    /// Arbitrary string-keyed predicate values. Accepts any JSON scalar so
    /// annotations like `@cfg({ tenant: 'acme' })` or `@cfg({ version: 2 })`
    /// can match exactly.
    #[serde(default)]
    pub custom: HashMap<String, serde_json::Value>,
}

/// Behavior knobs for the `@deprecated` attribute macro.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeprecatedConfig {
    /// Inject a one-shot `console.warn(...)` into the deprecated declaration's
    /// body so runtime use is visible alongside tsc's static `@deprecated` tag.
    #[serde(default = "crate::config::default_true")]
    pub runtime_warn: bool,

    /// Treat any use of a `@deprecated` symbol as a macro-expansion error.
    /// Off by default; turn on when chasing deprecations out of a codebase.
    #[serde(default)]
    pub fail_on_use: bool,
}

impl Default for DeprecatedConfig {
    fn default() -> Self {
        Self {
            runtime_warn: true,
            fail_on_use: false,
        }
    }
}

/// Enforcement strategy for `@mustUse`. Only one mode today; keeping this as
/// an enum reserves room for a future `Wrap` variant without a breaking change.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MustUseMode {
    /// Emit a macroforge diagnostic at the discarded-call site. No runtime cost.
    #[default]
    Lint,
}

/// Behavior knobs for the `@mustUse` attribute macro.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MustUseConfig {
    #[serde(default)]
    pub mode: MustUseMode,
}

/// Behavior knobs for the `@nonExhaustive` attribute macro.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NonExhaustiveConfig {
    /// Brand property name used in the intersection. Keep this stable across a
    /// project so downstream consumers can pattern-match on it.
    #[serde(default = "default_non_exhaustive_brand")]
    pub brand: String,
}

impl Default for NonExhaustiveConfig {
    fn default() -> Self {
        Self {
            brand: default_non_exhaustive_brand(),
        }
    }
}

fn default_non_exhaustive_brand() -> String {
    "__nonExhaustive".to_string()
}

pub(crate) fn default_true() -> bool {
    true
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

    /// Build flags consumed by `@cfg`. Missing key is equivalent to an empty block.
    #[serde(default)]
    pub cfg: CfgFlags,

    /// Knobs for `@deprecated`. Missing key uses per-field defaults.
    #[serde(default)]
    pub deprecated: DeprecatedConfig,

    /// Knobs for `@mustUse`. Missing key = lint-mode diagnostic.
    #[serde(default)]
    pub must_use: MustUseConfig,

    /// Knobs for `@nonExhaustive`. Missing key = default brand name.
    #[serde(default)]
    pub non_exhaustive: NonExhaustiveConfig,

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
            cfg: CfgFlags::default(),
            deprecated: DeprecatedConfig::default(),
            must_use: MustUseConfig::default(),
            non_exhaustive: NonExhaustiveConfig::default(),
            config_imports: HashMap::new(),
        }
    }
}
