use std::env;
use zed_extension_api::{self as zed, serde_json};

const TS_PLUGIN: &str = "@macroforge/typescript-plugin";
const TS_PLUGIN_VERSION: &str = "0.1.0";

struct MacroforgesTsExtension;

impl MacroforgesTsExtension {
    /// Ensure the plugin is installed and return the path to it
    fn ensure_plugin_installed() -> Result<String, String> {
        // Check if already installed
        let installed = zed::npm_package_installed_version(TS_PLUGIN)?;

        if installed.is_none() {
            // Install the plugin
            zed::npm_install_package(TS_PLUGIN, TS_PLUGIN_VERSION)?;
        }

        // Get the extension's working directory where npm packages are installed
        let ext_dir = env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?;

        let plugin_dir = ext_dir.join("node_modules").join(TS_PLUGIN);

        plugin_dir
            .to_str()
            .map(|s| s.to_owned())
            .ok_or_else(|| "Plugin path is not valid UTF-8".to_string())
    }

    fn plugin_config(plugin_dir: &str) -> serde_json::Value {
        let plugin_entry = format!("{}/dist/index.js", plugin_dir);

        // vtsls accepts typescript.tsserver for plugin configuration
        serde_json::json!({
            "typescript": {
                "tsserver": {
                    "allowLocalPluginLoads": true,
                    "pluginProbeLocations": [plugin_dir],
                    "plugins": [{
                        "name": &plugin_entry
                    }],
                    "globalPlugins": [{
                        "name": &plugin_entry,
                        "enableForWorkspaceTypeScriptVersions": true,
                        "languages": [
                            "typescript",
                            "typescriptreact",
                            "javascript",
                            "javascriptreact",
                            "vue",
                            "svelte"
                        ]
                    }]
                }
            }
        })
    }
}

impl zed::Extension for MacroforgesTsExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_additional_initialization_options(
        &mut self,
        _: &zed::LanguageServerId,
        target_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<Option<serde_json::Value>> {
        // Inject plugin configuration for vtsls (and typescript-language-server)
        if target_id.as_ref() == "vtsls" || target_id.as_ref() == "typescript-language-server" {
            // Ensure the plugin is installed and get its path
            let plugin_dir = Self::ensure_plugin_installed()?;
            return Ok(Some(Self::plugin_config(&plugin_dir)));
        }

        Ok(None)
    }
}

zed::register_extension!(MacroforgesTsExtension);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_can_be_instantiated() {
        let _ext = MacroforgesTsExtension;
    }

    #[test]
    fn test_plugin_config_structure() {
        let plugin_dir = "/ext/node_modules/@macroforge/typescript-plugin";
        let config = MacroforgesTsExtension::plugin_config(plugin_dir);

        // Verify typescript.tsserver config structure
        let typescript = config.get("typescript").expect("typescript config should exist");
        let tsserver = typescript.get("tsserver").expect("tsserver config should exist");

        // Check allowLocalPluginLoads
        assert_eq!(tsserver.get("allowLocalPluginLoads").unwrap(), true);

        // Check pluginProbeLocations
        let probe_locations = tsserver.get("pluginProbeLocations").unwrap().as_array().unwrap();
        assert_eq!(probe_locations.len(), 1);
        assert_eq!(probe_locations[0], plugin_dir);

        // Check plugins
        let plugins = tsserver.get("plugins").unwrap().as_array().unwrap();
        assert_eq!(plugins.len(), 1);
        assert!(plugins[0].get("name").unwrap().as_str().unwrap().ends_with("/dist/index.js"));

        // Check globalPlugins
        let global_plugins = tsserver.get("globalPlugins").unwrap().as_array().unwrap();
        assert_eq!(global_plugins.len(), 1);

        let plugin = &global_plugins[0];
        assert!(plugin.get("name").unwrap().as_str().unwrap().ends_with("/dist/index.js"));
        assert_eq!(plugin.get("enableForWorkspaceTypeScriptVersions").unwrap(), true);

        let languages = plugin.get("languages").unwrap().as_array().unwrap();
        assert!(languages.contains(&serde_json::json!("typescript")));
        assert!(languages.contains(&serde_json::json!("typescriptreact")));
        assert!(languages.contains(&serde_json::json!("javascript")));
        assert!(languages.contains(&serde_json::json!("javascriptreact")));
        assert!(languages.contains(&serde_json::json!("vue")));
        assert!(languages.contains(&serde_json::json!("svelte")));
    }

    #[test]
    fn test_ts_plugin_constant() {
        assert_eq!(TS_PLUGIN, "@macroforge/typescript-plugin");
    }

    #[test]
    fn test_ts_plugin_version() {
        assert_eq!(TS_PLUGIN_VERSION, "0.1.0");
    }
}
