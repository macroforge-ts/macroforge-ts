use super::*;
use std::collections::HashMap;

#[test]
fn test_parse_simple_config() {
    let content = r#"
            export default {
                keepDecorators: true,
                generateConvenienceConst: false
            }
        "#;

    let config = MacroforgeConfigLoader::from_config_file(content, "macroforge.config.js").unwrap();
    assert!(config.keep_decorators);
    assert!(!config.generate_convenience_const);
}

#[test]
fn test_parse_config_with_foreign_types() {
    let content = r#"
            export default {
                foreignTypes: {
                    DateTime: {
                        from: ["effect"],
                        serialize: (v, ctx) => v.toJSON(),
                        deserialize: (raw, ctx) => DateTime.fromJSON(raw)
                    }
                }
            }
        "#;

    let config = MacroforgeConfigLoader::from_config_file(content, "macroforge.config.js").unwrap();
    assert_eq!(config.foreign_types.len(), 1);

    let dt = &config.foreign_types[0];
    assert_eq!(dt.name, "DateTime");
    assert_eq!(dt.from, vec!["effect"]);
    assert!(dt.serialize_expr.is_some());
    assert!(dt.deserialize_expr.is_some());
}

#[test]
fn test_parse_config_with_multiple_sources() {
    let content = r#"
            export default {
                foreignTypes: {
                    DateTime: {
                        from: ["effect", "@effect/schema"]
                    }
                }
            }
        "#;

    let config = MacroforgeConfigLoader::from_config_file(content, "macroforge.config.js").unwrap();
    let dt = &config.foreign_types[0];
    assert_eq!(dt.from, vec!["effect", "@effect/schema"]);
}

#[test]
fn test_parse_typescript_config() {
    let content = r#"
            import { DateTime } from "effect";

            export default {
                foreignTypes: {
                    DateTime: {
                        from: ["effect"],
                        serialize: (v: DateTime, ctx: unknown) => v.toJSON(),
                    }
                }
            }
        "#;

    let config = MacroforgeConfigLoader::from_config_file(content, "macroforge.config.ts").unwrap();
    assert_eq!(config.foreign_types.len(), 1);
}

#[test]
fn test_default_values() {
    let content = "export default {}";
    let config = MacroforgeConfigLoader::from_config_file(content, "macroforge.config.js").unwrap();

    assert!(!config.keep_decorators);
    assert!(config.generate_convenience_const);
    assert!(config.foreign_types.is_empty());
}

#[test]
fn test_legacy_macro_config_conversion() {
    let mf_config = MacroforgeConfig {
        keep_decorators: true,
        generate_convenience_const: false,
        foreign_types: vec![],
        config_imports: HashMap::new(),
    };

    let legacy: MacroConfig = mf_config.into();
    assert!(legacy.keep_decorators);
    assert!(!legacy.generate_convenience_const);
}
