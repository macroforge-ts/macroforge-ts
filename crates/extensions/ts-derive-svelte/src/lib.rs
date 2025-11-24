use std::{
    env,
    path::{Path, PathBuf},
};
use zed_extension_api as zed;

const EXTENSION_ID: &str = "ts-derive-svelte";
const LANGUAGE_SERVER_RELATIVE_PATH: &str = "packages/language-server";
const NODE_MODULES_PATH: &str = "node_modules/svelte-language-server";
const LANGUAGE_SERVER_BIN_RELATIVE: &str = "bin/server.js";
const LANGUAGE_SERVER_BUILD_ARTIFACT: &str = "dist/src/server.js";

struct TsDeriveSvelteExtension;

impl TsDeriveSvelteExtension {
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

impl zed::Extension for TsDeriveSvelteExtension {
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

zed::register_extension!(TsDeriveSvelteExtension);
