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
