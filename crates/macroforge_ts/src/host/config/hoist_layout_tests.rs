//! How hoisted handlers are laid out in the expanded config.

use super::{handlers, project};

const NESTED: &str = r#"import { DateTime } from "effect";

export default {
    foreignTypes: {
        "DateTime.Utc": {
            from: ["effect"],
            decode: (raw: unknown) =>
                DateTime.make(raw as string),
            default: () => {
                const now = Date.now();
                return DateTime.make(now);
            },
        },
    },
};
"#;

#[test]
fn a_multi_line_handler_is_dedented_to_the_top_level() {
    let dir = project(&[("macroforge.config.ts", NESTED)]);
    let ts = handlers(&dir).typescript;
    assert!(
        ts.contains(
            "export const __foreign__dateTimeUtcDecode = (raw: unknown) =>\n    DateTime.make(raw as string);\n"
        ),
        "{ts}"
    );
    assert!(
        ts.contains(
            "export const __foreign__dateTimeUtcDefault = () => {\n    const now = Date.now();\n    return DateTime.make(now);\n};\n"
        ),
        "{ts}"
    );
}

#[test]
fn a_handler_with_a_multi_line_string_keeps_its_lines() {
    let config = "export default {\n    foreignTypes: {\n        Tag: {\n            from: [\"tags\"],\n            encode: (v: string) => `<${v}>\n            </${v}>`,\n            decode: (raw: unknown) => \"a\\\n            b\" + String(raw),\n        },\n    },\n};\n";
    let dir = project(&[("macroforge.config.ts", config)]);
    let ts = handlers(&dir).typescript;
    assert!(
        ts.contains("`<${v}>\n            </${v}>`"),
        "the template literal keeps its indentation. {ts}"
    );
    assert!(
        ts.contains("\"a\\\n            b\""),
        "the continued string keeps its indentation. {ts}"
    );
}
