use crate::host::MacroExpander;
use crate::ts_syn::abi::DiagnosticLevel;

fn expand(source: &str) -> crate::host::MacroExpansion {
    MacroExpander::new()
        .expect("expander should build")
        .expand_source(source, "eq.ts")
        .expect("source should expand")
}

#[test]
fn eq_generates_nothing_beside_partial_eq() {
    let with_eq =
        expand("/** @derive(PartialEq, Eq) */\nexport interface Point {\n  x: number;\n}\n");
    let without_eq =
        expand("/** @derive(PartialEq) */\nexport interface Point {\n  x: number;\n}\n");

    assert!(with_eq.diagnostics.is_empty(), "{:?}", with_eq.diagnostics);
    assert_eq!(with_eq.code, without_eq.code);
    assert_eq!(with_eq.type_output, without_eq.type_output);
}

#[test]
fn eq_without_partial_eq_is_an_error() {
    let expansion = expand("/** @derive(Eq) */\nexport interface Point {\n  x: number;\n}\n");

    assert!(
        expansion.diagnostics.iter().any(|diagnostic| {
            diagnostic.level == DiagnosticLevel::Error
                && diagnostic.message == "`Eq` requires `PartialEq`, which `Point` does not derive"
        }),
        "{:?}",
        expansion.diagnostics
    );
}
