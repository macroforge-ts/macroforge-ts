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
