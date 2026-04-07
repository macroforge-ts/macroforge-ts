//! # Configuration for the Macro Host
//!
//! This module handles loading and managing configuration for the macro system.
//! Configuration is provided via a `macroforge.config.ts` (or `.mts`, `.js`, `.mjs`, `.cjs`) file
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

mod loader;
#[cfg(feature = "swc")]
mod namespaces;
#[cfg(feature = "swc")]
mod parser;

#[cfg(test)]
mod tests;

pub use loader::MacroforgeConfigLoader;
#[cfg(feature = "swc")]
pub use namespaces::extract_expression_namespaces;

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

// Re-export config types from macroforge_ts_syn so they're available in MacroContextIR
// and can be passed to external macro processes.
pub use macroforge_ts_syn::config::{
    ForeignTypeAlias, ForeignTypeConfig, ImportInfo, MacroforgeConfig,
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
    #[serde(default = "macroforge_ts_syn::config::default_generate_convenience_const")]
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
            generate_convenience_const: true,
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
    pub fn find_with_root() -> super::error::Result<Option<(Self, std::path::PathBuf)>> {
        match MacroforgeConfigLoader::find_with_root()? {
            Some((cfg, path)) => Ok(Some((cfg.into(), path))),
            None => Ok(None),
        }
    }

    /// Finds and loads a configuration file, returning just the config.
    pub fn find_and_load() -> super::error::Result<Option<Self>> {
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
