use std::env;
use zed_extension_api::{self as zed, Command, LanguageServerId, Result, Worktree};

const SVELTE_LS_PACKAGE: &str = "@macroforge/svelte-language-server";
const SVELTE_LS_VERSION: &str = "0.1.4";

struct SvelteMacroforgeExtension {
    cached_server_path: Option<String>,
}

impl SvelteMacroforgeExtension {
    /// Get the platform-specific binary package name based on Zed's platform info
    fn get_binary_package() -> &'static str {
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

    /// Ensure the svelte language server is installed and return the path to the binary
    fn ensure_server_installed(&mut self) -> Result<String> {
        if let Some(path) = &self.cached_server_path {
            return Ok(path.clone());
        }

        // Install the binary package first (needed by macroforge)
        let binary_package = Self::get_binary_package();
        let binary_installed = zed::npm_package_installed_version(binary_package)?;
        if binary_installed.is_none() {
            zed::npm_install_package(binary_package, SVELTE_LS_VERSION)?;
        }

        // Install the svelte language server (which depends on macroforge + typescript-plugin)
        let installed = zed::npm_package_installed_version(SVELTE_LS_PACKAGE)?;
        if installed.is_none() {
            zed::npm_install_package(SVELTE_LS_PACKAGE, SVELTE_LS_VERSION)?;
        }

        let ext_dir =
            env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;

        let server_binary = ext_dir
            .join("node_modules")
            .join("@macroforge")
            .join("svelte-language-server")
            .join("bin")
            .join("server.js");

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
        assert_eq!(SVELTE_LS_VERSION, "0.1.3");
    }
}
