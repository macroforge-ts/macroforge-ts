use std::path::{Path, PathBuf};
use zed_extension_api as zed;

const LANGUAGE_TOOLS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/language-tools");
const LANGUAGE_SERVER_BIN: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/language-tools/packages/language-server/bin/server.js");
const LANGUAGE_SERVER_WORKSPACE: &str = "svelte-language-server";

struct TsDeriveSvelteExtension;

impl TsDeriveSvelteExtension {
    fn ensure_language_server(
        language_server_id: &zed::LanguageServerId,
    ) -> zed::Result<String> {
        let server_path = PathBuf::from(LANGUAGE_SERVER_BIN);
        if server_path.exists() {
            return Self::path_to_string(&server_path);
        }

        let repo_path = PathBuf::from(LANGUAGE_TOOLS_DIR);
        if !repo_path.exists() {
            return Err(format!(
                "language-tools repo not found at {}",
                repo_path.display()
            ));
        }

        let prefix = Self::path_to_string(&repo_path)?;
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );
        Self::run_npm(&["install", "--workspaces", "--prefix", &prefix])?;
        Self::run_npm(&[
            "run",
            "build",
            "--workspace",
            LANGUAGE_SERVER_WORKSPACE,
            "--prefix",
            &prefix,
        ])?;

        if !server_path.exists() {
            return Err(format!(
                "Expected {} after building svelte-language-server",
                server_path.display()
            ));
        }

        Self::path_to_string(&server_path)
    }

    fn run_npm(args: &[&str]) -> zed::Result<()> {
        let mut command = zed::process::Command::new("npm");
        command = command.args(args.iter().map(|arg| arg.to_string()));
        let output = command.output().map_err(|err| {
            format!("Failed to run npm {}: {err}", args.join(" "))
        })?;

        if output.status != Some(0) {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "npm {} failed with exit code {:?}: {}",
                args.join(" "),
                output.status,
                stderr
            ));
        }

        Ok(())
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
        _: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let server = Self::ensure_language_server(language_server_id)?;

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![server, "--stdio".into()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(TsDeriveSvelteExtension);
