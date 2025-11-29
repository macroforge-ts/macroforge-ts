use zed_extension_api::{self as zed, serde_json};

const TS_PLUGIN: &str = "@ts-macros/ts-derive-plugin";

struct TsMacrosTsExtension;

impl TsMacrosTsExtension {
    fn plugin_config(worktree: &zed::Worktree) -> serde_json::Value {
        let plugin_path = format!("{}/packages/ts-derive-plugin", worktree.root_path());
        serde_json::json!({
            "vtsls": {
                "tsserver": {
                    "globalPlugins": [{
                        "name": TS_PLUGIN,
                        "location": plugin_path,
                        "enableForWorkspaceTypeScriptVersions": true,
                        "languages": [
                            "typescript",
                            "typescriptreact"
                        ]
                    }]
                }
            },
            "typescript": {
                "tsserver": {
                    "pluginPaths": [plugin_path]
                }
            }
        })
    }
}

impl zed::Extension for TsMacrosTsExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_additional_workspace_configuration(
        &mut self,
        _: &zed::LanguageServerId,
        target_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<serde_json::Value>> {
        if target_id.as_ref() == "vtsls" {
            return Ok(Some(Self::plugin_config(worktree)));
        }

        Ok(None)
    }
}

zed::register_extension!(TsMacrosTsExtension);

#[cfg(test)]
mod tests {
    use super::*;

    /// Generate plugin config using the same logic as the extension
    fn generate_plugin_config(root_path: &str) -> serde_json::Value {
        let plugin_path = format!("{}/packages/ts-derive-plugin", root_path);
        serde_json::json!({
            "vtsls": {
                "tsserver": {
                    "globalPlugins": [{
                        "name": TS_PLUGIN,
                        "location": plugin_path,
                        "enableForWorkspaceTypeScriptVersions": true,
                        "languages": [
                            "typescript",
                            "typescriptreact"
                        ]
                    }]
                }
            },
            "typescript": {
                "tsserver": {
                    "pluginPaths": [plugin_path]
                }
            }
        })
    }

    #[test]
    fn test_extension_can_be_instantiated() {
        let _ext = TsMacrosTsExtension;
    }

    #[test]
    fn test_plugin_config_structure() {
        let config = generate_plugin_config("/workspace/ts-macros");

        // Verify vtsls config structure
        let vtsls = config.get("vtsls").expect("vtsls config should exist");
        let tsserver = vtsls.get("tsserver").expect("tsserver config should exist");
        let plugins = tsserver
            .get("globalPlugins")
            .expect("globalPlugins should exist")
            .as_array()
            .expect("globalPlugins should be an array");

        assert_eq!(plugins.len(), 1);

        let plugin = &plugins[0];
        assert_eq!(plugin.get("name").unwrap(), TS_PLUGIN);
        assert_eq!(
            plugin.get("location").unwrap(),
            "/workspace/ts-macros/packages/ts-derive-plugin"
        );
        assert_eq!(plugin.get("enableForWorkspaceTypeScriptVersions").unwrap(), true);

        let languages = plugin
            .get("languages")
            .unwrap()
            .as_array()
            .expect("languages should be array");
        assert!(languages.contains(&serde_json::json!("typescript")));
        assert!(languages.contains(&serde_json::json!("typescriptreact")));
    }

    #[test]
    fn test_plugin_config_typescript_section() {
        let config = generate_plugin_config("/my/project");

        let typescript = config.get("typescript").expect("typescript config should exist");
        let tsserver = typescript.get("tsserver").expect("tsserver config should exist");
        let plugin_paths = tsserver
            .get("pluginPaths")
            .expect("pluginPaths should exist")
            .as_array()
            .expect("pluginPaths should be an array");

        assert_eq!(plugin_paths.len(), 1);
        assert_eq!(plugin_paths[0], "/my/project/packages/ts-derive-plugin");
    }

    #[test]
    fn test_plugin_config_with_different_roots() {
        let test_cases = vec![
            "/home/user/projects/ts-macros",
            "/Users/dev/code/ts-macros",
            "C:\\Users\\dev\\ts-macros",
            "/workspace",
        ];

        for root in test_cases {
            let config = generate_plugin_config(root);
            let expected_path = format!("{}/packages/ts-derive-plugin", root);

            let vtsls_plugin_location = config["vtsls"]["tsserver"]["globalPlugins"][0]["location"]
                .as_str()
                .unwrap();
            let ts_plugin_path = config["typescript"]["tsserver"]["pluginPaths"][0]
                .as_str()
                .unwrap();

            assert_eq!(vtsls_plugin_location, expected_path, "vtsls location mismatch for root: {}", root);
            assert_eq!(ts_plugin_path, expected_path, "typescript path mismatch for root: {}", root);
        }
    }

    #[test]
    fn test_ts_plugin_constant() {
        assert_eq!(TS_PLUGIN, "@ts-macros/ts-derive-plugin");
    }
}
