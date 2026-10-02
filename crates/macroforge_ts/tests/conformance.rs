//! The conformance corpus: every fixture and every playground source expanded
//! the way the engine's integrations expand it, compared against committed
//! golden output.
//!
//! A change that only makes expansion faster moves no golden here. Record or
//! accept changes with `pixi run test:conformance:update`.

use std::fmt::Write;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};

use ignore::WalkBuilder;
use macroforge_ts::has_macro_annotations;
use macroforge_ts::host::declarative::BuildMode;
use macroforge_ts::host::project::{ProjectRegistries, expand_project_file};
use macroforge_ts::host::scanner::{ProjectScanner, ScanConfig, ScanOutput};
use macroforge_ts::host::{MacroExpander, MacroExpansion};
use macroforge_ts::ts_syn::abi::ir::type_registry::TypeRegistry;
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

/// The playground projects, each a macroforge project of its own.
const PLAYGROUND_PROJECTS: [&str; 4] = ["vanilla", "svelte", "library", "macro"];

/// Set to skip the playground corpus when its macro package cannot be built,
/// such as offline. `mf test` never sets it.
const SKIP_PLAYGROUND_VAR: &str = "MACROFORGE_CONFORMANCE_SKIP_PLAYGROUND";

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn repo_root() -> PathBuf {
    manifest_dir()
        .join("../..")
        .canonicalize()
        .expect("the repository root resolves")
}

fn conformance_dir() -> PathBuf {
    manifest_dir().join("tests/fixtures/conformance")
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

/// `value` with every object's keys sorted, so equal values print equally
/// whatever order they were built in.
fn canonical(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut entries: Vec<(String, Value)> = map.into_iter().collect();
            entries.sort_by(|left, right| left.0.cmp(&right.0));
            let mut sorted = Map::new();
            for (key, entry) in entries {
                sorted.insert(key, canonical(entry));
            }
            Value::Object(sorted)
        }
        Value::Array(items) => Value::Array(items.into_iter().map(canonical).collect()),
        other => other,
    }
}

fn canonical_json(value: &impl serde::Serialize) -> String {
    let value = serde_json::to_value(value).expect("the value serializes");
    serde_json::to_string(&canonical(value)).expect("a JSON value prints")
}

/// Replaces machine-specific absolute paths with placeholders.
struct Normalizer {
    replacements: Vec<(String, &'static str)>,
}

impl Normalizer {
    fn new(project_root: &Path) -> Self {
        Self {
            replacements: vec![
                (project_root.display().to_string(), "<root>"),
                (repo_root().display().to_string(), "<repo>"),
            ],
        }
    }

    fn apply(&self, text: &str) -> String {
        let mut normalized = text.to_string();
        for (path, placeholder) in &self.replacements {
            normalized = normalized.replace(path.as_str(), placeholder);
        }
        normalized
    }
}

/// Where a snapshot's source came from, printed at its head.
enum SourceHeader<'a> {
    /// The whole input, for a small hand-written fixture.
    Full(&'a str),
    /// The project-relative path and a hash, for a playground file whose
    /// source lives in the repository already.
    Located { path: &'a str, source: &'a str },
}

fn push_section(out: &mut String, title: &str, body: &str) {
    if !out.is_empty() {
        out.push_str("\n\n");
    }
    out.push_str("## ");
    out.push_str(title);
    out.push_str("\n\n");
    out.push_str(if body.is_empty() { "(none)" } else { body });
}

fn diagnostics_section(expansion: &MacroExpansion) -> String {
    let mut body = String::new();
    for diagnostic in &expansion.diagnostics {
        let span = diagnostic
            .span
            .map(|span| format!(" [{}..{}]", span.start, span.end))
            .unwrap_or_default();
        writeln!(body, "{:?}{span}: {}", diagnostic.level, diagnostic.message)
            .expect("writing to a String cannot fail");
        for note in &diagnostic.notes {
            writeln!(body, "  note: {note}").expect("writing to a String cannot fail");
        }
        if let Some(help) = &diagnostic.help {
            writeln!(body, "  help: {help}").expect("writing to a String cannot fail");
        }
    }
    body.trim_end().to_string()
}

fn mapping_section(expansion: &MacroExpansion) -> String {
    let Some(mapping) = &expansion.source_mapping else {
        return String::new();
    };
    let mut body = String::new();
    for segment in &mapping.segments {
        writeln!(
            body,
            "original {}..{} -> expanded {}..{}",
            segment.original_start,
            segment.original_end,
            segment.expanded_start,
            segment.expanded_end
        )
        .expect("writing to a String cannot fail");
    }
    for region in &mapping.generated_regions {
        writeln!(
            body,
            "generated {}..{} <- {}",
            region.start, region.end, region.source_macro
        )
        .expect("writing to a String cannot fail");
    }
    body.trim_end().to_string()
}

fn metadata_section(expansion: &MacroExpansion) -> String {
    let mut body = String::new();
    for class in &expansion.classes {
        writeln!(body, "class {}: {}", class.name, canonical_json(class))
            .expect("writing to a String cannot fail");
    }
    for interface in &expansion.interfaces {
        writeln!(
            body,
            "interface {}: {}",
            interface.name,
            canonical_json(interface)
        )
        .expect("writing to a String cannot fail");
    }
    for enumeration in &expansion.enums {
        writeln!(
            body,
            "enum {}: {}",
            enumeration.name,
            canonical_json(enumeration)
        )
        .expect("writing to a String cannot fail");
    }
    for alias in &expansion.type_aliases {
        writeln!(body, "type {}: {}", alias.name, canonical_json(alias))
            .expect("writing to a String cannot fail");
    }
    body.trim_end().to_string()
}

fn format_expansion(
    header: SourceHeader,
    source: &str,
    result: &Result<MacroExpansion, String>,
    normalizer: &Normalizer,
) -> String {
    let mut out = String::new();
    match header {
        SourceHeader::Full(input) => push_section(&mut out, "Input", input.trim()),
        SourceHeader::Located { path, source } => push_section(
            &mut out,
            "Source",
            &format!("{path}\nsha256 {}", sha256_hex(source.as_bytes())),
        ),
    }

    let expansion = match result {
        Ok(expansion) => expansion,
        Err(error) => {
            push_section(&mut out, "Error", error);
            out.push('\n');
            return normalizer.apply(&out);
        }
    };

    push_section(&mut out, "Changed", &expansion.changed.to_string());
    if expansion.code == source {
        push_section(&mut out, "Output", "(unchanged)");
    } else {
        push_section(&mut out, "Output", expansion.code.trim());
    }
    push_section(
        &mut out,
        "DTS",
        expansion.type_output.as_deref().unwrap_or_default().trim(),
    );
    push_section(&mut out, "Diagnostics", &diagnostics_section(expansion));
    push_section(&mut out, "Mapping", &mapping_section(expansion));
    push_section(&mut out, "Metadata", &metadata_section(expansion));
    let reads = expansion
        .registry_reads
        .as_ref()
        .map(|reads| {
            reads
                .iter()
                .map(|read| format!("{read:?}"))
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_else(|| "(not recorded)".to_string());
    push_section(&mut out, "Registry reads", &reads);
    let dependencies = expansion
        .buildtime_dependencies
        .iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>()
        .join("\n");
    push_section(&mut out, "Buildtime dependencies", &dependencies);
    out.push('\n');
    normalizer.apply(&out)
}

fn assert_named_snapshot(snapshot_dir: &Path, name: &str, snapshot: &str) {
    insta::with_settings!({
        snapshot_path => snapshot_dir,
        prepend_module_to_snapshot => false,
        omit_expression => true,
    }, {
        insta::assert_snapshot!(name, snapshot);
    });
}

/// Runs every check, then fails once naming each one that failed, so a run
/// reports every moved golden rather than the first.
struct CheckFailures {
    failed: Vec<String>,
}

impl CheckFailures {
    fn new() -> Self {
        Self { failed: Vec::new() }
    }

    fn check(&mut self, label: &str, check: impl FnOnce()) {
        if catch_unwind(AssertUnwindSafe(check)).is_err() {
            self.failed.push(label.to_string());
        }
    }

    fn finish(self, corpus: &str) {
        assert!(
            self.failed.is_empty(),
            "{} {corpus} check(s) failed:\n{}",
            self.failed.len(),
            self.failed.join("\n")
        );
    }
}

// ---------------------------------------------------------------------------
// Fixture corpora
// ---------------------------------------------------------------------------

/// Expands with the expander the snapshot suites have always used: the config
/// found from the working directory, and no project registries.
fn expand_fixture(
    expander: &MacroExpander,
    source: &str,
    file_name: &str,
) -> Result<MacroExpansion, String> {
    expander
        .expand_source(source, file_name)
        .map_err(|error| error.to_string())
}

fn fixture_file_name(path: &Path) -> &str {
    path.file_name()
        .and_then(|name| name.to_str())
        .expect("a fixture has a UTF-8 file name")
}

fn fixture_snapshot_dir(path: &Path) -> PathBuf {
    path.parent()
        .expect("a fixture lives in a directory")
        .join("snapshots")
}

fn read_fixture(path: &Path) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}

#[test]
fn spec_ok() {
    let expander = MacroExpander::new().expect("the default expander builds");
    let normalizer = Normalizer::new(&manifest_dir());

    insta::glob!("fixtures/ok/**/*.ts", |path| {
        let input = read_fixture(path);
        let result = expand_fixture(&expander, &input, fixture_file_name(path));
        match &result {
            Ok(expansion) => assert!(
                expansion.changed,
                "fixture {} should contain macros (changed == true)",
                path.display()
            ),
            Err(error) => panic!(
                "fixture {} should expand successfully, got error: {error}",
                path.display()
            ),
        }
        let snapshot = format_expansion(SourceHeader::Full(&input), &input, &result, &normalizer);
        insta::with_settings!({
            snapshot_path => fixture_snapshot_dir(path),
            prepend_module_to_snapshot => false,
        }, {
            insta::assert_snapshot!(snapshot);
        });
    });
}

#[test]
fn spec_error() {
    let expander = MacroExpander::new().expect("the default expander builds");
    let normalizer = Normalizer::new(&manifest_dir());

    insta::glob!("fixtures/error/**/*.ts", |path| {
        let input = read_fixture(path);
        let result = expand_fixture(&expander, &input, fixture_file_name(path));
        let snapshot = format_expansion(SourceHeader::Full(&input), &input, &result, &normalizer);
        insta::with_settings!({
            snapshot_path => fixture_snapshot_dir(path),
            prepend_module_to_snapshot => false,
        }, {
            insta::assert_snapshot!(snapshot);
        });
    });
}

/// Ambient declarations, pinning what the expander does with a `.d.ts`.
#[test]
fn declaration_files() {
    let expander = MacroExpander::new().expect("the default expander builds");
    let normalizer = Normalizer::new(&manifest_dir());

    insta::glob!("fixtures/conformance/dts/*.d.ts", |path| {
        let input = read_fixture(path);
        let result = expand_fixture(&expander, &input, fixture_file_name(path));
        let snapshot = format_expansion(SourceHeader::Full(&input), &input, &result, &normalizer);
        insta::with_settings!({
            snapshot_path => fixture_snapshot_dir(path),
            prepend_module_to_snapshot => false,
        }, {
            insta::assert_snapshot!(snapshot);
        });
    });
}

fn differential_in_mode(label: &str, mode: BuildMode) {
    let normalizer = Normalizer::new(&manifest_dir());
    let mut expander = MacroExpander::new().expect("the default expander builds");
    expander.set_build_mode(mode);
    insta::glob!("fixtures/differential/*.ts", |path| {
        let input = read_fixture(path);
        let result = expand_fixture(&expander, &input, fixture_file_name(path));
        let snapshot = format_expansion(SourceHeader::Full(&input), &input, &result, &normalizer);
        insta::with_settings!({
            snapshot_path => fixture_snapshot_dir(path),
            prepend_module_to_snapshot => false,
        }, {
            insta::assert_snapshot!(format!("differential_{label}"), snapshot);
        });
    });
}

#[test]
fn differential_dev() {
    differential_in_mode("dev", BuildMode::dev());
}

#[test]
fn differential_prod() {
    differential_in_mode("prod", BuildMode::Prod);
}

/// Each foreign-type fixture expanded in full under its own config, as the
/// CLI expands a file.
#[test]
fn foreign_types_full() {
    insta::glob!("fixtures/foreign_types/*/input.ts", |path| {
        let fixture_dir = path.parent().expect("a fixture lives in a directory");
        let normalizer = Normalizer::new(fixture_dir);
        let input = read_fixture(path);
        let result = expand_project_file(fixture_dir, path, &input, &ProjectRegistries::default())
            .map_err(|error| format!("{error:#}"));
        let snapshot = format_expansion(SourceHeader::Full(&input), &input, &result, &normalizer);
        insta::with_settings!({
            snapshot_path => fixture_dir.join("snapshots"),
            prepend_module_to_snapshot => false,
        }, {
            insta::assert_snapshot!(snapshot);
        });
    });
}

// ---------------------------------------------------------------------------
// Playground corpus
// ---------------------------------------------------------------------------

/// Whether the playground corpus runs. It needs the playground's macro
/// package built and installed, which `mf test` prepares.
fn playground_available() -> bool {
    if std::env::var_os(SKIP_PLAYGROUND_VAR).is_some() {
        eprintln!(
            "WARNING: {SKIP_PLAYGROUND_VAR} is set, so the playground conformance corpus is SKIPPED"
        );
        return false;
    }
    let installed =
        repo_root().join("tooling/playground/vanilla/node_modules/@playground/macro/pkg");
    let has_wasm = std::fs::read_dir(&installed)
        .map(|entries| {
            entries.flatten().any(|entry| {
                entry
                    .path()
                    .extension()
                    .is_some_and(|extension| extension == "wasm")
            })
        })
        .unwrap_or(false);
    assert!(
        has_wasm,
        "the playground macro package is not installed at {}. Run `pixi run build:cli` and then \
         `target/debug/mf test playground` (or `pixi run test:rust`, which prepares it), or set \
         {SKIP_PLAYGROUND_VAR}=1 to skip the playground corpus",
        installed.display()
    );
    true
}

struct PlaygroundProject {
    name: &'static str,
    root: PathBuf,
    scan: ScanOutput,
}

fn scan_project(name: &'static str) -> PlaygroundProject {
    let root = repo_root().join("tooling/playground").join(name);
    let scan = ProjectScanner::new(ScanConfig {
        root_dir: root.clone(),
        ..ScanConfig::default()
    })
    .scan()
    .unwrap_or_else(|error| panic!("the {name} playground scan failed: {error:#}"));
    PlaygroundProject { name, root, scan }
}

/// A source file of a playground project, with the text an integration hands
/// the engine for it.
struct CorpusFile {
    path: PathBuf,
    relative: String,
    source: String,
}

/// The script block of a Svelte component, as the preprocessor hands it over.
fn svelte_script(component: &str) -> Option<&str> {
    let mut rest = component;
    while let Some(open) = rest.find("<script") {
        let after_open = &rest[open..];
        let tag_end = after_open.find('>')?;
        let tag = &after_open[..tag_end];
        let body = &after_open[tag_end + 1..];
        let close = body.find("</script>")?;
        if tag.contains("lang=\"ts\"") || tag.contains("lang='ts'") {
            return Some(&body[..close]);
        }
        rest = &body[close..];
    }
    None
}

/// The project's TypeScript sources and Svelte component scripts, in path
/// order, skipping ignored files and generated `.expanded.` copies.
fn project_files(project: &PlaygroundProject) -> Vec<CorpusFile> {
    let walker = WalkBuilder::new(&project.root)
        .hidden(false)
        .git_ignore(true)
        .git_global(false)
        .git_exclude(false)
        .sort_by_file_path(|left, right| left.cmp(right))
        .build();
    let mut files = Vec::new();
    for entry in walker {
        let entry = entry.unwrap_or_else(|error| panic!("walking {}: {error}", project.name));
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let file_name = fixture_file_name(path);
        if file_name.contains(".expanded.") {
            continue;
        }
        let extension = path.extension().and_then(|extension| extension.to_str());
        let is_typescript = matches!(extension, Some("ts" | "tsx"));
        let is_svelte = extension == Some("svelte");
        if !is_typescript && !is_svelte {
            continue;
        }
        let text = read_fixture(path);
        let source = if is_svelte {
            match svelte_script(&text) {
                Some(script) => script.to_string(),
                None => continue,
            }
        } else {
            text
        };
        let relative = path
            .strip_prefix(&project.root)
            .expect("a walked file lies under the project root")
            .display()
            .to_string();
        files.push(CorpusFile {
            path: path.to_path_buf(),
            relative,
            source,
        });
    }
    files
}

fn expand_corpus_file(
    project: &PlaygroundProject,
    registries: &ProjectRegistries,
    file: &CorpusFile,
) -> Result<MacroExpansion, String> {
    expand_project_file(&project.root, &file.path, &file.source, registries)
        .map_err(|error| format!("{error:#}"))
}

#[test]
fn playground_expansions() {
    if !playground_available() {
        return;
    }
    let mut failures = CheckFailures::new();
    for name in PLAYGROUND_PROJECTS {
        let project = scan_project(name);
        let registries = ProjectRegistries::from(&project.scan);
        let normalizer = Normalizer::new(&project.root);
        let snapshot_dir = conformance_dir().join("playground").join(name);
        for file in project_files(&project) {
            let result = expand_corpus_file(&project, &registries, &file);
            let snapshot = format_expansion(
                SourceHeader::Located {
                    path: &file.relative,
                    source: &file.source,
                },
                &file.source,
                &result,
                &normalizer,
            );
            let snapshot_name = file.relative.replace('/', "__");
            failures.check(&format!("{name}/{}", file.relative), || {
                assert_named_snapshot(&snapshot_dir, &snapshot_name, &snapshot);
            });
        }
    }
    failures.finish("playground expansion");
}

/// A short digest of a registry entry's content, with paths normalized.
fn entry_digest(entry: &impl serde::Serialize, normalizer: &Normalizer) -> String {
    let json = normalizer.apply(&canonical_json(entry));
    sha256_hex(json.as_bytes())[..16].to_string()
}

fn registry_snapshot(registry: &TypeRegistry, normalizer: &Normalizer) -> String {
    let mut types: Vec<(String, String)> = registry
        .types
        .iter()
        .map(|(name, entry)| (name.clone(), entry_digest(entry, normalizer)))
        .collect();
    types.sort();
    let mut qualified: Vec<(String, String)> = registry
        .qualified_types
        .iter()
        .map(|(key, entry)| (key.clone(), entry_digest(entry, normalizer)))
        .collect();
    qualified.sort();
    let mut ambiguous: Vec<String> = registry.ambiguous_names.iter().cloned().collect();
    ambiguous.sort();

    let mut out = String::new();
    let lines = |pairs: &[(String, String)]| {
        pairs
            .iter()
            .map(|(key, digest)| format!("{key} {digest}"))
            .collect::<Vec<_>>()
            .join("\n")
    };
    push_section(
        &mut out,
        &format!("Types ({})", types.len()),
        &lines(&types),
    );
    push_section(
        &mut out,
        &format!("Qualified types ({})", qualified.len()),
        &lines(&qualified),
    );
    push_section(&mut out, "Ambiguous names", &ambiguous.join("\n"));
    out.push('\n');
    out
}

fn scan_summary(scan: &ScanOutput, normalizer: &Normalizer) -> String {
    let mut warnings: Vec<String> = scan
        .warnings
        .iter()
        .map(|warning| normalizer.apply(warning))
        .collect();
    warnings.sort();
    let mut out = String::new();
    push_section(&mut out, "Files scanned", &scan.files_scanned.to_string());
    push_section(&mut out, "Warnings", &warnings.join("\n"));
    out.push('\n');
    out
}

#[test]
fn playground_registries() {
    if !playground_available() {
        return;
    }
    let mut failures = CheckFailures::new();
    for name in PLAYGROUND_PROJECTS {
        let project = scan_project(name);
        let normalizer = Normalizer::new(&project.root);
        let snapshot_dir = conformance_dir().join("playground").join(name);

        let registry = registry_snapshot(&project.scan.registry, &normalizer);
        failures.check(&format!("{name} type registry"), || {
            assert_named_snapshot(&snapshot_dir, "registry", &registry);
        });

        let declarative = serde_json::to_value(&project.scan.declarative_registry)
            .expect("the declarative registry serializes");
        let declarative = normalizer.apply(
            &serde_json::to_string_pretty(&canonical(declarative)).expect("a JSON value prints"),
        );
        failures.check(&format!("{name} declarative registry"), || {
            assert_named_snapshot(&snapshot_dir, "declarative-registry", &declarative);
        });

        let summary = scan_summary(&project.scan, &normalizer);
        failures.check(&format!("{name} scan"), || {
            assert_named_snapshot(&snapshot_dir, "scan", &summary);
        });
    }
    failures.finish("playground registry");
}

/// A scanned registry written as JSON and read back is the registry it was:
/// every lookup answers the same, and the copy records nothing yet.
#[test]
fn registries_round_trip_through_json() {
    if !playground_available() {
        return;
    }
    for name in PLAYGROUND_PROJECTS {
        let project = scan_project(name);

        let json = serde_json::to_string(&project.scan.registry).expect("the registry serializes");
        let restored: TypeRegistry =
            serde_json::from_str(&json).expect("the registry deserializes");
        assert_eq!(
            restored, project.scan.registry,
            "{name}: type registry round trip"
        );
        assert_eq!(
            canonical_json(&restored),
            canonical_json(&project.scan.registry),
            "{name}: the restored registry serializes differently"
        );
        assert_eq!(
            restored.finish_recording(),
            None,
            "{name}: a restored registry must start unrecorded"
        );

        let declarative_json = project
            .scan
            .declarative_registry
            .to_json()
            .expect("the declarative registry serializes");
        let declarative_restored =
            macroforge_ts::host::declarative::ProjectDeclarativeRegistry::from_json(
                &declarative_json,
            )
            .expect("the declarative registry deserializes");
        assert_eq!(
            declarative_restored, project.scan.declarative_registry,
            "{name}: declarative registry round trip"
        );
    }
}

// ---------------------------------------------------------------------------
// The macro check
// ---------------------------------------------------------------------------

/// Every corpus file `gate` rejects must be one expansion leaves untouched:
/// unchanged, silent, with no declarations output. A gate that rejects a file
/// the expander acts on drops that file's macros from every integration.
fn assert_gate_is_a_superset(gate: fn(&str, &str) -> bool) {
    let mut violations = Vec::new();
    let mut accepted_unchanged = Vec::new();

    let mut record = |label: String, source: &str, result: Result<MacroExpansion, String>| {
        // Each label ends with the file's own name, which is what the gate reads.
        let accepted = gate(source, &label);
        match result {
            Ok(expansion) => {
                let untouched = !expansion.changed
                    && expansion.diagnostics.is_empty()
                    && expansion.type_output.is_none()
                    && expansion.code == source;
                if !accepted && !untouched {
                    violations.push(format!("{label}: rejected, but expansion acts on it"));
                }
                if accepted && untouched {
                    accepted_unchanged.push(label);
                }
            }
            Err(error) => {
                if !accepted {
                    violations.push(format!("{label}: rejected, but expansion fails: {error}"));
                }
            }
        }
    };

    let expander = MacroExpander::new().expect("the default expander builds");
    for (directory, suffix) in [
        ("fixtures/ok", ".ts"),
        ("fixtures/error", ".ts"),
        ("fixtures/differential", ".ts"),
        ("fixtures/conformance/dts", ".d.ts"),
    ] {
        for path in fixture_paths(&manifest_dir().join("tests").join(directory), suffix) {
            let input = read_fixture(&path);
            let result = expand_fixture(&expander, &input, fixture_file_name(&path));
            record(path.display().to_string(), &input, result);
        }
    }

    if playground_available() {
        for name in PLAYGROUND_PROJECTS {
            let project = scan_project(name);
            let registries = ProjectRegistries::from(&project.scan);
            for file in project_files(&project) {
                let result = expand_corpus_file(&project, &registries, &file);
                record(format!("{name}/{}", file.relative), &file.source, result);
            }
        }
    }

    eprintln!(
        "the macro check accepts {} corpus file(s) that expansion leaves unchanged:\n{}",
        accepted_unchanged.len(),
        accepted_unchanged.join("\n")
    );
    assert!(
        violations.is_empty(),
        "the macro check rejects files the expander acts on:\n{}",
        violations.join("\n")
    );
}

/// Every file under `directory` whose name ends in `suffix`, in path order.
fn fixture_paths(directory: &Path, suffix: &str) -> Vec<PathBuf> {
    WalkBuilder::new(directory)
        .standard_filters(false)
        .sort_by_file_path(|left, right| left.cmp(right))
        .build()
        .map(|entry| {
            entry.unwrap_or_else(|error| panic!("walking {}: {error}", directory.display()))
        })
        .filter(|entry| entry.path().is_file() && fixture_file_name(entry.path()).ends_with(suffix))
        .map(|entry| entry.into_path())
        .collect()
}

#[test]
fn gate_is_a_strict_superset_of_expansion() {
    assert_gate_is_a_superset(has_macro_annotations);
}
