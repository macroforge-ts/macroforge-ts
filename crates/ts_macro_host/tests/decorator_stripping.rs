use ts_macro_host::{MacroConfig, MacroExpander};

/// Ensure decorator removal is applied when keep_decorators is not set
#[test]
fn strips_class_and_field_decorators_by_default() {
    let code = r#"
import { Derive } from "@macroforge/swc-napi";

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
