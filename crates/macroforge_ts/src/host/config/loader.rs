use super::super::error::Result;
use super::{CONFIG_CACHE, CONFIG_FILES, MacroforgeConfig};
use std::path::Path;

use super::parser::{extract_default_export, extract_imports};

use swc_core::{
    common::{FileName, SourceMap, sync::Lrc},
    ecma::parser::{EsSyntax, Lexer, Parser, StringInput, Syntax, TsSyntax},
};

/// Loader/parser for MacroforgeConfig files.
///
/// The [`MacroforgeConfig`] type is defined in `macroforge_ts_syn` so it can be
/// used in `MacroContextIR` for cross-process transfer. This wrapper struct
/// provides the SWC-dependent parsing and file-discovery methods that can't
/// live in the syn crate.
pub struct MacroforgeConfigLoader;

impl MacroforgeConfigLoader {
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
    pub fn from_config_file(content: &str, filepath: &str) -> Result<MacroforgeConfig> {
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

        let lexer = Lexer::new(
            syntax,
            swc_core::ecma::ast::EsVersion::latest(),
            StringInput::from(&*fm),
            None,
        );
        let mut parser = Parser::new_from(lexer);

        let module = parser.parse_module().map_err(|e| {
            super::super::MacroError::InvalidConfig(format!("Parse error: {:?}", e))
        })?;

        // Extract imports map for resolving function references
        let imports = extract_imports(&module);

        // Find default export and extract config
        let config = extract_default_export(&module, &imports, &cm)?;

        Ok(config)
    }

    /// Load configuration from cache or parse from file content.
    pub fn load_and_cache(content: &str, filepath: &str) -> Result<MacroforgeConfig> {
        if let Some(cached) = CONFIG_CACHE.get(filepath) {
            return Ok(cached.clone());
        }

        let config = Self::from_config_file(content, filepath)?;
        CONFIG_CACHE.insert(filepath.to_string(), config.clone());

        Ok(config)
    }

    /// Get cached configuration by file path.
    pub fn get_cached(filepath: &str) -> Option<MacroforgeConfig> {
        CONFIG_CACHE.get(filepath).map(|c| c.clone())
    }

    /// Find and load a configuration file from the filesystem.
    ///
    /// Searches for config files starting from the current directory and
    /// walking up to the nearest `package.json`.
    pub fn find_with_root() -> Result<Option<(MacroforgeConfig, std::path::PathBuf)>> {
        let current_dir = std::env::current_dir()?;
        Self::find_config_in_ancestors(&current_dir)
    }

    /// Finds configuration starting from a specific path.
    pub fn find_with_root_from_path(
        start_path: &Path,
    ) -> Result<Option<(MacroforgeConfig, std::path::PathBuf)>> {
        let start_dir = if start_path.is_file() {
            start_path
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| start_path.to_path_buf())
        } else {
            start_path.to_path_buf()
        };
        Self::find_config_in_ancestors(&start_dir)
    }

    /// Convenience wrapper that finds and loads config from a specific path.
    pub fn find_from_path(start_path: &Path) -> Result<Option<MacroforgeConfig>> {
        Ok(Self::find_with_root_from_path(start_path)?.map(|(cfg, _)| cfg))
    }

    /// Find configuration in ancestors, returning config and root dir.
    fn find_config_in_ancestors(
        start_dir: &Path,
    ) -> Result<Option<(MacroforgeConfig, std::path::PathBuf)>> {
        let mut current = start_dir.to_path_buf();

        loop {
            for config_name in CONFIG_FILES {
                let config_path = current.join(config_name);
                if config_path.exists() {
                    let content = std::fs::read_to_string(&config_path)?;
                    let config =
                        Self::from_config_file(&content, config_path.to_string_lossy().as_ref())?;
                    return Ok(Some((config, current.clone())));
                }
            }

            if current.join("package.json").exists() {
                break;
            }

            if !current.pop() {
                break;
            }
        }

        Ok(None)
    }

    /// Convenience wrapper for callers that don't need the project root path.
    pub fn find_and_load() -> Result<Option<MacroforgeConfig>> {
        Ok(Self::find_with_root()?.map(|(cfg, _)| cfg))
    }
}
