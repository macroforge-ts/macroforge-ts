//! Compile-time behavior of `#[ts_macro_derive]`, checked against the engine
//! that consumes its output.

#[test]
fn ui() {
    let cases = trybuild::TestCases::new();
    cases.pass("tests/macro_ui/pass_basic.rs");
    cases.pass("tests/macro_ui/pass_multiple.rs");
    cases.compile_fail("tests/macro_ui/fail_no_name.rs");
    cases.compile_fail("tests/macro_ui/fail_invalid_kind.rs");
}
