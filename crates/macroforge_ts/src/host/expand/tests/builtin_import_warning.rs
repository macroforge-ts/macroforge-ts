use super::super::imports::check_builtin_import_warnings;
use crate::host::MacroExpander;
use crate::ts_syn::abi::{Diagnostic, DiagnosticLevel};
use oxc::allocator::Allocator;
use oxc::parser::Parser;
use oxc::span::SourceType;

fn warnings_for(source: &str) -> Vec<Diagnostic> {
    let allocator = Allocator::default();
    let parsed = Parser::new(&allocator, source, SourceType::ts()).parse();
    assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
    check_builtin_import_warnings(&parsed.program)
}

#[test]
fn warns_on_importing_newtype_from_macroforge() {
    let source = r#"import { $Newtype } from "@macroforge/core";

export type Meters = $Newtype<number>;"#;

    let warnings = warnings_for(source);

    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].message.contains("$Newtype"));
    assert!(
        warnings[0]
            .help
            .as_deref()
            .is_some_and(|help| help.contains("$Newtype<T> in type position"))
    );
}

#[test]
fn warns_on_importing_debug_from_macroforge() {
    let source = r#"import { Debug } from "@macroforge/core";

/** @derive(Debug) */
class User {
name: string;
}"#;

    let warnings = warnings_for(source);

    assert_eq!(warnings.len(), 1);
    assert_eq!(warnings[0].level, DiagnosticLevel::Warning);
    assert!(warnings[0].message.contains("Debug"));
    assert!(warnings[0].message.contains("built-in macro"));
    assert!(
        warnings[0]
            .help
            .as_deref()
            .is_some_and(|help| help.contains("@derive(Debug)"))
    );
}

#[test]
fn warns_on_importing_encode_from_macroforge_core() {
    let source = r#"import { Encode, Decode } from "@macroforge/core";

/** @derive(Encode, Decode) */
class User {
name: string;
}"#;

    let warnings = warnings_for(source);

    assert_eq!(warnings.len(), 2);
    assert!(
        warnings
            .iter()
            .any(|warning| warning.message.contains("Encode"))
    );
    assert!(
        warnings
            .iter()
            .any(|warning| warning.message.contains("Decode"))
    );
}

#[test]
fn warns_on_importing_clone_from_macro_derive() {
    let source = r#"import { Clone, Default, Hash } from "@macro/derive";

/** @derive(Clone, Default, Hash) */
class Config {
value: number;
}"#;

    let warnings = warnings_for(source);

    assert_eq!(warnings.len(), 3);
    assert!(
        warnings
            .iter()
            .any(|warning| warning.message.contains("Clone"))
    );
    assert!(
        warnings
            .iter()
            .any(|warning| warning.message.contains("Default"))
    );
    assert!(
        warnings
            .iter()
            .any(|warning| warning.message.contains("Hash"))
    );
}

#[test]
fn no_warning_for_non_macro_imports() {
    let source = r#"import { Debug } from "my-custom-lib";
import { Clone } from "./local-utils";

class User {
name: string;
}"#;

    assert!(warnings_for(source).is_empty());
}

#[test]
fn no_warning_for_custom_macro_imports() {
    let source = r#"import { MyCustomMacro } from "@macroforge/core";

/** @derive(MyCustomMacro) */
class User {
name: string;
}"#;

    assert!(warnings_for(source).is_empty());
}

#[test]
fn warns_with_correct_span() {
    let source = r#"import { Debug } from "@macroforge/core";"#;

    let warnings = warnings_for(source);

    assert_eq!(warnings.len(), 1);
    let span = warnings[0].span.expect("warning should carry a span");
    // Spans are 1-based.
    assert_eq!(
        &source[span.start as usize - 1..span.end as usize - 1],
        "Debug"
    );
}

#[test]
fn warns_all_ord_variants() {
    let source = r#"import { Ord, PartialOrd, Eq, PartialEq } from "@macroforge/core";

/** @derive(Ord, PartialOrd, Eq, PartialEq) */
class Comparable {
value: number;
}"#;

    let warnings = warnings_for(source);

    assert_eq!(warnings.len(), 4);
    assert!(
        warnings
            .iter()
            .any(|warning| warning.message.contains("Ord"))
    );
    assert!(
        warnings
            .iter()
            .any(|warning| warning.message.contains("PartialOrd"))
    );
    assert!(
        warnings
            .iter()
            .any(|warning| warning.message.contains("PartialEq"))
    );
    assert!(
        warnings
            .iter()
            .any(|warning| warning.message.contains("'Eq'"))
    );
}

#[test]
fn expansion_reports_builtin_import_warning() {
    let source = r#"import { Debug } from "@macroforge/core";

/** @derive(Debug) */
class User {
name: string;
}"#;

    let expansion = MacroExpander::new()
        .expect("expander should build")
        .expand_source(source, "user.ts")
        .expect("source should expand");

    assert!(
        expansion.diagnostics.iter().any(|diagnostic| {
            diagnostic.level == DiagnosticLevel::Warning
                && diagnostic.message.contains("'Debug' is a built-in macro")
        }),
        "{:?}",
        expansion.diagnostics
    );
}
