#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/pass_basic.rs");
    t.pass("tests/ui/pass_multiple.rs");
    t.compile_fail("tests/ui/fail_no_name.rs");
    t.compile_fail("tests/ui/fail_invalid_kind.rs");
}
