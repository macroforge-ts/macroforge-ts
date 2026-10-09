use super::single_type_argument;

#[test]
fn simple_argument_is_left_bare() {
    assert_eq!(single_type_argument("number").as_deref(), Ok("number"));
    assert_eq!(
        single_type_argument(" Array<string> ").as_deref(),
        Ok("Array<string>")
    );
}

#[test]
fn operator_arguments_are_parenthesized() {
    assert_eq!(
        single_type_argument("string | number").as_deref(),
        Ok("(string | number)")
    );
    assert_eq!(single_type_argument("A & B").as_deref(), Ok("(A & B)"));
    assert_eq!(
        single_type_argument("(x: number) => void").as_deref(),
        Ok("((x: number) => void)")
    );
}

#[test]
fn argument_count_is_checked() {
    assert_eq!(
        single_type_argument("").unwrap_err(),
        "takes exactly one type argument, got 0"
    );
    assert_eq!(
        single_type_argument("number, string").unwrap_err(),
        "takes exactly one type argument, got 2"
    );
}
