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
//!   foreignTypes: {
//!     DateTime: {
//!       from: ["effect", "@effect/schema"],
//!       serialize: (v, ctx) => v.toJSON(),
//!       deserialize: (raw, ctx) => DateTime.fromJSON(raw),
//!       default: () => DateTime.now()
//!     }
//!   }
//! }
//! ```
//!
//! ## Foreign Types
//!
//! Foreign types allow global registration of handlers for external types.
//! When a field has a type that matches a configured foreign type, the appropriate
//! handler function is used automatically without per-field annotations.

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

/// Configuration for a single foreign type.
///
/// Foreign types allow global registration of handlers for external types
/// (like Effect's `DateTime`) so they work like primitives without per-field annotations.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ForeignTypeConfig {
    /// Type name (e.g., "DateTime").
    pub name: String,

    /// Import sources where this type can come from (e.g., ["effect", "@effect/schema"]).
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
                    if let Some(first_arg) = call.args.first() {
                        if let Expr::Object(obj) = &*first_arg.expr {
                            return parse_config_object(obj, imports, cm);
                        }
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
        if let PropOrSpread::Prop(prop) = prop {
            if let Prop::KeyValue(kv) = &**prop {
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
    }

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
        if let PropOrSpread::Prop(prop) = prop {
            if let Prop::KeyValue(kv) = &**prop {
                let type_name = get_prop_key(&kv.key);
                if let Expr::Object(type_obj) = &*kv.value {
                    let ft = parse_single_foreign_type(&type_name, type_obj, imports, cm)?;
                    foreign_types.push(ft);
                }
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
        if let PropOrSpread::Prop(prop) = prop {
            if let Prop::KeyValue(kv) = &**prop {
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
                    _ => {}
                }
            }
        }
    }

    Ok(ft)
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
        };

        let legacy: MacroConfig = mf_config.into();
        assert!(legacy.keep_decorators);
        assert!(!legacy.generate_convenience_const);
    }
}
