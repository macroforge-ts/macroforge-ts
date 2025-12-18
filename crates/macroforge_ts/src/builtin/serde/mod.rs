//! # Serde (Serialization/Deserialization) Module
//!
//! This module provides the `Serialize` and `Deserialize` macros for JSON
//! serialization with cycle detection and validation support.
//!
//! ## Generated Methods
//!
//! ### Serialize
//!
//! - `serialize(): string` - Serialize to JSON string
//! - `SerializeWithContext(ctx): Record<string, unknown>` - Internal method with cycle detection
//!
//! ### Deserialize
//!
//! - `static deserialize(input: unknown): Result<T, Error[]>` - Parse and validate (auto-detects string vs object)
//! - `static deserializeWithContext(value, ctx): T` - Internal method with cycle resolution
//!
//! ## Cycle Detection
//!
//! Both macros support cycle detection for object graphs with circular references:
//!
//! ```typescript
//! /** @derive(Serialize, Deserialize) */
//! class Node {
//!     value: number;
//!     next: Node | null;
//! }
//! // Creates: { __type: "Node", __id: 1, value: 1, next: { __ref: 1 } }
//! ```
//!
//! ## Field-Level Options
//!
//! The `@serde` decorator supports many options:
//!
//! | Option | Description |
//! |--------|-------------|
//! | `skip` | Skip both serialization and deserialization |
//! | `skipSerializing` | Skip only during serialization |
//! | `skipDeserializing` | Skip only during deserialization |
//! | `rename = "name"` | Use a different JSON key |
//! | `default` | Use type's default if missing |
//! | `default = "expr"` | Use specific expression if missing |
//! | `flatten` | Flatten nested object fields into parent |
//! | `serializeWith = "fn"` | Use custom function for serialization |
//! | `deserializeWith = "fn"` | Use custom function for deserialization |
//!
//! ## Container-Level Options
//!
//! | Option | Description |
//! |--------|-------------|
//! | `renameAll = "camelCase"` | Apply naming convention to all fields |
//! | `denyUnknownFields` | Reject JSON with extra fields |
//!
//! ## Naming Conventions
//!
//! Supported values for `renameAll`:
//! - `camelCase` - `user_name` → `userName`
//! - `snake_case` - `userName` → `user_name`
//! - `SCREAMING_SNAKE_CASE` - `userName` → `USER_NAME`
//! - `kebab-case` - `userName` → `user-name`
//! - `PascalCase` - `user_name` → `UserName`
//!
//! ## Validation
//!
//! The Deserialize macro supports 30+ validators for runtime validation:
//!
//! ### String Validators
//! - `email` - Valid email format
//! - `url` - Valid URL format
//! - `uuid` - Valid UUID format
//! - `pattern("regex")` - Match regex pattern
//! - `minLength(n)`, `maxLength(n)`, `length(n)` - Length constraints
//! - `nonEmpty`, `trimmed` - Content requirements
//! - `lowercase`, `uppercase`, `capitalized` - Case requirements
//! - `startsWith("prefix")`, `endsWith("suffix")`, `includes("text")`
//!
//! ### Number Validators
//! - `int` - Must be integer
//! - `positive`, `negative`, `nonNegative`, `nonPositive`
//! - `greaterThan(n)`, `lessThan(n)`, `between(min, max)`
//! - `multipleOf(n)`, `uint8`, `finite`, `nonNaN`
//!
//! ### Array Validators
//! - `minItems(n)`, `maxItems(n)`, `itemsCount(n)`
//!
//! ### Date Validators
//! - `validDate` - Must be valid Date
//! - `afterDate("2020-01-01")`, `beforeDate("2030-01-01")`
//! - `betweenDates("start", "end")`
//!
//! ### Custom Validators
//! - `custom(functionName)` - Call custom validation function
//!
//! ## Example
//!
//! ```typescript
//! /** @derive(Serialize, Deserialize) @serde({ renameAll: "camelCase" }) */
//! class User {
//!     /** @serde({ validate: { email: true } }) */
//!     emailAddress: string;
//!
//!     /** @serde({ validate: { minLength: 3, maxLength: 50 } }) */
//!     username: string;
//!
//!     /** @serde({ skipSerializing: true }) */
//!     password: string;
//!
//!     /** @serde({ default: true }) */
//!     role: string;
//!
//!     /** @serde({ flatten: true }) */
//!     metadata: UserMetadata;
//! }
//! ```
//!
//! ## Custom Serialization Functions
//!
//! For foreign types that can't be automatically serialized, use custom functions:
//!
//! ```typescript
//! import { ZonedDateTime } from "@internationalized/date";
//!
//! // Custom serializer/deserializer functions
//! function serializeZoned(value: ZonedDateTime): unknown {
//!     return value.toAbsoluteString();
//! }
//! function deserializeZoned(raw: unknown): ZonedDateTime {
//!     return parseZonedDateTime(raw as string);
//! }
//!
//! /** @derive(Serialize, Deserialize) */
//! interface Event {
//!     name: string;
//!     /** @serde({ serializeWith: "serializeZoned", deserializeWith: "deserializeZoned" }) */
//!     startTime: ZonedDateTime;
//! }
//! ```
//!
//! ## Foreign Types (Global Configuration)
//!
//! Instead of adding `serializeWith`/`deserializeWith` to every field, you can configure
//! foreign type handlers globally in `macroforge.config.js`:
//!
//! ```javascript
//! // macroforge.config.js
//! import { DateTime } from "effect";
//!
//! export default {
//!   foreignTypes: {
//!     "DateTime.DateTime": {
//!       from: ["effect"],
//!       aliases: [
//!         { name: "DateTime", from: "effect/DateTime" }
//!       ],
//!       serialize: (v) => DateTime.formatIso(v),
//!       deserialize: (raw) => DateTime.unsafeFromDate(new Date(raw)),
//!       default: () => DateTime.unsafeNow()
//!     }
//!   }
//! }
//! ```
//!
//! With this configuration, any field typed as `DateTime.DateTime` (imported from `effect`)
//! or `DateTime` (imported from `effect/DateTime`) will automatically use the configured
//! handlers without needing per-field decorators.
//!
//! ### Foreign Type Options
//!
//! | Option | Description |
//! |--------|-------------|
//! | `from` | Array of module paths this type can be imported from |
//! | `aliases` | Array of `{ name, from }` objects for alternative type-package pairs |
//! | `serialize` | Function `(value) => unknown` for serialization |
//! | `deserialize` | Function `(raw) => T` for deserialization |
//! | `default` | Function `() => T` for default value generation |
//!
//! ### Import Source Validation
//!
//! Foreign types are only matched when the type is imported from one of the configured
//! sources. Types with the same name from different packages are ignored and fall back
//! to generic handling (TypeScript will catch any issues downstream).
//!
//! See the [Configuration](crate::host::config) module for more details.

/// Deserialize macro implementation.
pub mod derive_deserialize;

/// Serialize macro implementation.
pub mod derive_serialize;

use crate::host::ForeignTypeConfig;
use crate::ts_syn::abi::{DecoratorIR, DiagnosticCollector, SpanIR};
use std::cell::RefCell;
use std::collections::HashMap;

// ============================================================================
// Thread-local storage for current expansion's foreign types and import sources
// ============================================================================

thread_local! {
    /// Thread-local storage for foreign type configurations during expansion.
    ///
    /// This is set by the expander before running macros and cleared after.
    /// Macros can query this to check if a field's type matches a foreign type.
    static FOREIGN_TYPES: RefCell<Vec<ForeignTypeConfig>> = const{ RefCell::new(Vec::new()) };

    /// Thread-local storage for import sources during expansion.
    ///
    /// Maps type/identifier names to their import module sources.
    /// Used to validate that foreign types are imported from the correct modules.
    static IMPORT_SOURCES: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());

    /// Thread-local storage for import aliases during expansion.
    ///
    /// Maps local alias names to their original imported names.
    /// For `import { Option as EffectOption } from "effect/Option"`,
    /// this would contain `"EffectOption" -> "Option"`.
    static IMPORT_ALIASES: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());

    /// Thread-local storage for type-only import tracking during expansion.
    ///
    /// Maps identifier names to whether they are type-only imports.
    /// For `import type { DateTime } from "effect"`, this contains `"DateTime" -> true`.
    /// For `import { DateTime } from "effect"`, this contains `"DateTime" -> false`.
    static TYPE_ONLY_IMPORTS: RefCell<HashMap<String, bool>> = RefCell::new(HashMap::new());

    /// Thread-local storage for required namespace imports during expansion.
    ///
    /// Accumulates namespaces that need to be imported for foreign type expressions.
    /// Maps namespace name -> (module_source, alias_to_use).
    /// For example, if `DateTime.formatIso` is used and DateTime is type-only imported,
    /// this would contain `"DateTime" -> ("effect/DateTime", "__mf_DateTime")`.
    static REQUIRED_NS_IMPORTS: RefCell<HashMap<String, (String, String)>> = RefCell::new(HashMap::new());

    /// Thread-local storage for config file imports during expansion.
    ///
    /// Stores the import sources from the config file (e.g., `import { DateTime } from "effect"`).
    /// Maps identifier names to their module sources.
    /// Used to determine the correct import source when generating namespace imports.
    static CONFIG_IMPORTS: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

/// Set the foreign types for the current expansion.
///
/// This should be called by the expander before running macros.
/// The previous value is returned so it can be restored after expansion.
pub fn set_foreign_types(types: Vec<ForeignTypeConfig>) -> Vec<ForeignTypeConfig> {
    FOREIGN_TYPES.with(|ft| ft.replace(types))
}

/// Get a reference to the current foreign types.
///
/// Returns a clone of the current foreign types for thread-safety.
pub fn get_foreign_types() -> Vec<ForeignTypeConfig> {
    FOREIGN_TYPES.with(|ft| ft.borrow().clone())
}

/// Clear the foreign types after expansion.
pub fn clear_foreign_types() {
    FOREIGN_TYPES.with(|ft| ft.borrow_mut().clear());
}

/// Set the import sources for the current expansion.
///
/// This should be called by the expander before running macros.
/// The previous value is returned so it can be restored after expansion.
pub fn set_import_sources(sources: HashMap<String, String>) -> HashMap<String, String> {
    IMPORT_SOURCES.with(|is| is.replace(sources))
}

/// Get a reference to the current import sources.
///
/// Returns a clone of the current import sources for thread-safety.
pub fn get_import_sources() -> HashMap<String, String> {
    IMPORT_SOURCES.with(|is| is.borrow().clone())
}

/// Clear the import sources after expansion.
pub fn clear_import_sources() {
    IMPORT_SOURCES.with(|is| is.borrow_mut().clear());
}

/// Set the import aliases for the current expansion.
///
/// Maps local alias names to their original imported names.
/// For `import { Option as EffectOption }`, stores `"EffectOption" -> "Option"`.
pub fn set_import_aliases(aliases: HashMap<String, String>) -> HashMap<String, String> {
    IMPORT_ALIASES.with(|ia| ia.replace(aliases))
}

/// Get a reference to the current import aliases.
///
/// Returns a clone of the current import aliases for thread-safety.
pub fn get_import_aliases() -> HashMap<String, String> {
    IMPORT_ALIASES.with(|ia| ia.borrow().clone())
}

/// Clear the import aliases after expansion.
pub fn clear_import_aliases() {
    IMPORT_ALIASES.with(|ia| ia.borrow_mut().clear());
}

/// Set the type-only import tracking for the current expansion.
///
/// Maps identifier names to whether they are type-only imports.
/// This should be called by the expander before running macros.
pub fn set_type_only_imports(imports: HashMap<String, bool>) -> HashMap<String, bool> {
    TYPE_ONLY_IMPORTS.with(|toi| toi.replace(imports))
}

/// Get a reference to the current type-only import tracking.
///
/// Returns a clone of the current tracking for thread-safety.
pub fn get_type_only_imports() -> HashMap<String, bool> {
    TYPE_ONLY_IMPORTS.with(|toi| toi.borrow().clone())
}

/// Clear the type-only import tracking after expansion.
pub fn clear_type_only_imports() {
    TYPE_ONLY_IMPORTS.with(|toi| toi.borrow_mut().clear());
}

/// Register a required namespace import for foreign type expressions.
///
/// When a foreign type expression uses a namespace (e.g., `DateTime.formatIso`),
/// and that namespace is not available as a value import, this registers the
/// need to generate an import for it.
///
/// # Arguments
/// * `namespace` - The namespace identifier (e.g., "DateTime")
/// * `module` - The module to import from (e.g., "effect/DateTime")
/// * `alias` - The alias to use in generated code (e.g., "__mf_DateTime")
pub fn register_required_namespace(namespace: &str, module: &str, alias: &str) {
    REQUIRED_NS_IMPORTS.with(|ns| {
        ns.borrow_mut()
            .insert(namespace.to_string(), (module.to_string(), alias.to_string()));
    });
}

/// Get all required namespace imports accumulated during expansion.
///
/// Returns a HashMap where keys are namespace names and values are (module, alias) tuples.
pub fn get_required_namespace_imports() -> HashMap<String, (String, String)> {
    REQUIRED_NS_IMPORTS.with(|ns| ns.borrow().clone())
}

/// Clear the required namespace imports after expansion.
pub fn clear_required_namespace_imports() {
    REQUIRED_NS_IMPORTS.with(|ns| ns.borrow_mut().clear());
}

/// Set the config file imports for the current expansion.
///
/// Maps identifier names to their module sources from the config file.
/// This should be called by the expander before running macros.
pub fn set_config_imports(imports: HashMap<String, String>) -> HashMap<String, String> {
    CONFIG_IMPORTS.with(|ci| ci.replace(imports))
}

/// Get a reference to the current config file imports.
///
/// Returns a clone of the current imports for thread-safety.
pub fn get_config_imports() -> HashMap<String, String> {
    CONFIG_IMPORTS.with(|ci| ci.borrow().clone())
}

/// Clear the config file imports after expansion.
pub fn clear_config_imports() {
    CONFIG_IMPORTS.with(|ci| ci.borrow_mut().clear());
}

/// Naming convention for JSON field renaming
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum RenameAll {
    #[default]
    None,
    CamelCase,
    SnakeCase,
    ScreamingSnakeCase,
    KebabCase,
    PascalCase,
}

impl std::str::FromStr for RenameAll {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().replace(['-', '_'], "").as_str() {
            "camelcase" => Ok(Self::CamelCase),
            "snakecase" => Ok(Self::SnakeCase),
            "screamingsnakecase" => Ok(Self::ScreamingSnakeCase),
            "kebabcase" => Ok(Self::KebabCase),
            "pascalcase" => Ok(Self::PascalCase),
            _ => Err(()),
        }
    }
}

impl RenameAll {
    pub fn apply(&self, name: &str) -> String {
        use convert_case::{Case, Casing};
        match self {
            Self::None => name.to_string(),
            Self::CamelCase => name.to_case(Case::Camel),
            Self::SnakeCase => name.to_case(Case::Snake),
            Self::ScreamingSnakeCase => name.to_case(Case::UpperSnake),
            Self::KebabCase => name.to_case(Case::Kebab),
            Self::PascalCase => name.to_case(Case::Pascal),
        }
    }
}

/// Container-level serde options (on the class/interface itself)
#[derive(Debug, Clone, Default)]
pub struct SerdeContainerOptions {
    pub rename_all: RenameAll,
    pub deny_unknown_fields: bool,
}

impl SerdeContainerOptions {
    pub fn from_decorators(decorators: &[DecoratorIR]) -> Self {
        let mut opts = Self::default();
        for decorator in decorators {
            if !decorator.name.eq_ignore_ascii_case("serde") {
                continue;
            }
            let args = decorator.args_src.trim();

            if let Some(rename_all) = extract_named_string(args, "renameAll")
                && let Ok(convention) = rename_all.parse::<RenameAll>()
            {
                opts.rename_all = convention;
            }

            if has_flag(args, "denyUnknownFields") {
                opts.deny_unknown_fields = true;
            }
        }
        opts
    }
}

/// Field-level serde options
#[derive(Debug, Clone, Default)]
pub struct SerdeFieldOptions {
    pub skip: bool,
    pub skip_serializing: bool,
    pub skip_deserializing: bool,
    pub rename: Option<String>,
    pub default: bool,
    pub default_expr: Option<String>,
    pub flatten: bool,
    pub validators: Vec<ValidatorSpec>,
    /// Custom serialization function name (like Rust's `#[serde(serialize_with)]`)
    pub serialize_with: Option<String>,
    /// Custom deserialization function name (like Rust's `#[serde(deserialize_with)]`)
    pub deserialize_with: Option<String>,
}

/// Result of parsing field options, containing both options and any diagnostics
#[derive(Debug, Clone, Default)]
pub struct SerdeFieldParseResult {
    pub options: SerdeFieldOptions,
    pub diagnostics: DiagnosticCollector,
}

impl SerdeFieldOptions {
    /// Parse field options from decorators, collecting diagnostics for invalid configurations
    pub fn from_decorators(decorators: &[DecoratorIR], field_name: &str) -> SerdeFieldParseResult {
        let mut opts = Self::default();
        let mut diagnostics = DiagnosticCollector::new();

        for decorator in decorators {
            if !decorator.name.eq_ignore_ascii_case("serde") {
                continue;
            }
            let args = decorator.args_src.trim();
            let decorator_span = decorator.span;

            if has_flag(args, "skip") {
                opts.skip = true;
            }
            if has_flag(args, "skipSerializing") {
                opts.skip_serializing = true;
            }
            if has_flag(args, "skipDeserializing") {
                opts.skip_deserializing = true;
            }
            if has_flag(args, "flatten") {
                opts.flatten = true;
            }

            // Check for default (both boolean flag and expression)
            if let Some(default_expr) = extract_named_string(args, "default") {
                opts.default = true;
                opts.default_expr = Some(default_expr);
            } else if has_flag(args, "default") {
                opts.default = true;
            }

            if let Some(rename) = extract_named_string(args, "rename") {
                opts.rename = Some(rename);
            }

            // Parse custom serialization/deserialization functions (like Rust's serde)
            if let Some(fn_name) = extract_named_string(args, "serializeWith") {
                opts.serialize_with = Some(fn_name);
            }
            if let Some(fn_name) = extract_named_string(args, "deserializeWith") {
                opts.deserialize_with = Some(fn_name);
            }

            // Extract validators with diagnostic collection
            let validators = extract_validators(args, decorator_span, field_name, &mut diagnostics);
            opts.validators.extend(validators);
        }

        SerdeFieldParseResult {
            options: opts,
            diagnostics,
        }
    }

    pub fn should_serialize(&self) -> bool {
        !self.skip && !self.skip_serializing
    }

    pub fn should_deserialize(&self) -> bool {
        !self.skip && !self.skip_deserializing
    }
}

/// Determines the serialization strategy for a TypeScript type
#[derive(Debug, Clone, PartialEq)]
pub enum TypeCategory {
    Primitive,
    Array(String),
    Optional(String),
    Nullable(String),
    Date,
    Map(String, String),
    Set(String),
    Record(String, String),
    /// Wrapper types like Partial<T>, Required<T>, Readonly<T>, Pick<T, K>, Omit<T, K>, NonNullable<T>
    /// These don't change the runtime value structure, so we serialize based on the inner type.
    /// Contains the inner type name (e.g., "User" for Partial<User>)
    Wrapper(String),
    Serializable(String),
    Unknown,
}

impl TypeCategory {
    pub fn from_ts_type(ts_type: &str) -> Self {
        let trimmed = ts_type.trim();

        // Handle string literal types (e.g., "Zoned", 'foo') - these are primitive-like
        if (trimmed.starts_with('"') && trimmed.ends_with('"'))
            || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
        {
            return Self::Primitive;
        }

        // Handle primitives
        match trimmed {
            "string" | "number" | "boolean" | "null" | "undefined" | "bigint" => {
                return Self::Primitive;
            }
            "Date" => return Self::Date,
            _ => {}
        }

        // Handle Array<T> or T[]
        if trimmed.starts_with("Array<") && trimmed.ends_with('>') {
            let inner = &trimmed[6..trimmed.len() - 1];
            return Self::Array(inner.to_string());
        }
        if let Some(inner) = trimmed.strip_suffix("[]") {
            return Self::Array(inner.to_string());
        }

        // Handle Map<K, V>
        if trimmed.starts_with("Map<") && trimmed.ends_with('>') {
            let inner = &trimmed[4..trimmed.len() - 1];
            if let Some(comma_pos) = find_top_level_comma(inner) {
                let key = inner[..comma_pos].trim().to_string();
                let value = inner[comma_pos + 1..].trim().to_string();
                return Self::Map(key, value);
            }
        }

        // Handle Set<T>
        if trimmed.starts_with("Set<") && trimmed.ends_with('>') {
            let inner = &trimmed[4..trimmed.len() - 1];
            return Self::Set(inner.to_string());
        }

        // Handle Record<K, V>
        if trimmed.starts_with("Record<") && trimmed.ends_with('>') {
            let inner = &trimmed[7..trimmed.len() - 1];
            if let Some(comma_pos) = find_top_level_comma(inner) {
                let key = inner[..comma_pos].trim().to_string();
                let value = inner[comma_pos + 1..].trim().to_string();
                return Self::Record(key, value);
            }
        }

        // Handle union types (T | undefined, T | null)
        if trimmed.contains('|') {
            let parts: Vec<&str> = trimmed.split('|').map(|s| s.trim()).collect();
            if parts.contains(&"undefined") {
                let non_undefined: Vec<&str> = parts
                    .iter()
                    .filter(|p| *p != &"undefined")
                    .copied()
                    .collect();
                return Self::Optional(non_undefined.join(" | "));
            }
            if parts.contains(&"null") {
                let non_null: Vec<&str> = parts.iter().filter(|p| *p != &"null").copied().collect();
                return Self::Nullable(non_null.join(" | "));
            }
        }

        // Check if it looks like a class/interface name (starts with uppercase)
        // Exclude built-in types and utility types
        if let Some(first_char) = trimmed.chars().next()
            && first_char.is_uppercase()
            && !matches!(
                trimmed,
                "String" | "Number" | "Boolean" | "Object" | "Function" | "Symbol"
            )
        {
            // Extract base type name (handle generics like Foo<T>)
            let base_type = if let Some(idx) = trimmed.find('<') {
                &trimmed[..idx]
            } else {
                trimmed
            };

            // Handle TypeScript wrapper utility types that preserve the inner type's structure
            // These don't change runtime values, so we serialize based on the inner type
            if matches!(
                base_type,
                "Partial" | "Required" | "Readonly" | "NonNullable"
            ) {
                // Single type argument: Partial<T>, Required<T>, Readonly<T>, NonNullable<T>
                if let Some(start) = trimmed.find('<') {
                    let inner = &trimmed[start + 1..trimmed.len() - 1];
                    return Self::Wrapper(inner.to_string());
                }
            }

            if matches!(base_type, "Pick" | "Omit") {
                // Two type arguments: Pick<T, K>, Omit<T, K> - we care about T
                if let Some(start) = trimmed.find('<') {
                    let inner = &trimmed[start + 1..trimmed.len() - 1];
                    if let Some(comma_pos) = find_top_level_comma(inner) {
                        let first_arg = inner[..comma_pos].trim();
                        return Self::Wrapper(first_arg.to_string());
                    }
                }
            }

            // These utility types don't have a meaningful serialization strategy
            // (they operate on function types, unions, or async types)
            if matches!(
                base_type,
                "Exclude"
                    | "Extract"
                    | "ReturnType"
                    | "Parameters"
                    | "InstanceType"
                    | "ThisType"
                    | "Awaited"
                    | "Promise"
            ) {
                return Self::Unknown;
            }

            return Self::Serializable(trimmed.to_string());
        }

        Self::Unknown
    }

    /// Check if a type matches a configured foreign type.
    ///
    /// Attempts to match the type name against the configured foreign types,
    /// checking both exact name matches and imports from configured sources.
    ///
    /// # Arguments
    ///
    /// * `ts_type` - The TypeScript type string (e.g., "DateTime", "ZonedDateTime")
    /// * `foreign_types` - List of configured foreign type handlers
    ///
    /// # Returns
    ///
    /// The matching foreign type configuration, if found.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let foreign_types = vec![ForeignTypeConfig {
    ///     name: "DateTime".to_string(),
    ///     from: vec!["effect".to_string()],
    ///     ..Default::default()
    /// }];
    ///
    /// // Matches by exact name
    /// assert!(TypeCategory::match_foreign_type("DateTime", &foreign_types).is_some());
    ///
    /// // Doesn't match other types
    /// assert!(TypeCategory::match_foreign_type("Date", &foreign_types).is_none());
    /// ```
    /// Match a TypeScript type against configured foreign types with import source validation.
    ///
    /// # Arguments
    ///
    /// * `ts_type` - The TypeScript type string (e.g., "DateTime.DateTime", "Duration")
    /// * `foreign_types` - List of configured foreign type handlers
    ///
    /// # Returns
    ///
    /// A `ForeignTypeMatch` containing the matched config (if any) and potential warnings
    /// about near-matches that failed due to import source validation.
    pub fn match_foreign_type<'a>(
        ts_type: &str,
        foreign_types: &'a [ForeignTypeConfig],
    ) -> ForeignTypeMatch<'a> {
        let trimmed = ts_type.trim();
        let import_sources = get_import_sources();
        let import_aliases = get_import_aliases();

        // Skip empty types
        if trimmed.is_empty() {
            return ForeignTypeMatch::none();
        }

        // Extract the base type name (handle generics like Foo<T>)
        let base_type = if let Some(idx) = trimmed.find('<') {
            &trimmed[..idx]
        } else {
            trimmed
        };

        // For qualified types like DateTime.DateTime, extract parts
        let (namespace_part, type_name) = if let Some(dot_idx) = base_type.rfind('.') {
            (Some(&base_type[..dot_idx]), &base_type[dot_idx + 1..])
        } else {
            (None, base_type)
        };

        // Extract the first part of the namespace for import lookup
        // For A.B.C, the import name is A (the first part before any dot)
        let first_namespace_part = namespace_part.map(|ns| {
            if let Some(dot_idx) = ns.find('.') {
                &ns[..dot_idx]
            } else {
                ns
            }
        });

        // Resolve the import name - use first namespace part if qualified, otherwise type_name
        let import_name = first_namespace_part.unwrap_or(type_name);

        // For unqualified types (no namespace), the type_name itself might be an alias
        // e.g., EffectOption -> Option. For qualified types, only the namespace is aliased.
        let resolved_type_name = if namespace_part.is_none() {
            import_aliases
                .get(type_name)
                .map(String::as_str)
                .unwrap_or(type_name)
        } else {
            // For qualified types, the type_name (last part) is not aliased
            type_name
        };

        // For unqualified aliased types, resolve the base_type
        let resolved_base_type = if namespace_part.is_none() {
            import_aliases
                .get(base_type)
                .map(String::as_str)
                .unwrap_or(base_type)
        } else {
            base_type
        };

        // For qualified types, resolve the namespace alias
        // e.g., EffectDateTime.DateTime with import { DateTime as EffectDateTime }
        // should resolve namespace EffectDateTime -> DateTime
        // For A.B.C where A is aliased to X, resolve to X.B.C
        let resolved_namespace: Option<String> = namespace_part.map(|ns| {
            let parts: Vec<&str> = ns.split('.').collect();
            if let Some(first_part) = parts.first() {
                if let Some(resolved_first) = import_aliases.get(*first_part) {
                    // Replace the first part with the resolved alias and rejoin
                    let mut resolved_parts: Vec<&str> = vec![resolved_first.as_str()];
                    resolved_parts.extend(&parts[1..]);
                    resolved_parts.join(".")
                } else {
                    ns.to_string()
                }
            } else {
                ns.to_string()
            }
        });

        let mut near_match: Option<(&ForeignTypeConfig, String)> = None;

        for ft in foreign_types {
            let ft_type_name = ft.get_type_name();
            let ft_namespace = ft.get_namespace();
            let ft_qualified = ft.get_qualified_name();

            // Check if the type name matches (using resolved name for aliased imports)
            let name_matches = type_name == ft_type_name
                || base_type == ft.name
                || base_type == ft_qualified
                || resolved_type_name == ft_type_name
                || resolved_base_type == ft.name
                || resolved_base_type == ft_qualified;

            // Also check namespace matches if both have namespaces
            // Use resolved namespace for aliased imports (e.g., EffectDateTime -> DateTime)
            let namespace_matches = match (namespace_part, ft_namespace) {
                (Some(ns), Some(ft_ns)) => {
                    // Match either original namespace or resolved (aliased) namespace
                    let resolved_ns = resolved_namespace.as_deref().unwrap_or(ns);
                    ns == ft_ns || resolved_ns == ft_ns
                }
                (None, None) => true,
                // If config has namespace but type doesn't, it's not a match
                (None, Some(_)) => false,
                // If type has namespace but config doesn't, require exact base_type match
                (Some(_), None) => base_type == ft.name || resolved_base_type == ft.name,
            };

            if name_matches && namespace_matches {
                // Now validate import source
                if let Some(actual_source) = import_sources.get(import_name) {
                    // Check if the actual import source matches any configured source
                    let source_matches = ft.from.iter().any(|configured_source| {
                        actual_source == configured_source
                            || actual_source.ends_with(configured_source)
                            || configured_source.ends_with(actual_source)
                    });

                    if source_matches {
                        // Register required namespace imports for this foreign type
                        register_foreign_type_namespaces(ft, actual_source);
                        return ForeignTypeMatch::matched(ft);
                    }
                    // Type is imported from a different source - don't match
                    // We can't know if that library's type has the right methods
                    // Let it fall through to generic handling; TypeScript will catch issues
                } else {
                    // No import found for this type - likely a local type or re-exported
                    // Don't match foreign type config for types we can't verify the source of
                }
            } else if (type_name == ft_type_name || resolved_type_name == ft_type_name)
                && !name_matches
            {
                // Type name matches but qualified form doesn't - helpful hint
                let warning = format!(
                    "Type '{}' has the same name as foreign type '{}' but uses different qualification. \
                     Expected '{}' or configure with namespace: '{}'.",
                    base_type,
                    ft.name,
                    ft_qualified,
                    namespace_part.unwrap_or(type_name)
                );
                if near_match.is_none() {
                    near_match = Some((ft, warning));
                }
            }

            // Check aliases for this foreign type
            for alias in &ft.aliases {
                // Parse alias name for potential namespace
                let (alias_namespace, alias_type_name) =
                    if let Some(dot_idx) = alias.name.rfind('.') {
                        (Some(&alias.name[..dot_idx]), &alias.name[dot_idx + 1..])
                    } else {
                        (None, alias.name.as_str())
                    };

                // Check if alias name matches the type
                let alias_name_matches = type_name == alias_type_name || base_type == alias.name;

                // Check namespace matches for alias
                let alias_namespace_matches = match (namespace_part, alias_namespace) {
                    (Some(ns), Some(alias_ns)) => ns == alias_ns,
                    (None, None) => true,
                    (None, Some(_)) => false,
                    (Some(_), None) => base_type == alias.name,
                };

                if alias_name_matches && alias_namespace_matches {
                    // Validate import source against alias's from
                    let import_name = namespace_part.unwrap_or(type_name);

                    if let Some(actual_source) = import_sources.get(import_name) {
                        // Check if import source matches the alias's from
                        if actual_source == &alias.from
                            || actual_source.ends_with(&alias.from)
                            || alias.from.ends_with(actual_source)
                        {
                            // Register required namespace imports for this foreign type
                            register_foreign_type_namespaces(ft, actual_source);
                            return ForeignTypeMatch::matched(ft);
                        }
                    }
                }
            }
        }

        if let Some((ft, warning)) = near_match {
            ForeignTypeMatch::near_match(ft, warning)
        } else {
            ForeignTypeMatch::none()
        }
    }
}

/// Rewrite namespace references in an expression to use the generated aliases.
///
/// For namespaces that need to be imported (registered via `register_required_namespace`),
/// this function replaces the namespace identifier with its alias.
///
/// For example, if `DateTime` is registered with alias `__mf_DateTime`, then:
/// - `(v) => DateTime.formatIso(v)` becomes `(v) => __mf_DateTime.formatIso(v)`
/// - `DateTime.unsafeNow()` becomes `__mf_DateTime.unsafeNow()`
///
/// # Arguments
/// * `expr` - The expression string to rewrite
///
/// # Returns
/// The rewritten expression string with namespace aliases applied
pub fn rewrite_expression_namespaces(expr: &str) -> String {
    let required_imports = get_required_namespace_imports();

    if required_imports.is_empty() {
        return expr.to_string();
    }

    let mut result = expr.to_string();

    for (namespace, (_module, alias)) in &required_imports {
        // Replace namespace references in member expressions
        // We need to be careful to only replace the namespace when it's followed by a dot
        // to avoid replacing unrelated identifiers
        //
        // Pattern: namespace. -> alias.
        let pattern = format!("{}.", namespace);
        let replacement = format!("{}.", alias);
        result = result.replace(&pattern, &replacement);
    }

    result
}

/// Register required namespace imports for a matched foreign type.
///
/// Checks each namespace referenced in the foreign type's expressions and determines
/// if it needs to be imported (i.e., if it's not already available as a value import).
///
/// The import source is determined by looking at the config file's imports first
/// (e.g., if the config has `import { DateTime } from "effect"`, we use "effect").
/// This ensures we import from the same place the config uses for its expressions.
///
/// # Arguments
/// * `ft` - The matched foreign type configuration
/// * `import_module` - The module the type is imported from (fallback if not in config)
fn register_foreign_type_namespaces(ft: &ForeignTypeConfig, import_module: &str) {
    let type_only_imports = get_type_only_imports();
    let import_sources = get_import_sources();
    let config_imports = get_config_imports();

    for ns in &ft.expression_namespaces {
        // Check if this namespace is imported in the source file
        let has_import = import_sources.contains_key(ns);

        // If the namespace is not imported at all, assume it's either:
        // - A global (like JSON, Math, Date, console)
        // - A local variable defined in the expression
        // In either case, we don't need to generate an import for it
        if !has_import {
            continue;
        }

        // Check if the import is type-only (won't be available at runtime)
        let is_type_only = type_only_imports.get(ns).copied().unwrap_or(false);

        // Only need to generate an import if:
        // 1. The namespace IS imported (it's a known external dependency), AND
        // 2. The namespace is imported as type-only (won't be available at runtime)
        if is_type_only {
            // Determine the module to import from
            // Priority:
            // 1. Config file imports (where the config actually imports from)
            // 2. First configured source in foreign type `from` array
            // 3. Fall back to where the user imported from
            let module = if let Some(config_source) = config_imports.get(ns) {
                // Use the module source from the config file
                // This is the correct source because the config uses this import
                // for its expressions (e.g., `import { DateTime } from "effect"`)
                config_source.clone()
            } else if !ft.from.is_empty() {
                // Fall back to the first configured source
                ft.from[0].clone()
            } else {
                import_module.to_string()
            };

            let alias = format!("__mf_{}", ns);
            register_required_namespace(ns, &module, &alias);
        }
    }
}

/// Result of matching a type against foreign type configurations.
#[derive(Debug)]
pub struct ForeignTypeMatch<'a> {
    /// The matched foreign type config, if any.
    pub config: Option<&'a ForeignTypeConfig>,
    /// Warning message for informational hints.
    pub warning: Option<String>,
    /// Error message for import source mismatches (should fail the build).
    pub error: Option<String>,
}

impl<'a> ForeignTypeMatch<'a> {
    /// Create a successful match.
    pub fn matched(config: &'a ForeignTypeConfig) -> Self {
        Self {
            config: Some(config),
            warning: None,
            error: None,
        }
    }

    /// Create an import mismatch error (type matches but import source doesn't).
    /// This should cause a build failure.
    pub fn import_mismatch(_config: &'a ForeignTypeConfig, error: String) -> Self {
        Self {
            config: None,
            warning: None,
            error: Some(error),
        }
    }

    /// Create a near-match (no match, but with a helpful warning).
    /// The config parameter is for API consistency but not stored since this is a non-match.
    pub fn near_match(_config: &'a ForeignTypeConfig, warning: String) -> Self {
        Self {
            config: None,
            warning: Some(warning),
            error: None,
        }
    }

    /// Create an empty result (no match, no warning, no error).
    pub fn none() -> Self {
        Self {
            config: None,
            warning: None,
            error: None,
        }
    }

    /// Returns true if there was a successful match.
    pub fn is_match(&self) -> bool {
        self.config.is_some()
    }

    /// Returns true if there was an error (import source mismatch).
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
}

// ============================================================================
// Validator types for field validation
// ============================================================================

/// A single validator with optional custom message
#[derive(Debug, Clone)]
pub struct ValidatorSpec {
    pub validator: Validator,
    pub custom_message: Option<String>,
}

/// All supported validators for field validation during deserialization
#[derive(Debug, Clone, PartialEq)]
pub enum Validator {
    // String validators
    Email,
    Url,
    Uuid,
    MaxLength(usize),
    MinLength(usize),
    Length(usize),
    LengthRange(usize, usize),
    Pattern(String),
    NonEmpty,
    Trimmed,
    Lowercase,
    Uppercase,
    Capitalized,
    Uncapitalized,
    StartsWith(String),
    EndsWith(String),
    Includes(String),

    // Number validators
    GreaterThan(f64),
    GreaterThanOrEqualTo(f64),
    LessThan(f64),
    LessThanOrEqualTo(f64),
    Between(f64, f64),
    Int,
    NonNaN,
    Finite,
    Positive,
    NonNegative,
    Negative,
    NonPositive,
    MultipleOf(f64),
    Uint8,

    // Array validators
    MaxItems(usize),
    MinItems(usize),
    ItemsCount(usize),

    // Date validators
    ValidDate,
    GreaterThanDate(String),
    GreaterThanOrEqualToDate(String),
    LessThanDate(String),
    LessThanOrEqualToDate(String),
    BetweenDate(String, String),

    // BigInt validators
    GreaterThanBigInt(String),
    GreaterThanOrEqualToBigInt(String),
    LessThanBigInt(String),
    LessThanOrEqualToBigInt(String),
    BetweenBigInt(String, String),
    PositiveBigInt,
    NonNegativeBigInt,
    NegativeBigInt,
    NonPositiveBigInt,

    // Custom validator
    Custom(String),
}

// ============================================================================
// Validator parsing errors
// ============================================================================

/// Error information from parsing a validator string
#[derive(Debug, Clone)]
pub struct ValidatorParseError {
    pub message: String,
    pub help: Option<String>,
}

impl ValidatorParseError {
    /// Create error for an unknown validator name
    pub fn unknown_validator(name: &str) -> Self {
        let similar = find_similar_validator(name);
        Self {
            message: format!("unknown validator '{}'", name),
            help: similar.map(|s| format!("did you mean '{}'?", s)),
        }
    }

    /// Create error for invalid arguments
    pub fn invalid_args(name: &str, reason: &str) -> Self {
        Self {
            message: format!("invalid arguments for '{}': {}", name, reason),
            help: None,
        }
    }
}

/// List of all known validator names for typo detection
const KNOWN_VALIDATORS: &[&str] = &[
    "email",
    "url",
    "uuid",
    "maxLength",
    "minLength",
    "length",
    "pattern",
    "nonEmpty",
    "trimmed",
    "lowercase",
    "uppercase",
    "capitalized",
    "uncapitalized",
    "startsWith",
    "endsWith",
    "includes",
    "greaterThan",
    "greaterThanOrEqualTo",
    "lessThan",
    "lessThanOrEqualTo",
    "between",
    "int",
    "nonNaN",
    "finite",
    "positive",
    "nonNegative",
    "negative",
    "nonPositive",
    "multipleOf",
    "uint8",
    "maxItems",
    "minItems",
    "itemsCount",
    "validDate",
    "greaterThanDate",
    "greaterThanOrEqualToDate",
    "lessThanDate",
    "lessThanOrEqualToDate",
    "betweenDate",
    "positiveBigInt",
    "nonNegativeBigInt",
    "negativeBigInt",
    "nonPositiveBigInt",
    "greaterThanBigInt",
    "greaterThanOrEqualToBigInt",
    "lessThanBigInt",
    "lessThanOrEqualToBigInt",
    "betweenBigInt",
    "custom",
];

/// Find a similar validator name for typo suggestions using Levenshtein distance
fn find_similar_validator(name: &str) -> Option<&'static str> {
    let name_lower = name.to_lowercase();
    KNOWN_VALIDATORS
        .iter()
        .filter_map(|v| {
            let dist = levenshtein_distance(&v.to_lowercase(), &name_lower);
            if dist <= 2 { Some((*v, dist)) } else { None }
        })
        .min_by_key(|(_, dist)| *dist)
        .map(|(v, _)| v)
}

/// Calculate Levenshtein distance between two strings
fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let len_a = a_chars.len();
    let len_b = b_chars.len();

    if len_a == 0 {
        return len_b;
    }
    if len_b == 0 {
        return len_a;
    }

    let mut prev_row: Vec<usize> = (0..=len_b).collect();
    let mut curr_row: Vec<usize> = vec![0; len_b + 1];

    for i in 1..=len_a {
        curr_row[0] = i;
        for j in 1..=len_b {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };
            curr_row[j] = (prev_row[j] + 1)
                .min(curr_row[j - 1] + 1)
                .min(prev_row[j - 1] + cost);
        }
        std::mem::swap(&mut prev_row, &mut curr_row);
    }
    prev_row[len_b]
}

// ============================================================================
// Helper functions (adapted from derive_debug.rs)
// ============================================================================

pub fn has_flag(args: &str, flag: &str) -> bool {
    if flag_explicit_false(args, flag) {
        return false;
    }

    args.split(|c: char| !c.is_alphanumeric() && c != '_')
        .any(|token| token.eq_ignore_ascii_case(flag))
}

fn flag_explicit_false(args: &str, flag: &str) -> bool {
    let lower = args.to_ascii_lowercase();
    let condensed: String = lower.chars().filter(|c| !c.is_whitespace()).collect();
    condensed.contains(&format!("{flag}:false")) || condensed.contains(&format!("{flag}=false"))
}

pub fn extract_named_string(args: &str, name: &str) -> Option<String> {
    let lower = args.to_ascii_lowercase();
    let name_lower = name.to_ascii_lowercase();

    // Find all occurrences and check for whole-word match
    let mut search_start = 0;
    while let Some(relative_idx) = lower[search_start..].find(&name_lower) {
        let idx = search_start + relative_idx;

        // Check that we're at a word boundary (not part of a larger identifier)
        // The character before must be non-alphanumeric (or we're at the start)
        let at_word_start = idx == 0 || {
            let prev_char = lower.chars().nth(idx - 1).unwrap_or(' ');
            !prev_char.is_alphanumeric() && prev_char != '_'
        };

        if at_word_start {
            let remainder = &args[idx + name.len()..];
            let remainder = remainder.trim_start();

            if remainder.starts_with(':') || remainder.starts_with('=') {
                let value = remainder[1..].trim_start();
                return parse_string_literal(value);
            }

            if remainder.starts_with('(')
                && let Some(close) = remainder.rfind(')')
            {
                let inner = remainder[1..close].trim();
                return parse_string_literal(inner);
            }
        }

        // Continue searching from after this match
        search_start = idx + 1;
    }

    None
}

fn parse_string_literal(input: &str) -> Option<String> {
    let trimmed = input.trim();
    let mut chars = trimmed.chars();
    let quote = chars.next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }

    let mut escaped = false;
    let mut buf = String::new();
    for c in chars {
        if escaped {
            buf.push(c);
            escaped = false;
            continue;
        }
        if c == '\\' {
            escaped = true;
            continue;
        }
        if c == quote {
            return Some(buf);
        }
        buf.push(c);
    }
    None
}

/// Find the position of a comma at the top level (not inside <> brackets)
fn find_top_level_comma(s: &str) -> Option<usize> {
    let mut depth = 0;
    for (i, c) in s.char_indices() {
        match c {
            '<' => depth += 1,
            '>' => depth -= 1,
            ',' if depth == 0 => return Some(i),
            _ => {}
        }
    }
    None
}

// ============================================================================
// Validator parsing functions
// ============================================================================

/// Known options that are NOT validators (to avoid false positives)
const KNOWN_OPTIONS: &[&str] = &[
    "skip",
    "skipSerializing",
    "skipDeserializing",
    "flatten",
    "default",
    "rename",
    "validate",
    "message",
    "serializeWith",
    "deserializeWith",
];

/// Extract validators from decorator arguments with diagnostic collection
/// Supports:
/// - Explicit array: validate: ["email", "maxLength(255)"]
/// - Object array: validate: [{ validate: "email", message: "..." }]
/// - Shorthand: @serde(email) or @serde(minLength(2), maxLength(50))
pub fn extract_validators(
    args: &str,
    decorator_span: SpanIR,
    field_name: &str,
    diagnostics: &mut DiagnosticCollector,
) -> Vec<ValidatorSpec> {
    let mut validators = Vec::new();

    // First, check for explicit validate: [...] format
    let lower = args.to_ascii_lowercase();
    if let Some(idx) = lower.find("validate") {
        let remainder = &args[idx + 8..].trim_start();
        if remainder.starts_with(':') || remainder.starts_with('=') {
            let value_start = &remainder[1..].trim_start();
            if value_start.starts_with('[') {
                return parse_validator_array(value_start, decorator_span, field_name, diagnostics);
            } else {
                diagnostics.error(
                    decorator_span,
                    format!(
                        "field '{}': validate must be an array, e.g., validate: [\"email\"]",
                        field_name
                    ),
                );
                return validators;
            }
        }
    }

    // Parse shorthand validators: @serde(email) or @serde(minLength(2), maxLength(50))
    for item in split_decorator_args(args) {
        let item = item.trim();
        if item.is_empty() {
            continue;
        }

        // Skip known options
        let item_lower = item.to_ascii_lowercase();
        let base_name = item_lower.split('(').next().unwrap_or(&item_lower);
        let base_name = base_name.split(':').next().unwrap_or(base_name).trim();

        if KNOWN_OPTIONS.contains(&base_name) {
            continue;
        }

        // Check if this looks like a validator (either a known name or has function syntax)
        let is_likely_validator = KNOWN_VALIDATORS.contains(&base_name) || item.contains('('); // Function-like syntax suggests validator

        if is_likely_validator {
            match parse_validator_string(item) {
                Ok(v) => validators.push(ValidatorSpec {
                    validator: v,
                    custom_message: None,
                }),
                Err(err) => {
                    if let Some(help) = err.help {
                        diagnostics.error_with_help(
                            decorator_span,
                            format!("field '{}': {}", field_name, err.message),
                            help,
                        );
                    } else {
                        diagnostics.error(
                            decorator_span,
                            format!("field '{}': {}", field_name, err.message),
                        );
                    }
                }
            }
        }
    }

    validators
}

/// Split decorator arguments by commas, respecting nested parentheses and strings
fn split_decorator_args(input: &str) -> Vec<String> {
    let mut items = Vec::new();
    let mut current = String::new();
    let mut depth = 0;
    let mut in_string = false;
    let mut string_char = '"';

    for c in input.chars() {
        if in_string {
            current.push(c);
            if c == string_char {
                in_string = false;
            }
            continue;
        }

        match c {
            '"' | '\'' => {
                in_string = true;
                string_char = c;
                current.push(c);
            }
            '(' | '[' | '{' => {
                depth += 1;
                current.push(c);
            }
            ')' | ']' | '}' => {
                depth -= 1;
                current.push(c);
            }
            ',' if depth == 0 => {
                let trimmed = current.trim().to_string();
                if !trimmed.is_empty() {
                    items.push(trimmed);
                }
                current.clear();
            }
            _ => current.push(c),
        }
    }

    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() {
        items.push(trimmed);
    }

    items
}

/// Parse array content: ["email", "maxLength(255)", { validate: "...", message: "..." }]
fn parse_validator_array(
    input: &str,
    decorator_span: SpanIR,
    field_name: &str,
    diagnostics: &mut DiagnosticCollector,
) -> Vec<ValidatorSpec> {
    let mut validators = Vec::new();

    // Find matching ] bracket
    let Some(content) = extract_bracket_content(input, '[', ']') else {
        diagnostics.error(
            decorator_span,
            format!("field '{}': malformed validator array", field_name),
        );
        return validators;
    };

    // Split by commas (respecting nested structures)
    for item in split_array_items(&content) {
        let item = item.trim();
        if item.starts_with('{') {
            // Object form: { validate: "...", message: "..." }
            match parse_validator_object(item) {
                Ok(spec) => validators.push(spec),
                Err(err) => {
                    if let Some(help) = err.help {
                        diagnostics.error_with_help(
                            decorator_span,
                            format!("field '{}': {}", field_name, err.message),
                            help,
                        );
                    } else {
                        diagnostics.error(
                            decorator_span,
                            format!("field '{}': {}", field_name, err.message),
                        );
                    }
                }
            }
        } else if item.starts_with('"') || item.starts_with('\'') {
            // String form: "email" or "maxLength(255)"
            if let Some(s) = parse_string_literal(item) {
                match parse_validator_string(&s) {
                    Ok(v) => validators.push(ValidatorSpec {
                        validator: v,
                        custom_message: None,
                    }),
                    Err(err) => {
                        if let Some(help) = err.help {
                            diagnostics.error_with_help(
                                decorator_span,
                                format!("field '{}': {}", field_name, err.message),
                                help,
                            );
                        } else {
                            diagnostics.error(
                                decorator_span,
                                format!("field '{}': {}", field_name, err.message),
                            );
                        }
                    }
                }
            }
        }
    }
    validators
}

/// Extract content between matching brackets
fn extract_bracket_content(input: &str, open: char, close: char) -> Option<String> {
    let mut depth = 0;
    let mut start = None;

    for (i, c) in input.char_indices() {
        if c == open {
            if depth == 0 {
                start = Some(i + 1);
            }
            depth += 1;
        } else if c == close {
            depth -= 1;
            if depth == 0
                && let Some(s) = start
            {
                return Some(input[s..i].to_string());
            }
        }
    }
    None
}

/// Split array items by commas, respecting nested brackets and strings
fn split_array_items(input: &str) -> Vec<String> {
    let mut items = Vec::new();
    let mut current = String::new();
    let mut depth = 0;
    let mut in_string = false;
    let mut string_char = '"';

    for c in input.chars() {
        if in_string {
            current.push(c);
            if c == string_char {
                in_string = false;
            }
            continue;
        }

        match c {
            '"' | '\'' => {
                in_string = true;
                string_char = c;
                current.push(c);
            }
            '[' | '{' | '(' => {
                depth += 1;
                current.push(c);
            }
            ']' | '}' | ')' => {
                depth -= 1;
                current.push(c);
            }
            ',' if depth == 0 => {
                let trimmed = current.trim().to_string();
                if !trimmed.is_empty() {
                    items.push(trimmed);
                }
                current.clear();
            }
            _ => current.push(c),
        }
    }

    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() {
        items.push(trimmed);
    }

    items
}

/// Parse object form: { validate: "email", message: "Invalid email" }
fn parse_validator_object(input: &str) -> Result<ValidatorSpec, ValidatorParseError> {
    let content = extract_bracket_content(input, '{', '}')
        .ok_or_else(|| ValidatorParseError::invalid_args("object", "malformed validator object"))?;

    let validator_str = extract_named_string(&content, "validate")
        .ok_or_else(|| ValidatorParseError::invalid_args("object", "missing 'validate' field"))?;
    let validator = parse_validator_string(&validator_str)?;
    let custom_message = extract_named_string(&content, "message");

    Ok(ValidatorSpec {
        validator,
        custom_message,
    })
}

/// Parse a validator string like "email", "maxLength(255)", "custom(myValidator)"
fn parse_validator_string(s: &str) -> Result<Validator, ValidatorParseError> {
    let trimmed = s.trim();

    // Check for function-call style: name(args)
    if let Some(paren_idx) = trimmed.find('(') {
        let name = &trimmed[..paren_idx];
        let Some(args_end) = trimmed.rfind(')') else {
            return Err(ValidatorParseError::invalid_args(
                name,
                "missing closing parenthesis",
            ));
        };
        let args = &trimmed[paren_idx + 1..args_end];
        return parse_validator_with_args(name, args);
    }

    // Simple validators without args
    match trimmed.to_lowercase().as_str() {
        "email" => Ok(Validator::Email),
        "url" => Ok(Validator::Url),
        "uuid" => Ok(Validator::Uuid),
        "nonempty" | "nonemptystring" => Ok(Validator::NonEmpty),
        "trimmed" => Ok(Validator::Trimmed),
        "lowercase" | "lowercased" => Ok(Validator::Lowercase),
        "uppercase" | "uppercased" => Ok(Validator::Uppercase),
        "capitalized" => Ok(Validator::Capitalized),
        "uncapitalized" => Ok(Validator::Uncapitalized),
        "int" => Ok(Validator::Int),
        "nonnan" => Ok(Validator::NonNaN),
        "finite" => Ok(Validator::Finite),
        "positive" => Ok(Validator::Positive),
        "nonnegative" => Ok(Validator::NonNegative),
        "negative" => Ok(Validator::Negative),
        "nonpositive" => Ok(Validator::NonPositive),
        "uint8" => Ok(Validator::Uint8),
        "validdate" | "validdatefromself" => Ok(Validator::ValidDate),
        "positivebigint" | "positivebigintfromself" => Ok(Validator::PositiveBigInt),
        "nonnegativebigint" | "nonnegativebigintfromself" => Ok(Validator::NonNegativeBigInt),
        "negativebigint" | "negativebigintfromself" => Ok(Validator::NegativeBigInt),
        "nonpositivebigint" | "nonpositivebigintfromself" => Ok(Validator::NonPositiveBigInt),
        "nonnegativeint" => Ok(Validator::Int), // Int + NonNegative combined
        _ => Err(ValidatorParseError::unknown_validator(trimmed)),
    }
}

/// Parse validators with arguments
fn parse_validator_with_args(name: &str, args: &str) -> Result<Validator, ValidatorParseError> {
    let name_lower = name.to_lowercase();
    match name_lower.as_str() {
        "maxlength" => args
            .trim()
            .parse()
            .map(Validator::MaxLength)
            .map_err(|_| ValidatorParseError::invalid_args(name, "expected a positive integer")),
        "minlength" => args
            .trim()
            .parse()
            .map(Validator::MinLength)
            .map_err(|_| ValidatorParseError::invalid_args(name, "expected a positive integer")),
        "length" => {
            let parts: Vec<&str> = args.split(',').collect();
            match parts.len() {
                1 => parts[0].trim().parse().map(Validator::Length).map_err(|_| {
                    ValidatorParseError::invalid_args(name, "expected a positive integer")
                }),
                2 => {
                    let min = parts[0].trim().parse().map_err(|_| {
                        ValidatorParseError::invalid_args(name, "expected two positive integers")
                    })?;
                    let max = parts[1].trim().parse().map_err(|_| {
                        ValidatorParseError::invalid_args(name, "expected two positive integers")
                    })?;
                    Ok(Validator::LengthRange(min, max))
                }
                _ => Err(ValidatorParseError::invalid_args(
                    name,
                    "expected 1 or 2 arguments",
                )),
            }
        }
        "pattern" => parse_validator_string_arg(args)
            .map(Validator::Pattern)
            .ok_or_else(|| ValidatorParseError::invalid_args(name, "expected a string pattern")),
        "startswith" => parse_validator_string_arg(args)
            .map(Validator::StartsWith)
            .ok_or_else(|| ValidatorParseError::invalid_args(name, "expected a string")),
        "endswith" => parse_validator_string_arg(args)
            .map(Validator::EndsWith)
            .ok_or_else(|| ValidatorParseError::invalid_args(name, "expected a string")),
        "includes" => parse_validator_string_arg(args)
            .map(Validator::Includes)
            .ok_or_else(|| ValidatorParseError::invalid_args(name, "expected a string")),
        "greaterthan" => args
            .trim()
            .parse()
            .map(Validator::GreaterThan)
            .map_err(|_| ValidatorParseError::invalid_args(name, "expected a number")),
        "greaterthanorequalto" => args
            .trim()
            .parse()
            .map(Validator::GreaterThanOrEqualTo)
            .map_err(|_| ValidatorParseError::invalid_args(name, "expected a number")),
        "lessthan" => args
            .trim()
            .parse()
            .map(Validator::LessThan)
            .map_err(|_| ValidatorParseError::invalid_args(name, "expected a number")),
        "lessthanorequalto" => args
            .trim()
            .parse()
            .map(Validator::LessThanOrEqualTo)
            .map_err(|_| ValidatorParseError::invalid_args(name, "expected a number")),
        "between" => {
            let parts: Vec<&str> = args.split(',').collect();
            if parts.len() == 2 {
                let min = parts[0]
                    .trim()
                    .parse()
                    .map_err(|_| ValidatorParseError::invalid_args(name, "expected two numbers"))?;
                let max = parts[1]
                    .trim()
                    .parse()
                    .map_err(|_| ValidatorParseError::invalid_args(name, "expected two numbers"))?;
                Ok(Validator::Between(min, max))
            } else {
                Err(ValidatorParseError::invalid_args(
                    name,
                    "expected two numbers separated by comma",
                ))
            }
        }
        "multipleof" => args
            .trim()
            .parse()
            .map(Validator::MultipleOf)
            .map_err(|_| ValidatorParseError::invalid_args(name, "expected a number")),
        "maxitems" => args
            .trim()
            .parse()
            .map(Validator::MaxItems)
            .map_err(|_| ValidatorParseError::invalid_args(name, "expected a positive integer")),
        "minitems" => args
            .trim()
            .parse()
            .map(Validator::MinItems)
            .map_err(|_| ValidatorParseError::invalid_args(name, "expected a positive integer")),
        "itemscount" => args
            .trim()
            .parse()
            .map(Validator::ItemsCount)
            .map_err(|_| ValidatorParseError::invalid_args(name, "expected a positive integer")),
        "greaterthandate" => parse_validator_string_arg(args)
            .map(Validator::GreaterThanDate)
            .ok_or_else(|| ValidatorParseError::invalid_args(name, "expected a date string")),
        "greaterthanorequaltodate" => parse_validator_string_arg(args)
            .map(Validator::GreaterThanOrEqualToDate)
            .ok_or_else(|| ValidatorParseError::invalid_args(name, "expected a date string")),
        "lessthandate" => parse_validator_string_arg(args)
            .map(Validator::LessThanDate)
            .ok_or_else(|| ValidatorParseError::invalid_args(name, "expected a date string")),
        "lessthanorequaltodate" => parse_validator_string_arg(args)
            .map(Validator::LessThanOrEqualToDate)
            .ok_or_else(|| ValidatorParseError::invalid_args(name, "expected a date string")),
        "betweendate" => {
            let parts: Vec<&str> = args.splitn(2, ',').collect();
            if parts.len() == 2 {
                let min = parse_validator_string_arg(parts[0].trim()).ok_or_else(|| {
                    ValidatorParseError::invalid_args(name, "expected two date strings")
                })?;
                let max = parse_validator_string_arg(parts[1].trim()).ok_or_else(|| {
                    ValidatorParseError::invalid_args(name, "expected two date strings")
                })?;
                Ok(Validator::BetweenDate(min, max))
            } else {
                Err(ValidatorParseError::invalid_args(
                    name,
                    "expected two date strings separated by comma",
                ))
            }
        }
        "greaterthanbigint" => Ok(Validator::GreaterThanBigInt(args.trim().to_string())),
        "greaterthanorequaltobigint" => Ok(Validator::GreaterThanOrEqualToBigInt(
            args.trim().to_string(),
        )),
        "lessthanbigint" => Ok(Validator::LessThanBigInt(args.trim().to_string())),
        "lessthanorequaltobigint" => {
            Ok(Validator::LessThanOrEqualToBigInt(args.trim().to_string()))
        }
        "betweenbigint" => {
            let parts: Vec<&str> = args.splitn(2, ',').collect();
            if parts.len() == 2 {
                Ok(Validator::BetweenBigInt(
                    parts[0].trim().to_string(),
                    parts[1].trim().to_string(),
                ))
            } else {
                Err(ValidatorParseError::invalid_args(
                    name,
                    "expected two bigint values separated by comma",
                ))
            }
        }
        "custom" => {
            // custom(myValidator) - extract function name (can be quoted or unquoted)
            let fn_name =
                parse_validator_string_arg(args).unwrap_or_else(|| args.trim().to_string());
            Ok(Validator::Custom(fn_name))
        }
        _ => Err(ValidatorParseError::unknown_validator(name)),
    }
}

/// Parse a string argument (handles both quoted and unquoted)
fn parse_validator_string_arg(input: &str) -> Option<String> {
    let trimmed = input.trim();
    // Try to parse as quoted string first
    if let Some(s) = parse_string_literal(trimmed) {
        return Some(s);
    }
    // Otherwise return as-is if not empty
    if !trimmed.is_empty() {
        return Some(trimmed.to_string());
    }
    None
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ts_syn::abi::SpanIR;

    fn span() -> SpanIR {
        SpanIR::new(0, 0)
    }

    fn make_decorator(args: &str) -> DecoratorIR {
        DecoratorIR {
            name: "serde".into(),
            args_src: args.into(),
            span: span(),
            node: None,
        }
    }

    #[test]
    fn test_field_skip() {
        let decorator = make_decorator("skip");
        let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
        let opts = result.options;
        assert!(opts.skip);
        assert!(!opts.should_serialize());
        assert!(!opts.should_deserialize());
    }

    #[test]
    fn test_field_skip_serializing() {
        let decorator = make_decorator("skipSerializing");
        let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
        let opts = result.options;
        assert!(opts.skip_serializing);
        assert!(!opts.should_serialize());
        assert!(opts.should_deserialize());
    }

    #[test]
    fn test_field_rename() {
        let decorator = make_decorator(r#"{ rename: "user_id" }"#);
        let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
        let opts = result.options;
        assert_eq!(opts.rename.as_deref(), Some("user_id"));
    }

    #[test]
    fn test_field_default_flag() {
        let decorator = make_decorator("default");
        let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
        let opts = result.options;
        assert!(opts.default);
        assert!(opts.default_expr.is_none());
    }

    #[test]
    fn test_field_default_expr() {
        let decorator = make_decorator(r#"{ default: "new Date()" }"#);
        let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
        let opts = result.options;
        assert!(opts.default);
        assert_eq!(opts.default_expr.as_deref(), Some("new Date()"));
    }

    #[test]
    fn test_field_flatten() {
        let decorator = make_decorator("flatten");
        let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
        let opts = result.options;
        assert!(opts.flatten);
    }

    #[test]
    fn test_container_rename_all() {
        let decorator = make_decorator(r#"{ renameAll: "camelCase" }"#);
        let opts = SerdeContainerOptions::from_decorators(&[decorator]);
        assert_eq!(opts.rename_all, RenameAll::CamelCase);
    }

    #[test]
    fn test_container_deny_unknown_fields() {
        let decorator = make_decorator("denyUnknownFields");
        let opts = SerdeContainerOptions::from_decorators(&[decorator]);
        assert!(opts.deny_unknown_fields);
    }

    #[test]
    fn test_type_category_primitives() {
        assert_eq!(
            TypeCategory::from_ts_type("string"),
            TypeCategory::Primitive
        );
        assert_eq!(
            TypeCategory::from_ts_type("number"),
            TypeCategory::Primitive
        );
        assert_eq!(
            TypeCategory::from_ts_type("boolean"),
            TypeCategory::Primitive
        );
    }

    #[test]
    fn test_type_category_date() {
        assert_eq!(TypeCategory::from_ts_type("Date"), TypeCategory::Date);
    }

    #[test]
    fn test_type_category_array() {
        assert_eq!(
            TypeCategory::from_ts_type("string[]"),
            TypeCategory::Array("string".into())
        );
        assert_eq!(
            TypeCategory::from_ts_type("Array<number>"),
            TypeCategory::Array("number".into())
        );
    }

    #[test]
    fn test_type_category_map() {
        assert_eq!(
            TypeCategory::from_ts_type("Map<string, number>"),
            TypeCategory::Map("string".into(), "number".into())
        );
    }

    #[test]
    fn test_type_category_set() {
        assert_eq!(
            TypeCategory::from_ts_type("Set<string>"),
            TypeCategory::Set("string".into())
        );
    }

    #[test]
    fn test_type_category_optional() {
        assert_eq!(
            TypeCategory::from_ts_type("string | undefined"),
            TypeCategory::Optional("string".into())
        );
    }

    #[test]
    fn test_type_category_nullable() {
        assert_eq!(
            TypeCategory::from_ts_type("string | null"),
            TypeCategory::Nullable("string".into())
        );
    }

    #[test]
    fn test_type_category_serializable() {
        assert_eq!(
            TypeCategory::from_ts_type("User"),
            TypeCategory::Serializable("User".into())
        );
    }

    #[test]
    fn test_rename_all_camel_case() {
        assert_eq!(RenameAll::CamelCase.apply("user_name"), "userName");
        assert_eq!(RenameAll::CamelCase.apply("created_at"), "createdAt");
    }

    #[test]
    fn test_rename_all_snake_case() {
        assert_eq!(RenameAll::SnakeCase.apply("userName"), "user_name");
        assert_eq!(RenameAll::SnakeCase.apply("createdAt"), "created_at");
    }

    #[test]
    fn test_rename_all_pascal_case() {
        assert_eq!(RenameAll::PascalCase.apply("user_name"), "UserName");
    }

    #[test]
    fn test_rename_all_kebab_case() {
        assert_eq!(RenameAll::KebabCase.apply("userName"), "user-name");
    }

    #[test]
    fn test_rename_all_screaming_snake_case() {
        assert_eq!(RenameAll::ScreamingSnakeCase.apply("userName"), "USER_NAME");
    }

    // ========================================================================
    // Validator parsing tests
    // ========================================================================

    #[test]
    fn test_parse_simple_validators() {
        assert!(matches!(
            parse_validator_string("email"),
            Ok(Validator::Email)
        ));
        assert!(matches!(parse_validator_string("url"), Ok(Validator::Url)));
        assert!(matches!(
            parse_validator_string("uuid"),
            Ok(Validator::Uuid)
        ));
        assert!(matches!(
            parse_validator_string("nonEmpty"),
            Ok(Validator::NonEmpty)
        ));
        assert!(matches!(
            parse_validator_string("trimmed"),
            Ok(Validator::Trimmed)
        ));
        assert!(matches!(
            parse_validator_string("lowercase"),
            Ok(Validator::Lowercase)
        ));
        assert!(matches!(
            parse_validator_string("uppercase"),
            Ok(Validator::Uppercase)
        ));
        assert!(matches!(parse_validator_string("int"), Ok(Validator::Int)));
        assert!(matches!(
            parse_validator_string("positive"),
            Ok(Validator::Positive)
        ));
        assert!(matches!(
            parse_validator_string("validDate"),
            Ok(Validator::ValidDate)
        ));
    }

    #[test]
    fn test_parse_validators_with_args() {
        assert!(matches!(
            parse_validator_string("maxLength(255)"),
            Ok(Validator::MaxLength(255))
        ));
        assert!(matches!(
            parse_validator_string("minLength(1)"),
            Ok(Validator::MinLength(1))
        ));
        assert!(matches!(
            parse_validator_string("length(36)"),
            Ok(Validator::Length(36))
        ));
        assert!(matches!(
            parse_validator_string("between(0, 100)"),
            Ok(Validator::Between(min, max)) if min == 0.0 && max == 100.0
        ));
        assert!(matches!(
            parse_validator_string("greaterThan(5)"),
            Ok(Validator::GreaterThan(n)) if n == 5.0
        ));
    }

    #[test]
    fn test_parse_validators_with_string_args() {
        assert!(matches!(
            parse_validator_string(r#"startsWith("https://")"#),
            Ok(Validator::StartsWith(s)) if s == "https://"
        ));
        assert!(matches!(
            parse_validator_string(r#"endsWith(".com")"#),
            Ok(Validator::EndsWith(s)) if s == ".com"
        ));
        assert!(matches!(
            parse_validator_string(r#"includes("@")"#),
            Ok(Validator::Includes(s)) if s == "@"
        ));
    }

    #[test]
    fn test_parse_custom_validator() {
        assert!(matches!(
            parse_validator_string("custom(myValidator)"),
            Ok(Validator::Custom(fn_name)) if fn_name == "myValidator"
        ));
    }

    #[test]
    fn test_extract_validators_from_args() {
        let mut diagnostics = DiagnosticCollector::new();
        let validators = extract_validators(
            r#"{ validate: ["email", "maxLength(255)"] }"#,
            span(),
            "test_field",
            &mut diagnostics,
        );
        assert_eq!(validators.len(), 2);
        assert!(matches!(validators[0].validator, Validator::Email));
        assert!(matches!(validators[1].validator, Validator::MaxLength(255)));
        assert!(!diagnostics.has_errors());
    }

    #[test]
    fn test_extract_validators_with_message() {
        let mut diagnostics = DiagnosticCollector::new();
        let validators = extract_validators(
            r#"{ validate: [{ validate: "email", message: "Invalid email!" }] }"#,
            span(),
            "test_field",
            &mut diagnostics,
        );
        assert_eq!(validators.len(), 1);
        assert!(matches!(validators[0].validator, Validator::Email));
        assert_eq!(
            validators[0].custom_message.as_deref(),
            Some("Invalid email!")
        );
        assert!(!diagnostics.has_errors());
    }

    #[test]
    fn test_extract_validators_mixed() {
        let mut diagnostics = DiagnosticCollector::new();
        let validators = extract_validators(
            r#"{ validate: ["nonEmpty", { validate: "email", message: "Bad email" }] }"#,
            span(),
            "test_field",
            &mut diagnostics,
        );
        assert_eq!(validators.len(), 2);
        assert!(matches!(validators[0].validator, Validator::NonEmpty));
        assert!(validators[0].custom_message.is_none());
        assert!(matches!(validators[1].validator, Validator::Email));
        assert_eq!(validators[1].custom_message.as_deref(), Some("Bad email"));
        assert!(!diagnostics.has_errors());
    }

    #[test]
    fn test_field_with_validators() {
        let decorator = make_decorator(r#"{ validate: ["email", "maxLength(255)"] }"#);
        let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
        let opts = result.options;
        assert_eq!(opts.validators.len(), 2);
        assert!(matches!(opts.validators[0].validator, Validator::Email));
        assert!(matches!(
            opts.validators[1].validator,
            Validator::MaxLength(255)
        ));
    }

    // ========================================================================
    // Date validator parsing tests
    // ========================================================================

    #[test]
    fn test_parse_date_validators() {
        assert!(matches!(
            parse_validator_string("validDate"),
            Ok(Validator::ValidDate)
        ));
        assert!(matches!(
            parse_validator_string(r#"greaterThanDate("2020-01-01")"#),
            Ok(Validator::GreaterThanDate(d)) if d == "2020-01-01"
        ));
        assert!(matches!(
            parse_validator_string(r#"greaterThanOrEqualToDate("2020-01-01")"#),
            Ok(Validator::GreaterThanOrEqualToDate(d)) if d == "2020-01-01"
        ));
        assert!(matches!(
            parse_validator_string(r#"lessThanDate("2030-01-01")"#),
            Ok(Validator::LessThanDate(d)) if d == "2030-01-01"
        ));
        assert!(matches!(
            parse_validator_string(r#"lessThanOrEqualToDate("2030-01-01")"#),
            Ok(Validator::LessThanOrEqualToDate(d)) if d == "2030-01-01"
        ));
        assert!(matches!(
            parse_validator_string(r#"betweenDate("2020-01-01", "2030-12-31")"#),
            Ok(Validator::BetweenDate(min, max)) if min == "2020-01-01" && max == "2030-12-31"
        ));
    }

    // ========================================================================
    // BigInt validator parsing tests
    // ========================================================================

    #[test]
    fn test_parse_bigint_validators() {
        assert!(matches!(
            parse_validator_string("positiveBigInt"),
            Ok(Validator::PositiveBigInt)
        ));
        assert!(matches!(
            parse_validator_string("nonNegativeBigInt"),
            Ok(Validator::NonNegativeBigInt)
        ));
        assert!(matches!(
            parse_validator_string("negativeBigInt"),
            Ok(Validator::NegativeBigInt)
        ));
        assert!(matches!(
            parse_validator_string("nonPositiveBigInt"),
            Ok(Validator::NonPositiveBigInt)
        ));
        assert!(matches!(
            parse_validator_string("greaterThanBigInt(100)"),
            Ok(Validator::GreaterThanBigInt(n)) if n == "100"
        ));
        assert!(matches!(
            parse_validator_string("greaterThanOrEqualToBigInt(0)"),
            Ok(Validator::GreaterThanOrEqualToBigInt(n)) if n == "0"
        ));
        assert!(matches!(
            parse_validator_string("lessThanBigInt(1000)"),
            Ok(Validator::LessThanBigInt(n)) if n == "1000"
        ));
        assert!(matches!(
            parse_validator_string("lessThanOrEqualToBigInt(999)"),
            Ok(Validator::LessThanOrEqualToBigInt(n)) if n == "999"
        ));
        assert!(matches!(
            parse_validator_string("betweenBigInt(0, 100)"),
            Ok(Validator::BetweenBigInt(min, max)) if min == "0" && max == "100"
        ));
    }

    // ========================================================================
    // Array validator parsing tests
    // ========================================================================

    #[test]
    fn test_parse_array_validators() {
        assert!(matches!(
            parse_validator_string("maxItems(10)"),
            Ok(Validator::MaxItems(10))
        ));
        assert!(matches!(
            parse_validator_string("minItems(1)"),
            Ok(Validator::MinItems(1))
        ));
        assert!(matches!(
            parse_validator_string("itemsCount(5)"),
            Ok(Validator::ItemsCount(5))
        ));
    }

    // ========================================================================
    // Additional number validator parsing tests
    // ========================================================================

    #[test]
    fn test_parse_additional_number_validators() {
        assert!(matches!(
            parse_validator_string("nonNaN"),
            Ok(Validator::NonNaN)
        ));
        assert!(matches!(
            parse_validator_string("finite"),
            Ok(Validator::Finite)
        ));
        assert!(matches!(
            parse_validator_string("uint8"),
            Ok(Validator::Uint8)
        ));
        assert!(matches!(
            parse_validator_string("multipleOf(5)"),
            Ok(Validator::MultipleOf(n)) if n == 5.0
        ));
        assert!(matches!(
            parse_validator_string("negative"),
            Ok(Validator::Negative)
        ));
        assert!(matches!(
            parse_validator_string("nonNegative"),
            Ok(Validator::NonNegative)
        ));
        assert!(matches!(
            parse_validator_string("nonPositive"),
            Ok(Validator::NonPositive)
        ));
    }

    // ========================================================================
    // Additional string validator parsing tests
    // ========================================================================

    #[test]
    fn test_parse_additional_string_validators() {
        assert!(matches!(
            parse_validator_string("capitalized"),
            Ok(Validator::Capitalized)
        ));
        assert!(matches!(
            parse_validator_string("uncapitalized"),
            Ok(Validator::Uncapitalized)
        ));
        assert!(matches!(
            parse_validator_string("length(5, 10)"),
            Ok(Validator::LengthRange(5, 10))
        ));
    }

    // ========================================================================
    // Case sensitivity tests
    // ========================================================================

    #[test]
    fn test_parse_validators_case_insensitive() {
        // Validators should be case-insensitive
        assert!(matches!(
            parse_validator_string("EMAIL"),
            Ok(Validator::Email)
        ));
        assert!(matches!(
            parse_validator_string("Email"),
            Ok(Validator::Email)
        ));
        assert!(matches!(
            parse_validator_string("NONEMPTY"),
            Ok(Validator::NonEmpty)
        ));
        assert!(matches!(
            parse_validator_string("NonEmpty"),
            Ok(Validator::NonEmpty)
        ));
        assert!(matches!(
            parse_validator_string("MAXLENGTH(10)"),
            Ok(Validator::MaxLength(10))
        ));
    }

    // ========================================================================
    // Pattern validator with special characters
    // ========================================================================

    #[test]
    fn test_parse_pattern_with_special_chars() {
        // Test pattern with various regex special chars
        assert!(matches!(
            parse_validator_string(r#"pattern("^[A-Z]{3}$")"#),
            Ok(Validator::Pattern(p)) if p == "^[A-Z]{3}$"
        ));
        assert!(matches!(
            parse_validator_string(r#"pattern("\\d+")"#),
            Ok(Validator::Pattern(p)) if p == "\\d+"
        ));
        assert!(matches!(
            parse_validator_string(r#"pattern("^test\\.json$")"#),
            Ok(Validator::Pattern(p)) if p == "^test\\.json$"
        ));
    }

    // ========================================================================
    // Edge case: validators with whitespace
    // ========================================================================

    #[test]
    fn test_parse_validators_with_whitespace() {
        assert!(matches!(
            parse_validator_string("  email  "),
            Ok(Validator::Email)
        ));
        assert!(matches!(
            parse_validator_string("between( 1 , 100 )"),
            Ok(Validator::Between(min, max)) if min == 1.0 && max == 100.0
        ));
        assert!(matches!(
            parse_validator_string("maxLength( 50 )"),
            Ok(Validator::MaxLength(50))
        ));
    }

    // ========================================================================
    // Validator error tests
    // ========================================================================

    #[test]
    fn test_unknown_validator_returns_error() {
        let result = parse_validator_string("unknownValidator");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("unknown validator"));
        assert!(err.message.contains("unknownValidator"));
    }

    #[test]
    fn test_unknown_validator_with_typo_suggests_correction() {
        // "emai" is close to "email"
        let result = parse_validator_string("emai");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.help.is_some());
        assert!(err.help.as_ref().unwrap().contains("email"));
    }

    #[test]
    fn test_unknown_validator_no_suggestion_for_unrelated() {
        // "xyz" has no similar validators
        let result = parse_validator_string("xyz");
        assert!(result.is_err());
        let err = result.unwrap_err();
        // For short strings with no matches, help may be None
        assert!(err.help.is_none() || !err.help.as_ref().unwrap().contains("email"));
    }

    #[test]
    fn test_invalid_maxlength_args_returns_error() {
        let result = parse_validator_string("maxLength(abc)");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("maxLength"));
    }

    #[test]
    fn test_invalid_between_args_returns_error() {
        // between requires two numbers
        let result = parse_validator_string("between(abc, def)");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("between"));
    }

    #[test]
    fn test_extract_validators_collects_errors() {
        let mut diagnostics = DiagnosticCollector::new();
        let validators = extract_validators(
            r#"{ validate: ["unknownValidator", "email"] }"#,
            span(),
            "test_field",
            &mut diagnostics,
        );
        // Should still extract the valid "email" validator
        assert_eq!(validators.len(), 1);
        assert!(matches!(validators[0].validator, Validator::Email));
        // Should have recorded an error for the unknown validator
        assert!(diagnostics.has_errors());
        assert_eq!(diagnostics.len(), 1);
    }

    #[test]
    fn test_extract_validators_multiple_errors() {
        let mut diagnostics = DiagnosticCollector::new();
        let validators = extract_validators(
            r#"{ validate: ["unknown1", "unknown2", "email"] }"#,
            span(),
            "test_field",
            &mut diagnostics,
        );
        // Should still extract the valid "email" validator
        assert_eq!(validators.len(), 1);
        // Should have recorded two errors
        assert!(diagnostics.has_errors());
        assert_eq!(diagnostics.len(), 2);
    }

    #[test]
    fn test_typo_suggestion_url_vs_uuid() {
        // "rul" is close to "url"
        let result = parse_validator_string("rul");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.help.is_some());
        assert!(err.help.as_ref().unwrap().contains("url"));
    }

    #[test]
    fn test_typo_suggestion_maxlength() {
        // "maxLenth" is close to "maxLength"
        let result = parse_validator_string("maxLenth(10)");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.help.is_some());
        assert!(err.help.as_ref().unwrap().contains("maxLength"));
    }

    // ========================================================================
    // Custom serializer/deserializer tests (serializeWith/deserializeWith)
    // ========================================================================

    #[test]
    fn test_field_serialize_with() {
        let decorator = make_decorator(r#"{ serializeWith: "mySerializer" }"#);
        let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
        let opts = result.options;
        assert_eq!(opts.serialize_with.as_deref(), Some("mySerializer"));
        assert!(opts.deserialize_with.is_none());
    }

    #[test]
    fn test_field_deserialize_with() {
        let decorator = make_decorator(r#"{ deserializeWith: "myDeserializer" }"#);
        let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
        let opts = result.options;
        assert!(opts.serialize_with.is_none());
        assert_eq!(opts.deserialize_with.as_deref(), Some("myDeserializer"));
    }

    #[test]
    fn test_field_serialize_and_deserialize_with() {
        let decorator =
            make_decorator(r#"{ serializeWith: "toJson", deserializeWith: "fromJson" }"#);
        let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
        let opts = result.options;
        assert_eq!(opts.serialize_with.as_deref(), Some("toJson"));
        assert_eq!(opts.deserialize_with.as_deref(), Some("fromJson"));
    }

    #[test]
    fn test_field_serialize_with_combined_with_other_options() {
        let decorator = make_decorator(
            r#"{ serializeWith: "customSerialize", rename: "custom_field", skip: false }"#,
        );
        let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
        let opts = result.options;
        assert_eq!(opts.serialize_with.as_deref(), Some("customSerialize"));
        assert_eq!(opts.rename.as_deref(), Some("custom_field"));
        assert!(!opts.skip);
    }

    // ========================================================================
    // String literal type classification tests
    // ========================================================================

    #[test]
    fn test_type_category_string_literal_double_quotes() {
        // String literal types like "Zoned" should be treated as primitives
        assert_eq!(
            TypeCategory::from_ts_type(r#""Zoned""#),
            TypeCategory::Primitive
        );
        assert_eq!(
            TypeCategory::from_ts_type(r#""some_value""#),
            TypeCategory::Primitive
        );
    }

    #[test]
    fn test_type_category_string_literal_single_quotes() {
        // Single-quoted string literals should also be primitive
        assert_eq!(TypeCategory::from_ts_type("'foo'"), TypeCategory::Primitive);
        assert_eq!(
            TypeCategory::from_ts_type("'bar_baz'"),
            TypeCategory::Primitive
        );
    }

    #[test]
    fn test_type_category_non_literal_type_names() {
        // Regular type names should still be Serializable
        assert_eq!(
            TypeCategory::from_ts_type("Zoned"),
            TypeCategory::Serializable("Zoned".into())
        );
        assert_eq!(
            TypeCategory::from_ts_type("User"),
            TypeCategory::Serializable("User".into())
        );
    }

    // ========================================================================
    // TypeScript utility type classification tests
    // ========================================================================

    #[test]
    fn test_type_category_record() {
        // Record<K, V> should be properly parsed as a Record variant
        assert_eq!(
            TypeCategory::from_ts_type("Record<string, unknown>"),
            TypeCategory::Record("string".into(), "unknown".into())
        );
        assert_eq!(
            TypeCategory::from_ts_type("Record<string, number>"),
            TypeCategory::Record("string".into(), "number".into())
        );
        assert_eq!(
            TypeCategory::from_ts_type("Record<string, User>"),
            TypeCategory::Record("string".into(), "User".into())
        );
    }

    #[test]
    fn test_type_category_wrapper_utility_types() {
        // Wrapper utility types that preserve structure should extract the inner type
        assert_eq!(
            TypeCategory::from_ts_type("Partial<User>"),
            TypeCategory::Wrapper("User".into())
        );
        assert_eq!(
            TypeCategory::from_ts_type("Required<Config>"),
            TypeCategory::Wrapper("Config".into())
        );
        assert_eq!(
            TypeCategory::from_ts_type("Readonly<Data>"),
            TypeCategory::Wrapper("Data".into())
        );
        assert_eq!(
            TypeCategory::from_ts_type("NonNullable<User>"),
            TypeCategory::Wrapper("User".into())
        );
        // Pick and Omit extract the first type argument
        assert_eq!(
            TypeCategory::from_ts_type("Pick<User, 'name' | 'email'>"),
            TypeCategory::Wrapper("User".into())
        );
        assert_eq!(
            TypeCategory::from_ts_type("Omit<User, 'password'>"),
            TypeCategory::Wrapper("User".into())
        );
    }

    #[test]
    fn test_type_category_non_serializable_utility_types() {
        // Utility types operating on functions/unions/async should be Unknown
        assert_eq!(
            TypeCategory::from_ts_type("Promise<string>"),
            TypeCategory::Unknown
        );
        assert_eq!(
            TypeCategory::from_ts_type("ReturnType<typeof fn>"),
            TypeCategory::Unknown
        );
        assert_eq!(
            TypeCategory::from_ts_type("Awaited<Promise<User>>"),
            TypeCategory::Unknown
        );
    }
}
