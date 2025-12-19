use std::env;
use zed_extension_api::{self as zed, Command, LanguageServerId, Result, Worktree, serde_json};

const TS_PLUGIN: &str = "@macroforge/typescript-plugin";
const TS_PLUGIN_VERSION: &str = "0.1.46";
const VTSLS_PACKAGE: &str = "@vtsls/language-server";
const VTSLS_VERSION: &str = "0.2.6";
const MACROFORGE_VERSION: &str = "0.1.46";

struct VtslsMacroforgeExtension {
    cached_vtsls_path: Option<String>,
    cached_plugin_path: Option<String>,
}

impl VtslsMacroforgeExtension {
    /// Check if installed version matches expected, reinstall if outdated
    fn ensure_package_version(package: &str, expected_version: &str) -> Result<()> {
        let installed = zed::npm_package_installed_version(package)?;
        match installed {
            Some(version) if version == expected_version => {
                // Already at correct version
                Ok(())
            }
            Some(_) | None => {
                // Outdated or not installed - install expected version
                zed::npm_install_package(package, expected_version)?;
                Ok(())
            }
        }
    }

    /// Ensure vtsls is installed and return the path to the binary
    fn ensure_vtsls_installed(&mut self) -> Result<String> {
        if let Some(path) = &self.cached_vtsls_path {
            return Ok(path.clone());
        }

        Self::ensure_package_version(VTSLS_PACKAGE, VTSLS_VERSION)?;

        let ext_dir =
            env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;

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
            (zed::Os::Mac, zed::Architecture::X8664 | zed::Architecture::X86) => {
                "@macroforge/bin-darwin-x64"
            }
            (zed::Os::Mac, zed::Architecture::Aarch64) => "@macroforge/bin-darwin-arm64",
            (zed::Os::Linux, zed::Architecture::X8664 | zed::Architecture::X86) => {
                "@macroforge/bin-linux-x64-gnu"
            }
            (zed::Os::Linux, zed::Architecture::Aarch64) => "@macroforge/bin-linux-arm64-gnu",
            (zed::Os::Windows, zed::Architecture::X8664 | zed::Architecture::X86) => {
                "@macroforge/bin-win32-x64-msvc"
            }
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
        Self::ensure_package_version(binary_package, MACROFORGE_VERSION)?;

        // Install the typescript plugin (which depends on macroforge)
        Self::ensure_package_version(TS_PLUGIN, TS_PLUGIN_VERSION)?;

        let ext_dir =
            env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;

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

impl zed::Extension for VtslsMacroforgeExtension {
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
        if language_server_id.as_ref() != "vtsls-macroforge" {
            return Err(format!(
                "Unknown language server: {}",
                language_server_id.as_ref()
            ));
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
        if language_server_id.as_ref() != "vtsls-macroforge" {
            return Ok(None);
        }

        let plugin_location = self.ensure_plugin_installed()?;

        let init_options = serde_json::json!({
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

        Ok(Some(init_options))
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        if language_server_id.as_ref() != "vtsls-macroforge" {
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

zed::register_extension!(VtslsMacroforgeExtension);

#[cfg(test)]
mod tests {
    use super::*;
    use zed_extension_api::Extension;

    #[test]
    fn test_extension_can_be_instantiated() {
        let _ext = VtslsMacroforgeExtension::new();
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
