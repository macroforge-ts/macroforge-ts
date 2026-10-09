//! Shell command abstractions
//!
//! Provides a clean API for running external tools (cargo, deno, git).

use anyhow::{Context, Result};
use std::path::Path;
use std::process::{Command, Stdio};

/// Result of a shell command execution
pub struct CommandResult {
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

impl CommandResult {
    /// Everything the command printed, both streams in full, for reporting a
    /// failure. Stdout and stderr each carry part of the story, so neither is
    /// dropped and nothing is truncated.
    pub fn transcript(&self) -> String {
        match (self.stdout.trim().is_empty(), self.stderr.trim().is_empty()) {
            (true, true) => "(no output)".to_string(),
            (false, true) => self.stdout.trim_end().to_string(),
            (true, false) => self.stderr.trim_end().to_string(),
            (false, false) => format!(
                "stdout:\n{}\nstderr:\n{}",
                self.stdout.trim_end(),
                self.stderr.trim_end()
            ),
        }
    }
}

/// Shell command builder
pub struct Shell<'a> {
    program: &'a str,
    args: Vec<&'a str>,
    /// Arguments passed through but shown as `***` wherever the command is printed.
    secrets: Vec<&'a str>,
    envs: Vec<(String, String)>,
    cwd: Option<&'a Path>,
    inherit_stdio: bool,
}

impl<'a> Shell<'a> {
    /// Create a new shell command
    pub fn new(program: &'a str) -> Self {
        Self {
            program,
            args: Vec::new(),
            secrets: Vec::new(),
            envs: Vec::new(),
            cwd: None,
            inherit_stdio: false,
        }
    }

    /// Add arguments
    pub fn args(mut self, args: &[&'a str]) -> Self {
        self.args.extend(args);
        self
    }

    /// Add a single argument
    pub fn arg(mut self, arg: &'a str) -> Self {
        self.args.push(arg);
        self
    }

    /// Add an argument that never appears in printed command lines
    pub fn secret_arg(mut self, arg: &'a str) -> Self {
        self.args.push(arg);
        self.secrets.push(arg);
        self
    }

    /// The command line for messages, with secret arguments redacted
    fn display(&self) -> String {
        let args: Vec<&str> = self
            .args
            .iter()
            .map(|arg| {
                if self.secrets.contains(arg) {
                    "***"
                } else {
                    arg
                }
            })
            .collect();
        format!("{} {}", self.program, args.join(" "))
    }

    /// Add environment variables
    pub fn envs(mut self, envs: Vec<(String, String)>) -> Self {
        self.envs.extend(envs);
        self
    }

    /// Set working directory
    pub fn dir(mut self, cwd: &'a Path) -> Self {
        self.cwd = Some(cwd);
        self
    }

    /// Inherit stdio (for live output)
    pub fn inherit(mut self) -> Self {
        self.inherit_stdio = true;
        self
    }

    /// Run the command and return result
    pub fn run(self) -> Result<CommandResult> {
        let mut cmd = Command::new(self.program);
        cmd.args(&self.args);

        for (key, value) in &self.envs {
            cmd.env(key, value);
        }

        if let Some(cwd) = self.cwd {
            cmd.current_dir(cwd);
        }

        if self.inherit_stdio {
            cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
            let status = cmd
                .status()
                .with_context(|| format!("Failed to execute: {}", self.display()))?;
            Ok(CommandResult {
                success: status.success(),
                exit_code: status.code(),
                stdout: String::new(),
                stderr: String::new(),
            })
        } else {
            let output = cmd
                .output()
                .with_context(|| format!("Failed to execute: {}", self.display()))?;
            Ok(CommandResult {
                success: output.status.success(),
                exit_code: output.status.code(),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            })
        }
    }

    /// Run and return Ok only if successful
    pub fn run_checked(self) -> Result<CommandResult> {
        let command = self.display();
        let cwd = self.cwd.map(|p| p.to_path_buf());
        let inherited = self.inherit_stdio;

        let result = self.run()?;

        if result.success {
            Ok(result)
        } else {
            let cwd_str = cwd
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| ".".to_string());

            let output = if inherited {
                "(output shown above)".to_string()
            } else {
                result.transcript()
            };
            anyhow::bail!(
                "Command `{}` failed in {}\nExit code: {}\n{}",
                command,
                cwd_str,
                result
                    .exit_code
                    .map(|c| c.to_string())
                    .unwrap_or("unknown".to_string()),
                output
            )
        }
    }
}

// ============================================================================
// Cargo commands
// ============================================================================

pub mod cargo {
    use super::*;

    /// Format every crate of the workspace (or crate) at `cwd`.
    pub fn fmt(cwd: &Path) -> Result<CommandResult> {
        Shell::new("cargo")
            .args(&["fmt", "--all"])
            .dir(cwd)
            .run_checked()
    }

    /// Fail when any crate of the workspace (or crate) at `cwd` is unformatted.
    pub fn fmt_check(cwd: &Path) -> Result<CommandResult> {
        Shell::new("cargo")
            .args(&["fmt", "--all", "--check"])
            .dir(cwd)
            .run_checked()
    }

    /// Run cargo login (interactive)
    pub fn login() -> Result<()> {
        let status = Command::new("cargo")
            .arg("login")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        if status.success() {
            Ok(())
        } else {
            anyhow::bail!("cargo login failed")
        }
    }

    /// Debug-build one binary of a workspace package.
    pub fn build_bin(cwd: &Path, package: &str, bin: &str) -> Result<CommandResult> {
        Shell::new("cargo")
            .args(&["build", "-p", package, "--bin", bin])
            .dir(cwd)
            .run_checked()
    }

    /// Run cargo build with a specific target
    pub fn build_target(cwd: &Path, target: &str) -> Result<CommandResult> {
        Shell::new("cargo")
            .args(&["build", "--target", target, "--release"])
            .dir(cwd)
            .run_checked()
    }

    /// Clippy over every workspace member, target and feature, as JSON.
    pub fn clippy_workspace_json(cwd: &Path) -> Result<CommandResult> {
        Shell::new("cargo")
            .args(&[
                "clippy",
                "--workspace",
                "--all-targets",
                "--all-features",
                "--message-format=json",
            ])
            .dir(cwd)
            .run()
    }

    /// Run cargo clippy with JSON output for diagnostics parsing
    /// Apply clippy's machine-applicable fixes to the workspace at `cwd`.
    pub fn clippy_fix_workspace(cwd: &Path) -> Result<CommandResult> {
        Shell::new("cargo")
            .args(&[
                "clippy",
                "--fix",
                "--allow-dirty",
                "--allow-staged",
                "--workspace",
                "--all-targets",
                "--all-features",
            ])
            .dir(cwd)
            .inherit()
            .run_checked()
    }

    /// Apply clippy's machine-applicable fixes to the crate at `cwd`.
    pub fn clippy_fix(cwd: &Path) -> Result<CommandResult> {
        Shell::new("cargo")
            .args(&[
                "clippy",
                "--fix",
                "--allow-dirty",
                "--allow-staged",
                "--all-targets",
            ])
            .dir(cwd)
            .inherit()
            .run_checked()
    }

    pub fn clippy_json(cwd: &Path) -> Result<CommandResult> {
        Shell::new("cargo")
            .args(&["clippy", "--all-targets", "--message-format=json"])
            .dir(cwd)
            .run()
    }

    /// Rewrite Cargo.lock so it records the workspace members' versions as the
    /// manifests on disk state them. `cargo metadata` resolves the graph and
    /// rewrites the lock without building, downloading any locked crate the
    /// local cache lacks.
    pub fn sync_lock(cwd: &Path) -> Result<CommandResult> {
        Shell::new("cargo")
            .args(&["metadata", "--format-version", "1"])
            .dir(cwd)
            .run_checked()
    }

    /// Run every test of the workspace (or crate) at `cwd` via cargo-nextest.
    /// nextest executes each test in its own process, so process-global
    /// state (config caches, registries) cannot leak between tests.
    /// nextest does not run doctests, so a `cargo test --doc` pass follows,
    /// over the members whose library cargo can doctest.
    /// A crate without tests has nothing to fail, so it passes.
    /// `features` are enabled for the nextest pass only, since a test target
    /// can require a feature the doctests never need.
    pub fn test(cwd: &Path, features: &[&str]) -> Result<CommandResult> {
        let mut nextest_args = vec!["nextest", "run", "--workspace", "--no-tests=pass"];
        let joined = features.join(",");
        if !features.is_empty() {
            nextest_args.extend(["--features", joined.as_str()]);
        }
        Shell::new("cargo")
            .args(&nextest_args)
            .dir(cwd)
            .run_checked()?;
        let skipped = members_without_doctests(cwd)?;
        let mut doc_args = vec!["test", "--doc", "--workspace"];
        for member in &skipped {
            doc_args.extend(["--exclude", member.as_str()]);
        }
        Shell::new("cargo").args(&doc_args).dir(cwd).run_checked()
    }

    /// Workspace members whose library cargo cannot doctest: a `cdylib`-only
    /// library, such as a Zed extension, or none at all.
    fn members_without_doctests(cwd: &Path) -> Result<Vec<String>> {
        #[derive(serde::Deserialize)]
        struct Metadata {
            packages: Vec<Package>,
        }
        #[derive(serde::Deserialize)]
        struct Package {
            name: String,
            targets: Vec<Target>,
        }
        #[derive(serde::Deserialize)]
        struct Target {
            kind: Vec<String>,
        }

        let output = Shell::new("cargo")
            .args(&["metadata", "--format-version", "1", "--no-deps"])
            .dir(cwd)
            .run_checked()?;
        let metadata: Metadata =
            serde_json::from_str(&output.stdout).context("cargo metadata printed invalid JSON")?;
        Ok(metadata
            .packages
            .into_iter()
            .filter(|package| {
                !package.targets.iter().any(|target| {
                    target
                        .kind
                        .iter()
                        .any(|kind| matches!(kind.as_str(), "lib" | "rlib" | "proc-macro"))
                })
            })
            .map(|package| package.name)
            .collect())
    }
}

// ============================================================================
// deno commands
// ============================================================================

pub mod deno {
    use super::*;

    /// Run deno install --node-modules-dir
    pub fn install(cwd: &Path) -> Result<CommandResult> {
        Shell::new("deno")
            .args(&["install", "--node-modules-dir"])
            .dir(cwd)
            .run_checked()
    }

    /// Run deno task <task>
    pub fn task(cwd: &Path, task_name: &str) -> Result<CommandResult> {
        Shell::new("deno")
            .args(&["task", task_name])
            .dir(cwd)
            .run_checked()
    }

    /// Format `paths` under `cwd`, or everything when `paths` is empty.
    pub fn deno_fmt(cwd: &Path, paths: &[&str]) -> Result<CommandResult> {
        let mut shell = Shell::new("deno").arg("fmt");
        if paths.is_empty() {
            shell = shell.arg(".");
        } else {
            shell = shell.args(paths);
        }
        shell.dir(cwd).run_checked()
    }

    /// Format `text` as `deno fmt` would format a file with `extension` under
    /// `cwd`, so generated files are written in their canonical form.
    pub fn format(cwd: &Path, extension: &str, text: &str) -> Result<String> {
        use std::io::Write;

        let mut child = Command::new("deno")
            .args(["fmt", "--ext", extension, "-"])
            .current_dir(cwd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("failed to start deno fmt")?;
        child
            .stdin
            .take()
            .context("deno fmt has no stdin")?
            .write_all(text.as_bytes())
            .context("failed to send the text to deno fmt")?;
        let output = child
            .wait_with_output()
            .context("deno fmt did not finish")?;
        if !output.status.success() {
            anyhow::bail!(
                "deno fmt failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        String::from_utf8(output.stdout).context("deno fmt wrote non-UTF-8 output")
    }

    /// Fail when anything under `cwd` is unformatted.
    pub fn deno_fmt_check(cwd: &Path) -> Result<CommandResult> {
        Shell::new("deno")
            .args(&["fmt", "--check", "."])
            .dir(cwd)
            .run_checked()
    }

    /// Lint everything under `cwd` as JSON.
    pub fn lint_json(cwd: &Path, fix: bool) -> Result<CommandResult> {
        let mut shell = Shell::new("deno").args(&["lint", "--json"]);
        if fix {
            shell = shell.arg("--fix");
        }
        shell.arg(".").dir(cwd).run()
    }

    /// Run deno task with live output
    pub fn task_inherit(cwd: &Path, task_name: &str) -> Result<CommandResult> {
        Shell::new("deno")
            .args(&["task", task_name])
            .dir(cwd)
            .inherit()
            .run_checked()
    }

    /// Generate a SvelteKit project's `.svelte-kit/` types and tsconfig, with
    /// the project's own `svelte-kit` resolved as its tasks resolve it.
    pub fn svelte_kit_sync(cwd: &Path) -> Result<CommandResult> {
        Shell::new("deno")
            .args(&["task", "--eval", "svelte-kit sync"])
            .dir(cwd)
            .run_checked()
    }

    /// Run deno publish to JSR with live output (auto-discovers deno.json).
    /// Without a token, deno authenticates interactively in the browser.
    pub fn publish(cwd: &Path, token: Option<&str>) -> Result<CommandResult> {
        let mut shell = Shell::new("deno").args(&["publish", "--allow-dirty"]);
        if let Some(token) = token {
            shell = shell.arg("--token").secret_arg(token);
        }
        shell.dir(cwd).inherit().run_checked()
    }

    /// Dry-run a JSR publish of every workspace member. This is the only step
    /// that type-checks with deno's own compiler and resolver, which differ
    /// from the `tsc` the packages build with, and it rejects exports the
    /// publish configuration would not ship.
    pub fn publish_check(cwd: &Path) -> Result<CommandResult> {
        Shell::new("deno")
            .args(&["publish", "--dry-run", "--allow-dirty"])
            .dir(cwd)
            .run_checked()
    }
}

// ============================================================================
// npm commands
// ============================================================================

pub mod npm {
    use super::*;

    /// Run npm login (interactive)
    pub fn login() -> Result<()> {
        let status = Command::new("npm")
            .arg("login")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        if status.success() {
            Ok(())
        } else {
            anyhow::bail!("npm login failed")
        }
    }
}

// ============================================================================
// git commands
// ============================================================================

pub mod git {
    use super::*;

    /// Get git status (short format)
    pub fn status(cwd: &Path) -> Result<String> {
        let result = Shell::new("git")
            .args(&["status", "--short"])
            .dir(cwd)
            .run_checked()?;
        Ok(result.stdout)
    }

    /// The checked-out branch; fails on a detached HEAD.
    pub fn current_branch(cwd: &Path) -> Result<String> {
        let result = Shell::new("git")
            .args(&["symbolic-ref", "--short", "HEAD"])
            .dir(cwd)
            .run_checked()
            .context("HEAD is not on a branch")?;
        Ok(result.stdout.trim().to_string())
    }

    /// Fetch one branch from origin into `FETCH_HEAD`.
    pub fn fetch(cwd: &Path, branch: &str) -> Result<()> {
        Shell::new("git")
            .args(&["fetch", "--quiet", "origin", branch])
            .dir(cwd)
            .run_checked()?;
        Ok(())
    }

    /// The commit a revision resolves to, or `None` when it names nothing.
    pub fn commit_of(cwd: &Path, revision: &str) -> Result<Option<String>> {
        let commit = format!("{revision}^{{commit}}");
        let result = Shell::new("git")
            .args(&["rev-parse", "--quiet", "--verify", &commit])
            .dir(cwd)
            .run()?;
        // `--verify --quiet` exits 1 without output for an unknown revision.
        match result.exit_code {
            Some(0) => Ok(Some(result.stdout.trim().to_string())),
            Some(1) if result.stdout.trim().is_empty() => Ok(None),
            _ => anyhow::bail!("git rev-parse {revision} failed: {}", result.transcript()),
        }
    }

    /// The commit origin's tag points at, if origin has the tag.
    pub fn remote_tag_commit(cwd: &Path, tag_name: &str) -> Result<Option<String>> {
        let reference = format!("refs/tags/{tag_name}");
        let result = Shell::new("git")
            .args(&["ls-remote", "--tags", "origin", &reference])
            .dir(cwd)
            .run_checked()?;
        Ok(result.stdout.split_whitespace().next().map(str::to_string))
    }

    /// Delete a local tag.
    pub fn delete_tag(cwd: &Path, tag_name: &str) -> Result<()> {
        Shell::new("git")
            .args(&["tag", "--delete", tag_name])
            .dir(cwd)
            .run_checked()?;
        Ok(())
    }

    /// Create a lightweight tag at HEAD; fails if the tag exists.
    pub fn tag(cwd: &Path, tag_name: &str) -> Result<()> {
        Shell::new("git")
            .args(&["tag", tag_name])
            .dir(cwd)
            .run_checked()?;
        Ok(())
    }

    /// Push a single tag to remote
    pub fn push_tag(cwd: &Path, tag_name: &str) -> Result<()> {
        Shell::new("git")
            .args(&["push", "origin", tag_name])
            .dir(cwd)
            .run_checked()?;
        Ok(())
    }
}

// ============================================================================
// GitLab commands
// ============================================================================

pub mod glab {
    use super::*;

    /// Delete a tag on the GitLab project of `cwd`. Release tags are protected,
    /// and GitLab refuses to delete a protected tag through `git push`.
    pub fn delete_tag(cwd: &Path, tag_name: &str) -> Result<()> {
        let endpoint = format!("projects/:id/repository/tags/{tag_name}");
        Shell::new("glab")
            .args(&["api", "--method", "DELETE", &endpoint])
            .dir(cwd)
            .run_checked()?;
        Ok(())
    }
}

// ============================================================================
// macroforge commands
// ============================================================================

pub mod macroforge {
    use super::*;

    /// `MACROFORGE_CLI` when set, otherwise this checkout's debug build. Never
    /// the `macroforge` on PATH, which other projects pin to their own version.
    pub fn binary(root: &Path) -> Result<String> {
        let binary = std::env::var_os("MACROFORGE_CLI")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| root.join("target/debug/macroforge"));
        if !binary.is_file() {
            anyhow::bail!(
                "macroforge CLI not found at {}; build it with `pixi run build:cli`",
                binary.display()
            );
        }
        Ok(binary.to_string_lossy().into_owned())
    }

    /// Runs `macroforge tsc` against a tsconfig, from the tsconfig's directory.
    pub fn tsc(root: &Path, tsconfig: &Path) -> Result<CommandResult> {
        let project_dir = tsconfig
            .parent()
            .with_context(|| format!("{} has no parent directory", tsconfig.display()))?;
        let binary = binary(root)?;
        let tsconfig = tsconfig.to_string_lossy();
        Shell::new(&binary)
            .args(&["tsc", "--project", &tsconfig])
            .dir(project_dir)
            .run()
    }

    /// Runs `macroforge cache` over a project, writing the type and
    /// declarative registries its builds read from `.macroforge/`.
    pub fn cache(root: &Path, project_dir: &Path) -> Result<()> {
        let binary = binary(root)?;
        Shell::new(&binary)
            .args(&["cache", "."])
            .dir(project_dir)
            .run_checked()
            .with_context(|| format!("macroforge cache failed in {}", project_dir.display()))?;
        Ok(())
    }

    /// Runs `macroforge svelte-check` in a Svelte project with machine-verbose output.
    pub fn svelte_check(root: &Path, project_dir: &Path) -> Result<CommandResult> {
        let binary = binary(root)?;
        Shell::new(&binary)
            .args(&["svelte-check", "--output", "machine-verbose"])
            .dir(project_dir)
            .run()
    }
}
