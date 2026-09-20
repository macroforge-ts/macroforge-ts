//! Build state for `macroforge svelte-package`.
//!
//! Packaging a library is a pure function of its inputs, so a run whose inputs
//! all match the previous one has nothing to produce. This module records what
//! the last successful run consumed and produced, and answers two questions on
//! the next one: is the existing output still correct, and if not, which files
//! actually moved.
//!
//! Both answers are deliberately conservative. A missed change ships a stale
//! published package, which is far worse than a rebuild that turns out to have
//! been unnecessary, so anything this module cannot account for precisely —
//! a config edit, a rebuilt macro binary, a shifted type surface — invalidates
//! everything rather than guessing.
//!
//! State lives in `.macroforge/svelte-package/state.json` next to the expanded
//! tree it describes, and is written through [`write_atomic`] like the rest of
//! `.macroforge/`.

use anyhow::{Context, Result};
use std::{
    collections::{BTreeMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use crate::atomic_fs::write_atomic;
use crate::cache::{content_hash, normalized_content_hash};

/// Directory owning every `svelte-package` artifact for a project.
pub(crate) fn state_dir(root: &Path) -> PathBuf {
    root.join(".macroforge").join("svelte-package")
}

/// The persisted tree of expanded sources the packager reads through.
pub(crate) fn expanded_dir(root: &Path) -> PathBuf {
    state_dir(root).join("expanded")
}

fn state_path(root: &Path) -> PathBuf {
    state_dir(root).join("state.json")
}

/// The packager options that only `svelte.config.js` can answer.
///
/// Deliberately just these two. Everything else the packager takes is a direct
/// reading of the command line, which the CLI already has and which
/// [`PackageInputs::resolver_hash`] already covers — storing a second copy here
/// would be one more thing that can disagree with itself. These two require
/// evaluating a JavaScript module, so they are filled by the wrapper's
/// `--macroforge-resolve-config` probe and cached for as long as that hash holds.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ResolvedPackageConfig {
    /// Absolute input directory (`config.kit.files.lib`, or `src/lib`).
    pub(crate) input: PathBuf,
    /// Extensions treated as Svelte components (`config.extensions`).
    pub(crate) extensions: Vec<String>,
}

/// Two hashes of one input file.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FileStamp {
    /// SHA-256 of the exact bytes the last successful run packaged.
    pub(crate) source_hash: String,
    /// SHA-256 of the whitespace-normalized text, for source files; equal to
    /// `source_hash` for everything else. This is what change detection
    /// compares, so reformatting a file is not a rebuild.
    pub(crate) normalized_hash: String,
}

/// Everything a run's output depends on, other than the resolved config.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PackageInputs {
    /// Macroforge crate version.
    pub(crate) version: String,
    /// SHA-256 of the macroforge config file (`cache::compute_config_hash`).
    pub(crate) config_hash: String,
    /// Metadata hash of external macro binaries
    /// (`cache::compute_external_macro_hash`).
    pub(crate) external_macro_hash: String,
    /// Hash of everything the config probe's answer depends on.
    pub(crate) resolver_hash: String,
    /// Aggregate hash of the project's TypeScript sources — the files that
    /// feed the type registry, which expansion output depends on.
    pub(crate) project_hash: String,
    /// Content hashes / versions of the surrounding toolchain.
    pub(crate) tool_hashes: BTreeMap<String, String>,
    /// Per-file stamps, keyed by `/`-separated path relative to the input dir.
    pub(crate) files: BTreeMap<String, FileStamp>,
    /// Fingerprint of the output directory, so an externally deleted or
    /// truncated `dist` is rebuilt even when every input matches.
    pub(crate) output_fingerprint: String,
}

/// What the last successful `svelte-package` run consumed and produced.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PackageState {
    pub(crate) inputs: PackageInputs,
    pub(crate) resolved: ResolvedPackageConfig,
    /// Hash of the type and declarative registries the expanded tree was
    /// produced against. Unlike the rest of `inputs` this cannot participate in
    /// the skip decision — the registries are only rebuilt once a run is
    /// already under way — so it reaches the build through
    /// [`Self::expansion_stale_reason`] instead.
    pub(crate) registry_hash: String,
    /// Input paths that had an expanded artifact when the run finished.
    ///
    /// Recorded so a missing artifact is noticed. Nothing else can tell a file
    /// that legitimately has none — no macros — from one whose artifact was
    /// deleted, and the second case would hand the packager raw source and
    /// publish a module with its generated runtime missing.
    #[serde(default)]
    pub(crate) expanded_entries: Vec<String>,
}

impl PackageState {
    pub(crate) fn load(root: &Path) -> Option<Self> {
        let text = fs::read_to_string(state_path(root)).ok()?;
        serde_json::from_str(&text).ok()
    }

    pub(crate) fn save(&self, root: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        write_atomic(&state_path(root), json.as_bytes())
    }

    /// Why the recorded output cannot be reused, or `None` if it can.
    ///
    /// The first mismatch wins and is phrased for the user, because "up to
    /// date" is a claim about correctness and an unexpected rebuild should be
    /// self-explaining rather than something to bisect.
    pub(crate) fn stale_reason(&self, current: &PackageInputs) -> Option<String> {
        let previous = &self.inputs;

        if previous.version != current.version {
            return Some("macroforge version changed".into());
        }
        if previous.config_hash != current.config_hash {
            return Some("macroforge config changed".into());
        }
        if previous.external_macro_hash != current.external_macro_hash {
            return Some("external macro binary changed".into());
        }
        if previous.resolver_hash != current.resolver_hash {
            return Some("svelte-package options changed".into());
        }
        for (name, hash) in &current.tool_hashes {
            if previous.tool_hashes.get(name) != Some(hash) {
                return Some(format!("{name} changed"));
            }
        }
        if previous.tool_hashes.len() != current.tool_hashes.len() {
            return Some("toolchain changed".into());
        }
        if previous.output_fingerprint != current.output_fingerprint {
            return Some(if current.output_fingerprint == MISSING_OUTPUT {
                "output directory is missing".into()
            } else {
                "output directory was modified".into()
            });
        }

        // Named files before the project-wide hash: an edit inside the input
        // directory moves both, and "index.ts changed" is the reason worth
        // reporting. What reaches the check below is a source the library
        // derives over from somewhere else in the project.
        let changes = diff_files(&previous.files, &current.files);
        if !changes.is_empty() {
            return Some(changes.describe());
        }
        if previous.project_hash != current.project_hash {
            return Some("a project source outside the input directory changed".into());
        }

        None
    }

    /// Why every expanded artifact has to be produced again, or `None` if the
    /// ones whose source did not move can be reused.
    ///
    /// [`Self::stale_reason`] decides whether to run at all; this decides what
    /// that run re-expands, and the two answers are not the same. Most of what
    /// makes a package stale (a tsconfig edit, a new `@sveltejs/package`, a
    /// deleted output directory) changes how sources are *packaged*, and the
    /// expansions still hold. What does not survive is a change to the engine
    /// that produced them, the config it read, or the type surface it resolved
    /// against. None of those touch a single input file, so comparing sources
    /// finds nothing to do and the run republishes the previous build's
    /// expansions under the new inputs, then records them as that build's work.
    /// That covers the case this command exists to prevent: a macro that would
    /// now fail is never invoked, and its last good output ships in place of
    /// the error.
    pub(crate) fn expansion_stale_reason(
        &self,
        current: &PackageInputs,
        registry_hash: &str,
    ) -> Option<&'static str> {
        let previous = &self.inputs;

        if previous.version != current.version {
            return Some("the macroforge version changed");
        }
        if previous.config_hash != current.config_hash {
            return Some("the macroforge config changed");
        }
        if previous.external_macro_hash != current.external_macro_hash {
            return Some("the external macro binary changed");
        }
        if self.registry_hash != registry_hash {
            return Some("the project's type surface changed");
        }

        None
    }
}

/// Input files that moved since the recorded run.
#[derive(Debug, Default)]
pub(crate) struct ChangeSet {
    /// Added, or changed by more than reformatting.
    pub(crate) changed: Vec<String>,
    /// Present in the recorded run, gone now.
    pub(crate) removed: Vec<String>,
}

impl ChangeSet {
    pub(crate) fn is_empty(&self) -> bool {
        self.changed.is_empty() && self.removed.is_empty()
    }

    /// A one-line summary naming the file when there is only one, because that
    /// is the common case and the name is the useful part.
    pub(crate) fn describe(&self) -> String {
        match (self.changed.len(), self.removed.len()) {
            (1, 0) => format!("{} changed", self.changed[0]),
            (0, 1) => format!("{} was removed", self.removed[0]),
            (c, 0) => format!("{c} files changed"),
            (0, r) => format!("{r} files were removed"),
            (c, r) => format!("{c} files changed, {r} removed"),
        }
    }
}

/// Compares two input snapshots.
///
/// A file counts as unchanged when its *normalized* hashes match, so trailing
/// whitespace and blank-line churn do not trigger a rebuild. Identical bytes
/// imply identical normalized text, so this subsumes the exact comparison.
pub(crate) fn diff_files(
    previous: &BTreeMap<String, FileStamp>,
    current: &BTreeMap<String, FileStamp>,
) -> ChangeSet {
    let mut changes = ChangeSet::default();

    for (rel, stamp) in current {
        match previous.get(rel) {
            Some(prev) if prev.normalized_hash == stamp.normalized_hash => {}
            _ => changes.changed.push(rel.clone()),
        }
    }
    for rel in previous.keys() {
        if !current.contains_key(rel) {
            changes.removed.push(rel.clone());
        }
    }

    changes
}

/// Fingerprint recorded when the output directory does not exist.
pub(crate) const MISSING_OUTPUT: &str = "missing";

/// Directories the project scan skips, mirroring the scanner's own
/// `ScanConfig::default().skip_dirs`.
const PROJECT_SKIP_DIRS: &[&str] = &[
    "node_modules",
    ".git",
    "dist",
    "build",
    ".macroforge",
    "coverage",
    ".next",
    ".nuxt",
    ".svelte-kit",
];

/// Extensions whose files are compared ignoring formatting.
///
/// Limited to code, because it is the only content whose meaning survives
/// reformatting. A `.json` or `.css` file is copied into the package verbatim,
/// so for those a byte is a byte.
const CODE_EXTENSIONS: &[&str] = &[".ts", ".tsx", ".mts", ".cts", ".js", ".mjs", ".cjs"];

/// Names and paths a walk skips.
#[derive(Default)]
struct SkipRules {
    names: HashSet<String>,
    paths: Vec<PathBuf>,
}

impl SkipRules {
    fn skips(&self, name: &str, path: &Path) -> bool {
        if self.names.contains(name) {
            return true;
        }
        if self.paths.is_empty() {
            return false;
        }
        // Compared canonically as well as literally: the paths to skip are
        // canonical, and a `dist` that is a symlink would otherwise walk right
        // past the check and feed a build's own output back into its inputs.
        let canonical = path.canonicalize();
        self.paths
            .iter()
            .any(|p| p == path || canonical.as_deref().is_ok_and(|c| c == p))
    }
}

/// Collects the files under `root` as `/`-separated relative paths, sorted.
///
/// Mirrors `@sveltejs/package`'s own `walk()` (`src/filesystem.js`): plain
/// recursive readdir with no gitignore handling, no extension filter, dotfiles
/// included, and `stat` rather than `symlink_metadata` so a symlinked directory
/// is followed. Anything hidden here is a file whose change would go unnoticed,
/// so the default is to hide nothing.
///
/// The one deliberate departure is the visited set: `walk()` recurses forever
/// through a symlink that points back up its own tree, and a build command that
/// hangs is not an acceptable way to report a cyclic source directory.
fn walk_files(root: &Path, skip: &SkipRules) -> Result<Vec<(String, PathBuf)>> {
    let mut out: Vec<(String, PathBuf)> = Vec::new();
    let mut visited: HashSet<PathBuf> = HashSet::new();
    let mut stack: Vec<(PathBuf, String)> = vec![(root.to_path_buf(), String::new())];

    while let Some((dir, rel)) = stack.pop() {
        if let Ok(canonical) = dir.canonicalize()
            && !visited.insert(canonical)
        {
            continue;
        }

        let entries =
            fs::read_dir(&dir).with_context(|| format!("failed to read {}", dir.display()))?;

        for entry in entries {
            let entry =
                entry.with_context(|| format!("failed to read an entry in {}", dir.display()))?;
            let name = entry.file_name().to_string_lossy().to_string();
            let path = entry.path();
            let child_rel = if rel.is_empty() {
                name.clone()
            } else {
                format!("{rel}/{name}")
            };

            // A broken symlink is not a file the packager can read either, so
            // treating an unreadable entry as absent keeps the two in step.
            let Ok(meta) = fs::metadata(&path) else {
                continue;
            };

            if meta.is_dir() {
                if !skip.skips(&name, &path) {
                    stack.push((path, child_rel));
                }
            } else {
                out.push((child_rel, path));
            }
        }
    }

    out.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(out)
}

/// Whether `rel` names a file compared ignoring formatting.
fn compares_normalized(rel: &str, extensions: &[String]) -> bool {
    CODE_EXTENSIONS.iter().any(|ext| rel.ends_with(ext))
        || extensions.iter().any(|ext| rel.ends_with(ext.as_str()))
}

/// Stamps one file's bytes.
fn stamp_bytes(bytes: &[u8], normalized: bool) -> FileStamp {
    let source_hash = content_hash(bytes);
    let normalized_hash = match normalized.then(|| std::str::from_utf8(bytes)) {
        Some(Ok(text)) => normalized_content_hash(text),
        // Not a source file, or not text at all: the bytes are the meaning.
        _ => source_hash.clone(),
    };
    FileStamp {
        source_hash,
        normalized_hash,
    }
}

/// Stamps every file under the input directory.
pub(crate) fn scan_input(
    input: &Path,
    extensions: &[String],
) -> Result<BTreeMap<String, FileStamp>> {
    let files = walk_files(input, &SkipRules::default())
        .with_context(|| format!("failed to scan {}", input.display()))?;

    let mut stamps = BTreeMap::new();
    for (rel, path) in files {
        let bytes =
            fs::read(&path).with_context(|| format!("failed to read {}", path.display()))?;
        let normalized = compares_normalized(&rel, extensions);
        stamps.insert(rel, stamp_bytes(&bytes, normalized));
    }
    Ok(stamps)
}

/// Fingerprints the output directory from paths and sizes alone.
///
/// This exists to notice a `dist` that was deleted, partially deleted, or
/// overwritten by something else while every input stayed put. Sizes are enough
/// for that and cost only a `stat`, which matters because this runs on the fast
/// path that is supposed to beat a real build by orders of magnitude.
pub(crate) fn output_fingerprint(output: &Path) -> Result<String> {
    if !output.is_dir() {
        return Ok(MISSING_OUTPUT.to_string());
    }

    let files = walk_files(output, &SkipRules::default())
        .with_context(|| format!("failed to scan {}", output.display()))?;

    let mut buf = String::new();
    for (rel, path) in files {
        let len = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        buf.push_str(&rel);
        buf.push(':');
        buf.push_str(&len.to_string());
        buf.push('\n');
    }
    Ok(content_hash(buf.as_bytes()))
}

/// Aggregate hash of the TypeScript sources that feed the type registry.
///
/// Expansion output depends on the whole project's type surface, not just on
/// the file being expanded: a type defined outside the input directory and
/// derived over inside it changes the generated code without changing any input
/// file. Tracking the sources the scanner reads closes that gap for the skip
/// decision; [`PackageState::registry_hash`] closes it for the incremental one.
///
/// Mirrors `ScanConfig::default()` — `.ts`/`.tsx` under the project root, minus
/// the usual non-source directories — plus the output directory, which is
/// otherwise a build's own product feeding back into its next input.
pub(crate) fn project_hash(root: &Path, output: &Path) -> Result<String> {
    let skip = SkipRules {
        names: PROJECT_SKIP_DIRS.iter().map(|s| s.to_string()).collect(),
        paths: vec![output.to_path_buf()],
    };

    let files =
        walk_files(root, &skip).with_context(|| format!("failed to scan {}", root.display()))?;

    let mut buf = String::new();
    for (rel, path) in files {
        if !(rel.ends_with(".ts") || rel.ends_with(".tsx")) {
            continue;
        }
        let Ok(bytes) = fs::read(&path) else {
            continue;
        };
        // Normalized, so reformatting the project does not force every library
        // in it to repackage.
        buf.push_str(&rel);
        buf.push(':');
        buf.push_str(&stamp_bytes(&bytes, true).normalized_hash);
        buf.push('\n');
    }
    Ok(content_hash(buf.as_bytes()))
}

/// Hash of the type and declarative registries as they stand on disk.
///
/// Read after the scan that writes them, and compared against the value stored
/// with the expanded tree: if the project's type surface moved, every expansion
/// is suspect, not just the ones whose own source changed.
pub(crate) fn registry_hash(root: &Path) -> String {
    let dir = root.join(".macroforge");
    let mut buf = String::new();
    for name in ["type-registry.json", "declarative-registry.json"] {
        buf.push_str(&canonical_json(&dir.join(name)));
        buf.push('\n');
    }
    content_hash(buf.as_bytes())
}

/// Reads a JSON file into a form whose text depends only on its content.
///
/// Both registries are `HashMap`-backed, so `serde_json` renders their keys in
/// whatever order the map iterates and the raw text differs between two runs
/// that scanned identical sources. Round-tripping through `Value` sorts the
/// keys — `serde_json` is built without `preserve_order` here, so its object
/// type is a `BTreeMap` — which makes the hash a function of the content alone.
fn canonical_json(path: &Path) -> String {
    let Ok(text) = fs::read_to_string(path) else {
        return "none".to_string();
    };
    match serde_json::from_str::<serde_json::Value>(&text) {
        Ok(value) => value.to_string(),
        // Unparseable is still deterministic, and the next run will rewrite it.
        Err(_) => text,
    }
}

/// The command-line options that reach `@sveltejs/package`.
#[derive(Debug, Clone)]
pub(crate) struct PackageFlags {
    pub(crate) input: Option<PathBuf>,
    pub(crate) output: Option<PathBuf>,
    pub(crate) tsconfig: Option<PathBuf>,
    pub(crate) no_types: bool,
}

impl PackageFlags {
    fn describe(&self) -> String {
        format!(
            "input={:?}\noutput={:?}\ntsconfig={:?}\nno_types={}\n",
            self.input, self.output, self.tsconfig, self.no_types
        )
    }
}

/// Hash of everything the config probe's answer depends on.
///
/// The probe costs a Node startup, which is most of the fast path's budget, so
/// its result is cached in the state file and reused while this hash holds.
/// That means covering every input to it: the command line, and the two files
/// `load_config()` reads.
pub(crate) fn resolver_hash(root: &Path, flags: &PackageFlags) -> String {
    let mut buf = flags.describe();
    for name in ["svelte.config.js", "svelte.config.ts", "package.json"] {
        buf.push_str(name);
        buf.push(':');
        buf.push_str(&file_hash(&root.join(name)));
        buf.push('\n');
    }
    content_hash(buf.as_bytes())
}

/// Content hashes and versions of the toolchain around the packager.
///
/// The tsconfig decides how sources are transpiled and typed; the three
/// packages decide how they are packaged, expanded, and preprocessed. None of
/// them is an input file, and all of them change the output.
///
/// Versions rather than content: a package's own files are too many to hash on
/// every invocation. A locally rebuilt link whose version did not change is the
/// gap this leaves, and `--full-rebuild` is its escape hatch — except for macro
/// packages, which `external_macro_hash` already covers by binary metadata.
pub(crate) fn tool_hashes(
    root: &Path,
    input: &Path,
    tsconfig: Option<&Path>,
) -> BTreeMap<String, String> {
    let mut hashes = BTreeMap::new();

    let tsconfig = tsconfig
        .map(Path::to_path_buf)
        .or_else(|| find_tsconfig(input, root));
    hashes.insert(
        "tsconfig".to_string(),
        tsconfig.map(|p| file_hash(&p)).unwrap_or_else(none),
    );

    for name in [
        "@sveltejs/package",
        "macroforge",
        "@macroforge/svelte-preprocessor",
    ] {
        hashes.insert(name.to_string(), dep_version(root, name));
    }

    hashes
}

fn none() -> String {
    "none".to_string()
}

fn file_hash(path: &Path) -> String {
    fs::read(path)
        .map(|b| content_hash(&b))
        .unwrap_or_else(|_| none())
}

/// Finds the tsconfig the packager would fall back to.
///
/// Mirrors `load_tsconfig`'s hand-rolled search (`typescript.js:157-181`) —
/// nearest `tsconfig.json` or `jsconfig.json` walking up — starting from the
/// input directory. Configs *inside* the input directory need no special
/// handling: they are input files, and already stamped as such.
fn find_tsconfig(input: &Path, root: &Path) -> Option<PathBuf> {
    let mut dir = input;
    loop {
        for name in ["tsconfig.json", "jsconfig.json"] {
            let candidate = dir.join(name);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
        if dir == root {
            return None;
        }
        dir = dir.parent()?;
    }
}

/// Reads an installed package's version from `node_modules`.
fn dep_version(root: &Path, name: &str) -> String {
    let path = root.join("node_modules").join(name).join("package.json");
    let Ok(text) = fs::read_to_string(&path) else {
        return none();
    };
    serde_json::from_str::<serde_json::Value>(&text)
        .ok()
        .and_then(|v| {
            v.get("version")
                .and_then(|s| s.as_str())
                .map(str::to_string)
        })
        .unwrap_or_else(|| "unknown".to_string())
}
