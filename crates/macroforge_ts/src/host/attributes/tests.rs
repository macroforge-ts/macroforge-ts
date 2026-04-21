//! End-to-end tests for the attribute pre-pass. Each test builds a tiny OXC
//! program, runs [`super::run_prepass`] against it, and asserts on the
//! rewritten source / diagnostics.

use std::path::PathBuf;

use oxc::allocator::Allocator;
use oxc::parser::Parser as OxcParser;
use oxc::span::SourceType;

use macroforge_ts_syn::config::{
    CfgFlags, DeprecatedConfig, MacroforgeConfig, MustUseConfig, NonExhaustiveConfig,
};

use super::run_prepass;

fn run(code: &str, config: &MacroforgeConfig) -> super::AttributePrepassOutput {
    let allocator = Allocator::default();
    let ret = OxcParser::new(&allocator, code, SourceType::ts()).parse();
    assert!(ret.errors.is_empty(), "parse errors: {:?}", ret.errors);
    run_prepass(&ret.program, code, &PathBuf::from("/tmp/test.ts"), config)
}

fn base_config() -> MacroforgeConfig {
    MacroforgeConfig::default()
}

// ---------------------------------------------------------------------------
// @cfg
// ---------------------------------------------------------------------------

#[test]
fn cfg_strips_when_feature_absent() {
    let src = r#"
/** @cfg({ feature: 'ssr' }) */
export function render() {}
"#;
    let out = run(src, &base_config());
    assert!(
        out.diagnostics
            .iter()
            .all(|d| !matches!(d.level, crate::ts_syn::abi::DiagnosticLevel::Error)),
        "unexpected errors: {:?}",
        out.diagnostics
    );
    let rewritten = out.rewritten.expect("should have rewritten");
    assert!(
        !rewritten.contains("render"),
        "render should be stripped, got:\n{rewritten}"
    );
    assert!(
        !rewritten.contains("@cfg"),
        "@cfg comment should be stripped"
    );
}

#[test]
fn cfg_keeps_when_feature_present() {
    let src = r#"
/** @cfg({ feature: 'ssr' }) */
export function render() {}
"#;
    let mut config = base_config();
    config.cfg = CfgFlags {
        features: vec!["ssr".into()],
        ..Default::default()
    };
    let out = run(src, &config);
    let rewritten = out
        .rewritten
        .expect("should have rewritten to strip comment");
    assert!(
        rewritten.contains("function render"),
        "render should survive:\n{rewritten}"
    );
    assert!(!rewritten.contains("@cfg"), "annotation should be stripped");
}

#[test]
fn cfg_implicit_and_across_keys() {
    // Both must match: 'web' target passes but 'debugAssertions: false' does not.
    let src = r#"
/** @cfg({ target: 'web', debugAssertions: true }) */
export function onlyOnDebug() {}
"#;
    let mut config = base_config();
    config.cfg = CfgFlags {
        target: Some("web".into()),
        debug_assertions: false,
        ..Default::default()
    };
    let out = run(src, &config);
    let rewritten = out.rewritten.expect("should strip");
    assert!(
        !rewritten.contains("onlyOnDebug"),
        "AND should strip when one key fails"
    );
}

// ---------------------------------------------------------------------------
// @deprecated
// ---------------------------------------------------------------------------

#[test]
fn deprecated_injects_tsc_jsdoc_with_message() {
    let src = r#"
/** @deprecated('use render2 instead') */
export function render() {}
"#;
    let out = run(src, &base_config());
    let rewritten = out.rewritten.expect("should rewrite");
    assert!(
        rewritten.contains("@deprecated use render2 instead"),
        "tsc-visible JSDoc missing:\n{rewritten}"
    );
    assert!(
        !rewritten.contains("@deprecated('"),
        "macroforge-style annotation should be gone"
    );
}

#[test]
fn deprecated_fail_on_use_emits_error() {
    let src = r#"
/** @deprecated('gone after 1.0') */
export function legacy() {}
"#;
    let mut config = base_config();
    config.deprecated = DeprecatedConfig {
        runtime_warn: false,
        fail_on_use: true,
    };
    let out = run(src, &config);
    assert!(
        out.diagnostics.iter().any(|d| matches!(
            d.level,
            crate::ts_syn::abi::DiagnosticLevel::Error
        ) && d.message.contains("legacy")),
        "expected fail_on_use error, got: {:?}",
        out.diagnostics
    );
}

// ---------------------------------------------------------------------------
// @mustUse
// ---------------------------------------------------------------------------

#[test]
fn must_use_diagnostic_on_discarded_call() {
    let src = r#"
/** @mustUse */
export function openConnection() { return 1; }
openConnection();
const kept = openConnection();
"#;
    let out = run(src, &base_config());
    let errors: Vec<_> = out
        .diagnostics
        .iter()
        .filter(|d| matches!(d.level, crate::ts_syn::abi::DiagnosticLevel::Error))
        .collect();
    assert_eq!(
        errors.len(),
        1,
        "expected exactly one diagnostic, got {:?}",
        out.diagnostics
    );
    assert!(errors[0].message.contains("openConnection"));
    // Suppress unused warning for `kept`.
    let _ = MustUseConfig::default();
}

// ---------------------------------------------------------------------------
// @nonExhaustive
// ---------------------------------------------------------------------------

#[test]
fn non_exhaustive_brands_type_alias_rhs() {
    let src = r#"
/** @nonExhaustive */
export type Kind = 'a' | 'b' | 'c';
"#;
    let out = run(src, &base_config());
    let rewritten = out.rewritten.expect("should rewrite RHS");
    assert!(
        rewritten.contains("readonly __nonExhaustive"),
        "brand missing from rewrite:\n{rewritten}"
    );
    assert!(
        rewritten.contains("'a' | 'b' | 'c'"),
        "original variants should be preserved:\n{rewritten}"
    );
}

#[test]
fn non_exhaustive_respects_custom_brand() {
    let src = r#"
/** @nonExhaustive */
type Kind = 'a';
"#;
    let mut config = base_config();
    config.non_exhaustive = NonExhaustiveConfig {
        brand: "__extensible".into(),
    };
    let out = run(src, &config);
    let rewritten = out.rewritten.expect("should rewrite");
    assert!(
        rewritten.contains("__extensible"),
        "custom brand missing:\n{rewritten}"
    );
}
