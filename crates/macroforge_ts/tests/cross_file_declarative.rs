//! End-to-end test for cross-file declarative macro imports.
//!
//! Builds a tempdir containing two files:
//! - `macros.ts` — declares `const $vec = macroRules\`...\``
//! - `consumer.ts` — imports `$vec` via `/** import macro { $vec } from "./macros" */`
//!   and calls it
//!
//! Runs `ProjectScanner` to produce a project-wide declarative registry,
//! hands it to a `MacroExpander`, and verifies the consumer file's call
//! sites expand using the definition from `macros.ts`.

use macroforge_ts::host::MacroExpander;
use macroforge_ts::host::scanner::{ProjectScanner, ScanConfig};

#[test]
fn cross_file_import_expands_from_library() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path().to_path_buf();

    let macros_path = root.join("macros.ts");
    std::fs::write(
        &macros_path,
        r#"import { macroRules } from "macroforge/rules";

export const $vec = macroRules`
  () => []
  ($($x:Expr),+) => [$($x),+]
`;
"#,
    )
    .expect("write macros.ts");

    let consumer_path = root.join("consumer.ts");
    std::fs::write(
        &consumer_path,
        r#"/** import macro { $vec } from "./macros" */
const empty = $vec();
const xs = $vec(1, 2, 3);
"#,
    )
    .expect("write consumer.ts");

    // Run the project scanner — this walks the tempdir, parses both files,
    // and produces both the type registry (empty here) and the declarative
    // registry (should contain `$vec` from macros.ts).
    let config = ScanConfig {
        root_dir: root.clone(),
        ..ScanConfig::default()
    };
    let scanner = ProjectScanner::new(config);
    let output = scanner.scan().expect("scan");

    assert_eq!(
        output.declarative_registry.macro_count(),
        1,
        "expected exactly one declarative macro in the project, got {}",
        output.declarative_registry.macro_count()
    );
    assert_eq!(
        output.declarative_registry.file_count(),
        1,
        "expected the macro to live in exactly one file"
    );

    // Expand the consumer file with the project registry installed.
    let consumer_src = std::fs::read_to_string(&consumer_path).expect("read consumer");
    let mut expander = MacroExpander::new().expect("expander");
    expander.set_declarative_registry(Some(output.declarative_registry));

    let expansion = expander
        .expand_source(&consumer_src, &consumer_path.to_string_lossy())
        .expect("expand");

    assert!(
        expansion.changed,
        "expected the expansion to rewrite the consumer file"
    );

    // The expansions should be inlined — no `$vec(` should remain.
    assert!(
        !expansion.code.contains("$vec("),
        "expected $vec(...) call sites to be rewritten, got:\n{}",
        expansion.code
    );
    // `$vec()` → `[]`
    assert!(
        expansion.code.contains("const empty = []"),
        "expected empty = []; got:\n{}",
        expansion.code
    );
    // `$vec(1, 2, 3)` → `[1, 2, 3]`
    assert!(
        expansion
            .code
            .matches(char::is_numeric)
            .collect::<Vec<_>>()
            .len()
            >= 3,
        "expected the rewritten code to contain 1, 2, 3; got:\n{}",
        expansion.code
    );
    // Diagnostics should be empty.
    assert!(
        expansion
            .diagnostics
            .iter()
            .all(|d| !matches!(d.level, macroforge_ts::ts_syn::abi::DiagnosticLevel::Error)),
        "expected no error-level diagnostics, got: {:?}",
        expansion.diagnostics
    );
}

#[test]
fn cross_file_unresolved_import_reports_diagnostic() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path().to_path_buf();

    // No macros.ts. Consumer imports from a file that doesn't exist.
    let consumer_path = root.join("consumer.ts");
    std::fs::write(
        &consumer_path,
        r#"/** import macro { $vec } from "./nonexistent" */
const xs = $vec(1, 2, 3);
"#,
    )
    .expect("write consumer.ts");

    let config = ScanConfig {
        root_dir: root.clone(),
        ..ScanConfig::default()
    };
    let scanner = ProjectScanner::new(config);
    let output = scanner.scan().expect("scan");

    assert_eq!(output.declarative_registry.macro_count(), 0);

    let consumer_src = std::fs::read_to_string(&consumer_path).expect("read consumer");
    let mut expander = MacroExpander::new().expect("expander");
    expander.set_declarative_registry(Some(output.declarative_registry));

    let expansion = expander
        .expand_source(&consumer_src, &consumer_path.to_string_lossy())
        .expect("expand");

    // Expansion should surface a diagnostic about the unresolved specifier.
    // It should NOT silently pass through.
    let has_error = expansion.diagnostics.iter().any(|d| {
        matches!(d.level, macroforge_ts::ts_syn::abi::DiagnosticLevel::Error)
            && d.message.contains("cannot resolve")
    });
    assert!(
        has_error,
        "expected a 'cannot resolve' diagnostic, got: {:?}",
        expansion.diagnostics
    );
}
