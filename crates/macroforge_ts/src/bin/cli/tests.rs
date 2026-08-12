#![cfg(feature = "swc")]

use std::path::{Path, PathBuf};

use crate::atomic_fs::{sibling_with_suffix, swap_dir, write_atomic};
use crate::cache::{
    CacheEntry, CacheManifest, compute_external_macro_hash, content_hash, normalized_content_hash,
    warm_cache,
};
use crate::expand::{get_expanded_path, offset_to_line_col, type_surface_rel_path};
use crate::lock::{ProjectLock, resolve_project_root};
use crate::package_expand::is_expandable;
use crate::package_state::{
    PackageState, diff_files, output_fingerprint, project_hash, registry_hash, scan_input,
    state_dir,
};

// =========================================================================
// get_expanded_path tests
// =========================================================================

#[test]
fn test_get_expanded_path_simple_ts() {
    let input = Path::new("src/User.ts");
    let result = get_expanded_path(input);
    assert_eq!(result, PathBuf::from("src/User.expanded.ts"));
}

#[test]
fn test_get_expanded_path_svelte_ts() {
    let input = Path::new("src/lib/demo/types/appointment.svelte.ts");
    let result = get_expanded_path(input);
    assert_eq!(
        result,
        PathBuf::from("src/lib/demo/types/appointment.expanded.svelte.ts")
    );
}

#[test]
fn test_get_expanded_path_tsx() {
    let input = Path::new("components/Button.tsx");
    let result = get_expanded_path(input);
    assert_eq!(result, PathBuf::from("components/Button.expanded.tsx"));
}

#[test]
fn test_get_expanded_path_no_extension() {
    let input = Path::new("src/Makefile");
    let result = get_expanded_path(input);
    assert_eq!(result, PathBuf::from("src/Makefile.expanded"));
}

#[test]
fn test_get_expanded_path_multiple_dots() {
    let input = Path::new("lib/config.test.spec.ts");
    let result = get_expanded_path(input);
    assert_eq!(result, PathBuf::from("lib/config.expanded.test.spec.ts"));
}

#[test]
fn test_get_expanded_path_root_file() {
    let input = Path::new("index.ts");
    let result = get_expanded_path(input);
    assert_eq!(result, PathBuf::from("index.expanded.ts"));
}

// =========================================================================
// type_surface_rel_path tests
// =========================================================================

#[test]
fn test_type_surface_rel_path_simple_ts() {
    let rel = Path::new("User.ts");
    assert_eq!(type_surface_rel_path(rel), PathBuf::from("User.d.ts"));
}

#[test]
fn test_type_surface_rel_path_svelte_ts() {
    let rel = Path::new("types/person-name.svelte.ts");
    assert_eq!(
        type_surface_rel_path(rel),
        PathBuf::from("types/person-name.svelte.d.ts")
    );
}

#[test]
fn test_type_surface_rel_path_tsx() {
    let rel = Path::new("components/Button.tsx");
    assert_eq!(
        type_surface_rel_path(rel),
        PathBuf::from("components/Button.d.ts")
    );
}

#[test]
fn test_type_surface_rel_path_nested() {
    let rel = Path::new("a/b/c/config.ts");
    assert_eq!(
        type_surface_rel_path(rel),
        PathBuf::from("a/b/c/config.d.ts")
    );
}

#[test]
fn test_type_surface_rel_path_root_file() {
    let rel = Path::new("index.ts");
    assert_eq!(type_surface_rel_path(rel), PathBuf::from("index.d.ts"));
}

// =========================================================================
// offset_to_line_col tests
// =========================================================================

#[test]
fn test_offset_to_line_col_first_char() {
    let source = "hello\nworld";
    assert_eq!(offset_to_line_col(source, 0), (1, 1));
}

#[test]
fn test_offset_to_line_col_same_line() {
    let source = "hello\nworld";
    assert_eq!(offset_to_line_col(source, 3), (1, 4)); // 'l' in hello
}

#[test]
fn test_offset_to_line_col_second_line() {
    let source = "hello\nworld";
    assert_eq!(offset_to_line_col(source, 6), (2, 1)); // 'w' in world
}

#[test]
fn test_offset_to_line_col_second_line_middle() {
    let source = "hello\nworld";
    assert_eq!(offset_to_line_col(source, 9), (2, 4)); // 'l' in world
}

#[test]
fn test_offset_to_line_col_multiple_lines() {
    let source = "line1\nline2\nline3";
    assert_eq!(offset_to_line_col(source, 12), (3, 1)); // 'l' in line3
}

#[test]
fn test_offset_to_line_col_empty_lines() {
    let source = "a\n\nb";
    assert_eq!(offset_to_line_col(source, 2), (2, 1)); // empty line
    assert_eq!(offset_to_line_col(source, 3), (3, 1)); // 'b'
}

// =========================================================================
// ImportRegistry::from_module tests (replaces extract_import_sources_from_code)
// =========================================================================

/// Parse code into a Module and build an ImportRegistry.
fn registry_from_code(code: &str) -> macroforge_ts_syn::ImportRegistry {
    use macroforge_ts_syn::parse_ts_module;
    let module = parse_ts_module(code).expect("failed to parse");
    macroforge_ts_syn::ImportRegistry::from_module(&module, code)
}

#[test]
fn test_extract_imports_named() {
    let code = r#"import { DateTime } from 'effect';"#;
    let r = registry_from_code(code);
    let imports = r.source_modules();
    assert_eq!(imports.get("DateTime"), Some(&"effect".to_string()));
}

#[test]
fn test_extract_imports_multiple_named() {
    let code = r#"import { DateTime, Duration } from 'effect';"#;
    let r = registry_from_code(code);
    let imports = r.source_modules();
    assert_eq!(imports.get("DateTime"), Some(&"effect".to_string()));
    assert_eq!(imports.get("Duration"), Some(&"effect".to_string()));
}

#[test]
fn test_extract_imports_type_import() {
    let code = r#"import type { DateTime } from 'effect';"#;
    let r = registry_from_code(code);
    let imports = r.source_modules();
    assert_eq!(imports.get("DateTime"), Some(&"effect".to_string()));
}

#[test]
fn test_extract_imports_default() {
    let code = r#"import React from 'react';"#;
    let r = registry_from_code(code);
    let imports = r.source_modules();
    assert_eq!(imports.get("React"), Some(&"react".to_string()));
}

#[test]
fn test_extract_imports_namespace() {
    let code = r#"import * as Effect from 'effect';"#;
    let r = registry_from_code(code);
    let imports = r.source_modules();
    assert_eq!(imports.get("Effect"), Some(&"effect".to_string()));
}

#[test]
fn test_extract_imports_scoped_package() {
    let code = r#"import { Schema } from '@effect/schema';"#;
    let r = registry_from_code(code);
    let imports = r.source_modules();
    assert_eq!(imports.get("Schema"), Some(&"@effect/schema".to_string()));
}

#[test]
fn test_extract_imports_subpath() {
    let code = r#"import { DateTime } from 'effect/DateTime';"#;
    let r = registry_from_code(code);
    let imports = r.source_modules();
    assert_eq!(
        imports.get("DateTime"),
        Some(&"effect/DateTime".to_string())
    );
}

#[test]
fn test_extract_imports_multiple_statements() {
    let code = r#"
            import { DateTime } from 'effect';
            import { ZonedDateTime } from '@internationalized/date';
            import type { Site } from './site.svelte';
        "#;
    let r = registry_from_code(code);
    let imports = r.source_modules();
    assert_eq!(imports.get("DateTime"), Some(&"effect".to_string()));
    assert_eq!(
        imports.get("ZonedDateTime"),
        Some(&"@internationalized/date".to_string())
    );
    assert_eq!(imports.get("Site"), Some(&"./site.svelte".to_string()));
}

#[test]
fn test_extract_imports_tsx_file() {
    // Note: parse_ts_module uses tsx mode by default in macroforge_ts_syn
    let code = r#"
            import React from 'react';
            import { useState } from 'react';
        "#;
    let r = registry_from_code(code);
    let imports = r.source_modules();
    assert_eq!(imports.get("React"), Some(&"react".to_string()));
    assert_eq!(imports.get("useState"), Some(&"react".to_string()));
}

#[test]
fn test_extract_imports_empty_code() {
    let code = "export {};";
    let r = registry_from_code(code);
    let imports = r.source_modules();
    assert!(imports.is_empty());
}

#[test]
fn test_extract_imports_no_imports() {
    let code = r#"
            const x = 1;
            export function foo() { return x; }
        "#;
    let r = registry_from_code(code);
    let imports = r.source_modules();
    assert!(imports.is_empty());
}

#[test]
fn test_extract_imports_with_jsdoc_decorators() {
    let code = r#"
            import type { DateTime } from 'effect';

            /** @derive(Serialize) */
            interface Event {
                /** @serde({ validate: ["nonEmpty"] }) */
                name: string;
                begins: DateTime.DateTime;
            }
        "#;
    let r = registry_from_code(code);
    let imports = r.source_modules();
    assert_eq!(imports.get("DateTime"), Some(&"effect".to_string()));
}

#[test]
fn test_extract_imports_with_real_decorators() {
    let code = r#"
            import type { DateTime } from 'effect';
            import { Component } from '@angular/core';

            @Component({ selector: 'app-root' })
            class AppComponent {
                begins: DateTime.DateTime;
            }
        "#;
    let r = registry_from_code(code);
    let imports = r.source_modules();
    assert_eq!(imports.get("DateTime"), Some(&"effect".to_string()));
    assert_eq!(imports.get("Component"), Some(&"@angular/core".to_string()));
}

#[test]
fn test_extract_imports_with_alias() {
    let code = r#"
            import type { Option as EffectOption } from 'effect/Option';
            import { DateTime as EffectDateTime } from 'effect';
        "#;
    let r = registry_from_code(code);
    let imports = r.source_modules();
    let aliases = r.aliases();

    assert_eq!(
        imports.get("EffectOption"),
        Some(&"effect/Option".to_string())
    );
    assert_eq!(imports.get("EffectDateTime"), Some(&"effect".to_string()));

    assert_eq!(aliases.get("EffectOption"), Some(&"Option".to_string()));
    assert_eq!(aliases.get("EffectDateTime"), Some(&"DateTime".to_string()));
}

// =========================================================================
// normalized_content_hash tests
// =========================================================================

#[test]
fn test_normalized_hash_trailing_whitespace() {
    let a = "fn foo() {\n    bar();\n}\n";
    let b = "fn foo() {   \n    bar();   \n}   \n";
    assert_eq!(normalized_content_hash(a), normalized_content_hash(b));
}

#[test]
fn test_normalized_hash_blank_lines() {
    let a = "import { x } from 'y';\n\nexport class Foo {}\n";
    let b = "import { x } from 'y';\n\n\n\nexport class Foo {}\n";
    assert_eq!(normalized_content_hash(a), normalized_content_hash(b));
}

#[test]
fn test_normalized_hash_trailing_newlines() {
    let a = "const x = 1;\n";
    let b = "const x = 1;\n\n\n";
    assert_eq!(normalized_content_hash(a), normalized_content_hash(b));
}

#[test]
fn test_normalized_hash_real_change_differs() {
    let a = "const x = 1;\n";
    let b = "const x = 2;\n";
    assert_ne!(normalized_content_hash(a), normalized_content_hash(b));
}

#[test]
fn test_normalized_hash_indentation_change_differs() {
    let a = "  const x = 1;\n";
    let b = "    const x = 1;\n";
    // Leading indentation IS significant
    assert_ne!(normalized_content_hash(a), normalized_content_hash(b));
}

// =========================================================================
// warm_cache normalized_hash backfill test
// =========================================================================

#[test]
fn test_warm_cache_backfills_normalized_hash() {
    // Simulate a manifest entry from before the normalized_hash feature:
    // source_hash matches the file on disk, but normalized_hash is empty.
    // warm_cache should backfill the normalized_hash without re-expanding.

    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let cache_dir = root.join(".macroforge").join("cache");
    std::fs::create_dir_all(&cache_dir).unwrap();

    // Write a source file
    let src_dir = root.join("src");
    std::fs::create_dir_all(&src_dir).unwrap();
    let src_file = src_dir.join("test.ts");
    let source = "const x = 1;\n";
    std::fs::write(&src_file, source).unwrap();

    // Create a manifest with an entry that has a matching source_hash
    // but an empty normalized_hash (simulating an old manifest)
    let source_hash = content_hash(source.as_bytes());
    let mut manifest = CacheManifest::new(
        env!("CARGO_PKG_VERSION").to_string(),
        "none".to_string(),
        "none".to_string(),
    );
    manifest.entries.insert(
        "src/test.ts".to_string(),
        CacheEntry {
            source_hash: source_hash.clone(),
            has_macros: false,
            normalized_hash: String::new(), // empty = old manifest
        },
    );

    // warm_cache should skip re-expansion (source_hash matches)
    // but backfill the normalized_hash
    warm_cache("test", root, &cache_dir, &mut manifest).unwrap();

    let entry = manifest.entries.get("src/test.ts").unwrap();
    assert_eq!(entry.source_hash, source_hash);
    assert!(
        !entry.normalized_hash.is_empty(),
        "normalized_hash should be backfilled"
    );
    assert_eq!(entry.normalized_hash, normalized_content_hash(source));
}

// =========================================================================
// write_atomic tests
// =========================================================================

#[test]
fn test_write_atomic_creates_missing_parents() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("deeply").join("nested").join("out.json");

    write_atomic(&target, b"{}").unwrap();

    assert_eq!(std::fs::read_to_string(&target).unwrap(), "{}");
}

#[test]
fn test_write_atomic_replaces_existing_content() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("registry.json");
    std::fs::write(&target, "old and considerably longer contents").unwrap();

    write_atomic(&target, b"new").unwrap();

    assert_eq!(std::fs::read_to_string(&target).unwrap(), "new");
}

#[test]
fn test_write_atomic_leaves_no_temp_files_behind() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("out.txt");

    write_atomic(&target, b"contents").unwrap();

    let leftovers: Vec<_> = std::fs::read_dir(dir.path())
        .unwrap()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|name| name != "out.txt")
        .collect();
    assert!(
        leftovers.is_empty(),
        "temp files should not survive a successful write: {leftovers:?}"
    );
}

// =========================================================================
// swap_dir tests
// =========================================================================

/// Creates `dir/marker.txt` containing `marker`, creating `dir` if needed.
fn populate_dir(dir: &Path, marker: &str) {
    std::fs::create_dir_all(dir).unwrap();
    std::fs::write(dir.join("marker.txt"), marker).unwrap();
}

#[test]
fn test_swap_dir_replaces_a_populated_destination() {
    let tmp = tempfile::tempdir().unwrap();
    let dest = tmp.path().join("dist");
    let staging = sibling_with_suffix(&dest, "macroforge-staging");
    populate_dir(&dest, "old");
    populate_dir(&staging, "new");
    // A file only the old tree had must not survive the swap.
    std::fs::write(dest.join("stale.txt"), "stale").unwrap();

    swap_dir(&staging, &dest).unwrap();

    assert_eq!(
        std::fs::read_to_string(dest.join("marker.txt")).unwrap(),
        "new"
    );
    assert!(
        !dest.join("stale.txt").exists(),
        "the swap must replace the destination, not merge into it"
    );
    assert!(!staging.exists(), "staging should have been consumed");
}

#[test]
fn test_swap_dir_creates_a_missing_destination() {
    let tmp = tempfile::tempdir().unwrap();
    let dest = tmp.path().join("dist");
    let staging = sibling_with_suffix(&dest, "macroforge-staging");
    populate_dir(&staging, "new");

    swap_dir(&staging, &dest).unwrap();

    assert_eq!(
        std::fs::read_to_string(dest.join("marker.txt")).unwrap(),
        "new"
    );
}

#[test]
fn test_swap_dir_leaves_the_destination_intact_when_staging_is_missing() {
    let tmp = tempfile::tempdir().unwrap();
    let dest = tmp.path().join("dist");
    let staging = sibling_with_suffix(&dest, "macroforge-staging");
    populate_dir(&dest, "old");

    let err = swap_dir(&staging, &dest).unwrap_err();

    assert!(
        err.to_string().contains("does not exist"),
        "unexpected error: {err}"
    );
    assert_eq!(
        std::fs::read_to_string(dest.join("marker.txt")).unwrap(),
        "old",
        "a failed swap must not disturb the existing output"
    );
}

#[test]
fn test_sibling_with_suffix_stays_at_the_same_depth() {
    // svelte-package writes .d.ts.map `sources` relative to the output
    // directory, so a staging path at a different depth would bake the wrong
    // number of `../` segments into the published sourcemaps.
    let dest = Path::new("packages/lib/dist");
    let staging = sibling_with_suffix(dest, "macroforge-staging");

    assert_eq!(staging.parent(), dest.parent());
    assert_eq!(
        staging.components().count(),
        dest.components().count(),
        "staging must sit at the same depth as the destination"
    );
}

// =========================================================================
// ProjectLock tests
// =========================================================================

#[test]
fn test_project_lock_is_reentrant_within_a_process() {
    // A second flock on a different descriptor would deadlock against the
    // first, so nested acquisitions have to be refcounted rather than
    // re-locked. This test hangs rather than fails if that regresses.
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    let outer = ProjectLock::acquire(root, "cache", false).unwrap();
    let inner = ProjectLock::acquire(root, "cache", false).unwrap();

    assert!(root.join(".macroforge").join(".lock").exists());

    drop(inner);
    drop(outer);

    // Fully released, so a fresh acquisition succeeds immediately.
    drop(ProjectLock::acquire(root, "cache", false).unwrap());
}

#[test]
fn test_project_lock_records_its_holder() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    let lock = ProjectLock::acquire(root, "svelte-package", false).unwrap();

    let record = std::fs::read_to_string(root.join(".macroforge").join(".lock")).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&record).unwrap();
    assert_eq!(
        parsed["pid"].as_u64().unwrap(),
        u64::from(std::process::id())
    );
    assert_eq!(parsed["command"].as_str().unwrap(), "svelte-package");

    drop(lock);
}

#[test]
fn test_resolve_project_root_canonicalizes_equivalent_paths() {
    // Two spellings of one project must produce one lock key, or the lock
    // silently stops excluding anything.
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    std::fs::create_dir_all(root.join("src")).unwrap();

    let direct = resolve_project_root(Some(root));
    let indirect = resolve_project_root(Some(&root.join("src").join("..")));

    assert_eq!(direct, indirect);
}

#[test]
fn test_resolve_project_root_keeps_a_path_that_does_not_exist_yet() {
    let tmp = tempfile::tempdir().unwrap();
    let missing = tmp.path().join("not-created-yet");

    assert_eq!(resolve_project_root(Some(&missing)), missing);
}

// =========================================================================
// svelte-package build state tests
// =========================================================================

fn no_extensions() -> Vec<String> {
    Vec::new()
}

fn write(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, contents).unwrap();
}

#[test]
fn test_scan_input_sees_every_file_the_packager_does() {
    // `@sveltejs/package` walks the input directory with a plain readdir: no
    // gitignore, no extension filter, dotfiles included. Anything this scan
    // hides is a file whose change would go unnoticed.
    let tmp = tempfile::tempdir().unwrap();
    let input = tmp.path().join("lib");

    write(&input.join(".npmignore"), "x\n");
    write(&input.join("logo.svg"), "<svg/>\n");
    write(&input.join("nested/a.ts"), "export const a = 1;\n");
    write(&tmp.path().join(".gitignore"), "lib/nested\n");

    let stamps = scan_input(&input, &no_extensions()).unwrap();
    let found: Vec<&str> = stamps.keys().map(String::as_str).collect();

    assert_eq!(found, vec![".npmignore", "logo.svg", "nested/a.ts"]);
}

#[test]
fn test_scan_input_ignores_formatting_in_code_but_not_in_data() {
    // A `.ts` file is transpiled on the way into the package, so its layout is
    // discarded anyway. A `.json` file is copied through byte for byte.
    let tmp = tempfile::tempdir().unwrap();
    let input = tmp.path().join("lib");

    write(&input.join("a.ts"), "const x = 1;\n");
    write(&input.join("a.json"), "{\"x\":1}\n");
    let before = scan_input(&input, &no_extensions()).unwrap();

    write(&input.join("a.ts"), "const x = 1;   \n\n\n");
    write(&input.join("a.json"), "{\"x\":1}   \n\n\n");
    let after = scan_input(&input, &no_extensions()).unwrap();

    let changes = diff_files(&before, &after);
    assert_eq!(changes.changed, vec!["a.json"]);
    assert!(changes.removed.is_empty());
}

#[test]
fn test_configured_component_extensions_ignore_formatting_too() {
    // `config.extensions` can name something other than `.svelte`, and those
    // files are components rather than data.
    let tmp = tempfile::tempdir().unwrap();
    let input = tmp.path().join("lib");
    let extensions = vec![".svx".to_string()];

    write(&input.join("a.svx"), "# title\n");
    write(&input.join("b.txt"), "title\n");
    let before = scan_input(&input, &extensions).unwrap();

    write(&input.join("a.svx"), "# title   \n\n");
    write(&input.join("b.txt"), "title   \n\n");
    let after = scan_input(&input, &extensions).unwrap();

    let changes = diff_files(&before, &after);
    assert_eq!(changes.changed, vec!["b.txt"]);
}

#[test]
fn test_diff_files_reports_additions_and_removals() {
    let tmp = tempfile::tempdir().unwrap();
    let input = tmp.path().join("lib");

    write(&input.join("keep.ts"), "const keep = 1;\n");
    write(&input.join("gone.ts"), "const gone = 1;\n");
    let before = scan_input(&input, &no_extensions()).unwrap();

    std::fs::remove_file(input.join("gone.ts")).unwrap();
    write(&input.join("added.ts"), "const added = 1;\n");
    let after = scan_input(&input, &no_extensions()).unwrap();

    let changes = diff_files(&before, &after);
    assert_eq!(changes.changed, vec!["added.ts"]);
    assert_eq!(changes.removed, vec!["gone.ts"]);
}

#[test]
fn test_output_fingerprint_reports_a_missing_directory() {
    let tmp = tempfile::tempdir().unwrap();

    assert_eq!(
        output_fingerprint(&tmp.path().join("dist")).unwrap(),
        "missing"
    );
}

#[test]
fn test_output_fingerprint_tracks_content_size_not_mtime() {
    let tmp = tempfile::tempdir().unwrap();
    let dist = tmp.path().join("dist");

    write(&dist.join("index.js"), "export const a = 1;\n");
    let original = output_fingerprint(&dist).unwrap();

    // Rewriting identical bytes moves the mtime and nothing else; a rebuild
    // triggered by that alone would make every `touch` a full repackage.
    write(&dist.join("index.js"), "export const a = 1;\n");
    assert_eq!(output_fingerprint(&dist).unwrap(), original);

    write(&dist.join("index.js"), "export const a = 12;\n");
    assert_ne!(output_fingerprint(&dist).unwrap(), original);
}

#[test]
fn test_output_fingerprint_notices_a_deleted_file() {
    let tmp = tempfile::tempdir().unwrap();
    let dist = tmp.path().join("dist");

    write(&dist.join("index.js"), "export const a = 1;\n");
    write(&dist.join("types/user.js"), "export const b = 2;\n");
    let original = output_fingerprint(&dist).unwrap();

    std::fs::remove_file(dist.join("types/user.js")).unwrap();
    assert_ne!(output_fingerprint(&dist).unwrap(), original);
}

#[test]
fn test_registry_hash_ignores_key_order() {
    // The registries are `HashMap`-backed, so two scans of identical sources
    // serialize their keys in whatever order the map iterates. Hashing the raw
    // text would report a changed type surface on every single build.
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join(".macroforge");

    write(&dir.join("declarative-registry.json"), "{}");
    write(
        &dir.join("type-registry.json"),
        r#"{"types":{"A":{"name":"A"},"B":{"name":"B"}}}"#,
    );
    let original = registry_hash(tmp.path());

    write(
        &dir.join("type-registry.json"),
        r#"{"types":{"B":{"name":"B"},"A":{"name":"A"}}}"#,
    );
    assert_eq!(registry_hash(tmp.path()), original);
}

#[test]
fn test_registry_hash_notices_a_changed_type() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join(".macroforge");

    write(&dir.join("declarative-registry.json"), "{}");
    write(
        &dir.join("type-registry.json"),
        r#"{"types":{"A":{"f":1}}}"#,
    );
    let original = registry_hash(tmp.path());

    write(
        &dir.join("type-registry.json"),
        r#"{"types":{"A":{"f":2}}}"#,
    );
    assert_ne!(registry_hash(tmp.path()), original);
}

#[test]
fn test_project_hash_skips_dependencies_and_the_output_directory() {
    // The output directory is a build's own product. Counting it as an input
    // would make every successful run invalidate the next one.
    let tmp = tempfile::tempdir().unwrap();
    let dist = tmp.path().join("dist");

    write(&tmp.path().join("src/lib/a.ts"), "export const a = 1;\n");
    write(&dist.join("a.js"), "export const a = 1;\n");
    let original = project_hash(tmp.path(), &dist).unwrap();

    write(&dist.join("a.js"), "export const a = 999;\n");
    write(&dist.join("a.ts"), "export const a = 999;\n");
    write(
        &tmp.path().join("node_modules/dep/index.ts"),
        "export {};\n",
    );
    assert_eq!(project_hash(tmp.path(), &dist).unwrap(), original);

    write(&tmp.path().join("src/lib/a.ts"), "export const a = 2;\n");
    assert_ne!(project_hash(tmp.path(), &dist).unwrap(), original);
}

#[test]
fn test_project_hash_ignores_formatting() {
    let tmp = tempfile::tempdir().unwrap();
    let dist = tmp.path().join("dist");

    write(&tmp.path().join("src/a.ts"), "export const a = 1;\n");
    let original = project_hash(tmp.path(), &dist).unwrap();

    write(&tmp.path().join("src/a.ts"), "export const a = 1;   \n\n\n");
    assert_eq!(project_hash(tmp.path(), &dist).unwrap(), original);
}

#[test]
fn test_unreadable_state_is_treated_as_a_first_run() {
    // State is a cache, not a source of truth. A file truncated by a crash or
    // written by a future version has to degrade into a full rebuild, because
    // failing the build over it would leave no way to recover except by hand.
    let tmp = tempfile::tempdir().unwrap();

    write(&state_dir(tmp.path()).join("state.json"), "{ not json");
    assert!(PackageState::load(tmp.path()).is_none());

    write(
        &state_dir(tmp.path()).join("state.json"),
        r#"{"inputs":{"version":"0.0.0"}}"#,
    );
    assert!(PackageState::load(tmp.path()).is_none());
}

#[test]
fn test_external_macro_hash_finds_a_workspace_root_package() {
    // A workspace installs the macro package once, at the repository root, so a
    // package building from `apps/web` has no `node_modules` of its own. Missing
    // it pins the hash at "none", and a rebuilt macro then invalidates nothing —
    // every consumer silently keeps serving the previous build's expansions.
    let tmp = tempfile::tempdir().unwrap();
    let workspace = tmp.path();
    let project = workspace.join("apps/web");
    std::fs::create_dir_all(&project).unwrap();

    let macros = workspace.join("node_modules/@acme/macros");
    write(
        &macros.join("index.js"),
        "exports.__macroforgeRunThing = () => {};",
    );
    write(&macros.join("macros.darwin-x64.node"), "binary");

    let found = compute_external_macro_hash(&project);
    assert_ne!(
        found, "none",
        "a macro package at the workspace root should be found from a nested project"
    );

    // And rebuilding it has to move the hash, or nothing downstream reruns.
    write(&macros.join("macros.darwin-x64.node"), "a different binary");
    assert_ne!(
        compute_external_macro_hash(&project),
        found,
        "a rebuilt macro artifact should invalidate the cache"
    );
}

#[test]
fn test_external_macro_hash_ignores_packages_without_the_marker() {
    let tmp = tempfile::tempdir().unwrap();
    let project = tmp.path().join("apps/web");
    std::fs::create_dir_all(&project).unwrap();

    let plain = tmp.path().join("node_modules/left-pad");
    write(&plain.join("index.js"), "module.exports = () => {};");

    assert_eq!(compute_external_macro_hash(&project), "none");
}

#[test]
fn test_is_expandable_covers_sources_but_not_declarations() {
    assert!(is_expandable("types/user.ts"));
    assert!(is_expandable("types/order.svelte.ts"));
    assert!(is_expandable("component.tsx"));

    assert!(!is_expandable("types/user.d.ts"));
    assert!(!is_expandable("index.js"));
    assert!(!is_expandable("Card.svelte"));
    assert!(!is_expandable("styles.css"));
}
