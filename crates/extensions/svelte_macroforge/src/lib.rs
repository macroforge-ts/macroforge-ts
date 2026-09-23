//! Zed extension wrapping the macroforge-aware Svelte language server.
//!
//! Provides the `svelte-macroforge` language server (see `extension.toml`),
//! which runs `@macroforge/svelte-language-server` over stdio via Zed's
//! bundled Node.js so Svelte files get macroforge macro expansion in the
//! editor.
//!
//! On first use, the extension installs pinned npm packages into its own
//! directory:
//!
//! - `@macroforge/svelte-language-server` at `SVELTE_LS_VERSION`
//! - `@macroforge/core` at the same version, installed at the root so the
//!   language server resolves it
//!
//! Outdated installs are detected by version mismatch and reinstalled. The
//! server is started from the `svelteserver` command the package declares in
//! its `bin` field, so the package's internal layout is not assumed here.

use std::collections::HashMap;
use std::env;
use zed_extension_api::{self as zed, Command, LanguageServerId, Result, Worktree};

const SVELTE_LS_PACKAGE: &str = "@macroforge/svelte-language-server";
const SVELTE_LS_VERSION: &str = "0.3.1";
const SVELTE_LS_COMMAND: &str = "svelteserver";
const MACROFORGE_PACKAGE: &str = "@macroforge/core";

/// The part of an installed package's `package.json` naming its commands.
#[derive(serde::Deserialize)]
struct PackageManifest {
    bin: HashMap<String, String>,
}

struct SvelteMacroforgeExtension {
    cached_server_path: Option<String>,
}

impl SvelteMacroforgeExtension {
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

    /// Ensure the svelte language server is installed and return the path to the binary
    fn ensure_server_installed(&mut self) -> Result<String> {
        if let Some(path) = &self.cached_server_path {
            return Ok(path.clone());
        }

        // Installed at the root so the language server resolves it rather than
        // picking up a transitive copy.
        Self::ensure_package_version(MACROFORGE_PACKAGE, SVELTE_LS_VERSION)?;

        // Install the svelte language server (which depends on macroforge + typescript-plugin)
        Self::ensure_package_version(SVELTE_LS_PACKAGE, SVELTE_LS_VERSION)?;

        let ext_dir =
            env::current_dir().map_err(|err| format!("Failed to get current directory: {err}"))?;

        let package_dir = ext_dir
            .join("node_modules")
            .join("@macroforge")
            .join("svelte-language-server");
        let manifest_path = package_dir.join("package.json");
        let manifest_text = std::fs::read_to_string(&manifest_path)
            .map_err(|err| format!("Failed to read {}: {err}", manifest_path.display()))?;
        let manifest: PackageManifest = serde_json::from_str(&manifest_text)
            .map_err(|err| format!("Failed to parse {}: {err}", manifest_path.display()))?;
        let command = manifest.bin.get(SVELTE_LS_COMMAND).ok_or_else(|| {
            format!("{SVELTE_LS_PACKAGE} declares no `{SVELTE_LS_COMMAND}` command")
        })?;
        let server_binary = package_dir.join(command);

        let path = server_binary
            .to_str()
            .ok_or_else(|| "Server path is not valid UTF-8".to_string())?
            .to_owned();

        self.cached_server_path = Some(path.clone());
        Ok(path)
    }
}

impl zed::Extension for SvelteMacroforgeExtension {
    fn new() -> Self {
        Self {
            cached_server_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Command> {
        if language_server_id.as_ref() != "svelte-macroforge" {
            return Err(format!(
                "Unknown language server: {}",
                language_server_id.as_ref()
            ));
        }

        let server_path = self.ensure_server_installed()?;

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![server_path, "--stdio".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(SvelteMacroforgeExtension);

#[cfg(test)]
mod tests {
    use super::*;
    use zed_extension_api::Extension;

    #[test]
    fn test_extension_can_be_instantiated() {
        let _ext = SvelteMacroforgeExtension::new();
    }

    #[test]
    fn test_svelte_ls_package_constant() {
        assert_eq!(SVELTE_LS_PACKAGE, "@macroforge/svelte-language-server");
    }

    #[test]
    fn test_svelte_ls_version_constant() {
        assert_eq!(SVELTE_LS_VERSION, "0.3.1");
    }
}
