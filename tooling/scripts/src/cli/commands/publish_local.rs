//! Publish-local command
//!
//! Publishes all packages to their registries (npm + crates.io) from a local
//! machine. Builds native binaries, publishes in dependency order (using the
//! same topological sort as the commit command), and polls registries to ensure
//! each package is available before publishing dependents.

use crate::cli::PublishLocalArgs;
use crate::core::config::{Config, PLATFORMS};
use crate::core::deps;
use crate::core::registry;
use crate::core::repos::RepoType;
use crate::core::shell::{self, Shell};
use crate::utils::format;
use anyhow::{Context, Result};
use colored::Colorize;
use std::io::{self, Write};
use std::path::Path;
use std::time::{Duration, Instant};

const POLL_INTERVAL: Duration = Duration::from_secs(30);
const POLL_TIMEOUT: Duration = Duration::from_secs(30 * 60);

// ---------------------------------------------------------------------------
// Registry helpers
// ---------------------------------------------------------------------------

fn npm_already_published(package: &str, version: &str) -> bool {
    registry::npm_version(package).ok().flatten().as_deref() == Some(version)
}

fn crate_already_published(crate_name: &str, version: &str) -> bool {
    registry::crates_version(crate_name)
        .ok()
        .flatten()
        .as_deref()
        == Some(version)
}

fn wait_for_npm(package: &str, version: &str) -> Result<()> {
    let start = Instant::now();
    loop {
        if npm_already_published(package, version) {
            return Ok(());
        }
        if start.elapsed() > POLL_TIMEOUT {
            anyhow::bail!(
                "Timed out waiting for {}@{} on npm ({}m)",
                package,
                version,
                POLL_TIMEOUT.as_secs() / 60,
            );
        }
        format::info(&format!(
            "Waiting for {}@{} on npm ({:.0}s)...",
            package,
            version,
            start.elapsed().as_secs_f64(),
        ));
        std::thread::sleep(POLL_INTERVAL);
    }
}

fn wait_for_crate(crate_name: &str, version: &str) -> Result<()> {
    let start = Instant::now();
    loop {
        if crate_already_published(crate_name, version) {
            return Ok(());
        }
        if start.elapsed() > POLL_TIMEOUT {
            anyhow::bail!(
                "Timed out waiting for {}@{} on crates.io ({}m)",
                crate_name,
                version,
                POLL_TIMEOUT.as_secs() / 60,
            );
        }
        format::info(&format!(
            "Waiting for {}@{} on crates.io ({:.0}s)...",
            crate_name,
            version,
            start.elapsed().as_secs_f64(),
        ));
        std::thread::sleep(POLL_INTERVAL);
    }
}

// ---------------------------------------------------------------------------
// Publish helpers
// ---------------------------------------------------------------------------

/// Returns true if actually published, false if skipped.
fn publish_npm(dir: &Path, package: &str, version: &str, dry_run: bool) -> Result<bool> {
    if npm_already_published(package, version) {
        format::warning(&format!("{}@{} already on npm, skipping", package, version));
        return Ok(false);
    }
    if dry_run {
        format::info(&format!(
            "[dry-run] npm publish {} from {}",
            package,
            dir.display()
        ));
        return Ok(false);
    }

    let result = Shell::new("npm")
        .args(&["publish", "--access", "public"])
        .dir(dir)
        .inherit()
        .run();

    match result {
        Ok(r) if r.success => {
            format::success(&format!("Published {}@{}", package, version));
            Ok(true)
        }
        _ => {
            format::warning("npm publish failed — possibly expired token. Please log in:");
            shell::npm::login()?;

            format::info(&format!("Retrying publish for {}...", package));
            Shell::new("npm")
                .args(&["publish", "--access", "public"])
                .dir(dir)
                .inherit()
                .run_checked()
                .with_context(|| format!("npm publish failed for {} (after re-auth)", package))?;
            format::success(&format!("Published {}@{}", package, version));
            Ok(true)
        }
    }
}

/// Returns true if actually published, false if skipped.
fn publish_crate(dir: &Path, crate_name: &str, version: &str, dry_run: bool) -> Result<bool> {
    if crate_already_published(crate_name, version) {
        format::warning(&format!(
            "{}@{} already on crates.io, skipping",
            crate_name, version
        ));
        return Ok(false);
    }
    if dry_run {
        format::info(&format!(
            "[dry-run] cargo publish {} from {}",
            crate_name,
            dir.display()
        ));
        return Ok(false);
    }

    // Try publish, re-auth on 403, retry once
    let result = Shell::new("cargo")
        .args(&["publish", "--allow-dirty"])
        .dir(dir)
        .inherit()
        .run();

    match result {
        Ok(r) if r.success => {
            format::success(&format!("Published {}@{}", crate_name, version));
            Ok(true)
        }
        _ => {
            format::warning("Publish failed — possibly expired token. Please log in:");
            shell::cargo::login()?;

            // Retry
            format::info(&format!("Retrying publish for {}...", crate_name));
            Shell::new("cargo")
                .args(&["publish", "--allow-dirty"])
                .dir(dir)
                .inherit()
                .run_checked()
                .with_context(|| {
                    format!("cargo publish failed for {} (after re-auth)", crate_name)
                })?;
            format::success(&format!("Published {}@{}", crate_name, version));
            Ok(true)
        }
    }
}

fn confirm(message: &str) -> Result<bool> {
    print!("{} [y/N] ", message);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().eq_ignore_ascii_case("y"))
}

// ---------------------------------------------------------------------------
// Auth checks
// ---------------------------------------------------------------------------

/// Check if logged in to npm. If not, prompt user to log in.
fn ensure_npm_auth() -> Result<()> {
    let result = Shell::new("npm").args(&["whoami"]).run();
    match result {
        Ok(r) if r.success => {
            format::success(&format!("npm: logged in as {}", r.output().trim()));
            Ok(())
        }
        _ => {
            format::warning("Not logged in to npm");
            println!("Running `npm login`...");
            shell::npm::login()?;

            // Verify
            let verify = Shell::new("npm").args(&["whoami"]).run_checked()?;
            format::success(&format!("npm: logged in as {}", verify.output().trim()));
            Ok(())
        }
    }
}

/// Check if logged in to crates.io by calling `cargo login --help` won't work,
/// so we use `cargo owner --list` on a known crate to verify the token is valid.
fn ensure_cargo_auth() -> Result<()> {
    // Try listing owners of one of our crates to verify the token works
    let check = Shell::new("cargo")
        .args(&["owner", "--list", "-q", "macroforge_ts_syn"])
        .run();

    match check {
        Ok(r) if r.success => {
            format::success("crates.io: authenticated");
            Ok(())
        }
        _ => {
            format::warning("crates.io auth failed or expired — please log in");
            shell::cargo::login()?;
            format::success("crates.io: logged in");
            Ok(())
        }
    }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

pub fn run(args: &PublishLocalArgs) -> Result<()> {
    let config = Config::load()?;
    let root = &config.root;
    let versions = &config.versions;

    let version = versions
        .get_local("core")
        .context("No local version for 'core'")?
        .to_string();

    // Dependency-ordered queue (same algorithm as commit command)
    let dep_order = deps::topo_order(&config.deps)?;

    format::header("Publish Local");
    if args.dry_run {
        println!("{}", "DRY RUN".yellow().bold());
    }
    println!("Version: {}", version.cyan());
    println!();

    // Check registry auth before doing anything
    if !args.dry_run {
        format::step(0, 0, "Checking registry authentication");
        ensure_npm_auth()?;
        ensure_cargo_auth()?;
        println!();
    }

    // Print plan
    println!("{}", "Publish order:".bold());
    for (i, name) in dep_order.iter().enumerate() {
        if let Some(repo) = config.repos.get(name.as_str()) {
            let registry_name = repo
                .crate_name
                .as_deref()
                .or(repo.npm_name.as_deref())
                .unwrap_or(&repo.name);
            let kind = match repo.repo_type {
                RepoType::Rust => "crate + npm",
                RepoType::Ts => "npm",
                _ => "skip",
            };
            println!("  {}. {} ({})", i + 1, registry_name, kind);
        }
    }
    println!();

    if !args.yes && !args.dry_run && !confirm("Proceed?")? {
        format::warning("Aborted");
        return Ok(());
    }

    let mut published: Vec<String> = Vec::new();
    let mut skipped: Vec<String> = Vec::new();

    // Total steps: build + platforms + each repo in dep_order
    let total = 2 + dep_order.len();
    let mut step = 0;

    // ── Step 1: Build native binaries ────────────────────────────────────
    step += 1;
    format::step(step, total, "Building native binaries");

    if args.skip_build {
        format::warning("Skipping (--skip-build)");
    } else if args.dry_run {
        format::info("[dry-run] napi build --platform --release");
    } else {
        shell::deno::run_checked(
            &root.join("crates/macroforge_ts"),
            &[
                "run",
                "-A",
                "npm:@napi-rs/cli/napi",
                "build",
                "--platform",
                "--release",
            ],
        )
        .context("napi build failed")?;
        format::success("Built all platform binaries");
    }

    // ── Step 2: Publish platform npm packages ────────────────────────────
    step += 1;
    format::step(step, total, "Publishing platform npm packages");

    for platform in PLATFORMS {
        let dir = root.join(format!("crates/macroforge_ts/npm/{}", platform));
        let pkg_json = dir.join("package.json");
        if !pkg_json.exists() {
            continue;
        }

        let content = std::fs::read_to_string(&pkg_json)?;
        let pkg: serde_json::Value = serde_json::from_str(&content)?;
        let name = pkg.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let ver = pkg
            .get("version")
            .and_then(|v| v.as_str())
            .unwrap_or(&version);

        if name.is_empty() {
            continue;
        }

        match publish_npm(&dir, name, ver, args.dry_run)? {
            true => published.push(format!("{}@{}", name, ver)),
            false => skipped.push(format!("{}@{}", name, ver)),
        }
    }

    // Wait for all platform packages
    if !args.dry_run {
        for platform in PLATFORMS {
            let pkg_json = root.join(format!(
                "crates/macroforge_ts/npm/{}/package.json",
                platform
            ));
            if let Ok(content) = std::fs::read_to_string(&pkg_json)
                && let Ok(pkg) = serde_json::from_str::<serde_json::Value>(&content)
            {
                let name = pkg.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let ver = pkg
                    .get("version")
                    .and_then(|v| v.as_str())
                    .unwrap_or(&version);
                if !name.is_empty() {
                    wait_for_npm(name, ver)?;
                }
            }
        }
    }

    // ── Steps 3+: Publish in dependency order ────────────────────────────
    for repo_name in &dep_order {
        step += 1;
        let repo = match config.repos.get(repo_name.as_str()) {
            Some(r) => r,
            None => continue,
        };

        let pkg_version = versions
            .get_local(repo_name)
            .unwrap_or(&version)
            .to_string();

        match repo.repo_type {
            RepoType::Rust => {
                // Publish crate if it has a crate name
                if let Some(crate_name) = &repo.crate_name {
                    format::step(step, total, &format!("Publishing crate {}", crate_name));
                    match publish_crate(&repo.abs_path, crate_name, &pkg_version, args.dry_run)? {
                        true => {
                            if !args.dry_run {
                                wait_for_crate(crate_name, &pkg_version)?;
                            }
                            published.push(format!("{}@{}", crate_name, pkg_version));
                        }
                        false => skipped.push(format!("{}@{}", crate_name, pkg_version)),
                    }
                }

                // Also publish npm if it has an npm name
                if let Some(npm_name) = &repo.npm_name {
                    format::info(&format!(
                        "Publishing npm {}@{}...",
                        npm_name.cyan(),
                        pkg_version
                    ));
                    match publish_npm(&repo.abs_path, npm_name, &pkg_version, args.dry_run)? {
                        true => {
                            if !args.dry_run {
                                wait_for_npm(npm_name, &pkg_version)?;
                            }
                            published.push(format!("{}@{}", npm_name, pkg_version));
                        }
                        false => skipped.push(format!("{}@{}", npm_name, pkg_version)),
                    }
                }
            }
            RepoType::Ts => {
                if let Some(npm_name) = &repo.npm_name {
                    format::step(step, total, &format!("Publishing npm {}", npm_name));
                    match publish_npm(&repo.abs_path, npm_name, &pkg_version, args.dry_run)? {
                        true => {
                            if !args.dry_run {
                                wait_for_npm(npm_name, &pkg_version)?;
                            }
                            published.push(format!("{}@{}", npm_name, pkg_version));
                        }
                        false => skipped.push(format!("{}@{}", npm_name, pkg_version)),
                    }
                }
            }
            _ => {
                format::step(
                    step,
                    total,
                    &format!("Skipping {} (no registry)", repo_name),
                );
            }
        }
    }

    // ── Summary ──────────────────────────────────────────────────────────
    println!();
    format::header("Summary");

    if args.dry_run {
        println!("{}", "DRY RUN - nothing published".yellow().bold());
    } else {
        if !published.is_empty() {
            println!("{}", "Published:".green().bold());
            for item in &published {
                format::success(item);
            }
        }
        if !skipped.is_empty() {
            println!("{}", "Skipped:".yellow().bold());
            for item in &skipped {
                format::warning(item);
            }
        }
        println!(
            "\n{}",
            format!("{} published, {} skipped", published.len(), skipped.len()).bold()
        );
    }

    Ok(())
}
