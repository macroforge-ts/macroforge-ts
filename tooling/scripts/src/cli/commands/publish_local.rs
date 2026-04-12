//! Publish-local command
//!
//! Publishes all packages to their registries (npm, crates.io, JSR) from a
//! local machine. Builds WASM, publishes in dependency order (topological sort),
//! and polls registries to ensure each package is available before publishing
//! dependents.

use crate::cli::PublishLocalArgs;
use crate::core::config::Config;
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

fn jsr_already_published(package: &str, version: &str) -> bool {
    if package.is_empty() {
        return false;
    }
    registry::jsr_version(package).ok().flatten().as_deref() == Some(version)
}

/// Read the JSR package name from deno.json in a directory
fn jsr_name(dir: &Path) -> String {
    let path = dir.join("deno.json");
    if let Ok(content) = std::fs::read_to_string(&path)
        && let Ok(jsr) = serde_json::from_str::<serde_json::Value>(&content)
    {
        jsr.get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    } else {
        String::new()
    }
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
            format::success(&format!("Published {}@{} to npm", package, version));
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
            format::success(&format!("Published {}@{} to npm", package, version));
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

    let result = Shell::new("cargo")
        .args(&["publish", "--allow-dirty"])
        .dir(dir)
        .inherit()
        .run();

    match result {
        Ok(r) if r.success => {
            format::success(&format!(
                "Published {}@{} to crates.io",
                crate_name, version
            ));
            Ok(true)
        }
        _ => {
            format::warning("Publish failed — possibly expired token. Please log in:");
            shell::cargo::login()?;

            format::info(&format!("Retrying publish for {}...", crate_name));
            Shell::new("cargo")
                .args(&["publish", "--allow-dirty"])
                .dir(dir)
                .inherit()
                .run_checked()
                .with_context(|| {
                    format!("cargo publish failed for {} (after re-auth)", crate_name)
                })?;
            format::success(&format!(
                "Published {}@{} to crates.io",
                crate_name, version
            ));
            Ok(true)
        }
    }
}

/// Returns true if actually published, false if skipped.
fn publish_jsr(dir: &Path, package: &str, version: &str, dry_run: bool) -> Result<bool> {
    if !dir.join("deno.json").exists() {
        return Ok(false);
    }
    if jsr_already_published(package, version) {
        format::warning(&format!("{}@{} already on JSR, skipping", package, version));
        return Ok(false);
    }
    if dry_run {
        format::info(&format!(
            "[dry-run] deno publish {} from {}",
            package,
            dir.display()
        ));
        return Ok(false);
    }

    shell::deno::publish(dir).with_context(|| format!("JSR publish failed for {}", package))?;
    format::success(&format!("Published {}@{} to JSR", package, version));
    Ok(true)
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

            let verify = Shell::new("npm").args(&["whoami"]).run_checked()?;
            format::success(&format!("npm: logged in as {}", verify.output().trim()));
            Ok(())
        }
    }
}

/// Check if logged in to crates.io by verifying the token works.
fn ensure_cargo_auth() -> Result<()> {
    let check = Shell::new("cargo")
        .args(&["owner", "--list", "-q", "macroforge_ts_syn"])
        .inherit()
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

/// Verify JSR auth by running a quiet dry-run publish.
fn ensure_jsr_auth(jsr_dir: &Path) -> Result<()> {
    let result = shell::deno::publish_dry_run(jsr_dir);

    match result {
        Ok(r) if r.success => {
            format::success("jsr: authenticated");
            Ok(())
        }
        _ => {
            format::warning("JSR auth check failed — you may be prompted to log in during publish");
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

    // Dependency-ordered queue
    let dep_order = deps::topo_order(&config.deps)?;

    format::header("Publish Local");
    if args.dry_run {
        println!("{}", "DRY RUN".yellow().bold());
    }
    println!("Version: {}", version.cyan());
    println!();

    // Pre-check registries to filter out already-published packages
    // (repo_name, pkg_version, needs_crate, needs_npm, needs_jsr)
    let mut to_publish: Vec<(&str, String, bool, bool, bool)> = Vec::new();
    let mut already_published: Vec<String> = Vec::new();

    for name in &dep_order {
        let Some(repo) = config.repos.get(name.as_str()) else {
            continue;
        };
        let pkg_version = versions.get_local(name).unwrap_or(&version).to_string();
        let has_jsr = repo.abs_path.join("deno.json").exists();

        match repo.repo_type {
            RepoType::Rust => {
                let needs_crate = repo
                    .crate_name
                    .as_deref()
                    .is_some_and(|c| !crate_already_published(c, &pkg_version));
                let needs_npm = repo
                    .npm_name
                    .as_deref()
                    .is_some_and(|n| !npm_already_published(n, &pkg_version));
                let needs_jsr =
                    has_jsr && !jsr_already_published(&jsr_name(&repo.abs_path), &pkg_version);

                if !needs_crate && !needs_npm && !needs_jsr {
                    let label = repo
                        .crate_name
                        .as_deref()
                        .or(repo.npm_name.as_deref())
                        .unwrap_or(name);
                    already_published.push(format!("{}@{}", label, pkg_version));
                } else {
                    to_publish.push((name, pkg_version, needs_crate, needs_npm, needs_jsr));
                }
            }
            RepoType::Ts => {
                let needs_npm = repo
                    .npm_name
                    .as_deref()
                    .is_some_and(|n| !npm_already_published(n, &pkg_version));
                let needs_jsr = has_jsr
                    && repo
                        .npm_name
                        .as_deref()
                        .is_some_and(|n| !jsr_already_published(n, &pkg_version));

                if !needs_npm && !needs_jsr {
                    let label = repo.npm_name.as_deref().unwrap_or(name);
                    already_published.push(format!("{}@{}", label, pkg_version));
                } else {
                    to_publish.push((name, pkg_version, false, needs_npm, needs_jsr));
                }
            }
            _ => {}
        }
    }

    if !already_published.is_empty() {
        println!("{}", "Already published:".dimmed());
        for item in &already_published {
            println!("  {} {}", "✓".green(), item.dimmed());
        }
        println!();
    }

    if to_publish.is_empty() {
        format::success("Everything is already published");
        return Ok(());
    }

    println!("{}", "Will publish:".bold());
    for (i, (name, pkg_version, needs_crate, needs_npm, needs_jsr)) in to_publish.iter().enumerate()
    {
        let repo = &config.repos[*name];
        let registry_name = repo
            .crate_name
            .as_deref()
            .or(repo.npm_name.as_deref())
            .unwrap_or(name);
        let mut targets = Vec::new();
        if *needs_crate {
            targets.push("crate");
        }
        if *needs_npm {
            targets.push("npm");
        }
        if *needs_jsr {
            targets.push("jsr");
        }
        println!(
            "  {}. {} @ {} ({})",
            i + 1,
            registry_name.bold(),
            pkg_version.green(),
            targets.join(" + ")
        );
    }
    println!();

    // Check registry auth before doing anything
    if !args.dry_run {
        format::step(0, 0, "Checking registry authentication");
        let has_npm = to_publish.iter().any(|(_, _, _, npm, _)| *npm);
        let has_crate = to_publish.iter().any(|(_, _, crate_, _, _)| *crate_);
        let has_jsr = to_publish.iter().any(|(_, _, _, _, jsr)| *jsr);
        if has_npm {
            ensure_npm_auth()?;
        }
        if has_crate {
            ensure_cargo_auth()?;
        }
        if has_jsr {
            if let Some((jsr_name, _, _, _, _)) = to_publish.iter().find(|(_, _, _, _, jsr)| *jsr) {
                let jsr_dir = &config.repos[*jsr_name].abs_path;
                ensure_jsr_auth(jsr_dir)?;
            }
        }
        println!();
    }

    if !args.yes && !args.dry_run && !confirm("Proceed?")? {
        format::warning("Aborted");
        return Ok(());
    }

    let mut published: Vec<String> = Vec::new();
    let mut skipped: Vec<String> = Vec::new();

    // Only build WASM if core needs publishing to npm or JSR
    let core_needs_publish = to_publish
        .iter()
        .any(|(name, _, _, needs_npm, needs_jsr)| *name == "core" && (*needs_npm || *needs_jsr));
    let needs_wasm_build = core_needs_publish && !args.skip_build;

    let total = if needs_wasm_build { 1 } else { 0 } + to_publish.len();
    let mut step = 0;

    if needs_wasm_build {
        step += 1;
        format::step(step, total, "Building WASM");
        if args.dry_run {
            format::info("[dry-run] deno task build:wasm");
        } else {
            shell::deno::task_inherit(&root.join("crates/macroforge_ts"), "build:wasm")
                .context("WASM build failed")?;
            format::success("Built WASM package");
        }
    } else if core_needs_publish && args.skip_build {
        format::warning("Skipping WASM build (--skip-build)");
    }

    // ── Steps 2+: Publish in dependency order ────────────────────────────
    for (repo_name, pkg_version, needs_crate, needs_npm, needs_jsr) in &to_publish {
        step += 1;
        let repo = &config.repos[*repo_name];
        let display_name = repo
            .crate_name
            .as_deref()
            .or(repo.npm_name.as_deref())
            .unwrap_or(repo_name);
        format::step(step, total, &format!("Publishing {}", display_name));

        // crates.io
        if *needs_crate {
            if let Some(crate_name) = &repo.crate_name {
                match publish_crate(&repo.abs_path, crate_name, pkg_version, args.dry_run)? {
                    true => {
                        if !args.dry_run {
                            wait_for_crate(crate_name, pkg_version)?;
                        }
                        published.push(format!("{}@{} (crates.io)", crate_name, pkg_version));
                    }
                    false => skipped.push(format!("{}@{} (crates.io)", crate_name, pkg_version)),
                }
            }
        }

        // npm
        if *needs_npm {
            let npm_name = repo.npm_name.as_deref().unwrap_or(repo_name);
            match publish_npm(&repo.abs_path, npm_name, pkg_version, args.dry_run)? {
                true => {
                    if !args.dry_run {
                        wait_for_npm(npm_name, pkg_version)?;
                    }
                    published.push(format!("{}@{} (npm)", npm_name, pkg_version));
                }
                false => skipped.push(format!("{}@{} (npm)", npm_name, pkg_version)),
            }
        }

        // JSR
        if *needs_jsr {
            let name = jsr_name(&repo.abs_path);
            match publish_jsr(&repo.abs_path, &name, pkg_version, args.dry_run)? {
                true => published.push(format!("{}@{} (jsr)", name, pkg_version)),
                false => {}
            }
        }
    }

    // ── Summary ─────────────────────────────────────────────────────────
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
