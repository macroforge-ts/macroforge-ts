use serde_json::json;
use std::path::{Path, PathBuf};
use zed_extension_api as zed;

const VTSLS_BIN: &str = "node_modules/@vtsls/language-server/bin/vtsls.js";
const PLUGIN_NAME: &str = "@macroforge/ts-derive-plugin";

struct MacroforgesExtension;

impl MacroforgesExtension {
    fn ensure_vtsls(worktree: &zed::Worktree) -> zed::Result<String> {
        let path = PathBuf::from(worktree.root_path()).join(VTSLS_BIN);
        if !path.exists() {
            return Err(format!(
                "vtsls binary not found at {}. Run `npm install` in the repository root.",
                path.display()
            ));
        }

        Self::path_to_string(&path)
    }

    fn workspace_config(worktree: &zed::Worktree) -> serde_json::Value {
        let plugin_path = format!("{}/packages/ts-derive-plugin", worktree.root_path());
        json!({
            "typescript": {
                "tsserver": {
                    "pluginPaths": [plugin_path]
                }
            },
            "vtsls": {
                "tsserver": {
                    "globalPlugins": [
                        {
                            "name": PLUGIN_NAME,
                            "location": plugin_path,
                            "languages": ["typescript", "typescriptreact"],
                            "enableForWorkspaceTypeScriptVersions": true
                        }
                    ]
                }
            }
        })
    }

    fn path_to_string(path: &Path) -> zed::Result<String> {
        path.to_str()
            .map(|s| s.to_owned())
            .ok_or_else(|| format!("Path {} is not valid UTF-8", path.display()))
    }
}

impl zed::Extension for MacroforgesExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let vtsls = Self::ensure_vtsls(worktree)?;
        Ok(zed::Command {
            command: "node".to_string(),
            args: vec![vtsls, "--stdio".to_string()],
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<serde_json::Value>> {
        Ok(Some(Self::workspace_config(worktree)))
    }
}

zed::register_extension!(MacroforgesExtension);
