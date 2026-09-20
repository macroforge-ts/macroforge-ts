use anyhow::{Context, Result};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
};

use crate::atomic_fs::{sibling_with_suffix, swap_dir, write_atomic};
use crate::cache::{compute_config_hash, compute_external_macro_hash, content_hash};
use crate::package_expand::run_expansion_pass;
use crate::package_state::{
    self, PackageFlags, PackageInputs, PackageState, ResolvedPackageConfig, diff_files,
    expanded_dir, output_fingerprint, project_hash, scan_input, state_dir, tool_hashes,
};

/// Cached path to the type registry JSON file (built once per process via scanProjectSync).
pub(crate) static TYPE_REGISTRY_CACHE_PATH: Mutex<Option<String>> = Mutex::new(None);

/// Cached path to the declarative macro registry JSON file (built in the
/// same scan as the type registry).
pub(crate) static DECLARATIVE_REGISTRY_CACHE_PATH: Mutex<Option<String>> = Mutex::new(None);

/// Builds the type registry using the native ProjectScanner (no Node.js needed)
/// and caches the result to `<root>/.macroforge/type-registry.json`.
/// Subsequent calls are no-ops.
///
/// The same scan also builds the declarative macro registry and writes it
/// to `.macroforge/declarative-registry.json` alongside the type registry,
/// so cross-file `/** import macro */` resolution works in the
/// tsc/svelte-check wrappers and the Vite plugin.
///
/// `root` is the resolved project root — the same directory the cache and the
/// project lock are keyed on. It is passed in rather than read from the current
/// directory so that `macroforge cache <elsewhere>` keeps the whole
/// `.macroforge/` tree in one place instead of splitting the registry from the
/// cache it belongs to.
///
/// Both registries are written atomically. The type registry runs to several
/// megabytes on a large project, and the Vite plugin reads it directly at
/// `buildStart`, so a plain write would give concurrent readers a wide window
/// in which to observe a truncated file.
pub(crate) fn ensure_type_registry_cache(root: &Path) {
    let mut cached = TYPE_REGISTRY_CACHE_PATH.lock().unwrap();
    if cached.is_some() {
        return;
    }

    // Write registry to project-local .macroforge/ directory (not global /tmp)
    let cache_dir = root.join(".macroforge");
    let _ = fs::create_dir_all(&cache_dir);
    let registry_path = cache_dir.join("type-registry.json");
    let declarative_path = cache_dir.join("declarative-registry.json");

    use macroforge_ts::host::scanner::{ProjectScanner, ScanConfig};

    let config = ScanConfig {
        root_dir: root.to_path_buf(),
        ..ScanConfig::default()
    };
    let scanner = ProjectScanner::new(config);

    match scanner.scan() {
        Ok(output) => {
            let types_found = output.registry.len();
            let macros_found = output.declarative_registry.macro_count();
            match serde_json::to_string(&output.registry) {
                Ok(json) => {
                    if let Err(e) = write_atomic(&registry_path, json.as_bytes()) {
                        eprintln!("[macroforge] Failed to write type registry: {e}");
                        *cached = None;
                        return;
                    }
                    eprintln!(
                        "[macroforge] Type scan: {} types from {} files",
                        types_found, output.files_scanned
                    );
                    *cached = Some(registry_path.to_string_lossy().to_string());
                }
                Err(e) => {
                    eprintln!("[macroforge] Failed to serialize type registry: {e}");
                    *cached = None;
                }
            }

            // Emit the declarative macro registry alongside the type
            // registry. Failures here are non-fatal: the type registry
            // still ships, and declarative cross-file imports just won't
            // resolve (they'll surface as diagnostics at use sites).
            match output.declarative_registry.to_json() {
                Ok(json) => {
                    if let Err(e) = write_atomic(&declarative_path, json.as_bytes()) {
                        eprintln!("[macroforge] Failed to write declarative macro registry: {e}");
                    } else {
                        eprintln!(
                            "[macroforge] Declarative scan: {} macros across {} files",
                            macros_found,
                            output.declarative_registry.file_count()
                        );
                        *DECLARATIVE_REGISTRY_CACHE_PATH.lock().unwrap() =
                            Some(declarative_path.to_string_lossy().to_string());
                    }
                }
                Err(e) => {
                    eprintln!("[macroforge] Failed to serialize declarative macro registry: {e}");
                }
            }
        }
        Err(e) => {
            eprintln!("[macroforge] Type scan failed: {e}");
            *cached = None;
        }
    }
}

/// Writes a wrapper's Node scripts into a temp directory and returns its path.
///
/// The scripts are compiled into the binary, so every macroforge process of a
/// given build writes byte-identical content. They still cannot share one fixed
/// path: two concurrent invocations would rewrite the files while the other's
/// `node` is reading them, and two *different* builds would overwrite each
/// other outright.
///
/// The directory is therefore content-addressed — `<version>-<hash>` over the
/// scripts themselves — so identical content collides harmlessly and differing
/// content never does. Each file is still written atomically, so a reader that
/// arrives mid-write sees the complete previous copy rather than a partial one.
///
/// All of a wrapper's scripts must live in one directory: the module-resolve
/// hook loads its fs shim by relative path.
fn materialize_scripts(name: &str, files: &[(&str, &str)]) -> Result<PathBuf> {
    let mut combined = String::new();
    for (filename, contents) in files {
        combined.push_str(filename);
        combined.push('\0');
        combined.push_str(contents);
        combined.push('\0');
    }
    let digest = content_hash(combined.as_bytes());

    let mut dir = std::env::temp_dir();
    dir.push("macroforge-cli");
    dir.push(format!(
        "{name}-{}-{}",
        env!("CARGO_PKG_VERSION"),
        &digest[..16]
    ));
    fs::create_dir_all(&dir).with_context(|| format!("failed to create {}", dir.display()))?;

    for (filename, contents) in files {
        write_atomic(&dir.join(filename), contents.as_bytes())?;
    }

    Ok(dir)
}

/// Runs TypeScript type checking with macro expansion baked into file reads.
///
/// This function creates a Node.js script that wraps `tsc --noEmit` behavior
/// while transparently expanding macros when reading `.ts` and `.tsx` files.
/// Files containing `@derive` are expanded before being passed to the TypeScript
/// compiler.
///
/// The wrapper intercepts `CompilerHost.getSourceFile` to:
/// 1. Read the file contents
/// 2. Check for `@derive` decorators
/// 3. Expand macros if found
/// 4. Return the expanded source to TypeScript
///
/// # Arguments
///
/// * `root` - The resolved project root (owns `.macroforge/`)
/// * `project` - Optional path to `tsconfig.json` (defaults to `tsconfig.json` in cwd)
///
/// # Returns
///
/// Returns `Ok(())` if type checking passes, or an error with diagnostic details.
pub fn run_tsc_wrapper(root: &Path, project: Option<PathBuf>) -> Result<()> {
    // Build the type registry before launching tsc so that expandSync
    // can resolve cross-module type references.
    ensure_type_registry_cache(root);
    let registry_path = TYPE_REGISTRY_CACHE_PATH.lock().unwrap().clone();
    let declarative_registry_path = DECLARATIVE_REGISTRY_CACHE_PATH.lock().unwrap().clone();

    // The tsc wrapper must be a JS script because it monkey-patches the TypeScript
    // CompilerHost.getSourceFile to expand macros on the fly — that hooking can only
    // happen in JS since it's patching the TS compiler API internals. We write it to
    // a temp file because `node` needs a file path to execute.
    let script = include_str!("../../../js/cli/tsc-wrapper.js");
    let temp_dir = materialize_scripts("tsc", &[("tsc-wrapper.js", script)])?;
    let script_path = temp_dir.join("tsc-wrapper.js");

    let project_arg = project
        .unwrap_or_else(|| PathBuf::from("tsconfig.json"))
        .to_string_lossy()
        .to_string();

    let mut cmd = std::process::Command::new("node");
    cmd.arg(script_path).arg(project_arg);
    if let Some(ref rp) = registry_path {
        cmd.env("MACROFORGE_TYPE_REGISTRY_PATH", rp);
    }
    if let Some(ref drp) = declarative_registry_path {
        cmd.env("MACROFORGE_DECLARATIVE_REGISTRY_PATH", drp);
    }
    let status = cmd.status().context("failed to run node tsc wrapper")?;

    if !status.success() {
        anyhow::bail!("tsc wrapper exited with status {}", status);
    }

    Ok(())
}

/// Runs svelte-check with macro expansion baked into file reads.
///
/// This function creates a Node.js script that patches `ts.sys.readFile` before
/// loading svelte-check. Since svelte-check uses TypeScript as a peer dependency
/// and Node.js caches modules, the patched `ts.sys.readFile` is shared with
/// svelte-check's internal TypeScript language service.
///
/// Files containing `@derive` are expanded before being passed to svelte-check.
///
/// # Arguments
///
/// * `root` - The resolved project root (owns `.macroforge/`)
/// * `workspace` - Optional workspace directory (defaults to cwd)
/// * `tsconfig` - Optional path to `tsconfig.json`
/// * `output` - Optional output format (human, human-verbose, machine, machine-verbose)
/// * `fail_on_warnings` - If true, exit with error on warnings
///
/// # Returns
///
/// Returns `Ok(())` if svelte-check passes, or an error with diagnostic details.
pub fn run_svelte_check_wrapper(
    root: &Path,
    workspace: Option<PathBuf>,
    tsconfig: Option<PathBuf>,
    output: Option<String>,
    fail_on_warnings: bool,
) -> Result<()> {
    // Build the type registry before launching svelte-check so that
    // expandSync can resolve cross-module type references (e.g., enum
    // fieldset variants defined in separate files).
    ensure_type_registry_cache(root);
    let registry_path = TYPE_REGISTRY_CACHE_PATH.lock().unwrap().clone();
    let declarative_registry_path = DECLARATIVE_REGISTRY_CACHE_PATH.lock().unwrap().clone();

    // Same pattern as tsc wrapper: must be JS because it patches svelte-check's
    // file reading to expand macros. Written to temp because `node` needs a file path.
    let script = include_str!("../../../js/cli/svelte-check-wrapper.js");
    let temp_dir = materialize_scripts("svelte-check", &[("svelte-check-wrapper.js", script)])?;
    let script_path = temp_dir.join("svelte-check-wrapper.js");

    let mut cmd = std::process::Command::new("node");
    cmd.arg(&script_path);

    // Pass the type registry path via environment variable so the JS
    // script can load it and feed it to expandSync.
    if let Some(ref rp) = registry_path {
        cmd.env("MACROFORGE_TYPE_REGISTRY_PATH", rp);
    }
    if let Some(ref drp) = declarative_registry_path {
        cmd.env("MACROFORGE_DECLARATIVE_REGISTRY_PATH", drp);
    }

    // Pass through CLI args for svelte-check to pick up
    if let Some(ref ws) = workspace {
        cmd.arg("--workspace").arg(ws);
    }
    if let Some(ref ts) = tsconfig {
        cmd.arg("--tsconfig").arg(ts);
    }
    if let Some(ref out) = output {
        cmd.arg("--output").arg(out);
    }
    if fail_on_warnings {
        cmd.arg("--fail-on-warnings");
    }

    let status = cmd
        .status()
        .context("failed to run node svelte-check wrapper")?;

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    Ok(())
}

/// What `@sveltejs/package` reports about options only `svelte.config.js` knows.
#[derive(serde::Deserialize)]
struct ConfigProbe {
    input: PathBuf,
    extensions: Vec<String>,
}

/// Asks the wrapper which options the packager would actually run with.
///
/// The input directory defaults to `config.kit.files.lib` and the Svelte
/// extensions to `config.extensions`, both of which live in a JavaScript module
/// that only Node can evaluate. Spawning it costs a Node startup, which is most
/// of the up-to-date path's budget, so the answer is cached in the build state
/// and this runs only when one of its inputs changes.
fn probe_package_config(main_path: &Path, flags: &PackageFlags) -> Result<ConfigProbe> {
    let mut cmd = std::process::Command::new("node");
    cmd.arg(main_path).arg("--macroforge-resolve-config");
    if let Some(ref i) = flags.input {
        cmd.arg("--input").arg(i);
    }

    let output = cmd
        .output()
        .context("failed to run the svelte-package config probe")?;

    if !output.status.success() {
        // The probe reports a missing @sveltejs/package itself, in the same
        // words the build would have used.
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(output.status.code().unwrap_or(1));
    }

    serde_json::from_slice(&output.stdout)
        .context("the svelte-package config probe returned unreadable output")
}

/// Resolves `path` against `root`, and against the real filesystem when it can.
///
/// Canonicalizing matters because these paths are compared across runs and
/// against the probe's own `path.resolve`, and a project reached through a
/// symlink would otherwise stamp a different string every time it is invoked
/// from a different direction.
fn absolutize(root: &Path, path: &Path) -> PathBuf {
    let joined = if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    };
    joined.canonicalize().unwrap_or(joined)
}

/// Runs `@sveltejs/package` over macro-expanded sources.
///
/// Produces a published package whose `.ts`/`.svelte.ts` type modules ship the
/// generated derive runtime (and correct `.d.ts`), with no separate expand step
/// or staging tree. Expansion happens here, once per changed file and in
/// parallel, into a tree the Node wrapper redirects the packager's reads
/// through — the packager itself still runs against the real input directory,
/// so `$lib` aliases and `.d.ts.map` sources keep pointing at the real sources.
///
/// The run is incremental. Build state from the previous run records what it
/// consumed and produced, and a run whose inputs all match it has nothing to do;
/// files whose only change is formatting do not count. `full_rebuild` discards
/// all of that and rebuilds from nothing.
///
/// The output directory is swapped into place rather than rebuilt in situ.
/// `@sveltejs/package` finishes by deleting the output directory and recursively
/// copying its own staging tree into it, which leaves anything reading the
/// package — a watching bundler, an editor, a publish step — looking at a
/// half-populated tree for the duration of the copy, and leaves it deleted
/// outright if the build fails. Redirecting the packager at a scratch directory
/// and renaming that into place reduces the exposure to a single `rename` and
/// keeps a failed build from destroying the previous output.
pub fn run_svelte_package_wrapper(
    root: &Path,
    input: Option<PathBuf>,
    output: Option<PathBuf>,
    tsconfig: Option<PathBuf>,
    no_types: bool,
    full_rebuild: bool,
) -> Result<()> {
    // The wrapper is three ES modules: the entrypoint, a module-resolve hook,
    // and an fs shim. They must sit together so the hook resolves the shim by
    // relative path, so write all three into one temp dir.
    let temp_dir = materialize_scripts(
        "svelte-package",
        &[
            (
                "svelte-package-wrapper.mjs",
                include_str!("../../../js/cli/svelte-package-wrapper.mjs"),
            ),
            (
                "svelte-package-fs-hook.mjs",
                include_str!("../../../js/cli/svelte-package-fs-hook.mjs"),
            ),
            (
                "svelte-package-fs-shim.mjs",
                include_str!("../../../js/cli/svelte-package-fs-shim.mjs"),
            ),
        ],
    )?;
    let main_path = temp_dir.join("svelte-package-wrapper.mjs");

    let flags = PackageFlags {
        input,
        output: output.clone(),
        tsconfig: tsconfig.clone(),
        no_types,
    };

    // `dist` is svelte-package's own default for `--output`; we have to resolve
    // it here because the packager is being pointed at a scratch directory and
    // this is the only place that knows where the result actually belongs.
    let dest = output.unwrap_or_else(|| PathBuf::from("dist"));
    let dest_abs = absolutize(root, &dest);

    if full_rebuild {
        eprintln!("[macroforge] rebuilding: --full-rebuild");
        let _ = fs::remove_dir_all(state_dir(root));
    }
    let previous = PackageState::load(root);

    // Reuse the cached resolution while everything it depends on holds.
    let resolver_hash = package_state::resolver_hash(root, &flags);
    let resolved = match previous
        .as_ref()
        .filter(|state| state.inputs.resolver_hash == resolver_hash)
    {
        Some(state) => state.resolved.clone(),
        None => {
            let probe = probe_package_config(&main_path, &flags)?;
            // Used exactly as the probe reported it. The redirect matches read
            // paths against this prefix, and the packager builds those paths
            // with `path.resolve`, which is purely lexical — canonicalizing here
            // would stop a symlinked input directory from ever matching, and a
            // read that fails to match is served unexpanded.
            ResolvedPackageConfig {
                input: probe.input,
                extensions: probe.extensions,
            }
        }
    };

    // A missing input directory is the packager's error to report, in the words
    // it has always used. There is nothing to expand or record, so hand over.
    if !resolved.input.is_dir() {
        return run_packager(&main_path, &flags, &dest, None);
    }

    let current = PackageInputs {
        version: env!("CARGO_PKG_VERSION").to_string(),
        config_hash: compute_config_hash(root),
        external_macro_hash: compute_external_macro_hash(root),
        resolver_hash,
        project_hash: project_hash(root, &dest_abs)?,
        tool_hashes: tool_hashes(
            root,
            &resolved.input,
            tsconfig.as_deref().map(|p| absolutize(root, p)).as_deref(),
        ),
        files: scan_input(&resolved.input, &resolved.extensions)?,
        output_fingerprint: output_fingerprint(&dest_abs)?,
    };

    if let Some(state) = previous.as_ref() {
        match state.stale_reason(&current) {
            None => {
                eprintln!(
                    "[macroforge] {} is up to date — {} files unchanged (--full-rebuild to force)",
                    dest.display(),
                    current.files.len()
                );
                return Ok(());
            }
            Some(reason) => eprintln!("[macroforge] rebuilding: {reason}"),
        }
    }

    // Build the type registry so expansion can resolve cross-module type
    // references, exactly as the tsc / svelte-check wrappers do. This is the
    // first expensive step, and everything above exists to avoid reaching it.
    ensure_type_registry_cache(root);
    let registry_hash = package_state::registry_hash(root);

    let expanded = expanded_dir(root);

    // An expansion depends on more than the file it was produced from: the
    // engine, the config, the project's type surface. None of those leave a
    // mark on a source file, so when one moves, comparing sources finds nothing
    // changed and the whole tree has to be produced again regardless.
    let full_reason: Option<&'static str> = match previous.as_ref() {
        Some(state) => state.expansion_stale_reason(&current, &registry_hash),
        None => Some("no previous build to reuse"),
    };

    let targets: Vec<String> = match (full_reason, previous.as_ref()) {
        (None, Some(state)) => {
            let mut targets = diff_files(&state.inputs.files, &current.files).changed;
            // An artifact that went missing since the last run has to be
            // rebuilt even though its source did not change. Nothing else would
            // notice: the packager would simply read the raw source and publish
            // a module with its generated runtime missing.
            targets.extend(
                state
                    .expanded_entries
                    .iter()
                    .filter(|rel| !expanded.join(rel).is_file() && current.files.contains_key(*rel))
                    .cloned(),
            );
            targets.sort();
            targets.dedup();
            targets
        }
        _ => current.files.keys().cloned().collect(),
    };

    let considered = targets.len();
    let outcome = run_expansion_pass(root, &resolved.input, &expanded, &targets, &current.files)?;
    let total = outcome.entries.len();

    // Report the work, and when it was more than the edit seemed to warrant,
    // why. Re-expanding a whole library after a three-field change is correct —
    // every expansion depends on the project's types — but it reads as a bug
    // unless the reason comes with it.
    if let Some(reason) = full_reason {
        eprintln!("[macroforge] Re-expanded all {total} macro module(s) — {reason}");
    } else if outcome.expanded > 0 {
        eprintln!(
            "[macroforge] Re-expanded {} of {total} macro module(s) ({considered} changed file(s))",
            outcome.expanded
        );
    } else if considered == 0 {
        eprintln!("[macroforge] Reused all {total} expanded module(s)");
    } else {
        eprintln!(
            "[macroforge] Reused all {total} expanded module(s) — none of the {considered} \
             changed file(s) carry macros"
        );
    }

    let entries_path = state_dir(root).join("entries.json");
    write_atomic(
        &entries_path,
        serde_json::to_string(&outcome.entries)?.as_bytes(),
    )?;

    run_packager(
        &main_path,
        &flags,
        &dest,
        Some(Redirect {
            input: &resolved.input,
            expanded: &expanded,
            entries: &entries_path,
        }),
    )?;

    // Record only the files that still hash the same as when they were read.
    // Anything edited while the packager ran was not necessarily packaged, so
    // leaving it out of the state makes the next run rebuild it rather than
    // recording work that may never have happened.
    let mut files = current.files;
    let rescanned = scan_input(&resolved.input, &resolved.extensions)?;
    files.retain(|rel, stamp| {
        rescanned
            .get(rel)
            .is_some_and(|fresh| fresh.normalized_hash == stamp.normalized_hash)
    });

    let state = PackageState {
        inputs: PackageInputs {
            files,
            output_fingerprint: output_fingerprint(&dest_abs)?,
            ..current
        },
        resolved,
        registry_hash,
        expanded_entries: outcome.entries,
    };
    state.save(root)?;

    Ok(())
}

/// Where the packager's reads are served from.
struct Redirect<'a> {
    input: &'a Path,
    expanded: &'a Path,
    entries: &'a Path,
}

/// Runs the packager into a scratch directory and swaps the result into `dest`.
fn run_packager(
    main_path: &Path,
    flags: &PackageFlags,
    dest: &Path,
    redirect: Option<Redirect<'_>>,
) -> Result<()> {
    // The scratch directory must be a *sibling* of the destination, at the same
    // depth. `emit_dts` writes `.d.ts.map` `sources` relative to the output
    // directory, so packaging into a path one level deeper or shallower would
    // bake the wrong number of `../` segments into the published sourcemaps.
    let staging = sibling_with_suffix(dest, "macroforge-staging");
    let _ = fs::remove_dir_all(&staging);

    let mut cmd = std::process::Command::new("node");
    cmd.arg(main_path);

    if let Some(redirect) = redirect {
        cmd.env("MACROFORGE_PACKAGE_INPUT", redirect.input);
        cmd.env("MACROFORGE_PACKAGE_EXPANDED", redirect.expanded);
        cmd.env("MACROFORGE_PACKAGE_ENTRIES", redirect.entries);
    }

    if let Some(ref i) = flags.input {
        cmd.arg("--input").arg(i);
    }
    cmd.arg("--output").arg(&staging);
    if let Some(ref t) = flags.tsconfig {
        cmd.arg("--tsconfig").arg(t);
    }
    if flags.no_types {
        cmd.arg("--no-types");
    }

    let status = cmd
        .status()
        .context("failed to run node svelte-package wrapper")?;

    if !status.success() {
        // Leave the previous output untouched — a failed package must not be
        // able to destroy a working one.
        let _ = fs::remove_dir_all(&staging);
        std::process::exit(status.code().unwrap_or(1));
    }

    swap_dir(&staging, dest).inspect_err(|_| {
        let _ = fs::remove_dir_all(&staging);
    })?;

    eprintln!("[macroforge] packaged into {}", dest.display());

    Ok(())
}
