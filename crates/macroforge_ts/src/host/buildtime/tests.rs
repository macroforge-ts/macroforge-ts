//! Unit tests for the buildtime sandbox + serializer + capabilities.
//!
//! Only runs when a backend feature is enabled. When no backend is on,
//! the file still compiles (so `cargo check` works in bare-feature mode)
//! but the tests themselves are gated behind `buildtime-boa`.

#![cfg(feature = "buildtime-boa")]

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::Duration;

use crate::host::buildtime::backends::boa::BoaSandbox;
use crate::host::buildtime::capabilities::{CapabilitySet, PathPattern};
use crate::host::buildtime::sandbox::{
    BuildtimeSandbox, SandboxError, SandboxOptions, SandboxValue,
};
use crate::host::buildtime::serialize::value_to_ts_source;

fn default_options() -> SandboxOptions {
    let mut opts = SandboxOptions::new(PathBuf::from("/tmp/macroforge_test.ts"));
    opts.capabilities = CapabilitySet::unrestricted();
    opts.timeout = Duration::from_secs(5);
    opts
}

#[test]
fn evaluate_arithmetic() {
    let sandbox = BoaSandbox::new();
    let result = sandbox
        .evaluate(
            "return 1 + 1;",
            &PathBuf::from("<test>"),
            &default_options(),
        )
        .expect("evaluation failed");
    assert_eq!(result.value, SandboxValue::Number(2.0));
}

#[test]
fn evaluate_string() {
    let sandbox = BoaSandbox::new();
    let result = sandbox
        .evaluate(
            "return 'hello' + ' world';",
            &PathBuf::from("<test>"),
            &default_options(),
        )
        .expect("evaluation failed");
    assert_eq!(
        result.value,
        SandboxValue::String("hello world".to_string())
    );
}

#[test]
fn evaluate_array() {
    let sandbox = BoaSandbox::new();
    let result = sandbox
        .evaluate(
            "return [1, 2, 3];",
            &PathBuf::from("<test>"),
            &default_options(),
        )
        .expect("evaluation failed");
    match result.value {
        SandboxValue::Array(items) => {
            assert_eq!(items.len(), 3);
            assert_eq!(items[0], SandboxValue::Number(1.0));
            assert_eq!(items[1], SandboxValue::Number(2.0));
            assert_eq!(items[2], SandboxValue::Number(3.0));
        }
        other => panic!("expected array, got {:?}", other),
    }
}

#[test]
fn evaluate_object() {
    let sandbox = BoaSandbox::new();
    let result = sandbox
        .evaluate(
            "return { a: 1, b: 'two', c: true };",
            &PathBuf::from("<test>"),
            &default_options(),
        )
        .expect("evaluation failed");
    match result.value {
        SandboxValue::Object(map) => {
            assert_eq!(map.get("a"), Some(&SandboxValue::Number(1.0)));
            assert_eq!(map.get("b"), Some(&SandboxValue::String("two".to_string())));
            assert_eq!(map.get("c"), Some(&SandboxValue::Bool(true)));
        }
        other => panic!("expected object, got {:?}", other),
    }
}

#[test]
fn evaluate_null_and_undefined() {
    let sandbox = BoaSandbox::new();
    let null_result = sandbox
        .evaluate("return null;", &PathBuf::from("<test>"), &default_options())
        .expect("null evaluation failed");
    assert_eq!(null_result.value, SandboxValue::Null);

    let undef_result = sandbox
        .evaluate(
            "return undefined;",
            &PathBuf::from("<test>"),
            &default_options(),
        )
        .expect("undefined evaluation failed");
    assert_eq!(undef_result.value, SandboxValue::Undefined);
}

#[test]
fn throws_propagate_as_threw() {
    let sandbox = BoaSandbox::new();
    let err = sandbox
        .evaluate(
            "throw new Error('boom');",
            &PathBuf::from("<test>"),
            &default_options(),
        )
        .unwrap_err();
    match err {
        SandboxError::Threw { message, .. } => {
            assert!(message.contains("boom"), "message was: {message}");
        }
        other => panic!("expected Threw, got {:?}", other),
    }
}

#[test]
fn functions_are_unserializable() {
    let sandbox = BoaSandbox::new();
    let err = sandbox
        .evaluate(
            "return () => 1;",
            &PathBuf::from("<test>"),
            &default_options(),
        )
        .unwrap_err();
    assert!(matches!(err, SandboxError::UnserializableResult { .. }));
}

#[test]
fn date_instances_are_unserializable() {
    let sandbox = BoaSandbox::new();
    let err = sandbox
        .evaluate(
            "return new Date();",
            &PathBuf::from("<test>"),
            &default_options(),
        )
        .unwrap_err();
    match err {
        SandboxError::UnserializableResult { kind } => {
            assert!(
                kind.contains("Date") || kind.contains("class"),
                "kind: {kind}"
            );
        }
        other => panic!("expected UnserializableResult, got {:?}", other),
    }
}

#[test]
fn timeout_fires_on_infinite_loop() {
    let sandbox = BoaSandbox::new();
    let mut opts = default_options();
    opts.timeout = Duration::from_millis(100);
    let err = sandbox
        .evaluate("while (true) {}", &PathBuf::from("<test>"), &opts)
        .unwrap_err();
    assert!(matches!(err, SandboxError::Timeout { .. }));
}

#[test]
fn capability_rejects_unauthorized_read() {
    let sandbox = BoaSandbox::new();
    // No fs_read patterns means every read is denied.
    let mut opts = SandboxOptions::new(PathBuf::from("/tmp/macroforge_test.ts"));
    opts.capabilities = CapabilitySet::default();
    opts.timeout = Duration::from_secs(5);
    let err = sandbox
        .evaluate(
            "return buildtime.fs.readText('/etc/passwd');",
            &PathBuf::from("<test>"),
            &opts,
        )
        .unwrap_err();
    assert!(
        matches!(err, SandboxError::UnauthorizedRead { .. }),
        "got {:?}",
        err
    );
}

#[test]
fn authorized_read_succeeds_and_records_dependency() {
    use std::io::Write;
    let tmp = tempfile::NamedTempFile::new().expect("tempfile");
    writeln!(tmp.as_file(), "hello from disk").unwrap();
    let path = tmp.path().to_path_buf();

    let sandbox = BoaSandbox::new();
    let mut opts = SandboxOptions::new(PathBuf::from("/tmp/macroforge_test.ts"));
    opts.capabilities = CapabilitySet {
        fs_read: vec![PathPattern::new("**").unwrap()],
        fs_write: vec![],
        env_allow: vec![],
        network: false,
    };
    opts.timeout = Duration::from_secs(5);

    let script = format!(
        "return buildtime.fs.readText({});",
        serde_json::to_string(&path.to_string_lossy().into_owned()).unwrap()
    );
    let result = sandbox
        .evaluate(&script, &PathBuf::from("<test>"), &opts)
        .expect("evaluation failed");
    match result.value {
        SandboxValue::String(s) => assert!(s.contains("hello from disk"), "got: {s}"),
        other => panic!("expected string, got {:?}", other),
    }
    assert_eq!(result.dependencies.len(), 1);
    assert_eq!(result.dependencies[0], path);
}

#[test]
fn read_json_returns_parsed_value() {
    use std::io::Write;
    let mut tmp = tempfile::NamedTempFile::new().expect("tempfile");
    writeln!(tmp.as_file_mut(), r#"{{"name":"test","count":7}}"#).unwrap();
    let path = tmp.path().to_path_buf();

    let sandbox = BoaSandbox::new();
    let mut opts = SandboxOptions::new(PathBuf::from("/tmp/macroforge_test.ts"));
    opts.capabilities = CapabilitySet {
        fs_read: vec![PathPattern::new("**").unwrap()],
        fs_write: vec![],
        env_allow: vec![],
        network: false,
    };
    opts.timeout = Duration::from_secs(5);

    let script = format!(
        "return buildtime.fs.readJson({});",
        serde_json::to_string(&path.to_string_lossy().into_owned()).unwrap()
    );
    let result = sandbox
        .evaluate(&script, &PathBuf::from("<test>"), &opts)
        .expect("evaluation failed");
    match result.value {
        SandboxValue::Object(map) => {
            assert_eq!(
                map.get("name"),
                Some(&SandboxValue::String("test".to_string()))
            );
            assert_eq!(map.get("count"), Some(&SandboxValue::Number(7.0)));
        }
        other => panic!("expected object, got {:?}", other),
    }
}

#[test]
fn crypto_sha256_is_stable() {
    let sandbox = BoaSandbox::new();
    let result = sandbox
        .evaluate(
            "return buildtime.crypto.sha256('hello');",
            &PathBuf::from("<test>"),
            &default_options(),
        )
        .expect("evaluation failed");
    match result.value {
        SandboxValue::String(s) => {
            assert_eq!(
                s,
                "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
            );
        }
        other => panic!("expected string, got {:?}", other),
    }
}

#[test]
fn serialize_round_trip_through_sandbox() {
    let sandbox = BoaSandbox::new();
    let result = sandbox
        .evaluate(
            "return { n: 42, s: 'hi', arr: [1, 2, 3], nested: { flag: true } };",
            &PathBuf::from("<test>"),
            &default_options(),
        )
        .expect("evaluation failed");
    let ts = value_to_ts_source(&result.value).expect("serialize");
    // Every SandboxValue must round-trip to some valid TS expression.
    assert!(ts.contains("n: 42"));
    assert!(ts.contains(r#"s: "hi""#));
    assert!(ts.contains("arr: [1, 2, 3]"));
    assert!(ts.contains("flag: true"));
}

#[test]
fn async_await_inside_script_works() {
    let sandbox = BoaSandbox::new();
    // Resolved-immediately promise: QuickJS runs the microtask queue
    // when we ask for the value.
    let result = sandbox.evaluate(
        "return Promise.resolve(99).then(x => x + 1);",
        &PathBuf::from("<test>"),
        &default_options(),
    );
    // PR 1 does not implement the async-boundary; PR 10 does. Either a
    // Promise (unserializable) or the resolved value is acceptable here
    // — what we DON'T want is a hang or a hard panic.
    match result {
        Ok(eval) => {
            // Fine if the backend resolved the promise synchronously.
            assert!(
                matches!(eval.value, SandboxValue::Number(_))
                    || matches!(eval.value, SandboxValue::Object(_)),
                "unexpected async result: {:?}",
                eval.value
            );
        }
        Err(SandboxError::UnserializableResult { .. }) => {
            // Also fine: PR 1 treats bare promises as unserializable.
        }
        Err(other) => panic!("unexpected error: {:?}", other),
    }
}

#[test]
fn env_allowlist_exposes_approved_vars() {
    unsafe {
        std::env::set_var("MACROFORGE_TEST_ENV", "value");
    }
    let sandbox = BoaSandbox::new();
    let mut opts = SandboxOptions::new(PathBuf::from("/tmp/macroforge_test.ts"));
    opts.capabilities = CapabilitySet {
        fs_read: vec![],
        fs_write: vec![],
        env_allow: vec!["MACROFORGE_TEST_ENV".to_string()],
        network: false,
    };
    opts.timeout = Duration::from_secs(5);

    let result = sandbox
        .evaluate(
            "return buildtime.env.MACROFORGE_TEST_ENV;",
            &PathBuf::from("<test>"),
            &opts,
        )
        .expect("evaluation failed");
    assert_eq!(result.value, SandboxValue::String("value".to_string()));
}

#[test]
fn env_not_in_allowlist_is_undefined() {
    let sandbox = BoaSandbox::new();
    let result = sandbox
        .evaluate(
            "return typeof buildtime.env.SOMETHING_NOT_ALLOWED;",
            &PathBuf::from("<test>"),
            &default_options(),
        )
        .expect("evaluation failed");
    // Not in allowlist → not on the env object → typeof is "undefined".
    assert_eq!(result.value, SandboxValue::String("undefined".to_string()));
}

#[test]
fn location_matches_options() {
    let sandbox = BoaSandbox::new();
    let mut opts = default_options();
    opts.source_file = PathBuf::from("/src/user.ts");
    opts.source_line = 42;
    opts.source_column = 3;

    let result = sandbox
        .evaluate(
            "return { f: buildtime.location.file, l: buildtime.location.line, c: buildtime.location.column };",
            &PathBuf::from("<test>"),
            &opts,
        )
        .expect("evaluation failed");
    match result.value {
        SandboxValue::Object(map) => {
            assert_eq!(
                map.get("f"),
                Some(&SandboxValue::String("/src/user.ts".to_string()))
            );
            assert_eq!(map.get("l"), Some(&SandboxValue::Number(42.0)));
            assert_eq!(map.get("c"), Some(&SandboxValue::Number(3.0)));
        }
        other => panic!("expected object, got {:?}", other),
    }
}

#[test]
fn capability_default_allows_nothing() {
    let caps = CapabilitySet::default();
    assert!(caps.check_read(&PathBuf::from("/tmp/any")).is_err());
    assert!(caps.check_write(&PathBuf::from("/tmp/any")).is_err());
    assert!(caps.check_env("NODE_ENV").is_err());
    assert!(caps.check_network("https://example.com").is_err());
}

#[test]
fn path_pattern_matches() {
    let pat = PathPattern::new("**/*.json").unwrap();
    assert!(pat.matches(&PathBuf::from("/tmp/schema.json")));
    assert!(pat.matches(&PathBuf::from("/src/data/user.json")));
    assert!(!pat.matches(&PathBuf::from("/tmp/schema.yaml")));
}

// These tests don't exercise the sandbox but live here to keep
// capability/serializer regression coverage adjacent to the sandbox
// tests they pair with.

// ---------------------------------------------------------------------
// Discovery + prepass integration tests
// ---------------------------------------------------------------------

#[cfg(feature = "oxc")]
fn run_prepass_fixture(source: &str) -> crate::host::buildtime::prepass::PrepassOutput {
    use crate::host::buildtime::prepass::run_prepass;
    use crate::host::buildtime::sandbox::SandboxOptions;
    use oxc::allocator::Allocator;
    use oxc::parser::Parser as OxcParser;
    use oxc::span::SourceType;

    let allocator = Allocator::default();
    let ret = OxcParser::new(&allocator, source, SourceType::ts()).parse();
    assert!(
        ret.errors.is_empty(),
        "parse errors in fixture: {:?}",
        ret.errors
    );
    let sandbox = BoaSandbox::new();
    let mut opts = SandboxOptions::new(PathBuf::from("/tmp/fixture.ts"));
    opts.capabilities = CapabilitySet::unrestricted();
    opts.timeout = Duration::from_secs(5);
    run_prepass(
        &ret.program,
        source,
        &PathBuf::from("/tmp/fixture.ts"),
        &sandbox,
        &opts,
    )
}

#[cfg(feature = "oxc")]
#[test]
fn prepass_rewrites_tier1_const() {
    let src = r#"/** @buildtime */
const ANSWER = 6 * 7;
"#;
    let out = run_prepass_fixture(src);
    assert!(
        out.diagnostics.is_empty(),
        "diagnostics: {:?}",
        out.diagnostics
    );
    let rewritten = out.rewritten.expect("expected rewrite");
    assert!(rewritten.contains("const ANSWER = 42;"), "got: {rewritten}");
    assert!(out.dependencies.is_empty());
}

#[cfg(feature = "oxc")]
#[test]
fn prepass_preserves_export_keyword() {
    let src = r#"/** @buildtime */
export const ANSWER = 6 * 7;
"#;
    let out = run_prepass_fixture(src);
    assert!(
        out.diagnostics.is_empty(),
        "diagnostics: {:?}",
        out.diagnostics
    );
    let rewritten = out.rewritten.expect("expected rewrite");
    assert!(
        rewritten.contains("export const ANSWER = 42;"),
        "got: {rewritten}"
    );
}

#[cfg(feature = "oxc")]
#[test]
fn prepass_leaves_non_annotated_decls_alone() {
    let src = r#"const NORMAL = 1;

/** @buildtime */
const ANSWER = 6 * 7;

const AFTER = 2;
"#;
    let out = run_prepass_fixture(src);
    assert!(
        out.diagnostics.is_empty(),
        "diagnostics: {:?}",
        out.diagnostics
    );
    let rewritten = out.rewritten.expect("expected rewrite");
    assert!(rewritten.contains("const NORMAL = 1;"));
    assert!(rewritten.contains("const ANSWER = 42;"));
    assert!(rewritten.contains("const AFTER = 2;"));
}

#[cfg(feature = "oxc")]
#[test]
fn prepass_returns_none_when_no_decls() {
    let src = r#"const X = 1;
const Y = 2;
"#;
    let out = run_prepass_fixture(src);
    assert!(out.is_identity());
    assert!(out.rewritten.is_none());
}

#[cfg(feature = "oxc")]
#[test]
fn prepass_rewrites_tier2_function_with_string_return() {
    let src = r#"/** @buildtime */
function gen() {
  return "export const FROM_GEN = 99;";
}
"#;
    let out = run_prepass_fixture(src);
    assert!(
        out.diagnostics.is_empty(),
        "diagnostics: {:?}",
        out.diagnostics
    );
    let rewritten = out.rewritten.expect("expected rewrite");
    // The function declaration is replaced with the returned source.
    assert!(
        rewritten.contains("export const FROM_GEN = 99;"),
        "got: {rewritten}"
    );
    assert!(!rewritten.contains("function gen"));
}

#[cfg(feature = "oxc")]
#[test]
fn prepass_tier2_non_string_degrades_to_const() {
    let src = r#"/** @buildtime */
function compute() {
  return 42;
}
"#;
    let out = run_prepass_fixture(src);
    assert!(
        out.diagnostics.is_empty(),
        "diagnostics: {:?}",
        out.diagnostics
    );
    let rewritten = out.rewritten.expect("expected rewrite");
    assert!(
        rewritten.contains("const compute = 42;"),
        "got: {rewritten}"
    );
    assert!(!rewritten.contains("function compute"));
}

#[cfg(feature = "oxc")]
#[test]
fn prepass_tier1_emits_diagnostic_on_throw() {
    let src = r#"/** @buildtime */
const BROKEN = (() => { throw new Error("kapow"); })();
"#;
    let out = run_prepass_fixture(src);
    let errors: Vec<_> = out
        .diagnostics
        .iter()
        .filter(|d| matches!(d.level, crate::ts_syn::abi::DiagnosticLevel::Error))
        .collect();
    assert_eq!(errors.len(), 1, "diagnostics: {:?}", out.diagnostics);
    let diag = errors[0];
    assert!(
        diag.message.contains("kapow"),
        "diag message: {}",
        diag.message
    );
    // No rewrite because the only decl failed.
    assert!(out.rewritten.is_none());
}

#[cfg(feature = "oxc")]
#[test]
fn prepass_records_dependencies() {
    use std::io::Write;
    let mut tmp = tempfile::NamedTempFile::new().unwrap();
    writeln!(tmp.as_file_mut(), r#"{{"value": "from-disk"}}"#).unwrap();
    let path_lit = serde_json::to_string(&tmp.path().to_string_lossy().into_owned()).unwrap();

    let src = format!(
        r#"/** @buildtime */
const FROM_FILE = buildtime.fs.readJson({path_lit}).value;
"#
    );
    let out = run_prepass_fixture(&src);
    assert!(
        out.diagnostics.is_empty(),
        "diagnostics: {:?}",
        out.diagnostics
    );
    assert_eq!(out.dependencies.len(), 1);
    let rewritten = out.rewritten.expect("expected rewrite");
    assert!(
        rewritten.contains(r#"const FROM_FILE = "from-disk";"#),
        "got: {rewritten}"
    );
}

#[cfg(feature = "oxc")]
#[test]
fn prepass_throw_remaps_stack_to_user_source() {
    // The declaration lives on line 2 of the source. When the sandbox
    // throws, the `notes` on the diagnostic should mention the user's
    // file path in the remapped stack, not `eval_script` or `<anonymous>`.
    let src = r#"// header
/** @buildtime */
const BAD = (() => { throw new Error("bang"); })();
"#;
    let out = run_prepass_fixture(src);
    let errors: Vec<_> = out
        .diagnostics
        .iter()
        .filter(|d| matches!(d.level, crate::ts_syn::abi::DiagnosticLevel::Error))
        .collect();
    assert_eq!(errors.len(), 1, "diagnostics: {:?}", out.diagnostics);
    let diag = errors[0];
    let joined_notes = diag.notes.join("\n");
    assert!(
        joined_notes.contains("/tmp/fixture.ts"),
        "expected user filepath in stack; got notes: {}",
        joined_notes
    );
    assert!(
        !joined_notes.contains("eval_script"),
        "eval_script not rewritten; got notes: {}",
        joined_notes
    );
    // The primary span now covers the `/** @buildtime */` JSDoc on
    // source line 2 *and* the const decl on line 3 (the rewrite
    // erases both). The span starts at the JSDoc.
    let span = diag.span.expect("span set");
    let line_of_span = src[..span.start as usize].matches('\n').count() + 1;
    assert_eq!(line_of_span, 2);
}

#[cfg(feature = "oxc")]
#[test]
fn prepass_same_file_prelude_allows_function_reference() {
    // Zig-comptime-style: a @buildtime body can call a pure function
    // defined elsewhere in the same file.
    let src = r#"function upper(s) { return s.toUpperCase(); }

/** @buildtime */
const RESULT = upper("hello");
"#;
    let out = run_prepass_fixture(src);
    assert!(
        out.diagnostics.is_empty(),
        "diagnostics: {:?}",
        out.diagnostics
    );
    let rewritten = out.rewritten.expect("expected rewrite");
    assert!(
        rewritten.contains(r#"const RESULT = "HELLO";"#),
        "got: {rewritten}"
    );
}

#[cfg(feature = "oxc")]
#[test]
fn prepass_same_file_prelude_with_pure_const() {
    let src = r#"const MULTIPLIER = 10;

/** @buildtime */
const TOTAL = MULTIPLIER * 5;
"#;
    let out = run_prepass_fixture(src);
    assert!(
        out.diagnostics.is_empty(),
        "diagnostics: {:?}",
        out.diagnostics
    );
    let rewritten = out.rewritten.expect("expected rewrite");
    assert!(rewritten.contains("const TOTAL = 50;"), "got: {rewritten}");
}

#[cfg(feature = "oxc")]
#[test]
fn prepass_iife_with_ts_annotations_in_body() {
    // Reproduces the GREETINGS issue from the vanilla playground:
    // a Tier 1 IIFE whose body contains TS-only constructs.
    let src = r#"/** @buildtime */
const GREETINGS = ((): Record<string, string> => {
    const names = ['alice', 'bob', 'cam'];
    const out: Record<string, string> = {};
    for (const n of names) out[n] = 'hello, ' + n;
    return out;
})();
"#;
    let out = run_prepass_fixture(src);
    let errors: Vec<_> = out
        .diagnostics
        .iter()
        .filter(|d| matches!(d.level, crate::ts_syn::abi::DiagnosticLevel::Error))
        .collect();
    assert!(
        errors.is_empty(),
        "errors: {:?}",
        out.diagnostics
            .iter()
            .map(|d| &d.message)
            .collect::<Vec<_>>()
    );
    let rewritten = out.rewritten.expect("expected rewrite");
    assert!(
        rewritten.contains(r#"alice"#) && rewritten.contains("GREETINGS"),
        "got: {rewritten}"
    );
}

#[cfg(feature = "oxc")]
#[test]
fn prepass_actual_playground_demo_file() {
    // Loads the playground demo source from disk and runs the prepass
    // against it. Catches integration regressions visible only with
    // the real demo's full structure.
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tooling/playground/vanilla/src/buildtime-demo.ts");
    let Ok(src) = std::fs::read_to_string(&path) else {
        eprintln!("skip: demo file not present at {}", path.display());
        return;
    };
    let allocator = oxc::allocator::Allocator::default();
    let parsed = oxc::parser::Parser::new(&allocator, &src, oxc::span::SourceType::ts()).parse();
    assert!(parsed.errors.is_empty(), "parse: {:?}", parsed.errors);
    let sandbox = BoaSandbox::new();
    let mut opts = SandboxOptions::new(path.clone());
    opts.capabilities = CapabilitySet::unrestricted();
    opts.timeout = Duration::from_secs(5);
    let out = crate::host::buildtime::run_prepass(&parsed.program, &src, &path, &sandbox, &opts);
    let errors: Vec<_> = out
        .diagnostics
        .iter()
        .filter(|d| matches!(d.level, crate::ts_syn::abi::DiagnosticLevel::Error))
        .collect();
    assert!(
        errors.is_empty(),
        "errors: {:?}",
        out.diagnostics
            .iter()
            .map(|d| &d.message)
            .collect::<Vec<_>>()
    );
    let rewritten = out.rewritten.expect("expected rewrite");
    eprintln!("REWRITTEN:\n{}\n----", rewritten);
    assert!(rewritten.contains("const GREETINGS"), "GREETINGS missing");
}

#[cfg(feature = "oxc")]
#[test]
fn prepass_full_playground_demo() {
    // Full reproduction of the vanilla playground's buildtime-demo.ts.
    // Catches integration regressions between discovery, TS-strip,
    // prelude builder, sandbox, and patch application.
    let src = r#"import { buildtime } from 'macroforge/buildtime';

/** @buildtime */
const ANSWER = 6 * 7;

/** @buildtime */
const SCHEMA_HASH = buildtime.crypto.sha256('user-schema-v1');

/** @buildtime */
const CONSTANT_OBJECT = {
    thirteen: 13,
    label: 'compile-time',
    items: [1, 2, 3]
};

/** @buildtime */
const GREETINGS = ((): Record<string, string> => {
    const names = ['alice', 'bob', 'cam'];
    const out: Record<string, string> = {};
    for (const n of names) out[n] = 'hello, ' + n;
    return out;
})();

/** @buildtime */
type UserId = 'string';

export interface BuildtimeDemoResult {
    answer: number;
    greetingAlice: string;
    userIdTag: UserId;
}

export function collectBuildtimeDemo(): BuildtimeDemoResult {
    return {
        answer: ANSWER,
        greetingAlice: GREETINGS.alice,
        userIdTag: 'placeholder' as UserId
    };
}
"#;
    let out = run_prepass_fixture(src);
    let errors: Vec<_> = out
        .diagnostics
        .iter()
        .filter(|d| matches!(d.level, crate::ts_syn::abi::DiagnosticLevel::Error))
        .collect();
    assert!(
        errors.is_empty(),
        "errors: {:?}",
        out.diagnostics
            .iter()
            .map(|d| &d.message)
            .collect::<Vec<_>>()
    );
    let rewritten = out.rewritten.expect("expected rewrite");
    eprintln!("REWRITTEN:\n{}\n----", rewritten);
    assert!(
        rewritten.contains("const ANSWER = 42;"),
        "ANSWER missing: {rewritten}"
    );
    assert!(
        rewritten.contains("const GREETINGS"),
        "GREETINGS missing: {rewritten}"
    );
    assert!(
        rewritten.contains(r#""alice""#) || rewritten.contains("alice:"),
        "alice key missing in GREETINGS: {rewritten}"
    );
}

#[cfg(feature = "oxc")]
#[test]
fn prepass_same_file_prelude_warns_on_impure_top_level() {
    // An impure top-level call disables the prelude and emits a warning
    // diagnostic. Evaluation still proceeds (the @buildtime can use
    // only what's in-scope inside its own body) — but in this case the
    // body also references the would-be prelude symbol, so it fails
    // with a reference error.
    let src = r#"console.log("side-effect at module load");

function helper() { return 42; }

/** @buildtime */
const VAL = helper();
"#;
    let out = run_prepass_fixture(src);
    // Expect at least one warning or error.
    let warnings: Vec<_> = out
        .diagnostics
        .iter()
        .filter(|d| matches!(d.level, crate::ts_syn::abi::DiagnosticLevel::Warning))
        .collect();
    assert!(
        !warnings.is_empty(),
        "expected a prelude-disabled warning; got: {:?}",
        out.diagnostics
    );
    assert!(
        warnings[0].message.contains("prelude disabled"),
        "warning: {}",
        warnings[0].message
    );
}

#[cfg(feature = "oxc")]
#[test]
fn prepass_top_level_await_inside_buildtime_resolves() {
    // Users can write `await` directly inside a @buildtime body.
    // The wrapper is async, so the promise chain is driven to
    // completion before the result is read.
    let src = r#"/** @buildtime */
const RESULT = await Promise.resolve(7).then(x => x * 6);
"#;
    let out = run_prepass_fixture(src);
    assert!(
        out.diagnostics.is_empty(),
        "diagnostics: {:?}",
        out.diagnostics
    );
    let rewritten = out.rewritten.expect("expected rewrite");
    assert!(rewritten.contains("const RESULT = 42;"), "got: {rewritten}");
}

#[cfg(feature = "oxc")]
#[test]
fn prepass_await_chain_resolves_fully() {
    let src = r#"/** @buildtime */
const CHAIN = await (async () => {
    const a = await Promise.resolve(1);
    const b = await Promise.resolve(2);
    return a + b + 39;
})();
"#;
    let out = run_prepass_fixture(src);
    assert!(
        out.diagnostics.is_empty(),
        "diagnostics: {:?}",
        out.diagnostics
    );
    let rewritten = out.rewritten.expect("expected rewrite");
    assert!(rewritten.contains("const CHAIN = 42;"), "got: {rewritten}");
}

#[cfg(feature = "oxc")]
#[test]
fn prepass_tier3_type_splices_string_as_type() {
    let src = r#"/** @buildtime */
type UserId = "string";
"#;
    let out = run_prepass_fixture(src);
    assert!(
        out.diagnostics.is_empty(),
        "diagnostics: {:?}",
        out.diagnostics
    );
    let rewritten = out.rewritten.expect("expected rewrite");
    assert!(
        rewritten.contains("type UserId = string;"),
        "got: {rewritten}"
    );
}

#[cfg(feature = "oxc")]
#[test]
fn prepass_tier3_type_template_literal_composes() {
    let src = r#"/** @buildtime */
type UnionOfLits = `"a" | "b" | "c"`;
"#;
    let out = run_prepass_fixture(src);
    assert!(
        out.diagnostics.is_empty(),
        "diagnostics: {:?}",
        out.diagnostics
    );
    let rewritten = out.rewritten.expect("expected rewrite");
    assert!(
        rewritten.contains(r#"type UnionOfLits = "a" | "b" | "c";"#),
        "got: {rewritten}"
    );
}

#[cfg(feature = "oxc")]
#[test]
fn prepass_tier3_type_export_preserved() {
    let src = r#"/** @buildtime */
export type Level = "number";
"#;
    let out = run_prepass_fixture(src);
    let rewritten = out.rewritten.expect("expected rewrite");
    assert!(
        rewritten.contains("export type Level = number;"),
        "got: {rewritten}"
    );
}

#[cfg(feature = "oxc")]
#[test]
fn prepass_tier3_non_string_errors() {
    // Non-string TS types — e.g., the type `number` itself — can't be
    // evaluated as a JS expression. So this intentionally fails at
    // compile time with a diagnostic on the user's decl.
    let src = r#"/** @buildtime */
type Bad = 42;
"#;
    let out = run_prepass_fixture(src);
    // Either the sandbox rejected evaluation (because `42` serialized
    // as a Number, not String) or serialization did. Either way: error.
    assert!(
        !out.diagnostics.is_empty(),
        "expected at least one diagnostic"
    );
    let msgs: String = out
        .diagnostics
        .iter()
        .map(|d| d.message.as_str())
        .collect::<Vec<_>>()
        .join("; ");
    assert!(
        msgs.to_lowercase().contains("type") || msgs.to_lowercase().contains("string"),
        "expected error hint about Tier 3 string requirement, got: {msgs}"
    );
}

#[cfg(feature = "oxc")]
#[test]
fn prepass_multiple_decls_compose() {
    let src = r#"/** @buildtime */
const A = 1 + 1;

/** @buildtime */
const B = 2 + 2;
"#;
    let out = run_prepass_fixture(src);
    assert!(
        out.diagnostics.is_empty(),
        "diagnostics: {:?}",
        out.diagnostics
    );
    let rewritten = out.rewritten.expect("expected rewrite");
    assert!(rewritten.contains("const A = 2;"));
    assert!(rewritten.contains("const B = 4;"));
}

// Shared-with-above util.
#[test]
fn serializer_table_matches_spec() {
    // null
    assert_eq!(value_to_ts_source(&SandboxValue::Null).unwrap(), "null");
    // bool
    assert_eq!(
        value_to_ts_source(&SandboxValue::Bool(true)).unwrap(),
        "true"
    );
    // number
    assert_eq!(
        value_to_ts_source(&SandboxValue::Number(1.23)).unwrap(),
        "1.23"
    );
    // bigint
    assert_eq!(
        value_to_ts_source(&SandboxValue::BigInt(42)).unwrap(),
        "42n"
    );
    // string
    assert_eq!(
        value_to_ts_source(&SandboxValue::String("hi".to_string())).unwrap(),
        r#""hi""#
    );
    // array
    assert_eq!(
        value_to_ts_source(&SandboxValue::Array(vec![
            SandboxValue::Number(1.0),
            SandboxValue::Number(2.0),
            SandboxValue::Number(3.0),
        ]))
        .unwrap(),
        "[1, 2, 3]"
    );
    // object
    let mut map = BTreeMap::new();
    map.insert("a".to_string(), SandboxValue::Number(1.0));
    map.insert("b".to_string(), SandboxValue::Number(2.0));
    assert_eq!(
        value_to_ts_source(&SandboxValue::Object(map)).unwrap(),
        "{a: 1, b: 2}"
    );
}
