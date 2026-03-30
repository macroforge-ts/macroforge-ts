use std::path::{Path, PathBuf};

use crate::cache::{
    CacheEntry, CacheManifest, content_hash, normalized_content_hash, warm_cache,
};
use crate::expand::{get_expanded_path, offset_to_line_col};

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
        true,
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
    warm_cache("test", root, &cache_dir, &mut manifest, true).unwrap();

    let entry = manifest.entries.get("src/test.ts").unwrap();
    assert_eq!(entry.source_hash, source_hash);
    assert!(
        !entry.normalized_hash.is_empty(),
        "normalized_hash should be backfilled"
    );
    assert_eq!(entry.normalized_hash, normalized_content_hash(source));
}
