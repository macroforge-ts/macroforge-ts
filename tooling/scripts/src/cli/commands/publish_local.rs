//! Publish-local command
//!
//! Publishes all packages to their registries (npm, crates.io, JSR), from a
//! local machine or, with `--non-interactive`, from CI. Crates publish first,
//! then WASM and the npm packages are built and npm and JSR publish. Both
//! phases run in dependency order (topological sort) and poll npm and
//! crates.io so each package is available before its dependents publish (JSR
//! publishes are not polled).
//!
//! `--registry` narrows a run to some registries, and `--build-only` stops
//! after the builds, so CI can build on one runner and publish from another.

use crate::cli::args::{PublishLocalArgs, Registry};
use crate::core::config::Config;
use crate::core::deps;
use crate::core::manifests;
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

/// How a registry authentication failure is handled.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Auth {
    /// Prompt for `npm login` / `cargo login` and retry.
    Interactive,
    /// Fail: credentials must come from the environment.
    NonInteractive,
}

/// How npm authenticates.
#[derive(Clone, Copy, PartialEq, Eq)]
enum NpmAuth {
    /// npm's own credentials: `npm login` or a token in `.npmrc`.
    Npm,
    /// A CI OIDC ID token (`NPM_ID_TOKEN`), which npm exchanges itself while
    /// publishing, so `npm whoami` cannot confirm it beforehand.
    TrustedPublishing,
}

/// How cargo authenticates to crates.io.
enum CratesAuth {
    /// cargo's own credentials: `cargo login` or `CARGO_REGISTRY_TOKEN`.
    Cargo,
    /// A CI OIDC ID token (`CRATES_IO_ID_TOKEN`), exchanged for a publish token.
    TrustedPublishing(String),
}

/// A repo with at least one selected registry still missing its version.
struct Pending<'a> {
    name: &'a str,
    /// The package name shown for it: the crate's, else npm's, else JSR's,
    /// among the selected registries.
    label: String,
    version: String,
    needs_crate: bool,
    needs_npm: bool,
    needs_jsr: bool,
}

/// The builds the pending npm and JSR publishes need.
#[derive(Default)]
struct Builds {
    /// Core's npm and JSR packages both ship the release WASM.
    wasm: bool,
    npm: bool,
}

impl Builds {
    fn needed(to_publish: &[Pending]) -> Self {
        Self {
            wasm: to_publish
                .iter()
                .any(|pending| pending.name == "core" && (pending.needs_npm || pending.needs_jsr)),
            npm: to_publish.iter().any(|pending| pending.needs_npm),
        }
    }

    fn count(&self) -> usize {
        usize::from(self.wasm) + usize::from(self.npm)
    }
}

/// Step numbering and outcome of a publish run.
struct Report {
    step: usize,
    total: usize,
    published: Vec<String>,
    skipped: Vec<String>,
}

impl Report {
    fn new(total: usize) -> Self {
        Self {
            step: 0,
            total,
            published: Vec::new(),
            skipped: Vec::new(),
        }
    }

    fn step(&mut self, message: &str) {
        self.step += 1;
        format::step(self.step, self.total, message);
    }
}

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

/// Publishes the package `tooling/npm/build.ts` built into `npm/<repo_name>`.
/// Returns true if actually published, false if skipped.
fn publish_npm(
    root: &Path,
    repo_name: &str,
    package: &str,
    version: &str,
    dry_run: bool,
    auth: Auth,
) -> Result<bool> {
    let built = root.join("npm").join(repo_name);
    if npm_already_published(package, version) {
        format::warning(&format!("{}@{} already on npm, skipping", package, version));
        return Ok(false);
    }
    if dry_run {
        format::info(&format!(
            "[dry-run] npm publish {} from {}",
            package,
            built.display()
        ));
        return Ok(false);
    }
    if !built.join("package.json").exists() {
        anyhow::bail!(
            "{} has not been built into {}; run the npm build first",
            package,
            built.display()
        );
    }
    let built_arg = built.to_string_lossy().into_owned();

    let result = Shell::new("npm")
        .args(&["publish", "--access", "public"])
        .arg(&built_arg)
        .inherit()
        .run();

    match result {
        Ok(r) if r.success => {
            format::success(&format!("Published {}@{} to npm", package, version));
            Ok(true)
        }
        _ if auth == Auth::NonInteractive => {
            anyhow::bail!("npm publish failed for {package} (output shown above)")
        }
        _ => {
            format::warning("npm publish failed — possibly expired token. Please log in:");
            shell::npm::login()?;

            format::info(&format!("Retrying publish for {}...", package));
            Shell::new("npm")
                .args(&["publish", "--access", "public"])
                .arg(&built_arg)
                .inherit()
                .run_checked()
                .with_context(|| format!("npm publish failed for {} (after re-auth)", package))?;
            format::success(&format!("Published {}@{} to npm", package, version));
            Ok(true)
        }
    }
}

/// Returns true if actually published, false if skipped.
fn publish_crate(
    dir: &Path,
    crate_name: &str,
    version: &str,
    dry_run: bool,
    auth: Auth,
    token: Option<&str>,
) -> Result<bool> {
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

    let envs = token
        .map(|token| vec![("CARGO_REGISTRY_TOKEN".to_string(), token.to_string())])
        .unwrap_or_default();
    let result = Shell::new("cargo")
        .args(&["publish", "--allow-dirty"])
        .envs(envs)
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
        _ if auth == Auth::NonInteractive => {
            anyhow::bail!("cargo publish failed for {crate_name}")
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
fn publish_jsr(
    dir: &Path,
    package: &str,
    version: &str,
    dry_run: bool,
    token: Option<&str>,
) -> Result<bool> {
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

    shell::deno::publish(dir, token)
        .with_context(|| format!("JSR publish failed for {}", package))?;
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
fn ensure_npm_auth(auth: Auth, npm_auth: NpmAuth) -> Result<()> {
    if npm_auth == NpmAuth::TrustedPublishing {
        format::success("npm: trusted publishing");
        return Ok(());
    }
    let result = Shell::new("npm").args(&["whoami"]).run();
    match result {
        Ok(r) if r.success => {
            format::success(&format!("npm: logged in as {}", r.stdout.trim()));
            Ok(())
        }
        _ if auth == Auth::NonInteractive => {
            anyhow::bail!("npm: not authenticated; configure an npm token")
        }
        _ => {
            format::warning("Not logged in to npm");
            println!("Running `npm login`...");
            shell::npm::login()?;

            let verify = Shell::new("npm").args(&["whoami"]).run_checked()?;
            format::success(&format!("npm: logged in as {}", verify.stdout.trim()));
            Ok(())
        }
    }
}

/// Check if logged in to crates.io by verifying the token works. A trusted
/// publishing ID token is single-use, so it is only exchanged when publishing.
fn ensure_cargo_auth(auth: Auth, crates_auth: &CratesAuth) -> Result<()> {
    if let CratesAuth::TrustedPublishing(_) = crates_auth {
        format::success("crates.io: trusted publishing");
        return Ok(());
    }
    let check = Shell::new("cargo")
        .args(&["owner", "--list", "-q", "macroforge_ts_syn"])
        .inherit()
        .run();

    match check {
        Ok(r) if r.success => {
            format::success("crates.io: authenticated");
            Ok(())
        }
        _ if auth == Auth::NonInteractive => {
            anyhow::bail!("crates.io: not authenticated; set CARGO_REGISTRY_TOKEN")
        }
        _ => {
            format::warning("crates.io auth failed or expired — please log in");
            shell::cargo::login()?;
            format::success("crates.io: logged in");
            Ok(())
        }
    }
}

/// Publishes every pending crate in dependency order, each indexed before its
/// dependents publish. A trusted publishing token lasts 30 minutes, so it is
/// exchanged here, right before use, and revoked once the crates are done.
fn publish_crates(
    config: &Config,
    to_publish: &[Pending],
    dry_run: bool,
    auth: Auth,
    crates_auth: &CratesAuth,
    report: &mut Report,
) -> Result<()> {
    let token = match crates_auth {
        CratesAuth::TrustedPublishing(id_token) if !dry_run => {
            Some(registry::crates_trusted_publishing_token(id_token)?)
        }
        _ => None,
    };
    let result = publish_each_crate(config, to_publish, dry_run, auth, token.as_deref(), report);
    if let Some(token) = &token
        && let Err(error) = registry::revoke_crates_trusted_publishing_token(token)
    {
        // The token expires by itself; the publish outcome is what matters.
        format::warning(&format!("{error:#}"));
    }
    result
}

fn publish_each_crate(
    config: &Config,
    to_publish: &[Pending],
    dry_run: bool,
    auth: Auth,
    token: Option<&str>,
    report: &mut Report,
) -> Result<()> {
    for pending in to_publish.iter().filter(|pending| pending.needs_crate) {
        let repo = &config.repos[pending.name];
        let Some(crate_name) = &repo.crate_name else {
            continue;
        };
        report.step(&format!("Publishing {crate_name} to crates.io"));
        let label = format!("{}@{} (crates.io)", crate_name, pending.version);
        if publish_crate(
            &repo.abs_path,
            crate_name,
            &pending.version,
            dry_run,
            auth,
            token,
        )? {
            if !dry_run {
                wait_for_crate(crate_name, &pending.version)?;
            }
            report.published.push(label);
        } else {
            report.skipped.push(label);
        }
    }
    Ok(())
}

/// Builds the WASM and the npm packages the pending publishes need.
fn build(root: &Path, builds: &Builds, dry_run: bool, report: &mut Report) -> Result<()> {
    if builds.wasm {
        report.step("Building WASM");
        if dry_run {
            format::info("[dry-run] deno task build:wasm:release");
        } else {
            // Publishing is what opts into the release profile; every other
            // build in this repo is debug.
            shell::deno::task_inherit(&root.join("crates/macroforge_ts"), "build:wasm:release")
                .context("WASM build failed")?;
            format::success("Built WASM package");
        }
    }

    if builds.npm {
        report.step("Building npm packages");
        if dry_run {
            format::info("[dry-run] deno run -A tooling/npm/build.ts");
        } else {
            Shell::new("deno")
                .args(&["run", "-A", "tooling/npm/build.ts"])
                .dir(root)
                .inherit()
                .run_checked()
                .context("npm package build failed")?;
            format::success("Built npm packages");
        }
    }
    Ok(())
}

/// Publishes every pending npm and JSR package in dependency order, each on
/// npm before its dependents publish.
fn publish_packages(
    config: &Config,
    to_publish: &[Pending],
    dry_run: bool,
    auth: Auth,
    jsr_token: Option<&str>,
    report: &mut Report,
) -> Result<()> {
    for pending in to_publish
        .iter()
        .filter(|pending| pending.needs_npm || pending.needs_jsr)
    {
        let repo = &config.repos[pending.name];
        let npm_name = repo.npm_name.as_deref().unwrap_or(pending.name);
        report.step(&format!("Publishing {npm_name}"));

        if pending.needs_npm {
            let label = format!("{}@{} (npm)", npm_name, pending.version);
            if publish_npm(
                &config.root,
                pending.name,
                npm_name,
                &pending.version,
                dry_run,
                auth,
            )? {
                if !dry_run {
                    wait_for_npm(npm_name, &pending.version)?;
                }
                report.published.push(label);
            } else {
                report.skipped.push(label);
            }
        }

        if pending.needs_jsr {
            let name = manifests::jsr_package_name(&repo.abs_path)?.with_context(|| {
                format!("{} has no JSR name in deno.json", repo.abs_path.display())
            })?;
            if publish_jsr(&repo.abs_path, &name, &pending.version, dry_run, jsr_token)? {
                report
                    .published
                    .push(format!("{}@{} (jsr)", name, pending.version));
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

/// The repos, in dependency order, with a selected registry still missing
/// their version, and labels for the ones already on every selected registry
/// that publishes them.
fn plan<'a>(
    config: &'a Config,
    version: &str,
    registries: &[Registry],
) -> Result<(Vec<Pending<'a>>, Vec<String>)> {
    let selected = |registry| registries.is_empty() || registries.contains(&registry);
    let mut to_publish = Vec::new();
    let mut already_published = Vec::new();

    for name in deps::topo_order(&config.deps)? {
        let Some((name, repo)) = config.repos.get_key_value(name.as_str()) else {
            continue;
        };
        let pkg_version = version.to_string();
        // The package each selected registry publishes this repo as.
        let crate_name = match repo.repo_type {
            RepoType::Rust => repo
                .crate_name
                .as_deref()
                .filter(|_| selected(Registry::Crates)),
            RepoType::Ts => None,
            _ => continue,
        };
        let jsr_name = if selected(Registry::Jsr) {
            manifests::jsr_package_name(&repo.abs_path)?
        } else {
            None
        };
        let npm_name = if selected(Registry::Npm) && !manifests::npm_private(&repo.abs_path)? {
            repo.npm_name.as_deref()
        } else {
            None
        };
        let Some(label) = crate_name.or(npm_name).or(jsr_name.as_deref()) else {
            continue;
        };
        let label = label.to_string();

        let needs_crate =
            crate_name.is_some_and(|crate_name| !crate_already_published(crate_name, &pkg_version));
        let needs_npm =
            npm_name.is_some_and(|npm_name| !npm_already_published(npm_name, &pkg_version));
        let needs_jsr = jsr_name
            .as_deref()
            .is_some_and(|package| !jsr_already_published(package, &pkg_version));

        if needs_crate || needs_npm || needs_jsr {
            to_publish.push(Pending {
                name,
                label,
                version: pkg_version,
                needs_crate,
                needs_npm,
                needs_jsr,
            });
        } else {
            already_published.push(format!("{}@{}", label, pkg_version));
        }
    }
    Ok((to_publish, already_published))
}

fn print_plan(to_publish: &[Pending], build_only: bool) {
    let heading = if build_only {
        "Will build for:"
    } else {
        "Will publish:"
    };
    println!("{}", heading.bold());
    for (index, pending) in to_publish.iter().enumerate() {
        let mut targets = Vec::new();
        if pending.needs_crate {
            targets.push("crate");
        }
        if pending.needs_npm {
            targets.push("npm");
        }
        if pending.needs_jsr {
            targets.push("jsr");
        }
        println!(
            "  {}. {} @ {} ({})",
            index + 1,
            pending.label.bold(),
            pending.version.green(),
            targets.join(" + ")
        );
    }
    println!();
}

fn print_summary(report: &Report, dry_run: bool) {
    println!();
    format::header("Summary");

    if dry_run {
        println!("{}", "DRY RUN - nothing published".yellow().bold());
        return;
    }
    if !report.published.is_empty() {
        println!("{}", "Published:".green().bold());
        for item in &report.published {
            format::success(item);
        }
    }
    if !report.skipped.is_empty() {
        println!("{}", "Skipped:".yellow().bold());
        for item in &report.skipped {
            format::warning(item);
        }
    }
    println!(
        "\n{}",
        format!(
            "{} published, {} skipped",
            report.published.len(),
            report.skipped.len()
        )
        .bold()
    );
}

/// Entry point for `mf publish-local`: publishes unpublished packages in dependency order.
pub fn run(args: &PublishLocalArgs) -> Result<()> {
    let config = Config::load()?;
    let version = manifests::current_version(&config.root)?;

    format::header("Publish Local");
    if args.dry_run {
        println!("{}", "DRY RUN".yellow().bold());
    }
    println!("Version: {}", version.cyan());
    println!();

    let (to_publish, already_published) = plan(&config, &version, &args.registries)?;

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

    print_plan(&to_publish, args.build_only);

    let builds = Builds::needed(&to_publish);
    if args.build_only {
        let mut report = Report::new(builds.count());
        build(&config.root, &builds, args.dry_run, &mut report)?;
        format::success("Built what the pending packages need; publish them with --skip-build");
        return Ok(());
    }
    let builds = if args.skip_build {
        if builds.count() > 0 {
            format::warning("Skipping the WASM and npm package builds (--skip-build)");
        }
        Builds::default()
    } else {
        builds
    };

    let auth = if args.non_interactive {
        Auth::NonInteractive
    } else {
        Auth::Interactive
    };
    let npm_auth = if std::env::var_os("NPM_ID_TOKEN").is_some() {
        NpmAuth::TrustedPublishing
    } else {
        NpmAuth::Npm
    };
    let crates_auth = match std::env::var("CRATES_IO_ID_TOKEN") {
        Ok(id_token) => CratesAuth::TrustedPublishing(id_token),
        Err(_) => CratesAuth::Cargo,
    };
    let jsr_token = std::env::var("JSR_TOKEN").ok();

    let crate_count = to_publish
        .iter()
        .filter(|pending| pending.needs_crate)
        .count();
    let package_count = to_publish
        .iter()
        .filter(|pending| pending.needs_npm || pending.needs_jsr)
        .count();

    // Check registry auth before doing anything
    if !args.dry_run {
        format::step(0, 0, "Checking registry authentication");
        if to_publish.iter().any(|pending| pending.needs_npm) {
            ensure_npm_auth(auth, npm_auth)?;
        }
        if crate_count > 0 {
            ensure_cargo_auth(auth, &crates_auth)?;
        }
        if to_publish.iter().any(|pending| pending.needs_jsr)
            && auth == Auth::NonInteractive
            && jsr_token.is_none()
        {
            anyhow::bail!("JSR: JSR_TOKEN is not set");
        }
        println!();
    }

    if !args.yes && !args.non_interactive && !args.dry_run && !confirm("Proceed?")? {
        format::warning("Aborted");
        return Ok(());
    }

    let mut report = Report::new(crate_count + builds.count() + package_count);

    // crates.io needs none of the builds
    publish_crates(
        &config,
        &to_publish,
        args.dry_run,
        auth,
        &crates_auth,
        &mut report,
    )?;
    build(&config.root, &builds, args.dry_run, &mut report)?;
    publish_packages(
        &config,
        &to_publish,
        args.dry_run,
        auth,
        jsr_token.as_deref(),
        &mut report,
    )?;

    print_summary(&report, args.dry_run);
    Ok(())
}
