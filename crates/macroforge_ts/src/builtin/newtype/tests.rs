use super::{single_type_argument, unexpanded_base};

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

#[test]
fn unexpanded_base_reads_primitive_newtypes() {
    assert_eq!(unexpanded_base("$Newtype<number>"), Some("number"));
    assert_eq!(unexpanded_base("$Newtype< string >"), Some("string"));
    assert_eq!(unexpanded_base("$Newtype<User>"), None);
    assert_eq!(unexpanded_base("number"), None);
}
