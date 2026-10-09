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
//! The search starts from the file being expanded (or the current directory)
//! and stops at the nearest package manifest: `package.json`, `deno.json` or
//! `deno.jsonc`. [`find_project_root`] applies the same rule to locate the
//! directory that owns a project's `.macroforge/` state.
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
//!       encode: (v) => DateTime.formatIso(v),
//!       decode: (raw) => DateTime.unsafeFromDate(new Date(raw)),
//!       default: () => DateTime.unsafeNow(),
//!       hasShape: (v) => v instanceof Date || typeof v === "string"
//!     }
//!   }
//! };
//! ```
//!
//! ## Inheriting Configuration
//!
//! The config is read statically, never run, and the reader follows what it
//! inherits:
//!
//! - `extends: "./base.config.ts"` (or an array of them, later ones winning)
//!   builds on a base config. Fields the config sets replace the base's,
//!   except `foreignTypes`, which merge by type name.
//! - `export default base`, `export default { ...base, keepDecorators: true }`
//!   and `defineConfig(base)` use a config imported from another module or
//!   declared in the file, with JavaScript's spread semantics: a field set
//!   after a spread replaces the spread one.
//! - `foreignTypes: { ...base.foreignTypes, … }` spreads another config's
//!   foreign types.
//!
//! Relative specifiers resolve against the importing file, and package
//! specifiers from the nearest `node_modules`, honouring `exports` and
//! `main`. A reference the reader cannot follow, such as a config built by a
//! function call, is an error.
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
//! | `encode` | Function `(value) => unknown` for encoding |
//! | `decode` | Function `(raw) => T` for decoding |
//! | `default` | Function `() => T` for default value generation |
//!
//! ### Import Source Validation
//!
//! Foreign types are only matched when the type is imported from one of the configured
//! sources (in `from` or `aliases`). Types imported from other packages with the same
//! name are ignored, falling back to generic handling.

mod attribute_blocks;
mod loader;
mod resolve;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod resolve_tests;

pub use loader::MacroforgeConfigLoader;

use dashmap::DashMap;
use std::sync::LazyLock;

// Re-export config types from macroforge_ts_syn so they're available in MacroContextIR
// and can be passed to external macro processes.
pub use macroforge_ts_syn::config::{
    ForeignTypeAlias, ForeignTypeConfig, ImportInfo, MacroforgeConfig,
};

/// A parsed configuration, a fingerprint of the file content it came from,
/// and the stamps of the base configs it was built from.
#[derive(Debug, Clone)]
pub struct CachedConfig {
    pub content_hash: u64,
    /// Shared with every expansion that uses it.
    pub config: std::sync::Arc<MacroforgeConfig>,
    pub dependencies: Vec<(std::path::PathBuf, crate::host::file_stamp::FileStamp)>,
}

impl CachedConfig {
    /// Whether every base config is still the file this was built from.
    pub fn dependencies_unchanged(&self) -> bool {
        self.dependencies.iter().all(|(path, stamp)| {
            crate::host::file_stamp::FileStamp::of(path).is_ok_and(|current| current == *stamp)
        })
    }
}

/// Global cache for parsed configurations, keyed by config file path. An entry
/// is reused only while the file's content matches its fingerprint, so a
/// long-lived process picks up edits to the config.
pub static CONFIG_CACHE: LazyLock<DashMap<String, CachedConfig>> = LazyLock::new(DashMap::new);

/// Clear the configuration cache.
///
/// This is useful for testing to ensure each test starts with a clean state.
/// In production, clearing the cache will force configs to be re-parsed on next access.
pub fn clear_config_cache() {
    CONFIG_CACHE.clear();
}

/// Supported config file names in order of precedence.
pub(crate) const CONFIG_FILES: &[&str] = &[
    "macroforge.config.ts",
    "macroforge.config.mts",
    "macroforge.config.js",
    "macroforge.config.mjs",
    "macroforge.config.cjs",
];

/// Package manifests that mark a directory as a project root.
const PROJECT_MANIFESTS: &[&str] = &["package.json", "deno.json", "deno.jsonc"];

/// Whether `dir` holds a package manifest, which bounds the config search.
fn has_project_manifest(dir: &std::path::Path) -> bool {
    PROJECT_MANIFESTS.iter().any(|name| dir.join(name).exists())
}

/// The project that owns `start`: the nearest directory at or above it that
/// holds a macroforge config or a package manifest. A file starts the search
/// from its own directory. `None` when nothing above `start` is a project.
pub fn find_project_root(start: &std::path::Path) -> Option<std::path::PathBuf> {
    let start_dir = if start.is_file() {
        start.parent()?
    } else {
        start
    };
    start_dir.ancestors().find_map(|dir| {
        (has_project_manifest(dir) || CONFIG_FILES.iter().any(|name| dir.join(name).exists()))
            .then(|| dir.to_path_buf())
    })
}

/// The expansion settings a [`MacroExpander`](crate::host::MacroExpander) reads,
/// taken from a [`MacroforgeConfig`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacroConfig {
    /// Whether to preserve `@derive` decorators in the output code.
    pub keep_decorators: bool,

    /// Whether to generate a convenience const for non-class types.
    pub generate_convenience_const: bool,

    /// Maximum number of diagnostics one expansion reports.
    pub max_diagnostics: usize,
}

impl Default for MacroConfig {
    fn default() -> Self {
        Self {
            keep_decorators: false,
            generate_convenience_const:
                macroforge_ts_syn::config::default_generate_convenience_const(),
            max_diagnostics: 100,
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
}
