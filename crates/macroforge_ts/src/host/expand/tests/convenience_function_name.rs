use super::super::helpers::extract_function_names_from_patches;
use crate::ts_syn::abi::{Patch, SpanIR};

#[test]
fn generic_and_plain_functions_are_both_collected() {
    let patch = Patch::Insert {
        at: SpanIR::new(1, 1),
        code: "export function boxDecode<T>(input: unknown) {}\n\
               export function boxHasShape(obj: unknown) {}\n\
               export function boxDecodeWithContext <T, U>(value: any) {}\n\
               export function otherDecode(input: unknown) {}"
            .to_string(),
        source_macro: None,
    };
    let names = extract_function_names_from_patches(&[patch], "Box");
    assert_eq!(
        names,
        [
            ("boxDecode".to_string(), "decode".to_string()),
            ("boxHasShape".to_string(), "hasShape".to_string()),
            (
                "boxDecodeWithContext".to_string(),
                "decodeWithContext".to_string()
            ),
        ]
    );
}

/// A function is found by its whole name, in whatever script it is written
/// and with a `$` in it.
#[test]
fn a_function_named_outside_ascii_is_read_whole() {
    for (type_name, function) in [
        ("Ünïcode", "ünïcodeClone"),
        ("$Dollar", "$dollarClone"),
        ("_Hidden", "_hiddenClone"),
    ] {
        let patch = Patch::Insert {
            at: SpanIR::new(0, 0),
            code: format!("export function {function}(value: {type_name}): {type_name} {{}}"),
            source_macro: None,
        };
        assert_eq!(
            extract_function_names_from_patches(&[patch], type_name),
            [(function.to_string(), "clone".to_string())],
            "{type_name}"
        );
    }
}
