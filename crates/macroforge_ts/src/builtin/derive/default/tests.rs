/// The `Settings.defaultValue` method of `source`, which derives Default on
/// a class `Settings`.
fn default_value_method(source: &str) -> String {
    let result = crate::expand_core::expand_inner(source, "probe.ts", None).unwrap();
    assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
    let start = result
        .code
        .find("static defaultValue()")
        .expect("Default adds defaultValue");
    let end = start + result.code[start..].find("return instance;").unwrap();
    result.code[start..end].to_string()
}

/// The diagnostic messages of expanding `source`.
fn diagnostics(source: &str) -> Vec<String> {
    crate::expand_core::expand_inner(source, "probe.ts", None)
        .unwrap()
        .diagnostics
        .into_iter()
        .map(|diagnostic| diagnostic.message)
        .collect()
}

#[test]
fn fields_default_to_their_initializers() {
    let method = default_value_method(
        "/** @derive(Default) */\nexport class Settings {\n  theme: string = \"dark\";\n  retries?: number = 3;\n  label?: string;\n  count: number;\n}\n",
    );
    assert!(method.contains(r#"instance.theme = "dark";"#), "{method}");
    assert!(method.contains("instance.retries = 3;"), "{method}");
    assert!(!method.contains("instance.label"), "{method}");
    assert!(method.contains("instance.count = 0;"), "{method}");
}

#[test]
fn an_explicit_default_wins_over_the_initializer() {
    let method = default_value_method(
        "/** @derive(Default) */\nexport class Settings {\n  /** @default(7) */\n  limit: number = 5;\n}\n",
    );
    assert!(method.contains("instance.limit = 7;"), "{method}");
}

#[test]
fn an_initializer_stands_in_for_a_missing_default() {
    let method = default_value_method(
        "class Clock { now(): number { return 0; } }\n/** @derive(Default) */\nexport class Settings {\n  clock: Clock = new Clock();\n}\n",
    );
    assert!(method.contains("instance.clock = new Clock();"), "{method}");
}

#[test]
fn static_properties_get_no_default() {
    let method = default_value_method(
        "/** @derive(Default) */\nexport class Settings {\n  static instances: number = 0;\n  count: number = 1;\n}\n",
    );
    assert!(!method.contains("instances"), "{method}");
}

#[test]
fn an_initializer_reading_this_is_rejected() {
    for initializer in ["this.base + 1", "() => this.base"] {
        let messages = diagnostics(&format!(
            "/** @derive(Default) */\nexport class Settings {{\n  base: number = 1;\n  next: number | (() => number) = {initializer};\n}}\n"
        ));
        assert!(
            messages
                .iter()
                .any(|message| message.contains("the initializer of 'next' reads `this`")),
            "{initializer}: {messages:?}"
        );
    }
}

#[test]
fn a_function_initializer_keeps_its_own_this() {
    let method = default_value_method(
        "/** @derive(Default) */\nexport class Settings {\n  theme: string = \"dark\";\n  describe = function (this: Settings): string { return this.theme; };\n}\n",
    );
    assert!(
        method.contains("instance.describe = (function(this: Settings)"),
        "{method}"
    );
}

#[test]
fn a_function_typed_field_needs_a_default() {
    let messages =
        diagnostics("/** @derive(Default) */\nexport class Greeter {\n  greet: () => string;\n}\n");
    assert!(
        messages
            .iter()
            .any(|message| message.contains("function-typed fields") && message.contains("greet")),
        "{messages:?}"
    );
    let method = default_value_method(
        "/** @derive(Default) */\nexport class Greeter {\n  greet: () => string = () => \"hi\";\n  /** @default(() => 1) */\n  count: () => number;\n}\n",
    );
    assert!(
        method.contains("instance.greet = () => \"hi\";"),
        "{method}"
    );
    assert!(method.contains("instance.count = () => 1;"), "{method}");
}
