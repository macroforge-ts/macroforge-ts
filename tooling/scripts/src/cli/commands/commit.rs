//! Commit queue for releasing packages in dependency order
//!
//! Uses dependency graph to ensure packages are committed/pushed in the correct
//! order. Handles git operations, tag creation, and registry polling.
//!
//! Design: Collects all commit messages upfront while a background thread
//! starts processing items. Once all messages are entered, the main thread
//! joins the background processing.

use crate::cli::CommitArgs;
use crate::core::config::{self, Config};
use crate::core::deps;
use crate::core::registry;
use crate::core::repos::RepoType;
use crate::core::shell;
use crate::utils::format;
use anyhow::Result;
use colored::Colorize;
use dialoguer::{Confirm, Input};
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Represents a repository to be committed
#[derive(Debug, Clone)]
struct CommitItem {
    name: String,
    version: String,
    path: PathBuf,
    repo_type: RepoType,
    npm_name: Option<String>,
    crate_name: Option<String>,
    dependencies: Vec<String>,
}

/// Message sent from main thread to worker thread
#[derive(Debug)]
enum WorkerMessage {
    /// Process this item with the given commit message
    Process { index: usize, message: String },
    /// Skip this item (no changes to commit)
    Skip { index: usize },
    /// Retag this item (no changes, but user wants to recreate tag)
    Retag { index: usize },
    /// All messages have been collected, finish up
    AllMessagesCollected,
    /// Shutdown the worker
    Shutdown,
}

/// Status update from worker to main thread
#[derive(Debug)]
enum WorkerStatus {
    /// Item processing started (index, deps info)
    Started(usize, String),
    /// Item completed successfully
    Completed(usize),
    /// Item failed
    Failed(usize, String),
    /// Worker is waiting for dependencies
    WaitingForDeps(usize, String),
    /// Waiting for package on registry
    WaitingForRegistry(usize, String, String), // index, package, registry
    /// Worker finished all work
    Done,
}

pub fn run(args: CommitArgs) -> Result<()> {
    let config = Config::load()?;
    let mut versions = config.versions.clone();
    let mut repos = config.filter_repos(&args.repos);

    // Refresh registry versions for repos we're processing to avoid stale cache issues
    let npm_names = config::npm_package_names();
    let crate_names = config::crate_package_names();
    for repo in &repos {
        if let Some(npm_name) = npm_names.get(repo.name.as_str()) {
            if let Ok(Some(v)) = registry::npm_version(npm_name) {
                versions.set_registry(&repo.name, &v);
            }
        }
        if let Some(crate_name) = crate_names.get(repo.name.as_str()) {
            if let Ok(Some(v)) = registry::crates_version(crate_name) {
                versions.set_registry(&repo.name, &v);
            }
        }
    }

    // Set up Ctrl+C handler
    let interrupted = Arc::new(AtomicBool::new(false));
    let interrupted_clone = Arc::clone(&interrupted);
    ctrlc::set_handler(move || {
        eprintln!("\n{} Interrupted by user", "⚠".yellow());
        interrupted_clone.store(true, Ordering::SeqCst);
        std::process::exit(130);
    })?;

    if repos.is_empty() {
        anyhow::bail!("No repos matched filter: {}", args.repos);
    }

    // Filter repos with cascading if not disabled
    if !args.no_cascade {
        // Cascade to dependents
        let initial_names: Vec<String> = repos.iter().map(|r| r.name.clone()).collect();
        let cascaded = cascade_to_dependents(&config.deps, &initial_names);
        repos = cascaded
            .into_iter()
            .filter_map(|name| config.repos.get(&name))
            .collect();
    }

    // Filter to only repos that have been prepped (local != registry)
    let prepped_repos: Vec<_> = repos
        .into_iter()
        .filter(|repo| is_prepped(&versions, &repo.name))
        .collect();

    if prepped_repos.is_empty() && !args.dry_run {
        format::warning("No repos have been prepped (all local versions match registry)");
        return Ok(());
    }

    format::header("Commit Queue");

    // Build commit queue in dependency order
    let order = deps::topo_order(&config.deps).unwrap_or_default();

    let mut queue: Vec<CommitItem> = Vec::new();

    for name in &order {
        if let Some(repo) = prepped_repos.iter().find(|r| r.name == *name) {
            let version = versions
                .get_local(&repo.name)
                .map(|s| s.to_string())
                .unwrap_or_else(|| "0.1.0".to_string());

            let dependencies = config
                .deps
                .get(name)
                .cloned()
                .unwrap_or_default();

            queue.push(CommitItem {
                name: repo.name.clone(),
                version,
                path: repo.abs_path.clone(),
                repo_type: repo.repo_type,
                npm_name: npm_names.get(repo.name.as_str()).map(|s| s.to_string()),
                crate_name: crate_names.get(repo.name.as_str()).map(|s| s.to_string()),
                dependencies,
            });
        }
    }

    // Add any repos not in the dependency graph
    for repo in &prepped_repos {
        if !queue.iter().any(|p| p.name == repo.name) {
            let version = versions
                .get_local(&repo.name)
                .map(|s| s.to_string())
                .unwrap_or_else(|| "0.1.0".to_string());

            let dependencies = config
                .deps
                .get(&repo.name)
                .cloned()
                .unwrap_or_default();

            queue.push(CommitItem {
                name: repo.name.clone(),
                version,
                path: repo.abs_path.clone(),
                repo_type: repo.repo_type,
                npm_name: npm_names.get(repo.name.as_str()).map(|s| s.to_string()),
                crate_name: crate_names.get(repo.name.as_str()).map(|s| s.to_string()),
                dependencies,
            });
        }
    }

    // Display queue
    println!("\n{}", "Commit queue (in dependency order):".bold());
    for (i, item) in queue.iter().enumerate() {
        let type_label = match item.repo_type {
            RepoType::Rust => "rust".yellow(),
            RepoType::Ts => "ts".cyan(),
            RepoType::Website => "web".magenta(),
            RepoType::Tooling => "tool".blue(),
            RepoType::Extension => "ext".white(),
        };

        let pkg_name = item
            .npm_name
            .as_ref()
            .or(item.crate_name.as_ref())
            .map(|s| s.as_str())
            .unwrap_or(&item.name);

        let deps_display = if item.dependencies.is_empty() {
            String::new()
        } else {
            format!(" {}", format!("(deps: {})", item.dependencies.join(", ")).dimmed())
        };

        println!(
            "  {}. {} {} @ {}{}",
            i + 1,
            type_label,
            pkg_name.bold(),
            item.version.green(),
            deps_display
        );
    }

    // Confirm
    if !args.yes && !args.dry_run {
        println!();
        if !Confirm::new()
            .with_prompt("Proceed with commits?")
            .default(false)
            .interact()?
        {
            format::warning("Aborted");
            return Ok(());
        }
    }

    // For dry run, just show what would happen
    if args.dry_run {
        for (i, item) in queue.iter().enumerate() {
            println!();
            format::step(i + 1, queue.len(), &format!("Processing {}", item.name));

            let status = shell::git::status(&item.path)?;
            let has_changes = !status.trim().is_empty();

            let message = if let Some(ref msg) = args.message {
                msg.clone()
            } else {
                default_commit_message(&item.name, &item.version, &status, has_package(item))
            };

            println!("  {} Would commit: {}", "→".blue(), message.dimmed());
            if has_changes {
                println!("  {} Would stage changes with git add -A", "→".blue());
            }
            if should_create_tag(item, &interrupted)? {
                println!("  {} Would create tag: v{}", "→".blue(), item.version.dimmed());
            }
            println!("  {} Would push to remote", "→".blue());
        }

        println!();
        format::success("All commits complete");
        format::warning("This was a dry run - no changes were made");
        return Ok(());
    }

    // Collect all commit messages and item info upfront
    let mut messages: Vec<Option<String>> = vec![None; queue.len()];
    let mut skip_items: Vec<bool> = vec![false; queue.len()];

    println!("\n{}", "Collecting commit messages...".bold());
    println!("{}", "(Background processing will start as you enter messages)".dimmed());
    println!();

    // Channels for communication with worker thread
    let (tx_to_worker, rx_from_main): (Sender<WorkerMessage>, Receiver<WorkerMessage>) = mpsc::channel();
    let (tx_to_main, rx_from_worker): (Sender<WorkerStatus>, Receiver<WorkerStatus>) = mpsc::channel();

    // Shared state
    let committed_packages: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    let registry_ready: Arc<Mutex<std::collections::HashSet<String>>> = Arc::new(Mutex::new(std::collections::HashSet::new()));
    let current_processing: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(usize::MAX));
    let messages_done = Arc::new(AtomicBool::new(false));

    // Clone for worker thread
    let queue_clone = queue.clone();
    let versions_clone = versions.clone();
    let npm_names_clone: HashMap<String, String> = npm_names.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
    let crate_names_clone: HashMap<String, String> = crate_names.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
    let interrupted_clone = Arc::clone(&interrupted);
    let committed_packages_clone = Arc::clone(&committed_packages);
    let registry_ready_clone = Arc::clone(&registry_ready);
    let current_processing_clone = Arc::clone(&current_processing);
    let messages_done_clone = Arc::clone(&messages_done);

    // Spawn worker thread
    let worker_handle = thread::spawn(move || {
        worker_thread(
            rx_from_main,
            tx_to_main,
            queue_clone,
            versions_clone,
            npm_names_clone,
            crate_names_clone,
            interrupted_clone,
            committed_packages_clone,
            registry_ready_clone,
            current_processing_clone,
            messages_done_clone,
        )
    });

    // Main thread: collect all commit messages
    for (i, item) in queue.iter().enumerate() {
        if interrupted.load(Ordering::SeqCst) {
            let _ = tx_to_worker.send(WorkerMessage::Shutdown);
            anyhow::bail!("Interrupted by user");
        }

        if !item.path.exists() {
            format::warning(&format!("Path does not exist: {:?}", item.path));
            skip_items[i] = true;
            let _ = tx_to_worker.send(WorkerMessage::Skip { index: i });
            continue;
        }

        // Check for changes
        let status = shell::git::status(&item.path)?;
        let has_changes = !status.trim().is_empty();
        let unpushed_count = shell::git::unpushed_count(&item.path);

        if !has_changes && unpushed_count == 0 {
            // No changes - check if retagging makes sense
            let tag = format!("v{}", item.version);
            let can_retag = can_retag_item(item);

            if can_retag && Confirm::new()
                .with_prompt(format!("  {} {} - no changes. Retag {}?", "→".dimmed(), item.name, tag.cyan()))
                .default(false)
                .interact()?
            {
                let _ = tx_to_worker.send(WorkerMessage::Retag { index: i });
            } else {
                if !can_retag {
                    println!("  {} {} - no changes, version already published", "→".dimmed(), item.name.dimmed());
                }
                skip_items[i] = true;
                let _ = tx_to_worker.send(WorkerMessage::Skip { index: i });
            }
            continue;
        }

        // Collect commit message
        let message = if let Some(ref msg) = args.message {
            msg.clone()
        } else {
            // Interactive prompt
            let default_msg = default_commit_message(&item.name, &item.version, &status, has_package(item));
            Input::new()
                .with_prompt(format!("[{}/{}] {}", i + 1, queue.len(), item.name.cyan()))
                .default(default_msg)
                .interact_text()?
        };

        messages[i] = Some(message.clone());

        // Send to worker immediately
        let _ = tx_to_worker.send(WorkerMessage::Process { index: i, message });
    }

    // Signal that all messages have been collected
    messages_done.store(true, Ordering::SeqCst);
    let _ = tx_to_worker.send(WorkerMessage::AllMessagesCollected);

    println!();
    println!("{}", "All messages collected. Processing...".bold());
    println!();

    // Now wait for worker to finish, showing progress
    let mut completed_count = 0;
    let mut failed_count = 0;
    let total_items = messages.iter().filter(|m| m.is_some()).count()
        + skip_items.iter().filter(|&&s| !s).count().saturating_sub(messages.iter().filter(|m| m.is_some()).count());

    // Check for items that completed during message collection (show summary)
    {
        let committed = committed_packages.lock().unwrap();
        if !committed.is_empty() {
            println!("{}", "Background processing completed:".dimmed());
            for (name, _version) in committed.iter() {
                if let Some(item) = queue.iter().find(|i| &i.name == name) {
                    println!("  {} {} {}", "✓".green(), item.name.bold(), "complete".green());
                    completed_count += 1;
                }
            }
            println!();
        }
    }

    loop {
        match rx_from_worker.recv_timeout(Duration::from_millis(100)) {
            Ok(WorkerStatus::Started(idx, deps_info)) => {
                let item = &queue[idx];
                format::step(completed_count + 1, total_items, &format!("Processing {}", item.name));
                if !deps_info.is_empty() {
                    println!("  {} {}", "→".blue(), deps_info.dimmed());
                }
            }
            Ok(WorkerStatus::Completed(idx)) => {
                let item = &queue[idx];
                println!("  {} {} {}", "✓".green(), item.name.bold(), "complete".green());
                completed_count += 1;
            }
            Ok(WorkerStatus::Failed(idx, err)) => {
                let item = &queue[idx];
                println!("  {} {} failed: {}", "✗".red(), item.name.bold(), err);
                failed_count += 1;
                // Continue with other items
            }
            Ok(WorkerStatus::WaitingForDeps(idx, dep)) => {
                let item = &queue[idx];
                println!("  {} {} waiting for {} to be committed...", "⏳".yellow(), item.name, dep.cyan());
            }
            Ok(WorkerStatus::WaitingForRegistry(idx, pkg, registry)) => {
                let item = &queue[idx];
                println!("  {} {} waiting for {} on {}...", "⏳".yellow(), item.name, pkg.cyan(), registry);
            }
            Ok(WorkerStatus::Done) => {
                break;
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                // Check if interrupted
                if interrupted.load(Ordering::SeqCst) {
                    let _ = tx_to_worker.send(WorkerMessage::Shutdown);
                    break;
                }
                continue;
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                break;
            }
        }
    }

    // Wait for worker thread
    let _ = worker_handle.join();

    println!();
    if failed_count > 0 {
        format::error(&format!("{} packages failed, {} succeeded", failed_count, completed_count));
        anyhow::bail!("{} packages failed to commit/publish", failed_count);
    } else {
        format::success("All commits complete");
    }

    Ok(())
}

/// Worker thread that processes items in order
fn worker_thread(
    rx: Receiver<WorkerMessage>,
    tx: Sender<WorkerStatus>,
    queue: Vec<CommitItem>,
    versions: crate::core::versions::VersionsCache,
    npm_names: HashMap<String, String>,
    crate_names: HashMap<String, String>,
    interrupted: Arc<AtomicBool>,
    committed_packages: Arc<Mutex<HashMap<String, String>>>,
    registry_ready: Arc<Mutex<std::collections::HashSet<String>>>,
    current_processing: Arc<AtomicUsize>,
    messages_done: Arc<AtomicBool>,
) {
    let mut pending_items: HashMap<usize, String> = HashMap::new();
    let mut processed: Vec<bool> = vec![false; queue.len()];
    let mut skipped_names: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut failed_names: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut all_messages_collected = false;

    loop {
        if interrupted.load(Ordering::SeqCst) {
            break;
        }

        // Receive messages (non-blocking if we have pending work, blocking otherwise)
        let timeout = if pending_items.is_empty() && !all_messages_collected {
            Duration::from_secs(60) // Wait longer if nothing to do
        } else {
            Duration::from_millis(10) // Quick check if we have work
        };

        match rx.recv_timeout(timeout) {
            Ok(WorkerMessage::Process { index, message }) => {
                pending_items.insert(index, message);
            }
            Ok(WorkerMessage::Skip { index }) => {
                // Mark skipped items as processed so we don't wait for them
                processed[index] = true;
                // Track skipped item names for dependency resolution
                skipped_names.insert(queue[index].name.clone());
            }
            Ok(WorkerMessage::Retag { index }) => {
                // Queue retag operation (treated like a process but no commit)
                pending_items.insert(index, String::new()); // Empty message signals retag-only
            }
            Ok(WorkerMessage::AllMessagesCollected) => {
                all_messages_collected = true;
            }
            Ok(WorkerMessage::Shutdown) => {
                break;
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                // Continue processing
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                break;
            }
        }

        // Process items in order (respecting dependencies)
        for i in 0..queue.len() {
            if processed[i] {
                continue;
            }

            // Check if we have a message for this item
            let message = match pending_items.get(&i) {
                Some(msg) => msg.clone(),
                None => continue,
            };

            let item = &queue[i];

            // Check if any dependency failed - if so, fail this item too
            let failed_dep = item.dependencies.iter().find(|dep| failed_names.contains(*dep));
            if let Some(dep) = failed_dep {
                processed[i] = true;
                pending_items.remove(&i);
                failed_names.insert(item.name.clone());
                let _ = tx.send(WorkerStatus::Failed(
                    i,
                    format!("dependency '{}' failed", dep),
                ));
                continue;
            }

            // Check if dependencies are satisfied
            let deps_satisfied = {
                let committed = committed_packages.lock().unwrap();
                item.dependencies.iter().all(|dep| {
                    // Dependency is satisfied if:
                    // 1. We've committed it in this session, OR
                    // 2. It's not in our queue (already published), OR
                    // 3. It was skipped (no changes to commit)
                    committed.contains_key(dep)
                        || !queue.iter().any(|q| q.name == *dep)
                        || skipped_names.contains(dep)
                })
            };

            if !deps_satisfied {
                // Wait for dependencies - but only report if messages are done
                if messages_done.load(Ordering::SeqCst) {
                    for dep in &item.dependencies {
                        let committed = committed_packages.lock().unwrap();
                        // Only report waiting if dep is in queue, not committed, and not skipped
                        if !committed.contains_key(dep)
                            && queue.iter().any(|q| q.name == *dep)
                            && !skipped_names.contains(dep)
                        {
                            let _ = tx.send(WorkerStatus::WaitingForDeps(i, dep.clone()));
                            break;
                        }
                    }
                }
                continue;
            }

            // Process this item
            current_processing.store(i, Ordering::SeqCst);

            // Build deps info for display (only show when messages are done)
            let deps_info = if item.dependencies.is_empty() || !messages_done.load(Ordering::SeqCst) {
                String::new()
            } else {
                let committed = committed_packages.lock().unwrap();
                let ready = registry_ready.lock().unwrap();
                let dep_status: Vec<String> = item.dependencies.iter().map(|dep| {
                    if ready.contains(dep) {
                        format!("{} ✓", dep)
                    } else if committed.contains_key(dep) {
                        format!("{} (waiting)", dep)
                    } else if skipped_names.contains(dep) {
                        format!("{} (skipped)", dep)
                    } else {
                        format!("{} (pending)", dep)
                    }
                }).collect();
                format!("deps: {}", dep_status.join(", "))
            };
            // Only send Started status when messages are done (avoid interrupting user input)
            if messages_done.load(Ordering::SeqCst) {
                let _ = tx.send(WorkerStatus::Started(i, deps_info));
            }

            // Empty message means retag-only mode
            let result = if message.is_empty() {
                retag_item(item)
            } else {
                process_item(
                    item,
                    &message,
                    &versions,
                    &npm_names,
                    &crate_names,
                    &committed_packages,
                    &registry_ready,
                    &interrupted,
                )
            };

            match result {
                Ok(()) => {
                    // Mark as committed and registry-ready (package is available after push)
                    {
                        let mut committed = committed_packages.lock().unwrap();
                        committed.insert(item.name.clone(), item.version.clone());
                    }
                    {
                        let mut ready = registry_ready.lock().unwrap();
                        ready.insert(item.name.clone());
                    }
                    processed[i] = true;
                    pending_items.remove(&i);
                    // Only send Completed status when messages are done
                    if messages_done.load(Ordering::SeqCst) {
                        let _ = tx.send(WorkerStatus::Completed(i));
                    }
                }
                Err(e) => {
                    processed[i] = true;
                    pending_items.remove(&i);
                    failed_names.insert(item.name.clone());
                    // Only send Failed status when messages are done
                    if messages_done.load(Ordering::SeqCst) {
                        let _ = tx.send(WorkerStatus::Failed(i, e.to_string()));
                    }
                }
            }

            current_processing.store(usize::MAX, Ordering::SeqCst);
        }

        // Check if we're done
        if all_messages_collected && pending_items.is_empty() && processed.iter().all(|&p| p) {
            break;
        }

        // If all messages collected but we still have pending items, they might be waiting for deps
        if all_messages_collected && !pending_items.is_empty() {
            // Small sleep to avoid busy loop
            thread::sleep(Duration::from_millis(100));
        }
    }

    let _ = tx.send(WorkerStatus::Done);
}

/// Process a single commit item
fn process_item(
    item: &CommitItem,
    message: &str,
    versions: &crate::core::versions::VersionsCache,
    npm_names: &HashMap<String, String>,
    crate_names: &HashMap<String, String>,
    committed_packages: &Arc<Mutex<HashMap<String, String>>>,
    registry_ready: &Arc<Mutex<std::collections::HashSet<String>>>,
    interrupted: &Arc<AtomicBool>,
) -> Result<()> {
    // Wait for dependencies to appear on registries
    for dep_name in &item.dependencies {
        let committed = committed_packages.lock().unwrap();
        if let Some(dep_version) = committed.get(dep_name) {
            let dep_version = dep_version.clone();
            drop(committed); // Release lock before waiting

            // Check if already verified on registry
            {
                let ready = registry_ready.lock().unwrap();
                if ready.contains(dep_name) {
                    continue; // Already verified
                }
            }

            wait_for_package(
                dep_name,
                &dep_version,
                npm_names,
                crate_names,
                interrupted,
            )?;

            // Mark dependency as registry-ready after successful wait
            {
                let mut ready = registry_ready.lock().unwrap();
                ready.insert(dep_name.clone());
            }
        }
    }

    // Sync lockfile for npm packages
    if item.path.join("package-lock.json").exists() {
        let committed = committed_packages.lock().unwrap();
        sync_lockfile(item, &item.dependencies, versions, &committed, npm_names)?;
    }

    // Sync Cargo.lock for Rust crates
    if item.path.join("Cargo.toml").exists() {
        let committed = committed_packages.lock().unwrap();
        sync_cargo_lock(item, &item.dependencies, &committed, crate_names)?;
    }

    // Check for changes
    let status = shell::git::status(&item.path)?;
    let has_changes = !status.trim().is_empty();
    let debug = std::env::var("DEBUG_COMMIT").is_ok();

    if debug {
        eprintln!("  [debug] {} has_changes={}, status={:?}", item.name, has_changes, status.trim());
    }

    // Stage and commit changes (only if there are changes)
    if has_changes {
        shell::git::add_all(&item.path)?;
        shell::git::commit(&item.path, message)?;
        if debug {
            eprintln!("  [debug] {} committed", item.name);
        }
    }

    // Create tag if needed (force to overwrite local tag if it exists)
    if should_create_tag(item, interrupted)? {
        let tag = format!("v{}", item.version);
        shell::git::tag_force(&item.path, &tag)?;
        if debug {
            eprintln!("  [debug] {} created tag {}", item.name, tag);
        }
    }

    // Push
    if shell::git::has_upstream(&item.path) {
        shell::git::push(&item.path)?;
    } else if let Some(branch) = shell::git::current_branch(&item.path) {
        shell::git::push_with_upstream(&item.path, &branch)?;
    } else {
        shell::git::push(&item.path)?;
    }
    shell::git::push_tags(&item.path)?;
    if debug {
        eprintln!("  [debug] {} pushed", item.name);
    }

    Ok(())
}

/// Check if retagging makes sense (version not on registry)
fn can_retag_item(item: &CommitItem) -> bool {
    // If version is already on npm, no point retagging
    if let Some(ref npm_name) = item.npm_name {
        if let Ok(Some(v)) = registry::npm_version(npm_name) {
            if v == item.version {
                return false;
            }
        }
    }

    // If version is already on crates.io, no point retagging
    if let Some(ref crate_name) = item.crate_name {
        if let Ok(Some(v)) = registry::crates_version(crate_name) {
            if v == item.version {
                return false;
            }
        }
    }

    // For core package, also check platform binary packages
    if item.name == "core" {
        for platform in config::PLATFORMS {
            let bin_pkg = format!("@macroforge/bin-{}", platform);
            if let Ok(Some(v)) = registry::npm_version(&bin_pkg) {
                if v == item.version {
                    // At least one platform package is already published
                    return false;
                }
            }
        }
    }

    // Version not on registry, retagging might help trigger CI
    true
}

/// Retag an item (force recreate tag and push)
fn retag_item(item: &CommitItem) -> Result<()> {
    let tag = format!("v{}", item.version);

    // Create/overwrite local tag
    shell::git::tag_force(&item.path, &tag)?;

    // Force push the tag to remote (overwrites existing remote tag)
    shell::git::push_tag_force(&item.path, &tag)?;

    Ok(())
}

/// Check if a repo has been prepped (local version differs from registry)
/// For repos without version tracking, returns true (always include)
fn is_prepped(versions: &crate::core::versions::VersionsCache, name: &str) -> bool {
    match versions.versions.get(name).and_then(|v| v.as_ref()) {
        Some(v) => v.local != v.registry,
        None => true, // No version tracking = always include (e.g., zed-extensions)
    }
}

/// Check if a repo has a publishable package
fn has_package(item: &CommitItem) -> bool {
    item.npm_name.is_some() || item.crate_name.is_some()
}

/// Generate default commit message
fn default_commit_message(_repo_name: &str, version: &str, status: &str, has_package: bool) -> String {
    if has_package {
        format!("Bump to {}", version)
    } else {
        // For repos without packages (tooling, website, zed-extensions),
        // show "Updated" with the list of changed files
        let files: Vec<&str> = status
            .lines()
            .filter_map(|line| {
                // Git status --short format: "XY filename"
                if line.len() > 3 {
                    Some(line[3..].trim())
                } else {
                    None
                }
            })
            .collect();

        if files.is_empty() {
            "Updated".to_string()
        } else if files.len() <= 3 {
            format!("Updated {}", files.join(", "))
        } else {
            format!("Updated {} files", files.len())
        }
    }
}

/// Cascade to dependent packages
fn cascade_to_dependents(deps: &HashMap<String, Vec<String>>, initial: &[String]) -> Vec<String> {
    let mut result = std::collections::HashSet::new();
    let mut to_process: Vec<String> = initial.to_vec();

    while let Some(repo) = to_process.pop() {
        if result.insert(repo.clone()) {
            // Add all dependents
            for dependent in deps::get_dependents(deps, &repo) {
                to_process.push(dependent);
            }
        }
    }

    // Return in topological order
    if let Ok(order) = deps::topo_order(deps) {
        order.into_iter().filter(|r| result.contains(r)).collect()
    } else {
        result.into_iter().collect()
    }
}


/// Determine if we should create a tag for this repo/version
/// Tag if version is NOT on registry (even if tag exists - retag to retry CI)
fn should_create_tag(item: &CommitItem, _interrupted: &Arc<AtomicBool>) -> Result<bool> {
    let tag = format!("v{}", item.version);
    let debug = std::env::var("DEBUG_COMMIT").is_ok();

    // Check if version exists on registries - if so, no need to tag
    let on_npm = if let Some(ref npm_name) = item.npm_name {
        match registry::npm_version(npm_name) {
            Ok(Some(v)) if v == item.version => {
                if debug {
                    eprintln!("  [debug] {} version {} already on npm, skipping tag", item.name, item.version);
                }
                true
            }
            _ => false,
        }
    } else {
        true // No npm package, consider "satisfied"
    };

    let on_crates = if let Some(ref crate_name) = item.crate_name {
        match registry::crates_version(crate_name) {
            Ok(Some(v)) if v == item.version => {
                if debug {
                    eprintln!("  [debug] {} version {} already on crates.io, skipping tag", item.name, item.version);
                }
                true
            }
            _ => false,
        }
    } else {
        true // No crate, consider "satisfied"
    };

    // If both registries are satisfied (or N/A), don't tag
    if on_npm && on_crates {
        return Ok(false);
    }

    // Version not on registry - need to tag (even if tag exists, to retry CI)
    let tag_exists = shell::git::tag_exists_remote(&item.path, &tag);
    if debug {
        if tag_exists {
            eprintln!("  [debug] {} tag {} exists but not on registry, will retag", item.name, tag);
        } else {
            eprintln!("  [debug] {} will create tag {}", item.name, tag);
        }
    }
    Ok(true)
}

/// Wait for a package to be available on the registry
fn wait_for_package(
    repo_name: &str,
    version: &str,
    npm_names: &HashMap<String, String>,
    crate_names: &HashMap<String, String>,
    interrupted: &Arc<AtomicBool>,
) -> Result<()> {
    let npm_name = npm_names.get(repo_name);
    let crate_name = crate_names.get(repo_name);

    // No registry to check
    if npm_name.is_none() && crate_name.is_none() {
        return Ok(());
    }

    // Wait for crates.io first if applicable (usually faster)
    if let Some(crate_name) = crate_name {
        wait_for_registry(
            crate_name,
            version,
            "crates.io",
            || registry::crates_version(crate_name).ok().flatten().as_deref() == Some(version),
            Duration::from_secs(600),
            Duration::from_secs(10),
            interrupted,
        )?;
    }

    // Wait for npm if applicable
    if let Some(npm_name) = npm_name {
        wait_for_registry(
            npm_name,
            version,
            "npm",
            || registry::npm_version(npm_name).ok().flatten().as_deref() == Some(version),
            Duration::from_secs(600),
            Duration::from_secs(10),
            interrupted,
        )?;
    }

    Ok(())
}

/// Wait for a specific registry
fn wait_for_registry<F>(
    package_name: &str,
    version: &str,
    registry: &str,
    check_fn: F,
    timeout: Duration,
    poll_interval: Duration,
    interrupted: &Arc<AtomicBool>,
) -> Result<()>
where
    F: Fn() -> bool,
{
    print!(
        "  {} Waiting for {}@{} on {}... ",
        "⏳".yellow(),
        package_name.cyan(),
        version.cyan(),
        registry
    );
    io::stdout().flush()?;

    let start = std::time::Instant::now();
    let mut dots = 0;

    loop {
        if interrupted.load(Ordering::SeqCst) {
            println!("{}", "interrupted".yellow());
            anyhow::bail!("Interrupted by user");
        }

        if check_fn() {
            println!("{}{}", ".".repeat(dots), "available!".green());
            return Ok(());
        }

        if start.elapsed() > timeout {
            println!("{}{}", ".".repeat(dots), "timeout".red());
            anyhow::bail!(
                "Timeout waiting for {}@{} on {}",
                package_name,
                version,
                registry
            );
        }

        dots += 1;
        print!(".");
        io::stdout().flush()?;
        std::thread::sleep(poll_interval);
    }
}

/// Sync npm lockfile after dependencies are available
fn sync_lockfile(
    item: &CommitItem,
    dependencies: &[String],
    versions: &crate::core::versions::VersionsCache,
    committed_packages: &HashMap<String, String>,
    npm_names: &HashMap<String, String>,
) -> Result<()> {
    let mut packages: Vec<String> = Vec::new();

    // Force install registry versions of ALL dependencies to break local links
    for dep_repo in dependencies {
        if let Some(pkg_name) = npm_names.get(dep_repo) {
            // If we just committed it, use local (new) version
            // Otherwise, use registry (stable) version
            let target_ver = if committed_packages.contains_key(dep_repo) {
                versions.get_local(dep_repo).unwrap_or("latest")
            } else {
                versions.get_registry(dep_repo).unwrap_or("latest")
            };
            packages.push(format!("{}@{}", pkg_name, target_ver));
        }
    }

    let cmd = if packages.is_empty() {
        "npm install --ignore-scripts --no-audit".to_string()
    } else {
        format!("npm install --ignore-scripts --no-audit {}", packages.join(" "))
    };

    shell::run(&cmd, &item.path, false)?;
    Ok(())
}

/// Sync Cargo.lock after dependencies are published to crates.io
fn sync_cargo_lock(
    item: &CommitItem,
    dependencies: &[String],
    committed_packages: &HashMap<String, String>,
    crate_names: &HashMap<String, String>,
) -> Result<()> {
    for dep_repo in dependencies {
        if let Some(crate_name) = crate_names.get(dep_repo) {
            // Only update if this dependency was just committed (published)
            if committed_packages.contains_key(dep_repo) {
                // Update Cargo.lock to fetch the newly published version from crates.io
                shell::run(
                    &format!("cargo update -p {}", crate_name),
                    &item.path,
                    false,
                )?;
            }
        }
    }
    Ok(())
}
