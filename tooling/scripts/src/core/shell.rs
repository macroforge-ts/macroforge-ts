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
    cwd: Option<&'a Path>,
    inherit_stdio: bool,
}

impl<'a> Shell<'a> {
    /// Create a new shell command
    pub fn new(program: &'a str) -> Self {
        Self {
            program,
            args: Vec::new(),
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

    /// Run cargo fmt --check (fails if unformatted)
    pub fn fmt_check(cwd: &Path) -> Result<CommandResult> {
        Shell::new("cargo")
            .args(&["fmt", "--all", "--", "--check"])
            .dir(cwd)
            .run_checked()
    }

    /// Run cargo clippy with warnings as errors
    pub fn clippy(cwd: &Path) -> Result<CommandResult> {
        Shell::new("cargo")
            .args(&["clippy", "--", "-D", "warnings"])
            .dir(cwd)
            .run_checked()
    }

    /// Run cargo clippy with --all-targets --all-features and warnings as errors
    pub fn clippy_all(cwd: &Path) -> Result<CommandResult> {
        Shell::new("cargo")
            .args(&[
                "clippy",
                "--all-targets",
                "--all-features",
                "--",
                "-D",
                "warnings",
            ])
            .dir(cwd)
            .run_checked()
    }

    /// Run cargo clippy with a specific target
    pub fn clippy_target(cwd: &Path, target: &str) -> Result<CommandResult> {
        Shell::new("cargo")
            .args(&["clippy", "--target", target, "--", "-D", "warnings"])
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

    /// Run cargo clippy with JSON output for diagnostics parsing
    pub fn clippy_json(cwd: &Path) -> Result<CommandResult> {
        Shell::new("cargo")
            .args(&["clippy", "--all-targets", "--message-format=json"])
            .dir(cwd)
            .run()
    }

    /// Run cargo test, automatically detecting workspace
    pub fn test(cwd: &Path) -> Result<CommandResult> {
        if has_workspace(cwd) {
            Shell::new("cargo")
                .args(&["test", "--workspace"])
                .dir(cwd)
                .run_checked()
        } else {
            Shell::new("cargo").arg("test").dir(cwd).run_checked()
        }
    }

    /// Check if a directory has a Cargo.toml with a [workspace] section
    fn has_workspace(cwd: &Path) -> bool {
        let cargo_toml = cwd.join("Cargo.toml");
        if let Ok(content) = std::fs::read_to_string(&cargo_toml) {
            content.contains("[workspace]")
        } else {
            false
        }
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

    /// Run deno fmt to format code
    pub fn deno_fmt(cwd: &Path) -> Result<CommandResult> {
        Shell::new("deno")
            .args(&["fmt", "."])
            .dir(cwd)
            .run_checked()
    }

    /// Run deno lint
    pub fn lint(cwd: &Path) -> Result<CommandResult> {
        Shell::new("deno").args(&["lint"]).dir(cwd).run_checked()
    }

    /// Run deno lint with JSON output
    pub fn lint_json(cwd: &Path, config: Option<&Path>) -> Result<CommandResult> {
        let mut shell = Shell::new("deno");
        shell = shell.args(&["lint", "--json"]);

        if let Some(config_path) = config {
            // We need to convert to string and leak it for the lifetime
            let config_str: &'static str =
                Box::leak(config_path.to_string_lossy().into_owned().into_boxed_str());
            shell = shell.args(&["--config", config_str]);
        }

        shell.arg(".").dir(cwd).run()
    }

    /// Run svelte-check via deno
    pub fn svelte_check(cwd: &Path) -> Result<CommandResult> {
        Shell::new("deno")
            .args(&[
                "run",
                "-A",
                "npm:svelte-check",
                "--tsconfig",
                "./tsconfig.json",
            ])
            .dir(cwd)
            .run()
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
            .run()?;
        Ok(result.stdout)
    }

    /// Stage all changes
    pub fn add_all(cwd: &Path) -> Result<()> {
        Shell::new("git")
            .args(&["add", "-A"])
            .dir(cwd)
            .run_checked()?;
        Ok(())
    }

    /// Create a commit
    pub fn commit(cwd: &Path, message: &str) -> Result<()> {
        Shell::new("git")
            .args(&["commit", "-m", message])
            .dir(cwd)
            .run_checked()?;
        Ok(())
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

    /// Delete a remote tag (used to force GitHub to re-trigger CI on retag)
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

    /// Pull from remote
    pub fn pull(cwd: &Path, remote: &str) -> Result<()> {
        Shell::new("git")
            .args(&["pull", remote])
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

    /// Load environment variables from tooling/.env file
    pub fn load_env(root: &Path) -> Vec<(String, String)> {
        let env_path = root.join("tooling/.env");
        let mut vars = Vec::new();

        if let Ok(content) = std::fs::read_to_string(&env_path) {
            for line in content.lines() {
                let line = line.trim();
                // Skip comments and empty lines
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                // Parse KEY=value or export KEY=value
                let line = line.strip_prefix("export ").unwrap_or(line);
                if let Some((key, value)) = line.split_once('=') {
                    // Remove quotes from value
                    let value = value.trim_matches('"').trim_matches('\'');
                    vars.push((key.to_string(), value.to_string()));
                }
            }
        }

        vars
    }

    /// Get the local macroforge crate path from env vars
    pub fn get_crate_path(root: &Path, env_vars: &[(String, String)]) -> String {
        env_vars
            .iter()
            .find(|(k, _)| k == "MACROFORGE_TS_CRATE")
            .map(|(_, v)| v.clone())
            .unwrap_or_else(|| {
                root.join("crates/macroforge_ts")
                    .to_string_lossy()
                    .to_string()
            })
    }

    /// Run macroforge tsc on a tsconfig file
    pub fn tsc(root: &Path, tsconfig: &Path) -> Result<CommandResult> {
        let env_vars = load_env(root);
        let crate_path = get_crate_path(root, &env_vars);

        let mut cmd = Command::new("deno");
        cmd.args([
            "run",
            "-A",
            &crate_path,
            "tsc",
            "-p",
            &tsconfig.to_string_lossy(),
        ])
        .current_dir(root);

        // Pass env vars to the command
        for (key, value) in &env_vars {
            cmd.env(key, value);
        }

        let output = cmd
            .output()
            .with_context(|| format!("Failed to run macroforge tsc on {}", tsconfig.display()))?;

        Ok(CommandResult {
            success: output.status.success(),
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
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
