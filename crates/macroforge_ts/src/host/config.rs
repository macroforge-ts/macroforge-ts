//! # Configuration for the Macro Host
//!
//! This module handles loading and managing configuration for the macro system.
//! Configuration is provided via a `macroforge.config.js` (or `.ts`, `.mjs`, `.cjs`) file
//! in the project root.
//!
//! ## Configuration File Locations
//!
//! The system searches for configuration files in this order:
//! 1. `macroforge.config.ts` (preferred)
//! 2. `macroforge.config.mts`
//! 3. `macroforge.config.js`
//! 4. `macroforge.config.mjs`
//! 5. `macroforge.config.cjs`
//!
//! The search starts from the current directory and walks up to the nearest
//! `package.json` (project root).
//!
//! ## Example Configuration
//!
//! ```javascript
//! import { DateTime } from "effect";
//!
//! export default {
//!   keepDecorators: false,
//!   generateConvenienceConst: true,
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
//! ## Configuration Caching
//!
//! Configurations are parsed once and cached globally by file path. When using
//! [`expand_sync`](crate::expand_sync) or [`NativePlugin::process_file`](crate::NativePlugin::process_file),
//! you can pass `config_path` in the options to use a previously loaded configuration.
//! This is particularly useful for accessing foreign type handlers during expansion.
//!
//! ## Foreign Types
//!
//! Foreign types allow global registration of handlers for external types.
//! When a field has a type that matches a configured foreign type, the appropriate
//! handler function is used automatically without per-field annotations.
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
//! sources (in `from` or `aliases`). Types imported from other packages with the same
//! name are ignored, falling back to generic handling.

use super::error::Result;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::LazyLock;

use swc_core::{
    common::{FileName, SourceMap, sync::Lrc},
    ecma::{
        ast::*,
        codegen::{text_writer::JsWriter, Config, Emitter, Node},
        parser::{EsSyntax, Parser, StringInput, Syntax, TsSyntax, lexer::Lexer},
    },
};

/// Global cache for parsed configurations.
/// Maps config file path to the parsed configuration.
pub static CONFIG_CACHE: LazyLock<DashMap<String, MacroforgeConfig>> = LazyLock::new(DashMap::new);

/// Clear the configuration cache.
///
/// This is useful for testing to ensure each test starts with a clean state.
/// In production, clearing the cache will force configs to be re-parsed on next access.
pub fn clear_config_cache() {
    CONFIG_CACHE.clear();
}

/// Supported config file names in order of precedence.
const CONFIG_FILES: &[&str] = &[
    "macroforge.config.ts",
    "macroforge.config.mts",
    "macroforge.config.js",
    "macroforge.config.mjs",
    "macroforge.config.cjs",
];

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

    /// Aliases for this foreign type, allowing different name-package pairs to use the same config.
    ///
    /// Useful when a type can be imported from different paths or with different names.
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
    ///     // ...
    ///   }
    /// }
    /// ```
    #[serde(default)]
    pub aliases: Vec<ForeignTypeAlias>,

    /// Namespaces referenced in expressions (serialize_expr, deserialize_expr, default_expr).
    ///
    /// This is auto-extracted during config parsing by analyzing the expression ASTs.
    /// For example, if `serialize: (v) => DateTime.formatIso(v)`, this would contain `["DateTime"]`.
    ///
    /// Used to determine which namespaces need to be imported for the generated code to work.
    #[serde(default, skip_serializing)]
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
fn default_generate_convenience_const() -> bool {
    true
}

impl Default for MacroforgeConfig {
    fn default() -> Self {
        Self {
            keep_decorators: false,
            generate_convenience_const: true, // Default to true
            foreign_types: Vec::new(),
            config_imports: HashMap::new(),
        }
    }
}

impl MacroforgeConfig {
    /// Parse a macroforge.config.js/ts file and extract configuration.
    ///
    /// Uses SWC to parse the JavaScript/TypeScript config file and extract
    /// the configuration object from the default export.
    ///
    /// # Arguments
    ///
    /// * `content` - The raw file content
    /// * `filepath` - Path to the config file (used to determine syntax)
    ///
    /// # Returns
    ///
    /// The parsed configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if parsing fails.
    pub fn from_config_file(content: &str, filepath: &str) -> Result<Self> {
        let is_typescript = filepath.ends_with(".ts") || filepath.ends_with(".mts");

        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.new_source_file(
            FileName::Custom(filepath.to_string()).into(),
            content.to_string(),
        );

        let syntax = if is_typescript {
            Syntax::Typescript(TsSyntax {
                tsx: false,
                decorators: true,
                ..Default::default()
            })
        } else {
            Syntax::Es(EsSyntax {
                decorators: true,
                ..Default::default()
            })
        };

        let lexer = Lexer::new(syntax, EsVersion::latest(), StringInput::from(&*fm), None);
        let mut parser = Parser::new_from(lexer);

        let module = parser
            .parse_module()
            .map_err(|e| super::MacroError::InvalidConfig(format!("Parse error: {:?}", e)))?;

        // Extract imports map for resolving function references
        let imports = extract_imports(&module);

        // Find default export and extract config
        let config = extract_default_export(&module, &imports, &cm)?;

        Ok(config)
    }

    /// Load configuration from cache or parse from file content.
    ///
    /// Caches the parsed configuration for reuse during macro expansion.
    ///
    /// # Arguments
    ///
    /// * `content` - The raw file content
    /// * `filepath` - Path to the config file
    ///
    /// # Returns
    ///
    /// The parsed configuration.
    pub fn load_and_cache(content: &str, filepath: &str) -> Result<Self> {
        // Check cache first
        if let Some(cached) = CONFIG_CACHE.get(filepath) {
            return Ok(cached.clone());
        }

        // Parse and cache
        let config = Self::from_config_file(content, filepath)?;
        CONFIG_CACHE.insert(filepath.to_string(), config.clone());

        Ok(config)
    }

    /// Get cached configuration by file path.
    pub fn get_cached(filepath: &str) -> Option<Self> {
        CONFIG_CACHE.get(filepath).map(|c| c.clone())
    }

    /// Find and load a configuration file from the filesystem.
    ///
    /// Searches for config files starting from `start_dir` and walking up
    /// to the nearest `package.json`.
    ///
    /// # Returns
    ///
    /// - `Ok(Some((config, dir)))` - Found and loaded configuration
    /// - `Ok(None)` - No configuration file found
    /// - `Err(_)` - Error reading or parsing configuration
    pub fn find_with_root() -> Result<Option<(Self, std::path::PathBuf)>> {
        let current_dir = std::env::current_dir()?;
        Self::find_config_in_ancestors(&current_dir)
    }

    /// Finds configuration starting from a specific path.
    ///
    /// This is useful when expanding files in different directories,
    /// as it allows finding the config relative to each file rather
    /// than from the current working directory.
    ///
    /// # Arguments
    ///
    /// * `start_path` - The file or directory to start searching from.
    ///   If a file is provided, the search starts from its parent directory.
    ///
    /// # Returns
    ///
    /// - `Ok(Some((config, dir)))` - Found and loaded configuration
    /// - `Ok(None)` - No configuration file found
    /// - `Err(_)` - Error reading or parsing configuration
    pub fn find_with_root_from_path(start_path: &Path) -> Result<Option<(Self, std::path::PathBuf)>> {
        let start_dir = if start_path.is_file() {
            start_path.parent().map(|p| p.to_path_buf()).unwrap_or_else(|| start_path.to_path_buf())
        } else {
            start_path.to_path_buf()
        };
        Self::find_config_in_ancestors(&start_dir)
    }

    /// Convenience wrapper that finds and loads config from a specific path.
    ///
    /// # Arguments
    ///
    /// * `start_path` - The file or directory to start searching from.
    pub fn find_from_path(start_path: &Path) -> Result<Option<Self>> {
        Ok(Self::find_with_root_from_path(start_path)?.map(|(cfg, _)| cfg))
    }

    /// Find configuration in ancestors, returning config and root dir.
    fn find_config_in_ancestors(start_dir: &Path) -> Result<Option<(Self, std::path::PathBuf)>> {
        let mut current = start_dir.to_path_buf();

        loop {
            // Try each config file name in order
            for config_name in CONFIG_FILES {
                let config_path = current.join(config_name);
                if config_path.exists() {
                    let content = std::fs::read_to_string(&config_path)?;
                    let config = Self::from_config_file(&content, config_path.to_string_lossy().as_ref())?;
                    return Ok(Some((config, current.clone())));
                }
            }

            // Check for package.json as a stop condition
            if current.join("package.json").exists() {
                break;
            }

            // Move to parent directory
            if !current.pop() {
                break;
            }
        }

        Ok(None)
    }

    /// Convenience wrapper around [`find_with_root`](Self::find_with_root)
    /// for callers that don't need the project root path.
    pub fn find_and_load() -> Result<Option<Self>> {
        Ok(Self::find_with_root()?.map(|(cfg, _)| cfg))
    }
}

/// Helper to convert a Wtf8Atom (string value) to a String.
fn atom_to_string(atom: &swc_core::ecma::utils::swc_atoms::Wtf8Atom) -> String {
    String::from_utf8_lossy(atom.as_bytes()).to_string()
}

/// Extract import statements into a lookup map.
fn extract_imports(module: &Module) -> HashMap<String, ImportInfo> {
    let mut imports = HashMap::new();

    for item in &module.body {
        if let ModuleItem::ModuleDecl(ModuleDecl::Import(import)) = item {
            let source = atom_to_string(&import.src.value);

            for specifier in &import.specifiers {
                match specifier {
                    ImportSpecifier::Named(named) => {
                        let local = named.local.sym.to_string();
                        let imported = named
                            .imported
                            .as_ref()
                            .map(|i| match i {
                                ModuleExportName::Ident(id) => id.sym.to_string(),
                                ModuleExportName::Str(s) => atom_to_string(&s.value),
                            })
                            .unwrap_or_else(|| local.clone());
                        imports.insert(
                            local,
                            ImportInfo {
                                name: imported,
                                source: source.clone(),
                            },
                        );
                    }
                    ImportSpecifier::Default(default) => {
                        imports.insert(
                            default.local.sym.to_string(),
                            ImportInfo {
                                name: "default".to_string(),
                                source: source.clone(),
                            },
                        );
                    }
                    ImportSpecifier::Namespace(ns) => {
                        imports.insert(
                            ns.local.sym.to_string(),
                            ImportInfo {
                                name: "*".to_string(),
                                source: source.clone(),
                            },
                        );
                    }
                }
            }
        }
    }

    imports
}

/// Extract the default export and parse it as configuration.
fn extract_default_export(
    module: &Module,
    imports: &HashMap<String, ImportInfo>,
    cm: &Lrc<SourceMap>,
) -> Result<MacroforgeConfig> {
    for item in &module.body {
        if let ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(export)) = item {
            match &*export.expr {
                // export default { ... }
                Expr::Object(obj) => {
                    return parse_config_object(obj, imports, cm);
                }
                // export default defineConfig({ ... }) or similar
                Expr::Call(call) => {
                    if let Some(first_arg) = call.args.first()
                        && let Expr::Object(obj) = &*first_arg.expr
                    {
                        return parse_config_object(obj, imports, cm);
                    }
                }
                _ => {}
            }
        }
    }

    // No default export found - return defaults
    Ok(MacroforgeConfig::default())
}

/// Parse the config object to extract configuration values.
fn parse_config_object(
    obj: &ObjectLit,
    imports: &HashMap<String, ImportInfo>,
    cm: &Lrc<SourceMap>,
) -> Result<MacroforgeConfig> {
    let mut config = MacroforgeConfig::default();

    for prop in &obj.props {
        if let PropOrSpread::Prop(prop) = prop
            && let Prop::KeyValue(kv) = &**prop
        {
            let key = get_prop_key(&kv.key);

            match key.as_str() {
                "keepDecorators" => {
                    config.keep_decorators = get_bool_value(&kv.value).unwrap_or(false);
                }
                "generateConvenienceConst" => {
                    config.generate_convenience_const =
                        get_bool_value(&kv.value).unwrap_or(true);
                }
                "foreignTypes" => {
                    if let Expr::Object(ft_obj) = &*kv.value {
                        config.foreign_types = parse_foreign_types(ft_obj, imports, cm)?;
                    }
                }
                _ => {}
            }
        }
    }

    // Store the config file's imports for use when generating namespace imports
    config.config_imports = imports.clone();

    Ok(config)
}

/// Parse the foreignTypes object.
fn parse_foreign_types(
    obj: &ObjectLit,
    imports: &HashMap<String, ImportInfo>,
    cm: &Lrc<SourceMap>,
) -> Result<Vec<ForeignTypeConfig>> {
    let mut foreign_types = vec![];

    for prop in &obj.props {
        if let PropOrSpread::Prop(prop) = prop
            && let Prop::KeyValue(kv) = &**prop
        {
            let type_name = get_prop_key(&kv.key);
            if let Expr::Object(type_obj) = &*kv.value {
                let ft = parse_single_foreign_type(&type_name, type_obj, imports, cm)?;
                foreign_types.push(ft);
            }
        }
    }

    Ok(foreign_types)
}

/// Parse a single foreign type configuration.
fn parse_single_foreign_type(
    name: &str,
    obj: &ObjectLit,
    imports: &HashMap<String, ImportInfo>,
    cm: &Lrc<SourceMap>,
) -> Result<ForeignTypeConfig> {
    let mut ft = ForeignTypeConfig {
        name: name.to_string(),
        ..Default::default()
    };

    for prop in &obj.props {
        if let PropOrSpread::Prop(prop) = prop
            && let Prop::KeyValue(kv) = &**prop
        {
            let key = get_prop_key(&kv.key);

            match key.as_str() {
                "from" => {
                    ft.from = extract_string_or_array(&kv.value);
                }
                "serialize" => {
                    let (expr, import) = extract_function_expr(&kv.value, imports, cm);
                    ft.serialize_expr = expr;
                    ft.serialize_import = import;
                }
                "deserialize" => {
                    let (expr, import) = extract_function_expr(&kv.value, imports, cm);
                    ft.deserialize_expr = expr;
                    ft.deserialize_import = import;
                }
                "default" => {
                    let (expr, import) = extract_function_expr(&kv.value, imports, cm);
                    ft.default_expr = expr;
                    ft.default_import = import;
                }
                "aliases" => {
                    ft.aliases = parse_aliases_array(&kv.value);
                }
                _ => {}
            }
        }
    }

    // Extract namespace references from all expressions
    let mut all_namespaces = std::collections::HashSet::new();
    if let Some(ref expr) = ft.serialize_expr {
        for ns in extract_expression_namespaces(expr) {
            all_namespaces.insert(ns);
        }
    }
    if let Some(ref expr) = ft.deserialize_expr {
        for ns in extract_expression_namespaces(expr) {
            all_namespaces.insert(ns);
        }
    }
    if let Some(ref expr) = ft.default_expr {
        for ns in extract_expression_namespaces(expr) {
            all_namespaces.insert(ns);
        }
    }
    ft.expression_namespaces = all_namespaces.into_iter().collect();

    Ok(ft)
}

/// Parse an array of aliases: [{ name: "DateTime", from: "effect/DateTime" }, ...]
fn parse_aliases_array(expr: &Expr) -> Vec<ForeignTypeAlias> {
    let mut aliases = Vec::new();

    if let Expr::Array(arr) = expr {
        for elem in arr.elems.iter().flatten() {
            if let Expr::Object(obj) = &*elem.expr
                && let Some(alias) = parse_single_alias(obj)
            {
                aliases.push(alias);
            }
        }
    }

    aliases
}

/// Parse a single alias object: { name: "DateTime", from: "effect/DateTime" }
fn parse_single_alias(obj: &ObjectLit) -> Option<ForeignTypeAlias> {
    let mut name = None;
    let mut from = None;

    for prop in &obj.props {
        if let PropOrSpread::Prop(prop) = prop
            && let Prop::KeyValue(kv) = &**prop
        {
            let key = get_prop_key(&kv.key);

            match key.as_str() {
                "name" => {
                    if let Expr::Lit(Lit::Str(s)) = &*kv.value {
                        name = Some(atom_to_string(&s.value));
                    }
                }
                "from" => {
                    if let Expr::Lit(Lit::Str(s)) = &*kv.value {
                        from = Some(atom_to_string(&s.value));
                    }
                }
                _ => {}
            }
        }
    }

    // Both name and from are required
    match (name, from) {
        (Some(name), Some(from)) => Some(ForeignTypeAlias { name, from }),
        _ => None,
    }
}

/// Get property key as string.
fn get_prop_key(key: &PropName) -> String {
    match key {
        PropName::Ident(id) => id.sym.to_string(),
        PropName::Str(s) => atom_to_string(&s.value),
        PropName::Num(n) => n.value.to_string(),
        PropName::BigInt(b) => b.value.to_string(),
        PropName::Computed(c) => {
            if let Expr::Lit(Lit::Str(s)) = &*c.expr {
                atom_to_string(&s.value)
            } else {
                "[computed]".to_string()
            }
        }
    }
}

/// Get boolean value from expression.
fn get_bool_value(expr: &Expr) -> Option<bool> {
    match expr {
        Expr::Lit(Lit::Bool(b)) => Some(b.value),
        _ => None,
    }
}

/// Extract string or array of strings from expression.
fn extract_string_or_array(expr: &Expr) -> Vec<String> {
    match expr {
        Expr::Lit(Lit::Str(s)) => vec![atom_to_string(&s.value)],
        Expr::Array(arr) => arr
            .elems
            .iter()
            .filter_map(|elem| {
                elem.as_ref().and_then(|e| {
                    if let Expr::Lit(Lit::Str(s)) = &*e.expr {
                        Some(atom_to_string(&s.value))
                    } else {
                        None
                    }
                })
            })
            .collect(),
        _ => vec![],
    }
}

/// Extract function expression - either inline arrow or reference to imported/declared function.
fn extract_function_expr(
    expr: &Expr,
    imports: &HashMap<String, ImportInfo>,
    cm: &Lrc<SourceMap>,
) -> (Option<String>, Option<ImportInfo>) {
    match expr {
        // Inline arrow function: (v, ctx) => v.toJSON()
        Expr::Arrow(_) => {
            let source = codegen_expr(expr, cm);
            (Some(source), None)
        }
        // Function expression: function(v, ctx) { return v.toJSON(); }
        Expr::Fn(_) => {
            let source = codegen_expr(expr, cm);
            (Some(source), None)
        }
        // Reference to a variable: serializeDateTime
        Expr::Ident(ident) => {
            let name = ident.sym.to_string();
            if let Some(import_info) = imports.get(&name) {
                // It's an imported function
                (Some(name.clone()), Some(import_info.clone()))
            } else {
                // It's a locally declared function - just use the name
                (Some(name), None)
            }
        }
        // Member expression: DateTime.fromJSON
        Expr::Member(_) => {
            let source = codegen_expr(expr, cm);
            (Some(source), None)
        }
        _ => (None, None),
    }
}

/// Convert expression AST back to source code using SWC's codegen.
fn codegen_expr(expr: &Expr, cm: &Lrc<SourceMap>) -> String {
    let mut buf = Vec::new();

    {
        let writer = JsWriter::new(cm.clone(), "\n", &mut buf, None);
        let mut emitter = Emitter {
            cfg: Config::default(),
            cm: cm.clone(),
            comments: None,
            wr: writer,
        };

        // Use the Node trait's emit_with method
        if expr.emit_with(&mut emitter).is_err() {
            return String::new();
        }
    }

    String::from_utf8(buf).unwrap_or_default()
}

/// Extract namespace identifiers referenced in an expression string.
///
/// Parses the expression and finds all member expression roots that could be namespaces.
/// For example, `(v) => DateTime.formatIso(v)` would return `["DateTime"]`.
///
/// This is used to determine which namespaces need to be imported for foreign type
/// expressions to work at runtime.
pub fn extract_expression_namespaces(expr_str: &str) -> Vec<String> {
    use std::collections::HashSet;

    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(
        FileName::Custom("expr.ts".to_string()).into(),
        expr_str.to_string(),
    );

    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax {
            tsx: false,
            decorators: false,
            ..Default::default()
        }),
        EsVersion::latest(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);
    let expr = match parser.parse_expr() {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };

    let mut namespaces = HashSet::new();
    collect_member_expression_roots(&expr, &mut namespaces);
    namespaces.into_iter().collect()
}

/// Recursively collect root identifiers from member expressions.
///
/// For `DateTime.formatIso(v)`, this extracts `DateTime`.
/// For `A.B.c()`, this extracts `A`.
fn collect_member_expression_roots(expr: &Expr, namespaces: &mut std::collections::HashSet<String>) {
    match expr {
        // Member expression: DateTime.formatIso
        Expr::Member(member) => {
            // Get the root of the member expression chain
            if let Some(root) = get_member_root(&member.obj) {
                namespaces.insert(root);
            }
            // Also check the object recursively for nested member expressions
            collect_member_expression_roots(&member.obj, namespaces);
        }
        // Call expression: DateTime.formatIso(v)
        Expr::Call(call) => {
            if let Callee::Expr(callee) = &call.callee {
                collect_member_expression_roots(callee, namespaces);
            }
            // Also check arguments
            for arg in &call.args {
                collect_member_expression_roots(&arg.expr, namespaces);
            }
        }
        // Arrow function: (v) => DateTime.formatIso(v)
        Expr::Arrow(arrow) => {
            match &*arrow.body {
                BlockStmtOrExpr::Expr(e) => collect_member_expression_roots(e, namespaces),
                BlockStmtOrExpr::BlockStmt(block) => {
                    for stmt in &block.stmts {
                        collect_statement_namespaces(stmt, namespaces);
                    }
                }
            }
        }
        // Function expression: function(v) { return DateTime.formatIso(v); }
        Expr::Fn(fn_expr) => {
            if let Some(body) = &fn_expr.function.body {
                for stmt in &body.stmts {
                    collect_statement_namespaces(stmt, namespaces);
                }
            }
        }
        // Parenthesized expression
        Expr::Paren(paren) => {
            collect_member_expression_roots(&paren.expr, namespaces);
        }
        // Binary expression
        Expr::Bin(bin) => {
            collect_member_expression_roots(&bin.left, namespaces);
            collect_member_expression_roots(&bin.right, namespaces);
        }
        // Conditional expression
        Expr::Cond(cond) => {
            collect_member_expression_roots(&cond.test, namespaces);
            collect_member_expression_roots(&cond.cons, namespaces);
            collect_member_expression_roots(&cond.alt, namespaces);
        }
        // New expression: new DateTime()
        Expr::New(new) => {
            collect_member_expression_roots(&new.callee, namespaces);
            if let Some(args) = &new.args {
                for arg in args {
                    collect_member_expression_roots(&arg.expr, namespaces);
                }
            }
        }
        // Array expression
        Expr::Array(arr) => {
            for elem in arr.elems.iter().flatten() {
                collect_member_expression_roots(&elem.expr, namespaces);
            }
        }
        // Object expression
        Expr::Object(obj) => {
            for prop in &obj.props {
                if let PropOrSpread::Prop(p) = prop {
                    if let Prop::KeyValue(kv) = &**p {
                        collect_member_expression_roots(&kv.value, namespaces);
                    }
                }
            }
        }
        // Template literal
        Expr::Tpl(tpl) => {
            for expr in &tpl.exprs {
                collect_member_expression_roots(expr, namespaces);
            }
        }
        // Sequence expression
        Expr::Seq(seq) => {
            for expr in &seq.exprs {
                collect_member_expression_roots(expr, namespaces);
            }
        }
        _ => {}
    }
}

/// Collect namespaces from statements.
fn collect_statement_namespaces(stmt: &Stmt, namespaces: &mut std::collections::HashSet<String>) {
    match stmt {
        Stmt::Return(ret) => {
            if let Some(arg) = &ret.arg {
                collect_member_expression_roots(arg, namespaces);
            }
        }
        Stmt::Expr(expr) => {
            collect_member_expression_roots(&expr.expr, namespaces);
        }
        Stmt::If(if_stmt) => {
            collect_member_expression_roots(&if_stmt.test, namespaces);
            collect_statement_namespaces(&if_stmt.cons, namespaces);
            if let Some(alt) = &if_stmt.alt {
                collect_statement_namespaces(alt, namespaces);
            }
        }
        Stmt::Block(block) => {
            for s in &block.stmts {
                collect_statement_namespaces(s, namespaces);
            }
        }
        Stmt::Decl(decl) => {
            if let Decl::Var(var) = decl {
                for decl in &var.decls {
                    if let Some(init) = &decl.init {
                        collect_member_expression_roots(init, namespaces);
                    }
                }
            }
        }
        _ => {}
    }
}

/// Get the root identifier of a member expression chain.
///
/// For `DateTime.formatIso`, returns `Some("DateTime")`.
/// For `a.b.c`, returns `Some("a")`.
fn get_member_root(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Ident(ident) => Some(ident.sym.to_string()),
        Expr::Member(member) => get_member_root(&member.obj),
        _ => None,
    }
}

// ============================================================================
// Legacy MacroConfig for backwards compatibility during transition
// ============================================================================

/// Legacy configuration struct for backwards compatibility.
///
/// This is used internally when the MacroExpander needs a simpler config format.
/// New code should use [`MacroforgeConfig`] instead.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MacroConfig {
    /// List of macro packages to load.
    #[serde(default)]
    pub macro_packages: Vec<String>,

    /// Whether to allow native (non-WASM) macros.
    #[serde(default)]
    pub allow_native_macros: bool,

    /// Per-package runtime mode overrides.
    #[serde(default)]
    pub macro_runtime_overrides: std::collections::HashMap<String, RuntimeMode>,

    /// Resource limits for macro execution.
    #[serde(default)]
    pub limits: ResourceLimits,

    /// Whether to preserve `@derive` decorators in the output code.
    #[serde(default)]
    pub keep_decorators: bool,

    /// Whether to generate a convenience const for non-class types.
    #[serde(default = "default_generate_convenience_const")]
    pub generate_convenience_const: bool,
}

impl Default for MacroConfig {
    fn default() -> Self {
        Self {
            macro_packages: Vec::new(),
            allow_native_macros: false,
            macro_runtime_overrides: Default::default(),
            limits: Default::default(),
            keep_decorators: false,
            generate_convenience_const: default_generate_convenience_const(),
        }
    }
}

impl From<MacroforgeConfig> for MacroConfig {
    fn from(cfg: MacroforgeConfig) -> Self {
        MacroConfig {
            keep_decorators: cfg.keep_decorators,
            generate_convenience_const: cfg.generate_convenience_const,
            ..Default::default()
        }
    }
}

impl MacroConfig {
    /// Finds and loads a configuration file, returning both the config and its directory.
    pub fn find_with_root() -> Result<Option<(Self, std::path::PathBuf)>> {
        match MacroforgeConfig::find_with_root()? {
            Some((cfg, path)) => Ok(Some((cfg.into(), path))),
            None => Ok(None),
        }
    }

    /// Finds and loads a configuration file, returning just the config.
    pub fn find_and_load() -> Result<Option<Self>> {
        Ok(Self::find_with_root()?.map(|(cfg, _)| cfg))
    }
}

/// Runtime mode for macro execution.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuntimeMode {
    /// Execute in a WebAssembly sandbox.
    Wasm,
    /// Execute as native Rust code.
    Native,
}

/// Resource limits for macro execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceLimits {
    /// Maximum execution time per macro invocation in milliseconds.
    #[serde(default = "default_max_execution_time")]
    pub max_execution_time_ms: u64,

    /// Maximum memory usage in bytes.
    #[serde(default = "default_max_memory")]
    pub max_memory_bytes: usize,

    /// Maximum size of generated output in bytes.
    #[serde(default = "default_max_output_size")]
    pub max_output_size: usize,

    /// Maximum number of diagnostics a single macro can emit.
    #[serde(default = "default_max_diagnostics")]
    pub max_diagnostics: usize,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_execution_time_ms: default_max_execution_time(),
            max_memory_bytes: default_max_memory(),
            max_output_size: default_max_output_size(),
            max_diagnostics: default_max_diagnostics(),
        }
    }
}

fn default_max_execution_time() -> u64 {
    5000
}

fn default_max_memory() -> usize {
    100 * 1024 * 1024
}

fn default_max_output_size() -> usize {
    10 * 1024 * 1024
}

fn default_max_diagnostics() -> usize {
    100
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_config() {
        let content = r#"
            export default {
                keepDecorators: true,
                generateConvenienceConst: false
            }
        "#;

        let config = MacroforgeConfig::from_config_file(content, "macroforge.config.js").unwrap();
        assert!(config.keep_decorators);
        assert!(!config.generate_convenience_const);
    }

    #[test]
    fn test_parse_config_with_foreign_types() {
        let content = r#"
            export default {
                foreignTypes: {
                    DateTime: {
                        from: ["effect"],
                        serialize: (v, ctx) => v.toJSON(),
                        deserialize: (raw, ctx) => DateTime.fromJSON(raw)
                    }
                }
            }
        "#;

        let config = MacroforgeConfig::from_config_file(content, "macroforge.config.js").unwrap();
        assert_eq!(config.foreign_types.len(), 1);

        let dt = &config.foreign_types[0];
        assert_eq!(dt.name, "DateTime");
        assert_eq!(dt.from, vec!["effect"]);
        assert!(dt.serialize_expr.is_some());
        assert!(dt.deserialize_expr.is_some());
    }

    #[test]
    fn test_parse_config_with_multiple_sources() {
        let content = r#"
            export default {
                foreignTypes: {
                    DateTime: {
                        from: ["effect", "@effect/schema"]
                    }
                }
            }
        "#;

        let config = MacroforgeConfig::from_config_file(content, "macroforge.config.js").unwrap();
        let dt = &config.foreign_types[0];
        assert_eq!(dt.from, vec!["effect", "@effect/schema"]);
    }

    #[test]
    fn test_parse_typescript_config() {
        let content = r#"
            import { DateTime } from "effect";

            export default {
                foreignTypes: {
                    DateTime: {
                        from: ["effect"],
                        serialize: (v: DateTime, ctx: unknown) => v.toJSON(),
                    }
                }
            }
        "#;

        let config = MacroforgeConfig::from_config_file(content, "macroforge.config.ts").unwrap();
        assert_eq!(config.foreign_types.len(), 1);
    }

    #[test]
    fn test_default_values() {
        let content = "export default {}";
        let config = MacroforgeConfig::from_config_file(content, "macroforge.config.js").unwrap();

        assert!(!config.keep_decorators);
        assert!(config.generate_convenience_const);
        assert!(config.foreign_types.is_empty());
    }

    #[test]
    fn test_legacy_macro_config_conversion() {
        let mf_config = MacroforgeConfig {
            keep_decorators: true,
            generate_convenience_const: false,
            foreign_types: vec![],
            config_imports: HashMap::new(),
        };

        let legacy: MacroConfig = mf_config.into();
        assert!(legacy.keep_decorators);
        assert!(!legacy.generate_convenience_const);
    }
}
