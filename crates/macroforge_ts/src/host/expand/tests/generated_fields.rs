//! Each built-in derive over fields of generated types: none panics, and an
//! expansion that reports no error is code that parses.

use oxc::allocator::Allocator;
use oxc::parser::Parser;
use oxc::span::SourceType;
use proptest::test_runner::{Config, TestCaseError};
use proptest::{prop_assert, prop_assume, proptest};

use crate::host::{DiagnosticLevel, MacroExpander};
use crate::ts_syn::strategies::{property_config, ts_type};

const DERIVES: [&str; 11] = [
    "Debug",
    "Clone",
    "PartialEq",
    "Eq",
    "Hash",
    "PartialOrd",
    "Ord",
    "Default",
    "Encode",
    "Decode",
    "Encode, Decode",
];

/// A type deriving `derive` over a field of type `field`, and the
/// declarations a generated type may name.
fn source(derive: &str, field: &str) -> String {
    format!(
        r#"export interface User {{
    id: string;
}}

export namespace NS {{
    export type T = string;
}}

/** @derive({derive}) */
export interface Probe<T> {{
    field: {field};
}}
"#
    )
}

/// Whether oxc reads `source` as TypeScript without an error.
fn parses(source: &str) -> bool {
    let allocator = Allocator::default();
    Parser::new(&allocator, source, SourceType::ts())
        .parse()
        .diagnostics
        .is_empty()
}

proptest! {
    #![proptest_config(Config { cases: 96, ..property_config() })]

    #[test]
    fn a_derive_over_any_field_panics_never_and_writes_code_that_parses(ts_type in ts_type()) {
        let field = ts_type.written().text;
        prop_assume!(parses(&source("Debug", &field)));
        let host = MacroExpander::new().map_err(|error| TestCaseError::fail(format!("{error:#}")))?;
        for derive in DERIVES {
            let expansion = host
                .expand_source(&source(derive, &field), "probe.ts")
                .map_err(|error| TestCaseError::fail(format!("{derive} over {field}: {error:#}")))?;
            let panicked = expansion
                .diagnostics
                .iter()
                .find(|diagnostic| diagnostic.message.contains("panicked"));
            prop_assert!(panicked.is_none(), "{} over {}: {:?}", derive, field, panicked);
            let failed = expansion.diagnostics.iter().any(|diagnostic| diagnostic.level == DiagnosticLevel::Error);
            prop_assert!(failed || parses(&expansion.code), "{} over {}:\n{}", derive, field, expansion.code);
        }
    }
}
