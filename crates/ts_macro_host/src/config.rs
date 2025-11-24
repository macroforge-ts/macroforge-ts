//! Configuration for the macro host

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

const DEFAULT_CONFIG_FILENAME: &str = "macro.toml";
const LEGACY_CONFIG_FILENAME: &str = "ts-macro.config.json";

/// Configuration for the macro host system
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct MacroConfig {
    /// List of macro packages to load
    pub macro_packages: Vec<String>,

    /// Whether to allow native macros (default: false for security)
    #[serde(default)]
    pub allow_native_macros: bool,

    /// Per-package runtime overrides
    #[serde(default)]
    pub macro_runtime_overrides: std::collections::HashMap<String, RuntimeMode>,

    /// Resource limits for macro execution
    #[serde(default)]
    pub limits: ResourceLimits,
}

/// Runtime mode for macro execution
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuntimeMode {
    /// Execute in WASM sandbox
    Wasm,
    /// Execute as native code (requires allow_native_macros)
    Native,
}

/// Resource limits for macro execution
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceLimits {
    /// Maximum execution time per macro in milliseconds
    #[serde(default = "default_max_execution_time")]
    pub max_execution_time_ms: u64,

    /// Maximum memory usage in bytes (for WASM)
    #[serde(default = "default_max_memory")]
    pub max_memory_bytes: usize,

    /// Maximum output size in bytes
    #[serde(default = "default_max_output_size")]
    pub max_output_size: usize,

    /// Maximum number of diagnostics
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
    5000 // 5 seconds
}

fn default_max_memory() -> usize {
    100 * 1024 * 1024 // 100MB
}

fn default_max_output_size() -> usize {
    10 * 1024 * 1024 // 10MB
}

fn default_max_diagnostics() -> usize {
    100
}

impl MacroConfig {
    /// Load configuration from a file
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)?;
        match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) if ext.eq_ignore_ascii_case("json") => Ok(serde_json::from_str(&content)?),
            _ => Ok(toml::from_str(&content)?),
        }
    }

    /// Try to find and load configuration file
    /// Looks for macro.toml (preferred) or legacy ts-macro.config.json in current directory and ancestors
    pub fn find_and_load() -> Result<Option<Self>> {
        let current_dir = std::env::current_dir()?;
        Self::find_config_in_ancestors(&current_dir)
    }

    fn find_config_in_ancestors(start_dir: &Path) -> Result<Option<Self>> {
        let mut current = start_dir.to_path_buf();

        loop {
            if let Some(config) = Self::load_if_exists(&current.join(DEFAULT_CONFIG_FILENAME))? {
                return Ok(Some(config));
            }

            if let Some(config) = Self::load_if_exists(&current.join(LEGACY_CONFIG_FILENAME))? {
                return Ok(Some(config));
            }

            // Check for package.json as a stop condition
            if current.join("package.json").exists() {
                // We're at a package root, stop searching
                break;
            }

            // Move to parent directory
            if !current.pop() {
                break;
            }
        }

        Ok(None)
    }

    fn load_if_exists(path: &Path) -> Result<Option<Self>> {
        if path.exists() {
            Ok(Some(Self::from_file(path)?))
        } else {
            Ok(None)
        }
    }

    /// Save configuration to a file
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        let content = match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) if ext.eq_ignore_ascii_case("json") => serde_json::to_string_pretty(self)?,
            _ => toml::to_string_pretty(self)?,
        };
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
        };

        let toml = toml::to_string(&config).unwrap();
        let parsed: MacroConfig = toml::from_str(&toml).unwrap();

        assert_eq!(config.macro_packages, parsed.macro_packages);
        assert_eq!(config.allow_native_macros, parsed.allow_native_macros);
    }
}
