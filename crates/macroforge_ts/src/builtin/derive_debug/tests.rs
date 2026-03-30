use super::types::DebugFieldOptions;
use crate::ts_syn::abi::{DecoratorIR, SpanIR};

fn span() -> SpanIR {
    SpanIR::new(0, 0)
}

#[test]
fn test_skip_flag() {
    let decorator = DecoratorIR {
        name: "Debug".into(),
        args_src: "skip".into(),
        span: span(),
        node: None,
    };

    let opts = DebugFieldOptions::from_decorators(&[decorator]);
    assert!(opts.skip, "skip flag should be true");
}

#[test]
fn test_skip_false_keeps_field() {
    let decorator = DecoratorIR {
        name: "Debug".into(),
        args_src: r#"{ skip: false }"#.into(),
        span: span(),
        node: None,
    };

    let opts = DebugFieldOptions::from_decorators(&[decorator]);
    assert!(!opts.skip, "skip: false should not skip the field");
}

#[test]
fn test_rename_option() {
    let decorator = DecoratorIR {
        name: "Debug".into(),
        args_src: r#"{ rename: "identifier" }"#.into(),
        span: span(),
        node: None,
    };

    let opts = DebugFieldOptions::from_decorators(&[decorator]);
    assert_eq!(opts.rename.as_deref(), Some("identifier"));
}
