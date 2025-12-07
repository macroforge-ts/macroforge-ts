use std::env;
use zed_extension_api::{self as zed, serde_json, Command, LanguageServerId, Result, Worktree};

const TS_PLUGIN: &str = "@macroforge/typescript-plugin";
const TS_PLUGIN_VERSION: &str = "0.1.1";
const VTSLS_PACKAGE: &str = "@vtsls/language-server";
const VTSLS_VERSION: &str = "0.2.6";
const MACROFORGE_VERSION: &str = "0.1.1";

struct MacroforgeExtension {
    cached_vtsls_path: Option<String>,
    cached_plugin_path: Option<String>,
}

impl MacroforgeExtension {
    /// Ensure vtsls is installed and return the path to the binary
    fn ensure_vtsls_installed(&mut self) -> Result<String> {
        if let Some(path) = &self.cached_vtsls_path {
            return Ok(path.clone());
        }

        let installed = zed::npm_package_installed_version(VTSLS_PACKAGE)?;
        if installed.is_none() {
            zed::npm_install_package(VTSLS_PACKAGE, VTSLS_VERSION)?;
        }

        let ext_dir = env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?;

        let vtsls_binary = ext_dir
            .join("node_modules")
            .join("@vtsls")
            .join("language-server")
            .join("bin")
            .join("vtsls.js");

        let path = vtsls_binary
            .to_str()
            .ok_or_else(|| "VTSLS path is not valid UTF-8".to_string())?
            .to_owned();

        self.cached_vtsls_path = Some(path.clone());
        Ok(path)
    }

    /// Get the platform-specific binary package name based on Zed's platform info
    fn get_binary_package() -> &'static str {
        // Get platform from Zed's API
        let (os, arch) = zed::current_platform();

        match (os, arch) {
            (zed::Os::Mac, zed::Architecture::X8664 | zed::Architecture::X86) => "@macroforge/bin-darwin-x64",
            (zed::Os::Mac, zed::Architecture::Aarch64) => "@macroforge/bin-darwin-arm64",
            (zed::Os::Linux, zed::Architecture::X8664 | zed::Architecture::X86) => "@macroforge/bin-linux-x64-gnu",
            (zed::Os::Linux, zed::Architecture::Aarch64) => "@macroforge/bin-linux-arm64-gnu",
            (zed::Os::Windows, zed::Architecture::X8664 | zed::Architecture::X86) => "@macroforge/bin-win32-x64-msvc",
            (zed::Os::Windows, zed::Architecture::Aarch64) => "@macroforge/bin-win32-arm64-msvc",
        }
    }

    /// Ensure the TypeScript plugin is installed and return the path
    fn ensure_plugin_installed(&mut self) -> Result<String> {
        if let Some(path) = &self.cached_plugin_path {
            return Ok(path.clone());
        }

        // Install the binary package first (needed by macroforge)
        let binary_package = Self::get_binary_package();
        let binary_installed = zed::npm_package_installed_version(binary_package)?;
        if binary_installed.is_none() {
            zed::npm_install_package(binary_package, MACROFORGE_VERSION)?;
        }

        // Install the typescript plugin (which depends on macroforge)
        let installed = zed::npm_package_installed_version(TS_PLUGIN)?;
        if installed.is_none() {
            zed::npm_install_package(TS_PLUGIN, TS_PLUGIN_VERSION)?;
        }

        let ext_dir = env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?;

        // Return the node_modules directory - vtsls will resolve the package from here
        let node_modules_dir = ext_dir.join("node_modules");

        let path = node_modules_dir
            .to_str()
            .ok_or_else(|| "Plugin path is not valid UTF-8".to_string())?
            .to_owned();

        self.cached_plugin_path = Some(path.clone());
        Ok(path)
    }
}

impl zed::Extension for MacroforgeExtension {
    fn new() -> Self {
        Self {
            cached_vtsls_path: None,
            cached_plugin_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Command> {
        // Debug: write to file to confirm this function is called
        if let Ok(dir) = env::current_dir() {
            let debug_file = dir.join("command_called.txt");
            let _ = std::fs::write(&debug_file, format!(
                "language_server_command called for: {}\nTime: {:?}\n",
                language_server_id.as_ref(),
                std::time::SystemTime::now()
            ));
        }

        if language_server_id.as_ref() != "macroforge-ts" {
            return Err(format!("Unknown language server: {}", language_server_id.as_ref()));
        }

        let vtsls_path = self.ensure_vtsls_installed()?;

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![vtsls_path, "--stdio".to_string()],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        // Debug: write to a file to confirm this function is called
        let ext_dir = env::current_dir().ok();
        if let Some(dir) = &ext_dir {
            let debug_file = dir.join("init_options_called.txt");
            let _ = std::fs::write(&debug_file, format!(
                "Called for: {}\nTime: {:?}\n",
                language_server_id.as_ref(),
                std::time::SystemTime::now()
            ));
        }

        if language_server_id.as_ref() != "macroforge-ts" {
            return Ok(None);
        }

        let plugin_location = self.ensure_plugin_installed()?;

        // Get the log directory path (in extension work dir)
        let ext_dir = env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?;
        let log_dir = ext_dir.join("tsserver-logs");
        let log_dir_str = log_dir.to_str().unwrap_or("/tmp/tsserver-logs");

        let init_options = serde_json::json!({
            "typescript": {
                "tsserver": {
                    "logDirectory": log_dir_str,
                    "logVerbosity": "verbose"
                }
            },
            "vtsls": {
                "tsserver": {
                    "globalPlugins": [{
                        "name": TS_PLUGIN,
                        "location": plugin_location,
                        "enableForWorkspaceTypeScriptVersions": true,
                        "languages": [
                            "typescript",
                            "typescriptreact",
                            "javascript",
                            "javascriptreact",
                            "svelte"
                        ]
                    }]
                }
            }
        });

        // Debug: write the init options to a file
        let debug_file = ext_dir.join("init_options_value.json");
        let _ = std::fs::write(&debug_file, serde_json::to_string_pretty(&init_options).unwrap_or_default());

        Ok(Some(init_options))
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        // vtsls might need settings via workspace configuration too
        if language_server_id.as_ref() != "macroforge-ts" {
            return Ok(None);
        }

        let plugin_location = self.ensure_plugin_installed()?;

        Ok(Some(serde_json::json!({
            "vtsls": {
                "tsserver": {
                    "globalPlugins": [{
                        "name": TS_PLUGIN,
                        "location": plugin_location,
                        "enableForWorkspaceTypeScriptVersions": true,
                        "languages": [
                            "typescript",
                            "typescriptreact",
                            "javascript",
                            "javascriptreact",
                            "svelte"
                        ]
                    }]
                }
            }
        })))
    }
}

zed::register_extension!(MacroforgeExtension);

#[cfg(test)]
mod tests {
    use super::*;
    use zed_extension_api::Extension;

    #[test]
    fn test_extension_can_be_instantiated() {
        let _ext = MacroforgeExtension::new();
    }

    #[test]
    fn test_ts_plugin_constant() {
        assert_eq!(TS_PLUGIN, "@macroforge/typescript-plugin");
    }

    #[test]
    fn test_vtsls_package_constant() {
        assert_eq!(VTSLS_PACKAGE, "@vtsls/language-server");
    }
}
