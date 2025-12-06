use std::{
    env,
    path::{Path, PathBuf},
};
use zed_extension_api as zed;

const EXTENSION_ID: &str = "svelte-macroforge";
const LANGUAGE_SERVER_RELATIVE_PATH: &str = "packages/svelte-language-server-macroforge";
const NODE_MODULES_PATH: &str = "node_modules/@macroforge/svelte-language-server";
const LANGUAGE_SERVER_BIN_RELATIVE: &str = "bin/server.js";
const LANGUAGE_SERVER_BUILD_ARTIFACT: &str = "dist/src/server.js";

struct SvelteMacroforgeExtension;

impl SvelteMacroforgeExtension {
    fn fallback_roots() -> Vec<PathBuf> {
        let mut roots = Vec::new();

        if let Ok(current_dir) = env::current_dir() {
            roots.push(current_dir.clone());

            if let Some(extensions_dir) = current_dir.parent().and_then(|work| work.parent()) {
                roots.push(extensions_dir.join("installed").join(EXTENSION_ID));
            }
        }

        roots.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")));

        roots
    }

    fn worktree_file_exists(worktree: &zed::Worktree, relative: &Path) -> bool {
        let relative_str = relative
            .components()
            .map(|comp| comp.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("/");
        worktree.read_text_file(&relative_str).is_ok()
    }

    fn language_server_dir(worktree: &zed::Worktree) -> zed::Result<PathBuf> {
        let workspace_root = PathBuf::from(worktree.root_path());
        let workspace_candidates = [
            (
                workspace_root.join(NODE_MODULES_PATH),
                PathBuf::from(NODE_MODULES_PATH).join("package.json"),
            ),
            (
                workspace_root.join(LANGUAGE_SERVER_RELATIVE_PATH),
                PathBuf::from(LANGUAGE_SERVER_RELATIVE_PATH).join("package.json"),
            ),
        ];

        let mut tried: Vec<String> = Vec::new();

        for (candidate, check) in &workspace_candidates {
            tried.push(candidate.display().to_string());
            if Self::worktree_file_exists(worktree, check) {
                return Ok(candidate.clone());
            }
        }

        for root in Self::fallback_roots() {
            let node_modules_candidate = root.join(NODE_MODULES_PATH);
            tried.push(node_modules_candidate.display().to_string());
            if node_modules_candidate.exists() {
                return Ok(node_modules_candidate);
            }

            let packages_candidate = root.join(LANGUAGE_SERVER_RELATIVE_PATH);
            tried.push(packages_candidate.display().to_string());
            if packages_candidate.exists() {
                return Ok(packages_candidate);
            }
        }

        Err(format!(
            "Svelte language server package not found. Tried: {}",
            tried.join(", ")
        ))
    }

    fn language_server_bin(language_server_dir: &Path) -> PathBuf {
        language_server_dir.join(LANGUAGE_SERVER_BIN_RELATIVE)
    }

    fn language_server_ready(language_server_dir: &Path, workspace_root: &Path) -> bool {
        if language_server_dir.starts_with(workspace_root) {
            // Assume ready; filesystem checks aren't available from Wasm for workspace paths.
            return true;
        }

        let bin = language_server_dir.join(LANGUAGE_SERVER_BIN_RELATIVE);
        let dist_entry = language_server_dir.join(LANGUAGE_SERVER_BUILD_ARTIFACT);
        bin.exists() && dist_entry.exists()
    }

    fn ensure_language_server(
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<String> {
        let workspace_root = PathBuf::from(worktree.root_path());
        let language_server_dir = Self::language_server_dir(worktree)?;
        let server_path = Self::language_server_bin(&language_server_dir);
        if Self::language_server_ready(&language_server_dir, &workspace_root) {
            return Self::path_to_string(&server_path);
        }

        let instruction = if language_server_dir.ends_with(NODE_MODULES_PATH) {
            "Run `npm install` in the repository root to install svelte-language-server."
        } else {
            "Run `npm install --workspace packages/language-server && npm run build --workspace packages/language-server`."
        };

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Failed(format!(
                "Missing build artifacts at {}. {}",
                language_server_dir
                    .join(LANGUAGE_SERVER_BUILD_ARTIFACT)
                    .display(),
                instruction
            )),
        );
        Err("svelte-language-server build artifacts are missing".into())
    }

    fn path_to_string(path: &Path) -> zed::Result<String> {
        path.to_str()
            .map(|s| s.to_owned())
            .ok_or_else(|| format!("Path {} is not valid UTF-8", path.display()))
    }
}

impl zed::Extension for SvelteMacroforgeExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let server = Self::ensure_language_server(language_server_id, worktree)?;

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![server, "--stdio".into()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(SvelteMacroforgeExtension);

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_extension_can_be_instantiated() {
        let _ext = SvelteMacroforgeExtension;
    }

    #[test]
    fn test_extension_id_constant() {
        assert_eq!(EXTENSION_ID, "svelte-macroforge");
    }

    #[test]
    fn test_language_server_relative_path_constant() {
        assert_eq!(LANGUAGE_SERVER_RELATIVE_PATH, "packages/svelte-language-server-macroforge");
    }

    #[test]
    fn test_node_modules_path_constant() {
        assert_eq!(NODE_MODULES_PATH, "node_modules/@macroforge/svelte-language-server");
    }

    #[test]
    fn test_language_server_bin_relative_constant() {
        assert_eq!(LANGUAGE_SERVER_BIN_RELATIVE, "bin/server.js");
    }

    #[test]
    fn test_language_server_build_artifact_constant() {
        assert_eq!(LANGUAGE_SERVER_BUILD_ARTIFACT, "dist/src/server.js");
    }

    #[test]
    fn test_fallback_roots_returns_non_empty() {
        let roots = SvelteMacroforgeExtension::fallback_roots();
        // Should always have at least one entry (CARGO_MANIFEST_DIR)
        assert!(
            !roots.is_empty(),
            "fallback_roots should return at least one path"
        );
    }

    #[test]
    fn test_fallback_roots_contains_cargo_manifest_dir() {
        let roots = SvelteMacroforgeExtension::fallback_roots();
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        assert!(
            roots.contains(&manifest_dir),
            "fallback_roots should contain CARGO_MANIFEST_DIR: {:?}",
            manifest_dir
        );
    }

    #[test]
    fn test_language_server_bin_path() {
        let server_dir = Path::new("/workspace/node_modules/@macroforge/svelte-language-server");
        let bin_path = SvelteMacroforgeExtension::language_server_bin(server_dir);
        assert_eq!(
            bin_path,
            PathBuf::from("/workspace/node_modules/@macroforge/svelte-language-server/bin/server.js")
        );
    }

    #[test]
    fn test_language_server_bin_with_packages_path() {
        let server_dir = Path::new("/workspace/packages/svelte-language-server-macroforge");
        let bin_path = SvelteMacroforgeExtension::language_server_bin(server_dir);
        assert_eq!(
            bin_path,
            PathBuf::from("/workspace/packages/svelte-language-server-macroforge/bin/server.js")
        );
    }

    #[test]
    fn test_language_server_ready_within_workspace() {
        // When server dir is within workspace, it should return true (assume ready)
        let workspace_root = Path::new("/workspace");
        let server_dir = Path::new("/workspace/node_modules/@macroforge/svelte-language-server");

        let ready = SvelteMacroforgeExtension::language_server_ready(server_dir, workspace_root);
        assert!(ready, "Should be ready when server is within workspace");
    }

    #[test]
    fn test_language_server_ready_outside_workspace_not_installed() {
        // When outside workspace and files don't exist, should return false
        let workspace_root = Path::new("/workspace");
        let server_dir = Path::new("/other/path/svelte-language-server");

        let ready = SvelteMacroforgeExtension::language_server_ready(server_dir, workspace_root);
        // Since /other/path doesn't exist, this should be false
        assert!(
            !ready,
            "Should not be ready when server is outside workspace and not installed"
        );
    }

    #[test]
    fn test_path_to_string_valid_utf8() {
        let path = Path::new("/valid/utf8/path");
        let result = SvelteMacroforgeExtension::path_to_string(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "/valid/utf8/path");
    }

    #[test]
    fn test_path_to_string_with_spaces() {
        let path = Path::new("/path/with spaces/file.js");
        let result = SvelteMacroforgeExtension::path_to_string(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "/path/with spaces/file.js");
    }

    #[test]
    fn test_path_to_string_with_unicode() {
        let path = Path::new("/path/with/ünïcödé/file.js");
        let result = SvelteMacroforgeExtension::path_to_string(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "/path/with/ünïcödé/file.js");
    }

    #[test]
    fn test_expected_binary_locations() {
        // Test that we correctly construct expected server binary paths
        let node_modules_dir = PathBuf::from("/workspace").join(NODE_MODULES_PATH);
        let packages_dir = PathBuf::from("/workspace").join(LANGUAGE_SERVER_RELATIVE_PATH);

        let node_modules_bin = SvelteMacroforgeExtension::language_server_bin(&node_modules_dir);
        let packages_bin = SvelteMacroforgeExtension::language_server_bin(&packages_dir);

        assert_eq!(
            node_modules_bin,
            PathBuf::from("/workspace/node_modules/@macroforge/svelte-language-server/bin/server.js")
        );
        assert_eq!(
            packages_bin,
            PathBuf::from("/workspace/packages/svelte-language-server-macroforge/bin/server.js")
        );
    }

    #[test]
    fn test_build_artifact_path() {
        let server_dir = Path::new("/workspace/packages/svelte-language-server-macroforge");
        let artifact_path = server_dir.join(LANGUAGE_SERVER_BUILD_ARTIFACT);
        assert_eq!(
            artifact_path,
            PathBuf::from("/workspace/packages/svelte-language-server-macroforge/dist/src/server.js")
        );
    }
}
