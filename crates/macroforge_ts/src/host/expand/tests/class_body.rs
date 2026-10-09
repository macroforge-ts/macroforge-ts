use super::super::helpers::class_body_payload;

/// `source` with `payload` inserted before its last `}`.
fn insert(source: &str) -> String {
    let brace = source.rfind('}').unwrap_or(source.len());
    let at = u32::try_from(brace + 1).unwrap_or(u32::MAX);
    let payload = class_body_payload("static make(): A {\n    return new A();\n}", source, at);
    format!("{}{payload}{}", &source[..brace], &source[brace..])
}

#[test]
fn members_go_on_their_own_lines_inside_the_braces() {
    assert_eq!(
        insert("class A {\n    x = 1;\n}"),
        "class A {\n    x = 1;\n\n    static make(): A {\n        return new A();\n    }\n}"
    );
}

#[test]
fn an_indented_class_keeps_its_brace_lined_up() {
    assert_eq!(
        insert("  class A {\n      x = 1;\n  }"),
        "  class A {\n      x = 1;\n      static make(): A {\n          return new A();\n      }\n  }"
    );
}

#[test]
fn a_template_literal_is_left_as_written() {
    let payload = class_body_payload("static s = `a\nb`;", "class A {}", 10);
    assert_eq!(payload, "\nstatic s = `a\nb`;\n");
}
