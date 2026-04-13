//! End-to-end tests exercising the full pipeline with `@buildtime`.
//!
//! Calls the public `expand_sync` entry point (via `expand_inner`)
//! to confirm:
//! 1. `has_macro_annotations` triggers on `@buildtime`.
//! 2. The buildtime pre-pass runs, rewrites the source.
//! 3. The rewrite flows through the declarative + derive passes
//!    unchanged, showing up in `ExpandResult::code`.
//! 4. `buildtime_dependencies` are surfaced on the result.
//! 5. Diagnostics from failed buildtime blocks propagate.

use crate::api_types::ExpandOptions;
use crate::expand_core::expand_inner;

fn default_options() -> ExpandOptions {
    ExpandOptions {
        keep_decorators: None,
        external_decorator_modules: None,
        config_path: None,
        type_registry_json: None,
        declarative_registry_json: None,
        build_mode: None,
    }
}

fn expand(code: &str) -> crate::api_types::ExpandResult {
    expand_inner(code, "/tmp/fixture.ts", Some(default_options())).expect("expand_inner failed")
}

#[test]
fn pipeline_rewrites_tier1_const() {
    let src = r#"/** @buildtime */
const ANSWER = 6 * 7;
"#;
    let result = expand(src);
    assert!(
        result.code.contains("const ANSWER = 42;"),
        "code: {}",
        result.code
    );
    assert!(
        result.diagnostics.iter().all(|d| d.level != "error"),
        "unexpected diagnostics: {:?}",
        result.diagnostics
    );
}

#[test]
fn pipeline_passes_through_when_no_buildtime() {
    let src = r#"const X = 1;
const Y = 2;
"#;
    let result = expand(src);
    // No @buildtime, no @derive, no macros — should round-trip untouched
    // via the early bailout path.
    assert_eq!(result.code, src);
    assert!(result.buildtime_dependencies.is_empty());
}

#[test]
fn pipeline_surfaces_buildtime_dependencies() {
    use std::io::Write;
    let mut tmp = tempfile::NamedTempFile::new().unwrap();
    writeln!(tmp.as_file_mut(), r#"{{"pi":3.14}}"#).unwrap();
    let path_lit = serde_json::to_string(&tmp.path().to_string_lossy().into_owned()).unwrap();

    let src = format!(
        r#"/** @buildtime */
const PI = buildtime.fs.readJson({path_lit}).pi;
"#
    );
    let result = expand(&src);
    assert!(
        result.code.contains("const PI = 3.14;"),
        "code: {}",
        result.code
    );
    assert_eq!(result.buildtime_dependencies.len(), 1);
    let dep = &result.buildtime_dependencies[0];
    assert_eq!(
        std::path::PathBuf::from(dep),
        tmp.path().to_path_buf(),
        "dep path: {dep}"
    );
}

#[test]
fn pipeline_emits_error_diagnostic_on_throw() {
    let src = r#"/** @buildtime */
const BAD = (() => { throw new Error("bang"); })();
"#;
    let result = expand(src);
    let errors: Vec<_> = result
        .diagnostics
        .iter()
        .filter(|d| d.level == "error")
        .collect();
    assert_eq!(errors.len(), 1, "diags: {:?}", result.diagnostics);
    assert!(
        errors[0].message.contains("bang") || errors[0].message.contains("threw"),
        "error message: {}",
        errors[0].message
    );
}

#[test]
fn pipeline_buildtime_then_derive() {
    // Confirm that a @derive class can sit alongside a @buildtime
    // constant without either pass interfering with the other. This
    // is the load-bearing test for the "buildtime runs first" design
    // choice: the derive macro sees the spliced literal as plain source.
    let src = r#"/** @buildtime */
const CLASS_NAME = "User";

/** @derive(Debug) */
class User {
  name: string = "";
}
"#;
    let result = expand(src);
    assert!(
        result.code.contains("const CLASS_NAME = \"User\";"),
        "code: {}",
        result.code
    );
    // Debug derive injects a `toString` method.
    assert!(
        result.code.contains("toString"),
        "expected Debug derive output, code: {}",
        result.code
    );
    // No errors.
    assert!(
        result.diagnostics.iter().all(|d| d.level != "error"),
        "unexpected errors: {:?}",
        result.diagnostics
    );
}
