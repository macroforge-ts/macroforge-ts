//! Shell command abstractions
//!
//! Provides a clean API for running external tools (cargo, npm, git, npx).

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
                format!("Failed to execute: {} {}", self.program, self.args.join(" "))
            })?;
            Ok(CommandResult {
                success: status.success(),
                exit_code: status.code(),
                stdout: String::new(),
                stderr: String::new(),
            })
        } else {
            let output = cmd.output().with_context(|| {
                format!("Failed to execute: {} {}", self.program, self.args.join(" "))
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
                result.exit_code.map(|c| c.to_string()).unwrap_or("unknown".to_string()),
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

    /// Run cargo fmt to fix formatting
    pub fn fmt_check(cwd: &Path) -> Result<CommandResult> {
        Shell::new("cargo")
            .args(&["fmt"])
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

    /// Run cargo test
    pub fn test(cwd: &Path) -> Result<CommandResult> {
        Shell::new("cargo").arg("test").dir(cwd).run_checked()
    }

}

// ============================================================================
// npm commands
// ============================================================================

pub mod npm {
    use super::*;

    /// Run npm run <script>
    pub fn run_script(cwd: &Path, script: &str) -> Result<CommandResult> {
        Shell::new("npm")
            .args(&["run", script])
            .dir(cwd)
            .run_checked()
    }
}

// ============================================================================
// npx commands
// ============================================================================

pub mod npx {
    use super::*;

    /// Run biome check with auto-fix (including unsafe fixes)
    pub fn biome_check(cwd: &Path) -> Result<CommandResult> {
        Shell::new("npx")
            .args(&["@biomejs/biome", "check", "--write", "--unsafe", "."])
            .dir(cwd)
            .run_checked()
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

    /// Push tags to remote
    pub fn push_tags(cwd: &Path) -> Result<()> {
        Shell::new("git")
            .args(&["push", "--tags"])
            .dir(cwd)
            .run_checked()?;
        Ok(())
    }

    /// Push a single tag to remote (with force to overwrite)
    pub fn push_tag_force(cwd: &Path, tag_name: &str) -> Result<()> {
        Shell::new("git")
            .args(&["push", "origin", "--force", tag_name])
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
