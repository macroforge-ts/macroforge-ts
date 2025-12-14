//! # Decorator Stripping Tests
//!
//! Tests verifying that Macroforge correctly removes decorator comments
//! from the expanded output.
//!
//! By default, decorator comments like `/** @derive(Debug) */` and
//! field-level decorators like `@debug()` are stripped from the output
//! since they are only meaningful during macro expansion.
//!
//! ## Configuration
//!
//! The `keep_decorators` configuration option can be used to preserve
//! decorators in the output if needed for debugging purposes.

use crate::host::{MacroConfig, MacroExpander};

/// Verifies that class-level and field-level decorators are removed by default.
///
/// This test ensures that:
/// - `/** @derive(Debug) */` comments are stripped
/// - `@debug()` field decorators are stripped
///
/// Users should not see macro decorators in the compiled output.
#[test]
fn strips_class_and_field_decorators_by_default() {
    let code = r#"
import { Derive } from "macroforge";

/** @derive(Debug) */
export class Example {
  @debug() foo: string;
}
"#;

    let expander =
        MacroExpander::with_config(MacroConfig::default(), std::env::current_dir().unwrap())
            .unwrap();
    let expanded = expander.expand(code, "example.ts").unwrap();

    assert!(
        !expanded.code.contains("@derive"),
        "expected class-level @derive decorator to be stripped",
    );
    assert!(
        !expanded.code.contains("@debug"),
        "expected field-level decorator to be stripped",
    );
}
