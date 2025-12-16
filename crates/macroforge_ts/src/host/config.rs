//! # Configuration for the Macro Host
//!
//! This module handles loading and managing configuration for the macro system.
//! Configuration can be provided via a `macroforge.json` file in the project root.
//!
//! ## Configuration File Locations
//!
//! The system searches for configuration in this order:
//! 1. `macroforge.json` (preferred)
//! 2. `macroforge.config.json` (legacy)
//!
//! The search starts from the current directory and walks up to the nearest
//! `package.json` (project root).
//!
//! ## Example Configuration
//!
//! ```json
//! {
//!   "macroPackages": ["@my-org/custom-macros"],
//!   "allowNativeMacros": false,
//!   "keepDecorators": false,
//!   "functionNamingStyle": "prefix",
//!   "generateConvenienceConst": true,
//!   "limits": {
//!     "maxExecutionTimeMs": 5000,
//!     "maxMemoryBytes": 104857600,
//!     "maxOutputSize": 10485760,
//!     "maxDiagnostics": 100
//!   }
//! }
//! ```
//!
//! ## Security Considerations
//!
//! - `allowNativeMacros` defaults to `false` for security
//! - WASM sandboxing is recommended for untrusted macro packages
//! - Resource limits prevent runaway macro execution

use super::error::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

// Re-export FunctionNamingStyle so users can configure it
pub use crate::ts_syn::abi::FunctionNamingStyle;

/// Default configuration filename (preferred).
const DEFAULT_CONFIG_FILENAME: &str = "macroforge.json";
/// Legacy configuration filename (for backwards compatibility).
const LEGACY_CONFIG_FILENAME: &str = "macroforge.config.json";

/// Configuration for the macro host system.
///
/// This struct represents the contents of a `macroforge.json` file.
/// It controls macro loading, execution, and security settings.
///
/// # Default Behavior
///
/// When no configuration file is found, the system uses sensible defaults:
/// - No external macro packages (only built-in macros)
/// - Native macros disabled
/// - Decorators stripped from output
/// - Prefix function naming style
/// - Convenience const generation enabled
/// - Standard resource limits
///
/// # Example
///
/// ```ignore
/// use macroforge_ts::host::MacroConfig;
///
/// // Load from file
/// let config = MacroConfig::from_file("macroforge.json")?;
///
/// // Or find automatically
/// let config = MacroConfig::find_and_load()?.unwrap_or_default();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MacroConfig {
    /// List of macro packages to load (e.g., `["@my-org/macros"]`).
    ///
    /// These packages are loaded in addition to built-in macros.
    /// Package resolution uses Node.js module resolution rules.
    #[serde(default)]
    pub macro_packages: Vec<String>,

    /// Whether to allow native (non-WASM) macros.
    ///
    /// Defaults to `false` for security. Native macros can execute
    /// arbitrary code with full system access.
    #[serde(default)]
    pub allow_native_macros: bool,

    /// Per-package runtime mode overrides.
    ///
    /// Allows forcing specific packages to run in WASM or native mode.
    /// Keys are package names, values are runtime modes.
    #[serde(default)]
    pub macro_runtime_overrides: std::collections::HashMap<String, RuntimeMode>,

    /// Resource limits for macro execution.
    ///
    /// Prevents runaway macros from consuming excessive resources.
    #[serde(default)]
    pub limits: ResourceLimits,

    /// Whether to preserve `@derive` decorators in the output code.
    ///
    /// When `false` (default), decorators are stripped after expansion.
    /// When `true`, decorators remain in the output (useful for debugging).
    #[serde(default)]
    pub keep_decorators: bool,

    /// Function naming style for generated functions on non-class types.
    ///
    /// Controls how standalone functions are named when generated for enums,
    /// interfaces, and type aliases. Classes always use instance methods.
    ///
    /// Default: `Prefix` (e.g., `myTypeClone`)
    #[serde(default)]
    pub function_naming_style: FunctionNamingStyle,

    /// Whether to generate a convenience const for non-class types.
    ///
    /// When `true` (default), generates an `export const TypeName = { ... } as const;`
    /// that groups all generated functions for a type into a single namespace-like object.
    /// For example: `export const User = { clone: userClone, serialize: userSerialize } as const;`
    ///
    /// When `false`, only the standalone functions are generated without the grouping const.
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
            function_naming_style: FunctionNamingStyle::default(),
            generate_convenience_const: default_generate_convenience_const(),
        }
    }
}

/// Runtime mode for macro execution.
///
/// Determines whether macros execute in a WASM sandbox or as native code.
///
/// # Security
///
/// - `Wasm`: Sandboxed execution with memory and CPU limits (recommended)
/// - `Native`: Full system access, only for trusted packages
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuntimeMode {
    /// Execute in a WebAssembly sandbox.
    ///
    /// Provides isolation and resource limits but may have
    /// performance overhead compared to native execution.
    Wasm,
    /// Execute as native Rust code.
    ///
    /// Maximum performance but no sandboxing. Requires
    /// `allow_native_macros: true` in configuration.
    Native,
}

/// Resource limits for macro execution.
///
/// These limits protect against:
/// - Infinite loops or slow macros (`max_execution_time_ms`)
/// - Memory exhaustion (`max_memory_bytes`)
/// - Output bombs (`max_output_size`)
/// - Diagnostic spam (`max_diagnostics`)
///
/// # Defaults
///
/// | Limit | Default Value |
/// |-------|---------------|
/// | `max_execution_time_ms` | 5000 (5 seconds) |
/// | `max_memory_bytes` | 104857600 (100 MB) |
/// | `max_output_size` | 10485760 (10 MB) |
/// | `max_diagnostics` | 100 |
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceLimits {
    /// Maximum execution time per macro invocation in milliseconds.
    ///
    /// Default: 5000 (5 seconds)
    #[serde(default = "default_max_execution_time")]
    pub max_execution_time_ms: u64,

    /// Maximum memory usage in bytes (primarily for WASM runtime).
    ///
    /// Default: 100 MB (104857600 bytes)
    #[serde(default = "default_max_memory")]
    pub max_memory_bytes: usize,

    /// Maximum size of generated output in bytes.
    ///
    /// Prevents macros from generating excessively large code.
    /// Default: 10 MB (10485760 bytes)
    #[serde(default = "default_max_output_size")]
    pub max_output_size: usize,

    /// Maximum number of diagnostics a single macro can emit.
    ///
    /// Prevents diagnostic spam from overwhelming the user.
    /// Default: 100
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

/// Returns the default maximum execution time (5 seconds).
fn default_max_execution_time() -> u64 {
    5000
}

/// Returns the default maximum memory (100 MB).
fn default_max_memory() -> usize {
    100 * 1024 * 1024
}

/// Returns the default maximum output size (10 MB).
fn default_max_output_size() -> usize {
    10 * 1024 * 1024
}

/// Returns the default maximum diagnostics (100).
fn default_max_diagnostics() -> usize {
    100
}

/// Returns the default for generate_convenience_const (true).
fn default_generate_convenience_const() -> bool {
    true
}

impl MacroConfig {
    /// Loads configuration from a JSON file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the configuration file
    ///
    /// # Returns
    ///
    /// The parsed configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read
    /// - The JSON is malformed
    /// - Required fields are missing
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }

    /// Finds and loads a configuration file, returning both the config and its directory.
    ///
    /// Searches for `macroforge.json` (preferred) or `macroforge.config.json` (legacy)
    /// starting from the current directory and walking up to the nearest `package.json`.
    ///
    /// # Returns
    ///
    /// - `Ok(Some((config, dir)))` - Found and loaded configuration
    /// - `Ok(None)` - No configuration file found
    /// - `Err(_)` - Error reading or parsing configuration
    ///
    /// # Example
    ///
    /// ```ignore
    /// if let Some((config, project_root)) = MacroConfig::find_with_root()? {
    ///     println!("Found config in: {:?}", project_root);
    ///     println!("Macro packages: {:?}", config.macro_packages);
    /// }
    /// ```
    pub fn find_with_root() -> Result<Option<(Self, std::path::PathBuf)>> {
        let current_dir = std::env::current_dir()?;
        Self::find_config_in_ancestors(&current_dir)
    }

    /// Finds and loads a configuration file, returning just the config.
    ///
    /// This is a convenience wrapper around [`find_with_root`](Self::find_with_root)
    /// for callers that don't need the project root path.
    ///
    /// # Returns
    ///
    /// - `Ok(Some(config))` - Found and loaded configuration
    /// - `Ok(None)` - No configuration file found
    /// - `Err(_)` - Error reading or parsing configuration
    pub fn find_and_load() -> Result<Option<Self>> {
        Ok(Self::find_with_root()?.map(|(cfg, _)| cfg))
    }

    /// Searches for configuration files in the directory hierarchy.
    ///
    /// # Algorithm
    ///
    /// 1. Check for `macroforge.json` in current directory
    /// 2. Check for `macroforge.config.json` (legacy) in current directory
    /// 3. If `package.json` exists, stop searching (project root reached)
    /// 4. Otherwise, move to parent directory and repeat
    ///
    /// # Arguments
    ///
    /// * `start_dir` - Directory to start searching from
    fn find_config_in_ancestors(start_dir: &Path) -> Result<Option<(Self, std::path::PathBuf)>> {
        let mut current = start_dir.to_path_buf();

        loop {
            // Try preferred filename first
            if let Some(config) = Self::load_if_exists(&current.join(DEFAULT_CONFIG_FILENAME))? {
                return Ok(Some((config, current.clone())));
            }

            // Fall back to legacy filename
            if let Some(config) = Self::load_if_exists(&current.join(LEGACY_CONFIG_FILENAME))? {
                return Ok(Some((config, current.clone())));
            }

            // Check for package.json as a stop condition.
            // If we've reached a package root without finding config, stop searching.
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

    /// Loads configuration from a path if the file exists.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to check and potentially load
    ///
    /// # Returns
    ///
    /// - `Ok(Some(config))` if file exists and was loaded
    /// - `Ok(None)` if file doesn't exist
    /// - `Err(_)` if file exists but couldn't be parsed
    fn load_if_exists(path: &Path) -> Result<Option<Self>> {
        if path.exists() {
            Ok(Some(Self::from_file(path)?))
        } else {
            Ok(None)
        }
    }

    /// Saves configuration to a JSON file.
    ///
    /// The output is pretty-printed for human readability.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to write the configuration to
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written.
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_serialization() {
        let config = MacroConfig {
            macro_packages: vec!["@macro/derive".to_string()],
            allow_native_macros: false,
            macro_runtime_overrides: Default::default(),
            limits: Default::default(),
            keep_decorators: false,
            function_naming_style: FunctionNamingStyle::Suffix,
            generate_convenience_const: false,
        };

        let json = serde_json::to_string(&config).unwrap();
        let parsed: MacroConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.macro_packages, parsed.macro_packages);
        assert_eq!(config.allow_native_macros, parsed.allow_native_macros);
        assert_eq!(config.function_naming_style, parsed.function_naming_style);
        assert_eq!(
            config.generate_convenience_const,
            parsed.generate_convenience_const
        );
    }

    #[test]
    fn test_function_naming_style_default() {
        let config = MacroConfig::default();
        assert_eq!(config.function_naming_style, FunctionNamingStyle::Prefix);
    }

    #[test]
    fn test_generate_convenience_const_default() {
        let config = MacroConfig::default();
        assert!(config.generate_convenience_const);
    }

    #[test]
    fn test_generate_convenience_const_deserialization() {
        // Default should be true when not specified
        let json = r#"{}"#;
        let config: MacroConfig = serde_json::from_str(json).unwrap();
        assert!(config.generate_convenience_const);

        // Explicit false
        let json = r#"{"generateConvenienceConst": false}"#;
        let config: MacroConfig = serde_json::from_str(json).unwrap();
        assert!(!config.generate_convenience_const);

        // Explicit true
        let json = r#"{"generateConvenienceConst": true}"#;
        let config: MacroConfig = serde_json::from_str(json).unwrap();
        assert!(config.generate_convenience_const);
    }

    #[test]
    fn test_function_naming_style_deserialization() {
        let json = r#"{"functionNamingStyle": "generic"}"#;
        let config: MacroConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.function_naming_style, FunctionNamingStyle::Generic);

        let json = r#"{"functionNamingStyle": "prefix"}"#;
        let config: MacroConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.function_naming_style, FunctionNamingStyle::Prefix);

        let json = r#"{"functionNamingStyle": "namespace"}"#;
        let config: MacroConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.function_naming_style, FunctionNamingStyle::Namespace);
    }
}
