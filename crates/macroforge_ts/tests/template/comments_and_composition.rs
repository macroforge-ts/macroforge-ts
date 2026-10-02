//! Comment tags, and what a `{$typescript}` injection carries along.
use macroforge_ts::macros::ts_template;
use macroforge_ts::ts_syn::abi::{Diagnostic, DiagnosticLevel};

#[test]
fn a_string_line_comment_is_its_text() {
    let name = "User";
    let stream = ts_template! {
        {> "Generated for @{name}" <}
        const x = 1;
    };
    assert!(
        stream.source().contains("// Generated for User\n"),
        "got:\n{}",
        stream.source()
    );
}

#[test]
fn a_string_block_comment_is_its_text() {
    let stream = ts_template! {
        {>> "Keep in sync" <<}
        const x = 1;
    };
    assert!(
        stream.source().contains("/* Keep in sync */"),
        "got:\n{}",
        stream.source()
    );
}

#[test]
fn a_multi_line_comment_stays_a_comment() {
    let text = "first\nsecond */ still comment";
    let line = ts_template! { {> "@{text}" <} const x = 1; };
    assert!(
        line.source()
            .contains("// first\n// second */ still comment\n"),
        "got:\n{}",
        line.source()
    );
    let block = ts_template! { {>> "@{text}" <<} const x = 1; };
    assert!(
        block
            .source()
            .contains("/* first\nsecond * / still comment */"),
        "got:\n{}",
        block.source()
    );
}

#[test]
fn an_injected_stream_keeps_its_suffixes_and_diagnostics() {
    let mut inner = ts_template! { const inner = 1; };
    inner.add_cross_module_suffix("GetFields");
    inner.add_diagnostic(Diagnostic {
        level: DiagnosticLevel::Warning,
        message: "careful".to_string(),
        span: None,
        notes: vec![],
        help: None,
    });
    let outer = ts_template! {
        {$typescript inner}
        const outer = 2;
    };
    assert!(outer.source().contains("const inner = 1;"));
    assert_eq!(outer.cross_module_suffixes, ["GetFields"]);
    let result = outer.into_result();
    assert_eq!(result.diagnostics.len(), 1);
    assert_eq!(result.diagnostics[0].message, "careful");
}

#[test]
fn every_explicit_position_survives_injection() {
    let top = ts_template!(Top { import { helper } from "./helper"; });
    let body = ts_template!(Within { helper(): void {} });
    let combined = ts_template! {
        export function standalone(): void {}
        {$typescript top}
        {$typescript body}
    };
    let source = combined.source();
    let standalone = source
        .find("standalone")
        .expect("the default-positioned code");
    let top_marker = source
        .find("/* @macroforge:top */")
        .expect("the Top marker");
    let body_marker = source
        .find("/* @macroforge:body */")
        .expect("the Within marker");
    assert!(
        standalone < top_marker && top_marker < body_marker,
        "got:\n{source}"
    );
}

#[test]
fn adjacent_tokens_join_and_spaced_tokens_stay_spaced() {
    let name = "User";
    let stream = ts_template! {
        function get@{name}(): void {}
        const label = "@{name} record";
    };
    let source = stream.source();
    assert!(
        source.contains("function getUser(): void {}"),
        "got:\n{source}"
    );
    assert!(
        source.contains("const label = \"User record\";"),
        "got:\n{source}"
    );
}

#[test]
fn a_doc_comment_becomes_jsdoc() {
    let name = "User";
    let stream = ts_template! {
        /// The @{name} id.
        export const id = 1;
        /**
         * First line.
         * Second line.
         */
        export const other = 2;
    };
    let source = stream.source();
    assert!(source.contains("/** The User id. */"), "got:\n{source}");
    assert!(
        source.contains("* First line.\n") && source.contains("* Second line.\n"),
        "each line of the doc comment stays a line, got:\n{source}"
    );
    assert!(
        !source.contains("\\n"),
        "escapes must be decoded, got:\n{source}"
    );
}
