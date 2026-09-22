//! Shell command abstractions
//!
//! Provides a clean API for running external tools (cargo, deno, git).

use anyhow::{Context, Result};
use colored::Colorize;
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
    /// Get combined output (stderr preferred if not empty)
    pub fn output(&self) -> &str {
        if self.stderr.trim().is_empty() {
            &self.stdout
        } else {
            &self.stderr
        }
    }

    /// Get last N lines of output
    pub fn last_lines(&self, n: usize) -> String {
        let lines: Vec<&str> = self.output().lines().collect();
        if lines.len() > n {
            format!("...\n{}", lines[lines.len() - n..].join("\n"))
        } else {
            lines.join("\n")
        }
    }
}

/// Shell command builder
pub struct Shell<'a> {
    program: &'a str,
    args: Vec<&'a str>,
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
            let status = cmd.status().with_context(|| {
                format!(
                    "Failed to execute: {} {}",
                    self.program,
                    self.args.join(" ")
                )
            })?;
            Ok(CommandResult {
                success: status.success(),
                exit_code: status.code(),
                stdout: String::new(),
                stderr: String::new(),
            })
        } else {
            let output = cmd.output().with_context(|| {
                format!(
                    "Failed to execute: {} {}",
                    self.program,
                    self.args.join(" ")
                )
            })?;
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
        let program = self.program.to_string();
        let args = self.args.join(" ");
        let cwd = self.cwd.map(|p| p.to_path_buf());

        let result = self.run()?;

        if result.success {
            Ok(result)
        } else {
            let cwd_str = cwd
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| ".".to_string());

            anyhow::bail!(
                "Command `{} {}` failed in {}\nExit code: {}\n{}",
                program,
                args,
                cwd_str,
                result
                    .exit_code
                    .map(|c| c.to_string())
                    .unwrap_or("unknown".to_string()),
                result.last_lines(25)
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

    /// PATH override that puts the toolchain pinned by `rust-toolchain.toml`
    /// ahead of every other Rust on PATH, or `None` if rustup cannot resolve
    /// one.
    ///
    /// The pixi environment ships a conda-provided `rust` whose `bin` shadows
    /// the rustup shims, and that compiler carries std for the host and
    /// `wasm32-unknown-unknown` only — conda-forge publishes no
    /// `wasm32-wasip1` std, so cross-target builds fail there with a missing
    /// `core`. `rustup which cargo` resolves the pinned toolchain, which does
    /// list the target; prepending its `bin` makes cargo, rustc, and
    /// clippy-driver all come from it.
    ///
    /// `rustup run <toolchain> cargo …` is not enough: it execs the requested
    /// binary without putting the toolchain ahead of the conda one on PATH,
    /// so the cargo it starts still picks up the shadowing rustc.
    fn pinned_toolchain_env(cwd: &Path) -> Option<Vec<(String, String)>> {
        let result = Shell::new("rustup")
            .args(&["which", "cargo"])
            .dir(cwd)
            .run()
            .ok()?;
        if !result.success {
            return None;
        }

        let bin = Path::new(result.stdout.trim()).parent()?.to_path_buf();
        let mut dirs = vec![bin];
        if let Some(existing) = std::env::var_os("PATH") {
            dirs.extend(std::env::split_paths(&existing));
        }
        let path = std::env::join_paths(dirs).ok()?;

        Some(vec![(
            "PATH".to_string(),
            path.to_string_lossy().into_owned(),
        )])
    }

    /// Run cargo clippy with a specific target
    pub fn clippy_target(cwd: &Path, target: &str) -> Result<CommandResult> {
        let mut shell = Shell::new("cargo")
            .args(&["clippy", "--target", target, "--", "-D", "warnings"])
            .dir(cwd);
        if let Some(envs) = pinned_toolchain_env(cwd) {
            shell = shell.envs(envs);
        }
        shell.run_checked()
    }

    /// Run cargo build with a specific target
    pub fn build_target(cwd: &Path, target: &str) -> Result<CommandResult> {
        let mut shell = Shell::new("cargo")
            .args(&["build", "--target", target, "--release"])
            .dir(cwd);
        if let Some(envs) = pinned_toolchain_env(cwd) {
            shell = shell.envs(envs);
        }
        shell.run_checked()
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
    pub fn clippy_json(cwd: &Path) -> Result<CommandResult> {
        Shell::new("cargo")
            .args(&["clippy", "--all-targets", "--message-format=json"])
            .dir(cwd)
            .run()
    }

    /// Rewrite Cargo.lock so it agrees with the manifests on disk.
    ///
    /// Workspace members are recorded in the lock by version, so restoring a
    /// bumped `Cargo.toml` leaves the lock naming a release that was abandoned.
    /// `cargo metadata` resolves the graph and rewrites the lock without
    /// building or touching the network.
    pub fn sync_lock(cwd: &Path) -> Result<CommandResult> {
        Shell::new("cargo")
            .args(&["metadata", "--format-version", "1", "--offline"])
            .dir(cwd)
            .run_checked()
    }

    /// Run every test of the workspace (or crate) at `cwd` via cargo-nextest.
    /// nextest executes each test in its own process, so process-global
    /// state (config caches, registries) cannot leak between tests.
    /// nextest does not run doctests, so a `cargo test --doc` pass follows,
    /// over the members whose library cargo can doctest.
    /// A crate without tests has nothing to fail, so it passes.
    pub fn test(cwd: &Path) -> Result<CommandResult> {
        Shell::new("cargo")
            .args(&["nextest", "run", "--workspace", "--no-tests=pass"])
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

    /// Format markdown `text` as `deno fmt` would format a file under `cwd`,
    /// so generated files are written in their canonical form.
    pub fn format_markdown(cwd: &Path, text: &str) -> Result<String> {
        use std::io::Write;

        let mut child = Command::new("deno")
            .args(["fmt", "--ext", "md", "-"])
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
            .context("failed to send markdown to deno fmt")?;
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
    pub fn lint_json(cwd: &Path) -> Result<CommandResult> {
        Shell::new("deno")
            .args(&["lint", "--json", "."])
            .dir(cwd)
            .run()
    }

    /// Run deno task with live output
    pub fn task_inherit(cwd: &Path, task_name: &str) -> Result<CommandResult> {
        Shell::new("deno")
            .args(&["task", task_name])
            .dir(cwd)
            .inherit()
            .run_checked()
    }

    /// Run deno publish to JSR with live output (auto-discovers deno.json)
    pub fn publish(cwd: &Path) -> Result<CommandResult> {
        Shell::new("deno")
            .args(&["publish", "--allow-dirty"])
            .dir(cwd)
            .inherit()
            .run_checked()
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

/// Run a command with arguments
pub fn run_args(cwd: &Path, program: &str, args: &[&str]) -> Result<CommandResult> {
    if program == "sh" && !args.is_empty() && args[0] == "-c" {
        // Special case for sh -c
        run(args[1], cwd, false)
    } else {
        Shell::new(program).args(args).dir(cwd).run()
    }
}

/// Spawn a binary and return the child process (for real-time output)
pub fn spawn_binary(cwd: &Path, binary: &Path, args: &[&str]) -> Result<std::process::Child> {
    Command::new(binary)
        .args(args)
        .current_dir(cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to spawn binary {}: {}", binary.display(), e))
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
            .run()?;
        Ok(result.stdout)
    }

    /// Create or overwrite a tag (force)
    pub fn tag_force(cwd: &Path, tag_name: &str) -> Result<()> {
        Shell::new("git")
            .args(&["tag", "-f", tag_name])
            .dir(cwd)
            .run_checked()?;
        Ok(())
    }

    /// Push to remote
    pub fn push(cwd: &Path) -> Result<()> {
        Shell::new("git").arg("push").dir(cwd).run_checked()?;
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

    /// Delete a remote tag
    pub fn delete_remote_tag(cwd: &Path, tag_name: &str) -> Result<()> {
        Shell::new("git")
            .args(&["push", "origin", "--delete", tag_name])
            .dir(cwd)
            .run_checked()?;
        Ok(())
    }

    /// Push with upstream tracking
    pub fn push_with_upstream(cwd: &Path, branch: &str) -> Result<()> {
        Shell::new("git")
            .args(&["push", "-u", "origin", branch])
            .dir(cwd)
            .run_checked()?;
        Ok(())
    }

    /// Get current branch name
    pub fn current_branch(cwd: &Path) -> Option<String> {
        Shell::new("git")
            .args(&["branch", "--show-current"])
            .dir(cwd)
            .run()
            .ok()
            .map(|r| r.stdout.trim().to_string())
            .filter(|s| !s.is_empty())
    }

    /// Check if tag exists on remote
    pub fn tag_exists_remote(cwd: &Path, tag_name: &str) -> bool {
        Shell::new("git")
            .args(&["ls-remote", "--tags", "origin", tag_name])
            .dir(cwd)
            .run()
            .map(|r| !r.stdout.trim().is_empty())
            .unwrap_or(false)
    }

    /// Get count of unpushed commits
    pub fn unpushed_count(cwd: &Path) -> usize {
        Shell::new("git")
            .args(&["rev-list", "@{u}..HEAD", "--count"])
            .dir(cwd)
            .run()
            .ok()
            .and_then(|r| r.stdout.trim().parse().ok())
            .unwrap_or(0)
    }

    /// Check if branch has upstream
    pub fn has_upstream(cwd: &Path) -> bool {
        Shell::new("git")
            .args(&["rev-parse", "--abbrev-ref", "@{u}"])
            .dir(cwd)
            .run()
            .map(|r| r.success)
            .unwrap_or(false)
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

    /// Runs `macroforge svelte-check` in a Svelte project with machine-verbose output.
    pub fn svelte_check(root: &Path, project_dir: &Path) -> Result<CommandResult> {
        let binary = binary(root)?;
        Shell::new(&binary)
            .args(&["svelte-check", "--output", "machine-verbose"])
            .dir(project_dir)
            .run()
    }
}

// ============================================================================
// Generic shell command (for npm scripts, etc.)
// ============================================================================

/// Run a shell command string (via sh -c)
pub fn run(cmd: &str, cwd: &Path, verbose: bool) -> Result<CommandResult> {
    if verbose {
        println!("  > {}", cmd.yellow());
    }

    let shell = if cfg!(target_os = "windows") {
        Shell::new("cmd").args(&["/C", cmd]).dir(cwd)
    } else {
        Shell::new("sh").args(&["-c", cmd]).dir(cwd)
    };

    if verbose {
        shell.inherit().run_checked()
    } else {
        let result = shell.run()?;
        if result.success {
            Ok(result)
        } else {
            // Show error details
            eprintln!("\n{}", "─".repeat(60).dimmed());
            eprintln!("{}: {}", "Command".red().bold(), cmd);
            eprintln!("{}: {}", "Directory".red().bold(), cwd.display());
            eprintln!(
                "{}: {}",
                "Exit code".red().bold(),
                result
                    .exit_code
                    .map(|c| c.to_string())
                    .unwrap_or("unknown".to_string())
            );
            let last = result.last_lines(25);
            if !last.trim().is_empty() {
                eprintln!("{}", "─".repeat(60).dimmed());
                eprintln!("{}", last);
            }
            eprintln!("{}", "─".repeat(60).dimmed());

            anyhow::bail!(
                "Command `{}` failed with exit code {}",
                cmd.split_whitespace().next().unwrap_or(cmd),
                result
                    .exit_code
                    .map(|c| c.to_string())
                    .unwrap_or("unknown".to_string())
            )
        }
    }
}
