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
    /// Push unpushed commits without creating a new commit
    PushOnly { index: usize },
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
    /// Worker finished all work
    Done,
}

/// Shared context for worker thread operations
struct WorkerContext {
    queue: Vec<CommitItem>,
    versions: crate::core::versions::VersionsCache,
    npm_names: HashMap<String, String>,
    crate_names: HashMap<String, String>,
    interrupted: Arc<AtomicBool>,
    committed_packages: Arc<Mutex<HashMap<String, String>>>,
    registry_ready: Arc<Mutex<std::collections::HashSet<String>>>,
    failed_packages: Arc<Mutex<HashMap<String, String>>>,
    processing_name: Arc<Mutex<Option<String>>>,
    messages_done: Arc<AtomicBool>,
    /// Buffer for output messages during input collection phase
    output_buffer: Arc<Mutex<Vec<String>>>,
}

impl WorkerContext {
    /// Log a message - buffers if messages not done, prints immediately otherwise
    fn log(&self, msg: &str) {
        if self.messages_done.load(Ordering::SeqCst) {
            println!("{}", msg);
        } else {
            let mut buffer = self.output_buffer.lock().unwrap();
            buffer.push(msg.to_string());
        }
    }

    /// Flush buffered messages to stdout
    fn flush_buffer(&self) {
        let mut buffer = self.output_buffer.lock().unwrap();
        for msg in buffer.drain(..) {
            println!("{}", msg);
        }
    }
}

pub fn run(args: CommitArgs) -> Result<()> {
    let config = Config::load()?;
    let mut versions = config.versions.clone();
    let mut repos = config.filter_repos(&args.repos);

    // Refresh registry versions for repos we're processing to avoid stale cache issues
    let npm_names = config::npm_package_names();
    let crate_names = config::crate_package_names();
    for repo in &repos {
        if let Some(npm_name) = npm_names.get(repo.name.as_str())
            && let Ok(Some(v)) = registry::npm_version(npm_name)
        {
            versions.set_registry(&repo.name, &v);
        }
        if let Some(crate_name) = crate_names.get(repo.name.as_str())
            && let Ok(Some(v)) = registry::crates_version(crate_name)
        {
            versions.set_registry(&repo.name, &v);
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
    let failed_packages: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    let processing_name: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

    // Clones for status display in main thread
    let committed_for_display = Arc::clone(&committed_packages);
    let failed_for_display = Arc::clone(&failed_packages);
    let processing_for_display = Arc::clone(&processing_name);

    // Clone for worker thread
    let output_buffer: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let ctx = WorkerContext {
        queue: queue.clone(),
        versions: versions.clone(),
        npm_names: npm_names.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
        crate_names: crate_names.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
        interrupted: Arc::clone(&interrupted),
        committed_packages: Arc::clone(&committed_packages),
        registry_ready: Arc::clone(&registry_ready),
        failed_packages: Arc::clone(&failed_packages),
        processing_name: Arc::clone(&processing_name),
        messages_done: Arc::clone(&messages_done),
        output_buffer,
    };
    let current_processing_clone = Arc::clone(&current_processing);

    // Spawn worker thread
    let worker_handle = thread::spawn(move || {
        worker_thread(
            rx_from_main,
            tx_to_main,
            ctx,
            current_processing_clone,
        )
    });

    // Helper to display background status
    let display_background_status = |committed: &Arc<Mutex<HashMap<String, String>>>,
                                      failed: &Arc<Mutex<HashMap<String, String>>>,
                                      processing: &Arc<Mutex<Option<String>>>| {
        let committed = committed.lock().unwrap();
        let failed = failed.lock().unwrap();
        let processing = processing.lock().unwrap();

        if committed.is_empty() && failed.is_empty() && processing.is_none() {
            return;
        }

        println!("{}", "─".repeat(50).dimmed());
        print!("  {} ", "background:".dimmed());

        let mut parts: Vec<String> = Vec::new();
        if !committed.is_empty() {
            parts.push(format!("{} {}", committed.len().to_string().green(), "done".green()));
        }
        if let Some(ref name) = *processing {
            parts.push(format!("{} {}", "→".yellow(), name.yellow()));
        }
        if !failed.is_empty() {
            parts.push(format!("{} {}", failed.len().to_string().red(), "failed".red()));
        }
        println!("{}", parts.join(" │ "));
        println!("{}", "─".repeat(50).dimmed());
    };

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
        let changed_files: Vec<&str> = status.lines().collect();

        // Display background status
        display_background_status(&committed_for_display, &failed_for_display, &processing_for_display);

        // Display verbose info for this repo
        println!();
        println!("{}", format!("[{}/{}] {}", i + 1, queue.len(), item.name.cyan().bold()));
        println!("  {} {}", "version:".dimmed(), item.version.green());
        if let Some(ref npm) = item.npm_name {
            println!("  {} {}", "npm:".dimmed(), npm);
        }
        if let Some(ref crate_name) = item.crate_name {
            println!("  {} {}", "crate:".dimmed(), crate_name);
        }
        if !item.dependencies.is_empty() {
            println!("  {} {}", "deps:".dimmed(), item.dependencies.join(", "));
        }
        println!("  {} {}, {} unpushed",
            "status:".dimmed(),
            if has_changes { format!("{} changed", changed_files.len()).yellow() } else { "clean".green() },
            if unpushed_count > 0 { unpushed_count.to_string().yellow() } else { "0".green() }
        );
        if has_changes && changed_files.len() <= 5 {
            for file in &changed_files {
                println!("    {}", file.dimmed());
            }
        }

        if !has_changes && unpushed_count == 0 {
            // No changes, no unpushed - check if retagging makes sense
            let tag = format!("v{}", item.version);
            let can_retag = can_retag_item(item);

            if can_retag && Confirm::new()
                .with_prompt(format!("  No changes. Retag {}?", tag.cyan()))
                .default(false)
                .interact()?
            {
                let _ = tx_to_worker.send(WorkerMessage::Retag { index: i });
            } else {
                if !can_retag {
                    println!("  {} version already published", "→".dimmed());
                }
                skip_items[i] = true;
                let _ = tx_to_worker.send(WorkerMessage::Skip { index: i });
            }
            continue;
        }

        if !has_changes && unpushed_count > 0 {
            // No uncommitted changes, but has unpushed commits - just confirm push
            let tag = format!("v{}", item.version);
            if args.yes || Confirm::new()
                .with_prompt(format!("  Push {} commits and tag {}?", unpushed_count, tag.cyan()))
                .default(true)
                .interact()?
            {
                let _ = tx_to_worker.send(WorkerMessage::PushOnly { index: i });
            } else {
                skip_items[i] = true;
                let _ = tx_to_worker.send(WorkerMessage::Skip { index: i });
            }
            continue;
        }

        // Has uncommitted changes - collect commit message
        let message = if let Some(ref msg) = args.message {
            msg.clone()
        } else {
            // Interactive prompt
            let default_msg = default_commit_message(&item.name, &item.version, &status, has_package(item));
            Input::new()
                .with_prompt("  commit message")
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

/// Pending operation type for worker thread
#[derive(Debug, Clone)]
enum PendingOp {
    /// Commit with message, tag, push
    Commit(String),
    /// Push existing commits, tag, push (no new commit)
    PushOnly,
    /// Just retag and push tag
    Retag,
}

/// Worker thread that processes items in order
fn worker_thread(
    rx: Receiver<WorkerMessage>,
    tx: Sender<WorkerStatus>,
    ctx: WorkerContext,
    current_processing: Arc<AtomicUsize>,
) {
    let mut pending_items: HashMap<usize, PendingOp> = HashMap::new();
    let mut processed: Vec<bool> = vec![false; ctx.queue.len()];
    let mut skipped_names: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut failed_names: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut all_messages_collected = false;

    loop {
        if ctx.interrupted.load(Ordering::SeqCst) {
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
                pending_items.insert(index, PendingOp::Commit(message));
            }
            Ok(WorkerMessage::PushOnly { index }) => {
                pending_items.insert(index, PendingOp::PushOnly);
            }
            Ok(WorkerMessage::Skip { index }) => {
                // Mark skipped items as processed so we don't wait for them
                processed[index] = true;
                // Track skipped item names for dependency resolution
                skipped_names.insert(ctx.queue[index].name.clone());
            }
            Ok(WorkerMessage::Retag { index }) => {
                pending_items.insert(index, PendingOp::Retag);
            }
            Ok(WorkerMessage::AllMessagesCollected) => {
                all_messages_collected = true;
                // Flush any buffered output now that input collection is done
                ctx.flush_buffer();
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

        // Process items STRICTLY in queue order - find the next unprocessed item
        let next_to_process = processed.iter().position(|&p| !p);

        if let Some(i) = next_to_process {
            let item = &ctx.queue[i];

            // Check if we have an operation pending for this item (user provided input)
            let op = match pending_items.get(&i) {
                Some(op) => op.clone(),
                None => continue, // Not ready yet, wait for user input
            };

            // Check if any dependency failed - if so, fail this item too
            let failed_dep = item.dependencies.iter().find(|dep| failed_names.contains(*dep));
            if let Some(dep) = failed_dep {
                processed[i] = true;
                pending_items.remove(&i);
                failed_names.insert(item.name.clone());
                {
                    let mut failed = ctx.failed_packages.lock().unwrap();
                    failed.insert(item.name.clone(), format!("dependency '{}' failed", dep));
                }
                let _ = tx.send(WorkerStatus::Failed(
                    i,
                    format!("dependency '{}' failed", dep),
                ));
                continue;
            }

            // It's this item's turn - process it now
            current_processing.store(i, Ordering::SeqCst);

            // Build deps info for display (only show when messages are done)
            let deps_info = if item.dependencies.is_empty() || !ctx.messages_done.load(Ordering::SeqCst) {
                String::new()
            } else {
                let committed = ctx.committed_packages.lock().unwrap();
                let ready = ctx.registry_ready.lock().unwrap();
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
            if ctx.messages_done.load(Ordering::SeqCst) {
                let _ = tx.send(WorkerStatus::Started(i, deps_info));
            }

            // Set processing name for live status display
            {
                let mut proc = ctx.processing_name.lock().unwrap();
                *proc = Some(item.name.clone());
            }

            // Execute based on operation type
            let result = match op {
                PendingOp::Commit(ref message) => process_item(item, message, &ctx),
                PendingOp::PushOnly => push_only_item(item, &ctx),
                PendingOp::Retag => retag_item(item, &ctx),
            };

            // Clear processing name
            {
                let mut proc = ctx.processing_name.lock().unwrap();
                *proc = None;
            }

            match result {
                Ok(()) => {
                    // Mark as committed (but NOT registry-ready - that's only set after actual verification)
                    {
                        let mut committed = ctx.committed_packages.lock().unwrap();
                        committed.insert(item.name.clone(), item.version.clone());
                    }
                    processed[i] = true;
                    pending_items.remove(&i);
                    // Only send Completed status when messages are done
                    if ctx.messages_done.load(Ordering::SeqCst) {
                        let _ = tx.send(WorkerStatus::Completed(i));
                    }
                }
                Err(e) => {
                    processed[i] = true;
                    pending_items.remove(&i);
                    failed_names.insert(item.name.clone());
                    // Track failed packages for live status display
                    {
                        let mut failed = ctx.failed_packages.lock().unwrap();
                        failed.insert(item.name.clone(), e.to_string());
                    }
                    // Only send Failed status when messages are done
                    if ctx.messages_done.load(Ordering::SeqCst) {
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

/// Wait for dependencies and sync lockfiles - shared logic for all item processing
fn wait_and_sync(item: &CommitItem, ctx: &WorkerContext) -> Result<()> {
    // Wait for ALL dependencies to appear on registries at expected versions
    if !item.dependencies.is_empty() {
        ctx.log(&format!("    {} Waiting for {} dependencies...", "⏳".yellow(), item.dependencies.len()));
    }

    for dep_name in &item.dependencies {
        // Skip if already verified
        {
            let ready = ctx.registry_ready.lock().unwrap();
            if ready.contains(dep_name) {
                ctx.log(&format!("    {} {} already verified", "✓".green(), dep_name));
                continue;
            }
        }

        // Get expected version for this dependency
        let dep_version = ctx.versions.get_local(dep_name)
            .map(|s| s.to_string())
            .unwrap_or_else(|| "latest".to_string());

        wait_for_package(
            dep_name,
            &dep_version,
            &ctx.npm_names,
            &ctx.crate_names,
            &ctx.interrupted,
            ctx,
        )?;

        // Mark as verified
        {
            let mut ready = ctx.registry_ready.lock().unwrap();
            ready.insert(dep_name.clone());
        }
    }

    // Sync lockfile for npm packages (check for package.json, not lockfile)
    if item.path.join("package.json").exists() {
        ctx.log(&format!("    {} Syncing npm lockfile...", "📦".cyan()));
        sync_lockfile(item, ctx)?;
    }

    // Sync Cargo.lock for Rust crates
    if item.path.join("Cargo.toml").exists() {
        ctx.log(&format!("    {} Syncing Cargo.lock...", "🦀".cyan()));
        sync_cargo_lock(item)?;
    }

    Ok(())
}

/// Process a single commit item
fn process_item(
    item: &CommitItem,
    message: &str,
    ctx: &WorkerContext,
) -> Result<()> {
    ctx.log(&format!("  {} Processing {}", "→".blue(), item.name.bold()));

    // Wait for deps and sync lockfiles
    wait_and_sync(item, ctx)?;

    // Check for changes
    let status = shell::git::status(&item.path)?;
    let has_changes = !status.trim().is_empty();

    // Stage and commit changes (only if there are changes)
    if has_changes {
        ctx.log(&format!("    {} Committing: {}", "📝".cyan(), message.dimmed()));
        shell::git::add_all(&item.path)?;
        shell::git::commit(&item.path, message)?;
    } else {
        ctx.log(&format!("    {} No changes to commit", "─".dimmed()));
    }

    // Create tag if needed (force to overwrite local tag if it exists)
    if should_create_tag(item, &ctx.interrupted)? {
        let tag = format!("v{}", item.version);
        ctx.log(&format!("    {} Creating tag {}", "🏷".cyan(), tag.cyan()));
        shell::git::tag_force(&item.path, &tag)?;
    }

    // Push
    ctx.log(&format!("    {} Pushing to remote...", "🚀".cyan()));
    if shell::git::has_upstream(&item.path) {
        shell::git::push(&item.path)?;
    } else if let Some(branch) = shell::git::current_branch(&item.path) {
        shell::git::push_with_upstream(&item.path, &branch)?;
    } else {
        shell::git::push(&item.path)?;
    }
    shell::git::push_tags(&item.path)?;
    ctx.log(&format!("    {} Done", "✓".green()));

    Ok(())
}

/// Check if retagging makes sense (version not on registry)
fn can_retag_item(item: &CommitItem) -> bool {
    // Non-published items (website, zed-extensions, tooling) should never be retagged
    if item.npm_name.is_none() && item.crate_name.is_none() {
        return false;
    }

    // If version is already on npm, no point retagging
    if let Some(ref npm_name) = item.npm_name
        && let Ok(Some(v)) = registry::npm_version(npm_name)
        && v == item.version
    {
        return false;
    }

    // If version is already on crates.io, no point retagging
    if let Some(ref crate_name) = item.crate_name
        && let Ok(Some(v)) = registry::crates_version(crate_name)
        && v == item.version
    {
        return false;
    }

    // For core package, also check platform binary packages
    if item.name == "core" {
        for platform in config::PLATFORMS {
            let bin_pkg = format!("@macroforge/bin-{}", platform);
            if let Ok(Some(v)) = registry::npm_version(&bin_pkg)
                && v == item.version
            {
                // At least one platform package is already published
                return false;
            }
        }
    }

    // Version not on registry, retagging might help trigger CI
    true
}

/// Retag an item (force recreate tag and push)
/// Also syncs lockfiles in case that's why CI failed
fn retag_item(item: &CommitItem, ctx: &WorkerContext) -> Result<()> {
    ctx.log(&format!("  {} Processing {} (retag)", "→".blue(), item.name.bold()));

    // Wait for deps and sync lockfiles
    wait_and_sync(item, ctx)?;

    // Check if lockfile sync created changes - if so, commit them
    let status = shell::git::status(&item.path)?;
    if !status.trim().is_empty() {
        ctx.log(&format!("    {} Committing lockfile changes", "📝".cyan()));
        shell::git::add_all(&item.path)?;
        shell::git::commit(&item.path, &format!("Update lockfile for v{}", item.version))?;
    }

    let tag = format!("v{}", item.version);

    // Create/overwrite local tag
    ctx.log(&format!("    {} Creating tag {}", "🏷".cyan(), tag.cyan()));
    shell::git::tag_force(&item.path, &tag)?;

    // Push commits if any were made
    ctx.log(&format!("    {} Pushing to remote...", "🚀".cyan()));
    if shell::git::has_upstream(&item.path) {
        shell::git::push(&item.path)?;
    } else if let Some(branch) = shell::git::current_branch(&item.path) {
        shell::git::push_with_upstream(&item.path, &branch)?;
    } else {
        shell::git::push(&item.path)?;
    }

    // Force push the tag to remote (overwrites existing remote tag)
    shell::git::push_tag_force(&item.path, &tag)?;
    ctx.log(&format!("    {} Done", "✓".green()));

    Ok(())
}

/// Push existing unpushed commits, tag, and push (no new commit)
fn push_only_item(item: &CommitItem, ctx: &WorkerContext) -> Result<()> {
    ctx.log(&format!("  {} Processing {} (push only)", "→".blue(), item.name.bold()));

    // Wait for deps and sync lockfiles
    wait_and_sync(item, ctx)?;

    // Check if lockfile sync created changes - if so, commit them
    let status = shell::git::status(&item.path)?;
    if !status.trim().is_empty() {
        ctx.log(&format!("    {} Committing lockfile changes", "📝".cyan()));
        shell::git::add_all(&item.path)?;
        shell::git::commit(&item.path, &format!("Update lockfile for v{}", item.version))?;
    }

    // Create tag if needed
    if should_create_tag(item, &ctx.interrupted)? {
        let tag = format!("v{}", item.version);
        ctx.log(&format!("    {} Creating tag {}", "🏷".cyan(), tag.cyan()));
        shell::git::tag_force(&item.path, &tag)?;
    }

    // Push
    ctx.log(&format!("    {} Pushing to remote...", "🚀".cyan()));
    if shell::git::has_upstream(&item.path) {
        shell::git::push(&item.path)?;
    } else if let Some(branch) = shell::git::current_branch(&item.path) {
        shell::git::push_with_upstream(&item.path, &branch)?;
    } else {
        shell::git::push(&item.path)?;
    }
    shell::git::push_tags(&item.path)?;
    ctx.log(&format!("    {} Done", "✓".green()));

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
    ctx: &WorkerContext,
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
            ctx,
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
            ctx,
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
    ctx: &WorkerContext,
) -> Result<()>
where
    F: Fn() -> bool,
{
    let start = std::time::Instant::now();

    // Check if already available
    if check_fn() {
        ctx.log(&format!(
            "    {} {}@{} on {} {}",
            "✓".green(),
            package_name.cyan(),
            version.cyan(),
            registry,
            "available".green()
        ));
        return Ok(());
    }

    ctx.log(&format!(
        "    {} Waiting for {}@{} on {}...",
        "⏳".yellow(),
        package_name.cyan(),
        version.cyan(),
        registry
    ));

    loop {
        if interrupted.load(Ordering::SeqCst) {
            ctx.log(&format!("    {} Interrupted", "✗".red()));
            anyhow::bail!("Interrupted by user");
        }

        std::thread::sleep(poll_interval);

        if check_fn() {
            ctx.log(&format!(
                "    {} {}@{} {} ({}s)",
                "✓".green(),
                package_name.cyan(),
                version.cyan(),
                "available".green(),
                start.elapsed().as_secs()
            ));
            return Ok(());
        }

        if start.elapsed() > timeout {
            ctx.log(&format!("    {} Timeout waiting for {}@{}", "✗".red(), package_name, version));
            anyhow::bail!(
                "Timeout waiting for {}@{} on {}",
                package_name,
                version,
                registry
            );
        }
    }
}

/// Sync npm lockfile with retry logic for CDN propagation delay
fn sync_lockfile(item: &CommitItem, ctx: &WorkerContext) -> Result<()> {
    // Delete node_modules and package-lock.json to force fresh resolution
    let node_modules = item.path.join("node_modules");
    let lockfile = item.path.join("package-lock.json");
    if node_modules.exists() {
        let _ = std::fs::remove_dir_all(&node_modules);
    }
    if lockfile.exists() {
        let _ = std::fs::remove_file(&lockfile);
    }

    // Retry npm install with exponential backoff (CDN propagation can take time)
    let max_retries = 5;
    let mut attempt = 0;
    let mut last_error = None;

    while attempt < max_retries {
        attempt += 1;

        // Clear npm cache before retry to avoid stale data
        if attempt > 1 {
            let wait_secs = 10 * attempt;
            ctx.log(&format!("      {} Retry {}/{} in {}s (waiting for npm CDN)...",
                "⏳".yellow(), attempt, max_retries, wait_secs));
            thread::sleep(Duration::from_secs(wait_secs as u64));
            let _ = shell::run("npm cache clean --force", &item.path, false);
        }

        match shell::run("npm install --no-audit", &item.path, false) {
            Ok(_) => return Ok(()),
            Err(e) => {
                let err_str = e.to_string();
                // Only retry on "notarget" errors (package not found yet)
                if err_str.contains("ETARGET") || err_str.contains("notarget") || err_str.contains("No matching version") {
                    last_error = Some(e);
                    continue;
                }
                // Other errors, fail immediately
                return Err(e);
            }
        }
    }

    Err(last_error.unwrap_or_else(|| anyhow::anyhow!("npm install failed after {} retries", max_retries)))
}

/// Sync Cargo.lock after dependencies are published to crates.io
fn sync_cargo_lock(item: &CommitItem) -> Result<()> {
    // Delete Cargo.lock and regenerate fresh
    let lockfile = item.path.join("Cargo.lock");
    if lockfile.exists() {
        let _ = std::fs::remove_file(&lockfile);
    }
    shell::run("cargo generate-lockfile", &item.path, false)?;
    Ok(())
}
